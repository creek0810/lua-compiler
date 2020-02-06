mod binary_chunk;
mod vm;
mod state;
mod api;
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
    let mut my_state = LuaState::new();


    my_state.push_boolean(true);
    print_stack(&my_state);

    my_state.push_integer(10);
    print_stack(&my_state);

    my_state.push_nil();
    print_stack(&my_state);

    my_state.push_string(String::from("hello"));
    print_stack(&my_state);

    my_state.push_value(-4);
    print_stack(&my_state);

    my_state.replace(3);
    print_stack(&my_state);

    my_state.set_top(6);
    print_stack(&my_state);

    my_state.remove(-3);
    print_stack(&my_state);

    my_state.set_top(-5);
    print_stack(&my_state);

    Ok(())
}
