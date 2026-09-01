// Translated from perf/util/mem2node.h
// Original includes:
// - <linux/rbtree.h> for rb_root
// - <linux/types.h> for u64

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phys_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mem2node {
    pub root: rb_root,
    pub entries: *mut phys_entry,
    pub cnt: ::std::os::raw::c_int,
}

unsafe extern "C" {
    pub fn mem2node__init(map: *mut mem2node, env: *mut perf_env) -> ::std::os::raw::c_int;
    pub fn mem2node__exit(map: *mut mem2node);
    pub fn mem2node__node(map: *mut mem2node, addr: u64) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
