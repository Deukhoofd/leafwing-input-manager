use bevy::prelude::*;
use leafwing_input_manager::prelude::Key;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        // The InputMap and ActionState components will be added to any entity with the Player component
        .add_systems(Startup, spawn_player)
        // Read the ActionState in your systems using queries!
        .add_systems(Update, jump)
        .run();
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Forward,
    Left,
    Backward,
    Right,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    let key: InputKind = Key::Character("W".into()).into();
    commands
        .spawn(InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // We can define a case-insensitive character for the logical keys.
            // If the user inputs a corresponding character, the keys will be pressed.
            // For example, a "w" input triggers both the lowercase and uppercase logical "W" keys.
            input_map: InputMap::new([
                (Action::Forward, key),
                (Action::Forward, KeyCode::KeyW.into()),
                (Action::Left, Key::Character("A".into()).into()),
                (Action::Backward, Key::Character("S".into()).into()),
                (Action::Right, Key::Character("D".into()).into()),
            ]),
        })
        .insert(Player);
}

// Query for the `ActionState` component in your game logic systems!
fn jump(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    dbg!(action_state);

    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&Action::Forward) {
        println!("Going forward!");
    } else if action_state.just_pressed(&Action::Left) {
        println!("Going left!");
    } else if action_state.just_pressed(&Action::Backward) {
        println!("Going backward!");
    } else if action_state.just_pressed(&Action::Right) {
        println!("Going right!");
    }
}
