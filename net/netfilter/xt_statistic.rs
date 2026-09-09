// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2006 Patrick McHardy <kaber@trash.net>
 *
 * Based on ipt_random and ipt_nth by Fabrice MARIE <fabrice@netfilter.org>.
 */

// The Linux kernel headers and module metadata used by this implementation are
// supplied by the surrounding Rust translation environment.

#[repr(C)]
pub struct xt_statistic_priv {
    pub count: atomic_t,
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_DESCRIPTION("Xtables: statistics-based matching (\"Nth\", random)");
// MODULE_ALIAS("ipt_statistic");
// MODULE_ALIAS("ip6t_statistic");

static unsafe fn statistic_mt(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let info: *const xt_statistic_info = unsafe { (*par).matchinfo as *const xt_statistic_info };
    let mut ret: bool = unsafe { ((*info).flags & XT_STATISTIC_INVERT) != 0 };
    let nval: i32;
    let oval: i32;

    unsafe {
        match (*info).mode {
            XT_STATISTIC_MODE_RANDOM => {
                if (get_random_u32() & 0x7fffffff) < (*info).u.random.probability {
                    ret = !ret;
                }
            }
            XT_STATISTIC_MODE_NTH => {
                loop {
                    let old = atomic_read(&(*(*info).master).count);
                    let new = if old == (*info).u.nth.every { 0 } else { old + 1 };
                    if atomic_cmpxchg(&(*(*info).master).count, old, new) == old {
                        nval = new;
                        oval = old;
                        break;
                    }
                }
                let _ = oval;
                if nval == 0 {
                    ret = !ret;
                }
            }
            _ => {}
        }
    }

    let _ = skb;
    ret
}

static unsafe fn statistic_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info: *mut xt_statistic_info = unsafe { (*par).matchinfo as *mut xt_statistic_info };

    unsafe {
        if (*info).mode > XT_STATISTIC_MODE_MAX
            || ((*info).flags & !XT_STATISTIC_MASK) != 0
        {
            return -EINVAL;
        }

        (*info).master = kzalloc_obj::<xt_statistic_priv>();
        if (*info).master.is_null() {
            return -ENOMEM;
        }
        atomic_set(&(*(*info).master).count, (*info).u.nth.count);
    }

    0
}

static unsafe fn statistic_mt_destroy(par: *const xt_mtdtor_param) {
    let info: *const xt_statistic_info = unsafe { (*par).matchinfo as *const xt_statistic_info };
    unsafe {
        kfree((*info).master as *mut core::ffi::c_void);
    }
}

static mut xt_statistic_mt_reg: xt_match = xt_match {
    name: b"statistic\0".as_ptr() as *const i8,
    revision: 0,
    family: NFPROTO_UNSPEC,
    match_: Some(statistic_mt),
    checkentry: Some(statistic_mt_check),
    destroy: Some(statistic_mt_destroy),
    matchsize: core::mem::size_of::<xt_statistic_info>(),
    usersize: core::mem::offset_of!(xt_statistic_info, master),
    me: THIS_MODULE,
};

unsafe fn statistic_mt_init() -> i32 {
    unsafe { xt_register_match(&raw mut xt_statistic_mt_reg) }
}

unsafe fn statistic_mt_exit() {
    unsafe { xt_unregister_match(&raw mut xt_statistic_mt_reg) };
}

// module_init(statistic_mt_init);
// module_exit(statistic_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
