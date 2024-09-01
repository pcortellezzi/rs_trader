use anyhow::Result;
use plugins_interfaces::{Pane, Plugin, PluginCategory, PluginRegistrar};

#[derive(Debug, Copy, Clone, Default)]
pub struct PanePositions {}

impl PanePositions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Pane for PanePositions {
    fn title(&self) -> String {
        "Positions".to_string()
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Positions");
    }
}

#[derive(Default)]
struct PluginPositions;

impl Plugin for PluginPositions {
    fn name(&self) -> String {
        "Positions".to_string()
    }
    fn new_pane(&self) -> Result<Box<dyn Pane>> {
        Ok(Box::new(PanePositions::default()))
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginPositions::default()), PluginCategory::Pane);
}
