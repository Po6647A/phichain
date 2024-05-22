use crate::chart::event::LineEvent;
use crate::selection::{SelectEvent, Selected, SelectedLine};
use crate::tab::timeline::{Timeline, TimelineViewport};
use crate::timing::BpmList;
use crate::widgets::event::event_ui;
use bevy::hierarchy::Parent;
use bevy::prelude::{Entity, EventWriter, In, Query, Res};
use egui::Ui;

pub fn event_timeline_system(
    In(ui): In<&mut Ui>,
    selected_line_query: Res<SelectedLine>,
    timeline_viewport: Res<TimelineViewport>,
    bpm_list: Res<BpmList>,
    event_query: Query<(&LineEvent, &Parent, Entity, Option<&Selected>)>,
    mut select_events: EventWriter<SelectEvent>,
    timeline: Timeline,
) {
    let selected_line = selected_line_query.0;
    let viewport = timeline_viewport;

    let event_timeline_viewport = viewport.event_timeline_viewport();

    for (event, parent, entity, selected) in event_query.iter() {
        if parent.get() != selected_line {
            continue;
        }

        let track = match event.kind {
            crate::chart::event::LineEventKind::X => 1,
            crate::chart::event::LineEventKind::Y => 2,
            crate::chart::event::LineEventKind::Rotation => 3,
            crate::chart::event::LineEventKind::Opacity => 4,
            crate::chart::event::LineEventKind::Speed => 5,
        };

        let x = event_timeline_viewport.width() / 5.0 * track as f32
            - event_timeline_viewport.width() / 5.0 / 2.0
            + event_timeline_viewport.min.x;
        let y = timeline.time_to_y(bpm_list.time_at(event.start_beat));

        let size = egui::Vec2::new(
            event_timeline_viewport.width() / 8000.0 * 989.0,
            timeline.duration_to_height(bpm_list.time_at(event.duration())),
        );

        let center = egui::Pos2::new(x, y - size.y / 2.0);

        if event_ui(
            ui,
            egui::Rect::from_center_size(center, size),
            event,
            selected.is_some(),
        )
        .clicked()
        {
            select_events.send(SelectEvent(entity));
        }
    }
}
