/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/dynamic_debug.h. Configuration-dependent C includes
// and attributes are intentionally represented by comments or Rust ABI items.

#[repr(C)]
pub struct _ddebug {
    pub modname: *const core::ffi::c_char,
    pub function: *const core::ffi::c_char,
    pub filename: *const core::ffi::c_char,
    pub format: *const core::ffi::c_char,
    // C bit-fields: lineno:18, class_id:6, flags:8.
    pub lineno: u32,
    pub class_id: u32,
    pub flags: u8,
    // key is present only with CONFIG_JUMP_LABEL; its external C layout is
    // supplied by the corresponding jump-label dependency.
    #[cfg(CONFIG_JUMP_LABEL)]
    pub key: _ddebug_key,
}

#[cfg(CONFIG_JUMP_LABEL)]
#[repr(C)]
pub union _ddebug_key {
    pub dd_key_true: static_key_true,
    pub dd_key_false: static_key_false,
}

pub const CLS_BITS: u32 = 6;
pub const _DPRINTK_CLASS_DFLT: u32 = (1 << CLS_BITS) - 1;
pub const _DPRINTK_FLAGS_NONE: u32 = 0;
pub const _DPRINTK_FLAGS_PRINT: u32 = 1 << 0;
pub const _DPRINTK_FLAGS_INCL_MODNAME: u32 = 1 << 1;
pub const _DPRINTK_FLAGS_INCL_FUNCNAME: u32 = 1 << 2;
pub const _DPRINTK_FLAGS_INCL_LINENO: u32 = 1 << 3;
pub const _DPRINTK_FLAGS_INCL_TID: u32 = 1 << 4;
pub const _DPRINTK_FLAGS_INCL_SOURCENAME: u32 = 1 << 5;
pub const _DPRINTK_FLAGS_INCL_STACK: u32 = 1 << 6;
pub const _DPRINTK_FLAGS_INCL_ANY: u32 = _DPRINTK_FLAGS_INCL_MODNAME
    | _DPRINTK_FLAGS_INCL_FUNCNAME
    | _DPRINTK_FLAGS_INCL_LINENO
    | _DPRINTK_FLAGS_INCL_TID
    | _DPRINTK_FLAGS_INCL_SOURCENAME
    | _DPRINTK_FLAGS_INCL_STACK;
#[cfg(DEBUG)]
pub const _DPRINTK_FLAGS_DEFAULT: u32 = _DPRINTK_FLAGS_PRINT;
#[cfg(not(DEBUG))]
pub const _DPRINTK_FLAGS_DEFAULT: u32 = 0;

#[repr(C)]
pub enum class_map_type {
    DD_CLASS_TYPE_DISJOINT_BITS,
    DD_CLASS_TYPE_LEVEL_NUM,
    DD_CLASS_TYPE_DISJOINT_NAMES,
    DD_CLASS_TYPE_LEVEL_NAMES,
}

#[repr(C)]
pub struct ddebug_class_map {
    pub link: list_head,
    pub r#mod: *mut module,
    pub mod_name: *const core::ffi::c_char,
    pub class_names: *const *const core::ffi::c_char,
    pub length: core::ffi::c_int,
    pub base: core::ffi::c_int,
    pub map_type: class_map_type,
}

#[repr(C)]
pub struct _ddebug_info {
    pub descs: *mut _ddebug,
    pub classes: *mut ddebug_class_map,
    pub num_descs: u32,
    pub num_classes: u32,
}

#[repr(C)]
pub union ddebug_class_param_bits {
    pub bits: *mut core::ffi::c_ulong,
    pub lvl: *mut u32,
}

#[repr(C)]
pub struct ddebug_class_param {
    pub value: ddebug_class_param_bits,
    pub flags: [core::ffi::c_char; 8],
    pub map: *const ddebug_class_map,
}

extern "C" {
    pub fn __dynamic_pr_debug(descriptor: *mut _ddebug, fmt: *const core::ffi::c_char, ...);
    pub fn __dynamic_dev_dbg(descriptor: *mut _ddebug, dev: *const device, fmt: *const core::ffi::c_char, ...);
    pub fn __dynamic_netdev_dbg(descriptor: *mut _ddebug, dev: *const net_device, fmt: *const core::ffi::c_char, ...);
    pub fn __dynamic_ibdev_dbg(descriptor: *mut _ddebug, ibdev: *const ib_device, fmt: *const core::ffi::c_char, ...);
    pub fn ddebug_dyndbg_module_param_cb(param: *mut core::ffi::c_char, val: *mut core::ffi::c_char, modname: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn param_set_dyndbg_classes(instr: *const core::ffi::c_char, kp: *const kernel_param) -> core::ffi::c_int;
    pub fn param_get_dyndbg_classes(buffer: *mut core::ffi::c_char, kp: *const kernel_param) -> core::ffi::c_int;
    pub static param_ops_dyndbg_classes: kernel_param_ops;
}

// External kernel types supplied by other translated headers.
extern "C" {
    pub type list_head;
    pub type module;
    pub type device;
    pub type net_device;
    pub type ib_device;
    pub type kernel_param;
    pub type kernel_param_ops;
    #[cfg(CONFIG_JUMP_LABEL)] pub type static_key_true;
    #[cfg(CONFIG_JUMP_LABEL)] pub type static_key_false;
}

#[cfg(not(CONFIG_DYNAMIC_DEBUG_CORE))]
pub unsafe fn ddebug_dyndbg_module_param_cb(param: *mut core::ffi::c_char, val: *mut core::ffi::c_char, modname: *const core::ffi::c_char) -> core::ffi::c_int {
    let _ = (param, val, modname);
    0
}

#[cfg(not(CONFIG_DYNAMIC_DEBUG_CORE))]
pub unsafe fn param_set_dyndbg_classes(_instr: *const core::ffi::c_char, _kp: *const kernel_param) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_DYNAMIC_DEBUG_CORE))]
pub unsafe fn param_get_dyndbg_classes(_buffer: *mut core::ffi::c_char, _kp: *const kernel_param) -> core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
