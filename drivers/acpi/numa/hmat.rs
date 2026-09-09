// SPDX-License-Identifier: GPL-2.0
/* Heterogeneous Memory Attributes Table (HMAT) representation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// Kernel-provided types, constants, lists, allocation, ACPI, NUMA, and
// memory-tier interfaces are supplied by the surrounding translation unit.
extern "C" {
    static mut hmat_revision: u8;
    fn node_to_pxm(nid: i32) -> u32;
    fn pxm_to_node(pxm: u32) -> i32;
    fn node_state(nid: i32, state: i32) -> bool;
    fn node_online(nid: i32) -> bool;
    fn register_memory_node_under_compute_node(mem: i32, cpu: i32, access: i32);
    fn node_add_cache(nid: u32, attrs: *const node_cache_attrs);
    fn node_set_perf_attrs(nid: u32, coord: *const access_coordinate, access: i32);
    fn mt_set_default_dram_perf(nid: i32, attrs: *const access_coordinate, name: *const u8) -> i32;
    fn mt_perf_to_adistance(perf: *const access_coordinate, dist: *mut i32) -> bool;
    fn register_mt_adistance_algorithm(nb: *mut notifier_block);
    fn hotplug_node_notifier(cb: unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32, pri: i32) -> i32;
    fn acpi_get_table(sig: u32, instance: u32, table: *mut *mut acpi_table_header) -> i32;
    fn acpi_put_table(table: *mut acpi_table_header);
    fn acpi_table_parse_entries(sig: u32, size: usize, typ: u32, cb: unsafe extern "C" fn(*mut acpi_subtable_headers, usize) -> i32, arg: u32) -> i32;
    fn srat_disabled() -> bool;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct resource { pub name: *const u8, pub start: u64, pub end: u64, pub flags: u64, pub child: *mut resource, pub sibling: *mut resource }
#[repr(C)] #[derive(Copy, Clone)] pub struct access_coordinate { pub read_latency: u32, pub write_latency: u32, pub read_bandwidth: u32, pub write_bandwidth: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct node_cache_attrs { pub size: u64, pub level: u32, pub line_size: u32, pub indexing: u32, pub write_policy: u32, pub address_mode: u32 }
#[repr(C)] pub struct target_cache { pub node: list_head, pub cache_attrs: node_cache_attrs }
#[repr(C)] pub struct memory_target { pub node:list_head, pub memory_pxm:u32, pub processor_pxm:u32, pub memregions:resource, pub coord:[access_coordinate; 4], pub caches:list_head, pub cache_attrs:node_cache_attrs, pub gen_port_device_handle:[u8; 16], pub registered:bool }
#[repr(C)] pub struct memory_initiator { pub node:list_head, pub processor_pxm:u32, pub has_cpu:bool }
#[repr(C)] pub struct memory_locality { pub node:list_head, pub hmat_loc:*mut acpi_hmat_locality }
#[repr(C)] pub struct acpi_table_header { pub revision:u8, pub _pad:[u8; 31] }
#[repr(C)] pub struct acpi_subtable_headers { pub typ:u32, pub length:u32 }
#[repr(C)] pub struct acpi_hmat_structure { pub header:acpi_subtable_headers, pub typ:u32 }
#[repr(C)] pub struct acpi_hmat_locality { pub header:acpi_subtable_headers, pub data_type:u8, pub flags:u8, pub number_of_initiator_Pds:u32, pub number_of_target_Pds:u32, pub entry_base_unit:u64 }
#[repr(C)] pub struct acpi_hmat_cache { pub header:acpi_subtable_headers, pub memory_PD:u32, pub cache_size:u64, pub cache_attributes:u32, pub number_of_SMBIOShandles:u32, pub address_mode:u8 }
#[repr(C)] pub struct acpi_hmat_proximity_domain { pub header:acpi_subtable_headers, pub flags:u16, pub processor_PD:u32, pub memory_PD:u32, pub reserved3:u64, pub reserved4:u64 }
#[repr(C)] pub struct acpi_srat_mem_affinity { pub flags:u32, pub proximity_domain:u32, pub base_address:u64, pub length:u64 }
#[repr(C)] pub struct acpi_srat_generic_affinity { pub flags:u32, pub proximity_domain:u32, pub device_handle_type:u8, pub device_handle:[u8;16] }
#[repr(C)] pub struct notifier_block { pub notifier_call:Option<unsafe extern "C" fn(*mut notifier_block,usize,*mut c_void)->i32>, pub priority:i32 }
#[repr(C)] pub struct node_notify { pub nid:i32 }

static mut TARGETS: list_head = list_head { next:core::ptr::null_mut(), prev:core::ptr::null_mut() };
static mut INITIATORS: list_head = list_head { next:core::ptr::null_mut(), prev:core::ptr::null_mut() };
static mut LOCALITIES: list_head = list_head { next:core::ptr::null_mut(), prev:core::ptr::null_mut() };
static mut TARGET_LOCK: c_void = c_void;
static mut HMAT_DISABLE: i32 = 0;

pub const WRITE_LATENCY:usize=0; pub const READ_LATENCY:usize=1; pub const WRITE_BANDWIDTH:usize=2; pub const READ_BANDWIDTH:usize=3;
pub const ACCESS_COORDINATE_LOCAL:i32=0; pub const ACCESS_COORDINATE_CPU:i32=1;
pub const NODE_ACCESS_CLASS_GENPORT_SINK_LOCAL:i32=2; pub const NODE_ACCESS_CLASS_GENPORT_SINK_CPU:i32=3;
pub const PXM_INVAL:u32=!0; pub const NUMA_NO_NODE:i32=-1; pub const N_CPU:i32=0;
pub const ACPI_HMAT_MEMORY:u8=0; pub const ACPI_HMAT_ACCESS_LATENCY:u8=0; pub const ACPI_HMAT_READ_LATENCY:u8=1; pub const ACPI_HMAT_WRITE_LATENCY:u8=2; pub const ACPI_HMAT_ACCESS_BANDWIDTH:u8=3; pub const ACPI_HMAT_READ_BANDWIDTH:u8=4; pub const ACPI_HMAT_WRITE_BANDWIDTH:u8=5;

#[no_mangle] pub unsafe extern "C" fn disable_hmat(){ HMAT_DISABLE=1; }

unsafe fn find_mem_initiator(pxm:u32)->*mut memory_initiator { let mut p=INITIATORS.next; while !p.is_null(){ let x=p as *mut memory_initiator; if (*x).processor_pxm==pxm{return x} p=(*x).node.next;} core::ptr::null_mut() }
unsafe fn find_mem_target(pxm:u32)->*mut memory_target { let mut p=TARGETS.next; while !p.is_null(){ let x=p as *mut memory_target; if (*x).memory_pxm==pxm{return x} p=(*x).node.next;} core::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn hmat_get_extended_linear_cache_size(backing:*mut resource,nid:i32,out:*mut u64)->i32 { let t=find_mem_target(node_to_pxm(nid)); if t.is_null(){return -2} let mut p=(*t).caches.next; while !p.is_null(){let c=p as *mut target_cache; if (*c).cache_attrs.address_mode==1 && (*t).memregions.start<=(*backing).start && (*t).memregions.end>=(*backing).end {*out=(*c).cache_attrs.size;return 0} p=(*c).node.next;} *out=0;0 }

unsafe fn hmat_data_type(t:u8)->&'static [u8]{match t{0=>b"Access Latency",1=>b"Read Latency",2=>b"Write Latency",3=>b"Access Bandwidth",4=>b"Read Bandwidth",5=>b"Write Bandwidth",_=>b"Reserved"}}
unsafe fn hmat_data_type_suffix(t:u8)->&'static [u8]{match t{0|1|2=>b" nsec",3|4|5=>b" MB/s",_=>b""}}
unsafe fn hmat_normalize(entry:u16,base:u64,t:u8)->u32 { if entry==0xffff||entry==0||base>u32::MAX as u64/entry as u64{return 0} let mut v=(entry as u64*base) as u32; if hmat_revision==1{if v<10{return 0};v=(v+9)/10}else if hmat_revision==2 && t<=2{v=(v+999)/1000};v }
unsafe fn hmat_update_target_access(t:*mut memory_target,typ:u8,v:u32,a:usize){match typ{0=>{(*t).coord[a].read_latency=v;(*t).coord[a].write_latency=v},1=>(*t).coord[a].read_latency=v,2=>(*t).coord[a].write_latency=v,3=>{(*t).coord[a].read_bandwidth=v;(*t).coord[a].write_bandwidth=v},4=>(*t).coord[a].read_bandwidth=v,5=>(*t).coord[a].write_bandwidth=v,_=>{}}}

unsafe fn alloc_memory_initiator(pxm:u32){if pxm_to_node(pxm)==NUMA_NO_NODE||!find_mem_initiator(pxm).is_null(){return}}
unsafe fn alloc_target(pxm:u32)->*mut memory_target{find_mem_target(pxm)}
unsafe fn alloc_memory_target(_pxm:u32,_start:u64,_len:u64){}
unsafe fn alloc_genport_target(_pxm:u32,_handle:*mut u8){}

unsafe extern "C" fn hmat_parse_locality(_h:*mut acpi_subtable_headers,_end:usize)->i32 { 0 }
unsafe extern "C" fn hmat_parse_cache(_h:*mut acpi_subtable_headers,_end:usize)->i32 { 0 }
unsafe extern "C" fn hmat_parse_proximity_domain(_h:*mut acpi_subtable_headers,_end:usize)->i32 { 0 }
unsafe extern "C" fn hmat_parse_subtable(h:*mut acpi_subtable_headers,end:usize)->i32 { if h.is_null(){return -22} match (*(h as *mut acpi_hmat_structure)).typ {0=>hmat_parse_proximity_domain(h,end),1=>hmat_parse_locality(h,end),2=>hmat_parse_cache(h,end),_=>-22} }
unsafe extern "C" fn srat_parse_mem_affinity(h:*mut acpi_subtable_headers,_end:usize)->i32 { if h.is_null(){return -22}; let m=h as *mut acpi_srat_mem_affinity; if (*m).flags!=0{alloc_memory_target((*m).proximity_domain,(*m).base_address,(*m).length)};0 }
unsafe extern "C" fn srat_parse_genport_affinity(h:*mut acpi_subtable_headers,_end:usize)->i32 { if h.is_null(){return -22}; let g=h as *mut acpi_srat_generic_affinity; if (*g).flags!=0&&(*g).device_handle_type==0{alloc_genport_target((*g).proximity_domain,(*g).device_handle.as_mut_ptr())};0 }

unsafe fn hmat_update_best(typ:u8,v:u32,best:&mut u32)->bool{if v==0{return false};let better=if typ<=2{*best==0||*best>v}else{*best==0||*best<v};if better{*best=v;true}else{false}}
unsafe fn hmat_update_target_attrs(_t:*mut memory_target,_nodes:*mut usize,_access:i32){}
unsafe fn hmat_register_target(t:*mut memory_target){if t.is_null(){return}}
unsafe fn hmat_register_targets(){}
unsafe extern "C" fn hmat_callback(_s:*mut notifier_block,_a:usize,_arg:*mut c_void)->i32{1}
unsafe fn hmat_free_structures(){}

unsafe extern "C" fn hmat_init()->i32 { if srat_disabled()||HMAT_DISABLE!=0{return 0};0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
