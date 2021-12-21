use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Clone,Hash)]
struct Player {
    score: u32,
    space: u32
}

#[derive(Debug,PartialEq,Eq,Clone,Hash)]
struct Game {
    players: [Player ; 2],
    current_player: usize
}

fn init(p1_start: u32, p2_start: u32) -> Game {
    Game {
        players: [
            Player { space: p1_start, score: 0 },
            Player { space: p2_start, score: 0 }
        ],
        current_player: 0
    }
}

fn step(game: &mut Game, rolls: &[u32], verbose : bool) -> u32 {
    let player = &mut game.players[game.current_player];

    player.space = (player.space + rolls.iter().sum::<u32>() - 1) % 10 + 1;
    player.score += player.space;

    if verbose {
        let rolls : Vec<String> = rolls.iter().map(|x| x.to_string()).collect();
        println!(
            "Player {} rolls {} an moves to space {} for a total \
             score of {}.",
            game.current_player + 1,
            rolls.join("+"),
            player.space,
            player.score);
    }

    player.score
}

fn part1(initial: &Game) -> u32 {
    println!("--- Part 1 ---");

    let mut game = initial.clone();
    let mut roll_count = 0;
    let mut last_roll = 0;
    let mut roll = || {
        roll_count += 1;
        last_roll = last_roll % 100 + 1;
        last_roll
    };

    let (winner, solution) = loop {
        let rolls : Vec<u32> = (0..3).map(|_| roll()).collect();

        if step(&mut game, &rolls, true) >= 1000 {
            let low_score = game.players[(game.current_player+1)%2].score;
            break (game.current_player + 1, low_score * roll_count);
        }

        game.current_player = (game.current_player + 1) % 2;
    };

    println!("Player {} wins !", winner);
    println!("Player 1  {} - {}  Player 2",
        game.players[0].score, game.players[1].score);
    println!("{} rolls, solution is {}", roll_count, solution);

    solution
}

fn explore_universes(cache: &mut HashMap<Game,[u64 ; 2]>, game: Game) -> [u64 ; 2] {
    if let Some(r) = cache.get(&game) {
        return *r;
    }

    let mut wins = [0 ; 2];
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                let mut game = game.clone();
                let rolls = [r1, r2, r3];
                if step(&mut game, &rolls, false) >= 21 {
                    wins[game.current_player] += 1;
                } else {
                    game.current_player = (game.current_player + 1) % 2;
                    let w = explore_universes(cache, game);
                    wins[0] += w[0];
                    wins[1] += w[1];
                }
            }
        }
    }

    cache.insert(game, wins);
    wins
}

fn part2(initial: &Game) -> u64 {
    println!("--- Part 2 ---");

    let game = initial.clone();
    let mut cache = HashMap::new();
    let wins = explore_universes(&mut cache, game);

    println!("Player 1 wins in {} universes", wins[0]);
    println!("Player 2 wins in {} universes", wins[1]);

    *wins.iter().max().unwrap()
}

pub fn solve(p1_start: u32, p2_start: u32) -> (u32,u64) {
    let initial = init(p1_start, p2_start);
    (part1(&initial),part2(&initial))
}


#[test]
fn test21_0() {
    let solution = solve(4,8);
    assert_eq!(solution, (739785,444356092776315));
}

#[test]
fn test21_1() {
    let solution = solve(6,2);
    assert_eq!(solution, (926610,146854918035875));
}
