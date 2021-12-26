use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub enum Position {
    Hallway(usize),
    SideRoom(usize,usize)
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub struct State {
    hallway: [char ; 11],
    rooms: [Vec<char> ; 4]
}

impl State {
    pub fn init(rooms: [Vec<char> ; 4]) -> State {
        State {
            hallway: ['.' ; 11],
            rooms
        }
    }

    pub fn get(&self, p: &Position) -> char {
        match *p {
            Position::Hallway(a) =>
                self.hallway[a],
            Position::SideRoom(i,x) =>
                self.rooms[i][x]
        }
    }

    pub fn get_mut(&mut self, p: &Position) -> &mut char {
        match *p {
            Position::Hallway(a) =>
                &mut self.hallway[a],
            Position::SideRoom(i,x) =>
                &mut self.rooms[i][x]
        }
    }

    pub fn set(&mut self, p: &Position, c: char) {
        *self.get_mut(p) = c
    }

    pub fn is_free(&self, p: &Position) -> bool {
        self.get(p) == '.'
    }

    #[allow(clippy::comparison_chain)]
    pub fn is_hallway_free(&self, p1: usize, p2: usize) -> bool {
        if p1 < p2 {
            ((p1+1)..=p2).all(|p| self.hallway[p] == '.')
        }
        else if p1 > p2 {
            (p2..=(p1-1)).all(|p| self.hallway[p] == '.')
        }
        else {
            true
        }
    }

    pub fn mov(&mut self, src: &Position, dst: &Position) -> u32 {
        let c = {
                let src_c = self.get_mut(src);
                let t = *src_c;
                *src_c = '.';
                t
            };
        
        let dst_c = self.get_mut(dst);
        assert_eq!(*dst_c, '.');
        *dst_c = c;

        dist(src,dst) as u32 * cost(c)
    }
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub struct StateAndCost {
    state: State,
    pred: State,
    cost: u32
}

impl StateAndCost {
    pub fn mov(&mut self, src: &Position, dst: &Position) {
        self.cost += self.state.mov(src, dst);
         
    }

    pub fn copy_and_mov(&self, src: &Position, dst: &Position) -> Self {
        let mut other = self.clone();
        other.mov(src, dst);
        other.pred = self.state.clone();
        other
    }
}


impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "#{}#", String::from_iter(self.hallway))?;
        let get = |i,j| (&self.rooms[i as usize] as &Vec<char>)[j as usize];
        writeln!(f, "###{}#{}#{}#{}###", get(0,0), get(1,0), get(2,0), get(3,0))?;
        for x in 1..self.rooms[0].len() {
            writeln!(f, "  #{}#{}#{}#{}#", get(0,x), get(1,x), get(2,x), get(3,x))?;
        }
        writeln!(f, "  #########")
    }
}

impl Ord for StateAndCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StateAndCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn ndist(x: usize, y: usize) -> usize {
    if x > y {x - y} else {y - x}
}

fn cost(c: char) -> u32 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!()
    }
}

fn dist(src: &Position, dst: &Position) -> usize {
    match *src {
        Position::SideRoom(i,x) =>
            match *dst {
                Position::SideRoom(j,y) =>
                    if i == j {
                        ndist(x,y)
                    }
                    else {
                        ndist(i,j) * 2 + x + y + 2
                    },
                Position::Hallway(b) => (x+1) + ndist(2*i+2,b)
            }
        Position::Hallway(a) =>
            match *dst {
                Position::SideRoom(j,y) => (y+1) + ndist(2*j+2,a),
                Position::Hallway(b) => ndist(a,b)
            }
    }
}

fn count() {
    static mut COUNT: u64 = 0;
    unsafe {
        COUNT += 1;
        if COUNT % 1000 == 0 { 
            println!("Explored {} states", COUNT);
        }
    }
}

fn show_preds(preds: &HashMap<State, (State,u32)>, state: &State) {
    if let Some((pred,cost)) = preds.get(state) {
        if pred != state {
            show_preds(preds, pred);
        }
        println!("{}\ncost:{}\n", state, cost);
    }
}

pub fn solve(input: [Vec<char> ; 4]) -> u32 {
    let initial = State::init(input);
    let mut states = HashSet::new();
    let mut pending: BinaryHeap<StateAndCost> = BinaryHeap::new();
    let mut preds = HashMap::new();
    pending.push(StateAndCost{state: initial.clone(), pred: initial, cost:0});

    while let Some(state_and_cost) = pending.pop() {
        if states.get(&state_and_cost.state).is_some() {
            continue;
        }
        states.insert(state_and_cost.state.clone());
        preds.insert(state_and_cost.state.clone(), (state_and_cost.pred.clone(), state_and_cost.cost));
        let state = &state_and_cost.state;

        // --- Are there free rooms ? ---

        let mut free_rooms: [usize ; 4] = [0 ; 4];
        let mut finished = true;
        for (i,room) in state.rooms.iter().enumerate() {
            let c = (b'A' + i as u8) as char;
            for x in (0..room.len()).rev() {
                if room[x] == '.' {
                    free_rooms[i] = x + 1;
                    finished = false;
                    break;
                } else if room[x] != c {
                    free_rooms[i] = 0;
                    finished = false;
                    break;
                }
            }
        }

        if finished {
            println!("Found a solution !");
            show_preds(&preds, state);
            return state_and_cost.cost;
        }

        // --- First, try to empty the hallway ---

        let mut priority_changes = false;

        for (a,&c) in state.hallway.iter().enumerate() {
            match c {
                '.' => (),
                _ => {
                    let i = (c as u8 - b'A') as usize;
                    let b = 2 * i + 2;
                    if free_rooms[i] > 0 && state.is_hallway_free(a, b) {
                        let src = Position::Hallway(a);
                        let dst = Position::SideRoom(i, free_rooms[i]-1);
                        let new_state = state_and_cost.copy_and_mov(&src, &dst);
                        pending.push(new_state);
                        priority_changes = true;
                        break;
                    }
                }
            }
        }

        if priority_changes {
            continue;
        }

        // --- Then, leave rooms ---

        for i in 0..=3 {
            let room = &state.rooms[i];
           
            let x =
                if let Some(x) = room.iter().position(|&c| c != '.') {
                    x
                } else {
                    continue;
                };
            let a = 2 * i + 2;
            let src = Position::SideRoom(i,x);
            let c = room[x];

            // Is it possible to go to dest room ?
            let j = (c as u8 - b'A') as usize;
            let b = 2 * j + 2;

            if i != j && free_rooms[j] > 0 && state.is_hallway_free(a, b) {
                let dst = Position::SideRoom(j, free_rooms[j]-1);
                let new_state = state_and_cost.copy_and_mov(&src, &dst);
                pending.push(new_state);
            }

            // Otherwise, go to one of the free spot of the hallway
            if i != j || free_rooms[j] == 0 { // Do not leave if already in place
                for b in [0,1,3,5,7,9,10] {
                    if state.is_hallway_free(a, b) {
                        let dst = Position::Hallway(b);
                        let new_state = state_and_cost.copy_and_mov(&src, &dst);
                        pending.push(new_state);
                    }
                }
            }
        }

        count();
    }

    panic!("No solution found")
}


#[test]
fn test23_0() {
    let solution = solve([
        vec!['B','A'],
        vec!['C','D'],
        vec!['B','C'],
        vec!['D','A']]);
    assert_eq!(solution, 12521);
}

#[test]
fn test23_1() {
    let solution = solve([
        vec!['D','C'],
        vec!['A','A'],
        vec!['D','B'],
        vec!['C','B']]);
    assert_eq!(solution, 14546);
}

#[test]
fn test23_2() {
    let solution = solve([
        vec!['B','D','D','A'],
        vec!['C','C','B','D'],
        vec!['B','B','A','C'],
        vec!['D','A','C','A']]);
    assert_eq!(solution, 44169);
}

#[test]
fn test23_3() {
    let solution = solve([
        vec!['D','D','D','C'],
        vec!['A','C','B','A'],
        vec!['D','B','A','B'],
        vec!['C','A','C','B']]);
    assert_eq!(solution, 42308);
}

