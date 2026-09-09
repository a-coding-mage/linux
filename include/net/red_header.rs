/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

/* Random Early Detection (RED) algorithm.
 *
 * This is the divisionless version of RED described by Floyd and Jacobson.
 * The original header's explanatory algorithm documentation is retained here
 * in the declarations and comments below.
 */

pub const RED_ONE_PERCENT: u32 = ((1u64 << 32) / 100) as u32;
pub const MAX_P_MIN: u32 = 1 * RED_ONE_PERCENT;
pub const MAX_P_MAX: u32 = 50 * RED_ONE_PERCENT;
pub const RED_STAB_SIZE: usize = 256;
pub const RED_STAB_MASK: usize = RED_STAB_SIZE - 1;

#[repr(C)]
pub struct red_stats {
    pub prob_drop: u32,   /* Early probability drops */
    pub prob_mark: u32,   /* Early probability marks */
    pub forced_drop: u32, /* Forced drops, qavg > max_thresh */
    pub forced_mark: u32, /* Forced marks, qavg > max_thresh */
    pub pdrop: u32,       /* Drops due to queue limits */
}

#[repr(C)]
pub struct red_parms {
    pub qth_min: u32,
    pub qth_max: u32,
    pub Scell_max: u32,
    pub max_P: u32,
    pub max_P_reciprocal: reciprocal_value,
    pub qth_delta: u32,
    pub target_min: u32,
    pub target_max: u32,
    pub Scell_log: u8,
    pub Wlog: u8,
    pub Plog: u8,
    pub Stab: [u8; RED_STAB_SIZE],
}

#[repr(C)]
pub struct red_vars {
    pub qcount: i32,
    pub qR: u32,
    pub qavg: usize,
    pub qidlestart: ktime_t,
}

#[inline]
pub fn red_maxp(Plog: u8) -> u32 {
    if Plog < 32 { u32::MAX >> Plog } else { u32::MAX }
}

#[inline]
pub unsafe fn red_set_vars(v: *mut red_vars) {
    (*v).qavg = 0;
    (*v).qcount = -1;
}

#[inline]
pub unsafe fn red_check_params(qth_min: u32, qth_max: u32, Wlog: u8,
                               Scell_log: u8, stab: *const u8) -> bool {
    if (fls(qth_min) + Wlog as i32) >= 32 || (fls(qth_max) + Wlog as i32) >= 32 || Scell_log >= 32 || qth_max < qth_min { return false; }
    if !stab.is_null() {
        for i in 0..RED_STAB_SIZE { if *stab.add(i) >= 32 { return false; } }
    }
    true
}

#[inline]
pub unsafe fn red_get_flags(qopt_flags: u8, historic_mask: u8, flags_attr: *mut nlattr,
                            supported_mask: u8, p_flags: *mut nla_bitfield32,
                            p_userbits: *mut u8, extack: *mut netlink_ext_ack) -> i32 {
    let flags: nla_bitfield32;
    if qopt_flags != 0 && !flags_attr.is_null() { NL_SET_ERR_MSG_MOD(extack, "flags should be passed either through qopt, or through a dedicated attribute"); return -EINVAL; }
    if !flags_attr.is_null() { flags = nla_get_bitfield32(flags_attr); }
    else { flags = nla_bitfield32 { selector: historic_mask, value: qopt_flags & historic_mask }; }
    *p_flags = flags;
    *p_userbits = qopt_flags & !historic_mask;
    0
}

#[inline]
pub unsafe fn red_validate_flags(flags: u8, extack: *mut netlink_ext_ack) -> i32 {
    if (flags & TC_RED_NODROP) != 0 && (flags & TC_RED_ECN) == 0 { NL_SET_ERR_MSG_MOD(extack, "nodrop mode is only meaningful with ECN"); return -EINVAL; }
    0
}

#[inline]
pub unsafe fn red_set_parms(p: *mut red_parms, qth_min: u32, qth_max: u32, Wlog: u8,
                            Plog: u8, Scell_log: u8, stab: *const u8, mut max_P: u32) {
    let mut delta = qth_max.wrapping_sub(qth_min);
    (*p).qth_min = qth_min << Wlog;
    (*p).qth_max = qth_max << Wlog;
    (*p).Wlog = Wlog; (*p).Plog = Plog;
    if delta == 0 { delta = 1; }
    (*p).qth_delta = delta;
    if max_P == 0 { max_P = red_maxp(Plog).wrapping_mul(delta); }
    (*p).max_P = max_P;
    let max_p_delta = core::cmp::max(max_P / delta, 1);
    (*p).max_P_reciprocal = reciprocal_value(max_p_delta);
    delta /= 5;
    (*p).target_min = qth_min + 2 * delta;
    (*p).target_max = qth_min + 3 * delta;
    (*p).Scell_log = Scell_log;
    (*p).Scell_max = 255 << Scell_log;
    if !stab.is_null() { core::ptr::copy_nonoverlapping(stab, (*p).Stab.as_mut_ptr(), RED_STAB_SIZE); }
}

#[inline] pub unsafe fn red_is_idling(v: *const red_vars) -> i32 { ((*v).qidlestart != 0) as i32 }
#[inline] pub unsafe fn red_start_of_idle_period(v: *mut red_vars) { (*v).qidlestart = ktime_get(); }
#[inline] pub unsafe fn red_end_of_idle_period(v: *mut red_vars) { (*v).qidlestart = 0; }
#[inline] pub unsafe fn red_restart(v: *mut red_vars) { red_end_of_idle_period(v); (*v).qavg = 0; (*v).qcount = -1; }

#[inline]
pub unsafe fn red_calc_qavg_from_idle_time(p: *const red_parms, v: *const red_vars) -> usize {
    let delta = ktime_us_delta(ktime_get(), (*v).qidlestart);
    let us_idle = core::cmp::min(delta, (*p).Scell_max as _);
    let shift = (*p).Stab[((us_idle as usize >> (*p).Scell_log) & RED_STAB_MASK)];
    if shift != 0 { (*v).qavg >> shift } else {
        let idle = ((*v).qavg as u64 * us_idle as u64 >> (*p).Scell_log) as usize;
        if idle < (*v).qavg >> 1 { (*v).qavg - idle } else { (*v).qavg >> 1 }
    }
}

#[inline]
pub unsafe fn red_calc_qavg_no_idle_time(p: *const red_parms, v: *const red_vars, backlog: u32) -> usize {
    (*v).qavg + ((backlog as usize) - ((*v).qavg >> (*p).Wlog))
}
#[inline]
pub unsafe fn red_calc_qavg(p: *const red_parms, v: *const red_vars, backlog: u32) -> usize {
    if red_is_idling(v) == 0 { red_calc_qavg_no_idle_time(p, v, backlog) } else { red_calc_qavg_from_idle_time(p, v) }
}
#[inline] pub unsafe fn red_random(p: *const red_parms) -> u32 { reciprocal_divide(get_random_u32(), (*p).max_P_reciprocal) }
#[inline] pub unsafe fn red_mark_probability(p: *const red_parms, v: *const red_vars, qavg: usize) -> i32 { (!(((qavg - (*p).qth_min as usize) >> (*p).Wlog) * (*v).qcount as usize < (*v).qR as usize)) as i32 }

pub const RED_BELOW_MIN_THRESH: i32 = 0;
pub const RED_BETWEEN_TRESH: i32 = 1;
pub const RED_ABOVE_MAX_TRESH: i32 = 2;
#[inline] pub unsafe fn red_cmp_thresh(p: *const red_parms, qavg: usize) -> i32 { if qavg < (*p).qth_min as usize { RED_BELOW_MIN_THRESH } else if qavg >= (*p).qth_max as usize { RED_ABOVE_MAX_TRESH } else { RED_BETWEEN_TRESH } }
pub const RED_DONT_MARK: i32 = 0;
pub const RED_PROB_MARK: i32 = 1;
pub const RED_HARD_MARK: i32 = 2;

#[inline]
pub unsafe fn red_action(p: *const red_parms, v: *mut red_vars, qavg: usize) -> i32 {
    match red_cmp_thresh(p, qavg) {
        RED_BELOW_MIN_THRESH => { (*v).qcount = -1; RED_DONT_MARK },
        RED_BETWEEN_TRESH => { (*v).qcount += 1; if (*v).qcount != 0 { if red_mark_probability(p, v, qavg) != 0 { (*v).qcount = 0; (*v).qR = red_random(p); return RED_PROB_MARK; } } else { (*v).qR = red_random(p); } RED_DONT_MARK },
        RED_ABOVE_MAX_TRESH => { (*v).qcount = -1; RED_HARD_MARK },
        _ => { BUG(); RED_DONT_MARK }
    }
}

#[inline]
pub unsafe fn red_adaptative_algo(p: *mut red_parms, v: *mut red_vars) {
    let mut qavg = (*v).qavg;
    if red_is_idling(v) != 0 { qavg = red_calc_qavg_from_idle_time(p, v); }
    qavg >>= (*p).Wlog;
    if qavg > (*p).target_max as usize && (*p).max_P <= MAX_P_MAX { (*p).max_P += core::cmp::min(MAX_P_MIN, (*p).max_P / 4); }
    else if qavg < (*p).target_min as usize && (*p).max_P >= MAX_P_MIN { (*p).max_P = ((*p).max_P / 10) * 9; }
    let max_p_delta = core::cmp::max((*p).max_P / (*p).qth_delta, 1);
    (*p).max_P_reciprocal = reciprocal_value(max_p_delta);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
