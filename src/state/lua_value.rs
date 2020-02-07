use crate::api::consts::*;

#[derive(Clone)]
pub enum LuaValue {
    Nil,
    Bool(bool),
    Int64(i64),
    Float64(f64),
    LuaString(String)
}

impl LuaValue {

    pub fn get_type(&self) -> i8 {
        match self {
            LuaValue::Nil => LUA_TNIL,
            LuaValue::Bool(_) => LUA_TBOOLEAN,
            LuaValue::Int64(_) => LUA_TNUMBER,
            LuaValue::Float64(_) => LUA_TNUMBER,
            LuaValue::LuaString(_) => LUA_TSTRING
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
            LuaValue::Float64(b) => (*b as i64, true),
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