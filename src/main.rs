use std::fs::File;
use std::io::{self, Read};
use std::{fmt::Debug, thread, time::Duration};

struct Display {
    pixels: [u8; 2048], // 64*32
}

impl Display {
    fn new() -> Display {
        Display { pixels: [0; 2048] }
    }

    fn xy_to_index(x: u16, y: u16) -> u16 {
        return y * 32 + x;
    }

    fn write(mut self, x: u16, y: u16, val: u8) {
        let index = Self::xy_to_index(x, y);
        self.pixels[usize::from(index)] = val;
    }
    fn read(self, x: u16, y: u16) -> u8 {
        let index = Self::xy_to_index(x, y);
        return self.pixels[usize::from(index)];
    }
}

struct NoOp {}

trait Instruction {
    fn execute(&self);
}

impl NoOp {
    fn new() -> NoOp {
        NoOp {}
    }
}

impl Instruction for NoOp {
    fn execute(&self) {
        println!("No-op instruction has been implemented");
    }
}

enum Instructions {
    NoOp(NoOp),
}

impl Instructions {
    fn new(instruction: u16) -> Instructions {
        match instruction {
            1 => Instructions::NoOp(NoOp::new()),
            0x00E0 => {
                println!("Fant 0x00E0: Clearer skjermen");
                Instructions::NoOp(NoOp::new())
            }
            v if (v & 0xF000) == 0x1000 => {
                println!("Fant 0x1NNN: Jump");
                Instructions::NoOp(NoOp::new())
            }
            v if (v & 0xF000) == 0x6000 => {
                println!("Fant 0x6XNN: Set register VX to NN");
                Instructions::NoOp(NoOp::new())
            }
            v if (v & 0xF000) == 0x7000 => {
                println!("Fant 0x7XNN: Add value NN to register VX");
                Instructions::NoOp(NoOp::new())
            }
            v if (v & 0xF000) == 0xA000 => {
                println!("Fant 0xANNN: Set index register I");
                Instructions::NoOp(NoOp::new())
            }
            v if (v & 0xF000) == 0xD000 => {
                println!("Fant 0xDXYN: Display");
                Instructions::NoOp(NoOp::new())
            }
            _ => {
                println!("==========================");
                println!("Unimplemented instruction! {}", instruction);
                println!("==========================");
                Instructions::NoOp(NoOp::new())
            },
        }
    }
}

impl Instruction for Instructions {
    fn execute(&self) {
        match self {
            Instructions::NoOp(instruction) => instruction.execute(),
        }
    }
}

impl Debug for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::NoOp(_) => write!(f, "Instruction: No-op"),
        }
    }
}

struct Chip8 {
    a: i32,
    mem: [u8; 4096],
    pc: i32,
    I: u16,
    stack: Vec<u16>,
    delay_timer: i8,
    sound_timer: i8,
    registers: [i8; 16],
}

impl Chip8 {
    fn new(memory: Option<[u8; 4096]>) -> Chip8 {
        let mut chip8 = Chip8 {
            a: 1,
            mem: memory.unwrap_or([0; 4096]),
            pc: 1,
            I: 0x200,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
        };

        // Font
        //0
        chip8.mem[0x050] = 0xF0;
        chip8.mem[0x051] = 0x90;
        chip8.mem[0x052] = 0x90;
        chip8.mem[0x053] = 0x90;
        chip8.mem[0x054] = 0xF0;
        //1
        chip8.mem[0x055] = 0x20;
        chip8.mem[0x056] = 0x60;
        chip8.mem[0x057] = 0x20;
        chip8.mem[0x058] = 0x20;
        chip8.mem[0x059] = 0x70;
        //2
        chip8.mem[0x05A] = 0xF0;
        chip8.mem[0x05B] = 0x10;
        chip8.mem[0x05C] = 0xF0;
        chip8.mem[0x05D] = 0x80;
        chip8.mem[0x05E] = 0xF0;
        //3
        chip8.mem[0x05F] = 0xF0;
        chip8.mem[0x060] = 0x10;
        chip8.mem[0x061] = 0xF0;
        chip8.mem[0x062] = 0x10;
        chip8.mem[0x063] = 0xF0;
        //4
        chip8.mem[0x064] = 0x90;
        chip8.mem[0x065] = 0x90;
        chip8.mem[0x066] = 0xF0;
        chip8.mem[0x067] = 0x10;
        chip8.mem[0x068] = 0x10;
        //5
        chip8.mem[0x069] = 0xF0;
        chip8.mem[0x06A] = 0x80;
        chip8.mem[0x06B] = 0xF0;
        chip8.mem[0x06C] = 0x10;
        chip8.mem[0x06D] = 0xF0;
        //6
        chip8.mem[0x06E] = 0xF0;
        chip8.mem[0x06F] = 0x80;
        chip8.mem[0x070] = 0xF0;
        chip8.mem[0x071] = 0x90;
        chip8.mem[0x072] = 0xF0;
        //7
        chip8.mem[0x073] = 0xF0;
        chip8.mem[0x074] = 0x10;
        chip8.mem[0x075] = 0x20;
        chip8.mem[0x076] = 0x40;
        chip8.mem[0x077] = 0x40;
        //8
        chip8.mem[0x078] = 0xF0;
        chip8.mem[0x079] = 0x90;
        chip8.mem[0x07A] = 0xF0;
        chip8.mem[0x07B] = 0x90;
        chip8.mem[0x07C] = 0xF0;
        //9
        chip8.mem[0x07D] = 0xF0;
        chip8.mem[0x07E] = 0x90;
        chip8.mem[0x07F] = 0xF0;
        chip8.mem[0x080] = 0x10;
        chip8.mem[0x081] = 0xF0;
        //A
        chip8.mem[0x082] = 0xF0;
        chip8.mem[0x083] = 0x90;
        chip8.mem[0x084] = 0xF0;
        chip8.mem[0x085] = 0x90;
        chip8.mem[0x086] = 0x90;
        //B
        chip8.mem[0x087] = 0xE0;
        chip8.mem[0x088] = 0x90;
        chip8.mem[0x089] = 0xE0;
        chip8.mem[0x08A] = 0x90;
        chip8.mem[0x08B] = 0xE0;
        //C
        chip8.mem[0x08C] = 0xF0;
        chip8.mem[0x08D] = 0x80;
        chip8.mem[0x08E] = 0x80;
        chip8.mem[0x08F] = 0x80;
        chip8.mem[0x090] = 0xF0;
        //D
        chip8.mem[0x091] = 0xE0;
        chip8.mem[0x092] = 0x90;
        chip8.mem[0x093] = 0x90;
        chip8.mem[0x094] = 0x90;
        chip8.mem[0x095] = 0xE0;
        //E
        chip8.mem[0x096] = 0xF0;
        chip8.mem[0x097] = 0x80;
        chip8.mem[0x098] = 0xF0;
        chip8.mem[0x099] = 0x80;
        chip8.mem[0x09A] = 0xF0;
        //F
        chip8.mem[0x09B] = 0xF0;
        chip8.mem[0x09C] = 0x80;
        chip8.mem[0x09D] = 0xF0;
        chip8.mem[0x09E] = 0x80;
        chip8.mem[0x09F] = 0x80;

        return chip8;
    }

    fn fetch(&mut self) -> Instructions {
        let index_1 = self.I;
        let instruction_1 = self.get_instruction(index_1);
        self.I += 1;
        let index_2 = self.I;
        let instruction_2 = self.get_instruction(index_2);
        self.I += 1;
        let combined: u16 = (instruction_1 as u16) << 8 | instruction_2 as u16;
        println!(
            "Instruction as decimal: {}. Instruction as hex: {:02X}",
            combined, combined
        );
        Instructions::new(combined)
    }

    fn get_instruction(&self, I: u16) -> u8 {
        self.mem[usize::from(I)]
    }

    fn execute(self, instruction: Instructions) {
        instruction.execute()
    }

    fn stack_pop(mut self) -> Option<u16> {
        self.stack.pop()
    }
    fn stack_push(mut self, val: u16) {
        self.stack.push(val)
    }

    fn progress_time(mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn should_beep(self) -> bool {
        self.sound_timer > 0
    }
}

fn read_rom(path: &str) -> io::Result<[u8; 4096]> {
    let mut memory: [u8; 4096] = [0; 4096];

    let mut file_buffer: Vec<u8> = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut file_buffer);

    for (i, &byte) in file_buffer.iter().enumerate() {
        memory[i + 0x200] = byte;
    }

    return Ok(memory);
}

fn main() {
    let rom_path = "./IBM Logo.ch8";
    let memory = read_rom(rom_path).unwrap();
    println!("{:?}", memory);

    let mut chip8 = Chip8::new(Some(memory));
    let _display = Display::new();

    loop {
        println!("The loop iteration has started");

        let _ = chip8.fetch();
        println!("--------------------------------");
        thread::sleep(Duration::from_secs(1));
    }
}
