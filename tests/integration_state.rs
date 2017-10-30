extern crate taxi;

use taxi::state::*;
use taxi::world::World;
use taxi::actions::Actions;


#[test]
#[should_panic(expected = "'C'")]
fn build_fails_unknown_passenger() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│G . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│Y .│B .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let w = World::build_from_str(source_world).unwrap();
    State::build(&w, (1, 3), Some('C'), 'B').unwrap();
}

#[test]
#[should_panic(expected = "'Q'")]
fn build_fails_unknown_destination() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│G . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│Y .│B .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let w = World::build_from_str(source_world).unwrap();
    State::build(&w, (1, 3), Some('Y'), 'Q').unwrap();
}

#[test]
#[should_panic(expected = "(1,6)")]
fn build_fails_invalid_taxi() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│G . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│Y .│B .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let w = World::build_from_str(source_world).unwrap();
    State::build(&w, (1, 6), Some('R'), 'B').unwrap();
}

#[test]
fn output_matches_str_simple() {
    let mut source_world = String::new();
    source_world += "     \n";
    source_world += " R . \n";
    source_world += "     \n";
    source_world += " . G \n";
    source_world += "     \n";

    let mut expected_str = String::new();
    expected_str += "     \n";
    expected_str += " d . \n";
    expected_str += "     \n";
    expected_str += " t p \n";
    expected_str += "     \n";

    let w = World::build_from_str(&source_world).unwrap();
    let state = State::build(&w, (0, 1), Some('G'), 'R').unwrap();

    let output = state.display(&w);
    assert_eq!(output, expected_str);
}


#[test]
fn output_matches_str_passenger_in_taxi() {
    let mut source_world = String::new();
    source_world += "     \n";
    source_world += " R . \n";
    source_world += "     \n";
    source_world += " . G \n";
    source_world += "     \n";

    let mut expected_str = String::new();
    expected_str += "     \n";
    expected_str += " d . \n";
    expected_str += "     \n";
    expected_str += " T . \n";
    expected_str += "     \n";

    let w = World::build_from_str(&source_world).unwrap();
    let state = State::build(&w, (0, 1), None, 'R').unwrap();

    let output = state.display(&w);
    assert_eq!(output, expected_str);
}

#[test]
fn output_matches_str_complex() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│. .│B .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_initial_str = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│t .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let world = World::build_from_str(source_world).unwrap();
    let initial_state = State::build(&world, (1, 3), Some('R'), 'B').unwrap();

    let initial_str = initial_state.display(&world);
    assert_eq!(expected_initial_str, initial_str);
}

#[test]
fn move_allowed_north() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_north = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. t . . .│\n\
        │         │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    let world = World::build_from_str(source_world).unwrap();
    let initial_state = State::build(&world, (1, 3), Some('R'), 'G').unwrap();

    let mut state_north = initial_state;
    state_north.apply_action(&world, Actions::North);
    assert_eq!(expected_north, state_north.display(&world));
}

#[test]
fn move_top_north() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_north = "\
        ┌───┬─────┐\n\
        │p t│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │         │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 0), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_north = state;
                    state_north.apply_action(&w, Actions::North);
                    assert_eq!(expected_north, state_north.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_wall_north() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_north = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│t .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 3), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_north = state;
                    state_north.apply_action(&w, Actions::North);
                    assert_eq!(expected_north, state_north.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_allowed_south() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_south = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . t .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (3, 1), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_south = state;
                    state_south.apply_action(&w, Actions::South);
                    assert_eq!(expected_south, state_south.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_bottom_south() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_south = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │t│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (0, 4), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_south = state;
                    state_south.apply_action(&w, Actions::South);
                    assert_eq!(expected_south, state_south.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_wall_south() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_south = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. t . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 2), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_south = state;
                    state_south.apply_action(&w, Actions::South);
                    assert_eq!(expected_south, state_south.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_allowed_east() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_east = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . t . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 2), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_east = state;
                    state_east.apply_action(&w, Actions::East);
                    assert_eq!(expected_east, state_east.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_right_east() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_east = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . t│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (3, 1), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_east = state;
                    state_east.apply_action(&w, Actions::East);
                    assert_eq!(expected_east, state_east.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_wall_east() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_east = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. t│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 1), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_east = state;
                    state_east.apply_action(&w, Actions::East);
                    assert_eq!(expected_east, state_east.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_allowed_west() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_west = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │t .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 1), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_west = state;
                    state_west.apply_action(&w, Actions::West);
                    assert_eq!(expected_west, state_west.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_left_west() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_west = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │t . . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (1, 2), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_west = state;
                    state_west.apply_action(&w, Actions::West);
                    assert_eq!(expected_west, state_west.display(&w));
                }
            }
        }
    }
}

#[test]
fn move_wall_west() {
    let source_world = "\
        ┌───┬─────┐\n\
        │R .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    let expected_west = "\
        ┌───┬─────┐\n\
        │p .│. . .│\n\
        │   │     │\n\
        │. .│. . .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│d .│\n\
        │ │   │   │\n\
        │.│. .│t .│\n\
        └─┴───┴───┘\n\
        ";


    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (3, 4), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    let mut state_west = state;
                    state_west.apply_action(&w, Actions::West);
                    assert_eq!(expected_west, state_west.display(&w));
                }
            }
        }
    }
}

#[test]
fn reaches_destination() {
    let source_world = "\
        ┌───┬─────┐\n\
        │. .│. . .│\n\
        │   │     │\n\
        │. .│. R .│\n\
        │         │\n\
        │. . . . .│\n\
        │ ┌─      │\n\
        │.│. .│G .│\n\
        │ │   │   │\n\
        │.│. .│. .│\n\
        └─┴───┴───┘\n\
        ";

    // ┌───┬─────┐
    // │. .│. . .│
    // │   │     │
    // │. .│t p .│
    // │         │
    // │. . . . .│
    // │         │
    // │.│. .│d .│
    // │ │   │   │
    // │.│. .│. .│
    // └─┴───┴───┘

    match World::build_from_str(source_world) {
        Err(msg) => panic!(msg),
        Ok(w) => {
            match State::build(&w, (2, 1), Some('R'), 'G') {
                Err(msg) => panic!(msg),
                Ok(state) => {
                    println!("");
                    println!("{}", state.display(&w));

                    let mut result0 = state;
                    result0.apply_action(&w, Actions::East);
                    println!("0:\n{}", result0.display(&w));
                    assert_eq!(result0.at_destination(), false);

                    let mut result1 = result0;
                    result1.apply_action(&w, Actions::PickUp);
                    println!("1:\n{}", result1.display(&w));
                    assert_eq!(result1.at_destination(), false);

                    let mut result2 = result1;
                    result2.apply_action(&w, Actions::South);
                    println!("2:\n{}", result2.display(&w));
                    assert_eq!(result2.at_destination(), false);

                    let mut result3 = result2;
                    result3.apply_action(&w, Actions::South);
                    println!("3:\n{}", result3.display(&w));
                    assert_eq!(result3.at_destination(), false);

                    let mut result4 = result3;
                    result4.apply_action(&w, Actions::DropOff);
                    println!("4:\n{}", result4.display(&w));
                    assert_eq!(result4.at_destination(), true);
                }
            }
        }
    }
}

#[test]
fn movement_reward() {
    let source_world = "\
        ┌─────┐\n\
        │R . G│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. Y .│\n\
        └─────┘\n\
        ";

    let expected_initial_str = "\
        ┌─────┐\n\
        │p . d│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. t .│\n\
        └─────┘\n\
        ";

    let world = World::build_from_str(source_world).unwrap();
    let initial_state = State::build(&world, (1, 2), Some('R'), 'G').unwrap();

    assert_eq!(expected_initial_str, initial_state.display(&world));


    let mut north_state = initial_state;
    let north_reward = north_state.apply_action(&world, Actions::North);
    assert_eq!(-1.0, north_reward);

    let mut south_state = initial_state;
    let south_reward = south_state.apply_action(&world, Actions::South);
    assert_eq!(-1.0, south_reward);

    let mut east_state = initial_state;
    let east_reward = east_state.apply_action(&world, Actions::East);
    assert_eq!(-1.0, east_reward);

    let mut west_state = initial_state;
    let west_reward = west_state.apply_action(&world, Actions::West);
    assert_eq!(-1.0, west_reward);
}

#[test]
fn correct_pickup_reward() {
    let source_world = "\
        ┌─────┐\n\
        │R . G│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. Y .│\n\
        └─────┘\n\
        ";

    let expected_initial_str = "\
        ┌─────┐\n\
        │p . d│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. . .│\n\
        └─────┘\n\
        ";

    let world = World::build_from_str(source_world).unwrap();
    let initial_state = State::build(&world, (0, 0), Some('R'), 'G').unwrap();

    assert_eq!(expected_initial_str, initial_state.display(&world));

    let mut pickup_state = initial_state;
    let pickup_reward = pickup_state.apply_action(&world, Actions::PickUp);
    assert_eq!(0.0, pickup_reward);
}

#[test]
fn incorrect_pickup_reward() {
    let source_world = "\
        ┌─────┐\n\
        │R . G│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. Y .│\n\
        └─────┘\n\
        ";

    let world = World::build_from_str(source_world).unwrap();

    let expected_off_passenger_str = "\
        ┌─────┐\n\
        │p . d│\n\
        │     │\n\
        │. t .│\n\
        │     │\n\
        │. . .│\n\
        └─────┘\n\
        ";

    let mut off_passenger_state = State::build(&world, (1, 1), Some('R'), 'G').unwrap();

    assert_eq!(
        expected_off_passenger_str,
        off_passenger_state.display(&world)
    );
    assert_eq!(
        -10.0,
        off_passenger_state.apply_action(&world, Actions::PickUp)
    );

    let expected_has_passenger_str = "\
        ┌─────┐\n\
        │. . d│\n\
        │     │\n\
        │. T .│\n\
        │     │\n\
        │. . .│\n\
        └─────┘\n\
        ";

    let mut has_passenger_state = State::build(&world, (1, 1), None, 'G').unwrap();

    assert_eq!(
        expected_has_passenger_str,
        has_passenger_state.display(&world)
    );
    assert_eq!(
        -10.0,
        has_passenger_state.apply_action(&world, Actions::PickUp)
    );

    let expected_wrong_fp_str = "\
        ┌─────┐\n\
        │p . d│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. t .│\n\
        └─────┘\n\
        ";

    let mut wrong_fp_state = State::build(&world, (1, 2), Some('R'), 'G').unwrap();

    assert_eq!(expected_wrong_fp_str, wrong_fp_state.display(&world));
    assert_eq!(-10.0, wrong_fp_state.apply_action(&world, Actions::PickUp));
}

#[test]
fn incorrect_dropoff_reward() {
    let source_world = "\
        ┌─────┐\n\
        │R . G│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. Y .│\n\
        └─────┘\n\
        ";

    let world = World::build_from_str(source_world).unwrap();

    let expected_no_passenger_str = "\
        ┌─────┐\n\
        │p . d│\n\
        │     │\n\
        │. t .│\n\
        │     │\n\
        │. . .│\n\
        └─────┘\n\
        ";

    let mut no_passenger_state = State::build(&world, (1, 1), Some('R'), 'G').unwrap();

    assert_eq!(
        expected_no_passenger_str,
        no_passenger_state.display(&world)
    );

    assert_eq!(
        -10.0,
        no_passenger_state.apply_action(&world, Actions::DropOff)
    );

    let expected_no_passenger_on_dest_str = "\
        ┌─────┐\n\
        │p . d│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. . .│\n\
        └─────┘\n\
        ";

    let mut no_passenger_on_deststate = State::build(&world, (2, 0), Some('R'), 'G').unwrap();

    assert_eq!(
        expected_no_passenger_on_dest_str,
        no_passenger_on_deststate.display(&world)
    );

    assert_eq!(
        -10.0,
        no_passenger_on_deststate.apply_action(&world, Actions::DropOff)
    );

    let expected_passenger_off_fp_str = "\
        ┌─────┐\n\
        │. . d│\n\
        │     │\n\
        │. T .│\n\
        │     │\n\
        │. . .│\n\
        └─────┘\n\
        ";

    let mut passenger_off_fp_state = State::build(&world, (1, 1), None, 'G').unwrap();

    assert_eq!(
        expected_passenger_off_fp_str,
        passenger_off_fp_state.display(&world)
    );

    assert_eq!(
        -10.0,
        passenger_off_fp_state.apply_action(&world, Actions::DropOff)
    );

    let expected_passenger_wrong_fp_str = "\
        ┌─────┐\n\
        │. . d│\n\
        │     │\n\
        │. . .│\n\
        │     │\n\
        │. T .│\n\
        └─────┘\n\
        ";

    let mut passenger_wrong_fp_state = State::build(&world, (1, 2), None, 'G').unwrap();

    assert_eq!(
        expected_passenger_wrong_fp_str,
        passenger_wrong_fp_state.display(&world)
    );

    assert_eq!(
        -10.0,
        passenger_wrong_fp_state.apply_action(&world, Actions::DropOff)
    );
}
