// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-1998  Grant R. Guenther <grant@torque.net>
 *
 * comm.c is a low-level protocol driver for some older models of the DataStor
 * "Commuter" parallel to IDE adapter. Some of the parallel port devices
 * marketed by Arista currently use this adapter.
 */

// Dependency declarations supplied by the surrounding kernel translation.

/*
 * mode codes:  0  nybble reads, 8-bit writes
 *              1  8-bit reads and writes
 *              2  8-bit EPP mode
 */

#[inline]
unsafe fn j44(a: i32, b: i32) -> i32 {
    ((a >> 3) & 0x0f) | ((b << 1) & 0xf0)
}

/* P1 and P2 are sequences of w2 calls. */

/*
 * cont = 0 - access the IDE register file
 * cont = 1 - access the IDE command set
 */
static mut CONT_MAP: [i32; 2] = [0x08, 0x10];

unsafe fn comm_read_regr(pi: *mut pi_adapter, cont: i32, regr: i32) -> i32 {
    let r = regr + CONT_MAP[cont as usize];
    let mut l: i32;
    let mut h: i32;

    match (*pi).mode {
        0 => {
            w0(r); w2(5); w2(0xd); w2(0xd); w2(5); w2(4); w0(0);
            w2(6); l = r1(); w0(0x80); h = r1(); w2(4);
            j44(l, h)
        }
        1 => {
            w0(r + 0x20); w2(5); w2(0xd); w2(0xd); w2(5); w2(4);
            w0(0); w2(0x26); h = r0(); w2(4);
            h
        }
        2 | 3 | 4 => {
            w3(r + 0x20); let _ = r1();
            w2(0x24); h = r4(); w2(4);
            h
        }
        _ => -1,
    }
}

unsafe fn comm_write_regr(pi: *mut pi_adapter, cont: i32, regr: i32, val: i32) {
    let r = regr + CONT_MAP[cont as usize];
    match (*pi).mode {
        0 | 1 => {
            w0(r); w2(5); w2(0xd); w2(0xd); w2(5); w2(4); w0(val);
            w2(5); w2(7); w2(7); w2(5); w2(4);
        }
        2 | 3 | 4 => {
            w3(r); let _ = r1(); w4(val);
        }
        _ => {}
    }
}

unsafe fn comm_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0();
    (*pi).saved_r2 = r2();
    w2(4); w0(0xff); w2(6);
    w2(4); w0(0xaa); w2(6);
    w2(4); w0(0x00); w2(6);
    w2(4); w0(0x87); w2(6);
    w2(4); w0(0xe0); w2(0xc); w2(0xc); w2(4);
}

unsafe fn comm_disconnect(pi: *mut pi_adapter) {
    w2(0); w2(0); w2(0); w2(4);
    w0((*pi).saved_r0);
    w2((*pi).saved_r2);
}

unsafe fn comm_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    match (*pi).mode {
        0 => {
            w0(0x48); w2(5); w2(0xd); w2(0xd); w2(5); w2(4);
            for i in 0..count {
                w0(0); w2(6); let l = r1();
                w0(0x80); let h = r1(); w2(4);
                *buf.offset(i as isize) = j44(l, h) as i8;
            }
        }
        1 => {
            w0(0x68); w2(5); w2(0xd); w2(0xd); w2(5); w2(4); w0(0);
            for i in 0..count {
                w2(0x26); *buf.offset(i as isize) = r0() as i8; w2(0x24);
            }
            w2(4);
        }
        2 => {
            w3(0x68); let _ = r1(); w2(0x24);
            for i in 0..count { *buf.offset(i as isize) = r4() as i8; }
            w2(4);
        }
        3 => {
            w3(0x68); let _ = r1(); w2(0x24);
            for i in 0..(count / 2) { *(buf as *mut u16).offset(i as isize) = r4w(); }
            w2(4);
        }
        4 => {
            w3(0x68); let _ = r1(); w2(0x24);
            for i in 0..(count / 4) { *(buf as *mut u32).offset(i as isize) = r4l(); }
            w2(4);
        }
        _ => {}
    }
}

/* NB: Watch out for the byte swapped writes ! */
unsafe fn comm_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    match (*pi).mode {
        0 | 1 => {
            w0(0x68); w2(5); w2(0xd); w2(0xd); w2(5); w2(4);
            for k in 0..count { w2(5); w0(*buf.offset((k ^ 1) as isize) as i32); w2(7); }
            w2(5); w2(4);
        }
        2 => {
            w3(0x48); let _ = r1();
            for k in 0..count { w4(*buf.offset((k ^ 1) as isize) as i32); }
        }
        3 => {
            w3(0x48); let _ = r1();
            for k in 0..(count / 2) { w4w(swab16((*(buf as *mut u16).offset(k as isize)) as i32) as u16); }
        }
        4 => {
            w3(0x48); let _ = r1();
            for k in 0..(count / 4) {
                let v = swab16((*(buf as *mut u16).offset((2 * k) as isize)) as i32)
                    | (swab16((*(buf as *mut u16).offset((2 * k + 1) as isize)) as i32) << 16);
                w4l(v);
            }
        }
        _ => {}
    }
}

unsafe fn comm_log_adapter(pi: *mut pi_adapter) {
    let mode_string: [&str; 5] = ["4-bit", "8-bit", "EPP-8", "EPP-16", "EPP-32"];
    dev_info(&mut (*pi).dev,
        "DataStor Commuter at 0x%x, mode %d (%s), delay %d\n",
        (*pi).port, (*pi).mode, mode_string[(*pi).mode as usize], (*pi).delay);
}

static mut COMM: pi_protocol = pi_protocol {
    owner: THIS_MODULE,
    name: "comm",
    max_mode: 5,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: Some(comm_write_regr),
    read_regr: Some(comm_read_regr),
    write_block: Some(comm_write_block),
    read_block: Some(comm_read_block),
    connect: Some(comm_connect),
    disconnect: Some(comm_disconnect),
    log_adapter: Some(comm_log_adapter),
};

MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Grant R. Guenther <grant@torque.net>");
MODULE_DESCRIPTION!("DataStor Commuter parallel port IDE adapter protocol driver");
module_pata_parport_driver!(COMM);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
