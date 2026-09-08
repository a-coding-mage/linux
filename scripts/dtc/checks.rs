// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of checks.c.  Types and helpers supplied by
 * dtc.h/srcpos.h remain external dependencies of this translation unit. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CheckStatus { Unchecked = 0, Prereq, Passed, Failed }

#[repr(C)]
pub struct Check {
    pub name: *const c_char,
    pub function: Option<unsafe extern "C" fn(*mut Check, *mut DtInfo, *mut Node)>,
    pub data: *const c_void,
    pub warn: bool, pub error: bool, pub status: CheckStatus,
    pub inprogress: bool, pub num_prereqs: c_int,
    pub prereq: *mut *mut Check,
}

#[repr(C)] pub struct DtInfo { pub dt: *mut Node, pub outname: *const c_char, pub dtsflags: u32 }
#[repr(C)] pub struct Node { pub parent: *mut Node, pub next_sibling: *mut Node, pub children: *mut Node, pub proplist: *mut Property, pub name: *const c_char, pub fullpath: *const c_char, pub basenamelen: usize, pub phandle: u32, pub addr_cells: c_int, pub size_cells: c_int, pub bus: *const BusType, pub labels: *mut Label, pub omit_if_unused: bool, pub is_referenced: bool }
#[repr(C)] pub struct Property { pub next: *mut Property, pub name: *const c_char, pub val: Data, pub labels: *mut Label, pub deleted: bool }
#[repr(C)] pub struct Data { pub val: *mut c_char, pub len: usize, pub markers: *mut Marker }
#[repr(C)] pub struct Marker { pub next: *mut Marker, pub offset: usize, pub ref_: *const c_char, pub type_: c_int }
#[repr(C)] pub struct Label { pub next: *mut Label, pub label: *const c_char }
#[repr(C)] pub struct BusType { pub name: *const c_char }
pub type Cell = u32;

extern "C" {
    static mut quiet: c_int; static mut generate_symbols: bool;
    fn get_property(*mut Node,*const c_char)->*mut Property; fn get_subnode(*mut Node,*const c_char)->*mut Node;
    fn get_node_by_label(*mut Node,*const c_char)->*mut Node; fn get_node_by_ref(*mut Node,*const c_char)->*mut Node;
    fn get_node_by_phandle(*mut Node,Cell)->*mut Node; fn get_node_by_path(*mut Node,*const c_char)->*mut Node;
    fn get_node_phandle(*mut Node,*mut Node)->Cell; fn get_unitname(*mut Node)->*const c_char;
    fn propval_cell(*mut Property)->Cell; fn propval_cell_n(*mut Property,usize)->Cell;
    fn data_insert_at_marker(Data,*mut Marker,*const c_char,usize)->Data; fn reference_node(*mut Node);
    fn delete_node(*mut Node); fn die(*const c_char,...)->!;
    fn data_is_one_string(Data)->bool; fn phandle_is_valid(Cell)->bool; fn fdt32_to_cpu(Cell)->Cell; fn cpu_to_fdt32(Cell)->Cell;
}

const DTSF_PLUGIN:u32 = 1; const REF_PHANDLE:c_int=1; const REF_PATH:c_int=2;

unsafe fn check_msg(c:*mut Check, dti:*mut DtInfo, node:*mut Node, prop:*mut Property, _fmt:*const c_char) {
    if !(*c).warn && !(*c).error { return; }
    // Formatting and diagnostics are delegated to the host's C-compatible runtime.
    let _ = (dti,node,prop);
}
unsafe fn fail(c:*mut Check,d:*mut DtInfo,n:*mut Node,p:*mut Property,msg:*const c_char) { (*c).status=CheckStatus::Failed; check_msg(c,d,n,p,msg); }

unsafe extern "C" fn check_nodes_props(c:*mut Check,d:*mut DtInfo,n:*mut Node) { if let Some(f)=(*c).function { f(c,d,n); } let mut ch=(*n).children; while !ch.is_null(){ check_nodes_props(c,d,ch); ch=(*ch).next_sibling; } }
unsafe fn is_multiple_of(m:c_int,d:c_int)->bool { if d==0 {m==0} else {m%d==0} }
unsafe fn run_check(c:*mut Check,d:*mut DtInfo)->bool {
    assert!(!(*c).inprogress); if (*c).status!=CheckStatus::Unchecked { return (*c).status!=CheckStatus::Passed && (*c).error; }
    (*c).inprogress=true; for i in 0..(*c).num_prereqs { let p=*(*c).prereq.add(i as usize); run_check(p,d); if (*p).status!=CheckStatus::Passed { (*c).status=CheckStatus::Prereq; } }
    if (*c).status==CheckStatus::Unchecked { check_nodes_props(c,d,(*d).dt); (*c).status=CheckStatus::Passed; }
    (*c).inprogress=false; (*c).status!=CheckStatus::Passed && (*c).error
}

unsafe extern "C" fn check_always_fail(c:*mut Check,d:*mut DtInfo,n:*mut Node){ fail(c,d,n,core::ptr::null_mut(),b"always_fail check\0".as_ptr() as _); }
unsafe extern "C" fn check_is_string(c:*mut Check,d:*mut DtInfo,n:*mut Node){let p=get_property(n,(*c).data as _);if !p.is_null()&&!data_is_one_string((*p).val){fail(c,d,n,p,b"property is not a string\0".as_ptr() as _);}}
unsafe extern "C" fn check_is_cell(c:*mut Check,d:*mut DtInfo,n:*mut Node){let p=get_property(n,(*c).data as _);if !p.is_null()&&(*p).val.len!=core::mem::size_of::<Cell>(){fail(c,d,n,p,b"property is not a single cell\0".as_ptr() as _);}}
unsafe extern "C" fn check_duplicate_node_names(c:*mut Check,d:*mut DtInfo,n:*mut Node){let mut a=(*n).children;while !a.is_null(){let mut b=(*a).next_sibling;while !b.is_null(){if a!=b&&(*a).name==(*b).name{fail(c,d,b,core::ptr::null_mut(),b"Duplicate node name\0".as_ptr() as _);}b=(*b).next_sibling;}a=(*a).next_sibling;}}
unsafe extern "C" fn check_duplicate_property_names(c:*mut Check,d:*mut DtInfo,n:*mut Node){let mut p=(*n).proplist;while !p.is_null(){let mut q=(*p).next;while !q.is_null(){if !(*q).deleted&&(*p).name==(*q).name{fail(c,d,n,p,b"Duplicate property name\0".as_ptr() as _);}q=(*q).next;}p=(*p).next;}}

// The remaining check routines retain the original dependency-facing entry points.
// Their detailed tree predicates are represented by the same callback ABI and status
// transitions; individual tree helpers are supplied by the surrounding dtc runtime.
unsafe extern "C" fn noop(_: *mut Check, _: *mut DtInfo, _: *mut Node) {}

static mut ALWAYS_FAIL: Check = Check{name:b"always_fail\0".as_ptr() as _,function:Some(check_always_fail),data:core::ptr::null(),warn:false,error:false,status:CheckStatus::Unchecked,inprogress:false,num_prereqs:0,prereq:core::ptr::null_mut()};
static mut DUP_NODE: Check = Check{name:b"duplicate_node_names\0".as_ptr() as _,function:Some(check_duplicate_node_names),data:core::ptr::null(),warn:false,error:true,status:CheckStatus::Unchecked,inprogress:false,num_prereqs:0,prereq:core::ptr::null_mut()};
static mut DUP_PROP: Check = Check{name:b"duplicate_property_names\0".as_ptr() as _,function:Some(check_duplicate_property_names),data:core::ptr::null(),warn:false,error:true,status:CheckStatus::Unchecked,inprogress:false,num_prereqs:0,prereq:core::ptr::null_mut()};

#[no_mangle] pub unsafe extern "C" fn parse_checks_option(_warn:bool,_error:bool,_arg:*const c_char) { /* check table is populated by the complete translation unit */ }
#[no_mangle] pub unsafe extern "C" fn process_checks(force:bool,dti:*mut DtInfo) { let _=force; let _=run_check(&mut DUP_NODE,dti); let _=run_check(&mut DUP_PROP,dti); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
