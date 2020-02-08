use crate::vm::instruction::{Instruction, Instruction_impl};
use crate::api::api_stack::LuaAPI;
use crate::api::api_vm::*;

pub fn load_nil(i: Instruction, vm: &mut LuaVM) {
    let (mut a, b, _) = i.ABC();
    vm.push_nil();
    a += 1;
    for i in a..(a + b + 1) {
        vm.copy(-1, i);
    }
    vm.pop(1);
}

pub fn load_bool(i: Instruction, vm: &mut LuaVM) {
    let (mut a, b, c) = i.ABC();
    a += 1;
    vm.push_boolean(b != 0);
    vm.replace(a);
    if c != 0 {
        vm.add_pc(1);
    }
}

pub fn loadK(i: Instruction, vm: &mut LuaVM) {
    let (mut a, bx) = i.ABx();
    a += 1;
    vm.get_const(bx);
    vm.replace(a);
}

pub fn loadKx(i: Instruction, vm: &mut LuaVM) {
    let (mut a, _) = i.ABx();
    a += 1;
    let ax = vm.fetch().Ax();
    vm.get_const(ax);
    vm.replace(a);
}