/* SPDX-License-Identifier: GPL-2.0
 *
 * Legacy blkg rwstat helpers enabled by CONFIG_BLK_CGROUP_RWSTAT.
 * Do not use in new code.
 */
// Dependency declarations from "blk-cgroup-rwstat.h" are supplied elsewhere.

extern "C" {
    fn percpu_counter_init_many(
        cpu_cnt: *mut percpu_counter,
        amount: i64,
        gfp: gfp_t,
        nr: c_uint,
    ) -> c_int;
    fn percpu_counter_destroy_many(cpu_cnt: *mut percpu_counter, nr: c_uint);
    fn atomic64_set(v: *mut atomic64_t, i: i64);
    fn blkg_dev_name(blkg: *mut blkcg_gq) -> *const c_char;
    fn seq_printf(sf: *mut seq_file, fmt: *const c_char, ...);
    fn blkg_rwstat_read(rwstat: *mut c_void, sample: *mut blkg_rwstat_sample);
    fn blkg_rwstat_read_counter(rwstat: *mut blkg_rwstat, i: c_uint) -> u64;
    fn rcu_read_lock_held() -> bool;
    fn memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
}

#[repr(C)]
pub struct percpu_counter {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic64_t {
    _private: [u8; 0],
}
pub type gfp_t = c_uint;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_char = i8;
pub type c_void = core::ffi::c_void;

pub const BLKG_RWSTAT_NR: c_uint = 5;
pub const BLKG_RWSTAT_READ: usize = 0;
pub const BLKG_RWSTAT_WRITE: usize = 1;
pub const BLKG_RWSTAT_SYNC: usize = 2;
pub const BLKG_RWSTAT_ASYNC: usize = 3;
pub const BLKG_RWSTAT_DISCARD: usize = 4;

#[repr(C)]
pub struct blkg_rwstat {
    pub cpu_cnt: [percpu_counter; BLKG_RWSTAT_NR as usize],
    pub aux_cnt: [atomic64_t; BLKG_RWSTAT_NR as usize],
}

#[repr(C)]
pub struct blkg_rwstat_sample {
    pub cnt: [u64; BLKG_RWSTAT_NR as usize],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct blkcg_gq {
    pub online: bool,
    _private: [u8; 0],
}
#[repr(C)]
pub struct blkcg_policy {
    _private: [u8; 0],
}
#[repr(C)]
pub struct blkg_policy_data {
    pub blkg: *mut blkcg_gq,
    _private: [u8; 0],
}
#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn blkg_rwstat_init(rwstat: *mut blkg_rwstat, gfp: gfp_t) -> c_int {
    let ret = percpu_counter_init_many((*rwstat).cpu_cnt.as_mut_ptr(), 0, gfp, BLKG_RWSTAT_NR);
    if ret != 0 {
        return ret;
    }

    for i in 0..BLKG_RWSTAT_NR as usize {
        atomic64_set(&mut (*rwstat).aux_cnt[i], 0);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn blkg_rwstat_exit(rwstat: *mut blkg_rwstat) {
    percpu_counter_destroy_many((*rwstat).cpu_cnt.as_mut_ptr(), BLKG_RWSTAT_NR);
}

#[no_mangle]
pub unsafe extern "C" fn __blkg_prfill_rwstat(
    sf: *mut seq_file,
    pd: *mut blkg_policy_data,
    rwstat: *const blkg_rwstat_sample,
) -> u64 {
    static RWSTR: [&[u8]; BLKG_RWSTAT_NR as usize] = [
        b"Read\0", b"Write\0", b"Sync\0", b"Async\0", b"Discard\0",
    ];
    let dname = blkg_dev_name((*pd).blkg);
    if dname.is_null() {
        return 0;
    }

    for i in 0..BLKG_RWSTAT_NR as usize {
        seq_printf(sf, b"%s %s %llu\n\0".as_ptr() as *const c_char,
            dname, RWSTR[i].as_ptr() as *const c_char, (*rwstat).cnt[i]);
    }

    let v = (*rwstat).cnt[BLKG_RWSTAT_READ]
        .wrapping_add((*rwstat).cnt[BLKG_RWSTAT_WRITE])
        .wrapping_add((*rwstat).cnt[BLKG_RWSTAT_DISCARD]);
    seq_printf(sf, b"%s Total %llu\n\0".as_ptr() as *const c_char, dname, v);
    v
}

#[no_mangle]
pub unsafe extern "C" fn blkg_prfill_rwstat(
    sf: *mut seq_file,
    pd: *mut blkg_policy_data,
    off: c_int,
) -> u64 {
    let mut rwstat = blkg_rwstat_sample { cnt: [0; BLKG_RWSTAT_NR as usize] };
    blkg_rwstat_read((pd as *mut u8).offset(off as isize) as *mut c_void, &mut rwstat);
    __blkg_prfill_rwstat(sf, pd, &rwstat)
}

#[no_mangle]
pub unsafe extern "C" fn blkg_rwstat_recursive_sum(
    blkg: *mut blkcg_gq,
    pol: *mut blkcg_policy,
    off: c_int,
    sum: *mut blkg_rwstat_sample,
) {
    // WARN_ON_ONCE(!rcu_read_lock_held());
    let _ = rcu_read_lock_held();
    memset(sum as *mut c_void, 0, core::mem::size_of::<blkg_rwstat_sample>());

    // blkg_for_each_descendant_pre expands to the kernel hierarchy iterator.
    blkg_for_each_descendant_pre!(pos_blkg, pos_css, blkg, {
        let mut rwstat: *mut blkg_rwstat;

        if !(*pos_blkg).online {
            continue;
        }

        if !pol.is_null() {
            let pd = blkg_to_pd(pos_blkg, pol);
            if pd.is_null() {
                continue;
            }
            rwstat = (pd as *mut u8).offset(off as isize) as *mut blkg_rwstat;
        } else {
            rwstat = (pos_blkg as *mut u8).offset(off as isize) as *mut blkg_rwstat;
        }

        for i in 0..BLKG_RWSTAT_NR as usize {
            (*sum).cnt[i] = (*sum).cnt[i].wrapping_add(blkg_rwstat_read_counter(rwstat, i as c_uint));
        }
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
