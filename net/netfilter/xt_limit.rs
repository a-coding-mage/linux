// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999 Jérôme de Vivie <devivie@info.enserb.u-bordeaux.fr>
 * (C) 1999 Hervé Eychenne <eychenne@info.enserb.u-bordeaux.fr>
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */
// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// C dependencies supplied by the surrounding kernel translation unit:
// linux/slab.h, linux/module.h, linux/skbuff.h, linux/interrupt.h,
// linux/netfilter/x_tables.h, linux/netfilter/xt_limit.h

#[repr(C)]
struct xt_limit_priv {
    prev: usize,
    credit: u32,
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Herve Eychenne <rv@wallfire.org>");
// MODULE_DESCRIPTION("Xtables: rate-limit match");
// MODULE_ALIAS("ipt_limit");
// MODULE_ALIAS("ip6t_limit");

/* The algorithm used is the Simple Token Bucket Filter (TBF)
 * see net/sched/sch_tbf.c in the linux source tree
 */

/* Rusty: This is my (non-mathematically-inclined) understanding of
   this algorithm.  The `average rate' in jiffies becomes your initial
   amount of credit `credit' and the most credit you can ever have
   `credit_cap'.  The `peak rate' becomes the cost of passing the
   test, `cost'.

   `prev' tracks the last packet hit: you gain one credit per jiffy.
   If you get credit balance more than this, the extra credit is
   discarded.  Every time the match passes, you lose `cost' credits;
   if you don't have that many, the test fails.

   See Alexey's formal explanation in net/sched/sch_tbf.c.

   To get the maximum range, we multiply by this factor (ie. you get N
   credits per jiffy).  We want to allow a rate as low as 1 per day
   (slowest userspace tool allows), which means
   CREDITS_PER_JIFFY*HZ*60*60*24 < 2^32. ie. */
const MAX_CPJ: u32 = 0xFFFFFFFF / (HZ * 60 * 60 * 24);

const fn pow2_below32(x: u32) -> u32 {
    let x = x | (x >> 1);
    let x = x | (x >> 2) | ((x | (x >> 1)) >> 2);
    let x = x | (x >> 4) | ((x | (x >> 1) | ((x | (x >> 1)) >> 2)) >> 4);
    let x = x | (x >> 8);
    let x = x | (x >> 16);
    x
}

const POW2_BELOW32: u32 = (pow2_below32(MAX_CPJ) >> 1) + 1;
const CREDITS_PER_JIFFY: u32 = POW2_BELOW32;

unsafe fn limit_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let r: *const xt_rateinfo = (*par).matchinfo as *const xt_rateinfo;
    let priv_: *mut xt_limit_priv = (*r).master as *mut xt_limit_priv;
    let mut now: usize;
    let mut old_credit: u32;
    let mut new_credit: u32;
    let mut credit_increase: u32 = 0;
    let ret: bool;

    let _ = skb;
    if (READ_ONCE((*priv_).credit) < (*r).cost)
        && (READ_ONCE((*priv_).prev) == jiffies)
    {
        return false;
    }

    loop {
        now = jiffies;
        credit_increase = credit_increase.wrapping_add(
            (now.wrapping_sub(xchg(&mut (*priv_).prev, now)) as u32)
                .wrapping_mul(CREDITS_PER_JIFFY),
        );
        old_credit = READ_ONCE((*priv_).credit);
        new_credit = old_credit;
        new_credit = new_credit.wrapping_add(credit_increase);
        if new_credit > (*r).credit_cap {
            new_credit = (*r).credit_cap;
        }
        if new_credit >= (*r).cost {
            ret = true;
            new_credit = new_credit.wrapping_sub((*r).cost);
        } else {
            ret = false;
        }
        if cmpxchg(&mut (*priv_).credit, old_credit, new_credit) == old_credit {
            break;
        }
    }
    ret
}

/* Precision saver. */
fn user2credits(user: u32) -> u32 {
    /* If multiplying would overflow... */
    if user > 0xFFFFFFFF / (HZ * CREDITS_PER_JIFFY) {
        /* Divide first. */
        return (user / XT_LIMIT_SCALE) * HZ * CREDITS_PER_JIFFY;
    }

    (user * HZ * CREDITS_PER_JIFFY) / XT_LIMIT_SCALE
}

unsafe fn limit_mt_check(par: *const xt_mtchk_param) -> i32 {
    let r: *mut xt_rateinfo = (*par).matchinfo as *mut xt_rateinfo;
    let priv_: *mut xt_limit_priv;

    /* Check for overflow. */
    if (*r).burst == 0
        || user2credits((*r).avg * (*r).burst) < user2credits((*r).avg)
    {
        pr_info_ratelimited!("Overflow, try lower: %u/%u\n", (*r).avg, (*r).burst);
        return -ERANGE;
    }

    priv_ = kmalloc_obj::<xt_limit_priv>();
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* For SMP, we only want to use one set of state. */
    (*r).master = priv_ as *mut _;
    /* User avg in seconds * XT_LIMIT_SCALE: convert to jiffies *
       128. */
    (*priv_).prev = jiffies;
    (*priv_).credit = user2credits((*r).avg * (*r).burst); /* Credits full. */
    if (*r).cost == 0 {
        (*r).credit_cap = (*priv_).credit; /* Credits full. */
        (*r).cost = user2credits((*r).avg);
    }

    0
}

unsafe fn limit_mt_destroy(par: *const xt_mtdtor_param) {
    let info: *const xt_rateinfo = (*par).matchinfo as *const xt_rateinfo;
    kfree((*info).master);
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[repr(C)]
struct compat_xt_rateinfo {
    avg: u32,
    burst: u32,
    prev: compat_ulong_t,
    credit: u32,
    credit_cap: u32,
    cost: u32,
    master: u32,
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
unsafe fn limit_mt_compat_from_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    let cm = src as *const compat_xt_rateinfo;
    let m = xt_rateinfo {
        avg: (*cm).avg,
        burst: (*cm).burst,
        prev: (*cm).prev | ((*cm).master as usize) << 32,
        credit: (*cm).credit,
        credit_cap: (*cm).credit_cap,
        cost: (*cm).cost,
        master: core::ptr::null_mut(),
    };
    memcpy(dst, &m, core::mem::size_of::<xt_rateinfo>());
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
unsafe fn limit_mt_compat_to_user(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) -> i32 {
    let m = src as *const xt_rateinfo;
    let cm = compat_xt_rateinfo {
        avg: (*m).avg,
        burst: (*m).burst,
        prev: (*m).prev as compat_ulong_t,
        credit: (*m).credit,
        credit_cap: (*m).credit_cap,
        cost: (*m).cost,
        master: ((*m).prev >> 32) as u32,
    };
    if copy_to_user(dst, &cm, core::mem::size_of::<compat_xt_rateinfo>()) != 0 {
        -EFAULT
    } else {
        0
    }
}

static mut limit_mt_reg: xt_match = xt_match {
    name: "limit",
    revision: 0,
    family: NFPROTO_UNSPEC,
    match_: Some(limit_mt),
    checkentry: Some(limit_mt_check),
    destroy: Some(limit_mt_destroy),
    matchsize: core::mem::size_of::<xt_rateinfo>(),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compatsize: core::mem::size_of::<compat_xt_rateinfo>(),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compat_from_user: Some(limit_mt_compat_from_user),
    #[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
    compat_to_user: Some(limit_mt_compat_to_user),
    usersize: core::mem::offset_of!(xt_rateinfo, prev),
    me: THIS_MODULE,
};

unsafe fn limit_mt_init() -> i32 {
    xt_register_match(&mut limit_mt_reg)
}

unsafe fn limit_mt_exit() {
    xt_unregister_match(&mut limit_mt_reg);
}

// module_init(limit_mt_init);
// module_exit(limit_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
