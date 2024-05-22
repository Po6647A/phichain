use crate::chart::note::{Note, NoteBundle};
use bevy::prelude::*;
use undo::Edit;

#[derive(Debug, Copy, Clone)]
pub struct CreateNote {
    pub line_entity: Entity,
    pub note: Note,
    pub note_entity: Option<Entity>,
}

impl CreateNote {
    #[allow(dead_code)]
    pub fn new(line: Entity, note: Note) -> Self {
        Self {
            line_entity: line,
            note,
            note_entity: None,
        }
    }
}

impl Edit for CreateNote {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        target.entity_mut(self.line_entity).with_children(|parent| {
            self.note_entity = Some(parent.spawn(NoteBundle::new(self.note)).id());
        });
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(entity) = self.note_entity {
            target.despawn(entity);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RemoveNote(pub Entity, pub Option<Note>);

impl RemoveNote {
    #[allow(dead_code)]
    pub fn new(entity: Entity) -> Self {
        Self(entity, None)
    }
}

impl Edit for RemoveNote {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        self.1 = target.entity(self.0).get::<Note>().copied();
        target.despawn(self.0);
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(note) = self.1 {
            self.0 = target.spawn(NoteBundle::new(note)).id();
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EditNote {
    entity: Entity,
    from: Note,
    to: Note,
}

impl EditNote {
    pub fn new(entity: Entity, from: Note, to: Note) -> Self {
        Self { entity, from, to }
    }
}

impl Edit for EditNote {
    type Target = World;
    type Output = ();

    fn edit(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(mut note) = target.entity_mut(self.entity).get_mut::<Note>() {
            *note = self.to;
        }
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        if let Some(mut note) = target.entity_mut(self.entity).get_mut::<Note>() {
            *note = self.from;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::beat::Beat;
    use crate::chart::line::LineBundle;
    use crate::chart::note::{Note, NoteBundle, NoteKind};
    use crate::editing::command::EditorCommand;
    use undo::History;

    fn test_remove_note_system(world: &mut World) {
        let mut history = History::new();
        let note = Note::new(NoteKind::Tap, true, Beat::ZERO, 0.0, 1.0);
        let entity = world.spawn(NoteBundle::new(note)).id();
        assert!(world.query::<&Note>().get_single(world).is_ok());
        history.edit(world, EditorCommand::RemoveNote(RemoveNote::new(entity)));
        assert!(world.query::<&Note>().get_single(world).is_err());
        history.undo(world);
        assert!(world.query::<&Note>().get_single(world).is_ok());
        history.redo(world);
        assert!(world.query::<&Note>().get_single(world).is_err());
    }

    #[test]
    fn test_remove_note_command() {
        let mut app = App::new();
        app.add_systems(Update, test_remove_note_system);
        app.update();
    }

    fn test_create_note_system(world: &mut World) {
        let mut history = History::new();
        let line = world.spawn(LineBundle::new()).id();
        let note = Note::new(NoteKind::Tap, true, Beat::ZERO, 0.0, 1.0);
        assert!(world.query::<&Note>().get_single(world).is_err());
        history.edit(
            world,
            EditorCommand::CreateNote(CreateNote::new(line, note)),
        );
        assert!(world.query::<&Note>().get_single(world).is_ok());
        history.undo(world);
        assert!(world.query::<&Note>().get_single(world).is_err());
        history.redo(world);
        assert!(world.query::<&Note>().get_single(world).is_ok());
    }

    #[test]
    fn test_create_note_command() {
        let mut app = App::new();
        app.add_systems(Update, test_create_note_system);
        app.update();
    }
}