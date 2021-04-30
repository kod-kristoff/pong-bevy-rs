use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::sprite::collide_aabb::{collide, Collision};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .add_system(ball_movement_system.system())
        .add_system(ball_collision_system.system())
        .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());

    // left paddle
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(-600.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(20.0, 120.0),
                ..Default::default()
            },
            ..Default::default()
        });

    // ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(20.0, 20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });

    // game area
    commands
        // top
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 345.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(1280.0, 30.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
    commands
        // bottom
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -345.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(1280.0, 30.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
        // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-690.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(100.0, 720.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Scoreable);
        // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(690.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(100.0, 720.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Scoreable);
}

struct Ball {
    velocity: Vec3,
}

enum Collider {
    Solid,
    Scoreable,
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (ball, mut transform) in query.iter_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn ball_collision_system(
    // mut ball_reset_timer: ResMut<BallResetTimer>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    other_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    // mut scoreboard_query: Query<(&mut Scoreboard, &mut Text)>,
) {
    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        let ball_size = ball_sprite.size;
        let velocity = &mut ball.velocity;

        for (collider_entity, collider, other_transform, other_sprite) in other_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                other_transform.translation,
                other_sprite.size,
            );

            if let Some(collision) = collision {
                // Update scoreboard if we hit a goal
                if let &Collider::Scoreable = collider {
                    // for (mut scoreboard, mut text) in scoreboard_query.iter_mut() {
                    //     if ball_transform.translation.x > 0.0 {
                    //         scoreboard.0 += 1;
                    //     }
                    //     if ball_transform.translation.x < 0.0 {
                    //         scoreboard.1 += 1;
                    //     }
                    //     text.value = format!("{} {}", scoreboard.0, scoreboard.1);
                    //     *velocity = Vec3::new(0.0, 0.0, 0.0);
                    //     ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
                    //     ball_reset_timer.1 = true;
                    //     ball_reset_timer.0.reset();
                    // }
                    println!("collide with Scoreable");
                }

                match collision {
                    Collision::Left => {
                        if velocity.x > 0.0 {
                            velocity.x = -velocity.x;
                            // ball_transform.translation.x = other_transform.translation.x
                            //     - other_sprite.size.x / 2.0
                            //     - ball_size.x / 2.0;
                        }
                    }
                    Collision::Right => {
                        if velocity.x < 0.0 {
                            velocity.x = -velocity.x;
                            // ball_transform.translation.x = other_transform.translation.x
                            //     + other_sprite.size.x / 2.0
                            //     + ball_size.x / 2.0;
                        }
                    }
                    Collision::Top => {
                        if velocity.y < 0.0 {
                            velocity.y = -velocity.y;
                            // ball_transform.translation.y = other_transform.translation.y
                            //     + other_sprite.size.y / 2.0
                            //     + ball_size.y / 2.0;
                        }
                    }
                    Collision::Bottom => {
                        if velocity.y > 0.0 {
                            velocity.y = -velocity.y;
                            // ball_transform.translation.y = other_transform.translation.y
                            //     - other_sprite.size.y / 2.0
                            //     - ball_size.y / 2.0;
                        }
                    }
                }

                break;
            }
        }
    }
}
