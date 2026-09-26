/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_FUTEX and related CONFIG_* conditionals are represented as Rust
 * feature conditions; included kernel types are expected to be provided by
 * other translated units. */

#[cfg(CONFIG_FUTEX)]
pub struct compat_robust_list_head;
#[cfg(CONFIG_FUTEX)]
pub struct futex_pi_state;
#[cfg(CONFIG_FUTEX)]
pub struct robust_list_head;

#[cfg(CONFIG_FUTEX)]
pub struct futex_sched_data {
    pub robust_list: *mut robust_list_head,
    #[cfg(CONFIG_COMPAT)]
    pub compat_robust_list: *mut compat_robust_list_head,
    pub pi_state_list: list_head,
    pub pi_state_cache: *mut futex_pi_state,
    pub exit_mutex: mutex,
    pub state: ::core::ffi::c_uint,
}

#[cfg(all(CONFIG_FUTEX, CONFIG_FUTEX_PRIVATE_HASH))]
pub struct futex_mm_phash {
    pub lock: mutex,
    pub hash: *mut futex_private_hash,
    pub hash_new: *mut futex_private_hash,
    pub batches: ::core::ffi::c_ulong,
    pub rcu: rcu_head,
    pub atomic: atomic_long_t,
    pub r#ref: *mut ::core::ffi::c_uint,
}

#[cfg(all(CONFIG_FUTEX, not(CONFIG_FUTEX_PRIVATE_HASH)))]
pub struct futex_mm_phash;

#[cfg(all(CONFIG_FUTEX, CONFIG_FUTEX_ROBUST_UNLOCK))]
pub struct futex_unlock_cs_range {
    pub start_ip: ::core::ffi::c_ulong,
    pub len: ::core::ffi::c_uint,
    pub pop_size32: ::core::ffi::c_uint,
}

#[cfg(all(CONFIG_FUTEX, CONFIG_FUTEX_ROBUST_UNLOCK))]
pub const FUTEX_ROBUST_MAX_CS_RANGES: usize =
    1 + (cfg!(CONFIG_COMPAT) as usize);

#[cfg(all(CONFIG_FUTEX, CONFIG_FUTEX_ROBUST_UNLOCK))]
pub struct futex_unlock_cs_ranges {
    pub cs_ranges: [futex_unlock_cs_range; FUTEX_ROBUST_MAX_CS_RANGES],
}

#[cfg(all(CONFIG_FUTEX, not(CONFIG_FUTEX_ROBUST_UNLOCK)))]
pub struct futex_unlock_cs_ranges;

#[cfg(CONFIG_FUTEX)]
pub struct futex_mm_data {
    pub phash: futex_mm_phash,
    pub unlock: futex_unlock_cs_ranges,
}

#[cfg(not(CONFIG_FUTEX))]
pub struct futex_sched_data;

#[cfg(not(CONFIG_FUTEX))]
pub struct futex_mm_data;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
