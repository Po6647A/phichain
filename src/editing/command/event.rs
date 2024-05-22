use crate::chart::event::{LineEvent, LineEventBundle};
use bevy::hierarchy::BuildWorldChildren;
use bevy::prelude::*;
use undo::Edit;

#[derive(Debug, Copy, Clone)]
pub struct CreateEvent {
    pub line_entity: Entity,
    pub event: LineEvent,
    pub event_entity: Option<Entity>,
}

impl CreateEvent {
    #[allow(dead_code)]
    pub fn new(line: Entity, event: LineEvent) -> Self {
        Self {
            line_entity: line,
            event,
            event_entity: None,
        }
    }
}

impl Edit for CreateEvent {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        target.entity_mut(self.line_entity).with_children(|parent| {
            self.event_entity = Some(parent.spawn(LineEventBundle::new(self.event)).id());
        });
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(entity) = self.event_entity {
            target.despawn(entity);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RemoveEvent {
    pub event_entity: Entity,
    pub event: Option<LineEvent>,
    pub line_entity: Option<Entity>,
}

impl RemoveEvent {
    #[allow(dead_code)]
    pub fn new(entity: Entity) -> Self {
        Self {
            event_entity: entity,
            event: None,
            line_entity: None,
        }
    }
}

impl Edit for RemoveEvent {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        self.event = target.entity(self.event_entity).get::<LineEvent>().copied();
        self.line_entity = target
            .entity(self.event_entity)
            .get::<Parent>()
            .map(|x| x.get());
        target.despawn(self.event_entity);
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(event) = self.event {
            if let Some(line) = self.line_entity {
                target.entity_mut(line).with_children(|parent| {
                    self.event_entity = parent.spawn(LineEventBundle::new(event)).id();
                });
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EditEvent {
    entity: Entity,
    from: LineEvent,
    to: LineEvent,
}

impl EditEvent {
    pub fn new(entity: Entity, from: LineEvent, to: LineEvent) -> Self {
        Self { entity, from, to }
    }
}

impl Edit for EditEvent {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(mut event) = target.entity_mut(self.entity).get_mut::<LineEvent>() {
            *event = self.to;
        }
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(mut event) = target.entity_mut(self.entity).get_mut::<LineEvent>() {
            *event = self.from;
        }
    }
}

#[cfg(test)]
mod tests {}
