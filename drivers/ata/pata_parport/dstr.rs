// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-1998  Grant R. Guenther <grant@torque.net>
 *
 * dstr.c is a low-level protocol driver for the DataStor EP2000 parallel
 * to IDE adapter chip.
 */

// Linux kernel and pata_parport dependencies are supplied by other files.

macro_rules! j44 {
    ($a:expr, $b:expr) => {
        ((($a >> 3) & 0x07) | ((!$a >> 4) & 0x08) |
            (($b << 1) & 0x70) | ((!$b) & 0x80))
    };
}

macro_rules! p1 {
    () => {{ w2(5); w2(0xd); w2(5); w2(4); }};
}
macro_rules! p2 {
    () => {{ w2(5); w2(7); w2(5); w2(4); }};
}
macro_rules! p3 {
    () => {{ w2(6); w2(4); w2(6); w2(4); }};
}

// cont = 0 - access the IDE register file
// cont = 1 - access the IDE command set
static mut CONT_MAP: [i32; 2] = [0x20, 0x40];

unsafe fn dstr_read_regr(pi: *mut pi_adapter, cont: i32, regr: i32) -> i32 {
    let (mut a, mut b, r);

    r = regr + CONT_MAP[cont as usize];

    w0(0x81); p1!();
    if (*pi).mode != 0 { w0(0x11); } else { w0(1); }
    p2!(); w0(r); p1!();

    match (*pi).mode {
        0 => {
            w2(6); a = r1(); w2(4); w2(6); b = r1(); w2(4);
            j44!(a, b)
        }
        1 => {
            w0(0); w2(0x26); a = r0(); w2(4); a
        }
        2 | 3 | 4 => {
            w2(0x24); a = r4(); w2(4); a
        }
        _ => -1,
    }
}

unsafe fn dstr_write_regr(pi: *mut pi_adapter, cont: i32, regr: i32, val: i32) {
    let r = regr + CONT_MAP[cont as usize];

    w0(0x81); p1!();
    if (*pi).mode >= 2 { w0(0x11); } else { w0(1); }
    p2!(); w0(r); p1!();

    match (*pi).mode {
        0 | 1 => { w0(val); w2(5); w2(7); w2(5); w2(4); }
        2 | 3 | 4 => { w4(val); }
        _ => {}
    }
}

macro_rules! ccp {
    ($x:expr) => {{
        w0(0xff); w2(0xc); w2(4);
        w0(0xaa); w0(0x55); w0(0); w0(0xff);
        w0(0x87); w0(0x78);
        w0($x); w2(5); w2(4);
    }};
}

unsafe fn dstr_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0();
    (*pi).saved_r2 = r2();
    w2(4); ccp!(0xe0); w0(0xff);
}

unsafe fn dstr_disconnect(pi: *mut pi_adapter) {
    ccp!(0x30);
    w0((*pi).saved_r0);
    w2((*pi).saved_r2);
}

unsafe fn dstr_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let (mut k, mut a, mut b);

    w0(0x81); p1!();
    if (*pi).mode != 0 { w0(0x19); } else { w0(9); }
    p2!(); w0(0x82); p1!(); p3!(); w0(0x20); p1!();

    match (*pi).mode {
        0 => for k in 0..count { w2(6); a = r1(); w2(4); w2(6); b = r1(); w2(4); *buf.offset(k as isize) = j44!(a, b) as i8; },
        1 => { w0(0); for k in 0..count { w2(0x26); *buf.offset(k as isize) = r0() as i8; w2(0x24); } w2(4); }
        2 => { w2(0x24); for k in 0..count { *buf.offset(k as isize) = r4() as i8; } w2(4); }
        3 => { w2(0x24); for k in 0..count / 2 { *(buf as *mut u16).offset(k as isize) = r4w(); } w2(4); }
        4 => { w2(0x24); for k in 0..count / 4 { *(buf as *mut u32).offset(k as isize) = r4l(); } w2(4); }
        _ => {}
    }
}

unsafe fn dstr_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    w0(0x81); p1!();
    if (*pi).mode != 0 { w0(0x19); } else { w0(9); }
    p2!(); w0(0x82); p1!(); p3!(); w0(0x20); p1!();

    match (*pi).mode {
        0 | 1 => { for k in 0..count { w2(5); w0(*buf.offset(k as isize) as i32); w2(7); } w2(5); w2(4); }
        2 => { w2(0xc5); for k in 0..count { w4(*buf.offset(k as isize) as i32); } w2(0xc4); }
        3 => { w2(0xc5); for k in 0..count / 2 { w4w(*(buf as *mut u16).offset(k as isize)); } w2(0xc4); }
        4 => { w2(0xc5); for k in 0..count / 4 { w4l(*(buf as *mut u32).offset(k as isize)); } w2(0xc4); }
        _ => {}
    }
}

unsafe fn dstr_log_adapter(pi: *mut pi_adapter) {
    let mode_string: [&str; 5] = ["4-bit", "8-bit", "EPP-8", "EPP-16", "EPP-32"];
    dev_info(&(*pi).dev, "DataStor EP2000 at 0x%x, mode %d (%s), delay %d\n", (*pi).port, (*pi).mode, mode_string[(*pi).mode as usize], (*pi).delay);
}

static mut DSTR: pi_protocol = pi_protocol {
    owner: THIS_MODULE,
    name: "dstr",
    max_mode: 5,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: Some(dstr_write_regr),
    read_regr: Some(dstr_read_regr),
    write_block: Some(dstr_write_block),
    read_block: Some(dstr_read_block),
    connect: Some(dstr_connect),
    disconnect: Some(dstr_disconnect),
    log_adapter: Some(dstr_log_adapter),
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("DataStor EP2000 parallel port IDE adapter protocol driver");
// module_pata_parport_driver(DSTR);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
