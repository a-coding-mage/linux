// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Benjamin Tissoires
 */

// Dependencies supplied by vmlinux.h, bpf_helpers.h, bpf_tracing.h, and
// hid_bpf_helpers.h are intentionally referenced but not implemented here.

pub const HID_UP_BUTTON: u32 = 0x0009;
pub const HID_GD_WHEEL: u32 = 0x0038;

extern "C" {
    pub fn hid_bpf_get_data(
        hctx: *mut hid_bpf_ctx,
        offset: usize,
        size: usize,
    ) -> *mut u8;
    pub fn hid_bpf_allocate_context(hid: u32) -> *mut hid_bpf_ctx;
    pub fn hid_bpf_hw_request(
        ctx: *mut hid_bpf_ctx,
        data: *mut u8,
        size: usize,
        report_type: u32,
        request: u32,
    ) -> i32;
    pub fn hid_bpf_release_context(ctx: *mut hid_bpf_ctx);
    pub fn bpf_printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct hid_bpf_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct haptic_syscall_args {
    pub hid: u32,
    pub retval: i32,
}

pub const HID_FEATURE_REPORT: u32 = 0x03;
pub const HID_REQ_GET_REPORT: u32 = 0x01;
pub const HID_REQ_SET_REPORT: u32 = 0x09;

#[no_mangle]
pub unsafe extern "C" fn hid_event(hctx: *mut hid_bpf_ctx) -> i32 {
    let data = hid_bpf_get_data(hctx, 0, 9);

    if data.is_null() {
        return 0; /* EPERM check */
    }

    /* Touch */
    *data.add(1) &= 0xfd;

    /* X */
    *data.add(4) = 0;
    *data.add(5) = 0;

    /* Y */
    *data.add(6) = 0;
    *data.add(7) = 0;

    0
}

/* 72 == 360 / 5 -> 1 report every 5 degrees */
#[no_mangle]
pub static mut resolution: i32 = 72;
#[no_mangle]
pub static mut physical: i32 = 5;

static mut haptic_data: [u8; 8] = [0; 8];

#[no_mangle]
pub unsafe extern "C" fn set_haptic(args: *mut haptic_syscall_args) -> i32 {
    let ctx: *mut hid_bpf_ctx;
    let size: usize = core::mem::size_of::<[u8; 8]>();
    let res: *mut u16;
    let mut ret: i32;

    if size > core::mem::size_of::<[u8; 8]>() {
        return -7; /* -E2BIG */
    }

    ctx = hid_bpf_allocate_context((*args).hid);
    if ctx.is_null() {
        return -1; /* EPERM check */
    }

    haptic_data[0] = 1; /* report ID */

    ret = hid_bpf_hw_request(
        ctx,
        haptic_data.as_mut_ptr(),
        size,
        HID_FEATURE_REPORT,
        HID_REQ_GET_REPORT,
    );

    bpf_printk(b"probed/remove event ret value: %d\0".as_ptr(), ret);
    bpf_printk(b"buf: %02x %02x %02x\0".as_ptr(), haptic_data[0], haptic_data[1], haptic_data[2]);
    bpf_printk(b"     %02x %02x %02x\0".as_ptr(), haptic_data[3], haptic_data[4], haptic_data[5]);
    bpf_printk(b"     %02x %02x\0".as_ptr(), haptic_data[6], haptic_data[7]);

    /* whenever resolution multiplier is not 3600, we have the fixed report descriptor */
    res = haptic_data.as_mut_ptr().add(1) as *mut u16;
    if *res != 3600 {
        // haptic_data[1] = 72; /* resolution multiplier */
        // haptic_data[2] = 0;  /* resolution multiplier */
        // haptic_data[3] = 0;  /* Repeat Count */
        haptic_data[4] = 3; /* haptic Auto Trigger */
        // haptic_data[5] = 5;  /* Waveform Cutoff Time */
        // haptic_data[6] = 80; /* Retrigger Period */
        // haptic_data[7] = 0;  /* Retrigger Period */
    } else {
        haptic_data[4] = 0;
    }

    ret = hid_bpf_hw_request(
        ctx,
        haptic_data.as_mut_ptr(),
        size,
        HID_FEATURE_REPORT,
        HID_REQ_SET_REPORT,
    );

    bpf_printk(b"set haptic ret value: %d -> %d\0".as_ptr(), ret, haptic_data[4]);
    (*args).retval = ret;
    hid_bpf_release_context(ctx);
    0
}

/* Convert REL_DIAL into REL_WHEEL */
#[no_mangle]
pub unsafe extern "C" fn hid_rdesc_fixup(hctx: *mut hid_bpf_ctx) -> i32 {
    let data = hid_bpf_get_data(hctx, 0, 4096);

    if data.is_null() {
        return 0; /* EPERM check */
    }

    /* Convert TOUCH into a button */
    *data.add(31) = HID_UP_BUTTON as u8;
    *data.add(33) = 2;

    /* Convert REL_DIAL into REL_WHEEL */
    *data.add(45) = HID_GD_WHEEL as u8;

    /* Change Resolution Multiplier */
    *(data.add(61) as *mut u16) = physical as u16;
    *(data.add(66) as *mut u16) = resolution as u16;

    /* Convert X,Y from Abs to Rel */
    *data.add(88) = 0x06;
    *data.add(98) = 0x06;
    0
}

#[repr(C)]
pub struct hid_bpf_ops {
    pub hid_rdesc_fixup: *mut core::ffi::c_void,
    pub hid_device_event: *mut core::ffi::c_void,
}

#[no_mangle]
pub static mut surface_dial: hid_bpf_ops = hid_bpf_ops {
    hid_rdesc_fixup: hid_rdesc_fixup as *mut core::ffi::c_void,
    hid_device_event: hid_event as *mut core::ffi::c_void,
};

#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
#[no_mangle]
pub static mut _version: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
