use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait Spawner<'w, 's> {
    fn spawn<'a, T: Bundle>(&'a mut self, bundle: T) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> Spawner<'w, 's> for Commands<'w, 's> {
    fn spawn<'a, T: Bundle>(&'a mut self, bundle: T) -> EntityCommands<'w, 's, 'a> {
        self.spawn(bundle)
    }
}

impl<'w, 's> Spawner<'w, 's> for ChildBuilder<'w, 's, '_> {
    fn spawn<'a, T: Bundle>(&'a mut self, bundle: T) -> EntityCommands<'w, 's, 'a> {
        self.spawn(bundle)
    }
}
