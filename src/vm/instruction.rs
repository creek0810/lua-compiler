use crate::vm::opcodes;
use crate::api::api_vm::LuaVM;

use crate::vm::inst_load::*;
use crate::vm::inst_misc::*;
use crate::vm::inst_operators::*;
use crate::vm::inst_table::*;

const MAXARG_Bx: isize = (1 << 18) - 1;
const MAXARG_sBx: isize = MAXARG_Bx >> 1;

pub type Instruction = u32;


pub trait Instruction_impl {
    fn ABC(self) -> (isize, isize, isize);
    fn ABx(self) -> (isize, isize);
    fn AsBx(self) -> (isize, isize);
    fn Ax(self) -> isize;
    fn opcode(self) -> u8;
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
    fn execute(self, vm: &mut LuaVM);
}


impl Instruction_impl for u32 {
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

    fn execute(self, vm: &mut LuaVM) {
        match self.opcode() {
            0 => r#move(self, vm), //MOVE
            1 => loadK(self, vm), //LOADK
            2 => loadKx(self, vm), // LOADKX
            3 => load_bool(self, vm),// LOADBOOL
            4 => load_nil(self, vm), // LOADNIL
            // 5    GETUPVAL")
            // 6    GETTABUP")
            7 => get_table(self, vm), // GETTABLE
            // 8    SETTABUP")
            // 9    SETUPVAL")
            10 => set_table(self, vm), // SETTABLE
            11 => new_table(self, vm), // NEWTABLE
            // 12    SELF    ")
            13 => add(self, vm), // ADD
            14 => sub(self, vm), // SUB
            15 => mul(self, vm), // MUL
            16 => r#mod(self, vm), // MOD
            17 => pow(self, vm), // POW
            18 => div(self, vm), // DIV
            19 => idiv(self, vm), // IDIV
            20 => band(self, vm), // BAND
            21 => bor(self, vm), // BOR
            22 => bxor(self, vm), // BXOR
            23 => shl(self, vm), // SHL
            24 => shr(self, vm), // SHR
            25 => unm(self, vm), // UNM
            26 => bnot(self, vm), // BNOT
            27 => not(self, vm), // NOT
            28 => len(self, vm), // LEN
            29 => concat(self, vm), // CONCAT
            30 => jmp(self, vm), // JMP
            31 => eq(self, vm), // EQ
            32 => lt(self, vm), // LT
            33 => le(self, vm), // LE
            34 => test(self, vm), // TEST
            35 => test_set(self, vm), // TESTSET
            /*
            36    CALL    ")
            37    TAILCALL")
            38    RETURN  ")
            */
            39 => for_loop(self, vm), // FORLOOP
            40 => for_rep(self, vm), // FORREP
            // 41    TFORCALL")
            // 42    "TFORLOOP
            43 => set_list(self, vm), // SETLIST
            // 44    CLOSURE  "
            // 45    VARARG   "
            // 46    XTRAATG ")
            _ => panic!("{} todo!", self)

        }
    }
}