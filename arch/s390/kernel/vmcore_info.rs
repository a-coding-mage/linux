// SPDX-License-Identifier: GPL-2.0-only

// Dependency intent from the original C source:
// linux/vmcore_info.h, linux/mm.h, asm/abs_lowcore.h, asm/sections.h, asm/setup.h

extern "C" {
    fn vmcoreinfo_append_str(format: *const core::ffi::c_char, ...);
    fn get_abs_lowcore() -> *mut lowcore;
    fn paddr_vmcoreinfo_note() -> usize;
    fn put_abs_lowcore(abs_lc: *mut lowcore);
    fn kaslr_offset() -> usize;
}

#[repr(C)]
pub struct lowcore {
    pub vmcore_info: usize,
}

/// Preserve the original VMCOREINFO_SYMBOL(lowcore_ptr) declaration/macro.
macro_rules! VMCOREINFO_SYMBOL {
    ($symbol:ident) => {
        // Supplied by the vmcore-info dependency.
        const _: &str = stringify!($symbol);
    };
}

/// Preserve the original VMCOREINFO_LENGTH(lowcore_ptr, NR_CPUS) declaration/macro.
macro_rules! VMCOREINFO_LENGTH {
    ($symbol:ident, $length:expr) => {
        const _: &str = concat!(stringify!($symbol), ":", stringify!($length));
    };
}

extern "C" {
    static __samode31: usize;
    static __eamode31: usize;
    static __identity_base: usize;
    static __kaslr_offset_phys: usize;
}

pub unsafe fn arch_crash_save_vmcoreinfo() {
    let abs_lc: *mut lowcore;

    VMCOREINFO_SYMBOL!(lowcore_ptr);
    VMCOREINFO_SYMBOL!(high_memory);
    VMCOREINFO_LENGTH!(lowcore_ptr, NR_CPUS);
    vmcoreinfo_append_str(
        b"SAMODE31=%lx\n\0".as_ptr() as *const core::ffi::c_char,
        __samode31 as usize,
    );
    vmcoreinfo_append_str(
        b"EAMODE31=%lx\n\0".as_ptr() as *const core::ffi::c_char,
        __eamode31 as usize,
    );
    vmcoreinfo_append_str(
        b"IDENTITYBASE=%lx\n\0".as_ptr() as *const core::ffi::c_char,
        __identity_base,
    );
    vmcoreinfo_append_str(
        b"KERNELOFFSET=%lx\n\0".as_ptr() as *const core::ffi::c_char,
        kaslr_offset(),
    );
    vmcoreinfo_append_str(
        b"KERNELOFFPHYS=%lx\n\0".as_ptr() as *const core::ffi::c_char,
        __kaslr_offset_phys,
    );
    abs_lc = get_abs_lowcore();
    (*abs_lc).vmcore_info = paddr_vmcoreinfo_note();
    put_abs_lowcore(abs_lc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
