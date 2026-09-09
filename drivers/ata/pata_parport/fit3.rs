// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1998  Grant R. Guenther <grant@torque.net>
 *
 * fit3.c is a low-level protocol driver for newer models
 * of the Fidelity International Technology parallel port adapter.
 * This adapter is used in their TransDisk 3000 portable
 * hard-drives, as well as CD-ROM, PD-CD and other devices.
 *
 * The TD-2000 and certain older devices use a different protocol.
 * Try the fit2 protocol module with them.
 */

// Kernel and pata_parport dependencies are supplied by other files.

#[repr(C)]
pub struct pi_adapter {
    pub mode: i32,
    pub saved_r0: i32,
    pub saved_r2: i32,
    pub port: u16,
    pub delay: i32,
    pub dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct pi_protocol {
    pub owner: *mut core::ffi::c_void,
    pub name: *const u8,
    pub max_mode: i32,
    pub epp_first: i32,
    pub default_delay: i32,
    pub max_units: i32,
    pub write_regr: unsafe fn(*mut pi_adapter, i32, i32, i32),
    pub read_regr: unsafe fn(*mut pi_adapter, i32, i32) -> i32,
    pub write_block: unsafe fn(*mut pi_adapter, *mut i8, i32),
    pub read_block: unsafe fn(*mut pi_adapter, *mut i8, i32),
    pub connect: unsafe fn(*mut pi_adapter),
    pub disconnect: unsafe fn(*mut pi_adapter),
    pub log_adapter: unsafe fn(*mut pi_adapter),
}

#[inline]
fn j44(a: i32, b: i32) -> i32 {
    ((a >> 3) & 0x0f) | ((b << 1) & 0xf0)
}

extern "C" {
    fn out_p(port: u16, value: u8);
    fn in_p(port: u16) -> u8;
    fn w0(value: i32);
    fn w2(value: i32);
    fn w4(value: i32);
    fn r0() -> i32;
    fn r1() -> i32;
    fn r2() -> i32;
    fn r4() -> i32;
}

#[inline]
unsafe fn w7(byte: i32) { out_p(7, byte as u8); }

#[inline]
unsafe fn r7() -> i32 { (in_p(7) as i32) & 0xff }

// cont = 0 - access the IDE register file
// cont = 1 - access the IDE command set

unsafe fn fit3_write_regr(pi: *mut pi_adapter, mut cont: i32, mut regr: i32, val: i32) {
    regr += cont << 3;
    match (*pi).mode {
        0 | 1 => {
            w2(0xc); w0(regr); w2(0x8); w2(0xc);
            w0(val); w2(0xd);
            w0(0); w2(0xc);
        }
        2 => {
            w2(0xc); w0(regr); w2(0x8); w2(0xc);
            w4(val); w4(0);
            w2(0xc);
        }
        _ => {}
    }
}

unsafe fn fit3_read_regr(pi: *mut pi_adapter, cont: i32, mut regr: i32) -> i32 {
    let (mut a, mut b): (i32, i32);
    regr += cont << 3;
    match (*pi).mode {
        0 => {
            w2(0xc); w0(regr + 0x10); w2(0x8); w2(0xc);
            w2(0xd); a = r1();
            w2(0xf); b = r1();
            w2(0xc);
            j44(a, b)
        }
        1 => {
            w2(0xc); w0(regr + 0x90); w2(0x8); w2(0xc);
            w2(0xec); w2(0xee); w2(0xef); a = r0();
            w2(0xc);
            a
        }
        2 => {
            w2(0xc); w0(regr + 0x90); w2(0x8); w2(0xc);
            w2(0xec);
            a = r4(); b = r4();
            w2(0xc);
            a
        }
        _ => -1,
    }
}

unsafe fn fit3_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let (mut a, mut b, mut c, mut d): (i32, i32, i32, i32);
    match (*pi).mode {
        0 => {
            w2(0xc); w0(0x10); w2(0x8); w2(0xc);
            for k in 0..(count / 2) {
                w2(0xd); a = r1(); w2(0xf); b = r1();
                w2(0xc); c = r1(); w2(0xe); d = r1();
                *buf.add((2 * k) as usize) = j44(a, b) as i8;
                *buf.add((2 * k + 1) as usize) = j44(c, d) as i8;
            }
            w2(0xc);
        }
        1 => {
            w2(0xc); w0(0x90); w2(0x8); w2(0xc); w2(0xec); w2(0xee);
            for k in 0..(count / 2) {
                w2(0xef); a = r0(); w2(0xee); b = r0();
                *buf.add((2 * k) as usize) = a as i8;
                *buf.add((2 * k + 1) as usize) = b as i8;
            }
            w2(0xec); w2(0xc);
        }
        2 => {
            w2(0xc); w0(0x90); w2(0x8); w2(0xc); w2(0xec);
            for k in 0..count { *buf.add(k as usize) = r4() as i8; }
            w2(0xc);
        }
        _ => {}
    }
}

unsafe fn fit3_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    w2(0xc); w0(0); w2(0x8); w2(0xc);
    match (*pi).mode {
        0 | 1 => {
            for k in 0..(count / 2) {
                w0(*buf.add((2 * k) as usize) as i32); w2(0xd);
                w0(*buf.add((2 * k + 1) as usize) as i32); w2(0xc);
            }
        }
        2 => {
            for k in 0..count { w4(*buf.add(k as usize) as i32); }
            w2(0xc);
        }
        _ => {}
    }
}

unsafe fn fit3_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0();
    (*pi).saved_r2 = r2();
    w2(0xc); w0(0); w2(0xa);
    if (*pi).mode == 2 { w2(0xc); w0(0x9); w2(0x8); w2(0xc); }
}

unsafe fn fit3_disconnect(pi: *mut pi_adapter) {
    w2(0xc); w0(0xa); w2(0x8); w2(0xc);
    w0((*pi).saved_r0); w2((*pi).saved_r2);
}

unsafe fn fit3_log_adapter(_pi: *mut pi_adapter) {
    // dev_info(&pi->dev, "FIT 3000 adapter at 0x%x, mode %d (%s), delay %d\n", ...)
}

static mut fit3: pi_protocol = pi_protocol {
    owner: core::ptr::null_mut(),
    name: b"fit3\0".as_ptr(),
    max_mode: 3,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: fit3_write_regr,
    read_regr: fit3_read_regr,
    write_block: fit3_write_block,
    read_block: fit3_read_block,
    connect: fit3_connect,
    disconnect: fit3_disconnect,
    log_adapter: fit3_log_adapter,
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Fidelity International Technology parallel port IDE adapter (newer models) protocol driver");
// module_pata_parport_driver(fit3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
