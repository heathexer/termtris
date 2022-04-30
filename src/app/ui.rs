use crate::{app::App, game::Game};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use super::actions::Action;
pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(f.size());
    let title_rect = chunks[0];
    let body_rect = chunks[1];

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)].as_ref())
        .split(body_rect);
    let main_rect = body_chunks[0];
    let info_rect = body_chunks[1];

    let tmp_rect = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(Game::DISPLAY_HEIGHT as u16 + 2),
            Constraint::Min(0),
        ])
        .split(main_rect)[0];
    let main_chunks = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((Game::WIDTH as u16 * 2) + 2),
            Constraint::Length(8),
        ])
        .split(tmp_rect);
    let game_rect = main_chunks[0];
    let panel_rect = main_chunks[1];

    let panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(panel_rect);
    let next_block_rect = panel_chunks[0];
    let hold_block_rect = panel_chunks[2];

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(info_rect);
    let help_rect = info_chunks[0];
    let _unused_rect = info_chunks[1];

    let title = draw_title();
    f.render_widget(title, title_rect);

    draw_next_block(f, next_block_rect, &app.game);
    draw_hold_block(f, hold_block_rect, &app.game);

    let help = draw_help();
    f.render_widget(help, help_rect);

    draw_game_board(f, game_rect, &app.game);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("TERMTRIS")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_help<'a>() -> Table<'a> {
    let key_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let msg_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];

    for action in Action::iterator() {
        let key = action.keys()[0];
        let row = Row::new(vec![
            Cell::from(Span::styled(format!("  {key}"), key_style)),
            Cell::from(Span::styled(format!("{action}"), msg_style)),
        ]);
        rows.push(row);
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Controls"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_next_block<B>(f: &mut Frame<B>, rect: Rect, game: &Game)
where
    B: Backend,
{
    let widget = game
        .next_piece_paragraph()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Next")
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));

    f.render_widget(widget, rect);
}

fn draw_hold_block<B>(f: &mut Frame<B>, rect: Rect, game: &Game)
where
    B: Backend,
{
    let widget = game
        .hold_piece_paragraph()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Hold")
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    f.render_widget(widget, rect);
}

fn draw_game_board<B>(f: &mut Frame<B>, rect: Rect, game: &Game)
where
    B: Backend,
{
    let board: Paragraph = game.get_board_paragraph();
    let widget = board
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(widget, rect);
}
