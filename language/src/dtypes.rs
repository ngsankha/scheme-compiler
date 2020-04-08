use std::ops::Add;
use std::ops::Sub;

#[repr(C)]
pub struct Val(pub u64);

#[derive(PartialEq, Debug)]
pub enum TypeTag {
    IntTag,
    BoolTag
}

const PADDING: u64 = 1;

impl Val {
    pub fn type_tag(&self) -> TypeTag {
        match self.0 & !(!0 << PADDING) {
            0 => TypeTag::IntTag,
            1 => TypeTag::BoolTag,
            _ => unreachable!()
        }
    }

    pub fn to_u64(&self) -> u64 {
        self.0 >> PADDING
    }

    pub fn to_bool(&self) -> bool {
        if (self.0 >> PADDING) > 0 {
            true
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        match self.type_tag() {
            TypeTag::IntTag => self.to_u64().to_string(),
            TypeTag::BoolTag => if self.to_bool() {
                "#t".to_string()
            } else {
                "#f".to_string()
            }
        }
    }
}

impl Into<Val> for u64 {
    fn into(self) -> Val {
        Val((self << PADDING) | TypeTag::IntTag as u64)
    }
}

impl Into<Val> for bool {
    fn into(self) -> Val {
        if self {
            Val((1 << PADDING) | TypeTag::BoolTag as u64)
        } else {
            Val((0 << PADDING) | TypeTag::BoolTag as u64)
        }
    }
}

impl Add for Val {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.type_tag(), TypeTag::IntTag);
        assert_eq!(other.type_tag(), TypeTag::IntTag);
        (self.to_u64() + other.to_u64()).into()
    }
}

impl Sub for Val {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.type_tag(), TypeTag::IntTag);
        assert_eq!(other.type_tag(), TypeTag::IntTag);
        (self.to_u64() - other.to_u64()).into()
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
