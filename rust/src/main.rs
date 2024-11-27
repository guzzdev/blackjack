mod game;

use crate::game::Game;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = Game::new();
    let mut bet = 10;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let block = Block::default().title("Blackjack").borders(Borders::ALL);
            f.render_widget(block, size);

            let player_money = Paragraph::new(Spans::from(vec![Span::styled(
                format!("Money: €{}", game.get_player_money()),
                Style::default().fg(Color::Yellow),
            )]));
            f.render_widget(player_money, chunks[0]);

            let stats = Paragraph::new(Spans::from(vec![
                Span::styled(
                    format!("Wins: {}", game.get_wins()),
                    Style::default().fg(Color::Green),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("Losses: {}", game.get_losses()),
                    Style::default().fg(Color::Red),
                ),
            ]));
            f.render_widget(stats, chunks[1]);

            let bet_amount = Paragraph::new(Spans::from(vec![Span::styled(
                format!("Bet Amount: €{}", bet),
                Style::default().fg(Color::Cyan),
            )]));
            f.render_widget(bet_amount, chunks[2]);

            let player_hand = Paragraph::new(Spans::from(vec![Span::styled(
                format!("Player Hand: {}", game.get_player_hand().display(false)),
                Style::default().fg(Color::White),
            )]));
            f.render_widget(player_hand, chunks[3]);

            let dealer_hand = Paragraph::new(Spans::from(vec![Span::styled(
                format!("Dealer Hand: {}", game.get_dealer_hand().display(!game.is_game_over())),
                Style::default().fg(Color::White),
            )]));
            f.render_widget(dealer_hand, chunks[4]);

            let message = if game.is_game_over() {
                if game.get_player_money() <= 0 {
                    "Game Over! You have no more money."
                } else {
                    "Round Over! Place a new bet to continue. \n Press space to continue"
                }
            } else {
                "Press 'h' to Hit, 's' to Stand, 'q' to Quit."
            };
            let message_paragraph = Paragraph::new(Spans::from(vec![Span::styled(
                message,
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            )]));
            f.render_widget(message_paragraph, chunks[5]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => {
                    if !game.is_game_over() {
                        game.hit();
                    }
                }
                KeyCode::Char('s') => {
                    if !game.is_game_over() {
                        game.stand();
                    }
                }
                KeyCode::Char(' ') => {
                    if game.is_game_over() {
                        game.start_round(bet);
                    }
                }
                KeyCode::Char('+') => {
                    if game.is_game_over() {
                        bet += 10;
                    }
                }
                KeyCode::Char('-') => {
                    if game.is_game_over() && bet > 10 {
                        bet -= 10;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}