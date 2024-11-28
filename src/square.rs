
use std::fmt;

#[derive(Clone, Copy)]
pub struct Number_Possible {
    number: u32,
    possible: bool,
}

impl Number_Possible {
    pub fn new(value: u32) -> Number_Possible {
        Number_Possible {
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
    possible_numbers: [Number_Possible; 9],
}

impl Possiblities {
    pub fn default() -> Possiblities {
        Possiblities {
            possible_numbers: [
                Number_Possible {
                    number: 1,
                    possible: true,
                },
                Number_Possible {
                    number: 2,
                    possible: true,
                },
                Number_Possible {
                    number: 3,
                    possible: true,
                },
                Number_Possible {
                    number: 4,
                    possible: true,
                },
                Number_Possible {
                    number: 5,
                    possible: true,
                },
                Number_Possible {
                    number: 6,
                    possible: true,
                },
                Number_Possible {
                    number: 7,
                    possible: true,
                },
                Number_Possible {
                    number: 8,
                    possible: true,
                },
                Number_Possible {
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

    pub fn remove(&mut self, number: u32) {
        for possible_number in self.possible_numbers.iter_mut() {
            if possible_number.number == number {
                possible_number.possible = false;
            }
        }
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
