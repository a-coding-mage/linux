// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) 1998  Grant R. Guenther <grant@torque.net>
 *
 * ktti.c is a low-level protocol driver for the KT Technology
 * parallel port adapter.  This adapter is used in the "PHd"
 * portable hard-drives.  As far as I can tell, this device
 * supports 4-bit mode _only_.
 */

// C dependencies: linux module/init/delay/kernel/types/wait, asm/io, and pata_parport.h.

#[inline]
fn j44(a: i32, b: i32) -> i32 {
    ((a >> 4) & 0x0f) | (b & 0xf0)
}

/*
 * cont = 0 - access the IDE register file
 * cont = 1 - access the IDE command set
 */
static CONT_MAP: [i32; 2] = [0x10, 0x08];

unsafe fn ktti_write_regr(pi: *mut crate::pi_adapter, cont: i32, regr: i32, val: i32) {
    let r = regr + CONT_MAP[cont as usize];

    w0(r); w2(0xb); w2(0xa); w2(3); w2(6);
    w0(val); w2(3); w0(0); w2(6); w2(0xb);
}

unsafe fn ktti_read_regr(_pi: *mut crate::pi_adapter, cont: i32, regr: i32) -> i32 {
    let mut a: i32;
    let mut b: i32;
    let r = regr + CONT_MAP[cont as usize];

    w0(r); w2(0xb); w2(0xa); w2(9); w2(0xc); w2(9);
    a = r1(); w2(0xc); b = r1(); w2(9); w2(0xc); w2(9);
    j44(a, b)
}

unsafe fn ktti_read_block(_pi: *mut crate::pi_adapter, buf: *mut core::ffi::c_char, count: i32) {
    for k in 0..(count / 2) {
        w0(0x10); w2(0xb); w2(0xa); w2(9); w2(0xc); w2(9);
        let mut a = r1(); w2(0xc); let mut b = r1(); w2(9);
        *buf.add((2 * k) as usize) = j44(a, b) as core::ffi::c_char;
        a = r1(); w2(0xc); b = r1(); w2(9);
        *buf.add((2 * k + 1) as usize) = j44(a, b) as core::ffi::c_char;
    }
}

unsafe fn ktti_write_block(_pi: *mut crate::pi_adapter, buf: *mut core::ffi::c_char, count: i32) {
    for k in 0..(count / 2) {
        w0(0x10); w2(0xb); w2(0xa); w2(3); w2(6);
        w0(*buf.add((2 * k) as usize) as i32); w2(3);
        w0(*buf.add((2 * k + 1) as usize) as i32); w2(6);
        w2(0xb);
    }
}

unsafe fn ktti_connect(pi: *mut crate::pi_adapter) {
    (*pi).saved_r0 = r0();
    (*pi).saved_r2 = r2();
    w2(0xb); w2(0xa); w0(0); w2(3); w2(6);
}

unsafe fn ktti_disconnect(pi: *mut crate::pi_adapter) {
    w2(0xb); w2(0xa); w0(0xa0); w2(3); w2(4);
    w0((*pi).saved_r0);
    w2((*pi).saved_r2);
}

unsafe fn ktti_log_adapter(pi: *mut crate::pi_adapter) {
    dev_info!(&(*pi).dev, "KT adapter at 0x{:x}, delay {}\n", (*pi).port, (*pi).delay);
}

static mut KTTI: crate::pi_protocol = crate::pi_protocol {
    owner: THIS_MODULE,
    name: "ktti",
    max_mode: 1,
    epp_first: 2,
    default_delay: 1,
    max_units: 1,
    write_regr: Some(ktti_write_regr),
    read_regr: Some(ktti_read_regr),
    write_block: Some(ktti_write_block),
    read_block: Some(ktti_read_block),
    connect: Some(ktti_connect),
    disconnect: Some(ktti_disconnect),
    log_adapter: Some(ktti_log_adapter),
};

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Grant R. Guenther <grant@torque.net>");
// MODULE_DESCRIPTION("KT Technology parallel port IDE adapter protocol driver");
// module_pata_parport_driver(ktti);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
