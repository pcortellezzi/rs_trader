use anyhow::{Error, Result};
use std::collections::HashMap;
use std::fmt::Formatter;
use libloading::Library;
use regex::Regex;

pub trait Pane {
    fn ui(&mut self, ui: &mut egui::Ui) -> egui_tiles::UiResponse {
        self.show(ui);
        let dragged = ui
            .allocate_rect(ui.max_rect(), egui::Sense::click_and_drag())
            .on_hover_cursor(egui::CursorIcon::Grab)
            .dragged();
        if dragged {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }

    fn title(&self) -> String;
    fn show(&mut self, ui: &mut egui::Ui);
}

pub trait Connector {
    fn name(&self) -> String;
}

pub struct PluginRegistrar {
    plugins: HashMap<PluginCategory, Vec<Box<dyn Plugin>>>,
}

impl core::fmt::Debug for PluginRegistrar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.plugins.get(&PluginCategory::Pane).unwrap().len(), self.plugins.get(&PluginCategory::Connector).unwrap().len())
    }
}

impl Default for PluginRegistrar {
    fn default() -> Self {
        let mut pr = Self {
            plugins: HashMap::new(),
        };
        pr.plugins.insert(PluginCategory::Connector, vec![]);
        pr.plugins.insert(PluginCategory::Pane, vec![]);
        pr
    }
}

/// The `PluginRegistrar` is defined by the application and passed to `plugin_entry`. It's used
/// for a plugin module to register itself with the application.
impl PluginRegistrar {
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>, category: PluginCategory) {
        if self.plugins.contains_key(&category) {
            self.plugins
                .entry(category)
                .and_modify(|plugins| plugins.push(plugin));
        } else {
            self.plugins.insert(category, vec![plugin]);
        }
    }
    pub fn load_plugins(&mut self, path: std::path::PathBuf) {
        let pattern = r"libplugin.*.so$";
        let expression = Regex::new(&pattern).unwrap();
        for entry in std::fs::read_dir(path).unwrap() {
            // NOTE: You need to do something to ensure you're only loading "safe" code. Out of scope
            // for this code.
            let path = entry.unwrap().path();
            let path_str = path.to_str().unwrap();
            //let filename = entry.unwrap().file_name().to_str().unwrap();
            if expression.is_match(path_str) {
                unsafe {
                    let lib = Box::leak(Box::new(Library::new(path.to_str().unwrap()).unwrap()));
                    // In this code, we never close the shared library - if you need to be able to unload the
                    // library, that will require more work.
                    let func: libloading::Symbol<unsafe extern "C" fn(&mut PluginRegistrar) -> ()> =
                        lib.get(b"plugin_entry").unwrap();
                    func(self);
                }
            }
        }
    }

    pub fn get_plugins(&self, category: &PluginCategory) -> &Vec<Box<dyn Plugin>> {
        self.plugins.get(category).unwrap()
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PluginCategory {
    Pane,
    Connector,
}

/// `Plugin` is implemented by a plugin library for one or more types. As you need additional
/// callbacks, they can be defined here. These are first class Rust trait objects, so you have the
/// full flexibility of that system. The main thing you'll lose access to is generics, but that's
/// expected with a plugin system
pub trait Plugin {
    fn name(&self) -> String;

    fn new_pane(&self) -> Result<Box<dyn Pane>> {
        Err(Error::msg("Not implemented"))
    }
    fn new_connector(&self) -> Result<Box<dyn Connector>> {
        Err(Error::msg("Not implemented"))
    }
}
