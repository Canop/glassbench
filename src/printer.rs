use {
    super::*,
    termimad::{
        crossterm::tty::IsTty,
        minimad::{OwningTemplateExpander, TextTemplate},
        terminal_size, FmtText, MadSkin,
    },
};

/// A small helper to print using markdown templates
pub struct Printer {
    pub skin: MadSkin,
    pub terminal_width: usize,
}

impl Printer {
    /// create a new printer
    ///
    /// The skin will be without style and color if the
    /// output is piped.
    pub fn new() -> Self {
        let terminal_width = terminal_size().0 as usize;
        let color = !is_output_piped();
        let skin = skin::make_skin(color);
        Self {
            skin,
            terminal_width,
        }
    }
    pub fn print(&self, expander: OwningTemplateExpander, template: &str) {
        let template = TextTemplate::from(template);
        let text = expander.expand(&template);
        let fmt_text = FmtText::from_text(&self.skin, text, Some(self.terminal_width));
        print!("{}", fmt_text);
    }
}

fn is_output_piped() -> bool {
    !std::io::stdout().is_tty()
}
