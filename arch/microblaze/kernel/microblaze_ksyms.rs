// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 */

// Kernel and architecture headers supplied by the surrounding build provide
// the declarations corresponding to the symbols exported below.

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
extern "C" {
    pub fn _mcount();
}
// EXPORT_SYMBOL(_mcount);

/*
 * Assembly functions that may be used (directly or indirectly) by modules
 */
// EXPORT_SYMBOL(__copy_tofrom_user);

#[cfg(feature = "CONFIG_OPT_LIB_ASM")]
extern "C" {
    pub fn memcpy();
    pub fn memmove();
}
// EXPORT_SYMBOL(memcpy);
// EXPORT_SYMBOL(memmove);

// EXPORT_SYMBOL(mbc);

extern "C" {
    pub fn __divsi3();
}
// EXPORT_SYMBOL(__divsi3);

extern "C" {
    pub fn __modsi3();
}
// EXPORT_SYMBOL(__modsi3);

extern "C" {
    pub fn __mulsi3();
}
// EXPORT_SYMBOL(__mulsi3);

extern "C" {
    pub fn __udivsi3();
}
// EXPORT_SYMBOL(__udivsi3);

extern "C" {
    pub fn __umodsi3();
}
// EXPORT_SYMBOL(__umodsi3);

#[cfg(feature = "CONFIG_MB_MANAGER")]
extern "C" {
    pub fn xmb_manager_register(
        phys_baseaddr: usize,
        cr_val: u32,
        callback: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
        priv_: *mut core::ffi::c_void,
        reset_callback: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void)>,
    );
    pub fn xmb_inject_err();
}
// EXPORT_SYMBOL(xmb_manager_register);
// EXPORT_SYMBOL(xmb_inject_err);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
