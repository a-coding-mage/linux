// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Red hat */
/* Translated from C source using hid_bpf_helpers.h-provided definitions. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type __u8 = u8;
type __u64 = u64;
type size_t = usize;

const HID_INPUT_REPORT: hid_report_type = 0;
const BPF_F_BEFORE: __u64 = 1;
const BPF_MAP_TYPE_HASH: u32 = 1;

type hid_report_type = u32;
type hid_class_request = u32;

#[repr(C)]
pub struct hid_device {
    pub id: i32,
}

#[repr(C)]
pub struct hid_bpf_ctx {
    pub hid: *mut hid_device,
    pub size: i32,
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_wq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hid_bpf_ops {
    pub hid_device_event: *mut c_void,
    pub hid_rdesc_fixup: *mut c_void,
    pub hid_hw_request: *mut c_void,
    pub hid_hw_output_report: *mut c_void,
    pub hid_id: u32,
    pub flags: __u64,
}

#[repr(C)]
pub struct attach_prog_args {
    pub prog_fd: i32,
    pub hid: u32,
    pub retval: i32,
    pub insert_head: i32,
}

#[repr(C)]
pub struct hid_hw_request_syscall_args {
    /* data needs to come at offset 0 so we can use it in calls */
    pub data: [__u8; 10],
    pub hid: u32,
    pub retval: i32,
    pub size: size_t,
    pub type_: hid_report_type,
    pub request_type: __u8,
}

#[repr(C)]
pub struct elem {
    pub work: bpf_wq,
}

unsafe extern "C" {
    fn hid_bpf_get_data(hid_ctx: *mut hid_bpf_ctx, offset: u32, size: __u64) -> *mut __u8;
    fn hid_bpf_allocate_context(hid: u32) -> *mut hid_bpf_ctx;
    fn hid_bpf_release_context(ctx: *mut hid_bpf_ctx);
    fn hid_bpf_hw_request(
        ctx: *mut hid_bpf_ctx,
        data: *mut __u8,
        size: size_t,
        type_: hid_report_type,
        request_type: __u8,
    ) -> i32;
    fn hid_bpf_hw_output_report(ctx: *mut hid_bpf_ctx, data: *mut __u8, size: size_t) -> i32;
    fn hid_bpf_input_report(
        ctx: *mut hid_bpf_ctx,
        type_: hid_report_type,
        data: *mut __u8,
        size: size_t,
    ) -> i32;
    fn hid_bpf_try_input_report(
        ctx: *mut hid_bpf_ctx,
        type_: hid_report_type,
        data: *mut __u8,
        size: size_t,
    ) -> i32;

    fn bpf_map_update_elem(map: *mut c_void, key: *mut i32, value: *mut elem, flags: __u64) -> i32;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *mut i32) -> *mut elem;
    fn bpf_wq_init(wq: *mut bpf_wq, map: *mut c_void, flags: __u64) -> i32;
    fn bpf_wq_set_callback(
        wq: *mut bpf_wq,
        cb: unsafe extern "C" fn(*mut c_void, *mut i32, *mut c_void) -> i32,
        flags: __u64,
    ) -> i32;
    fn bpf_wq_start(wq: *mut bpf_wq, flags: __u64) -> i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut callback_check: __u64 = 52;
#[unsafe(no_mangle)]
pub static mut callback2_check: __u64 = 52;
#[unsafe(no_mangle)]
pub static mut get_data_overflow_check: __u64 = 0;

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_first_event(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let rw_data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 3 /* size */) };

    if rw_data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        callback_check = *rw_data.add(1) as __u64;
        *rw_data.add(2) = (*rw_data.add(1)).wrapping_add(5);
        (*hid_ctx).size
    }
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut first_event: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_first_event as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 2,
    flags: 0,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __hid_subprog_first_event(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let rw_data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 3 /* size */) };

    if rw_data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        *rw_data.add(2) = (*rw_data.add(1)).wrapping_add(5);
        (*hid_ctx).size
    }
}

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_subprog_first_event(
    hid_ctx: *mut hid_bpf_ctx,
    type_: hid_report_type,
) -> i32 {
    unsafe { __hid_subprog_first_event(hid_ctx, type_) }
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut subprog_first_event: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_subprog_first_event as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 2,
    flags: 0,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_second_event(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let rw_data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 4 /* size */) };

    if rw_data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        *rw_data.add(3) = (*rw_data.add(2)).wrapping_add(5);
        (*hid_ctx).size
    }
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut second_event: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_second_event as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_change_report_id(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let rw_data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 3 /* size */) };

    if rw_data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        *rw_data.add(0) = 2;
    }

    9
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut change_report_id: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_change_report_id as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("syscall") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_user_raw_request(args: *mut hid_hw_request_syscall_args) -> i32 {
    let ctx: *mut hid_bpf_ctx;
    let size = unsafe { (*args).size };
    let mut ret: i32 = 0;

    if size > unsafe { (*args).data.len() } {
        return -7; /* -E2BIG */
    }

    ctx = unsafe { hid_bpf_allocate_context((*args).hid) };
    if ctx.is_null() {
        return -1; /* EPERM check */
    }

    ret = unsafe {
        hid_bpf_hw_request(
            ctx,
            (*args).data.as_mut_ptr(),
            size,
            (*args).type_,
            (*args).request_type,
        )
    };
    unsafe {
        (*args).retval = ret;
        hid_bpf_release_context(ctx);
    }

    0
}

/* SEC("syscall") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_user_output_report(args: *mut hid_hw_request_syscall_args) -> i32 {
    let ctx: *mut hid_bpf_ctx;
    let size = unsafe { (*args).size };
    let mut ret: i32 = 0;

    if size > unsafe { (*args).data.len() } {
        return -7; /* -E2BIG */
    }

    ctx = unsafe { hid_bpf_allocate_context((*args).hid) };
    if ctx.is_null() {
        return -1; /* EPERM check */
    }

    ret = unsafe { hid_bpf_hw_output_report(ctx, (*args).data.as_mut_ptr(), size) };
    unsafe {
        (*args).retval = ret;
        hid_bpf_release_context(ctx);
    }

    0
}

/* SEC("syscall") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_user_input_report(args: *mut hid_hw_request_syscall_args) -> i32 {
    let ctx: *mut hid_bpf_ctx;
    let size = unsafe { (*args).size };
    let mut ret: i32 = 0;

    if size > unsafe { (*args).data.len() } {
        return -7; /* -E2BIG */
    }

    ctx = unsafe { hid_bpf_allocate_context((*args).hid) };
    if ctx.is_null() {
        return -1; /* EPERM check */
    }

    ret = unsafe { hid_bpf_input_report(ctx, HID_INPUT_REPORT, (*args).data.as_mut_ptr(), size) };
    unsafe {
        (*args).retval = ret;
        hid_bpf_release_context(ctx);
    }

    0
}

static rdesc: [__u8; 57] = [
    0x05, 0x01, /* USAGE_PAGE (Generic Desktop) */
    0x09, 0x32, /* USAGE (Z) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x81, 0x06, /* INPUT (Data,Var,Rel) */
    0x06, 0x00, 0xff, /* Usage Page (Vendor Defined Page 1) */
    0x19, 0x01, /* USAGE_MINIMUM (1) */
    0x29, 0x03, /* USAGE_MAXIMUM (3) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0x91, 0x02, /* Output (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x91, 0x01, /* Output (Cnst,Var,Abs) */
    0x06, 0x00, 0xff, /* Usage Page (Vendor Defined Page 1) */
    0x19, 0x06, /* USAGE_MINIMUM (6) */
    0x29, 0x08, /* USAGE_MAXIMUM (8) */
    0x15, 0x00, /* LOGICAL_MINIMUM (0) */
    0x25, 0x01, /* LOGICAL_MAXIMUM (1) */
    0x95, 0x03, /* REPORT_COUNT (3) */
    0x75, 0x01, /* REPORT_SIZE (1) */
    0xb1, 0x02, /* Feature (Data,Var,Abs) */
    0x95, 0x01, /* REPORT_COUNT (1) */
    0x75, 0x05, /* REPORT_SIZE (5) */
    0x91, 0x01, /* Output (Cnst,Var,Abs) */
    0xc0, /* END_COLLECTION */
    0xc0, /* END_COLLECTION */
];

/*
 * the following program is marked as sleepable (struct_ops.s).
 * This is not strictly mandatory but is a nice test for
 * sleepable struct_ops
 */
/* SEC("?struct_ops.s/hid_rdesc_fixup") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_rdesc_fixup(hid_ctx: *mut hid_bpf_ctx) -> i32 {
    let data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 4096 /* size */) };

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        callback2_check = *data.add(4) as __u64;
        /* insert rdesc at offset 73 */
        ptr::copy_nonoverlapping(rdesc.as_ptr(), data.add(73), rdesc.len());
        /* Change Usage Vendor globally */
        *data.add(4) = 0x42;
    }

    (size_of::<[__u8; 57]>() + 73) as i32
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut rdesc_fixup: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: hid_rdesc_fixup as *mut c_void,
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops.s/hid_rdesc_fixup") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_rdesc_fixup_get_data_overflow(hid_ctx: *mut hid_bpf_ctx) -> i32 {
    if unsafe { hid_bpf_get_data(hid_ctx, 2 /* offset */, !0u64 /* size */) }.is_null() {
        unsafe {
            get_data_overflow_check = 1;
        }
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut rdesc_fixup_get_data_overflow: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: hid_rdesc_fixup_get_data_overflow as *mut c_void,
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_insert1(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 4 /* size */) };

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* we need to be run first */
        if *data.add(2) != 0 || *data.add(3) != 0 {
            return -1;
        }

        *data.add(1) = 1;
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_insert1: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_test_insert1 as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: BPF_F_BEFORE,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_insert2(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 4 /* size */) };

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* after insert0 and before insert2 */
        if *data.add(1) == 0 || *data.add(3) != 0 {
            return -1;
        }

        *data.add(2) = 2;
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_insert2: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_test_insert2 as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_insert3(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 4 /* size */) };

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* at the end */
        if *data.add(1) == 0 || *data.add(2) == 0 {
            return -1;
        }

        *data.add(3) = 3;
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_insert3: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_test_insert3 as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_hw_request") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_filter_raw_request(
    _hctx: *mut hid_bpf_ctx,
    _reportnum: u8,
    _rtype: hid_report_type,
    _reqtype: hid_class_request,
    _source: __u64,
) -> i32 {
    -20
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_filter_raw_request: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: hid_test_filter_raw_request as *mut c_void,
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

static mut current_file: *mut file = ptr::null_mut();

/* SEC("fentry/hidraw_open") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hidraw_open(_inode: *mut inode, file: *mut file) -> i32 {
    unsafe {
        current_file = file;
    }
    0
}

/* SEC("?struct_ops.s/hid_hw_request") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_hidraw_raw_request(
    hctx: *mut hid_bpf_ctx,
    reportnum: u8,
    rtype: hid_report_type,
    reqtype: hid_class_request,
    source: __u64,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hctx, 0 /* offset */, 3 /* size */) };
    let ret: i32;

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* check if the incoming request comes from our hidraw operation */
        if source == current_file as __u64 {
            *data.add(0) = reportnum;

            ret = hid_bpf_hw_request(hctx, data, 2, rtype, reqtype as __u8);
            if ret != 2 {
                return -1;
            }
            *data.add(0) = reportnum.wrapping_add(1);
            *data.add(1) = reportnum.wrapping_add(2);
            *data.add(2) = reportnum.wrapping_add(3);
            return 3;
        }
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_hidraw_raw_request: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: hid_test_hidraw_raw_request as *mut c_void,
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops.s/hid_hw_request") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_infinite_loop_raw_request(
    hctx: *mut hid_bpf_ctx,
    reportnum: u8,
    rtype: hid_report_type,
    reqtype: hid_class_request,
    _source: __u64,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hctx, 0 /* offset */, 3 /* size */) };
    let ret: i32;

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* always forward the request as-is to the device, hid-bpf should prevent
         * infinite loops.
         */
        *data.add(0) = reportnum;

        ret = hid_bpf_hw_request(hctx, data, 2, rtype, reqtype as __u8);
        if ret == 2 {
            return 3;
        }
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_infinite_loop_raw_request: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: hid_test_infinite_loop_raw_request as *mut c_void,
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_hw_output_report") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_filter_output_report(
    _hctx: *mut hid_bpf_ctx,
    _reportnum: u8,
    _rtype: hid_report_type,
    _reqtype: hid_class_request,
    _source: __u64,
) -> i32 {
    -25
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_filter_output_report: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: hid_test_filter_output_report as *mut c_void,
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops.s/hid_hw_output_report") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_hidraw_output_report(
    hctx: *mut hid_bpf_ctx,
    source: __u64,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hctx, 0 /* offset */, 3 /* size */) };

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* check if the incoming request comes from our hidraw operation */
        if source == current_file as __u64 {
            return hid_bpf_hw_output_report(hctx, data, 2);
        }
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_hidraw_output_report: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: hid_test_hidraw_output_report as *mut c_void,
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops.s/hid_hw_output_report") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_infinite_loop_output_report(
    hctx: *mut hid_bpf_ctx,
    _source: __u64,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hctx, 0 /* offset */, 3 /* size */) };
    let ret: i32;

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /* always forward the request as-is to the device, hid-bpf should prevent
         * infinite loops.
         */

        ret = hid_bpf_hw_output_report(hctx, data, 2);
        if ret == 2 {
            return 2;
        }
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_infinite_loop_output_report: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: ptr::null_mut(),
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: hid_test_infinite_loop_output_report as *mut c_void,
    hid_id: 0,
    flags: 0,
};

#[repr(C)]
pub struct hmap_def {
    pub type_: u32,
    pub max_entries: u32,
}

/* struct { __uint(type, BPF_MAP_TYPE_HASH); __uint(max_entries, 1);
 * __type(key, int); __type(value, struct elem); } hmap SEC(".maps");
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hmap: hmap_def = hmap_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
};

unsafe extern "C" fn wq_cb_sleepable(_map: *mut c_void, key: *mut i32, _work: *mut c_void) -> i32 {
    let mut buf: [__u8; 9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];
    let hid_ctx: *mut hid_bpf_ctx;

    hid_ctx = unsafe { hid_bpf_allocate_context(*key as u32) };
    if hid_ctx.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        hid_bpf_input_report(hid_ctx, HID_INPUT_REPORT, buf.as_mut_ptr(), size_of::<[__u8; 9]>());
        hid_bpf_release_context(hid_ctx);
    }

    0
}

unsafe fn test_inject_input_report_callback(key: *mut i32) -> i32 {
    let mut init: elem = elem { work: bpf_wq { _private: [] } };
    let val: *mut elem;
    let wq: *mut bpf_wq;

    if unsafe { bpf_map_update_elem(&raw mut hmap as *mut c_void, key, &mut init, 0) } != 0 {
        return -1;
    }

    val = unsafe { bpf_map_lookup_elem(&raw mut hmap as *mut c_void, key) };
    if val.is_null() {
        return -2;
    }

    wq = unsafe { &mut (*val).work };
    if unsafe { bpf_wq_init(wq, &raw mut hmap as *mut c_void, 0) } != 0 {
        return -3;
    }

    if unsafe { bpf_wq_set_callback(wq, wq_cb_sleepable, 0) } != 0 {
        return -4;
    }

    if unsafe { bpf_wq_start(wq, 0) } != 0 {
        return -5;
    }

    0
}

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_multiply_events_wq(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 9 /* size */) };
    let mut hid = unsafe { (*(*hid_ctx).hid).id };
    let ret: i32;

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        if *data.add(0) != 1 {
            return 0;
        }

        ret = test_inject_input_report_callback(&mut hid);
        if ret != 0 {
            return ret;
        }

        *data.add(1) = (*data.add(1)).wrapping_add(5);
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_multiply_events_wq: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_test_multiply_events_wq as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_multiply_events(
    hid_ctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hid_ctx, 0 /* offset */, 9 /* size */) };
    let mut buf: [__u8; 9] = [0; 9];
    let ret: i32;

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        if *data.add(0) != 1 {
            return 0;
        }

        /*
         * we have to use an intermediate buffer as hid_bpf_input_report
         * will memset data to \0
         */
        ptr::copy_nonoverlapping(data, buf.as_mut_ptr(), size_of::<[__u8; 9]>());

        buf[0] = 2;
        buf[1] = buf[1].wrapping_add(5);
        ret = hid_bpf_try_input_report(
            hid_ctx,
            HID_INPUT_REPORT,
            buf.as_mut_ptr(),
            size_of::<[__u8; 9]>(),
        );
        if ret < 0 {
            return ret;
        }

        /*
         * In real world we should reset the original buffer as data might be garbage now,
         * but it actually now has the content of 'buf'
         */
        *data.add(1) = (*data.add(1)).wrapping_add(5);
    }

    9
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_multiply_events: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_test_multiply_events as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

/* SEC("?struct_ops/hid_device_event") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hid_test_infinite_loop_input_report(
    hctx: *mut hid_bpf_ctx,
    report_type: hid_report_type,
    _source: __u64,
) -> i32 {
    let data = unsafe { hid_bpf_get_data(hctx, 0 /* offset */, 6 /* size */) };
    let mut buf: [__u8; 6] = [0; 6];

    if data.is_null() {
        return 0; /* EPERM check */
    }

    unsafe {
        /*
         * we have to use an intermediate buffer as hid_bpf_input_report
         * will memset data to \0
         */
        ptr::copy_nonoverlapping(data, buf.as_mut_ptr(), size_of::<[__u8; 6]>());

        /* always forward the request as-is to the device, hid-bpf should prevent
         * infinite loops.
         * the return value is ignored so the event is passing to userspace.
         */

        hid_bpf_try_input_report(hctx, report_type, buf.as_mut_ptr(), size_of::<[__u8; 6]>());

        /* each time we process the event, we increment by one data[1]:
         * after each successful call to hid_bpf_try_input_report, buf
         * has been memcopied into data by the kernel.
         */
        *data.add(1) = (*data.add(1)).wrapping_add(1);
    }

    0
}

/* SEC(".struct_ops.link") */
#[unsafe(no_mangle)]
pub static mut test_infinite_loop_input_report: hid_bpf_ops = hid_bpf_ops {
    hid_device_event: hid_test_infinite_loop_input_report as *mut c_void,
    hid_rdesc_fixup: ptr::null_mut(),
    hid_hw_request: ptr::null_mut(),
    hid_hw_output_report: ptr::null_mut(),
    hid_id: 0,
    flags: 0,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
