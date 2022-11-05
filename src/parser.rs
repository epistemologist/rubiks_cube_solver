use regex::Regex;
use std::collections::HashMap;

// Idea is to have a parser for Rubik's cube algorithms
// that can be used to generate a sequence of moves

// We also need to deal with cube rotations

// Let the default orientation of the cube be as follows:
// Order of faces: U, R, F, D, L, B

pub fn parse_333_algorithm(alg: &str) -> Vec<String> {
    println!("alg: {}", alg);
    let move_regex = Regex::new(r"[URFBLDurfbldxyzMES]['2]{0,1}").unwrap();
    // Replace slice moves and wide turns with just turns and rotations
    let alg_no_slice = alg
        .replace("r", "R M'")
        .replace("l", "L M")
        .replace("u", "U E'")
        .replace("d", "D E")
        .replace("f", "F S")
        .replace("b", "B S'")
        .replace("r'", "R' M")
        .replace("l'", "L' M'")
        .replace("u'", "U' E")
        .replace("d'", "D' E'")
        .replace("f'", "F' S'")
        .replace("b'", "B' S")
        .replace("u2", "U2 E2")
        .replace("d2", "D2 E2")
        .replace("f2", "F2 S2")
        .replace("b2", "B2 S2")
        .replace("r2", "R2 M2")
        .replace("l2", "L2 M2")
        .replace("M", "L' R x'")
        .replace("M'", "L R' x")
        .replace("M2", "L2 R2 x2")
        .replace("E", "U D' y'")
        .replace("E'", "U' D y")
        .replace("E2", "U2 D2 y2")
        .replace("S", "F' B z")
        .replace("S'", "F B' z'")
        .replace("S2", "F2 B2 z2");
    let ROT_MAP: HashMap<&str, [u8; 6]> = [
        ("x", [2, 1, 3, 0, 4, 5]),  // x: F -> U -> B -> D
        ("x'", [5, 1, 0, 2, 4, 3]), // x': F -> D -> B -> U
        ("x2", [3, 1, 5, 0, 4, 2]), // x2: F -> B, U -> D
        ("y", [0, 5, 1, 3, 2, 4]),  // y: F -> L -> B -> R
        ("y'", [0, 2, 4, 3, 5, 1]), // y': F -> R -> B -> L
        ("y2", [0, 4, 5, 3, 1, 2]), // y2: F -> B, L -> R
        ("z", [4, 0, 2, 1, 3, 5]),  // z:  U -> R -> D -> L
        ("z'", [1, 3, 2, 4, 0, 5]), // z': U -> L -> D -> R
        ("z2", [3, 4, 2, 0, 1, 5]), // z2: U -> D, L -> R
    ]
    .iter()
    .cloned()
    .collect();
    let mut moves: Vec<String> = Vec::new();
    let mut curr_orientation = ['U', 'R', 'F', 'D', 'L', 'B'];
    for mat in move_regex.find_iter(&alg_no_slice) {
        let m = mat.as_str();
        if (ROT_MAP.contains_key(m)) {
            let rot = ROT_MAP.get(m).unwrap();
            let mut tmp = ['0'; 6];
            for i in 0..6 as usize {
                tmp[i] = curr_orientation[rot[i] as usize];
            }
            curr_orientation = tmp;
        } else {
            let move_str = [
                curr_orientation["URFDLB".find(m[0..1].chars().nth(0).unwrap()).unwrap()]
                    .to_string(),
                if (m.len() == 1) {
                    "1".to_string()
                } else {
                    match &m[1..2] {
                        "2" => "2".to_string(),
                        "'" => "3".to_string(),
                        _ => panic!("Invalid move!"),
                    }
                },
            ]
            .join("");
            println!("{:?}", move_str);
            moves.push(move_str);
        }
    }
    moves
}

// Test cases

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn test_parse_333_algorithm() {
        let alg = "R U R' U' R' F R2 U' R' U' R U R' F'";
        let moves = parse_333_algorithm(alg);
        assert_eq!(
            moves,
            vec![
                "R1", "U1", "R3", "U3", "R3", "F1", "R2", "U3", "R3", "U3", "R1", "U1", "R3", "F3"
            ]
        );
    }

    #[test]
    fn test_parse_333_algorithm_rotations() {
        let test_cases = [
            ("x R U R'", vec!["R1", "F1", "R3"]),
            ("x' R U R'", vec!["R1", "B1", "R3"]),
            ("x2 R U R'", vec!["R1", "D1", "R3"]),
            ("y R U R'", vec!["B1", "U1", "B3"]),
            ("y' R U R'", vec!["F1", "U1", "F3"]),
            ("y2 R U R'", vec!["L1", "U1", "L3"]),
            ("z R U R'", vec!["U1", "L1", "U3"]),
            ("z' R U R'", vec!["D1", "R1", "D3"]),
            ("z2 R U R'", vec!["L1", "D1", "L3"]),
        ];
        for (alg, moves) in test_cases.iter() {
            assert_eq!(parse_333_algorithm(alg), *moves);
        }

        assert_eq!(
            parse_333_algorithm("y x' R U' R' D R U R' D' R U R' D R U' R' D' x"),
            vec![
                "B1", "L3", "B3", "R1", "B1", "L1", "B3", "R3", "B1", "L1", "B3", "R1", "B1", "L3",
                "B3", "R3"
            ]
        );
    }
}
