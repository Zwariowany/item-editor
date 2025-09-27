use eframe::egui;
use eframe::egui::{Id, Ui};

use egui::{Direction, Layout, ScrollArea, SidePanel, TextEdit, TopBottomPanel};
use egui_dnd::{DragDropItem, Handle, dnd};

pub fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("DnD", options, Box::new(|_cc| Ok(Box::<MyApp>::default())))
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
    items: Vec<Item>,
    available_id: u16,
}

impl MyApp {
    fn draw_item_label(ui: &mut Ui, item: &mut Item, handle: Handle) {
        handle.ui(ui, |ui| {
            let response = ui.selectable_label(false, &item.name);

            if response.hovered() {
                response.highlight();
            }
        });
    }

    fn add_item(&mut self) {
        self.items.push(Item::new(self.available_id));
        self.available_id += 1;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::new(egui::panel::TopBottomSide::Top, "Header").show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.label(egui::RichText::new("Item editor").heading());
                },
            )
        });

        SidePanel::left("Item view")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Items");
                    ui.separator();
                });

                let mut input = String::new();
                let text_edit = TextEdit::singleline(&mut input).hint_text("Search your items");
                ui.add(text_edit);

                ScrollArea::vertical().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height() - 32.0);
                    let response = dnd(ui, "root").show(
                        self.items.iter_mut(),
                        |ui, item, handle, _pressed| {
                            MyApp::draw_item_label(ui, item, handle);
                        },
                    );
                    response.update_vec(&mut self.items);
                });

                ui.separator();

                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        let add_item_button = ui.button("Add new item");

                        if add_item_button.clicked() {
                            self.add_item();
                            // self.items.push(Item::default());
                        }
                    },
                );

                Layout::horizontal_align
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Here edit your item");
            // let response =
            //     dnd(ui, "root").show(self.items.iter_mut(), |ui, item, handle, _pressed| {
            //         MyApp::draw_item(ui, item, handle);
            //     });
            // response.update_vec(&mut self.items);
        });
    }
}
