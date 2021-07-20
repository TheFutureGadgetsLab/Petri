// Graciously taken from https://raw.githubusercontent.com/ggez/ggez-goodies/master/src/camera.rs

//! A camera object for ggez.
//! Currently ggez has no actual global camera state to use,
//! so this really just does the coordinate transforms for you.
//!
//! Basically it translates ggez's coordinate system with the origin
//! at the top-left and Y increasing downward to a coordinate system
//! with the origin at the center of the screen and Y increasing
//! upward.
//!
//! Because that makes sense, darn it.
//!
//! However, does not yet do any actual camera movements like
//! easing, pinning, etc.
//! But a great source for how such things work is this:
//! http://www.gamasutra.com/blogs/ItayKeren/20150511/243083/Scroll_Back_The_Theory_and_Practice_of_Cameras_in_SideScrollers.php

// TODO: Debug functions to draw world and camera grid!
#![allow(dead_code)]

use core::f32;
use ggez;
use ggez::graphics;
use ggez::GameResult;
use glam::Vec2;

// Used for mint interoperability.
struct Vector2(Vec2);

// Hmm.  Could, instead, use a 2d transformation
// matrix, or create one of such.
pub struct Camera {
    pub screen_size: Vec2,
    pub view_size: Vec2,
    pub view_center: Vec2,
}

impl Camera {
    pub fn new(screen_size: Vec2, view_size: Vec2) -> Self {
        Camera {
            screen_size: screen_size,
            view_size: view_size,
            view_center: Vec2::new(0.0, 0.0),
        }
    }

    pub fn move_by(&mut self, by: Vec2) {
        self.view_center.x += by.x;
        self.view_center.y += by.y;
    }

    pub fn move_to(&mut self, to: Vec2) {
        self.view_center = to;
    }

    /// Translates a point in world-space to a point in
    /// screen-space.
    ///
    /// Does not do any clipping or anything, since it does
    /// not know how large the thing that might be drawn is;
    /// that's not its job.
    pub fn world_to_screen_coords(&self, from: Vec2) -> Vec2 {
        //let pixels_per_unit = self.screen_size.component_div(&self.view_size);
        let pixels_per_unit = self.screen_size / self.view_size;
        let view_offset = from - self.view_center;
        let view_scale = view_offset * pixels_per_unit;

        let x = view_scale.x + self.screen_size.x / 2.0;
        let y = self.screen_size.y - (view_scale.y + self.screen_size.y / 2.0);
        Vec2::new(x, y)
    }

    // p_screen = max_p - p + max_p/2
    // p_screen - max_p/2 = max_p - p
    // p_screen - max_p/2 + max_p = -p
    // -p_screen - max_p/2 + max_p = p
    pub fn screen_to_world_coords(&self, from: Vec2) -> Vec2 {
        let (sx, sy) = from.into();
        let flipped_x = sx - (self.screen_size.x / 2.0);
        let flipped_y = -sy + self.screen_size.y / 2.0;
        let screen_coords = Vec2::new(flipped_x, flipped_y);
        let units_per_pixel = self.view_size / self.screen_size;
        let view_scale = screen_coords * units_per_pixel;
        let view_offset = self.view_center + view_scale;

        view_offset
    }

    pub fn location(&self) -> Vec2 {
        self.view_center
    }
}

pub trait CameraDraw
where
    Self: graphics::Drawable,
{
    fn draw_camera(
        &self,
        camera: &Camera,
        ctx: &mut ggez::Context,
        dest: Vec2,
        rotation: f32,
    ) -> GameResult<()> {
        let dest = camera.world_to_screen_coords(dest);
        let draw_param = ggez::graphics::DrawParam::default();
        self.draw(ctx, draw_param.rotation(rotation).dest(dest))
    }
}
