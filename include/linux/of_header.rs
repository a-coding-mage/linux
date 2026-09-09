/* SPDX-License-Identifier: GPL-2.0+ */
// Translation of linux/of.h. Included kernel types and operations are external dependencies.

pub type phandle = u32;
pub type ihandle = u32;

#[repr(C)]
pub struct property {
    pub name: *mut core::ffi::c_char,
    pub length: i32,
    pub value: *mut core::ffi::c_void,
    pub next: *mut property,
    #[cfg(any(CONFIG_OF_DYNAMIC, CONFIG_SPARC))] pub _flags: usize,
    #[cfg(CONFIG_OF_PROMTREE)] pub unique_id: u32,
    #[cfg(CONFIG_OF_KOBJ)] pub attr: bin_attribute,
}

#[repr(C)] pub struct device_node {
    pub name: *const core::ffi::c_char,
    pub phandle: phandle,
    pub full_name: *const core::ffi::c_char,
    pub fwnode: fwnode_handle,
    pub properties: *mut property,
    pub deadprops: *mut property,
    pub parent: *mut device_node,
    pub child: *mut device_node,
    pub sibling: *mut device_node,
    #[cfg(CONFIG_OF_KOBJ)] pub kobj: kobject,
    pub _flags: usize,
    pub data: *mut core::ffi::c_void,
    #[cfg(CONFIG_SPARC)] pub unique_id: u32,
    #[cfg(CONFIG_SPARC)] pub irq_trans: *mut of_irq_controller,
}

pub const MAX_PHANDLE_ARGS: usize = NR_FWNODE_REFERENCE_ARGS as usize;
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args_count: i32, pub args: [u32; MAX_PHANDLE_ARGS] }
#[repr(C)] pub struct of_phandle_iterator {
    pub cells_name: *const core::ffi::c_char, pub cell_count: i32, pub parent: *const device_node,
    pub list_end: *const __be32, pub phandle_end: *const __be32, pub cur: *const __be32,
    pub cur_count: u32, pub phandle: phandle, pub node: *mut device_node,
}
#[repr(C)] pub struct of_reconfig_data { pub dn: *mut device_node, pub prop: *mut property, pub old_prop: *mut property }

// External kernel types.
pub type __be32 = u32;
pub type phys_addr_t = u64;
pub type ssize_t = isize;
pub enum fwnode_handle {}
pub enum bin_attribute {}
pub enum kobject {}
pub enum of_irq_controller {}
pub enum of_device_id {}
pub enum device {}
pub enum notifier_block {}
pub enum list_head {}
pub const NR_FWNODE_REFERENCE_ARGS: u32 = 16;
pub const PHYS_ADDR_MAX: phys_addr_t = phys_addr_t::MAX;
pub const NUMA_NO_NODE: i32 = -1;

#[cfg(CONFIG_OF_KOBJ)] pub const fn of_node_kobj(n: *mut device_node) -> *mut kobject { unsafe { &mut (*n).kobj } }
#[cfg(not(CONFIG_OF_KOBJ))] pub const fn of_node_kobj(_n: *mut device_node) -> *mut kobject { core::ptr::null_mut() }

pub const OF_DYNAMIC: usize = 1;
pub const OF_DETACHED: usize = 2;
pub const OF_POPULATED: usize = 3;
pub const OF_POPULATED_BUS: usize = 4;
pub const OF_OVERLAY: usize = 5;
pub const OF_OVERLAY_FREE_CSET: usize = 6;
pub const OF_BAD_ADDR: u64 = u64::MAX;
pub const OF_RECONFIG_ATTACH_NODE: u32 = 0x0001;
pub const OF_RECONFIG_DETACH_NODE: u32 = 0x0002;
pub const OF_RECONFIG_ADD_PROPERTY: u32 = 0x0003;
pub const OF_RECONFIG_REMOVE_PROPERTY: u32 = 0x0004;
pub const OF_RECONFIG_UPDATE_PROPERTY: u32 = 0x0005;

#[cfg(CONFIG_OF)]
extern "C" {
    pub static of_node_ktype: kobject;
    pub static of_fwnode_ops: fwnode_operations;
    pub static mut of_root: *mut device_node;
    pub static mut of_chosen: *mut device_node;
    pub static mut of_aliases: *mut device_node;
    pub static mut of_stdout: *mut device_node;
}
pub enum fwnode_operations {}

#[inline] pub unsafe fn of_node_init(node: *mut device_node) { fwnode_init(&mut (*node).fwnode, &of_fwnode_ops); }
extern "C" { fn fwnode_init(f: *mut fwnode_handle, ops: *const fwnode_operations); }

#[cfg(CONFIG_OF_DYNAMIC)] extern "C" { pub fn of_node_get(n: *mut device_node) -> *mut device_node; pub fn of_node_put(n: *mut device_node); }
#[cfg(not(CONFIG_OF_DYNAMIC))] #[inline] pub unsafe fn of_node_get(n: *mut device_node) -> *mut device_node { n }
#[cfg(not(CONFIG_OF_DYNAMIC))] #[inline] pub unsafe fn of_node_put(_n: *mut device_node) {}

#[cfg(CONFIG_OF)]
extern "C" {
    pub fn of_core_init();
    pub fn of_find_node_by_name(from: *mut device_node, name: *const i8) -> *mut device_node;
    pub fn of_find_node_by_type(from: *mut device_node, typ: *const i8) -> *mut device_node;
    pub fn of_find_compatible_node(from: *mut device_node, typ: *const i8, compat: *const i8) -> *mut device_node;
    pub fn of_find_matching_node_and_match(from: *mut device_node, matches: *const of_device_id, m: *mut *const of_device_id) -> *mut device_node;
    pub fn of_find_node_opts_by_path(path: *const i8, opts: *mut *const i8) -> *mut device_node;
    pub fn of_find_node_by_phandle(h: phandle) -> *mut device_node;
    pub fn of_get_parent(n: *const device_node) -> *mut device_node;
    pub fn of_get_next_parent(n: *mut device_node) -> *mut device_node;
    pub fn of_get_next_child(n: *const device_node, prev: *mut device_node) -> *mut device_node;
    pub fn of_get_next_available_child(n: *const device_node, prev: *mut device_node) -> *mut device_node;
    pub fn of_find_node_with_property(from: *mut device_node, name: *const i8) -> *mut device_node;
    pub fn of_find_property(n: *const device_node, name: *const i8, len: *mut i32) -> *mut property;
    pub fn of_property_read_bool(n: *const device_node, name: *const i8) -> bool;
    pub fn of_property_count_elems_of_size(n: *const device_node, name: *const i8, size: i32) -> i32;
    pub fn of_property_read_u8_index(n: *const device_node, name: *const i8, index: u32, out: *mut u8) -> i32;
    pub fn of_property_read_u16_index(n: *const device_node, name: *const i8, index: u32, out: *mut u16) -> i32;
    pub fn of_property_read_u32_index(n: *const device_node, name: *const i8, index: u32, out: *mut u32) -> i32;
    pub fn of_property_read_u64_index(n: *const device_node, name: *const i8, index: u32, out: *mut u64) -> i32;
    pub fn of_property_read_variable_u8_array(n: *const device_node, name: *const i8, out: *mut u8, min: usize, max: usize) -> i32;
    pub fn of_property_read_variable_u16_array(n: *const device_node, name: *const i8, out: *mut u16, min: usize, max: usize) -> i32;
    pub fn of_property_read_variable_u32_array(n: *const device_node, name: *const i8, out: *mut u32, min: usize, max: usize) -> i32;
    pub fn of_property_read_variable_u64_array(n: *const device_node, name: *const i8, out: *mut u64, min: usize, max: usize) -> i32;
    pub fn of_property_read_string(n: *const device_node, name: *const i8, out: *mut *const i8) -> i32;
    pub fn of_property_read_string_helper(n: *const device_node, name: *const i8, out: *mut *const i8, sz: usize, index: i32) -> i32;
    pub fn of_get_property(n: *const device_node, name: *const i8, len: *mut i32) -> *const core::ffi::c_void;
    pub fn __of_parse_phandle_with_args(n: *const device_node, list: *const i8, cells: *const i8, count: i32, index: i32, out: *mut of_phandle_args) -> i32;
    pub fn of_phandle_iterator_init(it: *mut of_phandle_iterator, n: *const device_node, list: *const i8, cells: *const i8, count: i32) -> i32;
    pub fn of_phandle_iterator_next(it: *mut of_phandle_iterator) -> i32;
    pub fn of_phandle_iterator_args(it: *mut of_phandle_iterator, args: *mut u32, size: i32) -> i32;
    pub fn of_node_name_eq(n: *const device_node, name: *const i8) -> bool;
    pub fn of_node_name_prefix(n: *const device_node, prefix: *const i8) -> bool;
    pub fn of_n_addr_cells(n: *mut device_node) -> i32;
    pub fn of_n_size_cells(n: *mut device_node) -> i32;
    pub fn of_alias_get_id(n: *const device_node, stem: *const i8) -> i32;
    pub fn of_alias_get_highest_id(stem: *const i8) -> i32;
    pub fn of_add_property(n: *mut device_node, p: *mut property) -> i32;
    pub fn of_remove_property(n: *mut device_node, p: *mut property) -> i32;
    pub fn of_machine_compatible_match(c: *const *const i8) -> bool;
    pub fn of_machine_get_match(m: *const of_device_id) -> *const of_device_id;
    pub fn of_machine_get_match_data(m: *const of_device_id) -> *const core::ffi::c_void;
}

#[inline] pub unsafe fn of_find_node_by_path(path: *const i8) -> *mut device_node { of_find_node_opts_by_path(path, core::ptr::null_mut()) }
#[inline] pub unsafe fn of_node_is_root(n: *const device_node) -> bool { !n.is_null() && (*n).parent.is_null() }
#[inline] pub unsafe fn of_node_full_name(n: *const device_node) -> *const i8 { if n.is_null() { b"<no-node>\0".as_ptr() as *const i8 } else { (*n).full_name } }
#[inline] pub unsafe fn of_machine_is_compatible(c: *const i8) -> bool { let a = [c, core::ptr::null()]; of_machine_compatible_match(a.as_ptr()) }
#[inline] pub unsafe fn of_parse_phandle(n: *const device_node, name: *const i8, index: i32) -> *mut device_node { let mut a = core::mem::MaybeUninit::<of_phandle_args>::uninit(); if __of_parse_phandle_with_args(n,name,core::ptr::null(),0,index,a.as_mut_ptr()) != 0 { core::ptr::null_mut() } else { a.assume_init().np } }
#[inline] pub unsafe fn of_property_present(n: *const device_node, name: *const i8) -> bool { !of_find_property(n,name,core::ptr::null_mut()).is_null() }

#[repr(C)] pub struct of_changeset_entry { pub node: list_head, pub action: usize, pub np: *mut device_node, pub prop: *mut property, pub old_prop: *mut property }
#[repr(C)] pub struct of_changeset { pub entries: list_head }
pub enum of_reconfig_change { OF_RECONFIG_NO_CHANGE = 0, OF_RECONFIG_CHANGE_ADD, OF_RECONFIG_CHANGE_REMOVE }
pub enum of_overlay_notify_action { OF_OVERLAY_INIT = 0, OF_OVERLAY_PRE_APPLY, OF_OVERLAY_POST_APPLY, OF_OVERLAY_PRE_REMOVE, OF_OVERLAY_POST_REMOVE }
#[repr(C)] pub struct of_overlay_notify_data { pub overlay: *mut device_node, pub target: *mut device_node }

#[cfg(not(CONFIG_OF))]
#[inline] pub unsafe fn of_core_init() {}
#[cfg(not(CONFIG_OF))]
#[inline] pub unsafe fn is_of_node(_f: *const fwnode_handle) -> bool { false }
#[cfg(not(CONFIG_OF))]
#[inline] pub unsafe fn of_node_full_name(_n: *const device_node) -> *const i8 { b"<no-node>\0".as_ptr() as *const i8 }

// C iteration and declaration macros are represented by their source-level intent;
// callers may use the corresponding Rust loops over these extern functions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
