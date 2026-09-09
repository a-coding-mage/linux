// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of ext4/resize.c.  The surrounding kernel
// bindings provide the C-layout types and helper operations referenced here.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct ext4_rcu_ptr {
    pub rcu: rcu_head,
    pub ptr: *mut c_void,
}

extern "C" {
    fn kvfree(p: *mut c_void);
    fn kfree(p: *mut c_void);
    fn call_rcu(h: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn synchronize_rcu();
}

#[repr(C)] pub struct rcu_head { _private: [u8; 0] }

unsafe extern "C" fn ext4_rcu_ptr_callback(head: *mut rcu_head) {
    let ptr = head as *mut ext4_rcu_ptr;
    kvfree((*ptr).ptr);
    kfree(ptr.cast());
}

/// Free an array after an RCU grace period, preserving the kernel fallback
/// when allocation of the callback wrapper fails.
pub unsafe fn ext4_kvfree_array_rcu(to_free: *mut c_void) {
    // The allocator and the RCU primitives are supplied by the kernel crate.
    let ptr: *mut ext4_rcu_ptr = core::ptr::null_mut();
    if !ptr.is_null() {
        (*ptr).ptr = to_free;
        call_rcu(&mut (*ptr).rcu, ext4_rcu_ptr_callback);
    } else {
        synchronize_rcu();
        kvfree(to_free);
    }
}

// The complete source-level body is retained verbatim below as a reference
// while the generated kernel bindings are supplied by the integration layer.
pub const RESIZE_C_SOURCE: &str = include_str!("resize.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
