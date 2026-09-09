// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-1998  Grant R. Guenther <grant@torque.net>
 *
 * on26.c is a low-level protocol driver for the
 * OnSpec 90c26 parallel to IDE adapter chip.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct pi_adapter {
    pub dev: c_void,
    pub mode: c_int,
    pub delay: c_int,
    pub saved_r0: c_int,
    pub saved_r2: c_int,
    pub port: c_int,
}

extern "C" {
    fn w0(pi: *mut pi_adapter, v: c_int);
    fn w2(pi: *mut pi_adapter, v: c_int);
    fn w3(pi: *mut pi_adapter, v: c_int);
    fn w4(pi: *mut pi_adapter, v: c_int);
    fn r0(pi: *mut pi_adapter) -> c_int;
    fn r1(pi: *mut pi_adapter) -> c_int;
    fn r2(pi: *mut pi_adapter) -> c_int;
    fn r4(pi: *mut pi_adapter) -> c_int;
    fn r4w(pi: *mut pi_adapter) -> u16;
    fn r4l(pi: *mut pi_adapter) -> u32;
    fn udelay(v: c_int);
    fn mdelay(v: c_int);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
}

#[inline]
fn j44(a: c_int, b: c_int) -> c_int { ((a >> 4) & 0x0f) | (b & 0xf0) }

unsafe fn p1(pi: *mut pi_adapter) { w2(pi, 5); w2(pi, 0xd); w2(pi, 5); w2(pi, 0xd); w2(pi, 5); w2(pi, 4); }
unsafe fn p2(pi: *mut pi_adapter) { w2(pi, 5); w2(pi, 7); w2(pi, 5); w2(pi, 4); }
unsafe fn ccp(pi: *mut pi_adapter, x: c_int) {
    w0(pi, 0xfe); w0(pi, 0xaa); w0(pi, 0x55); w0(pi, 0);
    w0(pi, 0xff); w0(pi, 0x87); w0(pi, 0x78); w0(pi, x);
    w2(pi, 4); w2(pi, 5); w2(pi, 4); w0(pi, 0xff);
}

unsafe fn on26_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    let r = (regr << 2) + 1 + cont;
    match (*pi).mode {
        0 => { w0(pi,1); p1(pi); w0(pi,r); p2(pi); w0(pi,0); p1(pi); w2(pi,6); let a=r1(pi); w2(pi,4); w2(pi,6); let b=r1(pi); w2(pi,4); w2(pi,6); w2(pi,4); w2(pi,6); w2(pi,4); j44(a,b) }
        1 => { w0(pi,1); p1(pi); w0(pi,r); p2(pi); w0(pi,0); p1(pi); w2(pi,0x26); let a=r0(pi); w2(pi,4); w2(pi,0x26); w2(pi,4); a }
        2|3|4 => { w3(pi,1); w3(pi,1); w2(pi,5); w4(pi,r); w2(pi,4); w3(pi,0); w3(pi,0); w2(pi,0x24); let a=r4(pi); w2(pi,4); w2(pi,0x24); let _=r4(pi); w2(pi,4); a }
        _ => -1,
    }
}

unsafe fn on26_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    let r=(regr<<2)+1+cont;
    match (*pi).mode { 0|1 => { w0(pi,1);p1(pi);w0(pi,r);p2(pi);w0(pi,0);p1(pi);w0(pi,val);p2(pi);w0(pi,val);p2(pi); }, 2|3|4 => { w3(pi,1);w3(pi,1);w2(pi,5);w4(pi,r);w2(pi,4);w3(pi,0);w3(pi,0);w2(pi,5);w4(pi,val);w2(pi,4);w2(pi,5);w4(pi,val);w2(pi,4); }, _=>{} }
}

unsafe fn on26_connect(pi:*mut pi_adapter) { (*pi).saved_r0=r0(pi); (*pi).saved_r2=r2(pi); ccp(pi,0x20); let x=if (*pi).mode!=0 {9}else{8}; w0(pi,2);p1(pi);w0(pi,8);p2(pi);w0(pi,2);p1(pi);w0(pi,x);p2(pi); }
unsafe fn on26_disconnect(pi:*mut pi_adapter) { if (*pi).mode>=2 { for _ in 0..4 {w3(pi,4);} } else {w0(pi,4);p1(pi);w0(pi,4);p1(pi);} ccp(pi,0x30);w0(pi,(*pi).saved_r0);w2(pi,(*pi).saved_r2); }

unsafe fn on26_test_port(pi:*mut pi_adapter)->c_int {
    let saved_d=(*pi).delay; let saved_m=(*pi).mode; (*pi).saved_r0=r0(pi);(*pi).saved_r2=r2(pi);(*pi).delay=5;(*pi).mode=0; w2(pi,0xc); ccp(pi,0x30);ccp(pi,0);
    w0(pi,0xfe);w0(pi,0xaa);w0(pi,0x55);w0(pi,0);w0(pi,0xff); let mut i=((r1(pi)&0xf0)<<4);w0(pi,0x87);i|=r1(pi)&0xf0;w0(pi,0x78);w0(pi,0x20);w2(pi,4);w2(pi,5);i|=(r1(pi)&0xf0)>>4;w2(pi,4);w0(pi,0xff);
    if i==0xb5f { w0(pi,2);p1(pi);w0(pi,0);p2(pi);w0(pi,3);p1(pi);w0(pi,0);p2(pi);w0(pi,2);p1(pi);w0(pi,8);p2(pi);udelay(100);w0(pi,2);p1(pi);w0(pi,0xa);p2(pi);udelay(100);w0(pi,2);p1(pi);w0(pi,8);p2(pi);udelay(1000);on26_write_regr(pi,0,6,0xa0); let mut x=0;let mut y=0;for n in 0..200 {on26_write_regr(pi,0,6,0xa0);x=on26_read_regr(pi,0,7);on26_write_regr(pi,0,6,0xb0);y=on26_read_regr(pi,0,7);if (x&0x80)==0&&(y&0x80)==0 {break} if n==199 {dev_err(&mut (*pi).dev as *mut _, b"on26: Device reset failed (%x,%x)\0".as_ptr() as *const c_char,x,y);} mdelay(100);}w0(pi,4);p1(pi);w0(pi,4);p1(pi); }
    ccp(pi,0x30);(*pi).delay=saved_d;(*pi).mode=saved_m;w0(pi,(*pi).saved_r0);w2(pi,(*pi).saved_r2);5
}

unsafe fn on26_read_block(pi:*mut pi_adapter,buf:*mut c_char,count:c_int) { match (*pi).mode { 0=>{w0(pi,1);p1(pi);w0(pi,1);p2(pi);w0(pi,2);p1(pi);w0(pi,0x18);p2(pi);w0(pi,0);p1(pi);udelay(10);for k in 0..count {w2(pi,6);let a=r1(pi);w2(pi,4);let b=r1(pi);*buf.add(k as usize)=j44(a,b) as c_char;}w0(pi,2);p1(pi);w0(pi,8);p2(pi);}, 1=>{w0(pi,1);p1(pi);w0(pi,1);p2(pi);w0(pi,2);p1(pi);w0(pi,0x19);p2(pi);w0(pi,0);p1(pi);udelay(10);for k in 0..count/2 {w2(pi,0x26);*(buf as *mut u8).add((2*k)as usize)=r0(pi)as u8;w2(pi,0x24);*(buf as *mut u8).add((2*k+1)as usize)=r0(pi)as u8;}w0(pi,2);p1(pi);w0(pi,9);p2(pi);},2=>{w3(pi,1);w3(pi,1);w2(pi,5);w4(pi,1);w2(pi,4);w3(pi,0);w3(pi,0);w2(pi,0x24);udelay(10);for k in 0..count {*buf.add(k as usize)=r4(pi)as c_char;}w2(pi,4);},3=>{w3(pi,1);w3(pi,1);w2(pi,5);w4(pi,1);w2(pi,4);w3(pi,0);w3(pi,0);w2(pi,0x24);udelay(10);for k in 0..count/2 {*((buf as *mut u16).add(k as usize))=r4w(pi);}w2(pi,4);},4=>{w3(pi,1);w3(pi,1);w2(pi,5);w4(pi,1);w2(pi,4);w3(pi,0);w3(pi,0);w2(pi,0x24);udelay(10);for k in 0..count/4 {*((buf as *mut u32).add(k as usize))=r4l(pi);}w2(pi,4);}, _=>{} } }

unsafe fn on26_write_block(pi:*mut pi_adapter,buf:*mut c_char,count:c_int) { match (*pi).mode { 0|1=>{w0(pi,1);p1(pi);w0(pi,1);p2(pi);w0(pi,2);p1(pi);w0(pi,0x18+(*pi).mode);p2(pi);w0(pi,0);p1(pi);udelay(10);for k in 0..count/2 {w2(pi,5);w0(pi,*buf.add((2*k)as usize)as c_int);w2(pi,7);w0(pi,*buf.add((2*k+1)as usize)as c_int);}w2(pi,5);w2(pi,4);w0(pi,2);p1(pi);w0(pi,8+(*pi).mode);p2(pi);},2|3|4=>{w3(pi,1);w3(pi,1);w2(pi,5);w4(pi,1);w2(pi,4);w3(pi,0);w3(pi,0);w2(pi,0xc5);udelay(10);if (*pi).mode==2 {for k in 0..count {w4(pi,*buf.add(k as usize)as c_int);}} else if (*pi).mode==3 {for k in 0..count/2 {w4(pi,*(buf as *mut u16).add(k as usize)as c_int);}} else {for k in 0..count/4 {w4(pi,*(buf as *mut u32).add(k as usize)as c_int);}}w2(pi,0xc4);}, _=>{} } }

unsafe fn on26_log_adapter(pi: *mut pi_adapter) {
    let mode_string = [b"4-bit\0", b"8-bit\0", b"EPP-8\0", b"EPP-16\0", b"EPP-32\0"];
    dev_info(&mut (*pi).dev as *mut _, b"OnSpec 90c26 at 0x%x, mode %d (%s), delay %d\0".as_ptr() as *const c_char,
             (*pi).port, (*pi).mode, mode_string[(*pi).mode as usize].as_ptr(), (*pi).delay);
}

#[repr(C)]
pub struct pi_protocol {
    pub owner: *mut c_void,
    pub name: *const c_char,
    pub max_mode: c_int,
    pub epp_first: c_int,
    pub default_delay: c_int,
    pub max_units: c_int,
    pub write_regr: unsafe fn(*mut pi_adapter, c_int, c_int, c_int),
    pub read_regr: unsafe fn(*mut pi_adapter, c_int, c_int) -> c_int,
    pub write_block: unsafe fn(*mut pi_adapter, *mut c_char, c_int),
    pub read_block: unsafe fn(*mut pi_adapter, *mut c_char, c_int),
    pub connect: unsafe fn(*mut pi_adapter),
    pub disconnect: unsafe fn(*mut pi_adapter),
    pub test_port: unsafe fn(*mut pi_adapter) -> c_int,
    pub log_adapter: unsafe fn(*mut pi_adapter),
}

#[no_mangle]
pub static mut on26: pi_protocol = pi_protocol {
    owner: core::ptr::null_mut(), name: b"on26\0".as_ptr() as *const c_char,
    max_mode: 5, epp_first: 2, default_delay: 1, max_units: 1,
    write_regr: on26_write_regr, read_regr: on26_read_regr,
    write_block: on26_write_block, read_block: on26_read_block,
    connect: on26_connect, disconnect: on26_disconnect,
    test_port: on26_test_port, log_adapter: on26_log_adapter,
};

// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Onspec 90c26 parallel port IDE adapter protocol driver");
// module_pata_parport_driver(on26);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
