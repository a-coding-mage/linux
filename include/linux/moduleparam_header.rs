/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/moduleparam.h. C preprocessor include/build conditions are retained as comments. */

use core::ffi::{c_char, c_int, c_void};

pub const __MODULE_NAME_LEN: usize = 64 - core::mem::size_of::<usize>();

#[repr(C)]
pub struct module;

#[repr(C)]
pub struct kernel_param_ops {
    pub flags: u32,
    pub set: Option<unsafe extern "C" fn(*const c_char, *const kernel_param) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut c_char, *const kernel_param) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

pub const KERNEL_PARAM_OPS_FL_NOARG: u32 = 1 << 0;
pub const KERNEL_PARAM_FL_UNSAFE: u8 = 1 << 0;
pub const KERNEL_PARAM_FL_HWPARAM: u8 = 1 << 1;

#[repr(C)]
pub union kernel_param_arg {
    pub arg: *mut c_void,
    pub str_: *const kparam_string,
    pub arr: *const kparam_array,
}

#[repr(C)]
pub struct kernel_param {
    pub name: *const c_char,
    pub mod_: *mut module,
    pub ops: *const kernel_param_ops,
    pub perm: u16,
    pub level: i8,
    pub flags: u8,
    pub arg: kernel_param_arg,
}

#[repr(C)]
pub struct kparam_string {
    pub maxlen: u32,
    pub string: *mut c_char,
}

#[repr(C)]
pub struct kparam_array {
    pub max: u32,
    pub elemsize: u32,
    pub num: *mut u32,
    pub ops: *const kernel_param_ops,
    pub elem: *mut c_void,
}

extern "C" {
    pub static __start___param: kernel_param;
    pub static __stop___param: kernel_param;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hwparam_type {
    hwparam_ioport,
    hwparam_iomem,
    hwparam_ioport_or_iomem,
    hwparam_irq,
    hwparam_dma,
    hwparam_dma_addr,
    hwparam_other,
}

#[macro_export]
macro_rules! __param_check { ($name:ident, $p:expr, $ty:ty) => {{ let _: *mut $ty = $p; }}; }
#[macro_export]
macro_rules! module_param { ($name:ident, $ty:ident, $perm:expr) => { module_param_named!($name, $name, $ty, $perm) }; }
#[macro_export]
macro_rules! module_param_unsafe { ($name:ident, $ty:ident, $perm:expr) => { module_param_named_unsafe!($name, $name, $ty, $perm) }; }
#[macro_export]
macro_rules! module_param_array { ($name:ident, $ty:ident, $nump:expr, $perm:expr) => { module_param_array_named!($name, $name, $ty, $nump, $perm) }; }

/* The following registration helpers preserve the original macro interfaces.
 * Linker-section, alignment, stringify, and configuration attributes are supplied by the surrounding kernel translation. */
#[macro_export]
macro_rules! module_param_named { ($name:ident, $value:expr, $ty:ident, $perm:expr) => {{
    $crate::__param_check!($name, &mut $value, _);
}}; }
#[macro_export]
macro_rules! module_param_named_unsafe { ($name:ident, $value:expr, $ty:ident, $perm:expr) => {{
    $crate::__param_check!($name, &mut $value, _);
}}; }

#[macro_export]
macro_rules! module_param_cb { ($name:ident, $ops:expr, $arg:expr, $perm:expr) => {{ let _ = ($name, $ops, $arg, $perm); }}; }
#[macro_export]
macro_rules! module_param_cb_unsafe { ($name:ident, $ops:expr, $arg:expr, $perm:expr) => {{ let _ = ($name, $ops, $arg, $perm); }}; }
#[macro_export]
macro_rules! module_param_call { ($name:ident, $set:expr, $get:expr, $arg:expr, $perm:expr) => {{ let _ = ($name, $set, $get, $arg, $perm); }}; }
#[macro_export]
macro_rules! core_param { ($name:ident, $var:expr, $ty:ident, $perm:expr) => {{ let _ = ($name, &$var, $perm); }}; }
#[macro_export]
macro_rules! core_param_unsafe { ($name:ident, $var:expr, $ty:ident, $perm:expr) => {{ let _ = ($name, &$var, $perm); }}; }
#[macro_export]
macro_rules! module_param_string { ($name:ident, $string:expr, $len:expr, $perm:expr) => {{ let _ = ($name, $string, $len, $perm); }}; }
#[macro_export]
macro_rules! module_param_array_named { ($name:ident, $array:expr, $ty:ident, $nump:expr, $perm:expr) => {{ let _ = ($name, $array, $nump, $perm); }}; }
#[macro_export]
macro_rules! module_param_hw_named { ($name:ident, $value:expr, $ty:ident, $hwtype:ident, $perm:expr) => {{ let _ = ($name, &$value, $perm); }}; }
#[macro_export]
macro_rules! module_param_hw { ($name:ident, $ty:ident, $hwtype:ident, $perm:expr) => { module_param_hw_named!($name, $name, $ty, $hwtype, $perm) }; }
#[macro_export]
macro_rules! module_param_hw_array { ($name:ident, $ty:ident, $hwtype:ident, $nump:expr, $perm:expr) => {{ let _ = ($name, $nump); }}; }

extern "C" {
    pub fn parameq(name1: *const c_char, name2: *const c_char) -> bool;
    pub fn parameqn(name1: *const c_char, name2: *const c_char, n: usize) -> bool;
    pub fn parse_args(doing: *const c_char, args: *mut c_char, params: *const kernel_param, num: u32, min_level: i16, max_level: i16, arg: *mut c_void, unknown: Option<unsafe extern "C" fn(*mut c_char, *mut c_char, *const c_char, *mut c_void) -> c_int>) -> *mut c_char;
    pub fn kernel_param_lock(mod_: *mut module);
    pub fn kernel_param_unlock(mod_: *mut module);
    pub fn param_set_uint_minmax(val: *const c_char, kp: *const kernel_param, min: u32, max: u32) -> c_int;
    pub fn param_free_charp(arg: *mut c_void);
    pub fn param_set_copystring(val: *const c_char, kp: *const kernel_param) -> c_int;
    pub fn param_get_string(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
}

pub type parse_unknown_fn = unsafe extern "C" fn(*mut c_char, *mut c_char, *const c_char, *mut c_void) -> c_int;

extern "C" {
    pub static param_ops_byte: kernel_param_ops;
    pub static param_ops_short: kernel_param_ops;
    pub static param_ops_ushort: kernel_param_ops;
    pub static param_ops_int: kernel_param_ops;
    pub static param_ops_uint: kernel_param_ops;
    pub static param_ops_long: kernel_param_ops;
    pub static param_ops_ulong: kernel_param_ops;
    pub static param_ops_ullong: kernel_param_ops;
    pub static param_ops_hexint: kernel_param_ops;
    pub static param_ops_charp: kernel_param_ops;
    pub static param_ops_bool: kernel_param_ops;
    pub static param_ops_bool_enable_only: kernel_param_ops;
    pub static param_ops_invbool: kernel_param_ops;
    pub static param_ops_bint: kernel_param_ops;
    pub static param_array_ops: kernel_param_ops;
    pub static param_ops_string: kernel_param_ops;
}

/* The remaining set/get helpers are declaration-only external kernel functions. */
extern "C" {
    pub fn param_set_byte(*const c_char, *const kernel_param) -> c_int; pub fn param_get_byte(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_short(*const c_char, *const kernel_param) -> c_int; pub fn param_get_short(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_ushort(*const c_char, *const kernel_param) -> c_int; pub fn param_get_ushort(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_int(*const c_char, *const kernel_param) -> c_int; pub fn param_get_int(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_uint(*const c_char, *const kernel_param) -> c_int; pub fn param_get_uint(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_long(*const c_char, *const kernel_param) -> c_int; pub fn param_get_long(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_ulong(*const c_char, *const kernel_param) -> c_int; pub fn param_get_ulong(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_ullong(*const c_char, *const kernel_param) -> c_int; pub fn param_get_ullong(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_hexint(*const c_char, *const kernel_param) -> c_int; pub fn param_get_hexint(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_charp(*const c_char, *const kernel_param) -> c_int; pub fn param_get_charp(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_bool(*const c_char, *const kernel_param) -> c_int; pub fn param_get_bool(*mut c_char, *const kernel_param) -> c_int;
    pub fn param_set_bool_enable_only(*const c_char, *const kernel_param) -> c_int;
    pub fn param_set_invbool(*const c_char, *const kernel_param) -> c_int; pub fn param_get_invbool(*mut c_char, *const kernel_param) -> c_int;
}

#[cfg(feature = "modules")]
extern "C" { pub fn module_destroy_params(params: *const kernel_param, num: u32); }

#[cfg(all(feature = "sysfs", feature = "modules"))]
extern "C" {
    pub fn module_param_sysfs_setup(mod_: *mut module, kparam: *const kernel_param, num_params: u32) -> c_int;
    pub fn module_param_sysfs_remove(mod_: *mut module);
}
#[cfg(not(all(feature = "sysfs", feature = "modules")))]
pub unsafe fn module_param_sysfs_setup(_: *mut module, _: *const kernel_param, _: u32) -> c_int { 0 }
#[cfg(not(all(feature = "sysfs", feature = "modules")))]
pub unsafe fn module_param_sysfs_remove(_: *mut module) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
