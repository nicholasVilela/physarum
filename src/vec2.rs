use std::{str::FromStr, num::ParseFloatError};
use serde::{Deserialize, Serialize};
use ggez::mint::Point2;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        return Vec2 {
            x,
            y,
        };
    }

    pub fn point(self) -> Point2<f32> {
        return Point2 {x: self.x, y: self.y};
    }

    pub fn string(self) -> String {
        return String::from(format!("{:?},{:?}", self.x, self.y));
    }
}

impl Into<Point2<f32>> for Vec2 {
    fn into(self) -> Point2<f32> {
        return Point2 { x: self.x, y: self.y };
    }
}

impl FromStr for Vec2 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<&str> = s.split(",").collect();
        let x = list[0].parse::<f32>()?;
        let y = list[1].parse::<f32>()?;
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

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
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

impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        };
    }
}