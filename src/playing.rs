use crate::{actor::Actor, actor_manager::ActorManager, consts::GLYPH_SIZE, data::GameData, map_manager::MapManager, position::Position};
use glynnlib::*;

pub struct Playing {
    game_data: GameData,
    actors: ActorManager,
    map: MapManager,
}

impl Playing {
    pub fn new(game_data: GameData, player_actor: Actor) -> Self {
        Self {
            game_data,
            actors: ActorManager::new(player_actor),
            map: MapManager::new(),
        }
    }

    pub fn update(mut self, context: &mut Context) -> crate::AppState {
        // UI changes
        // TODO

        // Player turn handling
        if let Some(action) = self.handle_player_turn(context) {
            self.process_action(0, action);
            self.run_ai_turns();
        }

        // Update game logic here
        crate::AppState::Playing(self)
    }

    fn handle_player_turn(&self, context: &mut Context) -> Option<Action> {
        const DIRECTIONS: [(KeyCode, Position); 16] = [
            (KeyCode::Numpad8, Position { x: 0, y: -1 }),
            (KeyCode::Numpad9, Position { x: 1, y: -1 }),
            (KeyCode::Numpad6, Position { x: 1, y: 0 }),
            (KeyCode::Numpad3, Position { x: 1, y: 1 }),
            (KeyCode::Numpad2, Position { x: 0, y: 1 }),
            (KeyCode::Numpad1, Position { x: -1, y: 1 }),
            (KeyCode::Numpad4, Position { x: -1, y: 0 }),
            (KeyCode::Numpad7, Position { x: -1, y: -1 }),
            (KeyCode::KeyW, Position { x: 0, y: -1 }),
            (KeyCode::KeyE, Position { x: 1, y: -1 }),
            (KeyCode::KeyD, Position { x: 1, y: 0 }),
            (KeyCode::KeyC, Position { x: 1, y: 1 }),
            (KeyCode::KeyX, Position { x: 0, y: 1 }),
            (KeyCode::KeyZ, Position { x: -1, y: 1 }),
            (KeyCode::KeyA, Position { x: -1, y: 0 }),
            (KeyCode::KeyQ, Position { x: -1, y: -1 }),
        ];
        for (key, delta) in DIRECTIONS {
            if context.is_key_pressed(key) {
                let player_position = self.actors.get_player_actor().position();
                let new_position = Position {
                    x: player_position.x + delta.x,
                    y: player_position.y + delta.y,
                };
                return Some(Action::MoveTo(new_position));
            }
        }

        None
    }

    fn run_ai_turns(&mut self) {
        while let Some((actor_index, actor)) = self.actors.next_turn() {
            if actor_index == 0 {
                break;
            } else {
                let action = actor.ai_turn();
                self.process_action(actor_index, action);
            }
        }
    }

    fn process_action(&mut self, actor_index: usize, action: Action) {
        let cost = action.cost();
        match action {
            Action::Wait => {}
            Action::MoveTo(position) => self.actors.get_actor_mut(actor_index).unwrap().set_position(position),
        }
        self.actors.end_turn(cost);
    }

    pub fn draw(&self, context: &mut Context) {
        let player_actor_position = self.actors.get_player_actor().position();
        let camera = Camera {
            target: vec2(player_actor_position.x as f32 * GLYPH_SIZE as f32, player_actor_position.y as f32 * GLYPH_SIZE as f32),
            zoom: 1.0,
        };
        context.set_world_camera(&camera);

        self.map.draw(context);
        self.actors.draw(context);
    }
}

pub enum Action {
    Wait,
    MoveTo(Position),
}

impl Action {
    fn cost(&self) -> u32 {
        match self {
            Action::Wait => 100,
            Action::MoveTo(_) => 100,
        }
    }
}
