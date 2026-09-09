/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/fwnode.h. */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum dev_dma_attr {
    DEV_DMA_NOT_SUPPORTED,
    DEV_DMA_NON_COHERENT,
    DEV_DMA_COHERENT,
}

pub enum fwnode_operations {}
pub enum device {}
pub enum list_head {}

pub const FWNODE_FLAG_LINKS_ADDED: usize = 0;
pub const FWNODE_FLAG_NOT_DEVICE: usize = 1;
pub const FWNODE_FLAG_INITIALIZED: usize = 2;
pub const FWNODE_FLAG_NEEDS_CHILD_BOUND_ON_ADD: usize = 3;
pub const FWNODE_FLAG_BEST_EFFORT: usize = 4;
pub const FWNODE_FLAG_VISITED: usize = 5;

#[repr(C)]
pub struct fwnode_handle {
    pub secondary: *mut fwnode_handle,
    pub ops: *const fwnode_operations,
    pub dev: *mut device,
    pub suppliers: list_head,
    pub consumers: list_head,
    pub flags: ::core::ffi::c_ulong,
}

pub const FWLINK_FLAG_CYCLE: u8 = 1 << 0;
pub const FWLINK_FLAG_IGNORE: u8 = 1 << 1;

#[repr(C)]
pub struct fwnode_link {
    pub supplier: *mut fwnode_handle,
    pub s_hook: list_head,
    pub consumer: *mut fwnode_handle,
    pub c_hook: list_head,
    pub flags: u8,
}

#[repr(C)]
pub struct fwnode_endpoint {
    pub port: ::core::ffi::c_uint,
    pub id: ::core::ffi::c_uint,
    pub local_fwnode: *const fwnode_handle,
}

pub const SWNODE_GRAPH_PORT_NAME_FMT: &[u8] = b"port@%u\0";
pub const SWNODE_GRAPH_ENDPOINT_NAME_FMT: &[u8] = b"endpoint@%u\0";
pub const NR_FWNODE_REFERENCE_ARGS: usize = 16;

#[repr(C)]
pub struct fwnode_reference_args {
    pub fwnode: *mut fwnode_handle,
    pub nargs: ::core::ffi::c_uint,
    pub args: [u64; NR_FWNODE_REFERENCE_ARGS],
}

#[repr(C)]
pub struct fwnode_operations {
    pub get: Option<unsafe extern "C" fn(*mut fwnode_handle) -> *mut fwnode_handle>,
    pub put: Option<unsafe extern "C" fn(*mut fwnode_handle)>,
    pub device_is_available: Option<unsafe extern "C" fn(*const fwnode_handle) -> bool>,
    pub device_get_match_data: Option<unsafe extern "C" fn(*const fwnode_handle, *const device) -> *const ::core::ffi::c_void>,
    pub device_dma_supported: Option<unsafe extern "C" fn(*const fwnode_handle) -> bool>,
    pub device_get_dma_attr: Option<unsafe extern "C" fn(*const fwnode_handle) -> dev_dma_attr>,
    pub property_present: Option<unsafe extern "C" fn(*const fwnode_handle, *const ::core::ffi::c_char) -> bool>,
    pub property_read_bool: Option<unsafe extern "C" fn(*const fwnode_handle, *const ::core::ffi::c_char) -> bool>,
    pub property_read_int_array: Option<unsafe extern "C" fn(*const fwnode_handle, *const ::core::ffi::c_char, ::core::ffi::c_uint, *mut ::core::ffi::c_void, usize) -> ::core::ffi::c_int>,
    pub property_read_string_array: Option<unsafe extern "C" fn(*const fwnode_handle, *const ::core::ffi::c_char, *mut *const ::core::ffi::c_char, usize) -> ::core::ffi::c_int>,
    pub get_name: Option<unsafe extern "C" fn(*const fwnode_handle) -> *const ::core::ffi::c_char>,
    pub get_name_prefix: Option<unsafe extern "C" fn(*const fwnode_handle) -> *const ::core::ffi::c_char>,
    pub get_parent: Option<unsafe extern "C" fn(*const fwnode_handle) -> *mut fwnode_handle>,
    pub get_next_child_node: Option<unsafe extern "C" fn(*const fwnode_handle, *mut fwnode_handle) -> *mut fwnode_handle>,
    pub get_named_child_node: Option<unsafe extern "C" fn(*const fwnode_handle, *const ::core::ffi::c_char) -> *mut fwnode_handle>,
    pub get_reference_args: Option<unsafe extern "C" fn(*const fwnode_handle, *const ::core::ffi::c_char, *const ::core::ffi::c_char, ::core::ffi::c_uint, ::core::ffi::c_uint, *mut fwnode_reference_args) -> ::core::ffi::c_int>,
    pub graph_get_next_endpoint: Option<unsafe extern "C" fn(*const fwnode_handle, *mut fwnode_handle) -> *mut fwnode_handle>,
    pub graph_get_remote_endpoint: Option<unsafe extern "C" fn(*const fwnode_handle) -> *mut fwnode_handle>,
    pub graph_get_port_parent: Option<unsafe extern "C" fn(*mut fwnode_handle) -> *mut fwnode_handle>,
    pub graph_parse_endpoint: Option<unsafe extern "C" fn(*const fwnode_handle, *mut fwnode_endpoint) -> ::core::ffi::c_int>,
    pub iomap: Option<unsafe extern "C" fn(*mut fwnode_handle, ::core::ffi::c_int) -> *mut ::core::ffi::c_void>,
    pub irq_get: Option<unsafe extern "C" fn(*const fwnode_handle, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub add_links: Option<unsafe extern "C" fn(*mut fwnode_handle) -> ::core::ffi::c_int>,
}

/* fwnode_has_op and the fwnode_call_* macros retain their C-side meaning. */

pub unsafe fn fwnode_init(fwnode: *mut fwnode_handle, ops: *const fwnode_operations) {
    (*fwnode).secondary = core::ptr::null_mut();
    (*fwnode).ops = ops;
    (*fwnode).dev = core::ptr::null_mut();
    /* INIT_LIST_HEAD(&fwnode->consumers); INIT_LIST_HEAD(&fwnode->suppliers); */
    (*fwnode).flags = 0;
}

pub unsafe fn fwnode_set_flag(fwnode: *mut fwnode_handle, bit: ::core::ffi::c_uint) {
    (*fwnode).flags |= (1 as ::core::ffi::c_ulong).wrapping_shl(bit);
}

pub unsafe fn fwnode_clear_flag(fwnode: *mut fwnode_handle, bit: ::core::ffi::c_uint) {
    (*fwnode).flags &= !(1 as ::core::ffi::c_ulong).wrapping_shl(bit);
}

pub unsafe fn fwnode_assign_flag(fwnode: *mut fwnode_handle, bit: ::core::ffi::c_uint, value: bool) {
    if value { fwnode_set_flag(fwnode, bit); } else { fwnode_clear_flag(fwnode, bit); }
}

pub unsafe fn fwnode_test_flag(fwnode: *mut fwnode_handle, bit: ::core::ffi::c_uint) -> bool {
    ((*fwnode).flags & (1 as ::core::ffi::c_ulong).wrapping_shl(bit)) != 0
}

pub unsafe fn fwnode_dev_initialized(fwnode: *mut fwnode_handle, initialized: bool) {
    if fwnode.is_null() { return; }
    fwnode_assign_flag(fwnode, FWNODE_FLAG_INITIALIZED as _, initialized);
}

extern "C" {
    pub fn fwnode_link_add(con: *mut fwnode_handle, sup: *mut fwnode_handle, flags: u8) -> ::core::ffi::c_int;
    pub fn fwnode_links_purge(fwnode: *mut fwnode_handle);
    pub fn fw_devlink_purge_absent_suppliers(fwnode: *mut fwnode_handle);
    pub fn fw_devlink_refresh_fwnode(fwnode: *mut fwnode_handle);
    pub fn fw_devlink_is_strict() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
