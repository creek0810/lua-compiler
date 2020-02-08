use crate::state::lua_stack::LuaStack;
use crate::binary_chunk::prototype::Prototype;

pub struct LuaState {
    pub stack: LuaStack,
    pub proto: Prototype,
    pub pc: isize,
}

impl LuaState {
    pub fn new(size: usize, proto: Prototype) -> LuaState {
        LuaState {
            stack: LuaStack::new(size),
            proto: proto,
            pc: 0,
        }
    }
}