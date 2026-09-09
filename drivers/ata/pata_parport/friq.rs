// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1998    Grant R. Guenther <grant@torque.net>
 *
 * friq.c is a low-level protocol driver for the Freecom "IQ"
 * parallel port IDE adapter. Early versions of this adapter
 * use the 'frpw' protocol.
 */

use core::ffi::c_void;

// Supplied by the kernel and pata_parport dependencies.
extern "C" {
    fn w0(value: i32);
    fn w2(value: i32);
    fn w4(value: i32);
    fn w4w(value: u16);
    fn w4l(value: u32);
    fn r0() -> i32;
    fn r1() -> i32;
    fn r2() -> i32;
    fn r4() -> i32;
    fn r4w() -> u16;
    fn r4l() -> u32;
    fn udelay(usecs: u32);
}

#[repr(C)]
pub struct PiAdapter {
    pub mode: i32,
    pub saved_r0: i32,
    pub saved_r2: i32,
    pub port: u32,
    pub delay: i32,
    pub private: i32,
    pub dev: c_void,
}

#[inline]
unsafe fn cmd(x: i32) {
    w2(4); w0(0xff); w0(0xff); w0(0x73); w0(0x73);
    w0(0xc9); w0(0xc9); w0(0x26);
    w0(0x26); w0(x); w0(x);
}

#[inline]
fn j44(l: i32, h: i32) -> i32 { ((l >> 4) & 0x0f) | (h & 0xf0) }

static CONT_MAP: [i32; 2] = [0x08, 0x10];

unsafe fn friq_read_regr(pi: *mut PiAdapter, cont: i32, regr: i32) -> i32 {
    let r = regr + CONT_MAP[cont as usize];
    cmd(r); w2(6); let l = r1(); w2(4); let h = r1(); w2(4);
    let _ = pi;
    j44(l, h)
}

unsafe fn friq_write_regr(_pi: *mut PiAdapter, cont: i32, regr: i32, val: i32) {
    let r = regr + CONT_MAP[cont as usize];
    cmd(r); w0(val); w2(5); w2(7); w2(5); w2(4);
}

unsafe fn friq_read_block_int(pi: *mut PiAdapter, buf: *mut i8, count: i32, regr: i32) {
    let mut ph: i32;
    match (*pi).mode {
        0 => { cmd(regr); for k in 0..count { w2(6); let l = r1(); w2(4); let h = r1(); *buf.offset(k as isize) = j44(l, h) as i8; } w2(4); }
        1 => { ph = 2; cmd(regr + 0xc0); w0(0xff); for k in 0..count { w2(0xa4 + ph); *buf.offset(k as isize) = r0() as i8; ph = 2 - ph; } w2(0xac); w2(0xa4); w2(4); }
        2 => { cmd(regr + 0x80); for k in 0..(count - 2) { *buf.offset(k as isize) = r4() as i8; } w2(0xac); w2(0xa4); *buf.offset((count - 2) as isize) = r4() as i8; *buf.offset((count - 1) as isize) = r4() as i8; w2(4); }
        3 => { cmd(regr + 0x80); for k in 0..(count / 2 - 1) { *(buf as *mut u16).offset(k as isize) = r4w(); } w2(0xac); w2(0xa4); *buf.offset((count - 2) as isize) = r4() as i8; *buf.offset((count - 1) as isize) = r4() as i8; w2(4); }
        4 => { cmd(regr + 0x80); for k in 0..(count / 4 - 1) { *(buf as *mut u32).offset(k as isize) = r4l(); } *buf.offset((count - 4) as isize) = r4() as i8; *buf.offset((count - 3) as isize) = r4() as i8; w2(0xac); w2(0xa4); *buf.offset((count - 2) as isize) = r4() as i8; *buf.offset((count - 1) as isize) = r4() as i8; w2(4); }
        _ => {}
    }
}

unsafe fn friq_read_block(pi: *mut PiAdapter, buf: *mut i8, count: i32) { friq_read_block_int(pi, buf, count, 0x08); }

unsafe fn friq_write_block(pi: *mut PiAdapter, buf: *mut i8, count: i32) {
    match (*pi).mode {
        0 | 1 => { cmd(8); w2(5); for k in 0..count { w0(*buf.offset(k as isize) as i32); w2(7); w2(5); } w2(4); }
        2 => { cmd(0xc8); w2(5); for k in 0..count { w4(*buf.offset(k as isize) as i32); } w2(4); }
        3 => { cmd(0xc8); w2(5); for k in 0..(count / 2) { w4w(*(buf as *mut u16).offset(k as isize)); } w2(4); }
        4 => { cmd(0xc8); w2(5); for k in 0..(count / 4) { w4l(*(buf as *mut u32).offset(k as isize)); } w2(4); }
        _ => {}
    }
}

unsafe fn friq_connect(pi: *mut PiAdapter) { (*pi).saved_r0 = r0(); (*pi).saved_r2 = r2(); w2(4); }
unsafe fn friq_disconnect(pi: *mut PiAdapter) { cmd(0x20); w0((*pi).saved_r0); w2((*pi).saved_r2); }

unsafe fn friq_test_proto(pi: *mut PiAdapter) -> i32 {
    let mut e = [0i32; 2]; let mut scratch = [0i8; 512];
    (*pi).saved_r0 = r0(); w0(0xff); udelay(20); cmd(0x3d); udelay(500); w0((*pi).saved_r0);
    friq_connect(pi);
    for j in 0..2 { friq_write_regr(pi, 0, 6, 0xa0 + j * 0x10); for k in 0..256 { friq_write_regr(pi, 0, 2, k ^ 0xaa); friq_write_regr(pi, 0, 3, k ^ 0x55); if friq_read_regr(pi, 0, 2) != (k ^ 0xaa) { e[j as usize] += 1; } } }
    friq_disconnect(pi); friq_connect(pi); friq_read_block_int(pi, scratch.as_mut_ptr(), 512, 0x10);
    let mut r = 0; for k in 0..128 { if scratch[k as usize] != k as i8 { r += 1; } }
    friq_disconnect(pi); r || ((e[0] != 0 && e[1] != 0) as i32)
}

unsafe fn friq_log_adapter(pi: *mut PiAdapter) { (*pi).private = 1; friq_connect(pi); cmd(0x9e); friq_disconnect(pi); }
unsafe fn friq_release_proto(pi: *mut PiAdapter) { if (*pi).private != 0 { friq_connect(pi); cmd(0x1d); cmd(0x1e); friq_disconnect(pi); (*pi).private = 0; } }

#[repr(C)]
pub struct PiProtocol {
    pub owner: *mut c_void,
    pub name: *const u8,
    pub max_mode: i32,
    pub epp_first: i32,
    pub default_delay: i32,
    pub max_units: i32,
    pub write_regr: unsafe fn(*mut PiAdapter, i32, i32, i32),
    pub read_regr: unsafe fn(*mut PiAdapter, i32, i32) -> i32,
    pub write_block: unsafe fn(*mut PiAdapter, *mut i8, i32),
    pub read_block: unsafe fn(*mut PiAdapter, *mut i8, i32),
    pub connect: unsafe fn(*mut PiAdapter),
    pub disconnect: unsafe fn(*mut PiAdapter),
    pub test_proto: unsafe fn(*mut PiAdapter) -> i32,
    pub log_adapter: unsafe fn(*mut PiAdapter),
    pub release_proto: unsafe fn(*mut PiAdapter),
}

pub static mut FRIQ: PiProtocol = PiProtocol {
    owner: core::ptr::null_mut(),
    name: b"friq\0".as_ptr(),
    max_mode: 5,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: friq_write_regr,
    read_regr: friq_read_regr,
    write_block: friq_write_block,
    read_block: friq_read_block,
    connect: friq_connect,
    disconnect: friq_disconnect,
    test_proto: friq_test_proto,
    log_adapter: friq_log_adapter,
    release_proto: friq_release_proto,
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Freecom IQ parallel port IDE adapter protocol driver");
// module_pata_parport_driver(friq);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
