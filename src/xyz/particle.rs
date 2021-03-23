/**
 * Add support for parsing id, position and velocity in line string of xyz file.
 */

use std::iter::Iterator;

use std::str::FromStr;
use xyzio::Error;

type Real = f64;

pub struct Particle {
    pub id: u32,
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub vx: Real,
    pub vy: Real,
    pub vz: Real,
}

impl Particle {
    pub(crate) fn is_near_eq(&self, another: &Particle, error_limit: f64) -> bool {
        (self.x - another.x).abs() < error_limit &&
            (self.y - another.y).abs() < error_limit &&
            (self.z - another.z).abs() < error_limit &&
            (self.vx - another.vx).abs() < error_limit &&
            (self.vy - another.vy).abs() < error_limit &&
            (self.vz - another.vz).abs() < error_limit
    }
    // compare atoms position under periodic boundary checking
    pub(crate) fn is_near_eq_with_pbc(&self, another: &Particle, error_limit: f64, (box_x, box_y, box_z): (f64, f64, f64)) -> bool {
        return ((self.x - another.x).abs() < error_limit || ((self.x - another.x).abs() - box_x).abs() < error_limit)
            && ((self.y - another.y).abs() < error_limit || ((self.y - another.y).abs() - box_y).abs() < error_limit)
            && ((self.z - another.z).abs() < error_limit || ((self.z - another.z).abs() - box_z).abs() < error_limit)
            && (self.vx - another.vx).abs() < error_limit
            && (self.vy - another.vy).abs() < error_limit
            && (self.vz - another.vz).abs() < error_limit;
    }
}

impl FromStr for Particle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split_whitespace().collect();
        if splitted.len() != 7 {
            return Err(Error::IllegalState(String::from("")));
        }
        Ok(Particle {
            id: splitted[0].parse::<Real>()? as u32,
            x: splitted[1].parse::<Real>()?,
            y: splitted[2].parse::<Real>()?,
            z: splitted[3].parse::<Real>()?,
            vx: splitted[4].parse::<Real>()?,
            vy: splitted[6].parse::<Real>()?,
            vz: splitted[6].parse::<Real>()?,
        })
    }
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("id: {}, position: ({}, {}, {}), v:({}, {}, {})",
                self.id, self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xyz::xyz_reader::Reader;

    #[test]
    fn test_reader() {
        let data: &[u8] = b"\
            3
            commnet
            1 1.0 2.0 3.0 1.0 2.0 3.0
            2 4.0 3.0 6.0 4.0 3.0 6.0
            3 5.0 1.5 4.0 5.0 1.5 4.0";
        let mut reader = Reader::new(data);
        let success = reader.read_snapshot::<Particle>();
        assert!(success.is_ok());

        let snapshot = success.unwrap();
        assert_eq!(3, snapshot.size());
        assert_eq!(3, snapshot.atoms[2].id);
    }
}
