use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, IntoEnumIterator, IntEnum, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value as u8) {
        Ok(color) => format!("{:?}", color),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    use std::cmp::Ordering;
    let mut sorted_colors: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    sorted_colors.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    sorted_colors
}
