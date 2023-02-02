use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

struct InteractionPanelAPI;
struct TextPanelAPI;
struct OptionPanelAPI;
struct VisualPanelAPI;
struct SectionDriverAPI;
struct InterceptorAPI;
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
        EntityComponentInventory(InventoryData { inventory_options: InventoryOptions::HeadIndex })
    }
}

/// An observer used to monitor bevy_egui
trait EntityObservation {}

fn contextualize_world_ui(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(contextualize_world_ui)
        .run();
}
