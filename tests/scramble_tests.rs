use scramble_gen::{Cube, Move, MoveFace, MoveType, MoveWidth, Scramble};

#[test]
fn test_3x3_scramble_default_length() {
    let scramble = Scramble::generate(Cube::ThreeByThree, None);
    assert_eq!(scramble.moves.len(), 20);
}

#[test]
fn test_3x3_scramble_custom_length() {
    let scramble = Scramble::generate(Cube::ThreeByThree, Some(15));
    assert_eq!(scramble.moves.len(), 15);
}

#[test]
fn test_4x4_scramble_default_length() {
    let scramble = Scramble::generate(Cube::FourByFour, None);
    assert_eq!(scramble.moves.len(), 40);
}

#[test]
fn test_4x4_scramble_custom_length() {
    let scramble = Scramble::generate(Cube::FourByFour, Some(30));
    assert_eq!(scramble.moves.len(), 30);
}

#[test]
fn test_3x3_moves_are_single_width() {
    let scramble = Scramble::generate(Cube::ThreeByThree, Some(10));
    for move_ in &scramble.moves {
        assert!(matches!(move_.move_width, MoveWidth::Single));
    }
}

#[test]
fn test_4x4_has_various_widths() {
    let scramble = Scramble::generate(Cube::FourByFour, Some(100));
    let has_wide = scramble
        .moves
        .iter()
        .any(|m| matches!(m.move_width, MoveWidth::Wide));
    let has_slice = scramble
        .moves
        .iter()
        .any(|m| matches!(m.move_width, MoveWidth::Slice));

    // With 100 moves, we should have at least some wide and slice moves
    assert!(has_wide || has_slice);
}

#[test]
fn test_move_display() {
    let move_ = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Prime,
        move_width: MoveWidth::Single,
    };
    assert_eq!(format!("{}", move_), "R'");

    let wide_move = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Double,
        move_width: MoveWidth::Wide,
    };
    assert_eq!(format!("{}", wide_move), "Rw2");

    let slice_move = Move {
        move_face: MoveFace::Right,
        move_type: MoveType::Normal,
        move_width: MoveWidth::Slice,
    };
    assert_eq!(format!("{}", slice_move), "r");
}

#[test]
fn test_move_face_display() {
    assert_eq!(format!("{}", MoveFace::Left), "L");
    assert_eq!(format!("{}", MoveFace::Right), "R");
    assert_eq!(format!("{}", MoveFace::Up), "U");
    assert_eq!(format!("{}", MoveFace::Down), "D");
    assert_eq!(format!("{}", MoveFace::Front), "F");
    assert_eq!(format!("{}", MoveFace::Back), "B");
}

#[test]
fn test_move_type_display() {
    assert_eq!(format!("{}", MoveType::Normal), "");
    assert_eq!(format!("{}", MoveType::Prime), "'");
    assert_eq!(format!("{}", MoveType::Double), "2");
}

#[test]
fn test_scramble_display() {
    let moves = vec![
        Move {
            move_face: MoveFace::Right,
            move_type: MoveType::Normal,
            move_width: MoveWidth::Single,
        },
        Move {
            move_face: MoveFace::Up,
            move_type: MoveType::Prime,
            move_width: MoveWidth::Single,
        },
    ];
    let scramble = Scramble { moves };
    let output = format!("{}", scramble);
    assert!(output.contains("R "));
    assert!(output.contains("U' "));
}
