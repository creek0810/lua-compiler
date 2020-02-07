use crate::state::lua_stack::LuaStack;
use crate::state::lua_value::LuaValue;
use crate::api::consts::*;
use crate::api::api_arith::OPS;
use crate::api::api_arith;
use crate::api::api_cmp;
use crate::api::api_stack::LuaAPI;



pub struct LuaState {
    pub stack: LuaStack
}

impl LuaState {
    pub fn new() -> LuaState {
        LuaState {
            stack: LuaStack::new(20)
        }
    }

    pub fn arith(&mut self, op: u8) {
        let mut a: LuaValue;
        let b = self.stack.pop();
        if op != LUA_OPUNM && op != LUA_OPBNOT {
            a = self.stack.pop();
        } else {
            a = b.clone();
        }
        let result = api_arith::_arith(&a, &b, op);
        self.stack.push(result);
    }

    pub fn compare(&mut self, idx1: isize, idx2: isize, op: u8) -> bool {
        let a = self.stack.get(idx1);
        let b = self.stack.get(idx2);
        match op {
            LUA_OPEQ => api_cmp::_eq(&a, &b),
            LUA_OPLT => {
                if let Some(result) = api_cmp::_lt(&a, &b) {
                    return result;
                }
                panic!("invalid cmp op!");
            },
            LUA_OPLE => {
                if let Some(result) = api_cmp::_le(&a, &b) {
                    return result;
                }
                panic!("invalid cmp op!");

            },
            _ => panic!("invalid cmp op!")
        }
    }

    pub fn len(&mut self, idx: isize) {
        let tmp = self.stack.get(idx);
        if let LuaValue::LuaString(s) = tmp {
            self.stack.push(LuaValue::Int64(s.len() as i64));
        } else {
            panic!("length error!");
        }
    }

    pub fn concat(&mut self, n: isize) {
        if n == 0 {
            self.stack.push(LuaValue::LuaString(String::new()));
        } else {
            for i in 1..n {
                if self.is_string(-1) && self.is_string(-2) {
                    let s2 = self.to_string(-1);
                    let mut s1 = self.to_string(-2);
                    self.stack.pop();
                    self.stack.pop();
                    s1.push_str(s2.as_str());
                    self.stack.push(LuaValue::LuaString(s1.clone()));
                    continue;
                }
                panic!("concatenation error!");
            }

        }
    }
}