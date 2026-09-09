/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct msr_parts {
    pub l: u32,
    pub h: u32,
}

#[repr(C)]
pub union msr_union {
    pub parts: msr_parts,
    pub q: u64,
}

#[repr(C)]
pub struct msr {
    pub data: msr_union,
}

/*
 * The kernel proper already defines rdmsr()/wrmsr(), but they are not for the
 * boot kernel since they rely on tracepoint/exception handling infrastructure
 * that's not available here.
 */
pub unsafe fn raw_rdmsr(reg: u32, m: *mut msr) {
    core::arch::asm!(
        "rdmsr",
        out("eax") (*m).data.parts.l,
        out("edx") (*m).data.parts.h,
        in("ecx") reg,
    );
}

pub unsafe fn raw_wrmsr(reg: u32, m: *const msr) {
    core::arch::asm!(
        "wrmsr",
        in("ecx") reg,
        in("eax") (*m).data.parts.l,
        in("edx") (*m).data.parts.h,
        options(preserves_flags),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
