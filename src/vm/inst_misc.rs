use crate::vm::instruction::{Instruction, Instruction_impl};
use crate::api::api_stack::LuaAPI;
use crate::api::api_vm::*;

pub fn r#move(i: Instruction, vm: &mut LuaVM) {
    let (a, b, _) = i.ABC();
    vm.copy(b + 1, a + 1);
}

pub fn jmp(i: Instruction, vm: &mut LuaVM) {
    let (a, sBx) = i.AsBx();
    vm.add_pc(sBx);
    if a != 0 {
        panic!("todo");
    }
}