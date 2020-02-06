use crate::vm::opcodes;
const MAXARG_Bx: isize = (1 << 18) - 1;
const MAXARG_sBx: isize = MAXARG_Bx >> 1;

pub trait Instruction {
    fn ABC(self) -> (isize, isize, isize);
    fn ABx(self) -> (isize, isize);
    fn AsBx(self) -> (isize, isize);
    fn Ax(self) -> isize;
    fn opcode(self) -> u8;
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
}


impl Instruction for u32 {
    fn ABC(self) -> (isize, isize, isize) {
        // 000000000 100000000 00000000 000110
        //     b         c        a       op
        let a: isize = (self >> 6 & 0xFF) as isize;
        let c: isize = (self >> 14 & 0x1FF) as isize;
        let b: isize = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    fn ABx(self) -> (isize, isize) {
        let a: isize = (self >> 6 & 0xFF) as isize;
        let bx: isize = (self >> 14) as isize;
        (a, bx)
    }

    fn AsBx(self) -> (isize, isize) {
        let (a, bx) = self.ABx();
        (a, bx - MAXARG_sBx)
    }

    fn Ax(self) -> isize {
        (self >> 6) as isize
    }

    fn opcode(self) -> u8 {
        self as u8 & 0x3F
    }

    fn opname(self) -> &'static str {
        opcodes::OPCODES[self.opcode() as usize].name
    }

    fn opmode(self) -> u8 {
        opcodes::OPCODES[self.opcode() as usize].op_mode
    }

    fn b_mode(self) -> u8 {
        opcodes::OPCODES[self.opcode() as usize].arg_b_mode
    }

    fn c_mode(self) -> u8 {
        opcodes::OPCODES[self.opcode() as usize].arg_c_mode
    }
}