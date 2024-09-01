use crate::panes::Panes;

use crate::menu_button::PanesMenuButton;
use plugins_interfaces::Plugin;

pub trait MenuButton {
    fn menu_title(&self) -> String;
    fn show_button(&self, ui: &mut egui::Ui, panes: &mut Panes, plugins: &Vec<Box<dyn Plugin>>);

    fn menu_button(&self, ui: &mut egui::Ui, panes: &mut Panes, plugins: &Vec<Box<dyn Plugin>>) {
        ui.menu_button(self.menu_title(), |ui| self.show_button(ui, panes, plugins));
    }
}

pub struct Menu {
    buttons: Vec<Box<dyn MenuButton>>,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::new()
    }
}

impl Menu {
    pub fn new() -> Self {
        let buttons: Vec<Box<dyn MenuButton>> = vec![Box::new(PanesMenuButton)];
        Self { buttons }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        panes: &mut Panes,
        plugins: &Vec<Box<dyn Plugin>>,
    ) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                for button in self.buttons.iter_mut() {
                    button.menu_button(ui, panes, plugins);
                }
            });
        });
    }
}
