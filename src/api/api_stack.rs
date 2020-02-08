use crate::state::lua_state::LuaState;
use crate::state::lua_value::LuaValue;
use crate::api::consts::*;
use crate::api::api_arith;
use crate::api::api_cmp;

pub trait LuaAPI {
    // basic operation
    fn compare(&mut self, idx1: isize, idx2: isize, op: u8) -> bool;
    fn arith(&mut self, op: u8);
    fn concat(&mut self, n: isize);
    fn len(&mut self, idx: isize);
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
    // table function
    fn create_table(&mut self, n_arr: usize, n_rec: usize);
    fn new_table(&mut self);
    fn get_table(&mut self, idx: isize) -> i8;
    fn _get_table(&mut self, t: LuaValue, k: LuaValue) -> i8;
    fn get_field(&mut self, idx: isize, k: String) -> i8;
    fn get_i(&mut self, idx: isize, i: i64) -> i8;
    fn _set_table(&mut self, t: &LuaValue, k: LuaValue, v: LuaValue);
    fn set_table(&mut self, idx: isize);
    fn set_field(&mut self, idx: isize, k: String);
    fn set_i(&mut self, idx: isize, i: i64);
}

impl LuaAPI for LuaState {
    fn compare(&mut self, idx1: isize, idx2: isize, op: u8) -> bool {
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

    fn len(&mut self, idx: isize) {
        let tmp = self.stack.get(idx);
        let result = match tmp {
            LuaValue::LuaString(s) => s.len(),
            LuaValue::Table(t) => t.borrow().len(),
            _ => panic!("length error!")
        };
        self.stack.push(LuaValue::Int64(result as i64));
    }

    fn concat(&mut self, n: isize) {
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

    fn arith(&mut self, op: u8) {
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

    fn create_table(&mut self, n_arr: usize, n_rec: usize) {
        self.stack.push(LuaValue::new_table(n_arr, n_rec));
    }

    fn new_table(&mut self) {
        self.create_table(0, 0);
    }

    fn get_table(&mut self, idx: isize) -> i8 {
        let t = self.stack.get(idx);
        let k = self.stack.pop();
        self._get_table(t, k)
    }

    fn _get_table(&mut self, t: LuaValue, k: LuaValue) -> i8 {
        if let LuaValue::Table(tbl) = t {
            let v = tbl.borrow().get(&k);
            self.stack.push(v.clone());
            return v.get_type();
        }
        panic!("not a table!");
    }

    fn get_field(&mut self, idx: isize, k: String) -> i8 {
        let t = self.stack.get(idx);
        self._get_table(t, LuaValue::LuaString(k))
    }

    fn get_i(&mut self, idx: isize, i: i64) -> i8 {
        let t = self.stack.get(idx);
        self._get_table(t, LuaValue::Int64(i))
    }

    fn _set_table(&mut self, t: &LuaValue, k: LuaValue, v: LuaValue) {
        if let LuaValue::Table(tbl) = t {
            tbl.borrow_mut().put(k, v);
            return
        }
        panic!("not a table!");
    }

    fn set_table(&mut self, idx: isize) {
        let t = self.stack.get(idx);
        let v = self.stack.pop();
        let k = self.stack.pop();
        self._set_table(&t, k, v);
    }

    fn set_field(&mut self, idx: isize, k: String) {
        let t = self.stack.get(idx);
        let v = self.stack.pop();
        self._set_table(&t, LuaValue::LuaString(k), v);
    }

    fn set_i(&mut self, idx: isize, i: i64) {
        let t = self.stack.get(idx);
        let v = self.stack.pop();
        self._set_table(&t, LuaValue::Int64(i), v)
    }
}