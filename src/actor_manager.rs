use crate::actor::Actor;
use std::collections::BinaryHeap;

pub struct ActorManager {
    actors: Vec<Actor>,
    turn_queue: BinaryHeap<ActorTurn>,
    current_turn: Option<ActorTurn>,
}

impl ActorManager {
    pub fn new(player_actor: Actor) -> Self {
        Self {
            actors: vec![player_actor],
            turn_queue: BinaryHeap::new(),
            current_turn: Some(ActorTurn::new(0, 0)),
        }
    }

    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
        let index = self.actors.len() - 1;
        self.turn_queue.push(ActorTurn::new(index, 0));
    }

    pub fn remove_actor(&mut self, index: usize) {
        if index < self.actors.len() {
            self.actors.remove(index);
            self.turn_queue = self
                .turn_queue
                .drain()
                .filter_map(|mut turn| {
                    if turn.actor_index == index {
                        // Remove turns for the deleted actor
                        None
                    } else if turn.actor_index > index {
                        // Decrease index for actors that shifted down
                        turn.actor_index -= 1;
                        Some(turn)
                    } else {
                        // Keep turns for actors with lower indices unchanged
                        Some(turn)
                    }
                })
                .collect();
        }
    }

    pub fn get_actor(&self, index: usize) -> Option<&Actor> {
        self.actors.get(index)
    }

    pub fn get_actor_mut(&mut self, index: usize) -> Option<&mut Actor> {
        self.actors.get_mut(index)
    }

    pub fn get_player_actor(&self) -> &Actor {
        &self.actors[0]
    }

    pub fn next_turn(&mut self) -> Option<(usize, &Actor)> {
        match self.turn_queue.pop() {
            Some(turn) => {
                self.current_turn = Some(turn);
                let actor_index = self.current_turn.as_ref().unwrap().actor_index;
                return Some((actor_index, self.actors.get(actor_index).unwrap()));
            }
            None => unreachable!("Turn queue should never be empty"),
        }
    }

    pub fn end_turn(&mut self, cost: u32) {
        match self.current_turn.take() {
            Some(mut turn) => {
                turn.action_points += cost;
                self.turn_queue.push(turn);
            }
            None => unreachable!("No current turn to end"),
        }
    }

    pub fn draw(&self, context: &mut glynnlib::Context) {
        for actor in &self.actors {
            actor.draw(context);
        }
    }
}

struct ActorTurn {
    actor_index: usize,
    action_points: u32,
}

impl ActorTurn {
    pub fn new(actor_index: usize, action_points: u32) -> Self {
        Self { actor_index, action_points }
    }
}

impl PartialEq for ActorTurn {
    fn eq(&self, other: &Self) -> bool {
        self.actor_index == other.actor_index
    }
}

impl Eq for ActorTurn {}

impl PartialOrd for ActorTurn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for ActorTurn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.action_points.cmp(&self.action_points)
    }
}
