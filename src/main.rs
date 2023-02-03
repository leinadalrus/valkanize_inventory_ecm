use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_kira_audio::prelude::*;
use std::time::Duration;

struct NameHandle {
    name_handle: Handle<String>,
}

struct VisualHandle {
    visual_handle: Handle<Image>,
}

struct OptionHandle {
    option_handle: Option,
}

struct DialogHandler {
    name: NameHandle,
    visual: VisualHandle,
    option: OptionHandle,
} // ,,Handler'' is different from ,,handle''

struct SectionDriver;
struct InterceptEntity;
struct InterceptEntityToken;

pub enum InventoryOptions {
    HeadIndex,
    ChestIndex,
    ArmsIndex,
    LegsIndex,
    ShoesIndex,
    BackIndex,
}

pub struct InventoryData {
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
            inventory_options: InventoryOptions::HeadIndex,
        })
    }
}

/// An observer used to monitor bevy_egui
trait EntityObservation {}

fn contextualize_world_ui(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn serve_asset_resource_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
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
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(contextualize_world_ui)
        .add_system(serve_asset_resource_audio)
        .run();
}
