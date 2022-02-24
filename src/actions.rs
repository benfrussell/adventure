use bevy::prelude::*;

enum Action
{
    Move (MoveAction)
}

#[derive(Component)]
pub struct MoveAction { start_pos: Vec3, end_pos: Vec3 }

impl MoveAction {
    pub fn do_action(&self, mut t: Mut<Transform>) {
        t.translation = self.end_pos;
    }
}

#[derive(Component)]
pub struct ActionList { actions: Vec<Action>, next_action: Action }

pub fn do_actions(
    mut query: Query<(&ActionList, Option<&mut Transform>)>
) {
    for (actions, transform) in query.iter_mut() {
        match (&actions.next_action, transform) {
            (Action::Move(a), Some(t)) => a.do_action(t),
            _ => ()
        }
    }
}