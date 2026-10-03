//! A typed key sequence (0x20 bytes, no class name in the database): a std::string of
//! expected input at +0x04 and the matched length at +0x1c. A `'\0'` byte in the string marks
//! that the next byte is a virtual-key code to be matched by `KeyDown`; other bytes are
//! characters matched (case-insensitively) by `KeyChar`.

/// The key sequence matcher.
#[derive(Debug, Clone, Default)]
pub struct CheatCode {
    /// +0x04 the sequence (std::string).
    pub field_0x4: Vec<u8>,
    /// +0x1c characters matched so far.
    pub field_0x1c: u32,
}

/// port: 00504d30 FUN_00504d30
/// Constructor: an empty sequence.
pub fn FUN_00504d30() -> CheatCode {
    CheatCode { field_0x4: Vec::new(), field_0x1c: 0 }
}

/// port: 0050ff90 FUN_0050ff90
/// Constructor from a string (`FUN_0050e840`).
pub fn FUN_0050ff90(param_1: &[u8]) -> CheatCode {
    let mut c = CheatCode::default();
    FUN_0050e840(&mut c, param_1);
    c
}

/// port: 0050e840 FUN_0050e840
/// Sets the sequence and restarts matching.
pub fn FUN_0050e840(this: &mut CheatCode, param_1: &[u8]) {
    this.field_0x4 = param_1.to_vec();
    this.field_0x1c = 0;
}

/// port: 00505ba0 FUN_00505ba0
/// Appends a virtual key (`'\0'`, key).
pub fn FUN_00505ba0(this: &mut CheatCode, param_1: u8) {
    this.field_0x4.push(0);
    this.field_0x4.push(param_1);
}

/// port: 00505bd0 FUN_00505bd0
/// Appends a character.
pub fn FUN_00505bd0(this: &mut CheatCode, param_1: u8) {
    this.field_0x4.push(param_1);
}

/// port: 00504180 FUN_00504180
/// `KeyChar`: advances on the expected character (`_tolower` on both), restarts on any other;
/// true when the whole sequence has been matched (and restarts).
pub fn FUN_00504180(this: &mut CheatCode, param_1: u8) -> bool {
    let len = this.field_0x4.len() as u32;
    if len != 0 {
        let expected = this.field_0x4.get(this.field_0x1c as usize).copied().unwrap_or(0);
        if expected.to_ascii_lowercase() == param_1.to_ascii_lowercase() {
            this.field_0x1c += 1;
            if this.field_0x1c == len {
                this.field_0x1c = 0;
                return true;
            }
        } else {
            this.field_0x1c = 0;
        }
    }
    false
}

/// port: 005041f0 FUN_005041f0
/// `KeyDown`: at a `'\0'` marker, advances past the pair on the expected key and restarts on
/// any other key; elsewhere in the sequence the key is ignored. True when matched.
pub fn FUN_005041f0(this: &mut CheatCode, param_1: u8) -> bool {
    let idx = this.field_0x1c as i32;
    let len = this.field_0x4.len() as i32;
    if len <= idx + 1 {
        return false;
    }
    if this.field_0x4[idx as usize] == 0 {
        if this.field_0x4[(idx + 1) as usize] == param_1 {
            this.field_0x1c += 2;
            if this.field_0x1c as i32 == len {
                this.field_0x1c = 0;
                return true;
            }
        } else {
            this.field_0x1c = 0;
        }
    }
    false
}
