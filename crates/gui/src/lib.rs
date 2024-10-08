mod menu;
mod menu_button;
mod panes;
mod tree_behavior;

use egui::{Style, Visuals};
use flume::{unbounded, Receiver, Sender};
use plugins_interfaces::{PluginCategory, PluginRegistrar};

use crate::menu::Menu;
use crate::panes::Panes;

use messaging::{ToBackend, ToFrontend};

/// We derive Deserialize/Serialize so we can persist gui state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    menu: Menu,
    #[serde(skip)]
    panes: Panes,
    #[serde(skip)]
    registrar: PluginRegistrar,
    #[serde(skip)]
    front_tx: Option<Sender<ToBackend>>,
    #[serde(skip)]
    back_rx: Option<Receiver<ToFrontend>>,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, registrar: PluginRegistrar) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut visuals = Visuals::dark();
        visuals.indent_has_left_vline = false;
        cc.egui_ctx.set_style(Style {
            visuals,
            ..Default::default()
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: App = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();

        let (front_tx, front_rx) = unbounded();
        let (back_tx, back_rx) = unbounded();
        app.front_tx = Some(front_tx);
        app.back_rx = Some(back_rx);
        app.registrar = registrar;
        app
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.menu.show(
            ctx,
            frame,
            &mut self.panes,
            self.registrar.get_plugins(&PluginCategory::Pane),
        );
        self.panes.show(ctx);
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
