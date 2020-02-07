mod binary_chunk;
mod vm;
mod state;
mod api;
mod number;
use crate::api::api_stack::LuaAPI;
use crate::api::consts::*;
use crate::state::lua_state::LuaState;
use std::fs::File;
use std::io;
use std::io::prelude::*;


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
    let mut ls = LuaState::new();
    // init value
    ls.push_integer(1);
    ls.push_string(String::from("2.0"));
    ls.push_string(String::from("3.0"));
    ls.push_number(4.0);
    print_stack(&ls);

    // start arith
    ls.arith(LUA_OPADD);
    print_stack(&ls);

    ls.arith(LUA_OPBNOT);
    print_stack(&ls);

    ls.len(2);
    print_stack(&ls);

    ls.concat(3);
    print_stack(&ls);

    let result = ls.compare(1, 2, LUA_OPEQ);
    ls.push_boolean(result);
    print_stack(&ls);

    Ok(())
}
