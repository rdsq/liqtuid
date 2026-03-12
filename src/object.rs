use rand::prelude::*;
use crate::random_colour::random_colours;

fn initial_layout(bottles: usize, depth: usize, rng: &mut rand::rngs::ThreadRng) -> Vec<Vec<usize>> {
    let mut layout = vec![];
    for i in 0..bottles {
        for _ in 0..depth {
            layout.push(i)
        }
    }
    layout.shuffle(rng);
    let chunks: Vec<Vec<usize>> = layout.chunks(depth).map(|x| x.to_vec()).collect();
    chunks
}

const KEYS: &str = "1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
const NO_NUM_KEYS: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

pub struct Liqtuid {
    rng: ThreadRng,
    pub bottles: Vec<Vec<usize>>,
    pub colours: Vec<u8>,
    pub depth: usize,
    pub selected: Option<usize>,
    pub moves: usize,
    pub keys: &'static str,
}

impl Liqtuid {
    pub fn new(number: usize, depth: usize, empty: usize, no_num_keys: bool) -> Self {
        let mut rng = rand::rng();
        let mut bottles = initial_layout(number, depth, &mut rng);
        for _ in 0..empty {
            bottles.push(vec![]);
        }
        let colours = random_colours(&mut rng, number);
        Self {
            rng,
            bottles,
            colours,
            depth,
            selected: None,
            moves: 0,
            keys: if no_num_keys { NO_NUM_KEYS } else { KEYS },
        }
    }
    pub fn click_on_index(&mut self, index: usize) {
        if index > self.bottles.len() - 1 {
            // we do not like this input, so we hide our emotions (follow for more psychological advice)
            return;
        }
        match self.selected {
            None => {
                if !self.bottles[index].is_empty() {
                    self.selected = Some(index);
                }
            },
            Some(selected) => {
                // welp, i will actually have to write the main logic now...
                if selected == index {
                    // remove selection
                    self.selected = None;
                    return; // this was a horrible thing to forget...
                } else if self.bottles[index].len() == self.depth {
                    // that bottle will not accept more
                    return;
                }
                let element = self.bottles[selected].last().expect("I was wrong in my calculations").clone();
                if self.bottles[index].last().and_then(|x| Some(*x == element)).unwrap_or(true) {
                    // we can then move it
                    // but we also move all the other same liquids underneath
                    while self.bottles[selected].last() == Some(&element) && self.bottles[index].len() < self.depth {
                        let we_steal_it_here = self.bottles[selected].pop().expect("at this point this is just cosmic rays if it does not work");
                        self.bottles[index].push(we_steal_it_here);
                    }
                    self.selected = None;
                    self.moves += 1;
                }
            },
        }
    }
    pub fn check_win(&self) -> bool {
        for bottle in &self.bottles {
            let length = bottle.len();
            let is_sorted_out = length == 0 || (length == self.depth && bottle.iter().all(|x| *x == bottle[0]));
            if !is_sorted_out {
                // well not equal
                return false;
            }
        }
        true // congrats
    }
    pub fn regenerate_colours(&mut self) {
        let num = self.colours.len();
        self.colours = random_colours(&mut self.rng, num);
    }
}
