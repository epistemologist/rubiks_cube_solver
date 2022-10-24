use std::ops::Add;

pub fn factorial(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Permutation {
    pub n: usize,
    pub pv: Vec<i8>
}

impl Add for Permutation {
    
    type Output = Permutation;

    fn add(self, rhs: Permutation) -> Permutation {
        assert_eq!(self.n, rhs.n);
        // Instantiate a vector of 0s of length n
        let mut v = vec![0 as i8; self.n as usize];
        for i in 0..self.n as usize{
            v[i] = self.pv[rhs.pv[i] as usize];
        }
        Permutation{n: self.n, pv: v}
    }
}



impl Permutation {
    pub fn new(n: usize, pv: Vec<i8>) -> Permutation {
        Permutation{n: n, pv: pv}
    }

    pub fn identity(n: usize) -> Permutation {
        let mut v = vec![0 as i8; n as usize];
        for i in 0..n as usize{
            v[i] = i as i8;
        }
        Permutation{n: n, pv: v}
    }

    pub fn inverse(&self) -> Permutation {
        let mut v = vec![0 as i8; self.n as usize];
        for i in 0..self.n as usize{
            v[self.pv[i] as usize] = i as i8;
        }
        Permutation{n: self.n, pv: v}
    }

    pub fn to_int(&self) -> u128 {
        // Implementation from https://www.jaapsch.net/puzzles/compindx.htm
        let mut t: u128 = 0;
        for i in 1..self.n as usize {
            t *= (self.n - i + 1) as u128;
            for j in i+1..self.n+1 as usize {
                if self.pv[i-1] > self.pv[j-1] {
                    t += 1;
                }
            }
        }
        t
    }

    pub fn from_int(N: u128, length: usize) -> Permutation {
        assert!(N < factorial(length as u128));
        let mut perm: Vec<i8> = vec![0; length];
        let mut n = N;
        for i in (1..length).rev() {
            perm[i-1] = (n % ((length - i + 1) as u128)) as i8;
            n /= (length - i + 1) as u128;
            for j in i+1..length+1 {
                if perm[j-1] >= perm[i-1] {
                    perm[j-1] += 1;
                }
            }
        }
        Permutation::new(length, perm)
    } 

}




#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Orientation {
    pub n: usize,
    pub states: i8,
    pub ov: Vec<i8>
}

impl Add for Orientation {
    type Output = Orientation;
    fn add(self, rhs: Orientation) -> Orientation {
        // ( p1 + p2 )(x) = p2(p1(x))
        assert_eq!(self.n, rhs.n);
        assert_eq!(self.states, rhs.states);
        let mut v = vec![0 as i8; self.n as usize];
        for i in 0..self.n as usize{
            v[i] = (self.ov[i] + rhs.ov[i]) % self.states;
        }
        Orientation{n: self.n, states: self.states, ov: v}
    }
}

impl Orientation {
    pub fn new(n: usize, states: i8, ov: Vec<i8>) -> Orientation {
        Orientation{n: n, states: states, ov: ov}
    }

    pub fn identity(n: usize, states: i8) -> Orientation {
        let mut v = vec![0 as i8; n as usize];
        for i in 0..n as usize{
            v[i] = 0;
        }
        Orientation{n: n, states: states, ov: v}
    }

    pub fn inverse(&self) -> Orientation {
        let mut v = vec![0 as i8; self.n as usize];
        for i in 0..self.n as usize{
            v[i] = (self.states - self.ov[i]) % self.states;
        }
        Orientation{n: self.n, states: self.states, ov: v}
    }

    pub fn to_int(&self) -> u128 {
        // Implementation from https://www.jaapsch.net/puzzles/compindx.htm
        let mut t: u128 = 0;
        for i in 0..self.n as usize{
            t *= self.states as u128;
            t += self.ov[i] as u128;
        }
        t
    }

    pub fn from_int(N: u128, length: usize, states: i8) -> Orientation {
        assert!(N < (states as u128).pow(length as u32) as u128);
        let mut ov = vec![0; length];
        let mut n = N;
        for i in (0..length).rev() {
            ov[i] = (n % states as u128) as i8;
            n /= states as u128;
        }
        Orientation::new(length, states, ov)
    }

}




// Make test cases
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compose_permutations() {
        let p1 = Permutation::new(5, vec![0, 1, 2, 4, 3]);
        let p2 = Permutation::new(5, vec![0, 1, 3, 2, 4]);
        let p3 = Permutation::new(5, vec![0, 1, 4, 2, 3]);

        /*
        0 1 2 3 4 // identity
        0 1 2 4 3 // swap 3 and 4
        0 1 4 2 3 // swap 2 and 3
        */
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_permutation_to_from_int() {
        for i in 0..factorial(6) as u128 {
            let p = Permutation::from_int(i, 6);
            assert_eq!(p.to_int(), i);
        }
    }

    #[test]
    fn test_compose_orientations() {
        let o1 = Orientation::new(5, 3, vec![0, 1, 2, 0, 1]);
        let o2 = Orientation::new(5, 3, vec![0, 1, 0, 1, 2]);
        let o3 = Orientation::new(5, 3, vec![0, 2, 2, 1, 0]);

        assert_eq!(o1 + o2, o3);
    }

    #[test]
    fn test_orientation_to_from_int() {
        for i in 0..3u128.pow(6) {
            let o = Orientation::from_int(i, 6, 3);
            assert_eq!(o.to_int(), i);
        }
    }


    
}
// Create a hashset of integers in Rust
