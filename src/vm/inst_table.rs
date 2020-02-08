use crate::vm::instruction::{Instruction, Instruction_impl};
use crate::api::api_stack::LuaAPI;
use crate::api::api_vm::*;
use crate::vm::fpb::*;
const LFIELDS_PER_FLUSH: isize = 50;

pub fn new_table(i: Instruction, vm: &mut LuaVM) {
    let (mut a, b, c) = i.ABC();
    a += 1;
    vm.create_table(fb2int(b as usize), fb2int(c as usize));
    vm.replace(a);
}

pub fn get_table(i: Instruction, vm: &mut LuaVM) {
    let (mut a, mut b, c) = i.ABC();
    a += 1;
    b += 1;
    vm.get_rk(c);
    vm.get_table(b);
    vm.replace(a);
}

pub fn set_table(i: Instruction, vm: &mut LuaVM) {
    let (mut a, b, c) = i.ABC();
    a += 1;
    vm.get_rk(b);
    vm.get_rk(c);
    vm.set_table(a);
}

pub fn set_list(i: Instruction, vm: &mut LuaVM) {
    let (mut a, b, mut c) = i.ABC();
    a += 1;
    if c > 0 {
        c -= 1;
    } else {
        c = vm.fetch().Ax();
    }
    let mut idx = c * LFIELDS_PER_FLUSH;
    for j in 1..(b+1) {
        idx += 1;
        vm.push_value(a + j);
        vm.set_i(a, idx as i64);
    }
}