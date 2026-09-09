#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn handle_unaligned(regs: *mut pt_regs);
    pub fn check_unaligned(regs: *mut pt_regs) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
