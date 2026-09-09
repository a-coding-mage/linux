// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1996-1998  Grant R. Guenther <grant@torque.net>
 *
 * frpw.c is a low-level protocol driver for the Freecom "Power" parallel port
 * IDE adapter.
 */

// Linux kernel dependencies and "pata_parport.h" are supplied externally.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct pi_adapter {
    pub mode: c_int,
    pub delay: c_int,
    pub saved_r0: c_int,
    pub saved_r2: c_int,
    pub private: isize,
    pub port: c_int,
    pub dev: c_void,
}

extern "C" {
    fn r0() -> c_int;
    fn r1() -> c_int;
    fn r2() -> c_int;
    fn r4() -> u8;
    fn r4w() -> u16;
    fn r4l() -> u32;
    fn w0(v: c_int);
    fn w2(v: c_int);
    fn w4(v: u8);
    fn w4w(v: u16);
    fn w4l(v: u32);
    fn udelay(v: c_int);
    fn mdelay(v: c_int);
    fn dev_dbg(dev: *const c_void, fmt: *const c_char, ...);
    fn dev_info(dev: *const c_void, fmt: *const c_char, ...);
}

#[inline]
unsafe fn cec4() {
    w2(0xc); w2(0xe); w2(0xe); w2(0xc); w2(4); w2(4); w2(4);
}

#[inline]
fn j44(l: c_int, h: c_int) -> c_int { ((l >> 4) & 0x0f) | (h & 0xf0) }

static CONT_MAP: [c_int; 2] = [0x08, 0x10];

unsafe fn frpw_read_regr(_pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    let r = regr + CONT_MAP[cont as usize];
    w2(4); w0(r); cec4();
    w2(6); let l = r1();
    w2(4); let h = r1();
    w2(4);
    j44(l, h)
}

unsafe fn frpw_write_regr(_pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    let r = regr + CONT_MAP[cont as usize];
    w2(4); w0(r); cec4(); w0(val);
    w2(5); w2(7); w2(5); w2(4);
}

unsafe fn frpw_read_block_int(pi: *mut pi_adapter, buf: *mut c_char, count: c_int, regr: c_int) {
    match (*pi).mode {
        0 => { w2(4); w0(regr); cec4(); for k in 0..count { w2(6); let l=r1(); w2(4); let h=r1(); *buf.add(k as usize)=j44(l,h) as c_char; } w2(4); }
        1 => { let mut ph=2; w2(4); w0(regr+0xc0); cec4(); w0(0xff); for k in 0..count { w2(0xa4+ph); *buf.add(k as usize)=r0() as c_char; ph=2-ph; } w2(0xac); w2(0xa4); w2(4); }
        2 => { w2(4); w0(regr+0x80); cec4(); for k in 0..count { *buf.add(k as usize)=r4() as c_char; } w2(0xac); w2(0xa4); w2(4); }
        3 => { w2(4); w0(regr+0x80); cec4(); for k in 0..count-2 { *buf.add(k as usize)=r4() as c_char; } w2(0xac); w2(0xa4); *buf.add((count-2) as usize)=r4() as c_char; *buf.add((count-1) as usize)=r4() as c_char; w2(4); }
        4 => { w2(4); w0(regr+0x80); cec4(); for k in 0..count/2-1 { *(buf as *mut u16).add(k as usize)=r4w(); } w2(0xac); w2(0xa4); *buf.add((count-2) as usize)=r4() as c_char; *buf.add((count-1) as usize)=r4() as c_char; w2(4); }
        5 => { w2(4); w0(regr+0x80); cec4(); for k in 0..count/4-1 { *(buf as *mut u32).add(k as usize)=r4l(); } *buf.add((count-4) as usize)=r4() as c_char; *buf.add((count-3) as usize)=r4() as c_char; w2(0xac); w2(0xa4); *buf.add((count-2) as usize)=r4() as c_char; *buf.add((count-1) as usize)=r4() as c_char; w2(4); }
        _ => {}
    }
}

unsafe fn frpw_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) { frpw_read_block_int(pi, buf, count, 0x08); }

unsafe fn frpw_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    match (*pi).mode {
        0|1|2 => { w2(4); w0(8); cec4(); w2(5); for k in 0..count { w0(*buf.add(k as usize) as c_int); w2(7); w2(5); } w2(4); }
        3 => { w2(4); w0(0xc8); cec4(); w2(5); for k in 0..count { w4(*buf.add(k as usize) as u8); } w2(4); }
        4 => { w2(4); w0(0xc8); cec4(); w2(5); for k in 0..count/2 { w4w(*(buf as *mut u16).add(k as usize)); } w2(4); }
        5 => { w2(4); w0(0xc8); cec4(); w2(5); for k in 0..count/4 { w4l(*(buf as *mut u32).add(k as usize)); } w2(4); }
        _ => {}
    }
}

unsafe fn frpw_connect(pi: *mut pi_adapter) { (*pi).saved_r0=r0(); (*pi).saved_r2=r2(); w2(4); }
unsafe fn frpw_disconnect(pi: *mut pi_adapter) { w2(4); w0(0x20); cec4(); w0((*pi).saved_r0); w2((*pi).saved_r2); }

unsafe fn frpw_test_pnp(pi: *mut pi_adapter) -> c_int {
    // #ifdef FRPW_HARD_RESET: w0(0); w2(8); udelay(50); w2(0xc); mdelay(1500);
    let olddelay=(*pi).delay; (*pi).delay=10;
    (*pi).saved_r0=r0(); (*pi).saved_r2=r2();
    w2(4); w0(4); w2(6); w2(7); let a=r1()&0xff; w2(4); let b=r1()&0xff; w2(0xc); w2(0xe); w2(4);
    (*pi).delay=olddelay; w0((*pi).saved_r0); w2((*pi).saved_r2);
    (((!a)&0x40) != 0 && (b&0x40) != 0) as c_int
}

unsafe fn frpw_test_proto(pi: *mut pi_adapter) -> c_int {
    let mut e=[0,0]; let mut scratch=[0i8;512];
    if ((*pi).private >> 1) != (*pi).port as isize { (*pi).private=frpw_test_pnp(pi) as isize+2*(*pi).port as isize; }
    if ((*pi).private&1)==0 && (*pi).mode>2 { return 1; }
    if ((*pi).private&1)==1 && (*pi).mode==2 { return 1; }
    frpw_connect(pi);
    for j in 0..2 { frpw_write_regr(pi,0,6,0xa0+j*0x10); for k in 0..256 { frpw_write_regr(pi,0,2,k^0xaa); frpw_write_regr(pi,0,3,k^0x55); if frpw_read_regr(pi,0,2)!=(k^0xaa) { e[j as usize]+=1; } } }
    frpw_disconnect(pi); frpw_connect(pi); frpw_read_block_int(pi,scratch.as_mut_ptr(),512,0x10); let mut r=0; for k in 0..128 { if scratch[k]!=k as i8 { r+=1; } } frpw_disconnect(pi);
    (r != 0 || (e[0] != 0 && e[1] != 0)) as c_int
}

unsafe fn frpw_log_adapter(pi: *mut pi_adapter) {
    let _mode=["4-bit","8-bit","EPP","EPP-8","EPP-16","EPP-32"];
    let _ = (pi, _mode);
}

// The pi_protocol initializer and module metadata are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
