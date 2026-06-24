use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Terrain {
    DeepWater,
    Water,
    Sand,
    Grass,
    Forest,
    Stone,
    Snow,
}

impl Terrain {
    pub fn from_height(h: f64) -> Self {
        match h {
            _ if h < -0.3 => Self::DeepWater,
            _ if h < -0.1 => Self::Water,
            _ if h < 0.0 => Self::Sand,
            _ if h < 0.4 => Self::Grass,
            _ if h < 0.65 => Self::Forest,
            _ if h < 0.8 => Self::Stone,
            _ => Self::Snow,
        }
    }

    pub fn color(&self) -> [u8; 4] {
        match self {
            Self::DeepWater => [20, 50, 120, 255],
            Self::Water => [40, 90, 180, 255],
            Self::Sand => [210, 190, 130, 255],
            Self::Grass => [60, 160, 60, 255],
            Self::Forest => [30, 100, 30, 255],
            Self::Stone => [130, 130, 130, 255],
            Self::Snow => [230, 230, 240, 255],
        }
    }

    pub fn walkable(&self) -> bool {
        !matches!(self, Self::DeepWater | Self::Water)
    }
}
