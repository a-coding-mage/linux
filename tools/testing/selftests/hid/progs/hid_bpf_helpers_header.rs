/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2022 Benjamin Tissoires
 */

/*
 * C header guard and include directives removed for Rust translation.
 *
 * The original header temporarily renamed vmlinux.h HID/BPF symbols before
 * including vmlinux.h, then restored the names so the local definitions below
 * override them. It also defined BPF_NO_KFUNC_PROTOTYPES before that include.
 */

pub const HID_INPUT_REPORT: hid_report_type = 0;
pub const HID_OUTPUT_REPORT: hid_report_type = 1;
pub const HID_FEATURE_REPORT: hid_report_type = 2;
pub const HID_REPORT_TYPES: hid_report_type = 3;

pub type hid_report_type = ::core::ffi::c_uint;

#[repr(C)]
pub struct hid_device {
    pub id: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct bpf_wq {
    pub __opaque: [__u64; 2],
}

#[repr(C)]
pub union hid_bpf_ctx_retval_size {
    pub retval: __s32,
    pub size: __s32,
}

#[repr(C)]
pub struct hid_bpf_ctx {
    pub hid: *mut hid_device,
    pub allocated_size: __u32,
    pub retval_size: hid_bpf_ctx_retval_size,
}

pub const HID_REQ_GET_REPORT: hid_class_request = 0x01;
pub const HID_REQ_GET_IDLE: hid_class_request = 0x02;
pub const HID_REQ_GET_PROTOCOL: hid_class_request = 0x03;
pub const HID_REQ_SET_REPORT: hid_class_request = 0x09;
pub const HID_REQ_SET_IDLE: hid_class_request = 0x0A;
pub const HID_REQ_SET_PROTOCOL: hid_class_request = 0x0B;

pub type hid_class_request = ::core::ffi::c_uint;

#[repr(C)]
pub struct hid_bpf_ops {
    pub hid_id: ::core::ffi::c_int,
    pub flags: u32,
    pub list: list_head,
    pub hid_device_event: Option<
        unsafe extern "C" fn(
            ctx: *mut hid_bpf_ctx,
            report_type: hid_report_type,
            source: u64,
        ) -> ::core::ffi::c_int,
    >,
    pub hid_rdesc_fixup:
        Option<unsafe extern "C" fn(ctx: *mut hid_bpf_ctx) -> ::core::ffi::c_int>,
    pub hid_hw_request: Option<
        unsafe extern "C" fn(
            ctx: *mut hid_bpf_ctx,
            reportnum: ::core::ffi::c_uchar,
            rtype: hid_report_type,
            reqtype: hid_class_request,
            source: u64,
        ) -> ::core::ffi::c_int,
    >,
    pub hid_hw_output_report: Option<
        unsafe extern "C" fn(ctx: *mut hid_bpf_ctx, source: u64) -> ::core::ffi::c_int,
    >,
    pub hdev: *mut hid_device,
}

/* Defined only when absent in the C header. */
pub const BPF_F_BEFORE: ::core::ffi::c_uint = 1_u32 << 3;

/* following are kfuncs exported by HID for HID-BPF */
unsafe extern "C" {
    pub fn hid_bpf_get_data(
        ctx: *mut hid_bpf_ctx,
        offset: ::core::ffi::c_uint,
        __sz: size_t,
    ) -> *mut __u8;

    pub fn hid_bpf_allocate_context(hid_id: ::core::ffi::c_uint) -> *mut hid_bpf_ctx;

    pub fn hid_bpf_release_context(ctx: *mut hid_bpf_ctx);

    pub fn hid_bpf_hw_request(
        ctx: *mut hid_bpf_ctx,
        data: *mut __u8,
        buf__sz: size_t,
        type_: hid_report_type,
        reqtype: hid_class_request,
    ) -> ::core::ffi::c_int;

    pub fn hid_bpf_hw_output_report(
        ctx: *mut hid_bpf_ctx,
        buf: *mut __u8,
        buf__sz: size_t,
    ) -> ::core::ffi::c_int;

    pub fn hid_bpf_input_report(
        ctx: *mut hid_bpf_ctx,
        type_: hid_report_type,
        data: *mut __u8,
        buf__sz: size_t,
    ) -> ::core::ffi::c_int;

    pub fn hid_bpf_try_input_report(
        ctx: *mut hid_bpf_ctx,
        type_: hid_report_type,
        data: *mut __u8,
        buf__sz: size_t,
    ) -> ::core::ffi::c_int;

    /* bpf_wq implementation */
    pub fn bpf_wq_init(
        wq: *mut bpf_wq,
        p__map: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn bpf_wq_start(wq: *mut bpf_wq, flags: ::core::ffi::c_uint)
        -> ::core::ffi::c_int;

    pub fn bpf_wq_set_callback(
        wq: *mut bpf_wq,
        callback_fn: Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *mut ::core::ffi::c_int,
                arg3: *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
