//! Re:PhiEdit json format

use crate::chart::easing::Easing;
use crate::chart::event::{LineEvent, LineEventKind};
use crate::format::Format;
use crate::serialization::{LineWrapper, PhiChainChart};
use num::Rational32;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum NoteKind {
    #[default]
    Tap = 1,
    Drag = 4,
    Hold = 3,
    Flick = 2,
}

// generated by https://transform.tools/json-to-rust-serde
// TODO: event layer, easing, parent support
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Beat(i32, i32, i32);

impl From<Beat> for crate::chart::beat::Beat {
    fn from(val: Beat) -> Self {
        crate::chart::beat::Beat::new(val.0, Rational32::new(val.1, val.2))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpeChart {
    #[serde(rename = "BPMList")]
    bpm_list: Vec<BpmPoint>,
    #[serde(rename = "META")]
    meta: Meta,
    judge_line_group: Vec<String>,
    judge_line_list: Vec<JudgeLine>,
    multi_line_string: String,
    multi_scale: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BpmPoint {
    bpm: f32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    #[serde(rename = "RPEVersion")]
    rpeversion: i32,
    background: String,
    charter: String,
    composer: String,
    id: String,
    level: String,
    name: String,
    offset: i32,
    song: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JudgeLine {
    #[serde(rename = "Group")]
    group: i32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Texture")]
    texture: String,
    alpha_control: Vec<AlphaControl>,
    bpmfactor: f32,
    event_layers: Vec<EventLayer>,
    extended: Extended,
    father: i32,
    is_cover: i32,
    #[serde(default)]
    notes: Vec<Note>,
    num_of_notes: i32,
    pos_control: Vec<PosControl>,
    size_control: Vec<SizeControl>,
    skew_control: Vec<SkewControl>,
    y_control: Vec<YControl>,
    z_order: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlphaControl {
    alpha: f32,
    easing: i32,
    x: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventLayer {
    #[serde(default)]
    alpha_events: Vec<AlphaEvent>,
    #[serde(rename = "moveXEvents")]
    move_xevents: Vec<MoveXevent>,
    #[serde(rename = "moveYEvents")]
    move_yevents: Vec<MoveYevent>,
    #[serde(default)]
    rotate_events: Vec<RotateEvent>,
    #[serde(default)]
    speed_events: Vec<SpeedEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlphaEvent {
    bezier: i32,
    bezier_points: Vec<f32>,
    easing_left: f32,
    easing_right: f32,
    easing_type: i32,
    end: i32,
    end_time: Beat,
    linkgroup: i32,
    start: i32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoveXevent {
    bezier: i32,
    bezier_points: Vec<f32>,
    easing_left: f32,
    easing_right: f32,
    easing_type: i32,
    end: f32,
    end_time: Beat,
    linkgroup: i32,
    start: f32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoveYevent {
    bezier: i32,
    bezier_points: Vec<f32>,
    easing_left: f32,
    easing_right: f32,
    easing_type: i32,
    end: f32,
    end_time: Beat,
    linkgroup: i32,
    start: f32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RotateEvent {
    bezier: i32,
    bezier_points: Vec<f32>,
    easing_left: f32,
    easing_right: f32,
    easing_type: i32,
    end: f32,
    end_time: Beat,
    linkgroup: i32,
    start: f32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpeedEvent {
    end: f32,
    end_time: Beat,
    linkgroup: i32,
    start: f32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extended {
    incline_events: Vec<InclineEvent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InclineEvent {
    bezier: i32,
    bezier_points: Vec<f32>,
    easing_left: f32,
    easing_right: f32,
    easing_type: i32,
    end: f32,
    end_time: Beat,
    linkgroup: i32,
    start: f32,
    start_time: Beat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Note {
    above: i32,
    alpha: i32,
    end_time: Beat,
    is_fake: i32,
    position_x: f32,
    size: f32,
    speed: f32,
    start_time: Beat,
    #[serde(rename = "type")]
    kind: NoteKind,
    visible_time: f32,
    y_offset: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PosControl {
    easing: i32,
    pos: f32,
    x: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SizeControl {
    easing: i32,
    size: f32,
    x: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkewControl {
    easing: i32,
    skew: f32,
    x: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YControl {
    easing: i32,
    x: f32,
    y: f32,
}

impl Format for RpeChart {
    fn into_phichain(self) -> anyhow::Result<PhiChainChart> {
        let mut bpm_list = crate::timing::BpmList::new(
            self.bpm_list
                .iter()
                .map(|x| crate::timing::BpmPoint::new(x.start_time.clone().into(), x.bpm))
                .collect(),
        );
        bpm_list.compute();
        let mut phichain = PhiChainChart::new(self.meta.offset as f32, bpm_list, vec![]);

        for line in self.judge_line_list {
            let x_event_iter = line
                .event_layers
                .iter()
                .flat_map(|layer| layer.move_xevents.clone())
                .map(|event| LineEvent {
                    kind: LineEventKind::X,
                    start: event.start,
                    end: event.end,
                    start_beat: event.start_time.into(),
                    end_beat: event.end_time.into(),
                    easing: Easing::Linear,
                });
            let y_event_iter = line
                .event_layers
                .iter()
                .flat_map(|layer| layer.move_yevents.clone())
                .map(|event| LineEvent {
                    kind: LineEventKind::Y,
                    start: event.start,
                    end: event.end,
                    start_beat: event.start_time.into(),
                    end_beat: event.end_time.into(),
                    easing: Easing::Linear,
                });
            let rotate_event_iter = line
                .event_layers
                .iter()
                .flat_map(|layer| layer.rotate_events.clone())
                .map(|event| LineEvent {
                    kind: LineEventKind::Rotation,
                    start: event.start,
                    end: event.end,
                    start_beat: event.start_time.into(),
                    end_beat: event.end_time.into(),
                    easing: Easing::Linear,
                });
            let alpha_event_iter = line
                .event_layers
                .iter()
                .flat_map(|layer| layer.alpha_events.clone())
                .map(|event| LineEvent {
                    kind: LineEventKind::Opacity,
                    start: event.start as f32,
                    end: event.end as f32,
                    start_beat: event.start_time.into(),
                    end_beat: event.end_time.into(),
                    easing: Easing::Linear,
                });
            let speed_event_iter = line
                .event_layers
                .iter()
                .flat_map(|layer| layer.speed_events.clone())
                .map(|event| LineEvent {
                    kind: LineEventKind::Speed,
                    start: event.start,
                    end: event.end,
                    start_beat: event.start_time.into(),
                    end_beat: event.end_time.into(),
                    easing: Easing::Linear,
                });

            phichain.lines.push(LineWrapper(
                line.notes
                    .iter()
                    .map(|note| {
                        let start_beat = crate::chart::beat::Beat::from(note.start_time.clone());
                        let end_beat = crate::chart::beat::Beat::from(note.end_time.clone());
                        let kind: crate::chart::note::NoteKind = match note.kind {
                            NoteKind::Tap => crate::chart::note::NoteKind::Tap,
                            NoteKind::Drag => crate::chart::note::NoteKind::Drag,
                            NoteKind::Hold => crate::chart::note::NoteKind::Hold {
                                hold_beat: end_beat - start_beat,
                            },
                            NoteKind::Flick => crate::chart::note::NoteKind::Flick,
                        };

                        crate::chart::note::Note::new(
                            kind,
                            note.above == 1,
                            start_beat,
                            note.position_x,
                        )
                    })
                    .collect(),
                x_event_iter
                    .chain(y_event_iter)
                    .chain(rotate_event_iter)
                    .chain(alpha_event_iter)
                    .chain(speed_event_iter)
                    .collect(),
            ));
        }

        Ok(phichain)
    }

    fn from_phichain(_phichain: PhiChainChart) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        unimplemented!("");
    }
}
