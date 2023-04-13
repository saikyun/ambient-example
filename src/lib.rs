use ambient_api::{
    components::core::{
        game_objects::player_camera,
        player::{player, user_id},
        physics::{
            angular_velocity, box_collider, dynamic, linear_velocity, physics_controlled,
            visualizing,
        },
        primitives::cube,
        rendering::{cast_shadows, color},
        prefab::prefab_from_url,
        transform::{lookat_center, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use components::owner;

#[main]
pub async fn main() -> EventResult {
    Entity::new()
    .with_merge(make_transformable())
    .with(prefab_from_url(), asset_url("assets/Shape.glb").unwrap())
    .spawn();

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    spawn_query(player()).bind(move |players| {
        // For each player joining, spawn a random colored box somewhere
        for (id, _) in players {
            Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                .with(box_collider(), Vec3::ONE)
                .with(dynamic(), true)
                .with_default(physics_controlled())
                .with_default(cast_shadows())
                .with(translation(), rand::random())
                .with(color(), random::<Vec4>() + Vec4::new(0.0, 0.0, 0.0, 1.0))
                .with(owner(), id)
                .spawn();
        }
    });


    query(cube()).build().each_frame(|ids| {
        for (id, _) in ids {
            let player_id = entity::get_component(id, owner()).unwrap();
            let Some((delta, _)) = player::get_raw_input_delta(player_id) else { continue; };
            let Some(name) = entity::get_component(player_id, user_id()) else { continue; };

            if !delta.keys.is_empty() {
                let pos = entity::get_component(id, translation()).unwrap();

                if delta.keys.contains(&player::KeyCode::W) {
                    entity::set_component(id, translation(), pos + Vec3::Y * 0.3)
                }

                if delta.keys.contains(&player::KeyCode::S) {
                    entity::set_component(id, translation(), pos + Vec3::Y * -0.3)
                }

                if delta.keys.contains(&player::KeyCode::A) {
                    entity::set_component(id, translation(), pos + Vec3::X * 0.3)
                }

                if delta.keys.contains(&player::KeyCode::D) {
                    entity::set_component(id, translation(), pos + Vec3::X * -0.3)
                }
            }
        }
    });

    EventOk
}
