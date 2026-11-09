use rustigram::types::keyboard::{
    KeyboardButton, ReplyKeyboardMarkup, ReplyKeyboardRemove, ReplyMarkup,
};

pub fn yes_no() -> ReplyMarkup {
    ReplyMarkup::ReplyKeyboard(ReplyKeyboardMarkup {
        keyboard: vec![vec![
            KeyboardButton::text("Ha"),
            KeyboardButton::text("Yo'q"),
        ]],
        is_persistent: None,
        resize_keyboard: Some(true),
        one_time_keyboard: Some(true),
        input_field_placeholder: None,
        selective: None,
    })
}

pub fn remove() -> ReplyMarkup {
    ReplyMarkup::Remove(ReplyKeyboardRemove {
        remove_keyboard: true,
        selective: None,
    })
}
