// SPDX-License-Identifier: GPL-2.0
/* net/atm/atm_misc.c - Various functions for use by ATM drivers */

/* Written 1995-2000 by Werner Almesberger, EPFL ICA */

// External declarations supplied by the Linux ATM dependencies.

#[repr(C)]
pub struct AtomicT {
    pub counter: i32,
}

#[repr(C)]
pub struct AtmVccStats {
    pub rx_drop: AtomicT,
}

#[repr(C)]
pub struct Sock {
    pub sk_rmem_alloc: AtomicT,
    pub sk_rcvbuf: i32,
}

#[repr(C)]
pub struct AtmVcc {
    pub stats: *mut AtmVccStats,
}

extern "C" {
    pub fn atm_force_charge(vcc: *mut AtmVcc, truesize: i32);
    pub fn sk_atm(vcc: *mut AtmVcc) -> *mut Sock;
    pub fn atm_return(vcc: *mut AtmVcc, truesize: i32);
    pub fn atomic_read(v: *const AtomicT) -> i32;
    pub fn atomic_inc(v: *mut AtomicT);
}

pub unsafe fn atm_charge(vcc: *mut AtmVcc, truesize: i32) -> i32 {
    atm_force_charge(vcc, truesize);
    if atomic_read(&(*sk_atm(vcc)).sk_rmem_alloc) <= (*sk_atm(vcc)).sk_rcvbuf {
        return 1;
    }
    atm_return(vcc, truesize);
    atomic_inc(&mut (*(*vcc).stats).rx_drop);
    0
}

// EXPORT_SYMBOL(atm_charge);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
