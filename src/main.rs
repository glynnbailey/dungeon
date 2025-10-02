#![allow(dead_code)]

mod actor;
mod actor_manager;
mod character_creation;
mod consts;
mod main_menu;
mod map_manager;
mod playing;
mod position;
mod data;

use glynnlib::*;

struct App {
    state: Option<AppState>,
}

impl App {
    fn new() -> Self {
        Self {
            state: Some(AppState::MainMenu(main_menu::MainMenu::new())),
        }
    }
}

impl Application for App {
    fn update(&mut self, context: &mut Context) {
        self.state = Some(match self.state.take().unwrap() {
            AppState::MainMenu(main_menu) => main_menu.update(context),
            AppState::CharacterCreation(character_creation) => character_creation.update(context),
            AppState::Playing(playing) => playing.update(context),
        });

        match &self.state.as_ref().unwrap() {
            AppState::MainMenu(main_menu) => main_menu.draw(context),
            AppState::CharacterCreation(character_creation) => character_creation.draw(context),
            AppState::Playing(playing) => playing.draw(context),
        }
    }
}

enum AppState {
    MainMenu(main_menu::MainMenu),
    CharacterCreation(character_creation::CharacterCreation),
    Playing(playing::Playing),
}

fn main() {
    let app = App::new();
    let texture_paths = vec![];
    let font_path = "assets/fonts/terminus.ttf".to_string();
    let mut engine = Engine::new(app, "Dungeon".to_string(), WindowSize::None, texture_paths, font_path);
    engine.run();
}
