use crate::menu::MenuButton;
use crate::panes::Panes;

use plugins_interfaces::Plugin;

pub struct PanesMenuButton;

impl MenuButton for PanesMenuButton {
    fn menu_title(&self) -> String {
        "Panels".to_string()
    }
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes, plugins: &Vec<Box<dyn Plugin>>) {
        for plugin in plugins {
            if ui.button(plugin.name()).clicked() {
                match plugin.new_pane() {
                    Ok(pane) => {
                        panes.new_pane(pane)
                    },
                    Err(e) => log::error!("{}", e),
                }

                ui.close_menu();
            }
        }
    }
}
