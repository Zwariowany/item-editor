use std::collections::HashMap;

use eframe::egui;
use eframe::egui::Id;

use egui::{
    Align, CentralPanel, Context, Direction, Layout, ScrollArea, SidePanel, Stroke, TextEdit,
    TopBottomPanel,
};
use egui_dnd::DragDropItem;

pub fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Item editor",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct Item {
    id: u16,
    name: String,
    description: String,
}

impl Item {
    fn new(id: u16) -> Item {
        Item {
            id,
            name: String::from("Item"),
            description: String::from("Brilliant description"),
        }
    }
}

impl DragDropItem for &mut Item {
    fn id(&self) -> Id {
        Id::new(self.id)
    }
}

#[derive(Default)]
struct MyApp {
    items: HashMap<u16, Item>,
    available_id: u16,
    selected_item_id: Option<u16>,
}

impl MyApp {
    fn add_item(&mut self) {
        self.items
            .insert(self.available_id, Item::new(self.available_id));
        self.available_id += 1;
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("Item view")
            .min_width(150.0)
            .resizable(true)
            .show(ctx, |ui| {
                TopBottomPanel::top("Left header")
                    .resizable(false)
                    .frame(
                        egui::Frame::default()
                            .inner_margin(0.0)
                            .stroke(Stroke::NONE),
                    )
                    .show_inside(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Items");
                        });

                        let mut input = String::new();
                        let text_edit =
                            TextEdit::singleline(&mut input).hint_text("Search your items");
                        ui.add(text_edit);
                        ui.separator();
                    });

                TopBottomPanel::bottom("new item button")
                    .resizable(false)
                    .min_height(24.0)
                    .frame(egui::Frame::default().inner_margin(0.0).outer_margin(0.0))
                    .show_inside(ui, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                            if ui.button("Add new item").clicked() {
                                self.add_item();
                            }
                        });
                    });

                CentralPanel::default()
                    .frame(egui::Frame::default().stroke(Stroke::NONE))
                    .show_inside(ui, |ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            for item in self.items.values() {
                                ui.vertical_centered(|ui| {
                                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                                        let response = ui.selectable_label(false, &item.name);

                                        if response.clicked() {
                                            self.selected_item_id = Some(item.id);
                                        }
                                    });
                                });
                            }
                        });
                    });
            });
    }

    fn header(&self, ctx: &egui::Context) {
        TopBottomPanel::new(egui::panel::TopBottomSide::Top, "Header").show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.label(egui::RichText::new("Item editor").heading());
                },
            )
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            if let Some(item_id) = self.selected_item_id {
                self.items.entry(item_id).and_modify(|item| {
                    let name_input = TextEdit::singleline(&mut item.name);
                    let description_input = TextEdit::multiline(&mut item.description);

                    ui.add(name_input);
                    ui.add(description_input);
                });
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.header(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
    }
}
