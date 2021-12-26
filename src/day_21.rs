use std::fmt::Write;

#[derive(Clone, Copy)]
struct Player {
    position: u32,
    score: u32,
}

pub fn solve_a(input: &String, output: &mut String) {
    let mut starting_positions = input.lines().map(|l| l.as_bytes()[l.len() - 1] - b'0');
    let mut players = [
        Player {
        position: starting_positions.next().unwrap() as u32 - 1,
        score: 0,
    }, Player {
        position: starting_positions.next().unwrap() as u32 - 1,
        score: 0,
    }];

    let mut player_index = 0;
    let mut rolls = 0;
    loop {
        let player = &mut players[player_index];
        player_index = (player_index + 1) % 2;
        
        for _ in 0..3 {
            player.position += (rolls % 100) + 1;
            rolls += 1;
        }
        player.position %= 10;
        
        player.score += player.position + 1;
        if player.score >= 1000 {
            break;
        }
    }

    let rolls_losing_player_product = rolls * players[player_index].score;
    write!(output, "{}", rolls_losing_player_product).unwrap();
}

type PositionCounts = [u64; 10 * 10];
type PlayerStates = [PositionCounts; 21 * 21];

const DICE_OUTCOMES: [usize; 27] = [
    1 + 1 + 1, 1 + 2 + 1, 1 + 3 + 1,
    1 + 1 + 2, 1 + 2 + 2, 1 + 3 + 2,
    1 + 1 + 3, 1 + 2 + 3, 1 + 3 + 3,
    2 + 1 + 1, 2 + 2 + 1, 2 + 3 + 1,
    2 + 1 + 2, 2 + 2 + 2, 2 + 3 + 2,
    2 + 1 + 3, 2 + 2 + 3, 2 + 3 + 3,
    3 + 1 + 1, 3 + 2 + 1, 3 + 3 + 1,
    3 + 1 + 2, 3 + 2 + 2, 3 + 3 + 2,
    3 + 1 + 3, 3 + 2 + 3, 3 + 3 + 3,
];

pub fn solve_b(input: &String, output: &mut String) {
    let mut player_states: PlayerStates = [[0; 100]; 21 * 21];
    
    {
        let mut starting_positions = input.lines().map(|l| l.as_bytes()[l.len() - 1] - b'0');
        let starting_positions = [
            starting_positions.next().unwrap() as usize - 1,
            starting_positions.next().unwrap() as usize - 1
        ];
        player_states[0][
            starting_positions[0] * 10 + starting_positions[1]
        ] = 1;
    }

    let mut winning_states: [u64; 2] = [0, 0];
    let mut player_index = 0;

    while player_states.iter().any(|states| states.iter().any(|state_count| *state_count > 0)) {
        let mut new_states: PlayerStates = [[0; 100]; 21 * 21];
        for player_0_points in 0..21 {
            for player_1_points in 0..21 {
                for player_0_position in 0..10 {
                    for player_1_position in 0..10 {
                        let state_count = player_states[player_0_points * 21 + player_1_points][player_0_position * 10 + player_1_position];
                        for step in DICE_OUTCOMES {
                            if player_index == 0 {
                                let player_0_position = (player_0_position + step) % 10;
                                let player_0_points = player_0_points + player_0_position + 1;
                                if player_0_points >= 21 {
                                    winning_states[player_index] += state_count;
                                } else {
                                    new_states[player_0_points * 21 + player_1_points][player_0_position * 10 + player_1_position] += state_count;
                                }
                            } else {
                                let player_1_position = (player_1_position + step) % 10;
                                let player_1_points = player_1_points + player_1_position + 1;
                                if player_1_points >= 21 {
                                    winning_states[player_index] += state_count;
                                } else {
                                    new_states[player_0_points * 21 + player_1_points][player_0_position * 10 + player_1_position] += state_count;
                                }
                            }
                        }
                    }
                }
            }
        }

        player_states.copy_from_slice(&new_states);
        player_index = (player_index + 1) % 2;
    }

    write!(output, "{}", winning_states.iter().max().unwrap()).unwrap();
}
