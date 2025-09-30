use crate::{AppState, actor::Actor, playing::Playing, position::Position};
use glynnlib::*;

pub struct CharacterCreation {
    name: String,
}

impl CharacterCreation {
    pub fn new() -> Self {
        Self { name: String::new() }
    }

    pub fn update(mut self, context: &mut Context) -> AppState {
        for ch in context.get_chars() {
            if ch.is_ascii_alphanumeric() && self.name.len() < 32 {
                self.name.push(ch);
            }
        }

        if context.is_key_pressed(KeyCode::Backspace) {
            self.name.pop();
        }

        if context.is_key_pressed(KeyCode::Enter) {
            let player_actor = Actor::new(Some(self.name.clone()), Position::new(0, 0));
            let playing_state = Playing::new(player_actor);
            return AppState::Playing(playing_state);
        }

        if context.is_key_pressed(KeyCode::Escape) {
            return AppState::MainMenu(crate::main_menu::MainMenu::new());
        }

        AppState::CharacterCreation(self)
    }

    pub fn draw(&self, context: &mut Context) {
        context.draw_text("Character Creation".to_string(), 30.0, 30.0, 48, WHITE);
        context.draw_text("Enter your character's name:".to_string(), 30.0, 100.0, 30, WHITE);
        context.draw_text(self.name.clone(), 30.0, 150.0, 30, YELLOW);
        context.draw_text("Press Enter to confirm or Escape to cancel.".to_string(), 30.0, 250.0, 20, GRAY);
    }
}
