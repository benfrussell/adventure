use bevy::prelude::*;

enum Action {
    Move (MoveAction)
}

impl Action {
    pub fn is_finished(&self) -> bool {
        match &self {
            Action::Move(a) => a.reached_end
        }
    }
}

#[derive(Component)]
pub struct MoveAction { start_pos: Vec3, end_pos: Vec3, reached_end: bool }

impl MoveAction {
    pub fn new(start_pos: Vec3, end_pos: Vec3) -> MoveAction {
        MoveAction { start_pos: start_pos, end_pos: end_pos, reached_end: false}
    }

    pub fn do_action(&mut self, mut t: Mut<Transform>) {
        t.translation = self.end_pos;
        self.reached_end = true;
    }
}

#[derive(Component)]
pub struct ActionList { actions: Vec<Action> }

impl ActionList {
    pub fn new() -> ActionList {
        ActionList { actions: Vec::new() }
    }

    pub fn get_action(&mut self) -> Option<&mut Action> {
        if self.actions.len() == 0 {
            return None
        }

        loop {
            let action_is_finished = self.actions[0].is_finished();

            if action_is_finished {
                self.actions.remove(0);
            } else {
                return Some(&mut self.actions[0])
            }
        }
    }
}

pub fn do_actions(
    mut query: Query<(&mut ActionList, Option<&mut Transform>)>
) {
    for (mut actions, transform) in query.iter_mut() {
        let next_action = actions.get_action();

        if next_action.is_some() {
            match (next_action.unwrap(), transform) {
                (Action::Move(a), Some(t)) => a.do_action(t),
                _ => ()
            }
        }
        
    }
}