/* SPDX-License-Identifier: GPL-2.0 */

/* The C header is excluded when building for the assembler. */

unsafe extern "C" {
    pub static mut nospec_disable: ::core::ffi::c_int;
    pub static mut nobp: ::core::ffi::c_int;

    pub fn test_facility(facility: ::core::ffi::c_int) -> bool;

    pub fn nospec_init_branches();
    pub fn nospec_auto_detect();
    pub fn nospec_revert(start: *mut i32, end: *mut i32);

    pub fn __s390_indirect_jump_r1();
    pub fn __s390_indirect_jump_r2();
    pub fn __s390_indirect_jump_r3();
    pub fn __s390_indirect_jump_r4();
    pub fn __s390_indirect_jump_r5();
    pub fn __s390_indirect_jump_r6();
    pub fn __s390_indirect_jump_r7();
    pub fn __s390_indirect_jump_r8();
    pub fn __s390_indirect_jump_r9();
    pub fn __s390_indirect_jump_r10();
    pub fn __s390_indirect_jump_r11();
    pub fn __s390_indirect_jump_r12();
    pub fn __s390_indirect_jump_r13();
    pub fn __s390_indirect_jump_r14();
    pub fn __s390_indirect_jump_r15();
}

#[inline]
pub unsafe fn nobp_enabled() -> bool {
    /* __is_defined(__DECOMPRESSOR): build-time configuration condition. */
    if cfg!(feature = "__DECOMPRESSOR") {
        return false;
    }
    (nobp != 0) && unsafe { test_facility(82) }
}

#[inline]
pub unsafe fn nospec_uses_trampoline() -> bool {
    /* __is_defined(CC_USING_EXPOLINE): build-time configuration condition. */
    cfg!(feature = "CC_USING_EXPOLINE") && nospec_disable == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
