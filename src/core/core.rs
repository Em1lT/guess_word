use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::utils::read_file;
use rand::Rng;

enum InputMode {
    Editing,
}

struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
        }
    }
}

pub fn setup()-> Result<(), Box<dyn Error>> {
    let random_word: String = random_word();
    let total_tries: u8 = 5;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app: App = App::default();
    welcome_msg(total_tries, &mut app);
    let res = run_app(&mut terminal, app, random_word, total_tries);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn enumarate_answer(guess_word: &String, correct_word: String)-> String {
    let mut next_guess: String = "".to_owned();
    let word_str: Vec<char> = correct_word.chars().collect();

    for (i, c) in guess_word.chars().enumerate() {
        if c == word_str[i] {
            next_guess.push_str("O ")
        } else if correct_word.contains(c) {
            next_guess.push_str("! ")
        } else {
            next_guess.push_str("X ")
        }
    };
    next_guess
}

fn valid_answer(answer: &String)-> bool {
    if answer.len() == 5 { true } else { false }
}

fn new_msg(app: &mut App, msg: String) {
    app.messages.push(msg);
}

fn welcome_msg(total_tries: u8, app: &mut App) {
    let mut msg: String = "".to_owned();
    msg.push_str("Guess a 5 letter word");
    msg.push_str("\n ");
    msg.push_str(&total_tries.to_string());
    msg.push_str(" tries");
    msg.push_str("\n ");
    new_msg(app, msg);
    new_msg(app, "[ ? ? ? ? ? ]".to_string());
}

fn random_word()-> String {
    let word_list = read_file();
    let random_number: u16 = rand::thread_rng().gen_range(0..500);
    let mut random_word: String = "".to_string();
    let mut cnt: u16 = 0;
    for line in word_list {
        if cnt == random_number {
            // println!("{}", &line.unwrap());
            random_word.push_str(&line.unwrap());
        }
        cnt = cnt + 1;
    }
    random_word
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, winning_word: String, total_tries: u8) -> io::Result<()> {
    let mut game_end: bool = false;
    let mut tries: u8 = 1;
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let user_answer = app.input.drain(..).collect();
                        if game_end == false {
                            if valid_answer(&user_answer) {
                                 if user_answer == winning_word {
                                     app.messages.push("you won!".to_string());
                                     app.messages.push("Winning word is:".to_string());
                                     app.messages.push(winning_word.to_string());
                                     app.messages.push("Game ended".to_string());
                                     game_end = true;
                                     continue;
                                 }

                                 if tries == total_tries {
                                     app.messages.push(winning_word.to_string());
                                     app.messages.push("Game ended".to_string());
                                     game_end = true;
                                     continue;
                                 }

                                 let answer_row: String = enumarate_answer(&user_answer, winning_word.to_string());
                                app.messages.push(user_answer.to_string());
                                app.messages.push(answer_row);
                                let mut msg: String = "".to_owned();
                                msg.push_str("Tries left: ");
                                let tries_left = total_tries - tries;
                                msg.push_str(&tries_left.to_string());
                                msg.push_str("\n ");
                                app.messages.push(msg);
                                tries = tries + 1;
                            } else {
                                app.messages.push("Not valid answer".to_string());
                            }
                        } else {
                            return Ok(());
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(5),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quit, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quess word!"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[2]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(_, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}", m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL));
    f.render_widget(messages, chunks[0]);
}

