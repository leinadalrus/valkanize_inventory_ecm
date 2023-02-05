use bevy::{prelude::*, reflect::TypeUuid};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_kira_audio::prelude::*;
use std::time::Duration;
use uuid::uuid;

#[derive(TypeUuid)]
#[uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8"]
struct UntypedIdentifier(pub String);

struct NameHandle(pub Handle<UntypedIdentifier>);

struct VisualHandle(pub Handle<Image>);

struct OptionHandle(pub Option<String>);

pub struct DialogHandler {
    name: NameHandle,
    visual: VisualHandle,
    option: OptionHandle,
} // ,,Handler'' is different from ,,handle''

#[derive(Default, Resource)]
pub struct DialogState {
    label: String,
    value: String,
    texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
}

pub struct SectionDriver;
pub struct InterceptEntity;
pub struct InterceptEntityToken;

pub enum InventoryOptions {
    HeadIndex,
    ChestIndex,
    ArmsIndex,
    LegsIndex,
    ShoesIndex,
    BackIndex,
}

#[derive(Resource)]
pub struct InventoryData {
    id: String,
    inventory_options: InventoryOptions,
}

impl InventoryData {
    fn inventory_data(self: Self) -> Self {
        self
    }
}

pub struct EntityComponentInventory(InventoryData);

impl EntityComponentInventory {
    fn new(self: Self) -> Self {
        EntityComponentInventory(InventoryData {
            id: String::from("dialog/handles/named_resources_data.json"),
            inventory_options: InventoryOptions::HeadIndex,
        })
    }
}

impl FromWorld for DialogHandler {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let name = NameHandle(asset_server.load("dialog/handles/named_resources_data.json"));
        let visual = VisualHandle(asset_server.load("images/icons/user_icon.png"));
        let option = OptionHandle(None);
        Self {
            name,
            visual,
            option,
        }
    }
}

fn start_system_setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font_handle: bevy::prelude::Handle<Font> = asset_server.load("fonts/FiraSans-Retina.ttf");

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });
}

fn contextualize_world_ui(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn configure_visuals_system(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn configure_ui_state_system(mut ui_state: ResMut<DialogState>) {
    ui_state.is_window_open = true;
}

fn update_ui_scale_factor_system(
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

fn serve_asset_resource_audio(asset_server: Res<AssetServer>, audio: Res<bevy_kira_audio::Audio>) {
    audio
        .play(asset_server.load("audio/music/background_elevator_music.flac"))
        .loop_from(0.3)
        // Fade-in with a dynamic easing
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        // Only play on our right ear
        .with_panning(1.0)
        // Increase playback rate by 50% (this also increases the pitch)
        .with_playback_rate(1.5)
        // Play at half volume
        .with_volume(0.5);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<DialogState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_startup_system(start_system_setup_ui)
        .add_system(contextualize_world_ui)
        .add_system(configure_visuals_system)
        .add_system(configure_ui_state_system)
        .add_system(update_ui_scale_factor_system)
        .add_system(serve_asset_resource_audio)
        .run();
}
