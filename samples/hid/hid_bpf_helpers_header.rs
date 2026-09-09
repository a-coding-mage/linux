/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2022 Benjamin Tissoires
 */

/* following are kfuncs exported by HID for HID-BPF */
/* C __ksym annotations are preserved by this declaration comment. */
extern "C" {
    pub fn hid_bpf_get_data(
        ctx: *mut hid_bpf_ctx,
        offset: u32,
        sz: usize,
    ) -> *mut u8;

    pub fn hid_bpf_attach_prog(
        hid_id: u32,
        prog_fd: i32,
        flags: u32,
    ) -> i32;

    pub fn hid_bpf_allocate_context(hid_id: u32) -> *mut hid_bpf_ctx;

    pub fn hid_bpf_release_context(ctx: *mut hid_bpf_ctx);

    pub fn hid_bpf_hw_request(
        ctx: *mut hid_bpf_ctx,
        data: *mut u8,
        buf_sz: usize,
        type_: hid_report_type,
        reqtype: hid_class_request,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
