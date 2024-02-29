use bevy::{prelude::*, sprite::MaterialMesh2dBundle};



const PLAYER_HALF_SIZE: Vec2 = Vec2::new(32., 32.);
const PLAYER_SPEED: f32 = 250.;
const JUMP_IMPULSE: f32 = 1000.;
const GRAVITY: f32 = 3000.;
const EPSILON: f32 = 0.001;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                handle_inputs,
                apply_gravity,
                move_player,
            ).chain());
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerMotion {
    on_ground: bool,
    velocity: Vec2,
}

impl Default for PlayerMotion {
    fn default() -> Self {
        Self {
            on_ground: true,
            velocity: Vec2::ZERO,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    motion: PlayerMotion,
    mesh_material_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PlayerBundle {
        mesh_material_bundle: MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle {
                half_size: PLAYER_HALF_SIZE,
            }).into(),
            material: materials.add(Color::PURPLE),
            transform: Transform::default(),
            ..default()
        },
        motion: PlayerMotion::default(),
        player: Player
    });
}


fn handle_inputs(
    mut player_q: Query<&mut PlayerMotion, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut player_motion = player_q.single_mut();
    let mut vel_x = 0.;

    if keys.pressed(KeyCode::ArrowLeft) {
        vel_x -= PLAYER_SPEED;        
    }

    if keys.pressed(KeyCode::ArrowRight) {
        vel_x += PLAYER_SPEED;        
    }
   
    if vel_x.abs() > EPSILON {
        player_motion.velocity.x = vel_x;
    }

    if player_motion.on_ground && keys.pressed(KeyCode::Space) {
        player_motion.velocity.y = JUMP_IMPULSE;
        player_motion.on_ground = false;
    }
}


fn apply_gravity(
    mut player_q: Query<&mut PlayerMotion, With<Player>>,
    time: Res<Time>,
) {
    let mut player_motion = player_q.single_mut();

    player_motion.velocity.y -= GRAVITY * time.delta_seconds();
}


fn move_player(
    mut player_q: Query<(&mut PlayerMotion, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_motion, mut transform) = player_q.single_mut();

    transform.translation += Vec3::from((player_motion.velocity, 0.)) * time.delta_seconds();
    if transform.translation.y < PLAYER_HALF_SIZE.y {
        player_motion.on_ground = true;
        player_motion.velocity.y = 0.;
        transform.translation.y = PLAYER_HALF_SIZE.y;
    }

    player_motion.velocity.x *= 0.85;
    if player_motion.velocity.x.abs() < EPSILON {
        player_motion.velocity.x = 0.;
    }
}
