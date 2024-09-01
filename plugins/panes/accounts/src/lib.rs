use anyhow::Result;
use plugins_interfaces::{Pane, Plugin, PluginCategory, PluginRegistrar};

#[derive(Debug, Copy, Clone, Default)]
pub struct PaneAccounts {}

impl PaneAccounts {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Pane for PaneAccounts {
    fn title(&self) -> String {
        "Accounts".to_string()
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Accounts");
    }
}

#[derive(Default)]
struct PluginAccounts;

impl Plugin for PluginAccounts {
    fn name(&self) -> String {
        "Accounts".to_string()
    }
    fn new_pane(&self) -> Result<Box<dyn Pane>> {
        Ok(Box::new(PaneAccounts::default()))
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginAccounts::default()), PluginCategory::Pane);
}
