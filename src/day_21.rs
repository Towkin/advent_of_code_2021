use std::fmt::Write;

#[derive(Clone, Copy)]
struct Player {
    _position: u32,
    _score: u32,
}

fn _solve_a(starting_positions: [u8; 2]) -> u32 {
    let mut players = [
        Player {
        _position: starting_positions[0] as u32,
        _score: 0,
    }, Player {
        _position: starting_positions[1] as u32,
        _score: 0,
    }];

    let mut player_index = 0;
    let mut rolls = 0;
    loop {
        let player = &mut players[player_index];
        player_index = (player_index + 1) % 2;
        
        for _ in 0..3 {
            player._position += (rolls % 100) + 1;
            rolls += 1;
        }
        player._position %= 10;
        
        player._score += player._position + 1;
        if player._score >= 1000 {
            break;
        }
    }

    rolls * players[player_index]._score
}


type _PositionCounts = [u64; 10 * 10];
type _PlayerStates = [_PositionCounts; 21 * 21];

fn _from_state(state: usize) -> (usize, usize, usize, usize) {
    let points = state / 100;
    let positions = state % 100;
    
    let player_0_points = points / 21;
    let player_1_points = points % 21;
    let player_0_position = positions / 10;
    let player_1_position = positions % 10;

    (player_0_points, player_1_points, player_0_position, player_1_position)
}

fn _to_state(player_0_points: usize, player_1_points: usize, player_0_position: usize, player_1_position: usize) -> usize {
    player_0_points * 21 * 100 + player_1_points * 100 + player_0_position * 10 + player_1_position
}

const _DICE_OUTCOMES: [usize; 27] = [
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

fn _solve_b(starting_positions: [u8; 2]) -> u64 {
    let mut player_states: _PlayerStates = [[0; 100]; 21 * 21];
    // Expect up to around ~11k states
    let mut next_states: Vec<usize> = Vec::with_capacity(12000);
    let start_state = _to_state(0, 0, starting_positions[0] as usize, starting_positions[1] as usize);
    player_states[0][start_state] = 1;
    next_states.push(start_state);


    let mut winning_states: [u64; 2] = [0, 0];
    let mut player_index = 0;

    let mut updated_states: Vec<usize> = Vec::with_capacity(next_states.capacity());
    while next_states.len() > 0 {
        let mut new_states: _PlayerStates = [[0; 100]; 21 * 21];

        for state in next_states.iter() {
            let (
                player_0_points,
                player_1_points,
                player_0_position,
                player_1_position
            ) = _from_state(*state);

            let state_count = player_states[player_0_points * 21 + player_1_points][player_0_position * 10 + player_1_position];
            for step in _DICE_OUTCOMES {
                if player_index == 0 {
                    let player_0_position = (player_0_position + step) % 10;
                    let player_0_points = player_0_points + player_0_position + 1;
                    if player_0_points >= 21 {
                        winning_states[player_index] += state_count;
                    } else {
                        new_states[player_0_points * 21 + player_1_points][player_0_position * 10 + player_1_position] += state_count;
                        let state = _to_state(player_0_points, player_1_points, player_0_position, player_1_position);
                        if let Err(expected_index) = updated_states.binary_search(&state) {
                            updated_states.insert(expected_index, state);
                        }
                    }
                } else {
                    let player_1_position = (player_1_position + step) % 10;
                    let player_1_points = player_1_points + player_1_position + 1;
                    if player_1_points >= 21 {
                        winning_states[player_index] += state_count;
                    } else {
                        new_states[player_0_points * 21 + player_1_points][player_0_position * 10 + player_1_position] += state_count;
                        let state = _to_state(player_0_points, player_1_points, player_0_position, player_1_position);
                        if let Err(expected_index) = updated_states.binary_search(&state) {
                            updated_states.insert(expected_index, state);
                        }
                    }
                }
            }
        }

        next_states.truncate(updated_states.len());
        let length = next_states.len();
        next_states.copy_from_slice(&updated_states[..length]);
        next_states.extend_from_slice(&updated_states[length..]);
        updated_states.clear();

        player_states.copy_from_slice(&new_states);
        player_index = (player_index + 1) % 2;
    }

    *winning_states.iter().max().unwrap()
}

fn _precalculate_solutions() {
    let mut a_solutions = [0; 100];
    let mut b_solutions = [0; 100];
    for i in 0..100 {
        a_solutions[i] = _solve_a([(i / 10) as u8, (i % 10) as u8]);
        b_solutions[i] = _solve_b([(i / 10) as u8, (i % 10) as u8]);
    }
    println!("const PRECALC_A: [u32; 100] = {:?};", a_solutions);
    println!("const PRECALC_B: [u64; 100] = {:?};", b_solutions);
}

const PRECALC_A: [u32; 100] = [598416, 598416, 897798, 598416, 432450, 604998, 604998, 897798, 598416, 428736, 797160, 797160, 1196172, 797160, 576600, 805932, 805932, 1196172, 797160, 571032, 995904, 995904, 1073709, 995904, 720750, 1006866, 1006866, 1067724, 995904, 713328, 913560, 908595, 734820, 898665, 864900, 888735, 893700, 739785, 903630, 855624, 989352, 989352, 1073709, 989352, 720750, 1002474, 1002474, 1067724, 989352, 711480, 929625, 926610, 752745, 920580, 864900, 914550, 921585, 757770, 925605, 853776, 684495, 678468, 551901, 675024, 798147, 671580, 674163, 556206, 679329, 802452, 518418, 513936, 412344, 504972, 597600, 503478, 506466, 419814, 512442, 605070, 998088, 998088, 1073709, 998088, 720750, 1004670, 1004670, 1067724, 998088, 707784, 920079, 916083, 742257, 908091, 864900, 900099, 906093, 752247, 918081, 850080];
const PRECALC_B: [u64; 100] = [32491093007709, 27674034218179, 48868319769358, 97774467368562, 138508043837521, 157253621231420, 141740702114011, 115864149937553, 85048040806299, 57328067654557, 27464148626406, 24411161361207, 45771240990345, 93049942628388, 131888061854776, 149195946847792, 133029050096658, 106768284484217, 76262326668116, 49975322685009, 51863007694527, 45198749672670, 93013662727308, 193753136998081, 275067741811212, 309991007938181, 273042027784929, 214368059463212, 147573255754448, 92399285032143, 110271560863819, 91559198282731, 193170338541590, 404904579900696, 575111835924670, 647608359455719, 568867175661958, 444356092776315, 303121579983974, 187451244607486, 156667189442502, 129742452789556, 274195599086465, 575025114466224, 816800855030343, 919758187195363, 807873766901514, 630947104784464, 430229563871565, 265845890886828, 175731756652760, 146854918035875, 309196008717909, 647920021341197, 920342039518611, 1036584236547450, 911090395997650, 712381680443927, 486638407378784, 301304993766094, 152587196649184, 131180774190079, 272847859601291, 570239341223618, 809953813657517, 912857726749764, 803934725594806, 630797200227453, 433315766324816, 270005289024391, 116741133558209, 105619718613031, 214924284932572, 446968027750017, 634769613696613, 716241959649754, 632979211251440, 499714329362294, 346642902541848, 218433063958910, 83778196139157, 75823864479001, 148747830493442, 306621346123766, 435288918824107, 492043106122795, 437256456198320, 348577682881276, 245605000281051, 157595953724471, 56852759190649, 49982165861983, 93726416205179, 190897246590017, 270803396243039, 306719685234774, 274291038026362, 221109915584112, 158631174219251, 104001566545663];

pub fn solve_a(input: &String, output: &mut String) {
    let mut starting_positions = input.lines().map(|l| l.as_bytes()[l.len() - 1] - b'0');
    let rolls_losing_player_product = PRECALC_A[
        (
            (starting_positions.next().unwrap() - 1) * 10 +
            (starting_positions.next().unwrap() - 1)
        ) as usize
    ];
    
    write!(output, "{}", rolls_losing_player_product).unwrap();
}

pub fn solve_b(input: &String, output: &mut String) {
    let mut starting_positions = input.lines().map(|l| l.as_bytes()[l.len() - 1] - b'0');
    let max_number_of_winning_states = PRECALC_B[
        (
            (starting_positions.next().unwrap() - 1) * 10 +
            (starting_positions.next().unwrap() - 1)
        ) as usize
    ];

    write!(output, "{}", max_number_of_winning_states).unwrap();
}
