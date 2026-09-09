/*
 * Codel - The Controlled-Delay Active Queue Management algorithm
 *
 * Copyright and licensing notice preserved from the C source.
 */

// Dependency supplied by the surrounding translation unit: net/inet_ecn.h

unsafe fn codel_params_init(params: *mut codel_params) {
    (*params).interval = MS2TIME(100);
    (*params).target = MS2TIME(5);
    (*params).ce_threshold = CODEL_DISABLED_THRESHOLD;
    (*params).ce_threshold_mask = 0;
    (*params).ce_threshold_selector = 0;
    (*params).ecn = false;
}

unsafe fn codel_vars_init(vars: *mut codel_vars) {
    core::ptr::write_bytes(vars as *mut u8, 0, core::mem::size_of::<codel_vars>());
}

unsafe fn codel_stats_init(stats: *mut codel_stats) {
    (*stats).maxpacket = 0;
}

unsafe fn codel_Newton_step(vars: *mut codel_vars) {
    let invsqrt: u32 = ((*vars).rec_inv_sqrt as u32) << REC_INV_SQRT_SHIFT;
    let invsqrt2: u32 = (((invsqrt as u64).wrapping_mul(invsqrt as u64)) >> 32) as u32;
    let mut val: u64 = (3u64 << 32).wrapping_sub(((*vars).count as u64).wrapping_mul(invsqrt2 as u64));

    val >>= 2;
    val = (val.wrapping_mul(invsqrt as u64)) >> (32 - 2 + 1);
    (*vars).rec_inv_sqrt = (val >> REC_INV_SQRT_SHIFT) as _;
}

unsafe fn codel_control_law(
    t: codel_time_t,
    interval: codel_time_t,
    rec_inv_sqrt: u32,
) -> codel_time_t {
    t + reciprocal_scale(interval, rec_inv_sqrt << REC_INV_SQRT_SHIFT)
}

unsafe fn codel_should_drop(
    skb: *const sk_buff,
    _ctx: *mut core::ffi::c_void,
    vars: *mut codel_vars,
    params: *mut codel_params,
    stats: *mut codel_stats,
    skb_len_func: codel_skb_len_t,
    skb_time_func: codel_skb_time_t,
    backlog: *mut u32,
    now: codel_time_t,
) -> bool {
    let mut ok_to_drop: bool;
    let skb_len: u32;

    if skb.is_null() {
        (*vars).first_above_time = 0;
        return false;
    }

    skb_len = skb_len_func(skb);
    WRITE_ONCE!((*vars).ldelay, now - skb_time_func(skb));

    if skb_len > (*stats).maxpacket {
        WRITE_ONCE!((*stats).maxpacket, skb_len);
    }

    if codel_time_before((*vars).ldelay, (*params).target) || *backlog <= (*params).mtu {
        (*vars).first_above_time = 0;
        return false;
    }
    ok_to_drop = false;
    if (*vars).first_above_time == 0 {
        (*vars).first_above_time = now + (*params).interval;
    } else if codel_time_after(now, (*vars).first_above_time) {
        ok_to_drop = true;
    }
    ok_to_drop
}

unsafe fn codel_dequeue(
    ctx: *mut core::ffi::c_void,
    backlog: *mut u32,
    params: *mut codel_params,
    vars: *mut codel_vars,
    stats: *mut codel_stats,
    skb_len_func: codel_skb_len_t,
    skb_time_func: codel_skb_time_t,
    drop_func: codel_skb_drop_t,
    dequeue_func: codel_skb_dequeue_t,
) -> *mut sk_buff {
    let mut skb = dequeue_func(vars, ctx);
    let now: codel_time_t;
    let mut drop: bool;

    if skb.is_null() {
        (*vars).first_above_time = 0;
        WRITE_ONCE!((*vars).dropping, false);
        return skb;
    }
    now = codel_get_time();
    drop = codel_should_drop(skb, ctx, vars, params, stats, skb_len_func, skb_time_func, backlog, now);
    if (*vars).dropping {
        if !drop {
            WRITE_ONCE!((*vars).dropping, false);
        } else if codel_time_after_eq(now, (*vars).drop_next) {
            while (*vars).dropping && codel_time_after_eq(now, (*vars).drop_next) {
                WRITE_ONCE!((*vars).count, (*vars).count + 1);
                codel_Newton_step(vars);
                if (*params).ecn && INET_ECN_set_ce(skb) {
                    WRITE_ONCE!((*stats).ecn_mark, (*stats).ecn_mark + 1);
                    WRITE_ONCE!((*vars).drop_next, codel_control_law((*vars).drop_next, (*params).interval, (*vars).rec_inv_sqrt));
                    goto end;
                }
                (*stats).drop_len += skb_len_func(skb);
                drop_func(skb, ctx);
                (*stats).drop_count += 1;
                skb = dequeue_func(vars, ctx);
                if !codel_should_drop(skb, ctx, vars, params, stats, skb_len_func, skb_time_func, backlog, now) {
                    WRITE_ONCE!((*vars).dropping, false);
                } else {
                    WRITE_ONCE!((*vars).drop_next, codel_control_law((*vars).drop_next, (*params).interval, (*vars).rec_inv_sqrt));
                }
            }
        }
    } else if drop {
        let delta: u32;
        if (*params).ecn && INET_ECN_set_ce(skb) {
            WRITE_ONCE!((*stats).ecn_mark, (*stats).ecn_mark + 1);
        } else {
            (*stats).drop_len += skb_len_func(skb);
            drop_func(skb, ctx);
            (*stats).drop_count += 1;
            skb = dequeue_func(vars, ctx);
            drop = codel_should_drop(skb, ctx, vars, params, stats, skb_len_func, skb_time_func, backlog, now);
        }
        WRITE_ONCE!((*vars).dropping, true);
        delta = (*vars).count - (*vars).lastcount;
        if delta > 1 && codel_time_before(now - (*vars).drop_next, 16 * (*params).interval) {
            WRITE_ONCE!((*vars).count, delta);
            codel_Newton_step(vars);
        } else {
            WRITE_ONCE!((*vars).count, 1);
            (*vars).rec_inv_sqrt = !0u32 >> REC_INV_SQRT_SHIFT;
        }
        WRITE_ONCE!((*vars).lastcount, (*vars).count);
        WRITE_ONCE!((*vars).drop_next, codel_control_law(now, (*params).interval, (*vars).rec_inv_sqrt));
    }
end:
    if !skb.is_null() && codel_time_after((*vars).ldelay, (*params).ce_threshold) {
        let mut set_ce = true;
        if (*params).ce_threshold_mask != 0 {
            let dsfield = skb_get_dsfield(skb);
            set_ce = dsfield >= 0 && ((dsfield as u8 & (*params).ce_threshold_mask) == (*params).ce_threshold_selector);
        }
        if set_ce && INET_ECN_set_ce(skb) {
            WRITE_ONCE!((*stats).ce_mark, (*stats).ce_mark + 1);
        }
    }
    skb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
