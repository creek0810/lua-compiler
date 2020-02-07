use crate::state::lua_value::LuaValue;

macro_rules! cmp {
    ($a:ident $op:tt $b:ident) => {
        match $a {
            LuaValue::LuaString(x) => match $b {
                LuaValue::LuaString(y) => Some(x $op y),
                _ => None,
            },
            LuaValue::Int64(x) => match $b {
                LuaValue::Int64(y) => Some(x $op y),
                LuaValue::Float64(y) => Some((*x as f64) $op *y),
                _ => None,
            },
            LuaValue::Float64(x) => match $b {
                LuaValue::Float64(y) => Some(x $op y),
                LuaValue::Int64(y) => Some(*x $op (*y as f64)),
                _ => None,
            },
            _ => None,
        }
    }
}


pub fn _eq(a: &LuaValue, b: &LuaValue) -> bool {
    if let Some(x) = cmp!(a == b) {
        return x;
    } else {
        match a {
            LuaValue::Nil => match b {
                LuaValue::Nil => true,
                _ => false
            },
            LuaValue::Bool(cur_bool) => match b {
                LuaValue::Bool(cur_bool2) => cur_bool == cur_bool2,
                _ => false
            },
            _ => false
        }
    }
}

pub fn _lt(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a < b)
}

pub fn _le(a: &LuaValue, b: &LuaValue) -> Option<bool> {
    cmp!(a <= b)
}