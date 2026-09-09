// SPDX-License-Identifier: GPL-2.0-or-later
/* FS-Cache statistics
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// #define FSCACHE_DEBUG_LEVEL CACHE
// Dependencies are supplied by the surrounding kernel translation.

/*
 * operation counters
 */
extern "C" {
    pub static mut fscache_n_volumes: atomic_t;
    pub static mut fscache_n_volumes_collision: atomic_t;
    pub static mut fscache_n_volumes_nomem: atomic_t;
    pub static mut fscache_n_cookies: atomic_t;
    pub static mut fscache_n_cookies_lru: atomic_t;
    pub static mut fscache_n_cookies_lru_expired: atomic_t;
    pub static mut fscache_n_cookies_lru_removed: atomic_t;
    pub static mut fscache_n_cookies_lru_dropped: atomic_t;

    pub static mut fscache_n_acquires: atomic_t;
    pub static mut fscache_n_acquires_ok: atomic_t;
    pub static mut fscache_n_acquires_oom: atomic_t;

    pub static mut fscache_n_invalidates: atomic_t;

    pub static mut fscache_n_updates: atomic_t;

    pub static mut fscache_n_relinquishes: atomic_t;
    pub static mut fscache_n_relinquishes_retire: atomic_t;
    pub static mut fscache_n_relinquishes_dropped: atomic_t;

    pub static mut fscache_n_resizes: atomic_t;
    pub static mut fscache_n_resizes_null: atomic_t;

    pub static mut fscache_n_read: atomic_t;
    pub static mut fscache_n_write: atomic_t;
    pub static mut fscache_n_no_write_space: atomic_t;
    pub static mut fscache_n_no_create_space: atomic_t;
    pub static mut fscache_n_culled: atomic_t;
    pub static mut fscache_n_dio_misfit: atomic_t;
}

// EXPORT_SYMBOL(fscache_n_updates);
// EXPORT_SYMBOL(fscache_n_read);
// EXPORT_SYMBOL(fscache_n_write);
// EXPORT_SYMBOL(fscache_n_no_write_space);
// EXPORT_SYMBOL(fscache_n_no_create_space);
// EXPORT_SYMBOL(fscache_n_culled);
// EXPORT_SYMBOL(fscache_n_dio_misfit);

/*
 * display the general statistics
 */
pub unsafe extern "C" fn fscache_stats_show(m: *mut seq_file) -> i32 {
    seq_puts(m, "-- FS-Cache statistics --\n\0".as_ptr() as *const i8);
    seq_printf(
        m,
        "Cookies: n=%d v=%d vcol=%u voom=%u\n\0".as_ptr() as *const i8,
        atomic_read(&raw const fscache_n_cookies),
        atomic_read(&raw const fscache_n_volumes),
        atomic_read(&raw const fscache_n_volumes_collision),
        atomic_read(&raw const fscache_n_volumes_nomem),
    );

    seq_printf(
        m,
        "Acquire: n=%u ok=%u oom=%u\n\0".as_ptr() as *const i8,
        atomic_read(&raw const fscache_n_acquires),
        atomic_read(&raw const fscache_n_acquires_ok),
        atomic_read(&raw const fscache_n_acquires_oom),
    );

    seq_printf(
        m,
        "LRU    : n=%u exp=%u rmv=%u drp=%u at=%ld\n\0".as_ptr() as *const i8,
        atomic_read(&raw const fscache_n_cookies_lru),
        atomic_read(&raw const fscache_n_cookies_lru_expired),
        atomic_read(&raw const fscache_n_cookies_lru_removed),
        atomic_read(&raw const fscache_n_cookies_lru_dropped),
        if timer_pending(&raw const fscache_cookie_lru_timer) != 0 {
            fscache_cookie_lru_timer.expires - jiffies
        } else {
            0
        },
    );

    seq_printf(m, "Invals : n=%u\n\0".as_ptr() as *const i8,
               atomic_read(&raw const fscache_n_invalidates));
    seq_printf(m, "Updates: n=%u rsz=%u rsn=%u\n\0".as_ptr() as *const i8,
               atomic_read(&raw const fscache_n_updates),
               atomic_read(&raw const fscache_n_resizes),
               atomic_read(&raw const fscache_n_resizes_null));
    seq_printf(m, "Relinqs: n=%u rtr=%u drop=%u\n\0".as_ptr() as *const i8,
               atomic_read(&raw const fscache_n_relinquishes),
               atomic_read(&raw const fscache_n_relinquishes_retire),
               atomic_read(&raw const fscache_n_relinquishes_dropped));
    seq_printf(m, "NoSpace: nwr=%u ncr=%u cull=%u\n\0".as_ptr() as *const i8,
               atomic_read(&raw const fscache_n_no_write_space),
               atomic_read(&raw const fscache_n_no_create_space),
               atomic_read(&raw const fscache_n_culled));
    seq_printf(m, "IO     : rd=%u wr=%u mis=%u\n\0".as_ptr() as *const i8,
               atomic_read(&raw const fscache_n_read),
               atomic_read(&raw const fscache_n_write),
               atomic_read(&raw const fscache_n_dio_misfit));
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
