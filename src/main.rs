use the_nightmarish_forest::GamePlugin;
use autodefault::autodefault;
use bevy::prelude::*;

#[autodefault]
fn main() -> AppExit {
    // create app
    let mut app = App::new(); // create app

    // add default plugins and setup window
    app.add_plugins( 
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The nightmarish forest".into(),
            })
        }).set(AssetPlugin {
            watch_for_changes_override: Some(true),
        }),
    );

    // add game plugin
    app.add_plugins(GamePlugin);

    // run app
    app.run()
}

