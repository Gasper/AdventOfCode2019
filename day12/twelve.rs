extern crate itertools;
extern crate num;

use itertools::Itertools;
use num::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Vec3D {
    x: i64, y: i64, z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    position: Vec3D,
    velocity: Vec3D,
}

fn main() {

    let mut moons = Vec::new();
    // From input
    moons.push(Moon{position: Vec3D {x: 5, y: 4, z: 4}, velocity: Vec3D {x: 0, y: 0, z: 0}});
    moons.push(Moon{position: Vec3D {x: -11, y: -11, z: -3}, velocity: Vec3D {x: 0, y: 0, z: 0}});
    moons.push(Moon{position: Vec3D {x: 0, y: 7, z: 0}, velocity: Vec3D {x: 0, y: 0, z: 0}});
    moons.push(Moon{position: Vec3D {x: -13, y: 2, z: 10}, velocity: Vec3D {x: 0, y: 0, z: 0}});

    // Example 1
    /*moons.push(Moon{position: Vec3D {x: -1, y: 0, z: 2}, velocity: Vec3D {x: 0, y: 0, z: 0}});
    moons.push(Moon{position: Vec3D {x: 2, y: -10, z: -7}, velocity: Vec3D {x: 0, y: 0, z: 0}});
    moons.push(Moon{position: Vec3D {x: 4, y: -8, z: 8}, velocity: Vec3D {x: 0, y: 0, z: 0}});
    moons.push(Moon{position: Vec3D {x: 3, y: 5, z: -1}, velocity: Vec3D {x: 0, y: 0, z: 0}});*/

    let initial_moons = moons.clone();

    let mut its: u64 = 0;
    let mut repeat = vec![false, false, false];
    let mut frequency = vec![0, 0, 0];

    while !(repeat[0] && repeat[1] && repeat[2]) {
        let snapped_moons = moons.clone();

        for pair_index in (0..4).combinations(2) {
            moons[pair_index[0]].apply_gravity(&snapped_moons[pair_index[1]]);
            moons[pair_index[1]].apply_gravity(&snapped_moons[pair_index[0]]);
        }

        for moon in &mut moons {
            moon.apply_velocity();
        }

        its += 1;

        if moons[0].position.x == initial_moons[0].position.x &&
            moons[1].position.x == initial_moons[1].position.x &&
            moons[2].position.x == initial_moons[2].position.x &&
            moons[3].position.x == initial_moons[3].position.x && !repeat[0] {
            repeat[0] = true;
            frequency[0] = its + 1;
        }

        if moons[0].position.y == initial_moons[0].position.y &&
            moons[1].position.y == initial_moons[1].position.y &&
            moons[2].position.y == initial_moons[2].position.y &&
            moons[3].position.y == initial_moons[3].position.y && !repeat[1] {
            repeat[1] = true;
            frequency[1] = its + 1;
        }

        if moons[0].position.z == initial_moons[0].position.z &&
            moons[1].position.z == initial_moons[1].position.z &&
            moons[2].position.z == initial_moons[2].position.z &&
            moons[3].position.z == initial_moons[3].position.z && !repeat[2] {
            repeat[2] = true;
            frequency[2] = its + 1;
        }
    }

    let total_energy = moons.into_iter()
        .map(|moon| moon.total_energy())
        .fold(0, |a, b| a + b);
    
    println!("Total energy at the end: {}", total_energy);
    println!("Iterations needed: {}", its);
    println!("Frequencies: {:?}", frequency);
    println!("Common frequency: {}", frequency[0].lcm(&frequency[1]).lcm(&frequency[2]));
}

impl Moon {
    fn apply_gravity(&mut self, other: &Moon) {
        if other.position.x > self.position.x {
            self.velocity.x += 1;
        } else if other.position.x < self.position.x {
            self.velocity.x -= 1;
        }

        if other.position.y > self.position.y {
            self.velocity.y += 1;
        } else if other.position.y < self.position.y {
            self.velocity.y -= 1;
        }

        if other.position.z > self.position.z {
            self.velocity.z += 1;
        } else if other.position.z < self.position.z {
            self.velocity.z -= 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn total_energy(&self) -> i64 {
        let potential_energy = self.position.x.abs() + 
            self.position.y.abs() + self.position.z.abs();
        
        let kinetic_energy = self.velocity.x.abs() + 
            self.velocity.y.abs() + self.velocity.z.abs();

        return potential_energy * kinetic_energy;
    }
}
