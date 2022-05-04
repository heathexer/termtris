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
    // let chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
    //     .split(f.size());
    // let title_rect = chunks[0];
    // let body_rect = chunks[1];

    let tmp_rect = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin((f.size().height - Game::DISPLAY_HEIGHT as u16 - 2) / 2)
        .constraints(
            [
                Constraint::Max(0),
                Constraint::Length(Game::DISPLAY_HEIGHT as u16 + 2),
                Constraint::Max(0),
            ]
            .as_ref(),
        )
        .split(f.size())[1];
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Max(0),
                Constraint::Length(12),
                Constraint::Length((Game::WIDTH as u16 * 2) + 2),
                Constraint::Length(12),
                Constraint::Max(50),
                Constraint::Max(0),
            ]
            .as_ref(),
        )
        .split(tmp_rect);
    let left_panel_rect = main_chunks[1];
    let game_rect = main_chunks[2];
    let next_blocks_rect = main_chunks[3];
    let info_rect = main_chunks[4];

    let left_panel_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(0),
                Constraint::Length(6),
                Constraint::Min(0),
                Constraint::Length(4),
                Constraint::Max(0),
            ]
            .as_ref(),
        )
        .split(left_panel_rect);
    let hold_block_rect = left_panel_chunks[1];
    let high_score_rect = left_panel_chunks[3];

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(2, 5), Constraint::Ratio(3, 5)].as_ref())
        .split(info_rect);
    let score_rect = info_chunks[0];
    let help_rect = info_chunks[1];

    // let title = draw_title();
    // f.render_widget(title, title_rect);

    draw_next_blocks(f, next_blocks_rect, &app.game);
    draw_hold_block(f, hold_block_rect, &app.game);

    let help = draw_help();
    f.render_widget(help, help_rect);

    draw_game_board(f, game_rect, &app.game);

    draw_score(f, score_rect, &app.game);
    draw_high_score(f, high_score_rect, &app.game);
}

fn draw_help<'a>() -> Table<'a> {
    let key_style = Style::default().fg(Color::Gray);
    let msg_style = Style::default()
        .fg(Color::Gray)
        .add_modifier(Modifier::BOLD);

    let mut rows = vec![];
    // Empty row for padding
    rows.push(Row::new(vec![
        Cell::from(Span::raw("")),
        Cell::from(Span::raw("")),
    ]));

    for action in Action::iterator() {
        let key = action.keys()[0];
        let row = Row::new(vec![
            Cell::from(Span::styled(format!("{:^8}", format!("{key}")), key_style)),
            Cell::from(Span::styled(format!("{action}"), msg_style)),
        ]);
        rows.push(row);
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::White))
                .title("Controls"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_next_blocks<B>(f: &mut Frame<B>, rect: Rect, game: &Game)
where
    B: Backend,
{
    let widget = game
        .next_pieces_paragraph()
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

fn draw_high_score<B>(f: &mut Frame<B>, rect: Rect, game: &Game)
where
    B: Backend,
{
    let widget = game
        .get_high_score_paragraph()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("High─Score"),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    f.render_widget(widget, rect);
}

fn draw_score<B>(f: &mut Frame<B>, rect: Rect, game: &Game)
where
    B: Backend,
{
    let widget = game
        .get_score_paragraph()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Score"),
        )
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green));

    f.render_widget(widget, rect);
}
