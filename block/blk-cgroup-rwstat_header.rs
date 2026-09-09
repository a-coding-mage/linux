/* SPDX-License-Identifier: GPL-2.0
 *
 * Legacy blkg rwstat helpers enabled by CONFIG_BLK_CGROUP_RWSTAT.
 * Do not use in new code.
 */

// Dependency supplied by the corresponding C header: blk-cgroup.h.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum blkg_rwstat_type {
    BLKG_RWSTAT_READ,
    BLKG_RWSTAT_WRITE,
    BLKG_RWSTAT_SYNC,
    BLKG_RWSTAT_ASYNC,
    BLKG_RWSTAT_DISCARD,
    BLKG_RWSTAT_NR,
    BLKG_RWSTAT_TOTAL = BLKG_RWSTAT_NR as isize,
}

/*
 * blkg_[rw]stat->aux_cnt is excluded for local stats but included for
 * recursive.  Used to carry stats of dead children.
 */
#[repr(C)]
pub struct blkg_rwstat {
    pub cpu_cnt: [percpu_counter; BLKG_RWSTAT_NR as usize],
    pub aux_cnt: [atomic64_t; BLKG_RWSTAT_NR as usize],
}

#[repr(C)]
pub struct blkg_rwstat_sample {
    pub cnt: [u64; BLKG_RWSTAT_NR as usize],
}

#[inline]
pub unsafe fn blkg_rwstat_read_counter(
    rwstat: *mut blkg_rwstat,
    idx: c_uint,
) -> u64 {
    atomic64_read(&(*rwstat).aux_cnt[idx as usize])
        .wrapping_add(percpu_counter_sum_positive(&(*rwstat).cpu_cnt[idx as usize]))
}

extern "C" {
    pub fn blkg_rwstat_init(rwstat: *mut blkg_rwstat, gfp: gfp_t) -> c_int;
    pub fn blkg_rwstat_exit(rwstat: *mut blkg_rwstat);
    pub fn __blkg_prfill_rwstat(
        sf: *mut seq_file,
        pd: *mut blkg_policy_data,
        rwstat: *const blkg_rwstat_sample,
    ) -> u64;
    pub fn blkg_prfill_rwstat(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64;
    pub fn blkg_rwstat_recursive_sum(
        blkg: *mut blkcg_gq,
        pol: *mut blkcg_policy,
        off: c_int,
        sum: *mut blkg_rwstat_sample,
    );
}

/**
 * blkg_rwstat_add - add a value to a blkg_rwstat
 * @rwstat: target blkg_rwstat
 * @opf: REQ_OP and flags
 * @val: value to add
 *
 * Add @val to @rwstat.  The counters are chosen according to @rw.  The
 * caller is responsible for synchronizing calls to this function.
 */
#[inline]
pub unsafe fn blkg_rwstat_add(rwstat: *mut blkg_rwstat, opf: blk_opf_t, val: u64) {
    let mut cnt: *mut percpu_counter;

    if op_is_discard(opf) {
        cnt = &mut (*rwstat).cpu_cnt[BLKG_RWSTAT_DISCARD as usize];
    } else if op_is_write(opf) {
        cnt = &mut (*rwstat).cpu_cnt[BLKG_RWSTAT_WRITE as usize];
    } else {
        cnt = &mut (*rwstat).cpu_cnt[BLKG_RWSTAT_READ as usize];
    }

    percpu_counter_add_batch(cnt, val, BLKG_STAT_CPU_BATCH);

    if op_is_sync(opf) {
        cnt = &mut (*rwstat).cpu_cnt[BLKG_RWSTAT_SYNC as usize];
    } else {
        cnt = &mut (*rwstat).cpu_cnt[BLKG_RWSTAT_ASYNC as usize];
    }

    percpu_counter_add_batch(cnt, val, BLKG_STAT_CPU_BATCH);
}

/**
 * blkg_rwstat_read - read the current values of a blkg_rwstat
 * @rwstat: blkg_rwstat to read
 * @result: where to put the current values
 *
 * Read the current snapshot of @rwstat and return it in the @result counts.
 */
#[inline]
pub unsafe fn blkg_rwstat_read(rwstat: *mut blkg_rwstat, result: *mut blkg_rwstat_sample) {
    let mut i: c_int = 0;
    while i < BLKG_RWSTAT_NR as c_int {
        (*result).cnt[i as usize] =
            percpu_counter_sum_positive(&(*rwstat).cpu_cnt[i as usize]);
        i += 1;
    }
}

/**
 * blkg_rwstat_total - read the total count of a blkg_rwstat
 * @rwstat: blkg_rwstat to read
 *
 * Return the total count of @rwstat regardless of the IO direction.  This
 * function can be called without synchronization and takes care of u64
 * atomicity.
 */
#[inline]
pub unsafe fn blkg_rwstat_total(rwstat: *mut blkg_rwstat) -> u64 {
    let mut tmp = blkg_rwstat_sample { cnt: [0; BLKG_RWSTAT_NR as usize] };
    blkg_rwstat_read(rwstat, &mut tmp);
    tmp.cnt[BLKG_RWSTAT_READ as usize].wrapping_add(tmp.cnt[BLKG_RWSTAT_WRITE as usize])
}

/**
 * blkg_rwstat_reset - reset a blkg_rwstat
 * @rwstat: blkg_rwstat to reset
 */
#[inline]
pub unsafe fn blkg_rwstat_reset(rwstat: *mut blkg_rwstat) {
    let mut i: c_int = 0;
    while i < BLKG_RWSTAT_NR as c_int {
        percpu_counter_set(&mut (*rwstat).cpu_cnt[i as usize], 0);
        atomic64_set(&mut (*rwstat).aux_cnt[i as usize], 0);
        i += 1;
    }
}

/**
 * blkg_rwstat_add_aux - add a blkg_rwstat into another's aux count
 * @to: the destination blkg_rwstat
 * @from: the source
 *
 * Add @from's count including the aux one to @to's aux count.
 */
#[inline]
pub unsafe fn blkg_rwstat_add_aux(to: *mut blkg_rwstat, from: *mut blkg_rwstat) {
    let mut sum = [0u64; BLKG_RWSTAT_NR as usize];
    let mut i: c_int = 0;

    while i < BLKG_RWSTAT_NR as c_int {
        sum[i as usize] = percpu_counter_sum_positive(&(*from).cpu_cnt[i as usize]);
        i += 1;
    }

    i = 0;
    while i < BLKG_RWSTAT_NR as c_int {
        atomic64_add(
            sum[i as usize].wrapping_add(atomic64_read(&(*from).aux_cnt[i as usize])),
            &mut (*to).aux_cnt[i as usize],
        );
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
