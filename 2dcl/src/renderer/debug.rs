use bevy::prelude::*; 
use bevy_inspector_egui::RegisterInspectable;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::renderer::player::Player;

use super::scene_deserializer::CircleCollider;
use super::scene_deserializer::BoxCollider;
use super::scene_deserializer::AlphaCollider;

pub struct DebugPlugin;

impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App)
    {
        if cfg!(debug_assertions)
        {   
 
            app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<CircleCollider>()
            .register_inspectable::<BoxCollider>()
            .register_inspectable::<AlphaCollider>()
            .register_inspectable::<Player>();
        }
    }    
}