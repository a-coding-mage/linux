/* SPDX-License-Identifier: LGPL-2.0+ */
/* Generic MTRR (Memory Type Range Register) ioctls. */

/* Defines for hardware MTRR registers. */
pub const MTRR_CAP_VCNT: u64 = (1u64 << 8) - 1;
pub const MTRR_CAP_FIX: u64 = 1u64 << 8;
pub const MTRR_CAP_WC: u64 = 1u64 << 10;

pub const MTRR_DEF_TYPE_TYPE: u64 = (1u64 << 8) - 1;
pub const MTRR_DEF_TYPE_FE: u64 = 1u64 << 10;
pub const MTRR_DEF_TYPE_E: u64 = 1u64 << 11;

pub const MTRR_DEF_TYPE_ENABLE: u64 = MTRR_DEF_TYPE_FE | MTRR_DEF_TYPE_E;
pub const MTRR_DEF_TYPE_DISABLE: u64 = !(MTRR_DEF_TYPE_TYPE | MTRR_DEF_TYPE_ENABLE);

pub const MTRR_PHYSBASE_TYPE: u64 = (1u64 << 8) - 1;
pub const MTRR_PHYSBASE_RSVD: u64 = ((1u64 << 12) - 1) & !((1u64 << 8) - 1);

pub const MTRR_PHYSMASK_RSVD: u64 = (1u64 << 11) - 1;
pub const MTRR_PHYSMASK_V: u64 = 1u64 << 11;

#[repr(C)]
pub struct mtrr_state_type {
    pub var_ranges: [mtrr_var_range; MTRR_MAX_VAR_RANGES],
    pub fixed_ranges: [mtrr_type; MTRR_NUM_FIXED_RANGES],
    pub enabled: u8,
    pub have_fixed: bool,
    pub def_type: mtrr_type,
}

/*
 * The following functions are for use by other drivers that cannot use
 * arch_phys_wc_add and arch_phys_wc_del.
 */
#[cfg(CONFIG_MTRR)]
extern "C" {
    pub fn mtrr_bp_init();
    pub fn guest_force_mtrr_state(var: *mut mtrr_var_range, num_var: c_uint, def_type: mtrr_type);
    pub fn mtrr_type_lookup(addr: u64, end: u64, uniform: *mut u8) -> u8;
    pub fn mtrr_save_fixed_ranges(arg: *mut core::ffi::c_void);
    pub fn mtrr_save_state();
    pub fn mtrr_add(base: c_ulong, size: c_ulong, type_: c_uint, increment: bool) -> c_int;
    pub fn mtrr_add_page(base: c_ulong, size: c_ulong, type_: c_uint, increment: bool) -> c_int;
    pub fn mtrr_del(reg: c_int, base: c_ulong, size: c_ulong) -> c_int;
    pub fn mtrr_del_page(reg: c_int, base: c_ulong, size: c_ulong) -> c_int;
    pub fn mtrr_trim_uncached_memory(end_pfn: c_ulong) -> c_int;
    pub fn amd_special_default_mtrr() -> c_int;
    pub fn mtrr_disable();
    pub fn mtrr_enable();
    pub fn mtrr_generic_set_state();
}

/* CONFIG_MTRR-disabled fallback implementations. */
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn guest_force_mtrr_state(_var: *mut mtrr_var_range, _num_var: c_uint, _def_type: mtrr_type) {}

#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_type_lookup(_addr: u64, _end: u64, uniform: *mut u8) -> u8 {
    /* Return the default MTRR type, without any known other types in that range. */
    *uniform = 1;
    MTRR_TYPE_UNCACHABLE
}

#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_save_fixed_ranges(_arg: *mut core::ffi::c_void) {}
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_save_state() {}
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_add(_base: c_ulong, _size: c_ulong, _type_: c_uint, _increment: bool) -> c_int { -ENODEV }
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_add_page(_base: c_ulong, _size: c_ulong, _type_: c_uint, _increment: bool) -> c_int { -ENODEV }
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_del(_reg: c_int, _base: c_ulong, _size: c_ulong) -> c_int { -ENODEV }
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_del_page(_reg: c_int, _base: c_ulong, _size: c_ulong) -> c_int { -ENODEV }
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_trim_uncached_memory(_end_pfn: c_ulong) -> c_int { 0 }
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_bp_init() {}
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_disable() {}
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_enable() {}
#[cfg(not(CONFIG_MTRR))]
pub unsafe fn mtrr_generic_set_state() {}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct mtrr_sentry32 {
    pub base: compat_ulong_t,
    pub size: compat_uint_t,
    pub type_: compat_uint_t,
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct mtrr_gentry32 {
    pub regnum: compat_ulong_t,
    pub base: compat_uint_t,
    pub size: compat_uint_t,
    pub type_: compat_uint_t,
}

/* CONFIG_COMPAT ioctl constants retain the source ioctl encoding. */
#[cfg(CONFIG_COMPAT)]
pub const MTRR_IOCTL_BASE: u8 = b'M';
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_ADD_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 0, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_SET_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 1, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_DEL_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 2, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_GET_ENTRY: c_ulong = _IOWR(MTRR_IOCTL_BASE, 3, mtrr_gentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_KILL_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 4, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_ADD_PAGE_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 5, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_SET_PAGE_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 6, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_DEL_PAGE_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 7, mtrr_sentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_GET_PAGE_ENTRY: c_ulong = _IOWR(MTRR_IOCTL_BASE, 8, mtrr_gentry32);
#[cfg(CONFIG_COMPAT)] pub const MTRRIOC32_KILL_PAGE_ENTRY: c_ulong = _IOW(MTRR_IOCTL_BASE, 9, mtrr_sentry32);

/* Bit fields for enabled in struct mtrr_state_type. */
pub const MTRR_STATE_SHIFT: u32 = 10;
pub const MTRR_STATE_MTRR_FIXED_ENABLED: u64 = MTRR_DEF_TYPE_FE >> MTRR_STATE_SHIFT;
pub const MTRR_STATE_MTRR_ENABLED: u64 = MTRR_DEF_TYPE_E >> MTRR_STATE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
