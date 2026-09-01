// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2022. Huawei Technologies Co., Ltd */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_ANY: u64 = 0;

pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

/* Map value type: has BTF-managed field (bpf_timer) */
#[repr(C)]
pub struct val {
    pub t: bpf_timer,
    pub payload: __u64,
}

#[repr(C)]
pub struct htab_map {
    _private: [u8; 0],
}

// Original C map definition:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, struct val);
// } htab SEC(".maps");
#[used]
#[link_section = ".maps"]
pub static mut htab: htab_map = htab_map { _private: [] };

pub static mut pid: i32 = 0;
pub static mut update_err: i32 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_update_elem(
        map: *mut htab_map,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
}

#[no_mangle]
#[link_section = "?fentry/bpf_obj_cancel_fields"]
pub unsafe extern "C" fn bpf_obj_cancel_fields(ctx: *mut core::ffi::c_void) -> i32 {
    let key: __u32 = 0;
    let value: val = val {
        t: bpf_timer { _private: [] },
        payload: 1,
    };

    let _ = ctx;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != pid {
        return 0;
    }

    update_err = bpf_map_update_elem(
        core::ptr::addr_of_mut!(htab),
        core::ptr::addr_of!(key) as *const core::ffi::c_void,
        core::ptr::addr_of!(value) as *const core::ffi::c_void,
        BPF_ANY,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
