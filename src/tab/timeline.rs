use bevy::{ecs::system::SystemParam, prelude::*};
use egui::{Color32, Ui};
use url::Url;

use crate::{
    chart::{
        beat::Beat,
        event::LineEvent,
        note::{Note, NoteKind},
    },
    constants::{BASE_ZOOM, CANVAS_WIDTH, INDICATOR_POSITION},
    misc::WorkingDirectory,
    selection::{SelectNoteEvent, Selected, SelectedLine},
    timing::{BpmList, ChartTime},
};

pub struct TimelineTabPlugin;

impl Plugin for TimelineTabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimelineViewport(Rect::from_corners(Vec2::ZERO, Vec2::ZERO)))
            .insert_resource(TimelineSettings::default());
    }
}

pub fn timeline_ui_system(
    In(ui): In<&mut Ui>,
    selected_line_query: Res<SelectedLine>,
    timeline_viewport: Res<TimelineViewport>,
    bpm_list: Res<BpmList>,
    event_query: Query<(&LineEvent, &Parent)>,
    note_query: Query<(&Note, &Parent, Entity, Option<&Selected>)>,
    working_dir: Res<WorkingDirectory>,
    mut select_events: EventWriter<SelectNoteEvent>,
    timeline: Timeline,
) {
    let selected_line = selected_line_query.0;
    let viewport = timeline_viewport;

    ui.painter().rect_filled(
        egui::Rect::from_center_size(
            egui::Pos2::new(
                viewport.0.width() / 2.0 + viewport.0.min.x,
                viewport.0.height() * INDICATOR_POSITION,
            ),
            egui::Vec2::new(viewport.0.width(), 2.0),
        ),
        0.0,
        Color32::WHITE,
    );

    let event_timeline_viewport = viewport.event_timeline_viewport();

    for (event, parent) in event_query.iter() {
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

        ui.painter().rect(
            egui::Rect::from_center_size(center, size),
            0.0,
            Color32::LIGHT_BLUE,
            egui::Stroke::new(2.0, Color32::WHITE),
        );
    }

    let note_timeline_viewport = viewport.note_timeline_viewport();

    for (note, parent, entity, selected) in note_query.iter() {
        if parent.get() != selected_line {
            continue;
        }

        let x = (note.x / CANVAS_WIDTH + 0.5) * note_timeline_viewport.width();
        let y = timeline.time_to_y(bpm_list.time_at(note.beat));

        let image = match note.kind {
            NoteKind::Tap => "tap.png",
            NoteKind::Drag => "drag.png",
            NoteKind::Hold { hold_beat: _ } => "hold.png",
            NoteKind::Flick => "flick.png",
        };

        let image_size = match note.kind {
            NoteKind::Tap => egui::Vec2::new(989.0, 100.0),
            NoteKind::Drag => egui::Vec2::new(989.0, 60.0),
            NoteKind::Hold { hold_beat: _ } => egui::Vec2::new(989.0, 1900.0),
            NoteKind::Flick => egui::Vec2::new(989.0, 200.0),
        };

        let size = match note.kind {
            NoteKind::Hold { hold_beat } => egui::Vec2::new(
                note_timeline_viewport.width() / 8000.0 * image_size.x,
                timeline.duration_to_height(bpm_list.time_at(hold_beat)),
            ),
            _ => egui::Vec2::new(
                note_timeline_viewport.width() / 8000.0 * image_size.x,
                note_timeline_viewport.width() / 8000.0 * image_size.y,
            ),
        };

        let center = match note.kind {
            NoteKind::Hold { hold_beat: _ } => egui::Pos2::new(x, y - size.y / 2.0),
            _ => egui::Pos2::new(x, y),
        };

        let assets_dir = working_dir.0.join("assets");

        let response = ui.put(
            egui::Rect::from_center_size(center, size),
            egui::Image::new(
                Url::from_file_path(assets_dir.join(image))
                    .unwrap()
                    .as_str(),
            )
            .maintain_aspect_ratio(false)
            .fit_to_exact_size(size)
            .tint(if selected.is_some() {
                Color32::LIGHT_GREEN
            } else {
                Color32::WHITE
            })
            .sense(egui::Sense::click()),
        );

        if response.clicked() {
            select_events.send(SelectNoteEvent(entity));
        }
    }

    for beat_time in timeline.primary_beat_times() {
        ui.painter().rect_filled(
            egui::Rect::from_center_size(
                egui::Pos2::new(
                    viewport.0.width() / 2.0 + viewport.0.min.x,
                    timeline.time_to_y(beat_time),
                ),
                egui::Vec2::new(viewport.0.width(), 2.0),
            ),
            0.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 40),
        );
    }

    for beat_time in timeline.secondary_beat_times() {
        ui.painter().rect_filled(
            egui::Rect::from_center_size(
                egui::Pos2::new(
                    viewport.0.width() / 2.0 + viewport.0.min.x,
                    timeline.time_to_y(beat_time),
                ),
                egui::Vec2::new(viewport.0.width(), 0.5),
            ),
            0.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 40),
        );
    }
}

#[derive(Resource, Debug)]
pub struct TimelineViewport(pub Rect);

impl TimelineViewport {
    pub fn note_timeline_viewport(&self) -> Rect {
        Rect::from_corners(
            self.0.min,
            Vec2 {
                x: self.0.min.x + self.0.width() / 3.0 * 2.0,
                y: self.0.max.y,
            },
        )
    }

    pub fn event_timeline_viewport(&self) -> Rect {
        Rect::from_corners(
            Vec2 {
                x: self.0.min.x + self.0.width() / 3.0 * 2.0,
                y: self.0.min.y,
            },
            self.0.max,
        )
    }
}

#[derive(Resource)]
pub struct TimelineSettings {
    zoom: f32,
    density: f32,
}

impl Default for TimelineSettings {
    fn default() -> Self {
        Self {
            zoom: 2.0,
            density: 4.0,
        }
    }
}

#[derive(SystemParam)]
pub struct Timeline<'w> {
    bpm_list: Res<'w, BpmList>,
    timeline_settings: Res<'w, TimelineSettings>,
    current_time: Res<'w, ChartTime>,
    viewport: Res<'w, TimelineViewport>,
}

impl<'w> Timeline<'w> {
    pub fn primary_beat_times(&self) -> Vec<f32> {
        let audio_duration = 240.0; // TODO: replace with actual audio duration

        let interval = self.bpm_list.time_at(Beat::ONE);

        std::iter::repeat(0)
            .take((audio_duration / interval).round() as usize)
            .enumerate()
            .map(|(i, _)| i as f32 * interval)
            .collect()
    }

    pub fn secondary_beat_times(&self) -> Vec<f32> {
        let audio_duration = 240.0; // TODO: replace with actual audio duration

        let interval = self
            .bpm_list
            .time_at(Beat::from(1.0 / self.timeline_settings.density));

        std::iter::repeat(0)
            .take((audio_duration / interval).round() as usize)
            .enumerate()
            .map(|(i, _)| i as f32 * interval)
            .collect()
    }

    pub fn time_to_y(&self, time: f32) -> f32 {
        (self.current_time.0 - time) * BASE_ZOOM * self.timeline_settings.zoom + self.viewport.0.height() * INDICATOR_POSITION
    }

    pub fn y_to_time(&self, y: f32) -> f32 {
        self.current_time.0 - (y - self.viewport.0.height() * INDICATOR_POSITION) / (BASE_ZOOM * self.timeline_settings.zoom)
    }

    pub fn duration_to_height(&self, duration: f32) -> f32 {
        duration * BASE_ZOOM * self.timeline_settings.zoom
    }
}
