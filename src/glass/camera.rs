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
use core::f32;
use ggez;
use ggez::graphics::{DrawParam, Transform};
use glam::Vec2;

pub struct Camera {
    pub transform: DrawParam
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: DrawParam::default()
        }
    }

    pub fn move_by(&mut self, by: Vec2) {
        let mut pos = Vec2::ZERO;
        if let Transform::Values { ref mut dest, .. } = self.transform.trans {
            pos.x = dest.x - by.x;
            pos.y = dest.y + by.x;
        } else {
            panic!("Cannot set values for a DrawParam matrix")
        }
        self.transform = self.transform.dest(pos);
    }

    pub fn zoom_by(&mut self, by: f32) {
        let mut new_scale = Vec2::ZERO;
        if let Transform::Values { ref mut scale, .. } = self.transform.trans {
            new_scale.x = scale.x * (1.0 - by.signum() * 0.1);
            new_scale.y = scale.y * (1.0 - by.signum() * 0.1);
        } else {
            panic!("Cannot set values for a DrawParam matrix")
        }
        self.transform = self.transform.scale(new_scale);
    }
}