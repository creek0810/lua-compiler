use crate::state::lua_state::LuaState;
use crate::state::lua_value::LuaValue;
use crate::api::api_stack::LuaAPI;
use crate::binary_chunk::prototype::Constant;

pub type LuaVM = LuaState;

pub trait VmAPI {
    fn pc(&self) -> isize;
    fn add_pc(&mut self, n: isize);
    fn fetch(&mut self) -> u32;
    fn get_const(&mut self, idx: isize);
    fn get_rk(&mut self, rk: isize);
}

impl VmAPI for LuaState {
    fn pc(&self) -> isize {
        self.pc
    }

    fn add_pc(&mut self, n: isize) {
        self.pc += n;
    }

    fn fetch(&mut self) -> u32 {
        let result = self.proto.code[self.pc as usize];
        self.pc += 1;
        result
    }

    fn get_const(&mut self, idx: isize) {
        let tmp = &self.proto.constants[idx as usize];
        let val = match tmp {
            Constant::Nil => LuaValue::Nil,
            Constant::Boolean(b) => LuaValue::Bool(*b),
            Constant::Integer(i) => LuaValue::Int64(*i),
            Constant::Number(n) => LuaValue::Float64(*n),
            Constant::LuaStr(s) => LuaValue::LuaString((*s).clone()),
        };
        self.stack.push(val);
    }

    fn get_rk(&mut self, rk: isize) {
        if rk > 0xFF {
            self.get_const(rk & 0xFF);
        } else {
            self.push_value(rk + 1);
        }

    }
}



