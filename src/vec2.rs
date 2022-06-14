use crate::FVec2;
use std::{str::FromStr, num::ParseIntError};
use serde::{Deserialize, Serialize};
use ggez::mint::Point2;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        return Vec2 {
            x,
            y,
        };
    }

    pub fn point(self) -> Point2<i32> {
        return Point2 {x: self.x, y: self.y};
    }

    pub fn string(self) -> String {
        return String::from(format!("{:?},{:?}", self.x, self.y));
    }
}

impl Into<Point2<f32>> for Vec2 {
    fn into(self) -> Point2<f32> {
        return Point2 { x: self.x as f32, y: self.y as f32 };
    }
}

impl Into<FVec2> for Vec2 {
    fn into(self) -> FVec2 {
        return FVec2 { x: self.x as f32, y: self.y as f32 };
    }
}

impl FromStr for Vec2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<&str> = s.split(",").collect();
        let x = list[0].parse::<i32>()?;
        let y = list[1].parse::<i32>()?;
        let vec2 = Vec2::new(x, y);

        return Ok(vec2);
    }
}

impl std::ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vec2::new(self.x + other.x, self.y + other.y);
    }
}

impl std::ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vec2::new(self.x - other.x, self.y - other.y);
    }
}

impl std::ops::Mul<Self> for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return Vec2::new(self.x * other.x, self.y * other.y);
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        return Vec2::new(self.x * other, self.y * other);
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, other: i32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        };
    }
}