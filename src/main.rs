//! Pong Tutorial 1

use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let mut world = World::new();
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = app_root.join("examples/assets/");

    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and
            // drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?
    .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}
