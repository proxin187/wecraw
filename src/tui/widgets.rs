use super::*;


pub fn control_panel(frame: &mut Frame, stats: Stats, runtime: Instant) {
    let main_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(50), Constraint::Fill(1)])
        .vertical_margin(10)
        .split(frame.size());

    block(frame, main_layout[1], " control panel ");

    let inner_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(main_layout[1]);

    let stack_layout = Layout::vertical([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)])
        .split(inner_layout[0]);

    chunks(frame, stats, stack_layout[1]);

    general(frame, runtime, stack_layout[0]);
}

fn block(frame: &mut Frame, area: Rect, title: &str) {
    let block = Block::bordered().title_top(Line::from(title).centered()).border_type(BorderType::Rounded);

    frame.render_widget(block, area);
}

fn general(frame: &mut Frame, runtime: Instant, area: Rect) {
    let secs = runtime.elapsed().as_secs();

    let text = vec![
        Line::from(vec![
            Span::raw("runtime : ").style(Style::new().cyan()),
            Span::raw(((secs / 60) / 60).to_string()),
            Span::raw(" h, "),
            Span::raw(((secs / 60) % 60).to_string()),
            Span::raw(" m, "),
            Span::raw((secs % 60).to_string()),
            Span::raw(" s"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::bordered()
                .title_top(Line::from(" process timing ").centered())
                .border_type(BorderType::Rounded)
        );

    frame.render_widget(paragraph, area);
}

fn chunks(frame: &mut Frame, stats: Stats, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::raw("  chunk : ").style(Style::new().cyan()),
            Span::raw(stats.chunk_index.to_string()),
            Span::raw(" / "),
            Span::raw(stats.chunk_count.to_string()),
        ]),
        Line::from(vec![
            Span::raw("visited : ").style(Style::new().cyan()),
            Span::raw(stats.visited.to_string()),
            Span::raw(" unique url(s)"),
        ]),
        Line::from(vec![
            Span::raw("indexed : ").style(Style::new().cyan()),
            Span::raw(stats.indexed.to_string()),
        ]),
        Line::from(vec![
            Span::raw("  depth : ").style(Style::new().cyan()),
            Span::raw(stats.depth.to_string()),
        ]),
        Line::from(vec![
            Span::raw(" status : ").style(Style::new().cyan()),
            Span::raw(stats.status.to_string()),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::bordered()
                .title_top(Line::from(" crawler info ").centered())
                .border_type(BorderType::Rounded)
        );

    frame.render_widget(paragraph, area);
}


