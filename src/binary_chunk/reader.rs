use crate::binary_chunk::header;
use crate::binary_chunk::prototype;
use crate::vm::instruction::Instruction;
use crate::vm::opcodes;

pub struct Reader {
    data: Vec<u8>,
    loc: usize
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Reader {
        Reader {
            data: data,
            loc: 0
        }
    }
    // read basic type
    pub fn read_byte(&mut self) -> u8 {
        let result = self.data[self.loc];
        self.loc += 1;
        result
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.data[self.loc+i]);
        }
        self.loc += size;
        result
    }

    pub fn read_uint32(&mut self) -> u32 {
        let bytes: Vec<u8> = self.read_bytes(4);
        let mut result: u32 = 0;
        for i in 0..4 {
            let tmp: u32 = bytes[i] as u32;
            result |= tmp << (8 * i);
        }
        result
    }

    pub fn read_uint64(&mut self) -> u64 {
        let bytes: Vec<u8> = self.read_bytes(8);
        let mut result: u64 = 0;
        for i in 0..8 {
            let tmp: u64 = bytes[i] as u64;
            result |= tmp << (8 * i);
        }
        result
    }

    pub fn read_lua_integer(&mut self) -> i64 {
        self.read_uint64() as i64
    }

    pub fn read_lua_number(&mut self) -> f64 {
        f64::from_bits(self.read_uint64())
    }

    pub fn read_string(&mut self) -> String {
        let mut size = self.read_byte() as usize;
        if size == 0x00 {
            return String::new();
        }

        if size == 0xFF {
            size = self.read_uint64() as usize;
        }
        let bytes = self.read_bytes(size - 1);
        return String::from_utf8(bytes).unwrap();
    }

    // read prototype
    pub fn read_proto(&mut self, parent_source: String) -> prototype::Prototype {
        let mut source:String = self.read_string();
        if source.is_empty() {
            source = parent_source;
        }
        return prototype::Prototype {
            source: source.clone(),
            line_defined: self.read_uint32(),
            last_line_defined: self.read_uint32(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_stack_size: self.read_byte(),
            code: self.read_code(),
            constants: self.read_constants(),
            up_values: self.read_up_values(),
            protos: self.read_protos(source),
            line_info: self.read_line_info(),
            loc_vars: self.read_loc_vars(),
            up_value_names: self.read_up_value_names()
        };
    }

    pub fn read_code(&mut self) -> Vec<u32> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(self.read_uint32());
        }
        result
    }

    pub fn read_constants(&mut self) -> Vec<prototype::Constant> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(self.read_constant());
        }
        result
    }

    pub fn read_constant(&mut self) -> prototype::Constant {
        let const_type:u8 = self.read_byte();
        match const_type {
            prototype::TAG_NIL => prototype::Constant::Nil,
            prototype::TAG_BOOLEAN => prototype::Constant::Boolean(self.read_byte() != 0),
            prototype::TAG_INTEGER => prototype::Constant::Integer(self.read_lua_integer()),
            prototype::TAG_NUMBER => prototype::Constant::Number(self.read_lua_number()),
            prototype::TAG_SHORT_STR => prototype::Constant::LuaStr(self.read_string()),
            prototype::TAG_LONG_STR => prototype::Constant::LuaStr(self.read_string()),
            _ => panic!("unknown type!")
        }
    }

    pub fn read_up_values(&mut self) -> Vec<prototype::UpValue> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(prototype::UpValue{
                in_stack: self.read_byte(),
                idx: self.read_byte()
            });
        }
        result
    }

    pub fn read_protos(&mut self, parent_source: String) -> Vec<prototype::Prototype> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(self.read_proto(parent_source.clone()));
        }
        result
    }

    pub fn read_line_info(&mut self) -> Vec<u32> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(self.read_uint32());
        }
        result
    }

    pub fn read_loc_vars(&mut self) -> Vec<prototype::LocVar> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(prototype::LocVar {
                var_name: self.read_string(),
                start_pc: self.read_uint32(),
                end_pc: self.read_uint32(),
            });
        }
        result
    }

    pub fn read_up_value_names(&mut self) -> Vec<String> {
        let size = self.read_uint32() as usize;
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(self.read_string());
        }
        result
    }

    // print function
    pub fn print_content(&mut self, cur_proto: &prototype::Prototype) {
        println!("");
        self.print_header(cur_proto);
        self.print_code(cur_proto);
        self.print_detail(cur_proto);
        for it_proto in cur_proto.protos.iter() {
            self.print_content(it_proto);
        }
    }

    pub fn print_header(&mut self, cur_proto: &prototype::Prototype) {
        let mut func_type = "main";
        if cur_proto.line_defined > 0 {
            func_type = "function";
        }

        let mut vararg_flag = "";
        if cur_proto.is_vararg > 0 {
            vararg_flag = "+";
        }

        println!(
            "{} <{}:{},{}> ({} instructions)",
            func_type,
            cur_proto.source,
            cur_proto.line_defined,
            cur_proto.last_line_defined,
            cur_proto.code.len()
        );

        print!(
            "{}{} params, {} slots, {} upvalues, ",
            cur_proto.num_params,
            vararg_flag,
            cur_proto.max_stack_size,
            cur_proto.up_values.len()
        );

        println!(
            "{} locals, {} constants, {} functions",
            cur_proto.loc_vars.len(),
            cur_proto.constants.len(),
            cur_proto.protos.len()
        );
    }

    pub fn print_code(&mut self, cur_proto: &prototype::Prototype) {
        for pc in 0..cur_proto.code.len() {
            let mut line:String = String::from("-");
            if cur_proto.line_info.len() > 0 {
                line = cur_proto.line_info[pc].to_string();
            }
            print!(
                "\t{}\t[{}]\t{}\t",
                pc + 1,
                line,
                cur_proto.code[pc].opname()
            );
            self.print_operands(cur_proto.code[pc]);
            println!("");
        }
    }

    pub fn print_operands(&mut self, instr: u32) {
        match instr.opmode() {
            opcodes::IABC => {
                let (a, b, c) = instr.ABC();
                print!("{}", a);
                if instr.b_mode() != opcodes::OP_ARG_N{
                    if b > 0xFF {
                        print!(" {}", -1 - (b & 0xFF));
                    } else {
                        print!(" {}", b);
                    }
                }
                if instr.c_mode() != opcodes::OP_ARG_N{
                    if c > 0xFF {
                        print!(" {}", -1 - (c & 0xFF));
                    } else {
                        print!(" {}", c);
                    }
                }
            },
            opcodes::IABx => {
                let (a, bx) = instr.ABx();
                print!("{}", a);
                if instr.b_mode() == opcodes::OP_ARG_K {
                    print!(" {}", -1-bx);
                } else if instr.b_mode() == opcodes::OP_ARG_U {
                    print!(" {}", bx);
                }
            },
            opcodes::IAsBx => {
                let (a, sbx) = instr.AsBx();
                print!("{} {}", a, sbx);
            },
            opcodes::IAx => {
                let ax = instr.Ax();
                print!("{}", -1 - ax);
            },
            _ => print!("not exist")
        }
    }

    pub fn print_detail(&mut self, cur_proto: &prototype::Prototype) {
        println!("constants ({}):", cur_proto.constants.len());
        for i in 0..cur_proto.constants.len() {
            print!("\t{}\t", i+1);
            self.print_constant(&cur_proto.constants[i]);
            println!("");
        }

        println!("locals ({}):", cur_proto.loc_vars.len());
        for i in 0..cur_proto.loc_vars.len() {
            println!(
                "\t{}\t{}\t{}\t{}",
                i,
                cur_proto.loc_vars[i].var_name,
                cur_proto.loc_vars[i].start_pc+1,
                cur_proto.loc_vars[i].end_pc+1,
            );
        }

        println!("upvalues ({}):", cur_proto.up_values.len());
        for i in 0..cur_proto.up_values.len() {
            print!("\t{}\t", i);
            self.print_up_value_name(cur_proto, i);
            println!(
                "\t{}\t{}",
                cur_proto.up_values[i].in_stack,
                cur_proto.up_values[i].idx
            );
        }
    }

    pub fn print_up_value_name(&mut self, cur_proto: &prototype::Prototype, idx: usize) {
        if cur_proto.up_value_names.len() > 0 {
            print!("{}", cur_proto.up_value_names[idx]);
        }
    }

    pub fn print_constant(&mut self, cur_const: &prototype::Constant) {
        use crate::binary_chunk::prototype::Constant::*;
        match cur_const {
            Nil => print!("nil"),
            Boolean(a) => print!("{}", a),
            Integer(b) => print!("{}", b),
            Number(c) => print!("{}", c),
            LuaStr(d) => print!("\"{}\"", d),
        }
    }

    // another
    pub fn check_header(&mut self) {
        assert_eq!(
            self.read_bytes(4), header::HEADER.signature,
            "not a pre compiled chunk!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.version,
            "version mismatch!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.format,
            "format mismatch!"
        );
        assert_eq!(
            self.read_bytes(6), header::HEADER.luac_data,
            "corrupted!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.cint_size,
            "int size mismatch!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.sizet_size,
            "size_t size mismatch!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.instruction_size,
            "instruction size mismatch!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.lua_integer_size,
            "lua_integer size mismatch!"
        );
        assert_eq!(
            self.read_byte(), header::HEADER.lua_number_size,
            "lua_number size mismatch!"
        );
        assert_eq!(
            self.read_lua_integer(), header::HEADER.luac_int,
            "endianness mismatch!"
        );
        assert_eq!(
            self.read_lua_number(), header::HEADER.luac_num,
            "float format mismatch!"
        );
    }

}