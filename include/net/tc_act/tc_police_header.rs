/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <net/act_api.h> in the C source.

#[repr(C)]
pub struct tcf_police_params {
    pub action: i32,
    pub tcfp_result: i32,
    pub tcfp_ewma_rate: u32,
    pub tcfp_mtu: u32,
    pub tcfp_burst: i64,
    pub tcfp_mtu_ptoks: i64,
    pub tcfp_pkt_burst: i64,
    pub rate: psched_ratecfg,
    pub rate_present: bool,
    pub peak: psched_ratecfg,
    pub peak_present: bool,
    pub ppsrate: psched_pktrate,
    pub pps_present: bool,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_police {
    pub common: tc_action,
    pub params: *mut tcf_police_params,
    pub tcfp_lock: spinlock_t,
    pub tcfp_toks: i64,
    pub tcfp_ptoks: i64,
    pub tcfp_pkttoks: i64,
    pub tcfp_t_c: i64,
}

#[inline]
pub unsafe fn to_police(pc: *const tc_action) -> *mut tcf_police {
    pc as *mut tcf_police
}

/* old policer structure from before tc actions */
#[repr(C)]
pub struct tc_police_compat {
    pub index: u32,
    pub action: i32,
    pub limit: u32,
    pub burst: u32,
    pub mtu: u32,
    pub rate: tc_ratespec,
    pub peakrate: tc_ratespec,
}

#[inline]
pub unsafe fn tcf_police_rate_bytes_ps(act: *const tc_action) -> u64 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    (*params).rate.rate_bytes_ps
}

#[inline]
pub unsafe fn tcf_police_burst(act: *const tc_action) -> u32 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    /*
     *  "rate" * "burst"
     * = ---------------- bytes/second
     *     NSEC_PER_SEC
     */
    div_u64((*params).tcfp_burst * (*params).rate.rate_bytes_ps, NSEC_PER_SEC)
}

#[inline]
pub unsafe fn tcf_police_rate_pkt_ps(act: *const tc_action) -> u64 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    (*params).ppsrate.rate_pkts_ps
}

#[inline]
pub unsafe fn tcf_police_burst_pkt(act: *const tc_action) -> u32 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    div_u64((*params).tcfp_pkt_burst * (*params).ppsrate.rate_pkts_ps, NSEC_PER_SEC)
}

#[inline]
pub unsafe fn tcf_police_tcfp_mtu(act: *const tc_action) -> u32 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    (*params).tcfp_mtu
}

#[inline]
pub unsafe fn tcf_police_peakrate_bytes_ps(act: *const tc_action) -> u64 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    (*params).peak.rate_bytes_ps
}

#[inline]
pub unsafe fn tcf_police_tcfp_ewma_rate(act: *const tc_action) -> u32 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    (*params).tcfp_ewma_rate
}

#[inline]
pub unsafe fn tcf_police_rate_overhead(act: *const tc_action) -> u16 {
    let police = to_police(act);
    let params = rcu_dereference_protected(
        (*police).params,
        lockdep_is_held(&(*police).common.tcf_lock),
    );
    (*params).rate.overhead
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
