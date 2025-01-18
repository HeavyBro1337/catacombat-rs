use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_sprite3d::Sprite3d;

#[derive(Component, Default)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub library: String,
    pub current_animation: String,
    pub current_frame: usize,
    pub next: Option<String>,
    pub update_now: bool,
}

impl AnimationTimer {
    pub fn play(&mut self, animation: String, transition: Option<String>) {
        self.current_animation = animation;
        self.current_frame = 0;
        self.next = transition;
        self.update_now = true;
    }

    pub fn next_frame(&mut self, info: &AnimationInfo) {
        let old_frame = self.current_frame;

        self.current_frame = (self.current_frame + 1) % info.len;

        if self.current_frame == 0 && old_frame != 0 {
            if self.next.is_some() {
                let next = self.next.clone().unwrap().clone();
                self.play(next, None);
                self.update_now = false;
            } else if !info.looped {
                self.current_frame = old_frame;
            }
        }
    }
}

pub struct AnimationInfo {
    pub len: usize,
    pub row: usize,
    pub looped: bool,
}

#[derive(Resource, Default)]
pub struct Animations {
    pub atlases: HashMap<
        String,
        (
            HashMap<String, AnimationInfo>,
            Handle<TextureAtlasLayout>,
            usize,
        ),
    >,
}

impl Animations {
    pub fn new_animation(
        &mut self,
        key: String,
        name: String,
        info: AnimationInfo,
        layout: Handle<TextureAtlasLayout>,
        max_width: usize,
    ) {
        let mut anim_map = self.atlases.get_mut(&key);

        if anim_map.is_none() {
            self.atlases
                .insert(key.clone(), (HashMap::new(), layout, max_width));
            anim_map = self.atlases.get_mut(&key);
        }

        let (anim_map, _, _) = anim_map.unwrap();

        anim_map.insert(name, info);
    }

    pub fn get_layout_width(&self, key: String) -> Option<usize> {
        match self.atlases.get(&key) {
            Some((_, _, width)) => Some(*width),
            None => None,
        }
    }

    pub fn get_animation(&self, key: String, name: String) -> Option<&AnimationInfo> {
        let Some((animation, _, _)) = self.atlases.get(&key) else {
            return None;
        };
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
        if animation.timer.just_finished() || animation.update_now {
            animation.update_now = false;
            let info = animations
                .get_animation(
                    animation.library.clone(),
                    animation.current_animation.clone(),
                )
                .unwrap();
            let atlas = sprite_3d.texture_atlas.as_mut().unwrap();
            animation.next_frame(&info);
            let layout_width = animations
                .get_layout_width(animation.library.clone())
                .unwrap();

            atlas.index = index(animation.current_frame, info.row, layout_width);
        }
    }
}

fn index(column: usize, row: usize, width: usize) -> usize {
    (row * width) + column
}
