// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-1998  Grant R. Guenther <grant@torque.net>
 *
 * This is a low-level driver for the KBIC-951A and KBIC-971A
 * parallel to IDE adapter chips from KingByte Information Systems.
 *
 * The chips are almost identical, however, the wakeup code
 * required for the 971A interferes with the correct operation
 * of the 951A, so this driver registers itself twice, once for
 * each chip.
 */

// External kernel and pata_parport dependencies are supplied by other files.

#[repr(C)]
pub struct pi_adapter {
    pub port: i32,
    pub mode: i32,
    pub delay: i32,
    pub saved_r0: i32,
    pub saved_r2: i32,
    pub dev: *mut core::ffi::c_void,
}

extern "C" {
    fn w0(pi: *mut pi_adapter, value: i32);
    fn w2(pi: *mut pi_adapter, value: i32);
    fn w3(pi: *mut pi_adapter, value: i32);
    fn w4(pi: *mut pi_adapter, value: i32);
    fn r0(pi: *mut pi_adapter) -> i32;
    fn r1(pi: *mut pi_adapter) -> i32;
    fn r2(pi: *mut pi_adapter) -> i32;
    fn r4(pi: *mut pi_adapter) -> i32;
    fn r4w(pi: *mut pi_adapter) -> u16;
    fn r4l(pi: *mut pi_adapter) -> u32;
    fn swab16(value: u16) -> u16;
    fn inw(port: i32) -> u16;
    fn pata_parport_register_driver(protocol: *mut pi_protocol) -> i32;
    fn pata_parport_unregister_driver(protocol: *mut pi_protocol);
    fn dev_info(dev: *mut core::ffi::c_void, format: *const i8, ...);
    static mut delay_p: i32;
}

#[repr(C)]
pub struct pi_protocol {
    pub owner: *mut core::ffi::c_void,
    pub name: *const i8,
    pub max_mode: i32,
    pub epp_first: i32,
    pub default_delay: i32,
    pub max_units: i32,
    pub write_regr: Option<unsafe extern "C" fn(*mut pi_adapter, i32, i32, i32)>,
    pub read_regr: Option<unsafe extern "C" fn(*mut pi_adapter, i32, i32) -> i32>,
    pub write_block: Option<unsafe extern "C" fn(*mut pi_adapter, *mut i8, i32)>,
    pub read_block: Option<unsafe extern "C" fn(*mut pi_adapter, *mut i8, i32)>,
    pub connect: Option<unsafe extern "C" fn(*mut pi_adapter)>,
    pub disconnect: Option<unsafe extern "C" fn(*mut pi_adapter)>,
    pub log_adapter: Option<unsafe extern "C" fn(*mut pi_adapter)>,
}

#[inline]
unsafe fn r12w(pi: *mut pi_adapter) -> i32 {
    let _ = delay_p;
    (inw((*pi).port + 1) & 0xffff) as i32
}

#[inline]
fn j44(a: i32, b: i32) -> i32 { ((((a >> 4) & 0x0f) | (b & 0xf0)) ^ 0x88) }
#[inline]
fn j53(w: i32) -> i32 { (((w >> 3) & 0x1f) | ((w >> 4) & 0xe0)) }

static mut cont_map: [i32; 2] = [0x80, 0x40];

unsafe extern "C" fn kbic_read_regr(pi: *mut pi_adapter, cont: i32, regr: i32) -> i32 {
    let s = cont_map[cont as usize];
    let mut a;
    let mut b;
    match (*pi).mode {
        0 => { w0(pi, regr | 0x18 | s); w2(pi, 4); w2(pi, 6); w2(pi, 4); w2(pi, 1); w0(pi, 8); a = r1(pi); w0(pi, 0x28); b = r1(pi); w2(pi, 4); j44(a, b) }
        1 => { w0(pi, regr | 0x38 | s); w2(pi, 4); w2(pi, 6); w2(pi, 4); w2(pi, 5); w0(pi, 8); a = r12w(pi); w2(pi, 4); j53(a) }
        2 => { w0(pi, regr | 0x08 | s); w2(pi, 4); w2(pi, 6); w2(pi, 4); w2(pi, 0xa5); w2(pi, 0xa1); a = r0(pi); w2(pi, 4); a }
        3 | 4 | 5 => { w0(pi, 0x20 | s); w2(pi, 4); w2(pi, 6); w2(pi, 4); w3(pi, regr); a = r4(pi); let _b = r4(pi); w2(pi, 4); w2(pi, 0); w2(pi, 4); a }
        _ => -1,
    }
}

unsafe extern "C" fn kbic_write_regr(pi: *mut pi_adapter, cont: i32, regr: i32, val: i32) {
    let s = cont_map[cont as usize];
    match (*pi).mode {
        0 | 1 | 2 => { w0(pi, regr | 0x10 | s); w2(pi, 4); w2(pi, 6); w2(pi, 4); w0(pi, val); w2(pi, 5); w2(pi, 4); }
        3 | 4 | 5 => { w0(pi, 0x20 | s); w2(pi, 4); w2(pi, 6); w2(pi, 4); w3(pi, regr); w4(pi, val); w4(pi, val); w2(pi, 4); w2(pi, 0); w2(pi, 4); }
        _ => {}
    }
}

unsafe extern "C" fn k951_connect(pi: *mut pi_adapter) { (*pi).saved_r0 = r0(pi); (*pi).saved_r2 = r2(pi); w2(pi, 4); }
unsafe extern "C" fn k951_disconnect(pi: *mut pi_adapter) { w0(pi, (*pi).saved_r0); w2(pi, (*pi).saved_r2); }

unsafe fn ccp(pi: *mut pi_adapter, x: i32) { w2(pi, 0xc4); w0(pi, 0xaa); w0(pi, 0x55); w0(pi, 0); w0(pi, 0xff); w0(pi, 0x87); w0(pi, 0x78); w0(pi, x); w2(pi, 0xc5); w2(pi, 0xc4); w0(pi, 0xff); }
unsafe extern "C" fn k971_connect(pi: *mut pi_adapter) { (*pi).saved_r0 = r0(pi); (*pi).saved_r2 = r2(pi); ccp(pi, 0x20); w2(pi, 4); }
unsafe extern "C" fn k971_disconnect(pi: *mut pi_adapter) { ccp(pi, 0x30); w0(pi, (*pi).saved_r0); w2(pi, (*pi).saved_r2); }

// count must be congruent to 0 MOD 4, but all known applications have this property.
unsafe extern "C" fn kbic_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let mut k;
    match (*pi).mode {
        0 => { w0(pi, 0x98); w2(pi, 4); w2(pi, 6); w2(pi, 4); k = 0; while k < count / 2 { w2(pi, 1); w0(pi, 8); let a = r1(pi); w0(pi, 0x28); let mut b = r1(pi); *buf.offset((2*k) as isize) = j44(a,b) as i8; w2(pi,5); b=r1(pi); w0(pi,8); let a=r1(pi); *buf.offset((2*k+1) as isize)=j44(a,b) as i8; w2(pi,4); k+=1; } }
        1 => { w0(pi,0xb8); w2(pi,4); w2(pi,6); w2(pi,4); k=0; while k<count/4 { w0(pi,0xb8); w2(pi,4); w2(pi,5); w0(pi,8); *buf.offset((4*k) as isize)=j53(r12w(pi)) as i8; w0(pi,0xb8); *buf.offset((4*k+1) as isize)=j53(r12w(pi)) as i8; w2(pi,4); w2(pi,5); *buf.offset((4*k+3) as isize)=j53(r12w(pi)) as i8; w0(pi,8); *buf.offset((4*k+2) as isize)=j53(r12w(pi)) as i8; k+=1; } w2(pi,4); }
        2 => { w0(pi,0x88); w2(pi,4); w2(pi,6); w2(pi,4); k=0; while k<count/2 { w2(pi,0xa0); w2(pi,0xa1); *buf.offset((2*k) as isize)=r0(pi) as i8; w2(pi,0xa5); *buf.offset((2*k+1) as isize)=r0(pi) as i8; k+=1; } w2(pi,4); }
        3 => { w0(pi,0xa0); w2(pi,4); w2(pi,6); w2(pi,4); w3(pi,0); k=0; while k<count { *buf.offset(k as isize)=r4(pi) as i8; k+=1; } w2(pi,4); w2(pi,0); w2(pi,4); }
        4 => { w0(pi,0xa0); w2(pi,4); w2(pi,6); w2(pi,4); w3(pi,0); k=0; while k<count/2 { *(buf as *mut u16).offset(k as isize)=r4w(pi); k+=1; } w2(pi,4); w2(pi,0); w2(pi,4); }
        5 => { w0(pi,0xa0); w2(pi,4); w2(pi,6); w2(pi,4); w3(pi,0); k=0; while k<count/4 { *(buf as *mut u32).offset(k as isize)=r4l(pi); k+=1; } w2(pi,4); w2(pi,0); w2(pi,4); }
        _ => {}
    }
}

unsafe extern "C" fn kbic_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let mut k = 0;
    match (*pi).mode {
        0 | 1 | 2 => { w0(pi,0x90); w2(pi,4); w2(pi,6); w2(pi,4); while k<count/2 { w0(pi,*buf.offset((2*k+1) as isize) as i32); w2(pi,0); w2(pi,4); w0(pi,*buf.offset((2*k) as isize) as i32); w2(pi,5); w2(pi,4); k+=1; } }
        3 => { w0(pi,0xa0); w2(pi,4); w2(pi,6); w2(pi,4); w3(pi,0); while k<count/2 { w4(pi,*buf.offset((2*k+1) as isize) as i32); w4(pi,*buf.offset((2*k) as isize) as i32); k+=1; } w2(pi,4); w2(pi,0); w2(pi,4); }
        4 => { w0(pi,0xa0); w2(pi,4); w2(pi,6); w2(pi,4); w3(pi,0); while k<count/2 { w4(pi,swab16(*(buf as *mut u16).offset(k as isize)) as i32); k+=1; } w2(pi,4); w2(pi,0); w2(pi,4); }
        5 => { w0(pi,0xa0); w2(pi,4); w2(pi,6); w2(pi,4); w3(pi,0); while k<count/4 { let p=buf as *mut u16; w4(pi,(swab16(*p.offset((2*k) as isize)) as u32 | ((swab16(*p.offset((2*k+1) as isize)) as u32)<<16)) as i32); k+=1; } w2(pi,4); w2(pi,0); w2(pi,4); }
        _ => {}
    }
}

unsafe extern "C" fn kbic_log_adapter(pi: *mut pi_adapter, chip: *const i8) { dev_info((*pi).dev, b"KingByte %s at 0x%x, mode %d (%s), delay %d\0".as_ptr() as *const i8, chip, (*pi).port, (*pi).mode, chip, (*pi).delay); }
unsafe extern "C" fn k951_log_adapter(pi: *mut pi_adapter) { kbic_log_adapter(pi, b"KBIC-951A\0".as_ptr() as *const i8); }
unsafe extern "C" fn k971_log_adapter(pi: *mut pi_adapter) { kbic_log_adapter(pi, b"KBIC-971A\0".as_ptr() as *const i8); }

// Protocol registrations and module init/exit are supplied through the kernel module ABI.
static mut k951: Option<pi_protocol> = None;
static mut k971: Option<pi_protocol> = None;

unsafe extern "C" fn kbic_init() -> i32 {
    let mut rv = pata_parport_register_driver(core::ptr::addr_of_mut!(k951).cast());
    if rv < 0 { return rv; }
    rv = pata_parport_register_driver(core::ptr::addr_of_mut!(k971).cast());
    if rv < 0 { pata_parport_unregister_driver(core::ptr::addr_of_mut!(k951).cast()); }
    rv
}

unsafe extern "C" fn kbic_exit() {
    pata_parport_unregister_driver(core::ptr::addr_of_mut!(k951).cast());
    pata_parport_unregister_driver(core::ptr::addr_of_mut!(k971).cast());
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("KingByte Information Systems KBIC-951A and KBIC-971A parallel port IDE adapter protocol driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
