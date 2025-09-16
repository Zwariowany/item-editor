use eframe::egui;
use egui::{Button, CentralPanel, TopBottomPanel, Vec2};

struct Editor {
    items: Vec<String>,
    from: Option<usize>,
    to: Option<usize>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            items: vec!["A", "B", "C", "D", "E", "F"]
                .into_iter()
                .map(|i| i.to_string())
                .collect(),
            from: None,
            to: None,
        }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("Header").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("This is imple example of drag and drop in egui.");
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                for (id, item) in self.items.iter().enumerate() {
                    let item_id = egui::Id::new((id, item.clone()));
                    let response = ui
                        .dnd_drag_source(item_id, id, |ui| {
                            ui.add_sized(Vec2::splat(64.0), Button::selectable(false, item));
                        })
                        .response;

                    if let (Some(pointer), Some(hovered_payload)) = (
                        ui.input(|i| i.pointer.interact_pos()),
                        response.dnd_hover_payload::<usize>(),
                    ) {
                        // INFO: Here implement visual where payload put
                        let rect = response.rect;
                    }

                    if let Some(dragged_payload) = response.dnd_release_payload::<usize>() {
                        self.from = Some(*dragged_payload);
                        self.to = Some(id);
                    }

                    if response.hovered() {
                        response.highlight();
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Editor",
        options,
        Box::new(|_cc| Ok(Box::new(Editor::default()))),
    )
}
