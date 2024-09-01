use anyhow::Result;
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;

use plugins_interfaces::{Pane, Plugin, PluginCategory, PluginRegistrar};

#[derive(Default)]
struct PaneConnections {
    connections: Vec<backend::Connection>,
}

impl Pane for PaneConnections {
    fn title(&self) -> String {
        "Connections".to_string()
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let _ = TableBuilder::new(ui)
            //.striped(self.striped)
            //.resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .sense(egui::Sense::click())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Name");
                });
                header.col(|ui| {
                    ui.strong("Connector");
                });
            })
            .body(|mut body| {
                let row_height = 18.0;
                let len = self.connections.len();
                for i in 0..len {
                    body.row(row_height, |mut row| {
                        //row.set_selected(self.selection.contains(&row_index));

                        row.col(|ui| {
                            ui.label(self.connections[i].name.clone());
                        });
                        row.col(|ui| {
                            ui.label(self.connections[i].connector.name());
                        });
                        row.col(|ui| {
                            if ui.button("🗑").clicked() {
                                self.connections.remove(i);
                            }
                        });
                        //self.toggle_row_selection(row_index, &row.response());
                    });
                }
                body.row(row_height, |mut row| {
                    //row.set_selected(self.selection.contains(&row_index));

                    row.col(|ui| {});
                    row.col(|ui| {});
                    row.col(|ui| if ui.button("➕").clicked() {});

                    //self.toggle_row_selection(row_index, &row.response());
                });
            });
    }
}

#[derive(Default)]
struct PluginConnections;

impl Plugin for PluginConnections {
    fn name(&self) -> String {
        "Connections".to_string()
    }
    fn new_pane(&self) -> Result<Box<dyn Pane>> {
        Ok(Box::new(PaneConnections::default()))
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginConnections::default()), PluginCategory::Pane);
}
