pub fn test_option_type() -> Option<u8>{
    let opt1: Option<u8> = None;
    return opt1;
}

pub fn test_option_chartype() -> Option<CharType>{
    let mut chartype: Option<CharType> = None;
    chartype = Some(CharType::Rage);
    return chartype;
}

#[derive(Debug)]
pub enum CharType {
    Archer,
    Warrior,
    Rage,
}