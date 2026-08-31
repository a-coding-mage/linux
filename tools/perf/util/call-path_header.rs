/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * call-path.h: Manipulate a tree data structure containing function call paths
 * Copyright (c) 2014, Intel Corporation.
 */

/* Dependencies from the original header:
 * <sys/types.h>
 * <linux/types.h>
 * <linux/rbtree.h>
 */

/**
 * struct call_path - node in list of calls leading to a function call.
 * @parent: call path to the parent function call
 * @sym: symbol of function called
 * @ip: only if sym is null, the ip of the function
 * @db_id: id used for db-export
 * @in_kernel: whether function is a in the kernel
 * @rb_node: node in parent's tree of called functions
 * @children: tree of call paths of functions called
 *
 * In combination with the call_return structure, the call_path structure
 * defines a context-sensitive call-graph.
 */
#[repr(C)]
pub struct call_path {
    pub parent: *mut call_path,
    pub sym: *mut symbol,
    pub ip: u64,
    pub db_id: u64,
    pub in_kernel: bool,
    pub rb_node: rb_node,
    pub children: rb_root,
}

pub const CALL_PATH_BLOCK_SHIFT: usize = 8;
pub const CALL_PATH_BLOCK_SIZE: usize = 1usize << CALL_PATH_BLOCK_SHIFT;
pub const CALL_PATH_BLOCK_MASK: usize = CALL_PATH_BLOCK_SIZE - 1;

#[repr(C)]
pub struct call_path_block {
    pub cp: [call_path; CALL_PATH_BLOCK_SIZE],
    pub node: list_head,
}

/**
 * struct call_path_root - root of all call paths.
 * @call_path: root call path
 * @blocks: list of blocks to store call paths
 * @next: next free space
 * @sz: number of spaces
 */
#[repr(C)]
pub struct call_path_root {
    pub call_path: call_path,
    pub blocks: list_head,
    pub next: size_t,
    pub sz: size_t,
}

unsafe extern "C" {
    pub fn call_path_root__new() -> *mut call_path_root;
    pub fn call_path_root__free(cpr: *mut call_path_root);

    pub fn call_path__findnew(
        cpr: *mut call_path_root,
        parent: *mut call_path,
        sym: *mut symbol,
        ip: u64,
        ks: u64,
    ) -> *mut call_path;
}
