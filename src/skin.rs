use {
    crossterm::style::{Attribute::*, Color::*},
    minimad::Alignment,
    termimad::*,
};

pub fn make_skin(color: bool) -> MadSkin {
    if color {
        make_color_skin()
    } else {
        make_no_color_skin()
    }
}

fn make_color_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    //skin.paragraph.set_fg(AnsiValue(153));
    skin.headers[0].align = Alignment::Left;
    skin.set_headers_fg(AnsiValue(153));
    skin.strikeout.remove_attr(CrossedOut);
    skin.strikeout.set_fg(AnsiValue(9));
    skin.italic.remove_attr(Italic);
    skin.italic.set_fg(AnsiValue(70));
    skin
}

fn make_no_color_skin() -> MadSkin {
    MadSkin::no_style()
}
