/* mode */
pub const IABC: u8 = 0x00;
pub const IABx: u8 = 0x01;
pub const IAsBx: u8 = 0x02;
pub const IAx: u8 = 0x03;

/* op arg */
pub const OP_ARG_N: u8 = 0x00;
pub const OP_ARG_U: u8 = 0x01;
pub const OP_ARG_R: u8 = 0x02;
pub const OP_ARG_K: u8 = 0x03;

pub struct Opcode {
    pub test_flag: u8,
    pub set_a_flag: u8,
    pub arg_b_mode: u8,
    pub arg_c_mode: u8,
    pub op_mode: u8,
    pub name: &'static str
}

const fn opcode(test_flag: u8, set_a_flag: u8, arg_b_mode: u8, arg_c_mode: u8, op_mode: u8, name: &'static str) -> Opcode {
    Opcode {
        test_flag,
        set_a_flag,
        arg_b_mode,
        arg_c_mode,
        op_mode,
        name
    }
}

pub const OPCODES: [Opcode; 47] = [
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IABC, "MOVE    "),
    opcode(0, 1, OP_ARG_K, OP_ARG_N, IABx, "LOADK   "),
    opcode(0, 1, OP_ARG_N, OP_ARG_N, IABx, "LOADKX  "),
    opcode(0, 1, OP_ARG_U, OP_ARG_U, IABC, "LOADBOOL"),
    opcode(0, 1, OP_ARG_U, OP_ARG_N, IABC, "LOADNIL "),
    opcode(0, 1, OP_ARG_U, OP_ARG_N, IABC, "GETUPVAL"),
    opcode(0, 1, OP_ARG_U, OP_ARG_K, IABC, "GETTABUP"),
    opcode(0, 1, OP_ARG_R, OP_ARG_K, IABC, "GETTABLE"),
    opcode(0, 0, OP_ARG_K, OP_ARG_K, IABC, "SETTABUP"),
    opcode(0, 0, OP_ARG_U, OP_ARG_N, IABC, "SETUPVAL"),
    opcode(0, 0, OP_ARG_K, OP_ARG_K, IABC, "SETTABLE"),
    opcode(0, 1, OP_ARG_U, OP_ARG_U, IABC, "NEWTABLE"),
    opcode(0, 1, OP_ARG_R, OP_ARG_K, IABC, "SELF    "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "ADD     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "SUB     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "MUL     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "MOD     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "POW     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "DIV     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "IDIV    "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "BAND    "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "BOR     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "BXOR    "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "SHL     "),
    opcode(0, 1, OP_ARG_K, OP_ARG_K, IABC, "SHR     "),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IABC, "UNM     "),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IABC, "BNOT    "),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IABC, "NOT     "),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IABC, "LEN     "),
    opcode(0, 1, OP_ARG_R, OP_ARG_R, IABC, "CONCAT  "),
    opcode(0, 0, OP_ARG_R, OP_ARG_N, IAsBx, "JMP     "),
    opcode(1, 0, OP_ARG_K, OP_ARG_K, IABC, "EQ      "),
    opcode(1, 0, OP_ARG_K, OP_ARG_K, IABC, "LT      "),
    opcode(1, 0, OP_ARG_K, OP_ARG_K, IABC, "LE      "),
    opcode(1, 0, OP_ARG_N, OP_ARG_U, IABC, "TEST    "),
    opcode(1, 1, OP_ARG_R, OP_ARG_U, IABC, "TESTSET "),
    opcode(0, 1, OP_ARG_U, OP_ARG_U, IABC, "CALL    "),
    opcode(0, 1, OP_ARG_U, OP_ARG_U, IABC, "TAILCALL"),
    opcode(0, 0, OP_ARG_U, OP_ARG_N, IABC, "RETURN  "),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IAsBx, "FORLOOP "),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IAsBx, "FORREP  "),
    opcode(0, 0, OP_ARG_N, OP_ARG_U, IABC, "TFORCALL"),
    opcode(0, 1, OP_ARG_R, OP_ARG_N, IAsBx, "TFORLOOP "),
    opcode(0, 0, OP_ARG_U, OP_ARG_U, IABC, "SETLIST "),
    opcode(0, 1, OP_ARG_U, OP_ARG_N, IABx, "CLOSURE  "),
    opcode(0, 1, OP_ARG_U, OP_ARG_N, IABC, "VARARG   "),
    opcode(0, 0, OP_ARG_U, OP_ARG_U, IAx, "EXTRAATG ")
];