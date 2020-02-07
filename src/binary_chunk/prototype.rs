// tag
#[repr(u8)]
pub enum Tag {
    Nil = 0x00,
    Bool = 0x01,
    Number = 0x03,
    Integer = 0x13,
    Short_str = 0x04,
    Long_str = 0x14,
}

impl From<u8> for Tag {
    fn from(data: u8) -> Self {
        match data {
            0x00 => Tag::Nil,
            0x01 => Tag::Bool,
            0x03 => Tag::Number,
            0x13 => Tag::Integer,
            0x04 => Tag::Short_str,
            0x14 => Tag::Long_str,
            _ => panic!("Tag convert fail!")
        }
    }
}

// header
pub struct UpValue {
    pub in_stack: u8,
    pub idx: u8
}

pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32
}

pub enum Constant {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    LuaStr(String),
}

pub struct Prototype {
    pub source: String,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub up_values: Vec<UpValue>,
    pub protos: Vec<Prototype>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub up_value_names: Vec<String>
}