use std::marker::PhantomData;

use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{StatefulWidget, Tabs as TuiTabs, Widget};

use crate::cli::{components::get_block, TuiStates};

#[derive(Default, Clone, Debug)]
pub struct Tabs<'a> {
    _life: PhantomData<&'a ()>,
}

impl<'a> StatefulWidget for Tabs<'a> {
    type State = TuiStates<'a>;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let len = TuiStates::TITLES.len();
        let range = if state.show_debug { 0..len } else { 0..len - 1 };
        let titles = TuiStates::TITLES[range]
            .iter()
            .enumerate()
            .map(|(i, t)| {
                Spans::from(Span::styled(
                    format!("{} {}", i + 1, t),
                    Style::default().fg(Color::DarkGray),
                ))
            })
            .collect();
        let tabs = TuiTabs::new(titles)
            .block(get_block("Clashctl"))
            .highlight_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .select(state.page_index);
        tabs.render(area, buf)
    }
}
