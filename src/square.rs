use std::fmt;

#[derive(Clone, Copy)]
pub struct NumberPossible {
    number: u32,
    possible: bool,
}

impl NumberPossible {
    pub fn new(value: u32) -> NumberPossible {
        NumberPossible {
            number: value,
            possible: true,
        }
    }

    pub fn set_possible(&mut self, value: bool) {
        self.possible = value;
    }
}

//TODO: I want to change this to something like a hashmap, so that we have access time 1, instead of acces time O(9) all the hecking time. HOWEVER rust is a little baby about copy methods on things with uncertain compile time. maybe a match function? I don't think that's any faster.
#[derive(Clone, Copy)]
pub struct Possiblities {
    possible_numbers: [NumberPossible; 9],
}

impl Possiblities {
    pub fn default() -> Possiblities {
        Possiblities {
            possible_numbers: [
                NumberPossible {
                    number: 1,
                    possible: true,
                },
                NumberPossible {
                    number: 2,
                    possible: true,
                },
                NumberPossible {
                    number: 3,
                    possible: true,
                },
                NumberPossible {
                    number: 4,
                    possible: true,
                },
                NumberPossible {
                    number: 5,
                    possible: true,
                },
                NumberPossible {
                    number: 6,
                    possible: true,
                },
                NumberPossible {
                    number: 7,
                    possible: true,
                },
                NumberPossible {
                    number: 8,
                    possible: true,
                },
                NumberPossible {
                    number: 9,
                    possible: true,
                },
            ],
        }
    }

    pub fn clear(&mut self) {
        for possible_number in self.possible_numbers.iter_mut() {
            possible_number.set_possible(false)
        }
    }

    pub fn reset(&mut self) {
        for possible_number in self.possible_numbers.iter_mut() {
            possible_number.set_possible(true);
        }
    }

    pub fn get_possible_numbers(&self) -> Vec<u32> {
        let mut possibile_nums: Vec<u32> = Vec::new();

        for possiblity in self.possible_numbers.iter() {
            if possiblity.possible {
                possibile_nums.push(possiblity.number);
            }
        }

        possibile_nums
    }

    pub fn contains(&self, number: u32) -> bool {
        self.possible_numbers[(number - 1) as usize].possible
    }

    pub fn remove(&mut self, number: u32) {
        self.possible_numbers[(number - 1) as usize].possible = false;
    }
}

#[derive(Clone, Copy)]
pub struct Square {
    value: u32,
    possibilities: Possiblities,
}

impl Square {
    pub fn default() -> Square {
        Square {
            value: 0,
            possibilities: Possiblities::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn set_value(&mut self, input_value: u32) {
        self.value = input_value;
        if (input_value == 0) {
            self.possibilities.reset();
        } else {
            self.possibilities.clear();
        }
    }

    pub fn get_possible_numbers(&self) -> Vec<u32> {
        self.possibilities.get_possible_numbers()
    }

    pub fn remove_possibility(&mut self, number: u32) {
        self.possibilities.remove(number);
    }

    //this doesn't stricktly need to be be mutable, but is used in callback functions that also take mutable
    pub fn number_possible(&mut self, number: u32) -> bool {
        self.possibilities.contains(number)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        if self.value == 0 {
            print!("[ ]");
        } else {
            print!("[{}]", self.value);
        }
        Ok(())
    }
}
