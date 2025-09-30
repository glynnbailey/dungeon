use crate::{consts::GLYPH_SIZE, playing::Action, position::Position};
use glynnlib::*;

pub struct Actor {
    name: Option<String>,
    position: Position,
}

impl Actor {
    pub fn new(name: Option<String>, position: Position) -> Self {
        Self { name, position }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn ai_turn(&self) -> Action {
        // Placeholder for AI logic
        Action::Wait
    }

    pub fn draw(&self, context: &mut glynnlib::Context) {
        context.draw_char('@', self.position.x as f32 * GLYPH_SIZE as f32, self.position.y as f32 * GLYPH_SIZE as f32, GLYPH_SIZE, WHITE);
    }
}
