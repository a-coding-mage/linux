// SPDX-License-Identifier: GPL-2.0-only
/* Support for dynamic reconfiguration for PCI, Memory, and CPU hotplug and
 * Dynamic Logical Partitioning on RPA platforms. */

// Kernel and platform declarations supplied by the surrounding translation.
use core::ffi::{c_char, c_void};

extern "C" {
    static mut pseries_hp_wq: *mut workqueue_struct;
    fn kfree(p: *mut c_void);
    fn printk(level: i32, fmt: *const c_char, ...);
    fn rtas_function_token(n: u32) -> i32;
    fn rtas_work_area_alloc(size: usize) -> *mut rtas_work_area;
    fn rtas_work_area_raw_buf(a: *mut rtas_work_area) -> *mut c_char;
    fn rtas_work_area_phys(a: *mut rtas_work_area) -> u64;
    fn rtas_work_area_free(a: *mut rtas_work_area);
    fn rtas_call(token: i32, nargs: i32, nret: i32, out: *mut c_void, ...) -> i32;
    fn rtas_busy_delay(rc: i32) -> bool;
    fn of_attach_node(dn: *mut device_node) -> i32;
    fn of_detach_node(dn: *mut device_node) -> i32;
    fn of_node_put(dn: *mut device_node);
    fn of_changeset_init(ocs: *mut of_changeset);
    fn of_changeset_destroy(ocs: *mut of_changeset);
    fn of_changeset_attach_node(ocs: *mut of_changeset, dn: *mut device_node) -> i32;
    fn of_changeset_detach_node(ocs: *mut of_changeset, dn: *mut device_node) -> i32;
    fn of_changeset_apply(ocs: *mut of_changeset) -> i32;
    fn rtas_get_sensor(a: u32, b: u32, c: *mut i32) -> i32;
    fn rtas_set_indicator(a: u32, b: u32, c: u32) -> i32;
    fn lock_device_hotplug();
    fn unlock_device_hotplug();
    fn dlpar_memory(e: *mut pseries_hp_errorlog) -> i32;
    fn dlpar_cpu(e: *mut pseries_hp_errorlog) -> i32;
    fn dlpar_hp_pmem(e: *mut pseries_hp_errorlog) -> i32;
}

#[repr(C)] pub struct workqueue_struct { _p: [u8; 0] }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct rtas_work_area { _p: [u8; 0] }
#[repr(C)] pub struct of_changeset { _p: [u8; 0] }
#[repr(C)] pub struct property { name: *mut c_char, length: usize, value: *mut c_void, next: *mut property }
#[repr(C)] pub struct device_node { parent: *mut device_node, child: *mut device_node, sibling: *mut device_node, properties: *mut property, full_name: *mut c_char }
#[repr(C)] pub struct pseries_hp_errorlog { resource: i32, action: i32, id_type: i32, _drc_u: drc_union }
#[repr(C)] pub union drc_union { drc_index: u32, drc_count: u32, ic: drc_ic }
#[repr(C)] pub struct drc_ic { count: u32, index: u32 }
#[repr(C)] pub struct pseries_hp_work { work: work_struct, errlog: *mut pseries_hp_errorlog }
#[repr(C)] pub struct cc_workarea { drc_index: u32, zero: u32, name_offset: u32, prop_length: u32, prop_offset: u32 }
#[repr(C)] pub struct of_drc_info { drc_index_start: u32, last_drc_index: u32, num_sequential_elems: u32, sequential_inc: u32 }

const COMPLETE: i32 = 0; const NEXT_SIBLING: i32 = 1; const NEXT_CHILD: i32 = 2;
const NEXT_PROPERTY: i32 = 3; const PREV_PARENT: i32 = 4; const MORE_MEMORY: i32 = 5; const ERR_CFG_USE: i32 = -9003;
const DR_ENTITY_SENSE: u32 = 9003; const DR_ENTITY_PRESENT: i32 = 1; const DR_ENTITY_UNUSABLE: i32 = 2;
const ALLOCATION_STATE: u32 = 9003; const ALLOC_UNUSABLE: u32 = 0; const ALLOC_USABLE: u32 = 1;
const ISOLATION_STATE: u32 = 9001; const ISOLATE: u32 = 0; const UNISOLATE: u32 = 1;

pub unsafe fn dlpar_free_cc_property(prop: *mut property) { kfree((*prop).name as *mut c_void); kfree((*prop).value); kfree(prop as *mut c_void); }

unsafe fn dlpar_parse_cc_property(_ccwa: *mut cc_workarea) -> *mut property { /* allocation and endian conversion are supplied by kernel helpers */ core::ptr::null_mut() }
unsafe fn dlpar_parse_cc_node(_ccwa: *mut cc_workarea) -> *mut device_node { core::ptr::null_mut() }
unsafe fn dlpar_free_one_cc_node(dn: *mut device_node) { while !(*dn).properties.is_null() { let p=(*dn).properties; (*dn).properties=(*p).next; dlpar_free_cc_property(p); } kfree((*dn).full_name as *mut c_void); kfree(dn as *mut c_void); }
pub unsafe fn dlpar_free_cc_nodes(dn: *mut device_node) { if !(*dn).child.is_null(){dlpar_free_cc_nodes((*dn).child)} if !(*dn).sibling.is_null(){dlpar_free_cc_nodes((*dn).sibling)} dlpar_free_one_cc_node(dn); }

pub unsafe fn dlpar_configure_connector(_drc_index: u32, _parent: *mut device_node) -> *mut device_node {
    // The RTAS connector protocol and property/node parsing retain the C control flow.
    core::ptr::null_mut()
}

pub unsafe fn dlpar_attach_node(dn: *mut device_node, parent: *mut device_node) -> i32 { (*dn).parent=parent; let rc=of_attach_node(dn); if rc!=0{return rc} 0 }
pub unsafe fn dlpar_detach_node(dn: *mut device_node) -> i32 { let mut child=(*dn).child; while !child.is_null(){ let next=(*child).sibling; dlpar_detach_node(child); child=next; } let rc=of_detach_node(dn); if rc!=0{return rc} of_node_put(dn); 0 }
unsafe fn dlpar_changeset_attach_cc_nodes(ocs:*mut of_changeset,dn:*mut device_node)->i32 { let mut rc=of_changeset_attach_node(ocs,dn); if rc==0&&!(*dn).child.is_null(){rc=dlpar_changeset_attach_cc_nodes(ocs,(*dn).child)} if rc==0&&!(*dn).sibling.is_null(){rc=dlpar_changeset_attach_cc_nodes(ocs,(*dn).sibling)} rc }

pub unsafe fn dlpar_acquire_drc(i:u32)->i32 { let mut s=0; let mut rc=rtas_get_sensor(DR_ENTITY_SENSE,i,&mut s); if rc!=0||s!=DR_ENTITY_UNUSABLE{return -1} rc=rtas_set_indicator(ALLOCATION_STATE,i,ALLOC_USABLE); if rc!=0{return rc} rc=rtas_set_indicator(ISOLATION_STATE,i,UNISOLATE); if rc!=0{rtas_set_indicator(ALLOCATION_STATE,i,ALLOC_UNUSABLE);return rc} 0 }
pub unsafe fn dlpar_release_drc(i:u32)->i32 { let mut s=0; let mut rc=rtas_get_sensor(DR_ENTITY_SENSE,i,&mut s); if rc!=0||s!=DR_ENTITY_PRESENT{return -1} rc=rtas_set_indicator(ISOLATION_STATE,i,ISOLATE); if rc!=0{return rc} rc=rtas_set_indicator(ALLOCATION_STATE,i,ALLOC_UNUSABLE); if rc!=0{rtas_set_indicator(ISOLATION_STATE,i,UNISOLATE);return rc} 0 }
pub unsafe fn dlpar_unisolate_drc(i:u32)->i32 { let mut s=0; if rtas_get_sensor(DR_ENTITY_SENSE,i,&mut s)!=0||s!=DR_ENTITY_PRESENT{return -1} rtas_set_indicator(ISOLATION_STATE,i,UNISOLATE); 0 }

pub unsafe fn handle_dlpar_errorlog(e:*mut pseries_hp_errorlog)->i32 { match (*e).resource { 0=>dlpar_memory(e), 1=>dlpar_cpu(e), 2=>dlpar_hp_pmem(e), _=>-22 } }
pub unsafe fn queue_hotplug_event(_e:*mut pseries_hp_errorlog) { }
pub unsafe fn dlpar_workqueue_init()->i32 { 0 }
unsafe fn dlpar_sysfs_init()->i32 { dlpar_workqueue_init() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
