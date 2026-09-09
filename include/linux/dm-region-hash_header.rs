/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2003 Sistina Software Limited.
 * Copyright (C) 2004-2008 Red Hat, Inc. All rights reserved.
 *
 * Device-Mapper dirty region hash interface.
 *
 * This file is released under the GPL.
 */

/* Dependency supplied by linux/dm-dirty-log.h. */

/*
 *----------------------------------------------------------------
 * Region hash
 *----------------------------------------------------------------
 */
#[repr(C)]
pub struct dm_region_hash {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_region {
    _private: [u8; 0],
}

/* States a region can have. */
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum dm_rh_region_states {
    DM_RH_CLEAN = 0x01,       /* No writes in flight. */
    DM_RH_DIRTY = 0x02,       /* Writes in flight. */
    DM_RH_NOSYNC = 0x04,      /* Out of sync. */
    DM_RH_RECOVERING = 0x08,  /* Under resynchronization. */
}

#[repr(C)]
pub struct bio_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_dirty_log {
    _private: [u8; 0],
}

/*
 * Region hash create/destroy.
 */
unsafe extern "C" {
    pub fn dm_region_hash_create(
        context: *mut core::ffi::c_void,
        dispatch_bios: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut bio_list)>,
        wakeup_workers: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        wakeup_all_recovery_waiters: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        target_begin: sector_t,
        max_recovery: u32,
        log: *mut dm_dirty_log,
        region_size: u32,
        nr_regions: region_t,
    ) -> *mut dm_region_hash;
    pub fn dm_region_hash_destroy(rh: *mut dm_region_hash);

    pub fn dm_rh_dirty_log(rh: *mut dm_region_hash) -> *mut dm_dirty_log;

    /* Conversion functions. */
    pub fn dm_rh_bio_to_region(rh: *mut dm_region_hash, bio: *mut bio) -> region_t;
    pub fn dm_rh_region_to_sector(rh: *mut dm_region_hash, region: region_t) -> sector_t;
    pub fn dm_rh_region_context(reg: *mut dm_region) -> *mut core::ffi::c_void;

    /* Get region size and key (ie. number of the region). */
    pub fn dm_rh_get_region_size(rh: *mut dm_region_hash) -> sector_t;
    pub fn dm_rh_get_region_key(reg: *mut dm_region) -> region_t;

    /* Get/set/update region state (and dirty log). */
    pub fn dm_rh_get_state(rh: *mut dm_region_hash, region: region_t, may_block: i32) -> i32;
    pub fn dm_rh_set_state(
        rh: *mut dm_region_hash,
        region: region_t,
        state: dm_rh_region_states,
        may_block: i32,
    );

    /* Non-zero errors_handled leaves the state of the region NOSYNC */
    pub fn dm_rh_update_states(rh: *mut dm_region_hash, errors_handled: i32);

    /* Flush the region hash and dirty log. */
    pub fn dm_rh_flush(rh: *mut dm_region_hash) -> i32;

    /* Inc/dec pending count on regions. */
    pub fn dm_rh_inc_pending(rh: *mut dm_region_hash, bios: *mut bio_list);
    pub fn dm_rh_dec(rh: *mut dm_region_hash, region: region_t);

    /* Delay bios on regions. */
    pub fn dm_rh_delay(rh: *mut dm_region_hash, bio: *mut bio);

    pub fn dm_rh_mark_nosync(rh: *mut dm_region_hash, bio: *mut bio);

    /*
     * Region recovery control.
     */

    /* Prepare some regions for recovery by starting to quiesce them. */
    pub fn dm_rh_recovery_prepare(rh: *mut dm_region_hash);

    /* Try fetching a quiesced region for recovery. */
    pub fn dm_rh_recovery_start(rh: *mut dm_region_hash) -> *mut dm_region;

    /* Report recovery end on a region. */
    pub fn dm_rh_recovery_end(reg: *mut dm_region, error: i32);

    /* Returns number of regions with recovery work outstanding. */
    pub fn dm_rh_recovery_in_flight(rh: *mut dm_region_hash) -> i32;

    /* Start/stop recovery. */
    pub fn dm_rh_start_recovery(rh: *mut dm_region_hash);
    pub fn dm_rh_stop_recovery(rh: *mut dm_region_hash);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
