// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_limit
 *
 *	Authors:
 *	Tom Marshall <tommy@home.tig-grr.com>
 *
 *	Mostly copied from netfilter's ipt_limit.c, see that file for
 *	more explanation
 *
 *  September, 2003
 *
 */

// Dependencies supplied by the surrounding kernel translation.

static mut limit_lock: spinlock_t = spinlock_t::new();

const MAX_CPJ: u32 = 0xFFFFFFFF / (HZ * 60 * 60 * 24);

const fn pow2_below2(x: u32) -> u32 { x | (x >> 1) }
const fn pow2_below4(x: u32) -> u32 { pow2_below2(x) | pow2_below2(x >> 2) }
const fn pow2_below8(x: u32) -> u32 { pow2_below4(x) | pow2_below4(x >> 4) }
const fn pow2_below16(x: u32) -> u32 { pow2_below8(x) | pow2_below8(x >> 8) }
const fn pow2_below32(x: u32) -> u32 { pow2_below16(x) | pow2_below16(x >> 16) }
const fn pow2_below32_final(x: u32) -> u32 { (pow2_below32(x) >> 1) + 1 }

const CREDITS_PER_JIFFY: u32 = pow2_below32_final(MAX_CPJ);

unsafe fn ebt_limit_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *mut ebt_limit_info;
    let now: c_ulong = jiffies;

    spin_lock_bh(&raw mut limit_lock);
    (*info).credit += (now - xchg(&raw mut (*info).prev, now)) * CREDITS_PER_JIFFY as c_ulong;
    if (*info).credit > (*info).credit_cap {
        (*info).credit = (*info).credit_cap;
    }

    if (*info).credit >= (*info).cost {
        /* We're not limited. */
        (*info).credit -= (*info).cost;
        spin_unlock_bh(&raw mut limit_lock);
        return true;
    }

    spin_unlock_bh(&raw mut limit_lock);
    false
}

/* Precision saver. */
unsafe fn user2credits(user: u32) -> u32 {
    /* If multiplying would overflow... */
    if user > 0xFFFFFFFF / (HZ * CREDITS_PER_JIFFY) {
        /* Divide first. */
        return (user / EBT_LIMIT_SCALE) * HZ * CREDITS_PER_JIFFY;
    }

    (user * HZ * CREDITS_PER_JIFFY) / EBT_LIMIT_SCALE
}

unsafe fn ebt_limit_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *mut ebt_limit_info;

    /* Check for overflow. */
    if (*info).burst == 0
        || user2credits((*info).avg * (*info).burst) < user2credits((*info).avg)
    {
        pr_info_ratelimited!("overflow, try lower: %u/%u\n", (*info).avg, (*info).burst);
        return -EINVAL;
    }

    /* User avg in seconds * EBT_LIMIT_SCALE: convert to jiffies * 128. */
    (*info).prev = jiffies;
    (*info).credit = user2credits((*info).avg * (*info).burst);
    (*info).credit_cap = user2credits((*info).avg * (*info).burst);
    (*info).cost = user2credits((*info).avg);
    0
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
/*
 * no conversion function needed --
 * only avg/burst have meaningful values in userspace.
 */
#[repr(C)]
struct ebt_compat_limit_info {
    avg: compat_uint_t,
    burst: compat_uint_t,
    prev: compat_ulong_t,
    credit: compat_uint_t,
    credit_cap: compat_uint_t,
    cost: compat_uint_t,
}

static mut ebt_limit_mt_reg: xt_match = xt_match {
    name: c"limit".as_ptr(),
    revision: 0,
    family: NFPROTO_BRIDGE,
    match_: Some(ebt_limit_mt),
    checkentry: Some(ebt_limit_mt_check),
    matchsize: core::mem::size_of::<ebt_limit_info>(),
    usersize: core::mem::offset_of!(ebt_limit_info, prev),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compatsize: core::mem::size_of::<ebt_compat_limit_info>(),
    me: THIS_MODULE,
};

unsafe fn ebt_limit_init() -> c_int {
    xt_register_match(&raw mut ebt_limit_mt_reg)
}

unsafe fn ebt_limit_fini() {
    xt_unregister_match(&raw mut ebt_limit_mt_reg);
}

module_init!(ebt_limit_init);
module_exit!(ebt_limit_fini);
MODULE_DESCRIPTION!("Ebtables: Rate-limit match");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
