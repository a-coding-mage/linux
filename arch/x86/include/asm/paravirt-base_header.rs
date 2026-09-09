/* SPDX-License-Identifier: GPL-2.0-only */

/* Wrapper type for pointers to code which uses the non-standard calling
 * convention. See PV_CALL_SAVE_REGS_THUNK below. */
#[repr(C)]
pub struct paravirt_callee_save {
    pub func: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct pv_info {
    /* CONFIG_PARAVIRT_XXL: __USER_CS if none */
    #[cfg(CONFIG_PARAVIRT_XXL)]
    pub extra_user_64bit_cs: u16,
    pub io_delay: bool,
    pub name: *const core::ffi::c_char,
}

unsafe extern "C" {
    pub fn default_banner();
    pub static mut pv_info: pv_info;
    pub fn paravirt_ret0() -> core::ffi::c_ulong;
    /* CONFIG_PARAVIRT_XXL */
    #[cfg(CONFIG_PARAVIRT_XXL)]
    pub fn _paravirt_ident_64(value: u64) -> u64;
}

/* #define paravirt_nop ((void *)nop_func) */
macro_rules! paravirt_nop {
    () => {{ nop_func }};
}

/* #ifdef CONFIG_PARAVIRT */
#[cfg(CONFIG_PARAVIRT)]
macro_rules! call_io_delay {
    () => {{ unsafe { pv_info.io_delay } }};
}

/* #ifdef CONFIG_PARAVIRT_SPINLOCKS */
#[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
unsafe extern "C" {
    pub fn paravirt_set_cap();
}

#[cfg(not(CONFIG_PARAVIRT_SPINLOCKS))]
#[inline]
pub fn paravirt_set_cap() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
