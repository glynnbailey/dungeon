use crate::{AppState, character_creation::CharacterCreation};
use glynnlib::*;

const MENU_OPTIONS: [&str; 3] = ["New Game", "Load Game", "Quit"];

pub struct MainMenu {
    cursor: u8,
}

impl MainMenu {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn update(mut self, context: &mut Context) -> AppState {
        if context.is_key_pressed(KeyCode::ArrowDown) || context.is_key_pressed(KeyCode::Numpad2) {
            self.cursor = (self.cursor + 1) % MENU_OPTIONS.len() as u8;
        }

        if context.is_key_pressed(KeyCode::ArrowUp) || context.is_key_pressed(KeyCode::Numpad8) {
            if self.cursor == 0 {
                self.cursor = (MENU_OPTIONS.len() - 1) as u8;
            } else {
                self.cursor -= 1;
            }
        }

        if context.is_key_pressed(KeyCode::Enter) || context.is_key_pressed(KeyCode::NumpadEnter) {
            match self.cursor {
                0 => return AppState::CharacterCreation(CharacterCreation::new()),
                1 => println!("Load Game selected"),
                2 => context.quit(),
                _ => {}
            }
        }

        if context.is_key_pressed(KeyCode::Escape) {
            context.quit();
        }

        AppState::MainMenu(self)
    }

    pub fn draw(&self, context: &mut Context) {
        context.draw_text("Dungeon".to_string(), 30.0, 30.0, 48, WHITE);

        for (i, option) in MENU_OPTIONS.iter().enumerate() {
            let y = 150.0 + i as f32 * 50.0;
            let color = if i as u8 == self.cursor {
                YELLOW
            } else {
                WHITE
            };
            context.draw_text(option.to_string(), 32.0, y, 30, color);
        }
    }
}
