use bevy::{prelude::*, utils::{hashbrown::HashMap}};
use bevy_sprite3d::Sprite3d;

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub library: String,
    pub current_animation: String,
    pub current_frame: usize,
}

pub struct AnimationInfo {
    pub len: usize,
    pub row: usize,
}

#[derive(Resource, Default)]
pub struct Animations {
    pub atlases: HashMap<String, (HashMap<String, AnimationInfo>, Handle<TextureAtlasLayout>)>
}

impl Animations {
    pub fn new_animation(&mut self, key: String, name: String, info: AnimationInfo, layout: Handle<TextureAtlasLayout>) {
        let mut anim_map = self.atlases.get_mut(&key);

        if anim_map.is_none() {
            self.atlases.insert(key.clone(), (HashMap::new(), layout));
            anim_map = self.atlases.get_mut(&key);
        }

        let (anim_map, _) = anim_map.unwrap();

        anim_map.insert(name, info);
    }

    pub fn get_animation(&self, key: String, name: String) -> Option<&AnimationInfo> {
        let Some((animation, _)) = self.atlases.get(&key) else { return None };
        animation.get(&name)
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut q_sprites: Query<(&mut AnimationTimer, &mut Sprite3d)>,
    animations: Res<Animations>,
) {
    for (mut animation, mut sprite_3d) in q_sprites.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            let info = animations.get_animation(animation.library.clone(), animation.current_animation.clone()).unwrap();
            let length = sprite_3d.texture_atlas_keys.as_ref().unwrap().len();
            let atlas = sprite_3d.texture_atlas.as_mut().unwrap();
            animation.current_frame = (animation.current_frame + 1) % info.len;
            atlas.index = index(info.len, animation.current_frame, info.len) % length;
        }
    }
}

fn index(column: usize, row: usize, width: usize) -> usize {
    (column * width) + row
}