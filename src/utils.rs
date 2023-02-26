use crate::GameState;
use bevy::prelude::*;


use std::ops::{Add, Sub};


// SelectedLevel resource:
#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource)] 
pub struct SelectedLevel {
    pub level: String,
    pub city: String,
    pub map: String,
    pub solution_index: u16,
    pub maps: Vec<String>,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl From<(u16, u16)> for Coordinates {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(i16, i16)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i16, i16)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}