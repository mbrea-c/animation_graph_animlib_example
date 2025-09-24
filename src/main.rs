use bevy::app::App;
use bevy_animation_graph_editor::AnimationGraphEditorPlugin;

fn main() {
    println!(
        "The main program in this example just runs the animation graph editor for convenience"
    );
    let mut app = App::new();
    app.add_plugins(AnimationGraphEditorPlugin);
    app.run();
}
