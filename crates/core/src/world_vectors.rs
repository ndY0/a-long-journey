use noise::{NoiseFn, Perlin};

use crate::biome::{self, BiomeSample};
use crate::CHUNK_SIZE;

pub struct WorldVectors {
    // Regional scale (~0.003) — large-wavelength fields
    temperature: Perlin,
    moisture: Perlin,
    affinity: Perlin,
    density: Perlin,

    // Block/medium scale (~0.006–0.2) — terrain modulation
    height_bias: Perlin,
    roughness: Perlin,
    threshold_dither: Perlin,
}

#[derive(Debug, Clone, Copy)]
pub struct WorldSample {
    pub temperature: f64,
    pub moisture: f64,
    pub affinity_value: f64,
    pub density: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockSample {
    pub height_bias: f64,
    pub roughness: f64,
    pub dither: f64,
}

impl WorldVectors {
    pub fn new(seed: u32) -> Self {
        Self {
            temperature: Perlin::new(seed.wrapping_add(1000)),
            moisture: Perlin::new(seed.wrapping_add(2000)),
            affinity: Perlin::new(seed.wrapping_add(3000)),
            density: Perlin::new(seed.wrapping_add(4000)),
            height_bias: Perlin::new(seed.wrapping_add(5000)),
            roughness: Perlin::new(seed.wrapping_add(6000)),
            threshold_dither: Perlin::new(seed.wrapping_add(7000)),
        }
    }

    pub fn sample(&self, wx: f64, wy: f64) -> WorldSample {
        let large = 0.0015;
        let medium = 0.004;

        let temperature = self.temperature.get([wx * large, wy * large]) * 0.7
            + self.temperature.get([wx * medium, wy * medium]) * 0.3;

        let moisture = self.moisture.get([wx * large, wy * large]) * 0.7
            + self.moisture.get([wx * medium, wy * medium]) * 0.3;

        let affinity_value = self.affinity.get([wx * large, wy * large]);

        let raw_density = self.density.get([wx * large, wy * large]);
        let density = (raw_density + 1.0) * 0.5;

        WorldSample {
            temperature,
            moisture,
            affinity_value,
            density,
        }
    }

    pub fn sample_chunk_center(&self, chunk_x: i32, chunk_y: i32) -> WorldSample {
        let cx = chunk_x as f64 * CHUNK_SIZE as f64 + CHUNK_SIZE as f64 * 0.5;
        let cy = chunk_y as f64 * CHUNK_SIZE as f64 + CHUNK_SIZE as f64 * 0.5;
        self.sample(cx, cy)
    }

    pub fn sample_tile(&self, wx: f64, wy: f64) -> BlockSample {
        let height_bias = self.height_bias.get([wx * 0.006, wy * 0.006]) * 0.08
            + self.height_bias.get([wx * 0.015, wy * 0.015]) * 0.04;

        let raw_roughness = self.roughness.get([wx * 0.008, wy * 0.008]);
        let roughness = 0.5 + (raw_roughness + 1.0) * 0.5;

        let dither = self.threshold_dither.get([wx * 0.2, wy * 0.2]) * 0.04;

        BlockSample {
            height_bias,
            roughness,
            dither,
        }
    }

    pub fn sample_biome(&self, wx: f64, wy: f64) -> BiomeSample {
        let ws = self.sample(wx, wy);
        biome::sample_biome(ws.temperature, ws.moisture)
    }
}
