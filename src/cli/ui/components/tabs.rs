use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{StatefulWidget, Tabs as TuiTabs, Widget};

use crate::cli::{components::get_block, TuiStates};

#[derive(Default, Clone, Debug)]
pub struct Tabs {}

impl StatefulWidget for Tabs {
    type State = TuiStates;
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let titles = TuiStates::TITLES
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::DarkGray))))
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
