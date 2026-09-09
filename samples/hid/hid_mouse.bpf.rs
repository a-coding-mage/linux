// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by vmlinux.h, bpf helpers, bpf tracing helpers, and
// hid_bpf_helpers.h are intentionally referenced but not reimplemented here.

use core::ffi::c_void;

extern "C" {
    fn hid_bpf_get_data(hctx: *mut hid_bpf_ctx, offset: u32, size: u32) -> *mut u8;
    fn bpf_printk(fmt: *const u8, ...);
}

#[repr(C)]
pub struct hid_bpf_ctx {
    pub size: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hid_report_type {
    _Opaque = 0,
}

unsafe fn hid_y_event(hctx: *mut hid_bpf_ctx) -> i32 {
    let mut y: i16;
    let data = hid_bpf_get_data(hctx, 0 /* offset */, 9 /* size */);

    if data.is_null() {
        return 0; /* EPERM check */
    }

    bpf_printk(b"event: size: %d\0".as_ptr(), (*hctx).size);
    bpf_printk(
        b"incoming event: %02x %02x %02x\0".as_ptr(),
        *data.add(0),
        *data.add(1),
        *data.add(2),
    );
    bpf_printk(
        b"                %02x %02x %02x\0".as_ptr(),
        *data.add(3),
        *data.add(4),
        *data.add(5),
    );
    bpf_printk(
        b"                %02x %02x %02x\0".as_ptr(),
        *data.add(6),
        *data.add(7),
        *data.add(8),
    );

    y = (*data.add(3) as i16) | ((*data.add(4) as i16) << 8);
    y = -y;
    *data.add(3) = (y as u16 & 0xFF) as u8;
    *data.add(4) = ((y >> 8) as u16 & 0xFF) as u8;

    bpf_printk(
        b"modified event: %02x %02x %02x\0".as_ptr(),
        *data.add(0),
        *data.add(1),
        *data.add(2),
    );
    bpf_printk(
        b"                %02x %02x %02x\0".as_ptr(),
        *data.add(3),
        *data.add(4),
        *data.add(5),
    );
    bpf_printk(
        b"                %02x %02x %02x\0".as_ptr(),
        *data.add(6),
        *data.add(7),
        *data.add(8),
    );
    0
}

unsafe fn hid_x_event(hctx: *mut hid_bpf_ctx) -> i32 {
    let mut x: i16;
    let data = hid_bpf_get_data(hctx, 0 /* offset */, 9 /* size */);
    if data.is_null() {
        return 0; /* EPERM check */
    }
    x = (*data.add(1) as i16) | ((*data.add(2) as i16) << 8);
    x = -x;
    *data.add(1) = (x as u16 & 0xFF) as u8;
    *data.add(2) = ((x >> 8) as u16 & 0xFF) as u8;
    0
}

#[no_mangle]
pub unsafe extern "C" fn hid_event(
    hctx: *mut hid_bpf_ctx,
    _type: hid_report_type,
) -> i32 {
    let ret = hid_y_event(hctx);
    if ret != 0 { return ret; }
    hid_x_event(hctx)
}

#[no_mangle]
pub unsafe extern "C" fn hid_rdesc_fixup(hctx: *mut hid_bpf_ctx) -> i32 {
    let data = hid_bpf_get_data(hctx, 0 /* offset */, 4096 /* size */);
    if data.is_null() { return 0; /* EPERM check */ }

    bpf_printk(b"rdesc: %02x %02x %02x\0".as_ptr(), *data, *data.add(1), *data.add(2));
    bpf_printk(b"       %02x %02x %02x\0".as_ptr(), *data.add(3), *data.add(4), *data.add(5));
    bpf_printk(b"       %02x %02x %02x ...\0".as_ptr(), *data.add(6), *data.add(7), *data.add(8));

    /* Swap the X and Y usages in the original report descriptor. */
    *data.add(39) = 0x31;
    *data.add(41) = 0x30;
    0
}

#[repr(C)]
pub struct hid_bpf_ops {
    pub hid_rdesc_fixup: *mut c_void,
    pub hid_device_event: *mut c_void,
}

#[no_mangle]
pub static mut mouse_invert: hid_bpf_ops = hid_bpf_ops {
    hid_rdesc_fixup: hid_rdesc_fixup as *mut c_void,
    hid_device_event: hid_event as *mut c_void,
};

#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
