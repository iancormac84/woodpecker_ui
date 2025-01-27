use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use woodpecker_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WoodpeckerUIPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut ui_context: ResMut<WoodpeckerContext>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let root = commands
        .spawn((
            WoodpeckerAppBundle::default(),
            WidgetRender::Svg {
                handle: asset_server.load("woodpecker_svg/woodpecker.svg"),
                color: Some(Srgba::GREEN.into()),
            },
        ))
        .id();
    ui_context.set_root_widget(root);
}
