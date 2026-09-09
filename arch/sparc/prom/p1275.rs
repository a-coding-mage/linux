// SPDX-License-Identifier: GPL-2.0
/*
 * p1275.c: Sun IEEE 1275 PROM low level interface routines
 *
 * Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// The declarations below are supplied by the corresponding kernel/architecture
// dependencies in the surrounding translation unit.

#[repr(C)]
pub struct P1275Buf {
    pub prom_callback: libc::c_long, // 0x00
    pub prom_cif_handler: Option<unsafe extern "C" fn(*mut libc::c_long)>, // 0x08
}

#[no_mangle]
pub static mut p1275buf: P1275Buf = P1275Buf {
    prom_callback: 0,
    prom_cif_handler: None,
};

extern "C" {
    fn prom_world(arg: libc::c_int);
    fn prom_cif_direct(args: *mut libc::c_ulong);
    fn prom_cif_callback();

    static mut prom_entry_lock: RawSpinlock;

    fn local_save_flags(flags: *mut libc::c_ulong);
    fn local_irq_restore(flags: libc::c_ulong);
    fn raw_spin_lock(lock: *mut RawSpinlock);
    fn raw_spin_unlock(lock: *mut RawSpinlock);
}

#[repr(C)]
pub struct RawSpinlock {
    _private: [u8; 0],
}

pub unsafe fn p1275_cmd_direct(args: *mut libc::c_ulong) {
    let mut flags: libc::c_ulong = 0;

    local_save_flags(&mut flags);
    local_irq_restore(0x15 as libc::c_ulong); // PIL_NMI
    raw_spin_lock(&mut prom_entry_lock);

    prom_world(1);
    prom_cif_direct(args);
    prom_world(0);

    raw_spin_unlock(&mut prom_entry_lock);
    local_irq_restore(flags);
}

pub unsafe fn prom_cif_init(cif_handler: *mut libc::c_void) {
    p1275buf.prom_cif_handler = Some(core::mem::transmute::<
        *mut libc::c_void,
        unsafe extern "C" fn(*mut libc::c_long),
    >(cif_handler));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
