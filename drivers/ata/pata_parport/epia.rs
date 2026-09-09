// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-1998  Grant R. Guenther <grant@torque.net>
 *
 * epia.c is a low-level protocol driver for Shuttle Technologies
 * EPIA parallel to IDE adapter chip.  This device is now obsolete
 * and has been replaced with the EPAT chip, which is supported
 * by epat.c, however, some devices based on EPIA are still
 * available.
 */

// Linux kernel and pata_parport dependencies are supplied externally.

#[repr(C)]
pub struct pi_adapter {
    pub dev: core::ffi::c_void,
    pub mode: i32,
    pub port: i32,
    pub delay: i32,
    pub saved_r0: i32,
    pub saved_r2: i32,
}

extern "C" {
    fn w0(value: i32);
    fn w2(value: i32);
    fn w3(value: i32);
    fn w4(value: i32);
    fn w4w(value: u16);
    fn w4l(value: u32);
    fn r0() -> i32;
    fn r1() -> i32;
    fn r2() -> i32;
    fn r4() -> i32;
    fn r4w() -> u16;
    fn r4l() -> u32;
}

#[inline]
fn j44(a: i32, b: i32) -> i32 { ((a >> 4) & 0x0f) + (b & 0xf0) }
#[inline]
fn j53(a: i32, b: i32) -> i32 { ((a >> 3) & 0x1f) + ((b << 4) & 0xe0) }

static CONT_MAP: [i32; 2] = [0, 0x80];

unsafe fn epia_read_regr(pi: *mut pi_adapter, cont: i32, mut regr: i32) -> i32 {
    let (mut a, mut b, r): (i32, i32, i32);
    regr += CONT_MAP[cont as usize];
    match (*pi).mode {
        0 => { r = regr ^ 0x39; w0(r); w2(1); w2(3); w0(r); a = r1(); w2(1); b = r1(); w2(4); j44(a, b) }
        1 => { r = regr ^ 0x31; w0(r); w2(1); w0(r & 0x37); w2(3); w2(5); w0(r | 0xf0); a = r1(); b = r2(); w2(4); j53(a, b) }
        2 => { r = regr ^ 0x29; w0(r); w2(1); w2(0x21); w2(0x23); a = r0(); w2(4); a }
        3 | 4 | 5 => { w3(regr); w2(0x24); a = r4(); w2(4); a }
        _ => -1,
    }
}

unsafe fn epia_write_regr(pi: *mut pi_adapter, cont: i32, mut regr: i32, val: i32) {
    regr += CONT_MAP[cont as usize];
    match (*pi).mode {
        0 | 1 | 2 => { let r = regr ^ 0x19; w0(r); w2(1); w0(val); w2(3); w2(4); }
        3 | 4 | 5 => { let r = regr ^ 0x40; w3(r); w4(val); w2(4); }
        _ => {}
    }
}

unsafe fn wr(pi: *mut pi_adapter, r: i32, v: i32) { epia_write_regr(pi, 0, r, v); }
unsafe fn rr(pi: *mut pi_adapter, r: i32) -> i32 { epia_read_regr(pi, 0, r) }

unsafe fn epia_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0(); (*pi).saved_r2 = r2();
    w2(4); w0(0xa0); w0(0x50); w0(0xc0); w0(0x30); w0(0xa0); w0(0);
    w2(1); w2(4);
    if (*pi).mode >= 3 { w0(0xa); w2(1); w2(4); w0(0x82); w2(4); w2(0xc); w2(4); w2(0x24); w2(0x26); w2(4); }
    wr(pi, 0x86, 8);
}

unsafe fn epia_disconnect(pi: *mut pi_adapter) {
    w0((*pi).saved_r0); w2(1); w2(4); w0((*pi).saved_r0); w2((*pi).saved_r2);
}

unsafe fn epia_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let mut ph: i32; let mut a: i32; let mut b: i32;
    match (*pi).mode {
        0 => { w0(0x81); w2(1); w2(3); w0(0xc1); ph = 1; for k in 0..count { w2(2 + ph); a = r1(); w2(4 + ph); b = r1(); *buf.offset(k as isize) = j44(a,b) as i8; ph = 1-ph; } w0(0); w2(4); }
        1 => { w0(0x91); w2(1); w0(0x10); w2(3); w0(0x51); w2(5); w0(0xd1); ph=1; for k in 0..count { w2(4+ph); a=r1(); b=r2(); *buf.offset(k as isize)=j53(a,b) as i8; ph=1-ph; } w0(0); w2(4); }
        2 => { w0(0x89); w2(1); w2(0x23); w2(0x21); ph=1; for k in 0..count { w2(0x24+ph); *buf.offset(k as isize)=r0() as i8; ph=1-ph; } w2(6); w2(4); }
        3 => { if count>512 {wr(pi,0x84,3);} w3(0); w2(0x24); for k in 0..count {*buf.offset(k as isize)=r4() as i8;} w2(4); wr(pi,0x84,0); }
        4 => { if count>512 {wr(pi,0x84,3);} w3(0); w2(0x24); for k in 0..count/2 {*(buf as *mut u16).offset(k as isize)=r4w();} w2(4); wr(pi,0x84,0); }
        5 => { if count>512 {wr(pi,0x84,3);} w3(0); w2(0x24); for k in 0..count/4 {*(buf as *mut u32).offset(k as isize)=r4l();} w2(4); wr(pi,0x84,0); }
        _ => {}
    }
}

unsafe fn epia_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let (mut ph, mut last, mut d) = (0, 0x8000, 0);
    match (*pi).mode {
        0|1|2 => { w0(0xa1); w2(1); w2(3); w2(1); w2(5); for k in 0..count { d=*buf.offset(k as isize) as i32; if d!=last {last=d; w0(d);} w2(4+ph); ph=1-ph; } w2(7); w2(4); }
        3 => { if count<512 {wr(pi,0x84,1);} w3(0x40); for k in 0..count {w4(*buf.offset(k as isize) as i32);} if count<512 {wr(pi,0x84,0);} }
        4 => { if count<512 {wr(pi,0x84,1);} w3(0x40); for k in 0..count/2 {w4w(*(buf as *mut u16).offset(k as isize));} if count<512 {wr(pi,0x84,0);} }
        5 => { if count<512 {wr(pi,0x84,1);} w3(0x40); for k in 0..count/4 {w4l(*(buf as *mut u32).offset(k as isize));} if count<512 {wr(pi,0x84,0);} }
        _ => {}
    }
}

unsafe fn epia_test_proto(pi: *mut pi_adapter) -> i32 {
    let mut e = [0i32; 2]; let mut scratch = [0i8; 512];
    epia_connect(pi);
    for j in 0..2 { wr(pi, 6, 0xa0 + j * 0x10); for k in 0..256 { wr(pi, 2, k ^ 0xaa); wr(pi, 3, k ^ 0x55); if rr(pi, 2) != (k ^ 0xaa) {e[j as usize]+=1;} } wr(pi,2,1); wr(pi,3,1); }
    epia_disconnect(pi);
    let mut f=0; epia_connect(pi); wr(pi,0x84,8); epia_read_block(pi,scratch.as_mut_ptr(),512);
    for k in 0..256 { if (scratch[2*k] as i32 & 0xff) != ((k+1)&0xff) {f+=1;} if (scratch[2*k+1] as i32 & 0xff) != ((-2-k)&0xff) {f+=1;} }
    wr(pi,0x84,0); epia_disconnect(pi);
    (if e[0] != 0 && e[1] != 0 {1} else {0}) | if f != 0 {1} else {0}
}

unsafe fn epia_log_adapter(pi: *mut pi_adapter) {
    // Equivalent kernel logging hook; formatting/logging is supplied externally.
    let _ = ((*pi).port, (*pi).mode, (*pi).delay);
}

#[repr(C)]
pub struct pi_protocol {
    pub owner: *mut core::ffi::c_void,
    pub name: *const u8,
    pub max_mode: i32,
    pub epp_first: i32,
    pub default_delay: i32,
    pub max_units: i32,
}

#[no_mangle]
pub static mut epia: pi_protocol = pi_protocol {
    owner: core::ptr::null_mut(), name: b"epia\0".as_ptr(), max_mode: 6,
    epp_first: 3, default_delay: 1, max_units: 1,
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Shuttle Technologies EPIA parallel port IDE adapter protocol driver");
// module_pata_parport_driver(epia);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
