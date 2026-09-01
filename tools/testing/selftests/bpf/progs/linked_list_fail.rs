// SPDX-License-Identifier: GPL-2.0
// Translated from linked_list_fail.c. External BPF/kernel types, maps, globals,
// and helpers are declared here and are expected to be supplied by the build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_list_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct foo {
    pub lock: bpf_spin_lock,
    pub head: bpf_list_head,
    pub node: bpf_list_node,
    pub node2: bpf_list_node,
    pub data: i32,
}

#[repr(C)]
pub struct bar {
    pub node: bpf_list_node,
}

#[repr(C)]
pub struct map_value {
    pub lock: bpf_spin_lock,
    pub head: bpf_list_head,
}

#[repr(C)]
pub struct __sk_buff {
    pub protocol: u32,
}

#[repr(C)]
pub struct obj_new_flex_elem {
    pub lo: i32,
    pub hi: i32,
}

#[repr(C)]
pub struct obj_new_flex {
    pub hdr: i32,
    pub cells: [obj_new_flex_elem; 0],
}

#[repr(C)]
pub union obj_new_no_struct_anon {
    pub data: i32,
    pub udata: u32,
}

#[repr(C)]
pub struct ptr_walk_scalar_test2 {
    pub next: *mut ptr_walk_scalar_test2,
}

#[repr(C)]
pub struct ptr_walk_scalar_test1 {
    pub ptr: *mut ptr_walk_scalar_test2,
}

#[repr(C)]
pub struct no_node_value_type_anon {
    pub data: i32,
}

#[repr(C)]
pub struct no_head_type_anon {
    pub data: i32,
}

#[repr(C)]
pub struct pop_ptr_off_anon {
    pub head: bpf_list_head,
    pub lock: bpf_spin_lock,
}

unsafe extern "C" {
    static mut map_of_maps: c_void;
    static mut array_map: c_void;
    static mut ghead: bpf_list_head;
    static mut glock: bpf_spin_lock;
    static mut glock2: bpf_spin_lock;

    fn bpf_map_lookup_elem(map: *mut c_void, key: *const i32) -> *mut c_void;
    fn bpf_obj_new_impl(local_type_id: u64, meta: *mut c_void) -> *mut c_void;
    fn bpf_obj_drop_impl(ptr: *mut c_void, meta: *mut c_void);
    fn bpf_core_type_id_local_int() -> u64;
    fn bpf_core_type_id_local_obj_new_flex() -> u64;
    fn bpf_obj_new_foo() -> *mut foo;
    fn bpf_obj_new_bar() -> *mut bar;
    fn bpf_obj_new_ptr_walk_scalar_test1() -> *mut ptr_walk_scalar_test1;
    fn bpf_obj_new_no_node_value_type_anon() -> *mut no_node_value_type_anon;
    fn bpf_obj_new_no_head_type_anon() -> *mut no_head_type_anon;
    fn bpf_obj_new_pop_ptr_off_anon() -> *mut pop_ptr_off_anon;
    fn bpf_obj_new_obj_new_no_struct_anon() -> *mut obj_new_no_struct_anon;
    fn bpf_obj_drop(ptr: *mut c_void);
    fn bpf_list_push_front(head: *mut c_void, node: *mut c_void);
    fn bpf_list_push_back(head: *mut c_void, node: *mut c_void);
    fn bpf_list_pop_front(head: *mut c_void) -> *mut c_void;
    fn bpf_list_pop_back(head: *mut c_void) -> *mut c_void;
    fn bpf_spin_lock(lock: *mut c_void);
    fn bpf_spin_unlock(lock: *mut c_void);
    fn bpf_this_cpu_ptr(ptr: *mut c_void) -> *mut c_void;
}

macro_rules! init {
    () => {{
        let key: i32 = 0;
        let map = bpf_map_lookup_elem(&raw mut map_of_maps as *mut c_void, &key);
        if map.is_null() {
            return 0;
        }
        let v = bpf_map_lookup_elem(&raw mut array_map as *mut c_void, &key) as *mut map_value;
        if v.is_null() {
            return 0;
        }
        let v2 = bpf_map_lookup_elem(&raw mut array_map as *mut c_void, &key) as *mut map_value;
        if v2.is_null() {
            return 0;
        }
        let iv = bpf_map_lookup_elem(map, &key) as *mut map_value;
        if iv.is_null() {
            return 0;
        }
        let iv2 = bpf_map_lookup_elem(map, &key) as *mut map_value;
        if iv2.is_null() {
            return 0;
        }
        let f = bpf_obj_new_foo();
        if f.is_null() {
            return 0;
        }
        let f1 = f;
        let f2 = bpf_obj_new_foo();
        if f2.is_null() {
            bpf_obj_drop(f1 as *mut c_void);
            return 0;
        }
        let b = bpf_obj_new_bar();
        if b.is_null() {
            bpf_obj_drop(f2 as *mut c_void);
            bpf_obj_drop(f1 as *mut c_void);
            return 0;
        }
        (v, v2, iv, iv2, f, f1, f2, b)
    }};
}

macro_rules! missing_lock_pop {
    ($name:ident, $op:ident, $head:expr) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "?tc")]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let _ = ctx;
            let (v, _v2, iv, _iv2, f, _f1, _f2, _b) = init!();
            let p: unsafe extern "C" fn(*mut c_void) -> *mut c_void = $op;
            p($head);
            0
        }
    };
}

missing_lock_pop!(kptr_missing_lock_pop_front, bpf_list_pop_front, &raw mut (*f).head as *mut c_void);
missing_lock_pop!(kptr_missing_lock_pop_back, bpf_list_pop_back, &raw mut (*f).head as *mut c_void);
missing_lock_pop!(global_missing_lock_pop_front, bpf_list_pop_front, &raw mut ghead as *mut c_void);
missing_lock_pop!(global_missing_lock_pop_back, bpf_list_pop_back, &raw mut ghead as *mut c_void);
missing_lock_pop!(map_missing_lock_pop_front, bpf_list_pop_front, &raw mut (*v).head as *mut c_void);
missing_lock_pop!(map_missing_lock_pop_back, bpf_list_pop_back, &raw mut (*v).head as *mut c_void);
missing_lock_pop!(inner_map_missing_lock_pop_front, bpf_list_pop_front, &raw mut (*iv).head as *mut c_void);
missing_lock_pop!(inner_map_missing_lock_pop_back, bpf_list_pop_back, &raw mut (*iv).head as *mut c_void);

macro_rules! missing_lock_push {
    ($name:ident, $op:ident, $head:expr, $node:expr) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "?tc")]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let _ = ctx;
            let (v, _v2, iv, _iv2, f, _f1, _f2, b) = init!();
            $op($head, $node);
            0
        }
    };
}

missing_lock_push!(kptr_missing_lock_push_front, bpf_list_push_front, &raw mut (*f).head as *mut c_void, &raw mut (*b).node as *mut c_void);
missing_lock_push!(kptr_missing_lock_push_back, bpf_list_push_back, &raw mut (*f).head as *mut c_void, &raw mut (*b).node as *mut c_void);
missing_lock_push!(global_missing_lock_push_front, bpf_list_push_front, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
missing_lock_push!(global_missing_lock_push_back, bpf_list_push_back, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
missing_lock_push!(map_missing_lock_push_front, bpf_list_push_front, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
missing_lock_push!(map_missing_lock_push_back, bpf_list_push_back, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
missing_lock_push!(inner_map_missing_lock_push_front, bpf_list_push_front, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
missing_lock_push!(inner_map_missing_lock_push_back, bpf_list_push_back, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);

macro_rules! incorrect_lock_pop {
    ($name:ident, $op:ident, $lock:expr, $head:expr) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "?tc")]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let _ = ctx;
            let (v, v2, iv, iv2, _f, f1, f2, _b) = init!();
            let p: unsafe extern "C" fn(*mut c_void) -> *mut c_void = $op;
            bpf_spin_lock($lock);
            p($head);
            0
        }
    };
}

macro_rules! incorrect_lock_push {
    ($name:ident, $op:ident, $lock:expr, $head:expr, $node:expr) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "?tc")]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let _ = ctx;
            let (v, v2, iv, iv2, f, f1, f2, b) = init!();
            bpf_spin_lock($lock);
            $op($head, $node);
            0
        }
    };
}

incorrect_lock_pop!(kptr_kptr_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*f1).lock as *mut c_void, &raw mut (*f2).head as *mut c_void);
incorrect_lock_pop!(kptr_global_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*f1).lock as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(kptr_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*f1).lock as *mut c_void, &raw mut (*v).head as *mut c_void);
incorrect_lock_pop!(kptr_inner_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*f1).lock as *mut c_void, &raw mut (*iv).head as *mut c_void);
incorrect_lock_pop!(global_global_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut glock2 as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(global_kptr_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut glock as *mut c_void, &raw mut (*f1).head as *mut c_void);
incorrect_lock_pop!(global_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut glock as *mut c_void, &raw mut (*v).head as *mut c_void);
incorrect_lock_pop!(global_inner_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut glock as *mut c_void, &raw mut (*iv).head as *mut c_void);
incorrect_lock_pop!(map_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*v).lock as *mut c_void, &raw mut (*v2).head as *mut c_void);
incorrect_lock_pop!(map_kptr_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*v).lock as *mut c_void, &raw mut (*f2).head as *mut c_void);
incorrect_lock_pop!(map_global_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*v).lock as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(map_inner_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*v).lock as *mut c_void, &raw mut (*iv).head as *mut c_void);
incorrect_lock_pop!(inner_map_inner_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*iv).lock as *mut c_void, &raw mut (*iv2).head as *mut c_void);
incorrect_lock_pop!(inner_map_kptr_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*iv).lock as *mut c_void, &raw mut (*f2).head as *mut c_void);
incorrect_lock_pop!(inner_map_global_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*iv).lock as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(inner_map_map_incorrect_lock_pop_front, bpf_list_pop_front, &raw mut (*iv).lock as *mut c_void, &raw mut (*v).head as *mut c_void);

incorrect_lock_pop!(kptr_kptr_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*f1).lock as *mut c_void, &raw mut (*f2).head as *mut c_void);
incorrect_lock_pop!(kptr_global_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*f1).lock as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(kptr_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*f1).lock as *mut c_void, &raw mut (*v).head as *mut c_void);
incorrect_lock_pop!(kptr_inner_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*f1).lock as *mut c_void, &raw mut (*iv).head as *mut c_void);
incorrect_lock_pop!(global_global_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut glock2 as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(global_kptr_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut glock as *mut c_void, &raw mut (*f1).head as *mut c_void);
incorrect_lock_pop!(global_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut glock as *mut c_void, &raw mut (*v).head as *mut c_void);
incorrect_lock_pop!(global_inner_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut glock as *mut c_void, &raw mut (*iv).head as *mut c_void);
incorrect_lock_pop!(map_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*v).lock as *mut c_void, &raw mut (*v2).head as *mut c_void);
incorrect_lock_pop!(map_kptr_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*v).lock as *mut c_void, &raw mut (*f2).head as *mut c_void);
incorrect_lock_pop!(map_global_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*v).lock as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(map_inner_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*v).lock as *mut c_void, &raw mut (*iv).head as *mut c_void);
incorrect_lock_pop!(inner_map_inner_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*iv).lock as *mut c_void, &raw mut (*iv2).head as *mut c_void);
incorrect_lock_pop!(inner_map_kptr_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*iv).lock as *mut c_void, &raw mut (*f2).head as *mut c_void);
incorrect_lock_pop!(inner_map_global_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*iv).lock as *mut c_void, &raw mut ghead as *mut c_void);
incorrect_lock_pop!(inner_map_map_incorrect_lock_pop_back, bpf_list_pop_back, &raw mut (*iv).lock as *mut c_void, &raw mut (*v).head as *mut c_void);

incorrect_lock_push!(kptr_kptr_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*f1).lock as *mut c_void, &raw mut (*f2).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(kptr_global_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*f1).lock as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(kptr_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*f1).lock as *mut c_void, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(kptr_inner_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*f1).lock as *mut c_void, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(global_global_incorrect_lock_push_front, bpf_list_push_front, &raw mut glock2 as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(global_kptr_incorrect_lock_push_front, bpf_list_push_front, &raw mut glock as *mut c_void, &raw mut (*f1).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(global_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut glock as *mut c_void, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(global_inner_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut glock as *mut c_void, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(map_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*v).lock as *mut c_void, &raw mut (*v2).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(map_kptr_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*v).lock as *mut c_void, &raw mut (*f2).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(map_global_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*v).lock as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(map_inner_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*v).lock as *mut c_void, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(inner_map_inner_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*iv).lock as *mut c_void, &raw mut (*iv2).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(inner_map_kptr_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*iv).lock as *mut c_void, &raw mut (*f2).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(inner_map_global_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*iv).lock as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(inner_map_map_incorrect_lock_push_front, bpf_list_push_front, &raw mut (*iv).lock as *mut c_void, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);

incorrect_lock_push!(kptr_kptr_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*f1).lock as *mut c_void, &raw mut (*f2).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(kptr_global_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*f1).lock as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(kptr_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*f1).lock as *mut c_void, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(kptr_inner_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*f1).lock as *mut c_void, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(global_global_incorrect_lock_push_back, bpf_list_push_back, &raw mut glock2 as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(global_kptr_incorrect_lock_push_back, bpf_list_push_back, &raw mut glock as *mut c_void, &raw mut (*f1).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(global_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut glock as *mut c_void, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(global_inner_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut glock as *mut c_void, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(map_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*v).lock as *mut c_void, &raw mut (*v2).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(map_kptr_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*v).lock as *mut c_void, &raw mut (*f2).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(map_global_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*v).lock as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(map_inner_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*v).lock as *mut c_void, &raw mut (*iv).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(inner_map_inner_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*iv).lock as *mut c_void, &raw mut (*iv2).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(inner_map_kptr_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*iv).lock as *mut c_void, &raw mut (*f2).head as *mut c_void, &raw mut (*b).node as *mut c_void);
incorrect_lock_push!(inner_map_global_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*iv).lock as *mut c_void, &raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
incorrect_lock_push!(inner_map_map_incorrect_lock_push_back, bpf_list_push_back, &raw mut (*iv).lock as *mut c_void, &raw mut (*v).head as *mut c_void, &raw mut (*f).node2 as *mut c_void);

#[unsafe(no_mangle)]
#[unsafe(link_section = "?kprobe/xyz")]
pub unsafe extern "C" fn map_compat_kprobe(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_list_push_front(&raw mut ghead as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?kretprobe/xyz")]
pub unsafe extern "C" fn map_compat_kretprobe(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_list_push_front(&raw mut ghead as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tracepoint/xyz")]
pub unsafe extern "C" fn map_compat_tp(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_list_push_front(&raw mut ghead as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?perf_event")]
pub unsafe extern "C" fn map_compat_perf(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_list_push_front(&raw mut ghead as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?raw_tp/xyz")]
pub unsafe extern "C" fn map_compat_raw_tp(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_list_push_front(&raw mut ghead as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?raw_tp.w/xyz")]
pub unsafe extern "C" fn map_compat_raw_tp_w(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_list_push_front(&raw mut ghead as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn obj_type_id_oor(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_obj_new_impl(!0_u64, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn obj_new_no_composite(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    bpf_obj_new_impl(bpf_core_type_id_local_int(), 42usize as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn obj_new_no_struct(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let _ = bpf_obj_new_obj_new_no_struct_anon();
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn obj_new_flex_array(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let p = bpf_obj_new_impl(bpf_core_type_id_local_obj_new_flex(), core::ptr::null_mut()) as *mut obj_new_flex;
    if p.is_null() {
        return 0;
    }
    (*p).cells.as_mut_ptr().add(0).write(obj_new_flex_elem { lo: 0, hi: 42 });
    bpf_obj_drop_impl(p as *mut c_void, core::ptr::null_mut());
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn obj_drop_non_zero_off(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo() as *mut c_void;
    if f.is_null() {
        return 0;
    }
    bpf_obj_drop((f as *mut u8).add(1) as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn new_null_ret(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    (*bpf_obj_new_foo()).data
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn obj_new_acq(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let _ = bpf_obj_new_foo();
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn use_after_drop(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_obj_drop(f as *mut c_void);
    (*f).data
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn ptr_walk_scalar(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let p = bpf_obj_new_ptr_walk_scalar_test1();
    if p.is_null() {
        return 0;
    }
    bpf_this_cpu_ptr((*p).ptr as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn direct_read_lock(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    *(&raw mut (*f).lock as *mut i32)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn direct_write_lock(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    *(&raw mut (*f).lock as *mut i32) = 0;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn direct_read_head(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    *(&raw mut (*f).head as *mut i32)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn direct_write_head(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    *(&raw mut (*f).head as *mut i32) = 0;
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn direct_read_node(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    *(&raw mut (*f).node2 as *mut i32)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn direct_write_node(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    *(&raw mut (*f).node2 as *mut i32) = 0;
    0
}

#[inline(always)]
unsafe fn use_after_unlock(push_front: bool) -> i32 {
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    (*f).data = 42;
    if push_front {
        bpf_list_push_front(&raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
    } else {
        bpf_list_push_back(&raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
    }
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    (*f).data
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn use_after_unlock_push_front(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    use_after_unlock(true)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn use_after_unlock_push_back(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    use_after_unlock(false)
}

#[inline(always)]
unsafe fn list_double_add(push_front: bool) -> i32 {
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    if push_front {
        bpf_list_push_front(&raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
        bpf_list_push_front(&raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
    } else {
        bpf_list_push_back(&raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
        bpf_list_push_back(&raw mut ghead as *mut c_void, &raw mut (*f).node2 as *mut c_void);
    }
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn double_push_front(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    list_double_add(true)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn double_push_back(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    list_double_add(false)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn no_node_value_type(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let p = bpf_obj_new_no_node_value_type_anon() as *mut c_void;
    if p.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(&raw mut ghead as *mut c_void, p);
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_value_type(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let b = bpf_obj_new_bar();
    if b.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(&raw mut ghead as *mut c_void, &raw mut (*b).node as *mut c_void);
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_node_var_off(ctx: *mut __sk_buff) -> i32 {
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(
        &raw mut ghead as *mut c_void,
        (&raw mut (*f).node2 as *mut u8).add((*ctx).protocol as usize) as *mut c_void,
    );
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_node_off1(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(&raw mut ghead as *mut c_void, (&raw mut (*f).node2 as *mut u8).add(1) as *mut c_void);
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_node_off2(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(&raw mut ghead as *mut c_void, &raw mut (*f).node as *mut c_void);
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn no_head_type(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let p = bpf_obj_new_no_head_type_anon() as *mut c_void;
    if p.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(p, core::ptr::null_mut());
    bpf_spin_lock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_head_var_off1(ctx: *mut __sk_buff) -> i32 {
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(
        (&raw mut ghead as *mut u8).add((*ctx).protocol as usize) as *mut c_void,
        &raw mut (*f).node2 as *mut c_void,
    );
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_head_var_off2(ctx: *mut __sk_buff) -> i32 {
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front(
        (&raw mut (*f).head as *mut u8).add((*ctx).protocol as usize) as *mut c_void,
        &raw mut (*f).node2 as *mut c_void,
    );
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_head_off1(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    let b = bpf_obj_new_bar();
    if b.is_null() {
        bpf_obj_drop(f as *mut c_void);
        return 0;
    }
    bpf_spin_lock(&raw mut (*f).lock as *mut c_void);
    bpf_list_push_front((&raw mut (*f).head as *mut u8).add(1) as *mut c_void, &raw mut (*b).node as *mut c_void);
    bpf_spin_unlock(&raw mut (*f).lock as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn incorrect_head_off2(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut glock as *mut c_void);
    bpf_list_push_front((&raw mut ghead as *mut u8).add(1) as *mut c_void, &raw mut (*f).node2 as *mut c_void);
    bpf_spin_unlock(&raw mut glock as *mut c_void);
    0
}

#[inline(always)]
unsafe fn pop_ptr_off(op: unsafe extern "C" fn(*mut c_void) -> *mut c_void) -> i32 {
    let p = bpf_obj_new_pop_ptr_off_anon();
    if p.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut (*p).lock as *mut c_void);
    let n = op(&raw mut (*p).head as *mut c_void) as *mut bpf_list_node;
    bpf_spin_unlock(&raw mut (*p).lock as *mut c_void);
    if n.is_null() {
        return 0;
    }
    bpf_spin_lock(n as *mut c_void);
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn pop_front_off(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    pop_ptr_off(bpf_list_pop_front)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "?tc")]
pub unsafe extern "C" fn pop_back_off(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    pop_ptr_off(bpf_list_pop_back)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
