mod binary_chunk;
mod vm;
mod state;
mod api;
use crate::api::api_stack::LuaAPI;
use crate::api::consts::*;
use crate::state::lua_state::LuaState;
use crate::binary_chunk::prototype::Prototype;
use crate::vm::instruction::*;
use crate::api::api_vm::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn lua_main(proto: Prototype) {
    let n_regs = proto.max_stack_size as usize;
    let mut ls = LuaState::new(n_regs, proto);
    ls.set_top(n_regs as isize);
    // while
    loop {
        let pc = ls.pc();
        let inst = ls.fetch();
        // 38 is return
        if inst.opcode() != 38 {
            inst.execute(&mut ls);
            print!("[{:02}] {}", pc + 1, inst.opname());
            print_stack(&ls);
            println!("");
        } else {
            break;
        }
    }

}

fn undump(data: Vec<u8>) {
    let mut r = binary_chunk::reader::Reader::new(data);
    r.check_header();
    r.read_byte(); // size_upvalues
    let result = r.read_proto(String::from(""));
    // print
    r.print_content(&result);
}

fn print_stack(cur_state: &LuaState) {
    let top = cur_state.get_top() + 1;
    for i in 1..top {
        let type_id = cur_state.type_id(i);
        match type_id {
            LUA_TBOOLEAN => print!("[{}]", cur_state.to_boolean(i)),
            LUA_TNUMBER => print!("[{}]", cur_state.to_number(i)),
            LUA_TSTRING=> print!("[\"{}\"]", cur_state.to_string(i)),
            _ => print!("[{}]", cur_state.type_name(type_id))
        }
    }
    println!("");
}

fn main() -> io::Result<()> {
    let mut file = File::open("./tests/test.out")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    // undump
    let mut r = binary_chunk::reader::Reader::new(data);
    r.check_header();
    r.read_byte(); // size_upvalues
    let result = r.read_proto(String::from(""));
    // r.print_content(&result);
    lua_main(result);

    Ok(())
}
