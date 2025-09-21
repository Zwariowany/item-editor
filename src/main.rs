use eframe::egui;
use eframe::egui::{CollapsingHeader, Id, Ui};

use egui_dnd::{DragDropItem, Handle, dnd};

pub fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("DnD", options, Box::new(|_cc| Ok(Box::<MyApp>::default())))
}

#[derive(Default)]
struct Tag {
    name: String,

    children: Option<Vec<Tag>>,
}

impl DragDropItem for &mut Tag {
    fn id(&self) -> Id {
        Id::new(&self.name)
    }
}

struct MyApp {
    items: Vec<Tag>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            items: vec![
                Tag {
                    name: "Material".to_string(),
                    ..Tag::default()
                },
                Tag {
                    name: "Food".to_string(),
                    ..Tag::default()
                },
                Tag {
                    name: "Weapon".to_string(),
                    ..Tag::default()
                },
                Tag {
                    name: "Armor".to_string(),
                    children: Some(vec![
                        Tag {
                            name: "Helmet".to_string(),
                            ..Tag::default()
                        },
                        Tag {
                            name: "Chest".to_string(),
                            ..Tag::default()
                        },
                        Tag {
                            name: "Leggins".to_string(),
                            ..Tag::default()
                        },
                        Tag {
                            name: "Boots".to_string(),
                            ..Tag::default()
                        },
                    ]),
                },
            ],
        }
    }
}

impl MyApp {
    fn draw_item(ui: &mut Ui, item: &mut Tag, handle: Handle) {
        handle.ui(ui, |ui| {
            if let Some(children) = &mut item.children {
                CollapsingHeader::new(&item.name)
                    .default_open(true)
                    .show(ui, |ui| {
                        let response = dnd(ui, "dnd_shared_id").show(
                            children.iter_mut(),
                            |ui, item, handle, _pressed| {
                                Self::draw_item(ui, item, handle);
                            },
                        );

                        response.update_vec(children);
                    });
            } else {
                let response = ui.selectable_label(false, &item.name);

                if response.hovered() {
                    response.highlight();
                }
            }
        });

        // if let Some(children) = &mut item.children {
        //     CollapsingHeader::new("children")
        //         .default_open(true)
        //         .show(ui, |ui| {
        //             ui.label("Content");
        //
        //             let response = dnd(ui, &item.name).show(
        //                 children.iter_mut(),
        //                 |ui, item, handle, _pressed| {
        //                     Self::draw_item(ui, item, handle);
        //                 },
        //             );
        //
        //             response.update_vec(children);
        //         });
        // }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response =
                dnd(ui, "dnd_example").show(self.items.iter_mut(), |ui, item, handle, _pressed| {
                    MyApp::draw_item(ui, item, handle);
                });
            response.update_vec(&mut self.items);
        });
    }
}
