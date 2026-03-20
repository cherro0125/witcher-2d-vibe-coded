/// Renderowanie 3D - kamera TPS z myszką, oświetlenie, mgła, animacje
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::pbr::FogSettings;
use crate::world::{WorldData, TileMarker, TreeMarker, NpcMarker, LootMarker};
use crate::monsters::MonsterMarker;
use crate::player::{Player, PlayerMarker};
use crate::game_state::{GameScreen, CombatAnimEvent};

/// Quen shield bubble marker
#[derive(Component)]
pub struct QuenBubble;

/// Combat animation particle
#[derive(Component)]
pub struct CombatParticle {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec3,
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraOrbit {
            yaw: 0.0,
            pitch: 0.35,
            distance: 8.0,
            sensitivity: 0.003,
        })
        .init_resource::<DayNightCycle>()
        .insert_resource(WalkAnimState { phase: 0.0, last_x: 25.0, last_y: 18.0, is_moving: false })
        .insert_resource(AttackAnimState { timer: 0.0, is_attacking: false })
        .insert_resource(DodgeRollState { timer: 0.0, active: false, direction: Vec3::ZERO, start_pos: Vec3::ZERO })
        .add_event::<SpawnSignParticles>()
        .add_systems(OnEnter(GameScreen::Exploration), setup_3d_world)
        .add_systems(
            Update,
            (
                mouse_camera_control,
                camera_follow_player,
                sync_player_transform,
                sync_monster_visibility,
                animate_loot,
                day_night_cycle_system,
                update_sign_particles,
                spawn_sign_particles_system,
                update_quen_bubble,
                animate_player_walk,
                animate_sword_attack,
                sync_monster_transforms,
                update_monster_hp_bars,
                dodge_roll_system,
                spawn_combat_particles,
                update_combat_particles,
            )
                .run_if(in_state(GameScreen::Exploration)),
        );
    }
}

/// Main camera marker
#[derive(Component)]
pub struct MainCamera;

/// Camera orbit state (mouse-controlled)
#[derive(Resource)]
pub struct CameraOrbit {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub sensitivity: f32,
}

/// Marker for sword child entities on player
#[derive(Component)]
pub struct SwordMarker {
    pub is_steel: bool,
    pub is_blade: bool,
}

/// Monster HP bar marker
#[derive(Component)]
pub struct MonsterHpBar(pub usize); // monster index

/// Player attack animation state
#[derive(Resource)]
pub struct AttackAnimState {
    pub timer: f32,       // counts down from 0.4 to 0
    pub is_attacking: bool,
}

/// Dodge roll state
#[derive(Resource)]
pub struct DodgeRollState {
    pub timer: f32,
    pub active: bool,
    pub direction: Vec3,
    pub start_pos: Vec3,
}

/// Day/night cycle resource
#[derive(Resource)]
pub struct DayNightCycle {
    pub time_of_day: f32, // 0.0 = midnight, 0.5 = noon, 1.0 = midnight
    pub speed: f32,       // full cycle time in game-seconds
}

impl Default for DayNightCycle {
    fn default() -> Self {
        DayNightCycle {
            time_of_day: 0.35, // start at morning
            speed: 120.0,      // 2 minute full day cycle
        }
    }
}

/// Particle marker for sign effects
#[derive(Component)]
pub struct SignParticle {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec3,
}

/// Marker for NPC head
#[derive(Component)]
pub struct NpcHead;

/// Marker for monster horns/wings
#[derive(Component)]
pub struct MonsterDetail;

/// Walking animation: marks player limbs
#[derive(Component)]
pub struct PlayerLimb {
    pub is_left: bool,
    pub is_leg: bool,
    pub base_y: f32,
}

/// Track if player is moving for walk animation
#[derive(Resource)]
pub struct WalkAnimState {
    pub phase: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub is_moving: bool,
}

/// Setup the entire 3D world
fn setup_3d_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world: Res<WorldData>,
    player: Res<Player>,
) {
    // === Camera with fog ===
    let player_pos = Vec3::new(player.x, 1.5, player.y);
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(player_pos + Vec3::new(0.0, 4.0, 8.0))
            .looking_at(player_pos, Vec3::Y),
        MainCamera,
        // Atmospheric fog
        FogSettings {
            color: Color::srgba(0.35, 0.4, 0.5, 1.0),
            directional_light_color: Color::srgba(1.0, 0.9, 0.7, 1.0),
            directional_light_exponent: 30.0,
            falloff: bevy::pbr::FogFalloff::Linear {
                start: 15.0,
                end: 45.0,
            },
        },
    ));

    // === Lighting ===
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            color: Color::srgba(1.0, 0.95, 0.85, 1.0),
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.3, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgba(0.4, 0.45, 0.6, 1.0),
        brightness: 150.0,
    });

    // === Shared meshes ===
    let tile_mesh = meshes.add(Cuboid::new(0.98, 0.15, 0.98));
    let wall_mesh = meshes.add(Cuboid::new(0.98, 1.5, 0.98));
    let mountain_mesh = meshes.add(Cuboid::new(0.98, 2.0, 0.98));
    let building_mesh = meshes.add(Cuboid::new(0.98, 0.5, 0.98));
    let tree_trunk_mesh = meshes.add(Cylinder::new(0.08, 0.8));
    let tree_crown_mesh = meshes.add(Sphere::new(0.35));
    let npc_mesh = meshes.add(Cuboid::new(0.35, 1.0, 0.35));
    let loot_mesh = meshes.add(Sphere::new(0.15));

    // Player body parts
    let torso_mesh = meshes.add(Cuboid::new(0.4, 0.6, 0.25));
    let head_mesh = meshes.add(Sphere::new(0.18));
    let leg_mesh = meshes.add(Cuboid::new(0.15, 0.5, 0.15));
    let arm_mesh = meshes.add(Cuboid::new(0.12, 0.5, 0.12));
    let sword_blade_mesh = meshes.add(Cuboid::new(0.03, 0.7, 0.06));
    let sword_handle_mesh = meshes.add(Cuboid::new(0.04, 0.15, 0.04));
    let hair_mesh_asset = meshes.add(Cuboid::new(0.2, 0.06, 0.28));

    // === Materials ===
    let tree_trunk_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.4, 0.25, 0.1, 1.0),
        ..default()
    });
    let tree_crown_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.15, 0.5, 0.15, 1.0),
        ..default()
    });
    let skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.7, 0.55, 1.0),
        ..default()
    });
    let armor_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.15, 0.12, 0.1, 1.0),
        perceptual_roughness: 0.8,
        ..default()
    });
    let pants_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.18, 0.15, 1.0),
        ..default()
    });
    let steel_blade_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.75, 0.75, 0.8, 1.0),
        metallic: 1.0,
        perceptual_roughness: 0.2,
        ..default()
    });
    let silver_blade_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.92, 0.95, 1.0),
        metallic: 1.0,
        perceptual_roughness: 0.1,
        ..default()
    });
    let handle_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.15, 0.05, 1.0),
        ..default()
    });
    let hair_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.9, 0.85, 1.0),
        ..default()
    });
    let loot_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.5, 1.0),
        emissive: LinearRgba::new(0.0, 3.0, 1.5, 1.0),
        ..default()
    });

    // === Spawn terrain tiles ===
    for ty in 0..world.height {
        for tx in 0..world.width {
            let tile = world.get_tile(tx, ty);
            let tile_color = tile.color();
            let height = tile.height();

            let (mesh_to_use, y_pos) = match tile {
                crate::world::TileType::Wall => (wall_mesh.clone(), height / 2.0),
                crate::world::TileType::Mountain => (mountain_mesh.clone(), height / 2.0),
                crate::world::TileType::Building => (building_mesh.clone(), height / 2.0),
                _ => (tile_mesh.clone(), height),
            };

            let mat = materials.add(StandardMaterial {
                base_color: tile_color,
                perceptual_roughness: 0.9,
                ..default()
            });

            commands.spawn((
                Mesh3d(mesh_to_use),
                MeshMaterial3d(mat),
                Transform::from_xyz(tx as f32, y_pos, ty as f32),
                TileMarker,
            ));

            if tile.has_tree() {
                commands.spawn((
                    Mesh3d(tree_trunk_mesh.clone()),
                    MeshMaterial3d(tree_trunk_mat.clone()),
                    Transform::from_xyz(tx as f32, 0.5, ty as f32),
                    TreeMarker,
                ));
                commands.spawn((
                    Mesh3d(tree_crown_mesh.clone()),
                    MeshMaterial3d(tree_crown_mat.clone()),
                    Transform::from_xyz(tx as f32, 1.1, ty as f32),
                    TreeMarker,
                ));
            }
        }
    }

    // === Spawn Player ===
    commands.spawn((
        Transform::from_xyz(player.x, 0.0, player.y),
        Visibility::default(),
        PlayerMarker,
    )).with_children(|parent| {
        // Torso
        parent.spawn((
            Mesh3d(torso_mesh.clone()),
            MeshMaterial3d(armor_mat.clone()),
            Transform::from_xyz(0.0, 0.85, 0.0),
        ));
        // Head
        parent.spawn((
            Mesh3d(head_mesh.clone()),
            MeshMaterial3d(skin_mat.clone()),
            Transform::from_xyz(0.0, 1.35, 0.0),
        ));
        // Hair
        parent.spawn((
            Mesh3d(hair_mesh_asset.clone()),
            MeshMaterial3d(hair_mat.clone()),
            Transform::from_xyz(0.0, 1.5, -0.02),
        ));
        // Left leg
        parent.spawn((
            Mesh3d(leg_mesh.clone()),
            MeshMaterial3d(pants_mat.clone()),
            Transform::from_xyz(-0.1, 0.3, 0.0),
            PlayerLimb { is_left: true, is_leg: true, base_y: 0.3 },
        ));
        // Right leg
        parent.spawn((
            Mesh3d(leg_mesh.clone()),
            MeshMaterial3d(pants_mat.clone()),
            Transform::from_xyz(0.1, 0.3, 0.0),
            PlayerLimb { is_left: false, is_leg: true, base_y: 0.3 },
        ));
        // Left arm
        parent.spawn((
            Mesh3d(arm_mesh.clone()),
            MeshMaterial3d(skin_mat.clone()),
            Transform::from_xyz(-0.28, 0.85, 0.0),
            PlayerLimb { is_left: true, is_leg: false, base_y: 0.85 },
        ));
        // Right arm
        parent.spawn((
            Mesh3d(arm_mesh.clone()),
            MeshMaterial3d(skin_mat.clone()),
            Transform::from_xyz(0.28, 0.85, 0.0),
            PlayerLimb { is_left: false, is_leg: false, base_y: 0.85 },
        ));
        // Steel sword (back, left)
        parent.spawn((
            Mesh3d(sword_blade_mesh.clone()),
            MeshMaterial3d(steel_blade_mat.clone()),
            Transform::from_xyz(-0.12, 1.25, -0.18)
                .with_rotation(Quat::from_rotation_z(0.15)),
            SwordMarker { is_steel: true, is_blade: true },
        ));
        parent.spawn((
            Mesh3d(sword_handle_mesh.clone()),
            MeshMaterial3d(handle_mat.clone()),
            Transform::from_xyz(-0.12, 0.82, -0.18)
                .with_rotation(Quat::from_rotation_z(0.15)),
            SwordMarker { is_steel: true, is_blade: false },
        ));
        // Silver sword (back, right)
        parent.spawn((
            Mesh3d(sword_blade_mesh.clone()),
            MeshMaterial3d(silver_blade_mat.clone()),
            Transform::from_xyz(0.12, 1.25, -0.18)
                .with_rotation(Quat::from_rotation_z(-0.15)),
            SwordMarker { is_steel: false, is_blade: true },
        ));
        parent.spawn((
            Mesh3d(sword_handle_mesh.clone()),
            MeshMaterial3d(handle_mat.clone()),
            Transform::from_xyz(0.12, 0.82, -0.18)
                .with_rotation(Quat::from_rotation_z(-0.15)),
            SwordMarker { is_steel: false, is_blade: false },
        ));
    });

    // === Spawn NPCs (with head and clothing details) ===
    let npc_head_mesh = meshes.add(Sphere::new(0.14));
    let npc_hat_mesh = meshes.add(Cuboid::new(0.3, 0.06, 0.3));
    let npc_skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.7, 0.55, 1.0),
        ..default()
    });

    for (i, npc) in world.npcs.iter().enumerate() {
        let npc_mat = materials.add(StandardMaterial {
            base_color: Color::srgba(npc.color[0], npc.color[1], npc.color[2], npc.color[3]),
            ..default()
        });
        // Body
        commands.spawn((
            Transform::from_xyz(npc.x, 0.0, npc.y),
            Visibility::default(),
            NpcMarker(i),
        )).with_children(|parent| {
            // Torso
            parent.spawn((
                Mesh3d(npc_mesh.clone()),
                MeshMaterial3d(npc_mat.clone()),
                Transform::from_xyz(0.0, 0.6, 0.0),
            ));
            // Head
            parent.spawn((
                Mesh3d(npc_head_mesh.clone()),
                MeshMaterial3d(npc_skin_mat.clone()),
                Transform::from_xyz(0.0, 1.25, 0.0),
                NpcHead,
            ));
            // Hat (for quest givers) or bag (for merchants)
            if npc.quest_giver {
                let hat_mat = materials.add(StandardMaterial {
                    base_color: Color::srgba(0.6, 0.2, 0.1, 1.0),
                    ..default()
                });
                parent.spawn((
                    Mesh3d(npc_hat_mesh.clone()),
                    MeshMaterial3d(hat_mat),
                    Transform::from_xyz(0.0, 1.42, 0.0),
                ));
            }
            if npc.merchant {
                let bag_mesh = meshes.add(Cuboid::new(0.15, 0.2, 0.1));
                let bag_mat = materials.add(StandardMaterial {
                    base_color: Color::srgba(0.5, 0.35, 0.1, 1.0),
                    ..default()
                });
                parent.spawn((
                    Mesh3d(bag_mesh),
                    MeshMaterial3d(bag_mat),
                    Transform::from_xyz(0.22, 0.5, 0.0),
                ));
            }
        });
    }

    // === Spawn Monsters (with horns, wings, tails) ===
    use crate::monsters::MonsterType;
    let horn_mesh = meshes.add(Cuboid::new(0.04, 0.25, 0.04));
    let wing_mesh = meshes.add(Cuboid::new(0.5, 0.3, 0.03));
    let tail_mesh = meshes.add(Cuboid::new(0.06, 0.06, 0.4));
    let eye_mesh = meshes.add(Sphere::new(0.05));

    let eye_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.0, 0.0, 1.0),
        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
        unlit: true,
        ..default()
    });

    for (i, monster) in world.monsters.iter().enumerate() {
        let h = monster.monster_type.height();
        let m_mesh = meshes.add(Cuboid::new(0.4, h, 0.4));
        let m_color = monster.monster_type.color();
        let m_mat = materials.add(StandardMaterial {
            base_color: m_color,
            ..default()
        });

        commands.spawn((
            Transform::from_xyz(monster.x, 0.0, monster.y),
            Visibility::default(),
            MonsterMarker(i),
        )).with_children(|parent| {
            // Body
            parent.spawn((
                Mesh3d(m_mesh),
                MeshMaterial3d(m_mat.clone()),
                Transform::from_xyz(0.0, h / 2.0 + 0.1, 0.0),
            ));
            // Glowing eyes (all monsters)
            parent.spawn((
                Mesh3d(eye_mesh.clone()),
                MeshMaterial3d(eye_mat.clone()),
                Transform::from_xyz(-0.1, h + 0.05, 0.2),
            ));
            parent.spawn((
                Mesh3d(eye_mesh.clone()),
                MeshMaterial3d(eye_mat.clone()),
                Transform::from_xyz(0.1, h + 0.05, 0.2),
            ));
            // Type-specific details
            match monster.monster_type {
                MonsterType::Gryf | MonsterType::Bazyliszek => {
                    // Wings
                    let wing_mat = materials.add(StandardMaterial {
                        base_color: Color::srgba(m_color.to_srgba().red * 0.7, m_color.to_srgba().green * 0.7, m_color.to_srgba().blue * 0.7, 0.9),
                        ..default()
                    });
                    parent.spawn((
                        Mesh3d(wing_mesh.clone()),
                        MeshMaterial3d(wing_mat.clone()),
                        Transform::from_xyz(-0.45, h * 0.7, 0.0)
                            .with_rotation(Quat::from_rotation_z(0.3)),
                        MonsterDetail,
                    ));
                    parent.spawn((
                        Mesh3d(wing_mesh.clone()),
                        MeshMaterial3d(wing_mat),
                        Transform::from_xyz(0.45, h * 0.7, 0.0)
                            .with_rotation(Quat::from_rotation_z(-0.3)),
                        MonsterDetail,
                    ));
                }
                MonsterType::Wilkolak | MonsterType::Leszen => {
                    // Horns
                    let horn_mat = materials.add(StandardMaterial {
                        base_color: Color::srgba(0.3, 0.25, 0.2, 1.0),
                        ..default()
                    });
                    parent.spawn((
                        Mesh3d(horn_mesh.clone()),
                        MeshMaterial3d(horn_mat.clone()),
                        Transform::from_xyz(-0.12, h + 0.15, 0.08)
                            .with_rotation(Quat::from_rotation_z(0.2)),
                        MonsterDetail,
                    ));
                    parent.spawn((
                        Mesh3d(horn_mesh.clone()),
                        MeshMaterial3d(horn_mat),
                        Transform::from_xyz(0.12, h + 0.15, 0.08)
                            .with_rotation(Quat::from_rotation_z(-0.2)),
                        MonsterDetail,
                    ));
                }
                MonsterType::Kikimora | MonsterType::Endriaga => {
                    // Extra legs (insectoid)
                    let leg_mat = materials.add(StandardMaterial {
                        base_color: Color::srgba(m_color.to_srgba().red * 0.6, m_color.to_srgba().green * 0.6, m_color.to_srgba().blue * 0.6, 1.0),
                        ..default()
                    });
                    let insect_leg = meshes.add(Cuboid::new(0.04, 0.04, 0.3));
                    for side in [-1.0_f32, 1.0] {
                        for offset in [0.1_f32, -0.1] {
                            parent.spawn((
                                Mesh3d(insect_leg.clone()),
                                MeshMaterial3d(leg_mat.clone()),
                                Transform::from_xyz(side * 0.25, h * 0.3, offset)
                                    .with_rotation(Quat::from_rotation_z(side * 0.5)),
                            ));
                        }
                    }
                }
                _ => {
                    // Tail for other monsters
                    let tail_mat = materials.add(StandardMaterial {
                        base_color: m_color,
                        ..default()
                    });
                    parent.spawn((
                        Mesh3d(tail_mesh.clone()),
                        MeshMaterial3d(tail_mat),
                        Transform::from_xyz(0.0, h * 0.25, -0.35),
                        MonsterDetail,
                    ));
                }
            }
        });
    }

    // === Spawn Monster HP Bars ===
    let hp_bar_bg_mesh = meshes.add(Cuboid::new(0.6, 0.08, 0.02));
    let hp_bar_fg_mesh = meshes.add(Cuboid::new(0.58, 0.06, 0.03));
    let hp_bg_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.0, 0.0, 0.9),
        unlit: true,
        ..default()
    });
    let hp_fg_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.1, 0.1, 1.0),
        emissive: LinearRgba::new(2.0, 0.0, 0.0, 1.0),
        unlit: true,
        ..default()
    });

    for (i, monster) in world.monsters.iter().enumerate() {
        let h = monster.monster_type.height();
        // Background bar
        commands.spawn((
            Mesh3d(hp_bar_bg_mesh.clone()),
            MeshMaterial3d(hp_bg_mat.clone()),
            Transform::from_xyz(monster.x, h + 0.4, monster.y),
            MonsterHpBar(i),
        ));
        // Foreground bar (HP fill) — tagged with index + 10000 to distinguish
        commands.spawn((
            Mesh3d(hp_bar_fg_mesh.clone()),
            MeshMaterial3d(hp_fg_mat.clone()),
            Transform::from_xyz(monster.x, h + 0.4, monster.y + 0.01),
            MonsterHpBar(i + 10000), // offset to mark as foreground
        ));
    }

    // === Spawn Loot (with rotation marker) ===
    for (i, loot_item) in world.loot.iter().enumerate() {
        commands.spawn((
            Mesh3d(loot_mesh.clone()),
            MeshMaterial3d(loot_mat.clone()),
            Transform::from_xyz(loot_item.x, 0.4, loot_item.y),
            LootMarker(i),
        ));
    }
}

// ==================== MOUSE CAMERA CONTROL ====================

fn mouse_camera_control(
    mut orbit: ResMut<CameraOrbit>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        for ev in mouse_motion.read() {
            orbit.yaw -= ev.delta.x * orbit.sensitivity;
            orbit.pitch -= ev.delta.y * orbit.sensitivity;
            orbit.pitch = orbit.pitch.clamp(0.05, 1.2);
        }
    } else {
        mouse_motion.clear();
    }

    for ev in scroll_events.read() {
        orbit.distance -= ev.y * 0.8;
        orbit.distance = orbit.distance.clamp(3.0, 20.0);
    }
}

/// Camera follows the player from behind using orbit state
fn camera_follow_player(
    player: Res<Player>,
    orbit: Res<CameraOrbit>,
    mut cam_q: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok(mut cam_t) = cam_q.get_single_mut() else { return };

    // Use visual position for smooth camera
    let player_pos = Vec3::new(player.visual_x, 0.0, player.visual_y);

    // Orbit yaw is ABSOLUTE — not relative to player direction
    let cam_x = orbit.distance * orbit.pitch.cos() * orbit.yaw.sin();
    let cam_y = orbit.distance * orbit.pitch.sin();
    let cam_z = orbit.distance * orbit.pitch.cos() * orbit.yaw.cos();

    let desired_pos = player_pos + Vec3::new(cam_x, cam_y + 1.5, cam_z);
    let look_target = player_pos + Vec3::new(0.0, 1.2, 0.0);

    let lerp_speed = 6.0 * time.delta_secs();
    cam_t.translation = cam_t.translation.lerp(desired_pos, lerp_speed.min(1.0));
    cam_t.look_at(look_target, Vec3::Y);
}

// ==================== SYNC & ANIMATE ====================

fn sync_player_transform(
    mut player: ResMut<Player>,
    mut player_q: Query<&mut Transform, (With<PlayerMarker>, Without<QuenBubble>)>,
    mut quen_q: Query<&mut Transform, (With<QuenBubble>, Without<PlayerMarker>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let lerp_speed = 10.0 * dt;

    // Interpolate visual position toward logical position
    player.visual_x += (player.x - player.visual_x) * lerp_speed.min(1.0);
    player.visual_y += (player.y - player.visual_y) * lerp_speed.min(1.0);

    let target_rot = match player.direction {
        crate::player::Direction::Up => std::f32::consts::PI,
        crate::player::Direction::Down => 0.0,
        crate::player::Direction::Left => std::f32::consts::FRAC_PI_2,
        crate::player::Direction::Right => -std::f32::consts::FRAC_PI_2,
    };
    let target_quat = Quat::from_rotation_y(target_rot);

    for mut t in &mut player_q {
        t.translation.x = player.visual_x;
        t.translation.z = player.visual_y;
        t.rotation = t.rotation.slerp(target_quat, (8.0 * dt).min(1.0));
    }
    // Sync Quen bubble to visual position
    for mut t in &mut quen_q {
        t.translation.x = player.visual_x;
        t.translation.z = player.visual_y;
    }
}

/// Animate loot: rotate and bob up/down
fn animate_loot(
    time: Res<Time>,
    world: Res<WorldData>,
    mut q: Query<(&LootMarker, &mut Transform, &mut Visibility)>,
) {
    let t = time.elapsed_secs();
    for (lm, mut transform, mut vis) in &mut q {
        // Hide collected
        if lm.0 < world.loot.len() && world.loot[lm.0].collected {
            *vis = Visibility::Hidden;
            continue;
        }
        // Rotate
        transform.rotation = Quat::from_rotation_y(t * 2.0 + lm.0 as f32);
        // Bob up and down
        let base_y = 0.4;
        transform.translation.y = base_y + (t * 3.0 + lm.0 as f32 * 1.5).sin() * 0.12;
    }
}

/// Hide dead monsters
fn sync_monster_visibility(
    world: Res<WorldData>,
    mut monster_q: Query<(&MonsterMarker, &mut Visibility), Without<LootMarker>>,
) {
    for (mm, mut vis) in &mut monster_q {
        if mm.0 < world.monsters.len() && !world.monsters[mm.0].alive {
            *vis = Visibility::Hidden;
        }
    }
}

// ==================== DAY/NIGHT CYCLE ====================

fn day_night_cycle_system(
    time: Res<Time>,
    mut cycle: ResMut<DayNightCycle>,
    mut ambient: ResMut<AmbientLight>,
    mut dir_light_q: Query<&mut DirectionalLight>,
    mut fog_q: Query<&mut FogSettings>,
) {
    cycle.time_of_day += time.delta_secs() / cycle.speed;
    if cycle.time_of_day > 1.0 { cycle.time_of_day -= 1.0; }

    // Sun angle: 0 at midnight, PI at noon
    let sun_angle = cycle.time_of_day * std::f32::consts::TAU;
    let sun_height = (sun_angle).sin(); // -1 at midnight, +1 at noon

    // Brightness based on sun height
    let day_brightness = (sun_height * 0.5 + 0.5).clamp(0.05, 1.0);
    let night_factor = 1.0 - day_brightness;

    // Ambient light color shifts: warm yellow during day, cold blue at night
    let amb_r = 0.15 + day_brightness * 0.35;
    let amb_g = 0.15 + day_brightness * 0.35;
    let amb_b = 0.25 + day_brightness * 0.2 + night_factor * 0.15;
    ambient.color = Color::srgba(amb_r, amb_g, amb_b, 1.0);
    ambient.brightness = 50.0 + day_brightness * 200.0;

    // Directional light (sun/moon)
    for mut dl in &mut dir_light_q {
        if sun_height > 0.0 {
            // Daytime: warm sun
            dl.illuminance = 2000.0 + sun_height * 8000.0;
            let warmth = sun_height;
            dl.color = Color::srgba(1.0, 0.85 + warmth * 0.1, 0.7 + warmth * 0.15, 1.0);
        } else {
            // Nighttime: cold moonlight
            let moon = (-sun_height).clamp(0.0, 1.0);
            dl.illuminance = 200.0 + moon * 600.0;
            dl.color = Color::srgba(0.5, 0.55, 0.8, 1.0);
        }
    }

    // Fog density changes with time of day
    for mut fog in &mut fog_q {
        let fog_density = if sun_height > 0.0 { 0.5 + sun_height * 0.5 } else { 0.3 };
        fog.color = Color::srgba(
            0.2 + day_brightness * 0.2,
            0.22 + day_brightness * 0.2,
            0.3 + day_brightness * 0.15,
            1.0,
        );
        fog.falloff = bevy::pbr::FogFalloff::Linear {
            start: 10.0 + fog_density * 10.0,
            end: 30.0 + fog_density * 20.0,
        };
    }
}

// ==================== SIGN PARTICLE EFFECTS ====================

/// Event to spawn sign particles
#[derive(Event, Clone)]
pub struct SpawnSignParticles {
    pub sign_type: crate::signs::SignType,
    pub position: Vec3,
}

fn spawn_sign_particles_system(
    mut events: EventReader<SpawnSignParticles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use crate::signs::SignType;

    for ev in events.read() {
        let (color, emissive, count, speed, size) = match ev.sign_type {
            SignType::Igni => (
                Color::srgba(1.0, 0.4, 0.0, 0.9),
                LinearRgba::new(5.0, 1.5, 0.0, 1.0),
                20, 2.5, 0.08,
            ),
            SignType::Aard => (
                Color::srgba(0.6, 0.7, 1.0, 0.8),
                LinearRgba::new(1.0, 1.5, 4.0, 1.0),
                15, 4.0, 0.06,
            ),
            SignType::Quen => (
                Color::srgba(0.3, 0.9, 1.0, 0.7),
                LinearRgba::new(0.5, 3.0, 4.0, 1.0),
                12, 1.5, 0.07,
            ),
            SignType::Yrden => (
                Color::srgba(0.8, 0.3, 1.0, 0.8),
                LinearRgba::new(3.0, 0.5, 4.0, 1.0),
                10, 1.0, 0.1,
            ),
            SignType::Axii => (
                Color::srgba(0.4, 1.0, 0.4, 0.7),
                LinearRgba::new(0.5, 3.0, 0.5, 1.0),
                8, 2.0, 0.05,
            ),
        };

        let particle_mesh = meshes.add(Sphere::new(size));
        let particle_mat = materials.add(StandardMaterial {
            base_color: color,
            emissive,
            unlit: true,
            ..default()
        });

        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let rand_y = (i as f32 * 7.3).sin() * 0.5;
            let vel = Vec3::new(
                angle.cos() * speed,
                rand_y * speed * 0.5 + 1.0,
                angle.sin() * speed,
            );

            commands.spawn((
                Mesh3d(particle_mesh.clone()),
                MeshMaterial3d(particle_mat.clone()),
                Transform::from_translation(ev.position + Vec3::new(0.0, 1.0, 0.0))
                    .with_scale(Vec3::ONE),
                SignParticle {
                    lifetime: 0.0,
                    max_lifetime: 0.8 + (i as f32 * 0.37).sin().abs() * 0.4,
                    velocity: vel,
                },
            ));
        }
    }
}

fn update_sign_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut SignParticle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut particle, mut transform) in &mut particles {
        particle.lifetime += dt;
        if particle.lifetime >= particle.max_lifetime {
            commands.entity(entity).despawn();
            continue;
        }
        let progress = particle.lifetime / particle.max_lifetime;
        transform.translation += particle.velocity * dt * (1.0 - progress);
        transform.scale = Vec3::splat(1.0 - progress);
        transform.translation.y -= dt * 2.0 * progress;
    }
}

// ==================== QUEN SHIELD BUBBLE ====================

fn update_quen_bubble(
    player: Res<Player>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing: Query<Entity, With<QuenBubble>>,
) {
    let has_quen = player.quen_shield > 0;
    let bubble_exists = !existing.is_empty();

    if has_quen && !bubble_exists {
        // Spawn bubble
        let bubble_mesh = meshes.add(Sphere::new(0.7));
        let bubble_mat = materials.add(StandardMaterial {
            base_color: Color::srgba(0.2, 0.7, 1.0, 0.2),
            emissive: LinearRgba::new(0.3, 1.5, 2.5, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        commands.spawn((
            Mesh3d(bubble_mesh),
            MeshMaterial3d(bubble_mat),
            Transform::from_xyz(player.x, 0.9, player.y),
            QuenBubble,
        ));
    } else if has_quen && bubble_exists {
        // Already exists — position updated by sync below
    } else if !has_quen && bubble_exists {
        // Remove bubble
        for entity in &existing {
            commands.entity(entity).despawn();
        }
    }
}

// ==================== WALKING ANIMATION ====================

fn animate_player_walk(
    time: Res<Time>,
    player: Res<Player>,
    mut walk_state: ResMut<WalkAnimState>,
    mut limb_q: Query<(&PlayerLimb, &mut Transform)>,
) {
    let dt = time.delta_secs();

    // Check if player is moving
    let dx = player.visual_x - walk_state.last_x;
    let dy = player.visual_y - walk_state.last_y;
    let speed = (dx * dx + dy * dy).sqrt() / dt.max(0.001);
    walk_state.last_x = player.visual_x;
    walk_state.last_y = player.visual_y;
    walk_state.is_moving = speed > 0.5;

    if walk_state.is_moving {
        walk_state.phase += dt * 8.0; // walk cycle speed
    } else {
        // Smoothly return to idle
        walk_state.phase += dt * 4.0;
    }

    for (limb, mut transform) in &mut limb_q {
        let swing = if walk_state.is_moving {
            let phase_offset = if limb.is_left { 0.0 } else { std::f32::consts::PI };
            // Legs and arms swing opposite to each other
            let dir = if limb.is_leg { 1.0 } else { -1.0 };
            (walk_state.phase + phase_offset).sin() * 0.35 * dir
        } else {
            0.0
        };

        transform.rotation = Quat::from_rotation_x(swing);
    }
}

// ==================== SWORD ATTACK ANIMATION ====================

fn animate_sword_attack(
    time: Res<Time>,
    mut anim: ResMut<AttackAnimState>,
    mut sword_q: Query<(&SwordMarker, &mut Transform)>,
) {
    if anim.is_attacking {
        anim.timer -= time.delta_secs();
        if anim.timer <= 0.0 {
            anim.is_attacking = false;
        }
    }

    for (sword, mut t) in &mut sword_q {
        if !sword.is_steel { continue; } // only animate steel sword

        if anim.is_attacking {
            // Swing progress 0..1
            let progress = 1.0 - (anim.timer / 0.4).clamp(0.0, 1.0);
            let swing = (progress * std::f32::consts::PI).sin(); // 0 -> 1 -> 0

            if sword.is_blade {
                // Blade: move from back to right hand, swing forward
                let back_pos = Vec3::new(-0.12, 1.25, -0.18);
                let hand_pos = Vec3::new(0.3, 1.1, 0.3);
                t.translation = back_pos.lerp(hand_pos, swing);
                t.rotation = Quat::from_euler(EulerRot::XYZ,
                    -swing * 1.5,  // swing forward
                    swing * 0.5,   // rotate outward
                    0.15 - swing * 0.8,
                );
            } else {
                // Handle follows blade
                let back_pos = Vec3::new(-0.12, 0.82, -0.18);
                let hand_pos = Vec3::new(0.3, 0.7, 0.2);
                t.translation = back_pos.lerp(hand_pos, swing);
                t.rotation = Quat::from_euler(EulerRot::XYZ,
                    -swing * 1.5,
                    swing * 0.5,
                    0.15 - swing * 0.8,
                );
            }
        } else {
            // Rest position (on back)
            if sword.is_blade {
                t.translation = Vec3::new(-0.12, 1.25, -0.18);
                t.rotation = Quat::from_rotation_z(0.15);
            } else {
                t.translation = Vec3::new(-0.12, 0.82, -0.18);
                t.rotation = Quat::from_rotation_z(0.15);
            }
        }
    }
}

// ==================== MONSTER HP BARS ====================

fn update_monster_hp_bars(
    world: Res<WorldData>,
    cam_q: Query<&Transform, With<MainCamera>>,
    mut hp_q: Query<(&MonsterHpBar, &mut Transform, &mut Visibility), Without<MainCamera>>,
) {
    let Ok(cam_t) = cam_q.get_single() else { return };

    for (hp_bar, mut t, mut vis) in &mut hp_q {
        let is_foreground = hp_bar.0 >= 10000;
        let idx = if is_foreground { hp_bar.0 - 10000 } else { hp_bar.0 };

        if idx >= world.monsters.len() { continue; }
        let m = &world.monsters[idx];
        let h = m.monster_type.height();

        if !m.alive {
            *vis = Visibility::Hidden;
            continue;
        }

        // Only show when damaged
        let hp_frac = m.health as f32 / m.max_health as f32;
        if hp_frac >= 1.0 {
            *vis = Visibility::Hidden;
            continue;
        }
        *vis = Visibility::Inherited;

        // Position above monster
        t.translation.x = m.x;
        t.translation.y = h + 0.4;
        t.translation.z = m.y;

        // Billboard: face the camera
        let dir = cam_t.translation - t.translation;
        if dir.length_squared() > 0.01 {
            let flat_dir = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
            if flat_dir.length_squared() > 0.01 {
                t.rotation = Quat::from_rotation_y(flat_dir.x.atan2(flat_dir.z));
            }
        }

        // Scale foreground bar by HP fraction
        if is_foreground {
            t.scale.x = hp_frac.max(0.01);
            // Shift left to keep bar left-aligned
            let offset = (1.0 - hp_frac) * 0.29;
            t.translation.x = m.x - offset * t.rotation.mul_vec3(Vec3::X).x;
        }
    }
}

// ==================== DODGE ROLL ====================

fn dodge_roll_system(
    time: Res<Time>,
    mut player: ResMut<Player>,
    mut dodge: ResMut<DodgeRollState>,
    mut player_q: Query<&mut Transform, With<PlayerMarker>>,
) {
    if !dodge.active { return; }

    dodge.timer -= time.delta_secs();
    let progress = 1.0 - (dodge.timer / 0.3).clamp(0.0, 1.0);

    // Roll the player model
    for mut t in &mut player_q {
        let roll_angle = progress * std::f32::consts::TAU; // full 360 roll
        let base_rot = match player.direction {
            crate::player::Direction::Up => std::f32::consts::PI,
            crate::player::Direction::Down => 0.0,
            crate::player::Direction::Left => std::f32::consts::FRAC_PI_2,
            crate::player::Direction::Right => -std::f32::consts::FRAC_PI_2,
        };
        t.rotation = Quat::from_rotation_y(base_rot) * Quat::from_rotation_z(roll_angle);
    }

    // Move player position during dodge
    let dodge_dist = 2.0; // total tiles to dodge
    let speed = dodge_dist / 0.3; // tiles per second
    let dt = time.delta_secs();
    player.x += dodge.direction.x * speed * dt;
    player.y += dodge.direction.z * speed * dt;
    player.visual_x = player.x;
    player.visual_y = player.y;

    if dodge.timer <= 0.0 {
        dodge.active = false;
        // Snap to grid
        player.x = player.x.round();
        player.y = player.y.round();
    }
}

// ==================== MONSTER TRANSFORM SYNC ====================

fn sync_monster_transforms(
    world: Res<WorldData>,
    mut monster_q: Query<(&MonsterMarker, &mut Transform), Without<LootMarker>>,
) {
    for (mm, mut t) in &mut monster_q {
        if mm.0 < world.monsters.len() {
            let m = &world.monsters[mm.0];
            t.translation.x = m.x;
            t.translation.z = m.y;
        }
    }
}

// ==================== COMBAT ANIMATIONS ====================

fn spawn_combat_particles(
    mut events: EventReader<CombatAnimEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in events.read() {
        let (pos, color, emissive, count, speed, size) = match ev {
            CombatAnimEvent::PlayerSlash { position } => (
                Vec3::new(position.0, 1.0, position.1),
                Color::srgba(0.9, 0.9, 0.95, 0.9),
                LinearRgba::new(2.0, 2.0, 3.0, 1.0),
                12, 3.0_f32, 0.04,
            ),
            CombatAnimEvent::MonsterHit { position } => (
                Vec3::new(position.0, 1.0, position.1),
                Color::srgba(1.0, 0.2, 0.0, 0.9),
                LinearRgba::new(4.0, 0.5, 0.0, 1.0),
                15, 2.5, 0.05,
            ),
            CombatAnimEvent::PlayerDodge { position } => (
                Vec3::new(position.0, 0.5, position.1),
                Color::srgba(0.8, 0.8, 0.4, 0.7),
                LinearRgba::new(1.0, 1.0, 0.3, 1.0),
                8, 4.0, 0.03,
            ),
            CombatAnimEvent::PlayerParry { position } => (
                Vec3::new(position.0, 1.2, position.1),
                Color::srgba(1.0, 0.8, 0.0, 0.9),
                LinearRgba::new(3.0, 2.0, 0.0, 1.0),
                10, 2.0, 0.04,
            ),
        };

        let p_mesh = meshes.add(Sphere::new(size));
        let p_mat = materials.add(StandardMaterial {
            base_color: color,
            emissive,
            unlit: true,
            ..default()
        });

        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let ry = (i as f32 * 5.7).sin() * 0.5;
            let vel = Vec3::new(
                angle.cos() * speed,
                ry * speed * 0.3 + 1.5,
                angle.sin() * speed,
            );

            commands.spawn((
                Mesh3d(p_mesh.clone()),
                MeshMaterial3d(p_mat.clone()),
                Transform::from_translation(pos),
                CombatParticle {
                    lifetime: 0.0,
                    max_lifetime: 0.5 + (i as f32 * 0.23).sin().abs() * 0.3,
                    velocity: vel,
                },
            ));
        }
    }
}

fn update_combat_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut CombatParticle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut p, mut transform) in &mut particles {
        p.lifetime += dt;
        if p.lifetime >= p.max_lifetime {
            commands.entity(entity).despawn();
            continue;
        }
        let progress = p.lifetime / p.max_lifetime;
        transform.translation += p.velocity * dt * (1.0 - progress);
        transform.scale = Vec3::splat(1.0 - progress * 0.8);
        transform.translation.y -= dt * 3.0 * progress;
    }
}
