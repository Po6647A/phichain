use egui::{Color32, Response, Sense, Ui};

pub fn event_ui(ui: &mut Ui, rect: egui::Rect) -> Response {
    let response = ui.allocate_rect(rect, Sense::click());
    if ui.is_rect_visible(rect) {
        ui.painter().rect(
            rect,
            0.0,
            Color32::LIGHT_BLUE,
            egui::Stroke::new(2.0, Color32::WHITE),
        );
    }

    response
}
