use rand::prelude::*;
use random_color::RandomColor;
use ansi_colours::ansi256_from_rgb;

pub fn initial_layout(bottles: usize, depth: usize, rng: &mut rand::rngs::ThreadRng) -> Vec<Vec<usize>> {
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

pub fn random_colours(num: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let mut generator_probably = RandomColor::new();
    for _ in 0..num {
        let rgb_arr = generator_probably.to_rgb_array();
        let rgb = rgb::RGB8 { r: rgb_arr[0], g: rgb_arr[1], b: rgb_arr[2] };
        let index = ansi256_from_rgb(rgb);
        result.push(index);
    }
    result
}

pub struct Liqtui {
    pub bottles: Vec<Vec<usize>>,
    pub colours: Vec<u8>,
    pub depth: usize,
    pub selected: Option<usize>,
}

impl Liqtui {
    pub fn new(number: usize, depth: usize, empty: usize) -> Self {
        let mut rng = rand::rng();
        let mut bottles = initial_layout(number, depth, &mut rng);
        for _ in 0..empty {
            bottles.push(vec![]);
        }
        Self {
            bottles,
            colours: random_colours(number),
            depth,
            selected: None,
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
}
