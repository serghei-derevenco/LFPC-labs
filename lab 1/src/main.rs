use std::fmt;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

fn read_states(state_list:&mut StateList, filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        for (line_index, c) in line.chars().enumerate() {
            if index == 0 && c != ' ' {
                state_list.append_state(State::new(c));
            }
            if index == 0 && c != ' ' && state_list.list.len() == 1 {
                state_list.list[line_index].start_state = true;
            }
            if index == 1 {
                state_list.read_values(&line);
            }
        }
    }
}

fn check_string(state_list: StateList, content: String) -> bool {
    let mut curr = state_list.get_starting_state().unwrap();
    for chr in content.chars() {
        if curr.directions.contains_key(&chr.to_string()) {
            let dir = curr.directions.get(&chr.to_string()).unwrap();
            let ind = state_list.get_state_by_name(*dir).unwrap();
            curr = &state_list.list[ind];
        } else {
            return false;
        }
    }
    return true;
}

#[derive(Debug)]
struct State {
    name: String,
    directions: HashMap<String, char>,
    start_state: bool,
    end_state: bool,
}

impl State {
    fn new(name: char) -> State {
        State {
            name: name.to_string(),
            directions: HashMap::new(),
            start_state: false,
            end_state: false,
        }
    }
}

struct StateList {
    list: Vec<State>,
    capacity: usize,
    values: Vec<char>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{:?}\nstart state: {}\nend state: {}", self.name, self.directions, self.start_state, self.end_state)
    }
}

impl StateList {
    fn new() -> StateList {
        StateList {
            list: Vec::new(),
            capacity: 0,
            values: Vec::new(),
        }
    }

    fn append_state(&mut self, new: State) {
        self.list.push(new);
        self.capacity += 1;
    }

    fn read_values(&mut self, val: &String) {
        for c in val.chars() {
            if c != ' ' {
                self.values.push(c);
            }
        }
    }

    fn get_state_by_name(&self, name: char) -> Option<usize> {
        if self.list.is_empty() {
            return None;
        }
        let mut curr: Option<usize> = Some(0);
        for (index, state) in self.list.iter().enumerate() {
            if state.name == name.to_string() {
                curr = Some(index);
            }
        }
        curr
    }

    fn has_state_by_name(&self, name: char) -> bool {
        for state in self.list.iter() {
            if state.name == name.to_string() {
                return true;
            }
        }
        return false;
    }

    fn read_directions(&mut self, filename: String) {
        let mut curr: Option<usize> = None;
        let mut dir: char = '`';
        let mut val: char = '`';
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            for (line_index, c) in line.chars().enumerate() {
                if index >= 2 && c != ' ' && c != '>' {
                    if line_index == 0 {
                        curr = self.get_state_by_name(c);
                    } else {
                        if self.values.contains(&c) {
                            val = c;
                        }
                        if c == '0' {
                            dir = '0';
                            self.list[curr.unwrap()].end_state = true;
                        } else if self.has_state_by_name(c) {
                            dir = c;
                        }
                    }
                }
            }
            if dir != '`' && val != '`' {
                self.list[curr.unwrap()].directions.insert(val.to_string(), dir);
            }
        }
    }

    fn get_starting_state(&self) -> Option<&State> {
        for st in self.list.iter() {
            if st.start_state == true {
                return Some(st);
            }
        }
        None
    }
}

impl fmt::Display for StateList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for st in self.list.iter() {
            write!(f, "{}\n", st)?;
        }
        write!(f, " ")
    }
}

fn main() {
    let mut state_list = StateList::new();
    read_states(&mut state_list, "grammar.txt".to_string());
    state_list.read_directions("grammar.txt".to_string());

    println!("{}", check_string(state_list, "bbdb".to_string()));
}