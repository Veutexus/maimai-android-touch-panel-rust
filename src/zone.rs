use image::RgbImage;
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

/// The 7-group zone layout matching the maimai touch panel.
/// Each sub-array is one byte in the serial packet (bit-packed).
pub const ZONE_LAYOUT: &[&[&str]] = &[
    &["A1", "A2", "A3", "A4", "A5"],
    &["A6", "A7", "A8", "B1", "B2"],
    &["B3", "B4", "B5", "B6", "B7"],
    &["B8", "C1", "C2", "D1", "D2"],
    &["D3", "D4", "D5", "D6", "D7"],
    &["D8", "E1", "E2", "E3", "E4"],
    &["E5", "E6", "E7", "E8"],
];

/// Pre-computed circle offsets for area sampling.
struct CircleOffsets {
    offsets: Vec<(i32, i32)>,
}

impl CircleOffsets {
    fn new(radius: u32, num_points: usize) -> Self {
        let angle_inc = 2.0 * PI / num_points as f64;
        let offsets = (0..num_points)
            .map(|i| {
                let angle = i as f64 * angle_inc;
                let dx = (radius as f64 * angle.cos()) as i32;
                let dy = (radius as f64 * angle.sin()) as i32;
                (dx, dy)
            })
            .collect();
        Self { offsets }
    }
}

/// Handles image-based zone detection by sampling pixels and mapping RGB colors to zones.
pub struct ZoneLookup {
    image: RgbImage,
    color_to_zone: HashMap<String, String>,
    circle: CircleOffsets,
}

impl ZoneLookup {
    pub fn new(
        image: RgbImage,
        zone_colors: HashMap<String, String>,
        area_scope: u32,
        area_point_num: usize,
    ) -> Self {
        Self {
            image,
            color_to_zone: zone_colors,
            circle: CircleOffsets::new(area_scope, area_point_num),
        }
    }

    /// Samples the center pixel plus points on a circle, returns all matching zone names.
    pub fn lookup_zones(&self, x: i32, y: i32) -> HashSet<String> {
        let (w, h) = (self.image.width() as i32, self.image.height() as i32);
        let mut zones = HashSet::new();

        // Center pixel
        if x >= 0 && x < w && y >= 0 && y < h {
            let key = pixel_to_key(self.image.get_pixel(x as u32, y as u32));
            if let Some(zone) = self.color_to_zone.get(&key) {
                zones.insert(zone.clone());
            }
        }

        // Circle points
        for &(dx, dy) in &self.circle.offsets {
            let px = x + dx;
            let py = y + dy;
            if px >= 0 && px < w && py >= 0 && py < h {
                let key = pixel_to_key(self.image.get_pixel(px as u32, py as u32));
                if let Some(zone) = self.color_to_zone.get(&key) {
                    zones.insert(zone.clone());
                }
            }
        }

        zones
    }

    /// Converts a set of touched zone names into a binary grid matching ZONE_LAYOUT.
    /// Each inner Vec has one entry per zone in that group: 1 if touched, 0 otherwise.
    pub fn zones_to_grid(touched: &HashSet<String>) -> Vec<Vec<u8>> {
        ZONE_LAYOUT
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|zone| if touched.contains(*zone) { 1 } else { 0 })
                    .collect()
            })
            .collect()
    }
}

fn pixel_to_key(pixel: &image::Rgb<u8>) -> String {
    format!("{}-{}-{}", pixel[0], pixel[1], pixel[2])
}
