// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1996-1998  Grant R. Guenther <grant@torque.net>
 *
 * bpck.c is a low-level protocol driver for the MicroSolutions
 * "backpack" parallel port IDE adapter.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct pi_adapter {
    pub private_: c_int,
    pub mode: c_int,
    pub unit: c_int,
    pub saved_r0: c_int,
    pub delay: c_int,
    pub port: c_int,
    pub dev: *mut c_void,
}

extern "C" {
    fn in_p(port: c_int) -> c_int;
    fn out_p(port: c_int, value: c_int);
    fn w0(pi: *mut pi_adapter, value: c_int);
    fn r0(pi: *mut pi_adapter) -> c_int;
    fn r1(pi: *mut pi_adapter) -> c_int;
    fn r4(pi: *mut pi_adapter) -> u8;
    fn r4w(pi: *mut pi_adapter) -> u16;
    fn r4l(pi: *mut pi_adapter) -> u32;
}

static CONT_MAP: [c_int; 3] = [0x40, 0x48, 0];

unsafe fn r2(pi: *mut pi_adapter) { (*pi).private_ = in_p(2) & 0xff; }
unsafe fn w2(pi: *mut pi_adapter, byte: c_int) { out_p(2, byte); (*pi).private_ = byte; }
unsafe fn t2(pi: *mut pi_adapter, pat: c_int) { (*pi).private_ ^= pat; out_p(2, (*pi).private_); }
unsafe fn e2(pi: *mut pi_adapter) { (*pi).private_ &= 0xfe; out_p(2, (*pi).private_); }
unsafe fn o2(pi: *mut pi_adapter) { (*pi).private_ |= 1; out_p(2, (*pi).private_); }
fn j44(l: c_int, h: c_int) -> c_int { ((l >> 3) & 0x7) | ((l >> 4) & 0x8) | ((h << 1) & 0x70) | (h & 0x80) }

unsafe fn bpck_read_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int) -> c_int {
    let r = regr + CONT_MAP[cont as usize]; let mut h;
    match (*pi).mode {
        0 => { w0(pi,r&0xf); w0(pi,r); t2(pi,2); t2(pi,4); let l=r1(pi); t2(pi,4); h=r1(pi); j44(l,h) },
        1 => { w0(pi,r&0xf); w0(pi,r); t2(pi,2); e2(pi); t2(pi,0x20); t2(pi,4); h=r0(pi); t2(pi,1); t2(pi,0x20); h },
        2|3|4 => { w0(pi,r); w2(pi,9); w2(pi,0); w2(pi,0x20); h=r4(pi) as c_int; w2(pi,0); h },
        _ => -1,
    }
}

unsafe fn bpck_write_regr(pi: *mut pi_adapter, cont: c_int, regr: c_int, val: c_int) {
    let r = regr + CONT_MAP[cont as usize];
    match (*pi).mode { 0|1 => { w0(pi,r); t2(pi,2); w0(pi,val); o2(pi); t2(pi,4); t2(pi,1); }, 2|3|4 => { w0(pi,r); w2(pi,9); w2(pi,0); w0(pi,val); w2(pi,1); w2(pi,3); w2(pi,0); }, _ => {} }
}

unsafe fn bpck_write_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    for i in 0..match (*pi).mode { 3=>count/2, 4=>count/4, _=>count } { let p=buf.add(i as usize); match (*pi).mode { 0|1 => { if i==0 { bpck_write_regr(pi,2,4,if (*pi).mode==0 {0x40}else{0x50}); w0(pi,0x40); t2(pi,2); t2(pi,1); } w0(pi,*p as u8 as c_int); t2(pi,4); }, 2 => { if i==0 {bpck_write_regr(pi,2,4,0x48); w0(pi,0x40);w2(pi,9);w2(pi,0);w2(pi,1);} /* w4 */ w0(pi,*p as u8 as c_int); }, 3|4 => { /* w4w/w4l */ w0(pi,*p as u8 as c_int); }, _=>{} } }
    match (*pi).mode { 0=>bpck_write_regr(pi,2,4,0), 1=>bpck_write_regr(pi,2,4,0x10), 2|3|4=>{w2(pi,0);bpck_write_regr(pi,2,4,8)}, _=>{} }
}

unsafe fn bpck_read_block(pi: *mut pi_adapter, buf: *mut c_char, count: c_int) {
    bpck_write_regr(pi,2,4,match (*pi).mode {0=>0x40,1=>0x50,_=>0x48});
    for i in 0..count { let v=match (*pi).mode {0=>{t2(pi,2);t2(pi,4);let l=r1(pi);t2(pi,4);j44(l,r1(pi))},1=>{t2(pi,2);t2(pi,0x20);t2(pi,4);r0(pi)},2|3|4=>{w0(pi,0x40);w2(pi,9);w2(pi,0);w2(pi,0x20);r4(pi) as c_int},_=>0}; *buf.add(i as usize)=v as c_char; }
    w2(pi,0); bpck_write_regr(pi,2,4,if (*pi).mode>=2 {8} else if (*pi).mode==1 {0x10} else {0});
}

unsafe fn bpck_probe_unit(pi:*mut pi_adapter)->c_int { let id=(*pi).unit; let mut s=0; w2(pi,4);w2(pi,0xe);r2(pi);let o1=r1(pi)&0xf8;let o0=r0(pi);w0(pi,255-id);w2(pi,4);w0(pi,id);t2(pi,8);t2(pi,8);t2(pi,8);t2(pi,2);let t=r1(pi)&0xf8;let f7=id%8==7;if f7||t!=o1{t2(pi,2);s=r1(pi)&0xf8;}if t==o1&&(!f7||s==o1){w2(pi,0x4c);w0(pi,o0);0}else{t2(pi,8);w0(pi,0);t2(pi,2);w2(pi,0x4c);w0(pi,o0);1} }

unsafe fn bpck_connect(pi:*mut pi_adapter){(*pi).saved_r0=r0(pi);w0(pi,255-(*pi).unit);w2(pi,4);w0(pi,(*pi).unit);t2(pi,8);t2(pi,8);t2(pi,8);t2(pi,2);t2(pi,2);if (*pi).mode<2{t2(pi,8);bpck_write_regr(pi,2,4,if (*pi).mode==0{0}else{0x10})}else{w2(pi,0);bpck_write_regr(pi,2,4,8)}bpck_write_regr(pi,2,5,8);bpck_write_regr(pi,2,0x46,0x10);bpck_write_regr(pi,2,0x4c,0x38);bpck_write_regr(pi,2,0x4d,0x88);bpck_write_regr(pi,2,0x46,0xa0);bpck_write_regr(pi,2,0x41,0);bpck_write_regr(pi,2,0x4e,8)}
unsafe fn bpck_disconnect(pi:*mut pi_adapter){w0(pi,0);if (*pi).mode>=2{w2(pi,9);w2(pi,0)}else{t2(pi,2)}w2(pi,0x4c);w0(pi,(*pi).saved_r0)}
unsafe fn bpck_force_spp(pi:*mut pi_adapter){(*pi).saved_r0=r0(pi);w0(pi,255-(*pi).unit);w2(pi,4);w0(pi,(*pi).unit);t2(pi,8);t2(pi,8);t2(pi,8);t2(pi,2);t2(pi,2);w2(pi,0);w0(pi,4);w2(pi,9);w2(pi,0);w0(pi,0);w2(pi,1);w2(pi,3);w2(pi,0);w0(pi,0);w2(pi,9);w2(pi,0);w2(pi,0x4c);w0(pi,(*pi).saved_r0)}

const TEST_LEN: usize = 16;

unsafe fn bpck_test_proto(pi:*mut pi_adapter)->c_int {
    let mut buf=[0i8;TEST_LEN]; bpck_force_spp(pi);
    match (*pi).mode { 0|1=>{bpck_connect(pi);bpck_write_regr(pi,2,0x13,0x7f);w0(pi,0x13);t2(pi,2);if (*pi).mode==0{for i in 0..TEST_LEN{t2(pi,4);let l=r1(pi);t2(pi,4);buf[i]=j44(l,r1(pi)) as i8;}}else{t2(pi,0x20);for i in 0..TEST_LEN{t2(pi,4);buf[i]=r0() as i8;}t2(pi,1);t2(pi,0x20)}bpck_disconnect(pi)},2|3|4=>{let om=(*pi).mode;(*pi).mode=0;bpck_connect(pi);bpck_write_regr(pi,2,7,3);bpck_write_regr(pi,2,4,8);bpck_disconnect(pi);(*pi).mode=om;bpck_connect(pi);w0(pi,0x13);w2(pi,9);w2(pi,1);w0(pi,0);w2(pi,3);w2(pi,0);w2(pi,0xe0);for i in 0..TEST_LEN{buf[i]=r4(pi) as i8;}w2(pi,0);bpck_write_regr(pi,2,7,0);bpck_disconnect(pi)},_=>{}};
    let mut e=0;for i in 0..TEST_LEN{if buf[i]!=(i+1) as i8{e+=1}}e
}

unsafe fn bpck_read_eeprom(pi:*mut pi_adapter,buf:*mut c_char){bpck_force_spp(pi);let om=(*pi).mode;let od=(*pi).delay;(*pi).mode=0;(*pi).delay=6;bpck_connect(pi);bpck_write_regr(pi,2,4,0);for i in 0..64{bpck_write_regr(pi,2,6,8);bpck_write_regr(pi,2,6,0xc);let mut p=0x100;for k in 0..9{let f=if ((i+0x180)&p)!=0{2}else{0};bpck_write_regr(pi,2,6,f+0xc);bpck_write_regr(pi,2,6,f+0xd);bpck_write_regr(pi,2,6,f+0xc);p>>=1}for j in 0..2{let mut v=0;for _ in 0..8{bpck_write_regr(pi,2,6,0xc);bpck_write_regr(pi,2,6,0xd);bpck_write_regr(pi,2,6,0xc);v=2*v+(bpck_read_regr(pi,2,0)==0x84) as c_int}*buf.add(2*i+1-j)=v as c_char}}bpck_write_regr(pi,2,6,8);bpck_write_regr(pi,2,6,0);bpck_write_regr(pi,2,5,8);bpck_disconnect(pi);if om>=2{bpck_connect(pi);bpck_write_regr(pi,2,7,3);bpck_write_regr(pi,2,4,8);bpck_disconnect(pi)}(*pi).mode=om;(*pi).delay=od}

unsafe fn bpck_test_port(pi:*mut pi_adapter)->c_int{w2(pi,0x2c);let i=r0(pi);w0(pi,255-i);let r=r0(pi);w0(pi,i);let mut m=if r==i{2}else if r==255-i{0}else{-1};w2(pi,0xc);let i=r0(pi);w0(pi,255-i);let r=r0(pi);w0(pi,i);if r!=255-i{m=-1}if m==0{w2(pi,6);w2(pi,0xc);let r=r0(pi);w0(pi,0xaa);w0(pi,r);w0(pi,0xaa)}if m==2{w2(pi,0x26);w2(pi,0xc)}if m==-1{0}else{5}}

#[repr(C)] pub struct pi_protocol { pub owner:*mut c_void,pub name:*const u8,pub max_mode:c_int,pub epp_first:c_int,pub default_delay:c_int,pub max_units:c_int }
static mut BPCK: pi_protocol=pi_protocol{owner:core::ptr::null_mut(),name=b"bpck\0".as_ptr(),max_mode:5,epp_first:2,default_delay:4,max_units:255};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
