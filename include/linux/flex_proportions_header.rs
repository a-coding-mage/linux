/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Floating proportions with flexible aging period
 *
 *  Copyright (C) 2011, SUSE, Jan Kara <jack@suse.cz>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * When maximum proportion of some event type is specified, this is
 * the precision with which we allow limitting. Note that this creates
 * an upper bound on the number of events per period like
 *   ULLONG_MAX >> FPROP_FRAC_SHIFT.
 */
pub const FPROP_FRAC_SHIFT: usize = 10;
pub const FPROP_FRAC_BASE: usize = 1usize << FPROP_FRAC_SHIFT;

/*
 * ---- Global proportion definitions ----
 */
#[repr(C)]
pub struct fprop_global {
    /* Number of events in the current period */
    pub events: percpu_counter,
    /* Current period */
    pub period: ::core::ffi::c_uint,
    /* Synchronization with period transitions */
    pub sequence: seqcount_t,
}

extern "C" {
    pub fn fprop_global_init(p: *mut fprop_global, gfp: gfp_t) -> ::core::ffi::c_int;
    pub fn fprop_global_destroy(p: *mut fprop_global);
    pub fn fprop_new_period(p: *mut fprop_global, periods: ::core::ffi::c_int) -> bool;
}

/*
 * ---- PERCPU ----
 */
#[repr(C)]
pub struct fprop_local_percpu {
    /* the local events counter */
    pub events: percpu_counter,
    /* Period in which we last updated events */
    pub period: ::core::ffi::c_uint,
    /* Protect period and numerator */
    pub lock: raw_spinlock_t,
}

extern "C" {
    pub fn fprop_local_init_percpu(
        pl: *mut fprop_local_percpu,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn fprop_local_destroy_percpu(pl: *mut fprop_local_percpu);
    pub fn __fprop_add_percpu(
        p: *mut fprop_global,
        pl: *mut fprop_local_percpu,
        nr: ::core::ffi::c_long,
    );
    pub fn __fprop_add_percpu_max(
        p: *mut fprop_global,
        pl: *mut fprop_local_percpu,
        max_frac: ::core::ffi::c_int,
        nr: ::core::ffi::c_long,
    );
    pub fn fprop_fraction_percpu(
        p: *mut fprop_global,
        pl: *mut fprop_local_percpu,
        numerator: *mut ::core::ffi::c_ulong,
        denominator: *mut ::core::ffi::c_ulong,
    );
    pub fn local_irq_save(flags: *mut ::core::ffi::c_ulong);
    pub fn local_irq_restore(flags: ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn fprop_inc_percpu(p: *mut fprop_global, pl: *mut fprop_local_percpu) {
    let mut flags: ::core::ffi::c_ulong = 0;

    local_irq_save(&mut flags);
    __fprop_add_percpu(p, pl, 1);
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
