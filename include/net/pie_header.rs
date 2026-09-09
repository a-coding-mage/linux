/* SPDX-License-Identifier: GPL-2.0-only */

// C header dependencies are supplied by the surrounding kernel translation.

pub const MAX_PROB: u64 = u64::MAX >> 8; // BITS_PER_BYTE
pub const DTIME_INVALID: u64 = u64::MAX;
pub const QUEUE_THRESHOLD: u32 = 16384;
pub const DQCOUNT_INVALID: u64 = u64::MAX;
pub const PIE_SCALE: u32 = 8;

/// struct pie_params - contains pie parameters
#[repr(C)]
pub struct pie_params {
    pub target: psched_time_t,
    pub tupdate: u32,
    pub limit: u32,
    pub alpha: u32,
    pub beta: u32,
    pub ecn: u8,
    pub bytemode: u8,
    pub dq_rate_estimator: u8,
}

/// struct pie_vars - contains pie variables
#[repr(C)]
pub struct pie_vars {
    pub qdelay: psched_time_t,
    pub qdelay_old: psched_time_t,
    pub burst_time: psched_time_t,
    pub dq_tstamp: psched_time_t,
    pub prob: u64,
    pub accu_prob: u64,
    pub dq_count: u64,
    pub avg_dq_rate: u32,
    pub backlog_old: u32,
}

/// struct pie_stats - contains pie stats
#[repr(C)]
pub struct pie_stats {
    pub packets_in: u32,
    pub dropped: u32,
    pub overlimit: u32,
    pub ecn_mark: u32,
    pub maxq: u32,
}

/// struct pie_skb_cb - contains private skb vars
#[repr(C)]
pub struct pie_skb_cb {
    pub enqueue_time: psched_time_t,
    pub mem_usage: u32,
}

pub unsafe fn pie_params_init(params: *mut pie_params) {
    (*params).target = PSCHED_NS2TICKS(15 * NSEC_PER_MSEC);
    (*params).tupdate = usecs_to_jiffies(15 * USEC_PER_MSEC) as u32;
    (*params).limit = 1000;
    (*params).alpha = 2;
    (*params).beta = 20;
    (*params).ecn = 0;
    (*params).bytemode = 0;
    (*params).dq_rate_estimator = 0;
}

pub unsafe fn pie_vars_init(vars: *mut pie_vars) {
    (*vars).burst_time = PSCHED_NS2TICKS(150 * NSEC_PER_MSEC);
    (*vars).dq_tstamp = DTIME_INVALID as psched_time_t;
    (*vars).accu_prob = 0;
    (*vars).dq_count = DQCOUNT_INVALID;
    WRITE_ONCE(&mut (*vars).avg_dq_rate, 0);
}

pub unsafe fn get_pie_cb(skb: *const sk_buff) -> *mut pie_skb_cb {
    qdisc_cb_private_validate(skb, core::mem::size_of::<pie_skb_cb>());
    qdisc_skb_cb(skb).data as *mut pie_skb_cb
}

pub unsafe fn pie_get_enqueue_time(skb: *const sk_buff) -> psched_time_t {
    (*get_pie_cb(skb)).enqueue_time
}

pub unsafe fn pie_set_enqueue_time(skb: *mut sk_buff) {
    (*get_pie_cb(skb)).enqueue_time = psched_get_time();
}

unsafe extern "C" {
    pub fn pie_drop_early(
        sch: *mut Qdisc,
        params: *mut pie_params,
        vars: *mut pie_vars,
        backlog: u32,
        packet_size: u32,
    ) -> bool;

    pub fn pie_process_dequeue(
        skb: *mut sk_buff,
        params: *mut pie_params,
        vars: *mut pie_vars,
        backlog: u32,
    );

    pub fn pie_calculate_probability(
        params: *mut pie_params,
        vars: *mut pie_vars,
        backlog: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
