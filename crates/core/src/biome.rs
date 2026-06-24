use crate::decoration::DECORATION_COUNT;
use crate::terrain::Terrain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Biome {
    Temperate,
    Desert,
    Jungle,
    Swamp,
    Tundra,
    Mountain,
    Coastal,
}

const ALL_BIOMES: [Biome; 7] = [
    Biome::Temperate,
    Biome::Desert,
    Biome::Jungle,
    Biome::Swamp,
    Biome::Tundra,
    Biome::Mountain,
    Biome::Coastal,
];

struct BiomeProfile {
    thresholds: [f64; 6],
    colors: [[u8; 4]; 7],
    decoration_mults: [f32; DECORATION_COUNT],
    height_bias: f64,
    roughness_mult: f64,
}

pub struct BiomeSample {
    pub thresholds: [f64; 6],
    pub colors: [[u8; 4]; 7],
    pub decoration_mults: [f32; DECORATION_COUNT],
    pub height_bias: f64,
    pub roughness_mult: f64,
    pub primary: Biome,
}

impl Biome {
    fn climate_center(self) -> (f64, f64) {
        match self {
            Biome::Temperate => (0.0, 0.0),
            Biome::Desert => (0.6, -0.5),
            Biome::Jungle => (0.6, 0.6),
            Biome::Swamp => (0.0, 0.6),
            Biome::Tundra => (-0.6, 0.2),
            Biome::Mountain => (-0.6, -0.4),
            Biome::Coastal => (0.15, 0.3),
        }
    }

    // Decoration index order:
    //  0  PineTree    1  OakTree     2  BirchTree   3  Bush        4  FlowerBush
    //  5  Rock        6  Boulder     7  MountainPeak 8 Pebbles     9  Flowers
    // 10  Driftwood  11  SnowPile   12  Cactus      13 PalmTree   14  DeadTree
    // 15  TallGrass  16  DryShrub   17  SandDune    18 Reeds      19  Mushroom
    // 20  FlatRock   21  IceChunk   22  Fern        23 Moss       24  Seashell
    fn profile(self) -> BiomeProfile {
        match self {
            Biome::Temperate => BiomeProfile {
                thresholds: [-0.45, -0.25, 0.0, 0.4, 0.65, 0.8],
                colors: [
                    [20, 50, 120, 255],
                    [40, 90, 180, 255],
                    [210, 190, 130, 255],
                    [60, 160, 60, 255],
                    [30, 100, 30, 255],
                    [130, 130, 130, 255],
                    [230, 230, 240, 255],
                ],
                decoration_mults: [
                    1.0, 1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 0.0, 0.0, 0.2,
                    1.5, 0.3, 0.0, 0.3, 1.0,
                    0.5, 0.0, 0.3, 0.5, 0.0,
                ],
                height_bias: 0.0,
                roughness_mult: 1.0,
            },
            Biome::Desert => BiomeProfile {
                thresholds: [-0.6, -0.45, 0.3, 0.65, 0.78, 0.92],
                colors: [
                    [15, 40, 100, 255],
                    [30, 70, 120, 255],
                    [220, 195, 140, 255],
                    [185, 175, 100, 255],
                    [140, 125, 65, 255],
                    [175, 160, 130, 255],
                    [240, 235, 220, 255],
                ],
                decoration_mults: [
                    0.0, 0.0, 0.0, 0.1, 0.0,
                    1.5, 1.0, 0.3, 2.0, 0.0,
                    0.5, 0.0, 3.0, 0.0, 0.3,
                    0.0, 2.5, 2.5, 0.0, 0.0,
                    1.0, 0.0, 0.0, 0.0, 0.0,
                ],
                height_bias: -0.1,
                roughness_mult: 0.6,
            },
            Biome::Jungle => BiomeProfile {
                thresholds: [-0.4, -0.2, -0.05, 0.15, 0.5, 0.8],
                colors: [
                    [15, 45, 80, 255],
                    [30, 80, 100, 255],
                    [150, 120, 70, 255],
                    [30, 140, 30, 255],
                    [15, 85, 15, 255],
                    [100, 110, 90, 255],
                    [200, 210, 200, 255],
                ],
                decoration_mults: [
                    0.0, 1.5, 0.2, 2.5, 2.0,
                    0.3, 0.2, 0.0, 0.5, 1.5,
                    0.5, 0.0, 0.0, 2.5, 0.0,
                    2.0, 0.0, 0.0, 0.5, 2.0,
                    0.0, 0.0, 3.0, 2.0, 0.0,
                ],
                height_bias: 0.05,
                roughness_mult: 1.2,
            },
            Biome::Swamp => BiomeProfile {
                thresholds: [-0.35, -0.05, 0.1, 0.4, 0.58, 0.85],
                colors: [
                    [25, 50, 40, 255],
                    [50, 80, 60, 255],
                    [130, 110, 80, 255],
                    [80, 130, 60, 255],
                    [40, 90, 35, 255],
                    [110, 110, 100, 255],
                    [200, 210, 200, 255],
                ],
                decoration_mults: [
                    0.2, 0.3, 0.2, 1.5, 1.0,
                    0.5, 0.2, 0.0, 0.8, 1.0,
                    3.0, 0.0, 0.0, 0.0, 2.5,
                    1.5, 0.0, 0.0, 3.0, 2.0,
                    0.3, 0.0, 0.5, 2.5, 0.0,
                ],
                height_bias: -0.05,
                roughness_mult: 0.7,
            },
            Biome::Tundra => BiomeProfile {
                thresholds: [-0.5, -0.3, -0.1, 0.15, 0.35, 0.5],
                colors: [
                    [20, 45, 100, 255],
                    [35, 75, 150, 255],
                    [190, 185, 170, 255],
                    [140, 155, 130, 255],
                    [70, 90, 60, 255],
                    [140, 140, 145, 255],
                    [240, 240, 250, 255],
                ],
                decoration_mults: [
                    0.3, 0.0, 0.0, 0.2, 0.0,
                    1.5, 1.0, 0.5, 1.0, 0.0,
                    0.5, 3.0, 0.0, 0.0, 1.0,
                    0.0, 1.5, 0.0, 0.0, 0.0,
                    1.5, 3.0, 0.0, 0.0, 0.0,
                ],
                height_bias: 0.0,
                roughness_mult: 0.8,
            },
            Biome::Mountain => BiomeProfile {
                thresholds: [-0.55, -0.35, -0.15, 0.1, 0.3, 0.5],
                colors: [
                    [15, 40, 110, 255],
                    [35, 75, 160, 255],
                    [180, 170, 150, 255],
                    [90, 130, 80, 255],
                    [50, 80, 45, 255],
                    [110, 110, 115, 255],
                    [235, 235, 245, 255],
                ],
                decoration_mults: [
                    0.3, 0.1, 0.0, 0.2, 0.0,
                    2.0, 2.5, 3.0, 1.5, 0.0,
                    0.0, 1.5, 0.0, 0.0, 0.0,
                    0.0, 0.5, 0.0, 0.0, 0.0,
                    2.5, 0.5, 0.0, 0.0, 0.0,
                ],
                height_bias: 0.15,
                roughness_mult: 1.6,
            },
            Biome::Coastal => BiomeProfile {
                thresholds: [-0.35, -0.15, 0.12, 0.42, 0.62, 0.8],
                colors: [
                    [20, 60, 140, 255],
                    [50, 120, 200, 255],
                    [225, 205, 150, 255],
                    [70, 165, 70, 255],
                    [35, 110, 40, 255],
                    [135, 135, 130, 255],
                    [230, 230, 240, 255],
                ],
                decoration_mults: [
                    0.2, 0.5, 0.3, 0.5, 0.5,
                    0.8, 0.5, 0.0, 2.0, 0.5,
                    3.0, 0.0, 0.0, 1.5, 0.0,
                    0.5, 0.3, 0.0, 2.0, 0.0,
                    0.3, 0.0, 0.0, 0.0, 3.0,
                ],
                height_bias: -0.08,
                roughness_mult: 0.7,
            },
        }
    }
}

pub fn sample_biome(temperature: f64, moisture: f64) -> BiomeSample {
    let mut weights = [(Biome::Temperate, 0.0f64); 7];
    for (i, &biome) in ALL_BIOMES.iter().enumerate() {
        let (ct, cm) = biome.climate_center();
        let dist = ((temperature - ct).powi(2) + (moisture - cm).powi(2)).sqrt();
        weights[i] = (biome, 1.0 / (dist + 0.05));
    }

    weights.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let top_a = weights[0];
    let top_b = weights[1];
    let total = top_a.1 + top_b.1;
    let wa = top_a.1 / total;
    let wb = top_b.1 / total;

    let pa = top_a.0.profile();
    let pb = top_b.0.profile();

    let mut thresholds = [0.0; 6];
    for (i, t) in thresholds.iter_mut().enumerate() {
        *t = pa.thresholds[i] * wa + pb.thresholds[i] * wb;
    }

    let mut colors = [[0u8; 4]; 7];
    for (i, col) in colors.iter_mut().enumerate() {
        for (c, ch) in col.iter_mut().enumerate() {
            *ch = (pa.colors[i][c] as f64 * wa + pb.colors[i][c] as f64 * wb) as u8;
        }
    }

    let mut decoration_mults = [0.0f32; DECORATION_COUNT];
    for (i, m) in decoration_mults.iter_mut().enumerate() {
        *m = (pa.decoration_mults[i] as f64 * wa + pb.decoration_mults[i] as f64 * wb) as f32;
    }

    BiomeSample {
        thresholds,
        colors,
        decoration_mults,
        height_bias: pa.height_bias * wa + pb.height_bias * wb,
        roughness_mult: pa.roughness_mult * wa + pb.roughness_mult * wb,
        primary: top_a.0,
    }
}

impl BiomeSample {
    pub fn terrain_from_height(&self, h: f64) -> Terrain {
        if h < self.thresholds[0] {
            Terrain::DeepWater
        } else if h < self.thresholds[1] {
            Terrain::Water
        } else if h < self.thresholds[2] {
            Terrain::Sand
        } else if h < self.thresholds[3] {
            Terrain::Grass
        } else if h < self.thresholds[4] {
            Terrain::Forest
        } else if h < self.thresholds[5] {
            Terrain::Stone
        } else {
            Terrain::Snow
        }
    }

    pub fn color(&self, terrain: Terrain) -> [u8; 4] {
        self.colors[terrain as usize]
    }
}
