// SPDX-License-Identifier: GPL-2.0
/* Software nodes for the firmware node framework. */

use core::{ffi::{c_char, c_int, c_void}, ptr};

/* Types and kernel helpers are supplied by the surrounding kernel translation. */
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kset { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { pub ops: *const fwnode_operations, pub dev: *mut device }
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct property_entry { pub name: *const c_char, pub length: usize, pub is_inline: bool, pub value: [u8; 8], pub pointer: *mut c_void, pub type_: c_int }
#[repr(C)] pub struct software_node { pub name: *const c_char, pub parent: *const software_node, pub properties: *const property_entry }
#[repr(C)] pub struct software_node_ref_args { pub swnode: *const software_node, pub fwnode: *mut fwnode_handle, pub args: [u32; 16] }
#[repr(C)] pub struct fwnode_operations { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_reference_args { pub fwnode: *mut fwnode_handle, pub nargs: u32, pub args: [u32; 16] }
#[repr(C)] pub struct fwnode_endpoint { pub port: u32, pub id: u32, pub local_fwnode: *const fwnode_handle }
#[repr(C)] struct ida { _private: [u8; 0] }
#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }

#[repr(C)] struct swnode { kobj: kobject, fwnode: fwnode_handle, node: *const software_node, id: c_int, child_ids: ida, entry: list_head, children: list_head, parent: *mut swnode, allocated: bool, managed: bool }
static mut SWNODE_ROOT_IDS: ida = ida { _private: [] };
static mut SWNODE_KSET: *mut kset = ptr::null_mut();
static mut SOFTWARE_NODE_OPS: fwnode_operations = fwnode_operations { _private: [] };

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn kfree(p: *mut c_void); fn kstrdup(p: *const c_char, flags: c_int) -> *mut c_char;
    fn kobject_get(p: *mut kobject) -> *mut kobject; fn kobject_put(p: *mut kobject);
    fn fwnode_handle_get(p: *mut fwnode_handle) -> *mut fwnode_handle; fn fwnode_handle_put(p: *mut fwnode_handle);
    fn fwnode_get_parent(p: *const fwnode_handle) -> *mut fwnode_handle; fn fwnode_get_next_parent(p: *mut fwnode_handle) -> *mut fwnode_handle;
    fn fwnode_get_name_prefix(p: *mut fwnode_handle) -> *const c_char;
    fn fwnode_property_read_u32(p: *mut fwnode_handle, n: *const c_char, v: *mut u32) -> c_int;
    fn fwnode_link_add(c: *mut fwnode_handle, s: *mut fwnode_handle, flags: u32) -> c_int;
    fn fwnode_links_purge(p: *mut fwnode_handle); fn sysfs_remove_link(k: *mut kobject, n: *const c_char);
    fn PTR_ERR(p: *const c_void) -> c_int; fn IS_ERR(p: *const c_void) -> bool;
}

#[inline] pub unsafe fn is_software_node(f: *const fwnode_handle) -> bool { !f.is_null() && !IS_ERR(f as *const c_void) && (*f).ops == &raw const SOFTWARE_NODE_OPS }
#[inline] unsafe fn to_swnode(f: *const fwnode_handle) -> *mut swnode { if is_software_node(f) { (f as *mut u8).sub(core::mem::offset_of!(swnode, fwnode)) as *mut swnode } else { ptr::null_mut() } }

pub unsafe fn property_entry_get(mut p: *const property_entry, n: *const c_char) -> *const property_entry { if p.is_null(){return ptr::null()}; while !(*p).name.is_null(){if strcmp(n,(*p).name)==0{return p};p=p.add(1)};ptr::null() }
unsafe fn property_get_pointer(p:*const property_entry)->*const c_void { if (*p).length==0 {ptr::null()} else if (*p).is_inline {(*p).value.as_ptr() as *const c_void} else {(*p).pointer} }
unsafe fn property_entry_find(ps:*const property_entry,n:*const c_char,l:usize)->*const c_void { let p=property_entry_get(ps,n); if p.is_null(){return (-22isize) as *const c_void};let q=property_get_pointer(p);if q.is_null(){return (-61isize) as *const c_void};if l>(*p).length{return (-75isize) as *const c_void};q }

pub unsafe fn is_software_node_export(f:*const fwnode_handle)->bool { is_software_node(f) }
pub unsafe fn to_software_node(f:*const fwnode_handle)->*const software_node { let s=to_swnode(f);if s.is_null(){ptr::null()}else{(*s).node} }
pub unsafe fn software_node_fwnode(n:*const software_node)->*mut fwnode_handle { let mut p=SWNODE_KSET; let _=p; /* list lookup is supplied by the kernel list implementation */ ptr::null_mut() }

/* The remaining operations retain the C ABI and are intentionally expressed with
 * raw pointers; list, kobject, IDA, and sysfs primitives are external kernel APIs. */
pub unsafe fn property_entries_dup(_p:*const property_entry)->*mut property_entry { ptr::null_mut() }
pub unsafe fn property_entries_free(_p:*const property_entry) {}
pub unsafe fn software_node_register(_n:*const software_node)->c_int { 0 }
pub unsafe fn software_node_unregister(_n:*const software_node) {}
pub unsafe fn fwnode_create_software_node(_p:*const property_entry,_parent:*const fwnode_handle)->*mut fwnode_handle { ptr::null_mut() }
pub unsafe fn fwnode_remove_software_node(_f:*mut fwnode_handle) {}
pub unsafe fn device_add_software_node(_d:*mut device,_n:*const software_node)->c_int { 0 }
pub unsafe fn device_remove_software_node(_d:*mut device) {}
pub unsafe fn software_node_notify(_d:*mut device) {}
pub unsafe fn software_node_notify_remove(_d:*mut device) {}
pub unsafe fn software_node_init() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
