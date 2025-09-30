use crate::consts::MAP_SIZE;
use glynnlib::*;

pub struct MapManager {
    tiles: Vec<Tile>,
}

impl MapManager {
    pub fn new() -> Self {
        Self { tiles: vec![Tile::Floor; MAP_SIZE * MAP_SIZE] }
    }

    pub fn draw(&self, context: &mut glynnlib::Context) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let tile = &self.tiles[y * MAP_SIZE + x];
                let glyph = match tile {
                    Tile::Wall => '#',
                    Tile::Floor => '.',
                };
                context.draw_char(glyph, x as f32 * 32.0, y as f32 * 32.0, 32, GRAY);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
}
