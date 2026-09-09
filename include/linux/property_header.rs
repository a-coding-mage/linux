/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/property.h. Included Linux dependencies are external. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub enum device {}
pub enum fwnode_handle {}
pub enum software_node {}
pub enum fwnode_endpoint {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dev_prop_type { DEV_PROP_U8, DEV_PROP_U16, DEV_PROP_U32, DEV_PROP_U64, DEV_PROP_STRING, DEV_PROP_REF }

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type size_t = usize;
pub type dma_attr = c_uint;

#[repr(C)]
pub struct fwnode_reference_args { _private: [u8; 0] }
pub const NR_FWNODE_REFERENCE_ARGS: usize = 16;

extern "C" {
    pub fn __dev_fwnode_const(dev: *const device) -> *const fwnode_handle;
    pub fn __dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    pub fn device_property_present(dev: *const device, propname: *const c_char) -> bool;
    pub fn device_property_read_bool(dev: *const device, propname: *const c_char) -> bool;
    pub fn device_property_read_u8_array(dev: *const device, propname: *const c_char, val: *mut u8, nval: size_t) -> c_int;
    pub fn device_property_read_u16_array(dev: *const device, propname: *const c_char, val: *mut u16, nval: size_t) -> c_int;
    pub fn device_property_read_u32_array(dev: *const device, propname: *const c_char, val: *mut u32, nval: size_t) -> c_int;
    pub fn device_property_read_u64_array(dev: *const device, propname: *const c_char, val: *mut u64, nval: size_t) -> c_int;
    pub fn device_property_read_string_array(dev: *const device, propname: *const c_char, val: *mut *const c_char, nval: size_t) -> c_int;
    pub fn device_property_read_string(dev: *const device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    pub fn device_property_match_string(dev: *const device, propname: *const c_char, string: *const c_char) -> c_int;
    pub fn fwnode_property_present(fwnode: *const fwnode_handle, propname: *const c_char) -> bool;
    pub fn fwnode_property_read_bool(fwnode: *const fwnode_handle, propname: *const c_char) -> bool;
    pub fn fwnode_property_read_u8_array(fwnode: *const fwnode_handle, propname: *const c_char, val: *mut u8, nval: size_t) -> c_int;
    pub fn fwnode_property_read_u16_array(fwnode: *const fwnode_handle, propname: *const c_char, val: *mut u16, nval: size_t) -> c_int;
    pub fn fwnode_property_read_u32_array(fwnode: *const fwnode_handle, propname: *const c_char, val: *mut u32, nval: size_t) -> c_int;
    pub fn fwnode_property_read_u64_array(fwnode: *const fwnode_handle, propname: *const c_char, val: *mut u64, nval: size_t) -> c_int;
    pub fn fwnode_property_read_string_array(fwnode: *const fwnode_handle, propname: *const c_char, val: *mut *const c_char, nval: size_t) -> c_int;
    pub fn fwnode_property_read_string(fwnode: *const fwnode_handle, propname: *const c_char, val: *mut *const c_char) -> c_int;
    pub fn fwnode_property_match_string(fwnode: *const fwnode_handle, propname: *const c_char, string: *const c_char) -> c_int;
    pub fn fwnode_device_is_available(fwnode: *const fwnode_handle) -> bool;
    pub fn fwnode_property_match_property_string(fwnode: *const fwnode_handle, propname: *const c_char, array: *const *const c_char, n: size_t) -> c_int;
    pub fn fwnode_property_get_reference_args(fwnode: *const fwnode_handle, prop: *const c_char, nargs_prop: *const c_char, nargs: c_uint, index: c_uint, args: *mut fwnode_reference_args) -> c_int;
    pub fn fwnode_find_reference(fwnode: *const fwnode_handle, name: *const c_char, index: c_uint) -> *mut fwnode_handle;
    pub fn fwnode_get_name(fwnode: *const fwnode_handle) -> *const c_char;
    pub fn fwnode_get_name_prefix(fwnode: *const fwnode_handle) -> *const c_char;
    pub fn fwnode_name_eq(fwnode: *const fwnode_handle, name: *const c_char) -> bool;
    pub fn fwnode_get_parent(fwnode: *const fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_get_next_parent(fwnode: *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_count_parents(fwn: *const fwnode_handle) -> c_uint;
    pub fn fwnode_get_nth_parent(fwn: *const fwnode_handle, depth: c_uint) -> *mut fwnode_handle;
    pub fn fwnode_get_next_child_node(fwnode: *const fwnode_handle, child: *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_get_next_available_child_node(fwnode: *const fwnode_handle, child: *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn device_get_next_child_node(dev: *const device, child: *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_get_named_child_node(fwnode: *const fwnode_handle, childname: *const c_char) -> *mut fwnode_handle;
    pub fn device_get_named_child_node(dev: *const device, childname: *const c_char) -> *mut fwnode_handle;
    pub fn fwnode_handle_get(fwnode: *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_irq_get(fwnode: *const fwnode_handle, index: c_uint) -> c_int;
    pub fn fwnode_irq_get_byname(fwnode: *const fwnode_handle, name: *const c_char) -> c_int;
    pub fn fwnode_get_child_node_count(fwnode: *const fwnode_handle) -> c_uint;
    pub fn fwnode_get_named_child_node_count(fwnode: *const fwnode_handle, name: *const c_char) -> c_uint;
}

#[inline]
pub unsafe fn fwnode_device_is_big_endian(fwnode: *const fwnode_handle) -> bool {
    // CONFIG_CPU_BIG_ENDIAN is a build-time condition supplied by the surrounding kernel.
    fwnode_property_present(fwnode, b"big-endian\0".as_ptr() as *const c_char) ||
        (cfg!(target_endian = "big") && fwnode_property_present(fwnode, b"native-endian\0".as_ptr() as *const c_char))
}
#[inline] pub unsafe fn fwnode_device_is_compatible(f: *const fwnode_handle, c: *const c_char) -> bool { fwnode_property_match_string(f, b"compatible\0".as_ptr() as *const c_char, c) >= 0 }
#[inline] pub unsafe fn device_is_big_endian(d: *const device) -> bool { fwnode_device_is_big_endian(__dev_fwnode_const(d)) }
#[inline] pub unsafe fn device_is_compatible(d: *const device, c: *const c_char) -> bool { fwnode_device_is_compatible(__dev_fwnode_const(d), c) }

#[repr(C)]
pub struct software_node_ref_args { pub swnode: *const software_node, pub fwnode: *mut fwnode_handle, pub nargs: c_uint, pub args: [u64; NR_FWNODE_REFERENCE_ARGS] }

#[repr(C)]
pub union property_value { pub pointer: *const c_void, pub value: property_value_data }
#[repr(C)]
pub union property_value_data { pub u8_data: [u8; 8], pub u16_data: [u16; 4], pub u32_data: [u32; 2], pub u64_data: [u64; 1], pub str_: [*const c_char; 1] }
#[repr(C)]
pub struct property_entry { pub name: *const c_char, pub length: size_t, pub is_inline: bool, pub type_: dev_prop_type, pub data: property_value }

#[repr(C)]
pub struct software_node_desc { pub name: *const c_char, pub parent: *const software_node, pub properties: *const property_entry }

pub const FWNODE_GRAPH_ENDPOINT_NEXT: c_ulong = 1;
pub const FWNODE_GRAPH_DEVICE_DISABLED: c_ulong = 2;

pub type devcon_match_fn_t = Option<unsafe extern "C" fn(*const fwnode_handle, *const c_char, *mut c_void) -> *mut c_void>;

/* Remaining declarations and macro-like initializers retain their C ABI names. */
extern "C" {
    pub fn property_entries_dup(properties: *const property_entry) -> *mut property_entry;
    pub fn property_entries_free(properties: *const property_entry);
    pub fn device_dma_supported(dev: *const device) -> bool;
    pub fn device_get_match_data(dev: *const device) -> *const c_void;
    pub fn device_get_phy_mode(dev: *mut device) -> c_int;
    pub fn fwnode_get_phy_mode(fwnode: *const fwnode_handle) -> c_int;
    pub fn fwnode_iomap(fwnode: *mut fwnode_handle, index: c_int) -> *mut c_void;
    pub fn fwnode_graph_get_next_endpoint(fwnode: *const fwnode_handle, prev: *mut fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_graph_get_port_parent(fwnode: *const fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_graph_get_remote_port_parent(fwnode: *const fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_graph_get_remote_port(fwnode: *const fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_graph_get_remote_endpoint(fwnode: *const fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_graph_get_endpoint_by_id(fwnode: *const fwnode_handle, port: u32, endpoint: u32, flags: c_ulong) -> *mut fwnode_handle;
    pub fn fwnode_graph_get_endpoint_count(fwnode: *const fwnode_handle, flags: c_ulong) -> c_uint;
    pub fn fwnode_graph_parse_endpoint(fwnode: *const fwnode_handle, endpoint: *mut fwnode_endpoint) -> c_int;
    pub fn fwnode_connection_find_match(fwnode: *const fwnode_handle, con_id: *const c_char, data: *mut c_void, match_: devcon_match_fn_t) -> *mut c_void;
    pub fn fwnode_connection_find_matches(fwnode: *const fwnode_handle, con_id: *const c_char, data: *mut c_void, match_: devcon_match_fn_t, matches: *mut *mut c_void, matches_len: c_uint) -> c_int;
    pub fn is_software_node(fwnode: *const fwnode_handle) -> bool;
    pub fn to_software_node(fwnode: *const fwnode_handle) -> *const software_node;
    pub fn software_node_fwnode(node: *const software_node) -> *mut fwnode_handle;
    pub fn software_node_find_by_name(parent: *const software_node, name: *const c_char) -> *const software_node;
    pub fn software_node_register_node_group(node_group: *const *const software_node) -> c_int;
    pub fn software_node_unregister_node_group(node_group: *const *const software_node);
    pub fn software_node_register(node: *const software_node) -> c_int;
    pub fn software_node_unregister(node: *const software_node);
    pub fn fwnode_create_software_node(properties: *const property_entry, parent: *const fwnode_handle) -> *mut fwnode_handle;
    pub fn fwnode_remove_software_node(fwnode: *mut fwnode_handle);
    pub fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int;
    pub fn device_remove_software_node(dev: *mut device);
    pub fn device_create_managed_software_node(dev: *mut device, properties: *const property_entry, parent: *const software_node) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
