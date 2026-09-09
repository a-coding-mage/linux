// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1996-1998  Grant R. Guenther <grant@torque.net>
 *
 * on20.c is a low-level protocol driver for the
 * Onspec 90c20 parallel to IDE adapter.
 */

// Linux kernel and pata_parport dependencies are supplied externally.

macro_rules! op {
    ($f:expr) => {{
        w2(4); w0($f); w2(5); w2(0xd);
        w2(5); w2(0xd); w2(5); w2(4);
    }};
}

macro_rules! vl {
    ($v:expr) => {{
        w2(4); w0($v); w2(5);
        w2(7); w2(5); w2(4);
    }};
}

#[inline]
fn j44(a: i32, b: i32) -> i32 {
    ((a >> 4) & 0x0f) | (b & 0xf0)
}

unsafe fn on20_read_regr(pi: *mut pi_adapter, cont: i32, regr: i32) -> i32 {
    let mut h: i32;
    let mut l: i32;
    let mut r: i32;

    r = (regr << 2) + 1 + cont;

    op!(1); vl!(r); op!(0);

    match (*pi).mode {
        0 => {
            w2(4); w2(6); l = r1();
            w2(4); w2(6); h = r1();
            w2(4); w2(6); w2(4); w2(6); w2(4);
            j44(l, h)
        }
        1 => {
            w2(4); w2(0x26); r = r0();
            w2(4); w2(0x26); w2(4);
            r
        }
        _ => -1,
    }
}

unsafe fn on20_write_regr(_pi: *mut pi_adapter, cont: i32, regr: i32, val: i32) {
    let r = (regr << 2) + 1 + cont;

    op!(1); vl!(r);
    op!(0); vl!(val);
    op!(0); vl!(val);
}

unsafe fn on20_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0();
    (*pi).saved_r2 = r2();

    w2(4); w0(0); w2(0xc); w2(4); w2(6); w2(4); w2(6); w2(4);
    if (*pi).mode != 0 {
        op!(2); vl!(8); op!(2); vl!(9);
    } else {
        op!(2); vl!(0); op!(2); vl!(8);
    }
}

unsafe fn on20_disconnect(pi: *mut pi_adapter) {
    w2(4); w0(7); w2(4); w2(0xc); w2(4);
    w0((*pi).saved_r0);
    w2((*pi).saved_r2);
}

unsafe fn on20_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let mut l: i32;
    let mut h: i32;

    op!(1); vl!(1); op!(0);

    for k in 0..count {
        if (*pi).mode != 0 {
            w2(4); w2(0x26); *buf.add(k as usize) = r0() as i8;
        } else {
            w2(6); l = r1(); w2(4);
            w2(6); h = r1(); w2(4);
            *buf.add(k as usize) = j44(l, h) as i8;
        }
    }
    w2(4);
}

unsafe fn on20_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    op!(1); vl!(1); op!(0);

    for k in 0..count {
        w2(5); w0(*buf.add(k as usize) as i32); w2(7);
    }
    w2(4);
}

unsafe fn on20_log_adapter(pi: *mut pi_adapter) {
    let mode_string: [*const u8; 2] = [b"4-bit\0".as_ptr(), b"8-bit\0".as_ptr()];

    dev_info(
        &(*pi).dev,
        b"OnSpec 90c20 at 0x%x, mode %d (%s), delay %d\n\0".as_ptr(),
        (*pi).port, (*pi).mode, mode_string[(*pi).mode as usize], (*pi).delay,
    );
}

static mut on20: pi_protocol = pi_protocol {
    owner: THIS_MODULE,
    name: b"on20\0".as_ptr(),
    max_mode: 2,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: Some(on20_write_regr),
    read_regr: Some(on20_read_regr),
    write_block: Some(on20_write_block),
    read_block: Some(on20_read_block),
    connect: Some(on20_connect),
    disconnect: Some(on20_disconnect),
    log_adapter: Some(on20_log_adapter),
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Onspec 90c20 parallel port IDE adapter protocol driver");
// module_pata_parport_driver(on20);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
