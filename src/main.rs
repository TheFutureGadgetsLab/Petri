mod components;
mod rendering;
mod sim;

use bevy::prelude::*;

use crate::{
    rendering::{CellRenderPlugin, GuiRenderPlugins},
    sim::SimPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(GuiRenderPlugins)
        .add_plugin(SimPlugin)
        .add_plugin(CellRenderPlugin)
        .run();
}
