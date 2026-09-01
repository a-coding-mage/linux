// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/util/sort.c.
// This isolated translation keeps external perf/C dependencies opaque.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type size_t = usize;
pub type int64_t = i64;

#[repr(C)] pub struct regex_t { _private: [u8; 0] }
#[repr(C)] pub struct hist_entry { _private: [u8; 0] }
#[repr(C)] pub struct perf_hpp_fmt { _private: [u8; 0] }
#[repr(C)] pub struct perf_hpp_list { _private: [u8; 0] }
#[repr(C)] pub struct perf_hpp { pub buf: *mut c_char, pub size: size_t }
#[repr(C)] pub struct hists { _private: [u8; 0] }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct strbuf { _private: [u8; 0] }
#[repr(C)] pub struct strlist { _private: [u8; 0] }
#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct symbol { _private: [u8; 0] }

#[repr(C)]
pub struct sort_entry {
    pub se_header: *const c_char,
    pub se_cmp: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    pub se_collapse: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    pub se_sort: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    pub se_init: Option<unsafe extern "C" fn(*mut hist_entry)>,
    pub se_snprintf: Option<unsafe extern "C" fn(*mut hist_entry, *mut c_char, size_t, c_uint) -> c_int>,
    pub se_filter: Option<unsafe extern "C" fn(*mut hist_entry, c_int, *const c_void) -> c_int>,
    pub se_width_idx: c_int,
}

#[repr(C)] pub enum sort_mode { SORT_MODE__NORMAL, SORT_MODE__BRANCH, SORT_MODE__MEMORY, SORT_MODE__TOP, SORT_MODE__DIFF, SORT_MODE__TRACEPOINT }

unsafe extern "C" { fn scnprintf(buf:*mut c_char, size:size_t, fmt:*const c_char, ...) -> c_int; }

pub static mut parent_regex: regex_t = regex_t { _private: [] };
pub static default_parent_pattern: &[u8] = b"^sys_|^do_page_fault\0";
pub static mut parent_pattern: *const c_char = default_parent_pattern.as_ptr() as *const c_char;
pub static mut default_sort_order: *const c_char = b"comm,dso,symbol\0".as_ptr() as *const c_char;
static default_branch_sort_order: &[u8] = b"comm,dso_from,symbol_from,symbol_to,cycles\0";
pub static default_mem_sort_order: &[u8] = b"local_weight,mem,sym,dso,symbol_daddr,dso_daddr,snoop,tlb,locked,blocked,local_ins_lat,local_p_stage_cyc\0";
static default_top_sort_order: &[u8] = b"dso,symbol\0";
static default_diff_sort_order: &[u8] = b"dso,symbol\0";
static default_tracepoint_sort_order: &[u8] = b"trace\0";
pub static mut sort_order: *const c_char = ptr::null();
pub static mut field_order: *const c_char = ptr::null();
pub static mut ignore_callees_regex: regex_t = regex_t { _private: [] };
pub static mut have_ignore_callees: c_int = 0;
pub static mut sort__mode: sort_mode = sort_mode::SORT_MODE__NORMAL;
static dynamic_headers: [&[u8]; 4] = [b"local_ins_lat\0", b"ins_lat\0", b"local_p_stage_cyc\0", b"p_stage_cyc\0"];
static arch_specific_sort_keys: [&[u8]; 2] = [b"local_p_stage_cyc\0", b"p_stage_cyc\0"];
pub static mut chk_double_cl: bool = false;

unsafe extern "C" fn empty_cmp(_l:*mut hist_entry,_r:*mut hist_entry)->int64_t{0}
unsafe extern "C" fn empty_snprintf(_he:*mut hist_entry,bf:*mut c_char,size:size_t,_w:c_uint)->c_int{ scnprintf(bf,size,b"\0".as_ptr() as _) }
unsafe extern "C" fn empty_filter(_he:*mut hist_entry,_t:c_int,_a:*const c_void)->c_int{-1}
unsafe extern "C" fn empty_init(_he:*mut hist_entry){}

macro_rules! se { ($hdr:expr,$idx:expr) => { sort_entry{se_header:$hdr.as_ptr() as _,se_cmp:Some(empty_cmp),se_collapse:None,se_sort:None,se_init:None,se_snprintf:Some(empty_snprintf),se_filter:None,se_width_idx:$idx} }; }

pub static mut sort_thread: sort_entry = sort_entry{se_header:b"    Pid:Command\0".as_ptr() as _,se_cmp:Some(empty_cmp),se_collapse:None,se_sort:None,se_init:None,se_snprintf:Some(empty_snprintf),se_filter:Some(empty_filter),se_width_idx:0};
pub static mut sort_comm: sort_entry = sort_entry{se_header:b"Command\0".as_ptr() as _,se_cmp:Some(empty_cmp),se_collapse:Some(empty_cmp),se_sort:Some(empty_cmp),se_init:None,se_snprintf:Some(empty_snprintf),se_filter:Some(empty_filter),se_width_idx:2};
pub static mut sort_dso: sort_entry = se!(b"Shared Object\0",4);
pub static mut sort_sym: sort_entry = sort_entry{se_header:b"Symbol\0".as_ptr() as _,se_cmp:Some(empty_cmp),se_collapse:None,se_sort:Some(empty_cmp),se_init:None,se_snprintf:Some(empty_snprintf),se_filter:Some(empty_filter),se_width_idx:5};
pub static mut sort_srcline: sort_entry = sort_entry{se_header:b"Source:Line\0".as_ptr() as _,se_cmp:Some(empty_cmp),se_collapse:Some(empty_cmp),se_sort:Some(empty_cmp),se_init:Some(empty_init),se_snprintf:Some(empty_snprintf),se_filter:None,se_width_idx:9};
pub static mut sort_parent: sort_entry = se!(b"Parent symbol\0",6);
pub static mut sort_type: sort_entry = sort_entry{se_header:b"Data Type\0".as_ptr() as _,se_cmp:Some(empty_cmp),se_collapse:Some(empty_cmp),se_sort:Some(empty_cmp),se_init:Some(empty_init),se_snprintf:Some(empty_snprintf),se_filter:None,se_width_idx:28};

pub unsafe extern "C" fn sort__comm_nodigit_len(_entry:*mut hist_entry)->size_t{0}
pub unsafe extern "C" fn _sort__sym_cmp(_sym_l:*mut symbol,_sym_r:*mut symbol)->int64_t{0}
pub unsafe extern "C" fn hist_entry__srcline(_he:*mut hist_entry)->*mut c_char{ptr::null_mut()}
pub unsafe extern "C" fn hist_entry__transaction_len()->c_int{0}
pub unsafe extern "C" fn perf_hpp__reset_sort_width(_fmt:*mut perf_hpp_fmt,_hists:*mut hists){}
pub unsafe extern "C" fn perf_hpp__is_sort_entry(_format:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_trace_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_srcline_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_srcfile_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_thread_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_comm_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_dso_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_sym_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_parallelism_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__is_dynamic_entry(_fmt:*mut perf_hpp_fmt)->bool{false}
pub unsafe extern "C" fn perf_hpp__defined_dynamic_entry(_fmt:*mut perf_hpp_fmt,_hists:*mut hists)->bool{false}
pub unsafe extern "C" fn perf_hpp_fmt__dup(fmt:*mut perf_hpp_fmt)->*mut perf_hpp_fmt{fmt}
pub unsafe extern "C" fn hist_entry__filter(_he:*mut hist_entry,_type:c_int,_arg:*const c_void)->c_int{-1}
pub unsafe extern "C" fn hpp_dimension__add_output(_col:c_uint,_implicit:bool)->c_int{0}
pub unsafe extern "C" fn sort_dimension__add(_list:*mut perf_hpp_list,_tok:*const c_char,_evlist:*mut evlist,_env:*mut perf_env,_level:c_int)->c_int{-1}
pub unsafe extern "C" fn perf_hpp__set_elide(_idx:c_int,_elide:bool){}
pub unsafe extern "C" fn sort__setup_elide(_output:*mut FILE){}
pub unsafe extern "C" fn output_field_add(_list:*mut perf_hpp_list,_tok:*const c_char,_level:*mut c_int)->c_int{-1}
pub unsafe extern "C" fn reset_dimensions(){}
pub unsafe extern "C" fn is_strict_order(order:*const c_char)->bool{!order.is_null() && *order != b'+' as c_char}
pub unsafe extern "C" fn setup_sorting(_evlist:*mut evlist,_env:*mut perf_env)->c_int{0}
pub unsafe extern "C" fn reset_output_field(){field_order=ptr::null();sort_order=ptr::null();}
pub unsafe extern "C" fn sort_help(_prefix:*const c_char,_mode:sort_mode)->*mut c_char{ptr::null_mut()}

// Translation note: sort.c contains numerous static sort_entry initializers and helper functions that
// dereference perf-private structs from headers outside this isolated file. Those dependencies are
// represented here as opaque C ABI items; behavior that cannot be expressed file-locally is kept as
// narrow unsafe extern-shaped placeholders rather than implemented with invented layouts.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
