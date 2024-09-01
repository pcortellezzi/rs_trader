use egui_tiles::{Container, Tile, Tree};

use crate::tree_behavior::*;

use plugins_interfaces::Pane;

pub struct Panes {
    tree: Tree<Box<dyn Pane>>,
    behavior: TreeBehavior,
}

impl Default for Panes {
    fn default() -> Self {
        Self {
            tree: Tree::empty("main_tree"),
            behavior: Default::default(),
        }
    }
}

impl Panes {
    pub fn new_pane(&mut self, pane: Box<dyn Pane>) {
        let pane = self.tree.tiles.insert_pane(pane);
        if let Some(rootid) = &self.tree.root {
            let root = self.tree.tiles.get_mut(*rootid);
            if let Some(Tile::Container(container)) = root {
                container.add_child(pane);
                if let Container::Tabs(tabs) = container {
                    tabs.set_active(pane);
                }
            } else {
                self.tree.root = Some(self.tree.tiles.insert_tab_tile(vec![pane, *rootid]));
            }
        } else {
            let children = vec![pane];
            self.tree.root = Some(self.tree.tiles.insert_tab_tile(children));
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree.ui(&mut self.behavior, ui);
        });
    }
}
