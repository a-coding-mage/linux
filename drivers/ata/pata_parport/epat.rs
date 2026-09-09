// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1997-1998  Grant R. Guenther <grant@torque.net>
 *
 * This is the low level protocol driver for the EPAT parallel
 * to IDE adapter from Shuttle Technologies.  This adapter is
 * used in many popular parallel port disk products such as the
 * SyQuest EZ drives, the Avatar Shark and the Imation SuperDisk.
 */

// Dependency declarations supplied by the surrounding kernel translation.

const fn j44(a: i32, b: i32) -> i32 { ((a >> 4) & 0x0f) + (b & 0xf0) }
const fn j53(a: i32, b: i32) -> i32 { ((a >> 3) & 0x1f) + ((b << 4) & 0xe0) }

static mut epatc8: i32 = 0;

// cont = 0 IDE register file; cont = 1 IDE control registers;
// cont = 2 internal EPAT registers.
static cont_map: [i32; 3] = [0x18, 0x10, 0];

unsafe fn epat_write_regr(pi: *mut pi_adapter, cont: i32, regr: i32, val: i32) {
    let r = regr + cont_map[cont as usize];
    match (*pi).mode {
        0 | 1 | 2 => { w0(0x60 + r); w2(1); w0(val); w2(4); }
        3 | 4 | 5 => { w3(0x40 + r); w4(val); }
        _ => {}
    }
}

unsafe fn epat_read_regr(pi: *mut pi_adapter, cont: i32, regr: i32) -> i32 {
    let r = regr + cont_map[cont as usize];
    match (*pi).mode {
        0 => { w0(r); w2(1); w2(3); let a = r1(); w2(4); let b = r1(); j44(a, b) }
        1 => { w0(0x40 + r); w2(1); w2(4); let a = r1(); let b = r2(); w0(0xff); j53(a, b) }
        2 => { w0(0x20 + r); w2(1); w2(0x25); let a = r0(); w2(4); a }
        3 | 4 | 5 => { w3(r); w2(0x24); let a = r4(); w2(4); a }
        _ => -1,
    }
}

unsafe fn epat_read_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    let mut ph: i32;
    match (*pi).mode {
        0 => { w0(7); w2(1); w2(3); w0(0xff); ph = 0; for k in 0..count { if k == count - 1 { w0(0xfd); } w2(6 + ph); let a = r1(); let b = if a & 8 != 0 { a } else { w2(4 + ph); r1() }; *buf.offset(k as isize) = j44(a, b) as i8; ph = 1 - ph; } w0(0); w2(4); }
        1 => { w0(0x47); w2(1); w2(5); w0(0xff); ph = 0; for k in 0..count { if k == count - 1 { w0(0xfd); } w2(4 + ph); let a = r1(); let b = r2(); *buf.offset(k as isize) = j53(a, b) as i8; ph = 1 - ph; } w0(0); w2(4); }
        2 => { w0(0x27); w2(1); w2(0x25); w0(0); ph = 0; for k in 0..count - 1 { w2(0x24 + ph); *buf.offset(k as isize) = r0() as i8; ph = 1 - ph; } w2(0x26); w2(0x27); *buf.offset((count - 1) as isize) = r0() as i8; w2(0x25); w2(4); }
        3 => { w3(0x80); w2(0x24); for k in 0..count - 1 { *buf.offset(k as isize) = r4() as i8; } w2(4); w3(0xa0); w2(0x24); *buf.offset((count - 1) as isize) = r4() as i8; w2(4); }
        4 => { w3(0x80); w2(0x24); for k in 0..count / 2 - 1 { *(buf as *mut u16).offset(k as isize) = r4w(); } *buf.offset((count - 2) as isize) = r4() as i8; w2(4); w3(0xa0); w2(0x24); *buf.offset((count - 1) as isize) = r4() as i8; w2(4); }
        5 => { w3(0x80); w2(0x24); for k in 0..count / 4 - 1 { *(buf as *mut u32).offset(k as isize) = r4l(); } for k in count - 4..count - 1 { *buf.offset(k as isize) = r4() as i8; } w2(4); w3(0xa0); w2(0x24); *buf.offset((count - 1) as isize) = r4() as i8; w2(4); }
        _ => {}
    }
}

unsafe fn epat_write_block(pi: *mut pi_adapter, buf: *mut i8, count: i32) {
    match (*pi).mode {
        0 | 1 | 2 => { w0(0x67); w2(1); w2(5); let mut ph = 0; for k in 0..count { w0(*buf.offset(k as isize) as i32); w2(4 + ph); ph = 1 - ph; } w2(7); w2(4); }
        3 => { w3(0xc0); for k in 0..count { w4(*buf.offset(k as isize) as i32); } w2(4); }
        4 => { w3(0xc0); for k in 0..count / 2 { w4w(*(buf as *mut u16).offset(k as isize)); } w2(4); }
        5 => { w3(0xc0); for k in 0..count / 4 { w4l(*(buf as *mut u32).offset(k as isize)); } w2(4); }
        _ => {}
    }
}

unsafe fn epat_connect(pi: *mut pi_adapter) {
    (*pi).saved_r0 = r0(); (*pi).saved_r2 = r2();
    cpp(pi, 0);
    if epatc8 != 0 { cpp(pi, 0x40); cpp(pi, 0xe0); w0(0); w2(1); w2(4); epat_write_regr(pi, 2, 8, 0x12); epat_write_regr(pi, 2, 0xc, 0x14); epat_write_regr(pi, 2, 0x12, 0x10); epat_write_regr(pi, 2, 0xe, 0xf); epat_write_regr(pi, 2, 0xf, 4); epat_write_regr(pi, 2, 0xe, 0xd); epat_write_regr(pi, 2, 0xf, 0); }
    cpp(pi, 0xe0); w0(0); w2(1); w2(4);
    if (*pi).mode >= 3 { w0(0); w2(1); w2(4); w2(0xc); w0(0x40); w2(6); w2(7); w2(4); w2(0xc); w2(4); }
    if epatc8 == 0 { epat_write_regr(pi, 2, 8, 0x10); epat_write_regr(pi, 2, 0xc, 0x14); epat_write_regr(pi, 2, 0xa, 0x38); epat_write_regr(pi, 2, 0x12, 0x10); }
}

unsafe fn epat_disconnect(pi: *mut pi_adapter) { cpp(pi, 0x30); w0((*pi).saved_r0); w2((*pi).saved_r2); }

// CPP performs the EPAT chip protocol sequence.
unsafe fn cpp(_pi: *mut pi_adapter, x: i32) { w2(4); w0(0x22); w0(0xaa); w0(0x55); w0(0); w0(0xff); w0(0x87); w0(0x78); w0(x); w2(4); w2(5); w2(4); w0(0xff); }

// The remaining protocol callbacks and module registration retain their C interfaces.
// CONFIG_PATA_PARPORT_EPATC8 sets epatc8 during initialization.

unsafe fn epat_test_proto(pi: *mut pi_adapter) -> i32 {
    let mut e = [0i32; 2]; let mut scratch = [0i8; 512];
    epat_connect(pi); let cc = epat_read_regr(pi, 2, 0xd); epat_disconnect(pi);
    epat_connect(pi);
    for j in 0..2 { epat_write_regr(pi, 0, 6, 0xa0 + j * 0x10); for k in 0..256 { epat_write_regr(pi, 0, 2, k ^ 0xaa); epat_write_regr(pi, 0, 3, k ^ 0x55); if epat_read_regr(pi, 0, 2) != (k ^ 0xaa) { e[j as usize] += 1; } } }
    epat_disconnect(pi);
    let mut f = 0; epat_connect(pi); epat_write_regr(pi, 2, 0x13, 1); epat_write_regr(pi, 2, 0x13, 0); epat_write_regr(pi, 2, 0xa, 0x11); epat_read_block(pi, scratch.as_mut_ptr(), 512);
    for k in 0..256 { if (scratch[2 * k] as i32 & 0xff) != k as i32 { f += 1; } if (scratch[2 * k + 1] as i32 & 0xff) != 0xff - k as i32 { f += 1; } }
    epat_disconnect(pi); let _ = cc; ((e[0] != 0 && e[1] != 0) || f != 0) as i32
}

unsafe fn epat_log_adapter(pi: *mut pi_adapter) {
    let modes = ["4-bit", "5/3", "8-bit", "EPP-8", "EPP-16", "EPP-32"];
    epat_connect(pi); epat_write_regr(pi, 2, 0xa, 0x38); let ver = epat_read_regr(pi, 2, 0xb); epat_disconnect(pi); let _ = (ver, modes[(*pi).mode as usize]);
}

#[repr(C)]
struct pi_protocol {
    owner: *const core::ffi::c_void, name: *const u8, max_mode: i32, epp_first: i32,
    default_delay: i32, max_units: i32,
    write_regr: unsafe fn(*mut pi_adapter, i32, i32, i32), read_regr: unsafe fn(*mut pi_adapter, i32, i32) -> i32,
    write_block: unsafe fn(*mut pi_adapter, *mut i8, i32), read_block: unsafe fn(*mut pi_adapter, *mut i8, i32),
    connect: unsafe fn(*mut pi_adapter), disconnect: unsafe fn(*mut pi_adapter),
    test_proto: unsafe fn(*mut pi_adapter) -> i32, log_adapter: unsafe fn(*mut pi_adapter),
}

static mut epat: pi_protocol = pi_protocol {
    owner: core::ptr::null(), name: b"epat\0".as_ptr(), max_mode: 6, epp_first: 3,
    default_delay: 1, max_units: 1, write_regr: epat_write_regr, read_regr: epat_read_regr,
    write_block: epat_write_block, read_block: epat_read_block, connect: epat_connect,
    disconnect: epat_disconnect, test_proto: epat_test_proto, log_adapter: epat_log_adapter,
};

unsafe fn epat_init() -> i32 {
    // #ifdef CONFIG_PATA_PARPORT_EPATC8
    epatc8 = 1;
    // #endif
    pata_parport_register_driver(&mut epat)
}

unsafe fn epat_exit() { pata_parport_unregister_driver(&mut epat); }

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("Shuttle Technologies EPAT parallel port IDE adapter protocol driver");
// module_init(epat_init)
// module_exit(epat_exit)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
