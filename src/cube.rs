//mod utility;
//use utility::*;

use std::{collections::HashMap, hash::Hash};

// Move struct
#[derive(Debug, Clone)]
pub struct Move {
    pub co: [u8; 8],
    pub cp: [u8; 8],
    pub eo: [u8; 12],
    pub ep: [u8; 12],
}

// Slow cube (to create cube with lookup table)
#[derive(Debug)]
pub struct CubeSlow {
    pub co: [u8; 8],
    pub cp: [u8; 8],
    pub eo: Option<[u8; 12]>,
    pub ep: Option<[u8; 12]>,
    pub is_333: bool,
}

impl CubeSlow {
    // apply move to self
    pub fn apply_move(&self, m: &Move) -> CubeSlow {
        let mut co = [0; 8];
        let mut cp = [0; 8];
        let mut eo = None;
        let mut ep = None;

        // Update corners
        for i in 0..8 {
            cp[i] = self.cp[m.cp[i] as usize];
            co[i] = (self.co[m.cp[i] as usize] + m.co[i]) % 3;
        }

        // Update edges
        if self.is_333 {
            let mut eo_ = [0; 12];
            let mut ep_ = [0; 12];
            for i in 0..12 {
                ep_[i] = self.ep.as_ref().unwrap()[m.ep[i] as usize];
                eo_[i] = (self.eo.as_ref().unwrap()[m.ep[i] as usize] + m.eo[i]) % 2;
            }
            eo = Some(eo_);
            ep = Some(ep_);
        }

        CubeSlow {
            co: co,
            cp: cp,
            eo: eo,
            ep: ep,
            is_333: self.is_333,
        }
    }

    // only used to define double and inverse moves
    pub fn _to_move(&self) -> Move {
        Move {
            co: self.co,
            cp: self.cp,
            eo: if self.is_333 {
                *self.eo.as_ref().unwrap()
            } else {
                [0; 12]
            },
            ep: if self.is_333 {
                *self.ep.as_ref().unwrap()
            } else {
                [0; 12]
            },
        }
    }

    // Constructor
    pub fn new(is_333: bool) -> CubeSlow {
        CubeSlow {
            co: [0; 8],
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            eo: if is_333 { Some([0; 12]) } else { None },
            ep: if is_333 {
                Some([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
            } else {
                None
            },
            is_333: is_333,
        }
    }

    pub fn from_params(
        co: Option<[u8; 8]>,
        cp: Option<[u8; 8]>,
        eo: Option<[u8; 12]>,
        ep: Option<[u8; 12]>,
        is_333: bool,
    ) -> CubeSlow {
        CubeSlow {
            co: if co.is_some() { co.unwrap() } else { [0; 8] },
            cp: if cp.is_some() {
                cp.unwrap()
            } else {
                [0, 1, 2, 3, 4, 5, 6, 7]
            },
            eo: if eo.is_some() {
                Some(eo.unwrap())
            } else if is_333 {
                Some([0; 12])
            } else {
                None
            },
            ep: if ep.is_some() {
                Some(ep.unwrap())
            } else if is_333 {
                Some([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
            } else {
                None
            },
            is_333: is_333,
        }
    }
}

// Bunch of definitions of moves
// Using definitions from ksolve
// # CORNERS URF, ULF, ULB, URB, DRF, DLF, DLB, DRB
// # EDGES UF UL UB UR FR FL BL BR DF DL DB DR

pub const U: Move = Move {
    co: [0; 8],
    cp: [3, 0, 1, 2, 4, 5, 6, 7],
    eo: [0; 12],
    ep: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
};

pub const R: Move = Move {
    co: [1, 0, 0, 2, 2, 0, 0, 1],
    cp: [4, 1, 2, 0, 7, 5, 6, 3],
    eo: [0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1],
    ep: [0, 1, 2, 4, 11, 5, 6, 3, 8, 9, 10, 7],
};

pub const F: Move = Move {
    co: [2, 1, 0, 0, 1, 2, 0, 0],
    cp: [1, 5, 2, 3, 0, 4, 6, 7],
    eo: [0; 12],
    ep: [5, 1, 2, 3, 0, 8, 6, 7, 4, 9, 10, 11],
};

pub const D: Move = Move {
    co: [0; 8],
    cp: [0, 1, 2, 3, 5, 6, 7, 4],
    eo: [0; 12],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
};

pub const L: Move = Move {
    co: [0, 2, 1, 0, 0, 1, 2, 0],
    cp: [0, 2, 6, 3, 4, 1, 5, 7],
    eo: [0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0],
    ep: [0, 6, 2, 3, 4, 1, 9, 7, 8, 5, 10, 11],
};

pub const B: Move = Move {
    co: [0, 0, 2, 1, 0, 0, 1, 2],
    cp: [0, 1, 3, 7, 4, 5, 2, 6],
    eo: [0; 12],
    ep: [0, 1, 7, 3, 4, 5, 2, 10, 8, 9, 6, 11],
};
pub const U2: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [2, 3, 0, 1, 4, 5, 6, 7],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
};
pub const R2: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [7, 1, 2, 4, 3, 5, 6, 0],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 11, 7, 5, 6, 4, 8, 9, 10, 3],
};
pub const F2: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [5, 4, 2, 3, 1, 0, 6, 7],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [8, 1, 2, 3, 5, 4, 6, 7, 0, 9, 10, 11],
};
pub const B2: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 7, 6, 4, 5, 3, 2],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 10, 3, 4, 5, 7, 6, 8, 9, 2, 11],
};
pub const L2: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 6, 5, 3, 4, 2, 1, 7],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 9, 2, 3, 4, 6, 5, 7, 8, 1, 10, 11],
};
pub const D2: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 2, 3, 6, 7, 4, 5],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 10, 11, 8, 9],
};

pub const U3: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [1, 2, 3, 0, 4, 5, 6, 7],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
};
pub const R3: Move = Move {
    co: [1, 0, 0, 2, 2, 0, 0, 1],
    cp: [3, 1, 2, 7, 0, 5, 6, 4],
    eo: [0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1],
    ep: [0, 1, 2, 7, 3, 5, 6, 11, 8, 9, 10, 4],
};
pub const F3: Move = Move {
    co: [2, 1, 0, 0, 1, 2, 0, 0],
    cp: [4, 0, 2, 3, 5, 1, 6, 7],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [4, 1, 2, 3, 8, 0, 6, 7, 5, 9, 10, 11],
};
pub const B3: Move = Move {
    co: [0, 0, 2, 1, 0, 0, 1, 2],
    cp: [0, 1, 6, 2, 4, 5, 7, 3],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 6, 3, 4, 5, 10, 2, 8, 9, 7, 11],
};
pub const L3: Move = Move {
    co: [0, 2, 1, 0, 0, 1, 2, 0],
    cp: [0, 5, 1, 3, 4, 6, 2, 7],
    eo: [0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0],
    ep: [0, 5, 2, 3, 4, 9, 1, 7, 8, 6, 10, 11],
};
pub const D3: Move = Move {
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    cp: [0, 1, 2, 3, 7, 4, 5, 6],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10],
};

// Utility functions for orientation and permutation vectors

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        n => (1..n + 1).product(),
    }
}
pub struct Permutation {
    pub n: usize,
    pub pv: Vec<u8>,
}

impl Permutation {
    pub fn new(n: usize, pv: Vec<u8>) -> Permutation {
        Permutation { n: n, pv: pv }
    }

    pub fn to_int(&self) -> u32 {
        // Implementation from https://www.jaapsch.net/puzzles/compindx.htm
        let mut t: u64 = 0;
        for i in 1..self.n as usize {
            t *= (self.n - i + 1) as u64;
            for j in i + 1..self.n + 1 as usize {
                if self.pv[i - 1] > self.pv[j - 1] {
                    t += 1;
                }
            }
        }
        t as u32
    }

    pub fn from_int(N: u32, length: usize) -> Permutation {
        assert!(N < factorial(length as u32));
        let mut perm: Vec<u8> = vec![0; length];
        let mut n = N;
        for i in (1..length).rev() {
            perm[i - 1] = (n % ((length - i + 1) as u32)) as u8;
            n /= (length - i + 1) as u32;
            for j in i + 1..length + 1 {
                if perm[j - 1] >= perm[i - 1] {
                    perm[j - 1] += 1;
                }
            }
        }
        Permutation::new(length, perm)
    }
}
pub struct Orientation {
    pub n: usize,
    pub states: i8,
    pub ov: Vec<u8>,
}

impl Orientation {
    pub fn new(n: usize, states: i8, ov: Vec<u8>) -> Orientation {
        Orientation {
            n: n,
            states: states,
            ov: ov,
        }
    }
    pub fn to_int(&self) -> u16 {
        // Implementation from https://www.jaapsch.net/puzzles/compindx.htm
        let mut t: u16 = 0;
        for i in 0..self.n as usize {
            t *= self.states as u16;
            t += self.ov[i] as u16;
        }
        t
    }

    pub fn from_int(N: u16, length: usize, states: i8) -> Orientation {
        assert!(N < (states as u16).pow(length as u32) as u16);
        let mut ov = vec![0 as u8; length];
        let mut n = N;
        for i in (0..length).rev() {
            ov[i] = (n % states as u16) as u8;
            n /= states as u16;
        }
        Orientation::new(length, states, ov)
    }
}

// MoveTable struct for cube with lookup
#[derive(Clone)]
struct MoveTable {
    // table[i] = coord after applying move
    pub co_table: HashMap<u16, u16>,
    pub cp_table: HashMap<u32, u32>,
    pub eo_table: HashMap<u16, u16>,
    pub ep_table: HashMap<u32, u32>,
}

impl MoveTable {
    pub fn new() -> MoveTable {
        MoveTable {
            co_table: HashMap::new(),
            cp_table: HashMap::new(),
            eo_table: HashMap::new(),
            ep_table: HashMap::new(),
        }
    }
}
// Cube with move lookup
struct CubeFast<'a> {
    pub eo: u16,
    pub co: u16,
    pub cp: u32,
    pub ep: u32,
    pub move_info: &'a HashMap<&'a str, MoveTable>,
}

impl CubeFast<'_> {
    // Constructor
    pub fn new() -> CubeFast<'static> {
        // Create move table
        let moves: Vec<Move> = [
            U, R, F, D, L, B, U2, R2, F2, D2, L2, B2, U3, R3, F3, D3, L3, B3,
        ]
        .to_vec();
        let move_names: Vec<&str> = [
            "U", "R", "F", "D", "L", "B", "U2", "R2", "F2", "D2", "L2", "B2", "U'", "R'", "F'",
            "D'", "L'", "B'",
        ]
        .to_vec();
        let mut move_info: HashMap<&str, MoveTable> = HashMap::new();
        for (mv, mv_name) in moves.iter().zip(move_names.iter()) {
            let mut curr_move_table = MoveTable::new();
            // Update corners (orientation)
            for i in 0..3_u16.pow(8) {
                let co_curr = Orientation::from_int(i, 8, 3);
                let curr_cube = CubeSlow::from_params(
                    Some(co_curr.ov.try_into().unwrap()),
                    None,
                    None,
                    None,
                    true,
                );
                curr_cube.apply_move(&mv);
                curr_move_table
                    .co_table
                    .insert(i, Orientation::new(8, 3, curr_cube.co.to_vec()).to_int());
            }
            // Update corner (permutation)
            for i in 0..factorial(8) as u32 {
                let cp_curr = Permutation::from_int(i, 8);
                let curr_cube = CubeSlow::from_params(
                    None,
                    Some(cp_curr.pv.try_into().unwrap()),
                    None,
                    None,
                    true,
                );
                curr_cube.apply_move(&mv);
                curr_move_table
                    .cp_table
                    .insert(i, Permutation::new(8, curr_cube.cp.to_vec()).to_int());
            }
            // Update edges (orientation)
            for i in 0..2_u16.pow(12) {
                let eo_curr = Orientation::from_int(i, 12, 2);
                let curr_cube = CubeSlow::from_params(
                    None,
                    None,
                    Some(eo_curr.ov.try_into().unwrap()),
                    None,
                    true,
                );
                curr_cube.apply_move(&mv);
                curr_move_table.eo_table.insert(
                    i,
                    Orientation::new(12, 2, curr_cube.eo.unwrap().to_vec()).to_int(),
                );
            }
            // Update edges (permutation)
            for i in 0..factorial(12) {
                let ep_curr = Permutation::from_int(i, 12);
                let curr_cube = CubeSlow::from_params(
                    None,
                    None,
                    None,
                    Some(ep_curr.pv.try_into().unwrap()),
                    true,
                );
                curr_cube.apply_move(&mv);
                curr_move_table.ep_table.insert(
                    i,
                    Permutation::new(12, curr_cube.ep.unwrap().to_vec()).to_int(),
                );
            }
            move_info.insert(mv_name, curr_move_table);
        }
        CubeFast {
            eo: 0,
            co: 0,
            cp: 0,
            ep: 0,
            move_info: &(move_info.clone()),
        }
    }
}
