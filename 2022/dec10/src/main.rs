use std::io::stdin;

struct CPU {
    pc: i64,
    x: i64,
}

impl CPU {
    fn new() -> CPU {
        CPU { pc: 0, x: 1 }
    }

    fn cycle(&mut self) {
        self.pc += 1;

        let pc_pos = (self.pc - 1) % 40;
        let sprite = self.x % 40;
        if (sprite - 1..=sprite + 1).contains(&pc_pos) {
            print!("#")
        } else {
            print!(".")
        }

        if self.pc % 40 == 0 {
            println!()
        }
    }

    fn noop(&mut self) {
        self.cycle();
    }

    fn addx(&mut self, delta: i64) {
        self.cycle();
        self.cycle();
        self.x += delta;
    }
}

fn main() {
    let mut cpu = CPU::new();

    for line in stdin().lines().flatten() {
        if line.starts_with("noop") {
            cpu.noop();
        } else if line.starts_with("addx") {
            let (_, nstr) = line.split_once(' ').unwrap();
            let n: i64 = nstr.parse().unwrap();
            cpu.addx(n);
        }
    }
}
