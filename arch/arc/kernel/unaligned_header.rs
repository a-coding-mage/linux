#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callee_regs {
    _private: [u8; 0],
}

// Build-time equivalent of CONFIG_ARC_EMUL_UNALIGNED.
#[cfg(feature = "CONFIG_ARC_EMUL_UNALIGNED")]
extern "C" {
    pub fn misaligned_fixup(
        address: core::ffi::c_ulong,
        regs: *mut pt_regs,
        cregs: *mut callee_regs,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_ARC_EMUL_UNALIGNED"))]
#[inline]
pub unsafe fn misaligned_fixup(
    _address: core::ffi::c_ulong,
    _regs: *mut pt_regs,
    _cregs: *mut callee_regs,
) -> core::ffi::c_int {
    // Not fixed
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
