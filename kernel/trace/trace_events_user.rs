// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation boundary for trace_events_user.c.
// Linux-kernel types and operations referenced below are supplied externally.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

pub const FIELD_DEPTH_TYPE: c_int = 0;
pub const FIELD_DEPTH_NAME: c_int = 1;
pub const FIELD_DEPTH_SIZE: c_int = 2;
pub const MAX_EVENT_DESC: usize = 512;
pub const MAX_FIELD_ARRAY_SIZE: usize = 1024;
pub const EVENT_STATUS_FTRACE: u8 = 1 << 0;
pub const EVENT_STATUS_PERF: u8 = 1 << 1;
pub const EVENT_STATUS_OTHER: u8 = 1 << 7;
pub const ENABLE_VAL_BIT_MASK: usize = 0x3f;
pub const ENABLE_VAL_FAULTING_BIT: usize = 6;
pub const ENABLE_VAL_FREEING_BIT: usize = 7;
pub const ENABLE_VAL_32_ON_64_BIT: usize = 8;
pub const ENABLE_VAL_COMPAT_MASK: usize = 1 << ENABLE_VAL_32_ON_64_BIT;
pub const ENABLE_VAL_DUP_MASK: usize = ENABLE_VAL_BIT_MASK | ENABLE_VAL_COMPAT_MASK;
pub const VALIDATOR_ENSURE_NULL: c_int = 1 << 0;
pub const VALIDATOR_REL: c_int = 1 << 1;

#[repr(C)]
pub struct user_event_group {
    pub system_name: *mut c_char,
    pub system_multi_name: *mut c_char,
    pub node: hlist_node,
    pub reg_mutex: mutex,
    pub register_table: [hlist_head; 1 << 8],
    pub multi_id: u64,
}

#[repr(C)]
pub struct user_event {
    pub group: *mut user_event_group,
    pub reg_name: *mut c_char,
    pub tracepoint: tracepoint,
    pub call: trace_event_call,
    pub class: trace_event_class,
    pub devent: dyn_event,
    pub node: hlist_node,
    pub fields: list_head,
    pub validators: list_head,
    pub put_work: work_struct,
    pub refcnt: refcount_t,
    pub min_size: c_int,
    pub reg_flags: c_int,
    pub status: i8,
}

#[repr(C)]
pub struct user_event_enabler {
    pub mm_enablers_link: list_head,
    pub event: *mut user_event,
    pub addr: usize,
    pub values: usize,
    pub put_rwork: rcu_work,
}

#[repr(C)]
pub struct user_event_enabler_fault {
    pub work: work_struct,
    pub mm: *mut user_event_mm,
    pub enabler: *mut user_event_enabler,
    pub attempt: c_int,
}

#[repr(C)]
pub struct user_event_refs {
    pub rcu: rcu_head,
    pub count: c_int,
    pub events: [*mut user_event; 0],
}

#[repr(C)]
pub struct user_event_file_info {
    pub group: *mut user_event_group,
    pub refs: *mut user_event_refs,
}

#[repr(C)]
pub struct user_event_validator {
    pub user_event_link: list_head,
    pub offset: c_int,
    pub flags: c_int,
}

// External kernel declarations used by the implementation.
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct tracepoint { pub name: *const c_char }
#[repr(C)] pub struct trace_event_call { pub data: *mut c_void }
#[repr(C)] pub struct trace_event_class { _private: [u8; 0] }
#[repr(C)] pub struct dyn_event { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct rcu_work { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct user_event_mm { _private: [u8; 0] }

pub type user_event_func_t = unsafe extern "C" fn(
    user: *mut user_event, i: *mut c_void, tpdata: *mut c_void, faulted: *mut bool,
);

extern "C" {
    pub fn user_event_mm_remove(t: *mut c_void);
    pub fn user_event_mm_dup(t: *mut c_void, old_mm: *mut user_event_mm);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
