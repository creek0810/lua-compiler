use crate::api::consts::*;
use std::hash::{Hash, Hasher};
use crate::state::lua_table::LuaTable;
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Clone)]
pub enum LuaValue {
    Nil,
    Bool(bool),
    Int64(i64),
    Float64(f64),
    LuaString(String),
    Table(Rc<RefCell<LuaTable>>),
}

// the trait `std::hash::Hash` is not implemented for `f64`
impl Hash for LuaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::Nil => 0.hash(state),
            LuaValue::Bool(b) => b.hash(state),
            LuaValue::Int64(i) => i.hash(state),
            LuaValue::Float64(n) => n.to_bits().hash(state),
            LuaValue::LuaString(s) => s.hash(state),
            LuaValue::Table(t) => t.borrow().hash(state),
        }
    }
}

impl PartialEq for LuaValue {
    fn eq(&self, other: &LuaValue) -> bool {
        if let (LuaValue::Nil, LuaValue::Nil) = (self, other) {
            true
        } else if let (LuaValue::Bool(x), LuaValue::Bool(y)) = (self, other) {
            x == y
        } else if let (LuaValue::Int64(x), LuaValue::Int64(y)) = (self, other) {
            x == y
        } else if let (LuaValue::Float64(x), LuaValue::Float64(y)) = (self, other) {
            x == y
        } else if let (LuaValue::LuaString(x), LuaValue::LuaString(y)) = (self, other) {
            x == y
        } else if let (LuaValue::Table(x), LuaValue::Table(y)) = (self, other) {
            Rc::ptr_eq(x, y)
        } else {
            false
        }
    }
}

// the trait `std::cmp::Eq` is not implemented for `f64`
impl Eq for LuaValue {} // TODO

impl LuaValue {
    pub fn new_table(narr: usize, nrec: usize) -> LuaValue {
        LuaValue::Table(Rc::new(RefCell::new(LuaTable::new(narr, nrec))))
    }

    pub fn is_nil(&self) -> bool {
        match self{
            LuaValue::Nil => true,
            _ => false
        }
    }

    pub fn get_type(&self) -> i8 {
        match self {
            LuaValue::Nil => LUA_TNIL,
            LuaValue::Bool(_) => LUA_TBOOLEAN,
            LuaValue::Int64(_) => LUA_TNUMBER,
            LuaValue::Float64(_) => LUA_TNUMBER,
            LuaValue::LuaString(_) => LUA_TSTRING,
            LuaValue::Table(_) => LUA_TTABLE,
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            LuaValue::Nil => false,
            LuaValue::Bool(a) => *a,
            _ => true
        }
    }

    pub fn to_numberx(&self) -> (f64, bool) {
        match self {
            LuaValue::Int64(a) => (*a as f64, true),
            LuaValue::Float64(b) => (*b, true),
            LuaValue::LuaString(c) => parse_float(c.clone()),
            _ => (0.0, false)
        }
    }

    pub fn to_integerx(&self) -> (i64, bool) {
        match self {
            LuaValue::Int64(a) => (*a, true),
            LuaValue::Float64(b) => float_to_integer(*b),
            LuaValue::LuaString(c) => string_to_integer(c.clone()),
            _ => (0, false)
        }
    }

    pub fn to_stringx(&self) -> (String, bool) {
        match self {
            LuaValue::LuaString(a) => (a.clone(), true),
            LuaValue::Int64(b) => (b.to_string(), true),
            LuaValue::Float64(c) => (c.to_string(), true),
            _ => (String::new(), false)
        }
    }
}

pub fn float_to_integer(n: f64) -> (i64, bool){
    let i = n as i64;
    if i as f64 == n {
        return (i, true);
    } else {
        return (0, false);
    }
}

pub fn parse_integer(s: String) -> (i64, bool) {
    let i = s.parse::<i64>();
    match i {
        Ok(result) => (result, true),
        Err(_) => (0, false)
    }
}

pub fn parse_float(s: String) -> (f64, bool) {
    let i = s.parse::<f64>();
    match i {
        Ok(result) => (result, true),
        Err(_) => (0.0, false)
    }
}

pub fn string_to_integer(s: String) -> (i64, bool) {
    let (int_res, int_ok) = parse_integer(s.clone());
    if int_ok {
        return (int_res, int_ok);
    }

    let (f_res, f_ok) = parse_float(s.clone());
    if f_ok {
        return (f_res as i64, f_ok);
    }
    return (0, false);

}