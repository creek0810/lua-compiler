pub struct Header {
    pub signature: [u8; 4],
    pub version: u8,
    pub format: u8,
    pub luac_data: [u8; 6],
    pub cint_size: u8,
    pub sizet_size: u8,
    pub instruction_size: u8,
    pub lua_integer_size: u8,
    pub lua_number_size: u8,
    pub luac_int: i64,
    pub luac_num: f64,
}

pub const HEADER: Header = Header {
    signature: [0x1B, 0x4C, 0x75, 0x61],
    version: 0x53,
    format: 0x00,
    luac_data: [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A],
    cint_size: 0x04,
    sizet_size: 0x08,
    instruction_size: 0x04,
    lua_integer_size: 0x08,
    lua_number_size: 0x08,
    luac_int: 0x5678,
    luac_num: 370.5,
};