struct Chip8 {
    a: i32,
    mem: [u8; 4096],
    pc: i32,
    I: i16,
    stack: [i16; 100],
    timer: i8,
    sound: i8,
    registers: [i8; 16]
}

impl Chip8 {
    fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            a: 1,
            mem: [0; 4096],
            pc: 1,
            I: 0,
            stack: [0; 100],
            timer: 0,
            sound: 0,
            registers: [0; 16]
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
}

fn main() {
    let _chip8 = Chip8::new();
}
