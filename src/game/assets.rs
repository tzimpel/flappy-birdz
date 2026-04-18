use bevy::{
    image::{ImageAddressMode, ImageFilterMode, ImageLoaderSettings},
    prelude::*,
};

#[derive(Resource, Clone)]
pub struct GameAssets {
    pub bird_image: Handle<Image>,
    pub pipe_image: Handle<Image>,
    pub background_texture: Handle<Image>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            bird_image: asset_server.load("bevy-bird.png"),
            pipe_image: asset_server.load_with_settings(
                "pipe.png",
                |settings: &mut ImageLoaderSettings| {
                    settings
                        .sampler
                        .get_or_init_descriptor()
                        .set_filter(ImageFilterMode::Nearest);
                },
            ),
            background_texture: asset_server.load_with_settings(
                "background_color_grass.png",
                |settings: &mut ImageLoaderSettings| {
                    settings
                        .sampler
                        .get_or_init_descriptor()
                        .set_address_mode(ImageAddressMode::Repeat);
                },
            ),
        }
    }
}
