use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, MeshMaterial2d},
};

use super::{assets::GameAssets, config::GameConfig};

#[derive(Resource)]
pub struct WorldScroll {
    pub speed: f32,
}

impl FromWorld for WorldScroll {
    fn from_world(world: &mut World) -> Self {
        Self {
            speed: world.resource::<GameConfig>().world_scroll_speed,
        }
    }
}

#[derive(Component)]
pub struct ParallaxLayer {
    pub factor: f32,
    pub uv_offset: Vec2,
}

pub fn configure_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = false;
}

pub fn spawn_background(
    mut commands: Commands,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let material = materials.add(BackgroundMaterial {
        color_texture: assets.background_texture.clone(),
        uv_offset: Vec2::ZERO,
    });

    commands.spawn((
        ParallaxLayer {
            factor: config.background_parallax_factor,
            uv_offset: Vec2::ZERO,
        },
        Mesh2d(meshes.add(Rectangle::new(config.canvas_size.x, config.canvas_size.y))),
        MeshMaterial2d(material),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Handle<Image>,
    #[uniform(2)]
    pub uv_offset: Vec2,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}

pub fn update_parallax_offsets(
    mut layers: Query<&mut ParallaxLayer>,
    world_scroll: Res<WorldScroll>,
    time: Res<Time>,
) {
    for mut layer in &mut layers {
        layer.uv_offset.x = advance_parallax_offset(
            layer.uv_offset.x,
            world_scroll.speed,
            layer.factor,
            time.delta_secs(),
        );
    }
}

pub fn sync_parallax_materials(
    layers: Query<(&ParallaxLayer, &MeshMaterial2d<BackgroundMaterial>)>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    for (layer, material_handle) in &layers {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.uv_offset = layer.uv_offset;
        }
    }
}

pub fn advance_parallax_offset(
    current_offset: f32,
    world_scroll_speed: f32,
    parallax_factor: f32,
    delta_secs: f32,
) -> f32 {
    (current_offset + world_scroll_speed * parallax_factor * delta_secs).rem_euclid(1.0)
}

#[cfg(test)]
mod tests {
    use super::advance_parallax_offset;

    #[test]
    fn parallax_offset_advances_by_expected_delta() {
        let offset = advance_parallax_offset(0.25, 200.0, 0.0005, 1.0);
        assert!((offset - 0.35).abs() < f32::EPSILON);
    }

    #[test]
    fn parallax_offset_wraps_back_into_unit_interval() {
        let offset = advance_parallax_offset(0.95, 200.0, 0.0005, 1.0);
        assert!((offset - 0.05).abs() < 0.0001);
    }
}
