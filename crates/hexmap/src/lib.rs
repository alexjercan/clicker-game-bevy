use glam::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HexMap {
    pub size: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Direction {
    SouthEast,
    NorthEast,
    North,
    NorthWest,
    SouthWest,
    South,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::SouthEast,
            1 => Direction::NorthEast,
            2 => Direction::North,
            3 => Direction::NorthWest,
            4 => Direction::SouthWest,
            5 => Direction::South,
            _ => panic!("Invalid direction"),
        }
    }
}

impl HexMap {
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    pub fn cube_to_axial(&self, cube: IVec3) -> IVec2 {
        IVec2::new(cube.x, cube.y)
    }

    pub fn axial_to_cube(&self, axial: IVec2) -> IVec3 {
        IVec3::new(axial.x, axial.y, -axial.x - axial.y)
    }

    pub fn cube_direction(&self, direction: Direction) -> IVec3 {
        match direction {
            Direction::North => IVec3::new(0, -1, 1),
            Direction::NorthEast => IVec3::new(1, -1, 0),
            Direction::SouthEast => IVec3::new(1, 0, -1),
            Direction::South => IVec3::new(0, 1, -1),
            Direction::SouthWest => IVec3::new(-1, 1, 0),
            Direction::NorthWest => IVec3::new(-1, 0, 1),
        }
    }

    pub fn cube_neighbor(&self, hex: IVec3, direction: Direction) -> IVec3 {
        hex + self.cube_direction(direction)
    }

    pub fn cube_ring(&self, center: IVec3, radius: u32) -> Vec<IVec3> {
        let mut results = vec![];
        let mut hex = center + self.cube_direction(Direction::SouthWest) * radius as i32;

        for d in 0..6u8 {
            for _ in 0..radius {
                results.push(hex);
                hex = self.cube_neighbor(hex, d.into());
            }
        }

        results
    }

    pub fn axial_ring(&self, center: IVec2, radius: u32) -> Vec<IVec2> {
        let center = self.axial_to_cube(center);

        self.cube_ring(center, radius).iter().map(|&cube| self.cube_to_axial(cube)).collect()
    }

    pub fn cube_spiral(&self, center: IVec3, radius: u32) -> Vec<IVec3> {
        let mut results = vec![center];

        for r in 1..=radius {
            let ring = self.cube_ring(center, r);
            results.extend(ring);
        }

        results
    }

    pub fn axial_spiral(&self, center: IVec2, radius: u32) -> Vec<IVec2> {
        let center = self.axial_to_cube(center);

        self.cube_spiral(center, radius).iter().map(|&cube| self.cube_to_axial(cube)).collect()
    }

    pub fn axial_to_pixel(&self, hex: IVec2) -> Vec2 {
        let size = self.size;
        let x = size * 3.0f32.sqrt() * (hex.x as f32 + hex.y as f32 / 2.0);
        let y = size * 3.0 / 2.0 * hex.y as f32;

        Vec2::new(x, y)
    }

    pub fn pixel_to_axial(&self, point: Vec2) -> IVec2 {
        let size = self.size;
        let q = 2.0 / 3.0 * point.y / size;
        let r = (-1.0 / 3.0 * point.y + 3.0f32.sqrt() / 3.0 * point.x) / size;

        // Axial to cube
        let s = -q - r;

        // Cube round
        let mut q_rounded = q.round();
        let mut r_rounded = r.round();
        let mut s_rounded = s.round();

        let q_diff = (q - q_rounded).abs();
        let r_diff = (r - r_rounded).abs();
        let s_diff = (s - s_rounded).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q_rounded = -r_rounded - s_rounded;
        } else if r_diff > s_diff {
            r_rounded = -q_rounded - s_rounded;
        } else {
            s_rounded = -q_rounded - r_rounded;
        }

        // cube to axial
        let q_axial = q_rounded;
        let r_axial = r_rounded;
        let _ = s_rounded;

        IVec2::new(q_axial as i32, r_axial as i32)
    }
}
