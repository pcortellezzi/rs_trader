use plugins_interfaces::Pane;

#[derive(Default)]
pub struct TreeBehavior {
    simplification_options: egui_tiles::SimplificationOptions,
}

impl egui_tiles::Behavior<Box<dyn Pane>> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Box<dyn Pane>,
    ) -> egui_tiles::UiResponse {
        pane.ui(ui)
    }

    fn tab_title_for_pane(&mut self, pane: &Box<dyn Pane>) -> egui::WidgetText {
        pane.title().into()
    }

    /*fn top_bar_right_ui(
        &mut self,
        _tiles: &egui_tiles::Tiles<Box<dyn Pane>>,
        gui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        _tabs: &egui_tiles::Tabs,
        _scroll_offset: &mut f32,
    ) {
        if gui.button("➕").clicked() {
            gui.menu_button("Panels")
        }
    }*/

    /*fn on_tab_button(
        &mut self,
        _tiles: &egui_tiles::Tiles<Box<dyn Pane>>,
        tile_id: egui_tiles::TileId,
        button_response: egui::Response,
    ) -> egui::Response {
    }*/

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        self.simplification_options
    }
}
