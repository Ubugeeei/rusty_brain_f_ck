struct Instructions;
impl Instructions {
    pub const INC_PTR: char = '>';
    pub const DEC_PTR: char = '<';
    pub const INC_VAL: char = '+';
    pub const DEC_DEC: char = '-';
    pub const PUT: char = '.';
    pub const GET: char = ',';
    pub const JZ: char = '[';
    pub const JNZ: char = ']';
}

struct Processor {
    input: String,
    position: isize,
    memory: [u8; 30000],
}

impl Processor {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            memory: [0; 30000],
        }
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.input.len() {
            match self.input.chars().nth(i).unwrap() {
                Instructions::INC_PTR => self.position += 1,
                Instructions::DEC_PTR => self.position -= 1,
                Instructions::INC_VAL => self.memory[self.position as usize] += 1,
                Instructions::DEC_DEC => self.memory[self.position as usize] -= 1,
                Instructions::PUT => print!("{}", self.memory[self.position as usize] as char),
                Instructions::GET => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    self.memory[self.position as usize] = input.chars().nth(0).unwrap() as u8;
                }
                Instructions::JZ => {
                    if self.memory[self.position as usize] == 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            i += 1;
                            match self.input.chars().nth(i).unwrap() {
                                Instructions::JZ => depth += 1,
                                Instructions::JNZ => depth -= 1,
                                _ => (),
                            }
                        }
                    }
                }
                Instructions::JNZ => {
                    if self.memory[self.position as usize] != 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            i -= 1;
                            match self.input.chars().nth(i).unwrap() {
                                Instructions::JZ => depth -= 1,
                                Instructions::JNZ => depth += 1,
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
            i += 1;
        }
    }
}

fn main() {
    let mut processor = Processor::new(String::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."));
    processor.run();
}
