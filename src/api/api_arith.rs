use crate::state::lua_value::LuaValue;
pub const OPS: &'static [(fn(i64, i64) -> i64, fn(f64, f64) -> f64)] = &[
    (iadd, fadd),
    (isub, fsub),
    (imul, fmul),
    (imod, fmod),
    (inone, pow),
    (inone, div),
    (ii_div, fi_div),
    (band, fnone),
    (bor, fnone),
    (bxor, fnone),
    (shl, fnone),
    (shr, fnone),
    (iunm, funm),
    (bnot, fnone),
];

pub fn _arith(a: &LuaValue, b: &LuaValue, op: u8) -> LuaValue {
    let (iop, fop) = OPS[op as usize];
    if fop == fnone {
        // bit wise
        let (a_res, a_ok) = a.to_integerx();
        let (b_res, b_ok) = b.to_integerx();
        if a_ok && b_ok {
            return LuaValue::Int64(iop(a_res, b_res));
        }
    } else {
        // arith
        if iop != inone {
            // add,sub,mul,mod,idiv,unm
            if let LuaValue::Int64(x) = a {
                if let LuaValue::Int64(y) = b {
                    return LuaValue::Int64(iop(*x, *y));
                }
            }
        }

        let (a_res, a_ok) = a.to_numberx();
        let (b_res, b_ok) = b.to_numberx();
        if a_ok && b_ok {
            return LuaValue::Float64(fop(a_res, b_res));
        }
    }
    panic!("arithmetic error!");
}


pub fn iadd(a: i64, b: i64) -> i64{
    a + b
}

pub fn fadd(a: f64, b: f64) -> f64{
    a + b
}

pub fn isub(a: i64, b: i64) -> i64 {
    a - b
}

pub fn fsub(a: f64, b: f64) -> f64 {
    a - b
}


pub fn imul(a: i64, b: i64) -> i64 {
    a * b
}

pub fn fmul(a: f64, b: f64) -> f64 {
    a * b
}

pub fn imod(a: i64, b: i64) -> i64 {
    a - ii_div(a, b) * b
}

pub fn fmod(a: f64, b: f64) -> f64 {
    a - fi_div(a, b) * b
}

fn pow(a: f64, b: f64) -> f64 {
    a.powf(b)
}

pub fn div(a: f64, b: f64) -> f64 {
    a / b
}

pub fn ii_div(a: i64, b: i64) -> i64 {
    if a > 0 && b > 0 || a < 0 && b < 0 || a % b == 0 {
        return a / b;
    }
    a / b - 1
}

pub fn fi_div(a: f64, b: f64) -> f64 {
    (a / b).floor()
}

pub fn band(a: i64, b: i64) -> i64 {
    a & b
}

pub fn bor(a: i64, b: i64) -> i64 {
    a | b
}

pub fn bxor(a: i64, b: i64) -> i64 {
    a ^ b
}

pub fn shl(a: i64, n: i64) -> i64 {
    if n >= 0 {
        return a << n;
    }
    shr(a, -n)
}

pub fn shr(a: i64, n: i64) -> i64 {
    if n >= 0 {
        return ((a as u64) >> n) as i64;
    }
    shl(a, -n)
}

pub fn iunm(a: i64, _: i64) -> i64 {
    -a
}

pub fn funm(a: f64, _: f64) -> f64 {
    -a
}

pub fn bnot(a: i64, _:i64) -> i64 {
    !a
}

fn inone(_: i64, _: i64) -> i64 {
    0
}

fn fnone(_: f64, _: f64) -> f64 {
    0.0
}



