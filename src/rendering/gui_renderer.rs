use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::*,
};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

const BEVY_TEXTURE_ID: u64 = 0;

pub struct GuiRenderPlugins;

impl PluginGroup for GuiRenderPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(EguiPlugin);
        group.add(GuiRenderPlugin);
    }
}

struct GuiRenderPlugin;

impl Plugin for GuiRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_system(update_ui_scale_factor)
            .add_system(ui_example);
    }
}

#[derive(Default)]
struct UiState {
    label: String,
    value: f32,
    inverted: bool,
}

fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

fn ui_example(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<UiState>, assets: Res<AssetServer>) {
    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
            });

            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID),
                [256.0, 256.0],
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    if invert {
        ui_state.inverted = !ui_state.inverted;
    }
    if load || invert {
        let texture_handle = if ui_state.inverted {
            assets.load("icon_inverted.png")
        } else {
            assets.load("icon.png")
        };
        egui_ctx.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
    }
    if remove {
        egui_ctx.remove_egui_texture(BEVY_TEXTURE_ID);
    }
}
