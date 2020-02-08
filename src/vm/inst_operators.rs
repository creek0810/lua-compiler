use crate::vm::instruction::{Instruction, Instruction_impl};
use crate::api::api_stack::LuaAPI;
use crate::api::api_vm::*;
use crate::api::consts::*;

fn _binary_arith(i: Instruction, vm: &mut LuaVM, op: u8) {
    let (mut a, b, c) = i.ABC();
    a += 1;
    vm.get_rk(b);
    vm.get_rk(c);
    vm.arith(op);
    vm.replace(a);
}

fn _unary_arith(i: Instruction, vm: &mut LuaVM, op: u8) {
    let (mut a, mut b, _) = i.ABC();
    a += 1;
    b += 1;
    vm.push_value(b);
    vm.arith(op);
    vm.replace(a);
}

pub fn len(i: Instruction, vm: &mut LuaVM) {
    let (mut a, mut b, _) = i.ABC();
    a += 1;
    b += 1;
    vm.len(b);
    vm.replace(a);
}

fn _compare(i: Instruction, vm: &mut LuaVM, op: u8) {
    let (a, b, c) = i.ABC();
    vm.get_rk(b);
    vm.get_rk(c);
    if vm.compare(-2, -1, op) != (a != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
}

pub fn concat(i: Instruction, vm: &mut LuaVM) {
    let (mut a, mut b, mut c) = i.ABC();
    a += 1;
    b += 1;
    c += 1;
    let size = c - b + 1;
    vm.check_stack(size as usize);
    for i in b..(c+1) {
        vm.push_value(i);
    }
    vm.concat(size);
    vm.replace(a);
}

pub fn add(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPADD);
}

pub fn sub(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPSUB);
}

pub fn mul(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPMUL);
}

pub fn r#mod(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPMOD);
}

pub fn pow(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPPOW);
}

pub fn div(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPDIV);
}

pub fn idiv(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPIDIV);
}

pub fn band(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPBAND);
}

pub fn bor(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPBOR);
}

pub fn bxor(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPBXOR);
}

pub fn shl(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPSHL);
}

pub fn shr(i: Instruction, vm: &mut LuaVM) {
    _binary_arith(i, vm, LUA_OPSHR);
}

pub fn unm(i: Instruction, vm: &mut LuaVM) {
    _unary_arith(i, vm, LUA_OPUNM);
}

pub fn bnot(i: Instruction, vm: &mut LuaVM) {
    _unary_arith(i, vm, LUA_OPBNOT);
}

pub fn eq(i: Instruction, vm: &mut LuaVM) {
    _compare(i, vm, LUA_OPEQ);
}

pub fn lt(i: Instruction, vm: &mut LuaVM) {
    _compare(i, vm, LUA_OPLT);
}

pub fn le(i: Instruction, vm: &mut LuaVM) {
    _compare(i, vm, LUA_OPLE);
}

pub fn not(i: Instruction, vm: &mut LuaVM) {
    let (mut a, mut b, _) = i.ABC();
    a += 1;
    b += 1;
    vm.push_boolean(!vm.to_boolean(b));
    vm.replace(a);
}

pub fn test(i: Instruction, vm: &mut LuaVM) {
    let (mut a, _, c) = i.ABC();
    a += 1;
    if vm.to_boolean(a) != (c != 0) {
        vm.add_pc(1);
    }
}

pub fn test_set(i: Instruction, vm: &mut LuaVM) {
    let (mut a, mut b, c) = i.ABC();
    a += 1;
    b += 1;
    if vm.to_boolean(b) == (c != 0) {
        vm.copy(b, a);
    } else {
        vm.add_pc(1);
    }
}

pub fn for_rep(i: Instruction, vm: &mut LuaVM) {
    let (mut a, sBx) = i.AsBx();
    a += 1;
    vm.push_value(a);
    vm.push_value(a + 2);
    vm.arith(LUA_OPSUB);
    vm.replace(a);
    vm.add_pc(sBx);
}

pub fn for_loop(i: Instruction, vm: &mut LuaVM) {
    let (mut a, sBx) = i.AsBx();
    a += 1;
    vm.push_value(a + 2);
    vm.push_value(a);
    vm.arith(LUA_OPADD);
    vm.replace(a);

    let is_positive_step:bool = (vm.to_number(a + 2) >= 0.0);
    if is_positive_step && vm.compare(a, a+1, LUA_OPLE) ||
       !is_positive_step && vm.compare(a+1, a, LUA_OPLE) {
        vm.add_pc(sBx);
        vm.copy(a, a+3);
    }
}