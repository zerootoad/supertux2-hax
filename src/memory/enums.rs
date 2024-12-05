use std::fmt;

#[derive(Debug)]
pub enum BufType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Float,
    Double,
}

#[derive(Debug)]
pub enum BufValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Float(f32),
    Double(f64),
}

impl BufValue {
    pub fn to_u8(&self) -> Option<u8> {
        match self {
            BufValue::U8(val) => Some(*val),
            BufValue::I8(val) => Some(*val as u8),
            _ => None,
        }
    }

    pub fn to_u16(&self) -> Option<u16> {
        match self {
            BufValue::U16(val) => Some(*val),
            BufValue::I16(val) => Some(*val as u16),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> Option<u32> {
        match self {
            BufValue::U32(val) => Some(*val),
            BufValue::I32(val) => Some(*val as u32),
            BufValue::U8(val) => Some(*val as u32),
            BufValue::U16(val) => Some(*val as u32),
            _ => None,
        }
    }

    pub fn to_u64(&self) -> Option<u64> {
        match self {
            BufValue::U64(val) => Some(*val),
            BufValue::I64(val) => Some(*val as u64),
            BufValue::U8(val) => Some(*val as u64),
            BufValue::U16(val) => Some(*val as u64),
            BufValue::U32(val) => Some(*val as u64),
            BufValue::I8(val) => Some(*val as u64),
            BufValue::I16(val) => Some(*val as u64),
            BufValue::I32(val) => Some(*val as u64),
            BufValue::Float(val) => Some(*val as u64),
            BufValue::Double(val) => Some(*val as u64),
        }
    }

    pub fn to_i8(&self) -> Option<i8> {
        match self {
            BufValue::I8(val) => Some(*val),
            BufValue::U8(val) => Some(*val as i8),
            _ => None,
        }
    }

    pub fn to_i16(&self) -> Option<i16> {
        match self {
            BufValue::I16(val) => Some(*val),
            BufValue::U16(val) => Some(*val as i16),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            BufValue::I32(val) => Some(*val),
            BufValue::U32(val) => Some(*val as i32),
            BufValue::I8(val) => Some(*val as i32),
            BufValue::I16(val) => Some(*val as i32),
            BufValue::U8(val) => Some(*val as i32),
            BufValue::U16(val) => Some(*val as i32),
            _ => None,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            BufValue::I64(val) => Some(*val),
            BufValue::U64(val) => Some(*val as i64),
            BufValue::I8(val) => Some(*val as i64),
            BufValue::I16(val) => Some(*val as i64),
            BufValue::I32(val) => Some(*val as i64),
            BufValue::U8(val) => Some(*val as i64),
            BufValue::U16(val) => Some(*val as i64),
            BufValue::U32(val) => Some(*val as i64),
            BufValue::Float(val) => Some(*val as i64),
            BufValue::Double(val) => Some(*val as i64),
        }
    }

    pub fn to_float(&self) -> Option<f32> {
        match self {
            BufValue::Float(val) => Some(*val),
            BufValue::U8(val) => Some(*val as f32),
            BufValue::U16(val) => Some(*val as f32),
            BufValue::U32(val) => Some(*val as f32),
            BufValue::I8(val) => Some(*val as f32),
            BufValue::I16(val) => Some(*val as f32),
            BufValue::I32(val) => Some(*val as f32),
            BufValue::U64(val) => Some(*val as f32),
            BufValue::I64(val) => Some(*val as f32),
            BufValue::Double(val) => Some(*val as f32),
            _ => None,
        }
    }

    pub fn to_double(&self) -> Option<f64> {
        match self {
            BufValue::Double(val) => Some(*val),
            BufValue::U8(val) => Some(*val as f64),
            BufValue::U16(val) => Some(*val as f64),
            BufValue::U32(val) => Some(*val as f64),
            BufValue::I8(val) => Some(*val as f64),
            BufValue::I16(val) => Some(*val as f64),
            BufValue::I32(val) => Some(*val as f64),
            BufValue::I64(val) => Some(*val as f64),
            BufValue::U64(val) => Some(*val as f64),
            BufValue::Float(val) => Some(*val as f64),
            _ => None,
        }
    }
}

impl fmt::Display for BufValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BufValue::U8(val) => write!(f, "{}", val),
            BufValue::U16(val) => write!(f, "{}", val),
            BufValue::U32(val) => write!(f, "{}", val),
            BufValue::U64(val) => write!(f, "{}", val),
            BufValue::I8(val) => write!(f, "{}", val),
            BufValue::I16(val) => write!(f, "{}", val),
            BufValue::I32(val) => write!(f, "{}", val),
            BufValue::I64(val) => write!(f, "{}", val),
            BufValue::Float(val) => write!(f, "{}", val),
            BufValue::Double(val) => write!(f, "{}", val),
        }
    }
}

impl BufType {
    pub fn bytes(&self) -> usize {
        match self {
            BufType::U8 | BufType::I8 => 1,
            BufType::U16 | BufType::I16 => 2,
            BufType::U32 | BufType::I32 => 4,
            BufType::U64 | BufType::I64 => 8,
            BufType::Float => 4,
            BufType::Double => 8,
        }
    }
}
