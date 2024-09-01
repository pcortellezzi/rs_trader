use anyhow::Result;
use plugins_interfaces::{Pane, Plugin, PluginCategory, PluginRegistrar};

#[derive(Debug, Copy, Clone, Default)]
pub struct PaneOrders {}

impl PaneOrders {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Pane for PaneOrders {
    fn title(&self) -> String {
        "Orders".to_string()
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Orders");
    }
}

#[derive(Default)]
struct PluginOrders;

impl Plugin for PluginOrders {
    fn name(&self) -> String {
        "Orders".to_string()
    }
    fn new_pane(&self) -> Result<Box<dyn Pane>> {
        Ok(Box::new(PaneOrders::default()))
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginOrders::default()), PluginCategory::Pane);
}
