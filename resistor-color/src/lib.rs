use int_enum::IntEnum;
use std::fmt::{ Display, Formatter, Result };
use enum_iterator::{ all, Sequence };

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, IntEnum, Copy, Clone, Sequence, Ord, PartialOrd)]
pub enum ResistorColor {
    Brown = 1,
    Red = 2,
    Black = 0,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::White => write!(f, "White"),
            ResistorColor::Yellow => write!(f, "Yellow")
        }
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => format!("{color}"),
        Err(_error) => format!("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors_vec = all::<ResistorColor>().collect::<Vec<_>>();

    colors_vec.sort_by(|a, b| a.cmp(b));

    colors_vec
}
