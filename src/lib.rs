#[cfg(test)]
mod test {
    use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};

    #[derive(Resource, Default)]
    struct TheHandle(Handle<Image>);
    #[test]
    fn test_load_asset() {
        let mut app = App::new();
        app.add_plugins((
            TaskPoolPlugin::default(),
            ScheduleRunnerPlugin::default(),
            LogPlugin::default(),
            AssetPlugin::default(),
            ImagePlugin::default(),
        ));
        app.init_resource::<TheHandle>();
        app.add_systems(
            Startup,
            |server: Res<AssetServer>, mut thehandle: ResMut<TheHandle>| {
                let handle = server.load("test/test.png");
                thehandle.0 = handle;
            },
        );
        app.add_systems(
            Update,
            |server: Res<AssetServer>,
             thehandle: Res<TheHandle>,
             mut commands: Commands,
             images: Res<Assets<Image>>| {
                let image = images.get(thehandle.0.id());
                if let Some(_image) = image {
                    info!("GOT IMAGE!");
                    commands.write_message(AppExit::Success);
                }
                match server.load_state(thehandle.0.id()) {
                    bevy::asset::LoadState::NotLoaded => {
                        warn!("Asset not started to load.");
                    }
                    bevy::asset::LoadState::Loading => {
                        info_once!("loading...");
                    }
                    bevy::asset::LoadState::Loaded => {
                        info!("Loaded!");
                        commands.write_message(AppExit::Success);
                    }
                    bevy::asset::LoadState::Failed(asset_load_error) => {
                        error!(?asset_load_error);
                        commands.write_message(AppExit::error());
                    }
                }
            },
        );
        for _ in 0..5000 {
            app.update();
            if let Some(exit) = app.should_exit() {
                assert!(exit.is_success())
            }
        }
        assert!(false);
    }
}
