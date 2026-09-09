// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-8  Grant R. Guenther <grant@torque.net>
 *
 * aten.c is a low-level protocol driver for the ATEN EH-100
 * parallel port adapter.  The EH-100 supports 4-bit and 8-bit
 * modes only.  There is also an EH-132 which supports EPP mode
 * transfers.  The EH-132 is not yet supported.
 */

// C dependencies supplied by the surrounding kernel/driver translation.

#[inline]
fn j44(a: i32, b: i32) -> i32 {
    (((((a >> 4) & 0x0f) | (b & 0xf0)) ^ 0x88))
}

/*
 * cont = 0 - access the IDE register file
 * cont = 1 - access the IDE command set
 */
static mut CONT_MAP: [i32; 2] = [0x08, 0x20];

unsafe fn aten_write_regr(pi: *mut pi_adapter, cont: i32, regr: i32, val: i32) {
    let r = regr + CONT_MAP[cont as usize] + 0x80;

    w0(pi, r); w2(pi, 0xe); w2(pi, 6); w0(pi, val); w2(pi, 7); w2(pi, 6); w2(pi, 0xc);
}

unsafe fn aten_read_regr(pi: *mut pi_adapter, cont: i32, regr: i32) -> i32 {
    let mut a: i32;
    let mut b: i32;
    let mut r: i32;

    r = regr + CONT_MAP[cont as usize] + 0x40;

    match (*pi).mode {
        0 => {
            w0(pi, r); w2(pi, 0xe); w2(pi, 6);
            w2(pi, 7); w2(pi, 6); w2(pi, 0);
            a = r1(pi); w0(pi, 0x10); b = r1(pi); w2(pi, 0xc);
            j44(a, b)
        }
        1 => {
            r |= 0x10;
            w0(pi, r); w2(pi, 0xe); w2(pi, 6); w0(pi, 0xff);
            w2(pi, 0x27); w2(pi, 0x26); w2(pi, 0x20);
            a = r0(pi);
            w2(pi, 0x26); w2(pi, 0xc);
            a
        }
        _ => -1,
    }
}

unsafe fn aten_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;
    let mut d: i32;

    match (*pi).mode {
        0 => {
            w0(pi, 0x48); w2(pi, 0xe); w2(pi, 6);
            for k in 0..(count / 2) {
                w2(pi, 7); w2(pi, 6); w2(pi, 2);
                a = r1(pi); w0(pi, 0x58); b = r1(pi);
                w2(pi, 0); d = r1(pi); w0(pi, 0x48); c = r1(pi);
                *buf.offset((2 * k) as isize) = j44(c, d) as i8;
                *buf.offset((2 * k + 1) as isize) = j44(a, b) as i8;
            }
            w2(pi, 0xc);
        }
        1 => {
            w0(pi, 0x58); w2(pi, 0xe); w2(pi, 6);
            for k in 0..(count / 2) {
                w2(pi, 0x27); w2(pi, 0x26); w2(pi, 0x22);
                a = r0(pi); w2(pi, 0x20); b = r0(pi);
                *buf.offset((2 * k) as isize) = b as i8;
                *buf.offset((2 * k + 1) as isize) = a as i8;
            }
            w2(pi, 0x26); w2(pi, 0xc);
        }
        _ => {}
    }
}

unsafe fn aten_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    w0(pi, 0x88); w2(pi, 0xe); w2(pi, 6);
    for k in 0..(count / 2) {
        w0(pi, *buf.offset((2 * k + 1) as isize) as i32); w2(pi, 0xe); w2(pi, 6);
        w0(pi, *buf.offset((2 * k) as isize) as i32); w2(pi, 7); w2(pi, 6);
    }
    w2(pi, 0xc);
}

unsafe fn aten_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0(pi);
    (*pi).saved_r2 = r2(pi);
    w2(pi, 0xc);
}

unsafe fn aten_disconnect(pi: *mut pi_adapter) {
    w0(pi, (*pi).saved_r0);
    w2(pi, (*pi).saved_r2);
}

unsafe fn aten_log_adapter(pi: *mut pi_adapter) {
    let mode_string: [*const u8; 2] = [b"4-bit\0".as_ptr(), b"8-bit\0".as_ptr()];

    dev_info(&(*pi).dev,
        b"ATEN EH-100 at 0x%x, mode %d (%s), delay %d\n\0".as_ptr(),
        (*pi).port, (*pi).mode, mode_string[(*pi).mode as usize], (*pi).delay);
}

static mut ATEN: pi_protocol = pi_protocol {
    owner: THIS_MODULE,
    name: b"aten\0".as_ptr(),
    max_mode: 2,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: Some(aten_write_regr),
    read_regr: Some(aten_read_regr),
    write_block: Some(aten_write_block),
    read_block: Some(aten_read_block),
    connect: Some(aten_connect),
    disconnect: Some(aten_disconnect),
    log_adapter: Some(aten_log_adapter),
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("ATEN EH-100 parallel port IDE adapter protocol driver");
module_pata_parport_driver!(ATEN);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
