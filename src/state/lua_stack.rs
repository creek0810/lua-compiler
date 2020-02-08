use crate::state::lua_value::LuaValue;

pub struct LuaStack {
    vec: Vec<LuaValue>,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        LuaStack {
            vec: Vec::with_capacity(size)
        }
    }

    pub fn check(&mut self, size: usize) {
        self.vec.reserve(size);
    }

    pub fn top(&self) -> isize {
        self.vec.len() as isize
    }

    pub fn push(&mut self, cur_data: LuaValue) {
        self.vec.push(cur_data);
    }

    pub fn pop(&mut self) -> LuaValue {
        if self.vec.len() == 0 {
            panic!("stack underflow!");
        }
        return self.vec.pop().unwrap();
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx > 0 {
            return idx;
        }
        self.top() + 1 + idx
    }

    pub fn is_valid(&self, idx: isize) -> bool {
        let cur_abs_idx = self.abs_index(idx);
        cur_abs_idx > 0 && cur_abs_idx <= self.top()
    }

    pub fn get(&self, idx: isize) -> LuaValue {
        if self.is_valid(idx) {
            let cur_abs_idx = self.abs_index(idx) as usize - 1;
            return self.vec[cur_abs_idx].clone();
        }
        LuaValue::Nil
    }

    pub fn set(&mut self, idx: isize, val: LuaValue) {
        if self.is_valid(idx) {
            let cur_abs_idx = self.abs_index(idx) as usize - 1;
            self.vec[cur_abs_idx] = val;
            return;
        }
        panic!("invalid idx!")
    }

    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.vec.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
}