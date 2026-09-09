// Dependency declarations from <linux/types.h> and <asm/ptrace.h> are
// supplied by the surrounding translation unit.

// CONFIG_X86_UMIP is a build-time condition from the original header.
#[cfg(feature = "CONFIG_X86_UMIP")]
unsafe extern "C" {
    pub fn fixup_umip_exception(regs: *mut pt_regs) -> bool;
}

#[cfg(not(feature = "CONFIG_X86_UMIP"))]
#[inline]
pub unsafe fn fixup_umip_exception(_regs: *mut pt_regs) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
