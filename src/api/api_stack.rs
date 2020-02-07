use crate::state::lua_state::LuaState;
use crate::state::lua_value::LuaValue;
use crate::api::consts::*;

pub trait LuaAPI {
    // basic operation
    fn get_top(&self) -> isize;
    fn abs_index(&self, idx: isize) -> isize;
    fn check_stack(&mut self, size: usize) -> bool;
    fn pop(&mut self, size: usize);
    fn copy(&mut self, from: isize, to: isize);
    fn push_value(&mut self, idx: isize);
    fn replace(&mut self, idx: isize);
    fn insert(&mut self, idx: isize);
    fn remove(&mut self, idx: isize);
    fn rotate(&mut self, idx: isize, n: isize);
    fn set_top(&mut self, idx: isize);
    fn push_nil(&mut self);
    // push basic type
    fn push_boolean(&mut self, b: bool);
    fn push_integer(&mut self, n: i64);
    fn push_number(&mut self, n: f64);
    fn push_string(&mut self, s: String);
    // access
    fn type_name(&self, cur_type: i8) -> &str;
    fn type_id(&self, idx: isize) -> i8;
    fn is_none(&self, idx: isize) -> bool;
    fn is_nil(&self, idx: isize) -> bool;
    fn is_none_or_nil(&self, idx: isize) -> bool;
    fn is_bool(&self, idx: isize) -> bool;
    fn is_string(&self, idx: isize) -> bool;
    fn is_number(&self, idx: isize) -> bool;
    fn is_integer(&self, idx: isize) -> bool;
    fn to_boolean(&self, idx: isize) -> bool;
    fn to_number(&self, idx: isize) -> f64;
    fn to_numberx(&self, idx: isize) -> (f64, bool);
    fn to_integer(&self, idx: isize) -> i64;
    fn to_integerx(&self, idx: isize) -> (i64, bool);
    fn to_string(&self, idx: isize) -> String;
    fn to_stringx(&self, idx: isize) -> (String, bool);
}

impl LuaAPI for LuaState {
    fn get_top(&self) -> isize {
        self.stack.top()
    }

    fn abs_index(&self, idx: isize) -> isize {
        self.stack.abs_index(idx)
    }

    fn check_stack(&mut self, size: usize) -> bool {
        self.stack.check(size);
        true
    }

    fn pop(&mut self, size: usize) {
        for _ in 0..size {
            self.stack.pop();
        }
    }

    fn copy(&mut self, from: isize, to: isize) {
        let tmp = self.stack.get(from);
        self.stack.set(to, tmp);
    }

    fn push_value(&mut self, idx: isize) {
        let tmp = self.stack.get(idx);
        self.stack.push(tmp);
    }

    fn replace(&mut self, idx: isize) {
        let tmp = self.stack.pop();
        self.stack.set(idx, tmp);
    }

    fn insert(&mut self, idx: isize) {
        self.rotate(idx, 1);
    }

    fn remove(&mut self, idx: isize) {
        self.rotate(idx, -1);
        self.pop(1);
    }

    fn rotate(&mut self, idx: isize, n: isize) {
        let t = self.stack.top() - 1;
        let p = self.stack.abs_index(idx) - 1;
        let m = if n >= 0 {t - n} else {p - n - 1};
        self.stack.reverse(p as usize, m as usize);
        self.stack.reverse((m+1) as usize, t as usize);
        self.stack.reverse(p as usize, t as usize);
    }

    fn set_top(&mut self, idx: isize) {
        let new_top = self.stack.abs_index(idx);
        if new_top < 0 {
            panic!("stack underflow!");
        }
        let n = self.stack.top() - new_top;
        if n > 0 {
            for _ in 0..n {
                self.stack.pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.stack.push(LuaValue::Nil);
            }
        }
    }

    fn push_nil(&mut self) {
        self.stack.push(LuaValue::Nil);
    }

    fn push_boolean(&mut self, b: bool) {
        self.stack.push(LuaValue::Bool(b));
    }

    fn push_integer(&mut self, n: i64) {
        self.stack.push(LuaValue::Int64(n));
    }

    fn push_number(&mut self, n: f64) {
        self.stack.push(LuaValue::Float64(n));
    }

    fn push_string(&mut self, s: String) {
        self.stack.push(LuaValue::LuaString(s));
    }

    fn type_name(&self, cur_type: i8) -> &str {
        match cur_type {
            LUA_TNONE => "no value",
            LUA_TNIL => "nil",
            LUA_TBOOLEAN => "boolean",
            LUA_TNUMBER => "number",
            LUA_TSTRING => "string",
            LUA_TTABLE => "table",
            LUA_TFUNCTION => "function",
            LUA_TTHREAD => "thread",
            _ => "user data"
        }
    }

    fn type_id(&self, idx: isize) -> i8 {
        if self.stack.is_valid(idx) {
            return self.stack.get(idx).get_type();
        }
        return LUA_TNONE;
    }

    fn is_none(&self, idx: isize) -> bool {
        self.type_id(idx) == LUA_TNONE
    }

    fn is_nil(&self, idx: isize) -> bool {
        self.type_id(idx) == LUA_TNIL
    }

    fn is_none_or_nil(&self, idx: isize) -> bool {
        self.type_id(idx) <= LUA_TNIL
    }

    fn is_bool(&self, idx: isize) -> bool {
        self.type_id(idx) == LUA_TBOOLEAN
    }

    // warning: number is also a string
    fn is_string(&self, idx: isize) -> bool {
        let cur_type_id = self.type_id(idx);
        cur_type_id == LUA_TSTRING || cur_type_id == LUA_TNUMBER
    }

    fn is_number(&self, idx: isize) -> bool {
        let (_, result) = self.to_numberx(idx);
        result
    }

    fn is_integer(&self, idx: isize) -> bool {
        let (_, result) = self.to_integerx(idx);
        result
    }

    fn to_boolean(&self, idx: isize) -> bool {
        let val = self.stack.get(idx);
        val.to_bool()
    }

    fn to_number(&self, idx: isize) -> f64 {
        let (result, _) = self.to_numberx(idx);
        result
    }

    fn to_numberx(&self, idx: isize) -> (f64, bool) {
        let val = self.stack.get(idx);
        val.to_numberx()
    }

    fn to_integer(&self, idx: isize) -> i64 {
        let (result, _) = self.to_integerx(idx);
        result
    }

    fn to_integerx(&self, idx: isize) -> (i64, bool) {
        let val = self.stack.get(idx);
        val.to_integerx()
    }

    fn to_string(&self, idx: isize) -> String {
        let (result, _) = self.to_stringx(idx);
        result
    }

    fn to_stringx(&self, idx: isize) -> (String, bool) {
        let val = self.stack.get(idx);
        val.to_stringx()
    }
}