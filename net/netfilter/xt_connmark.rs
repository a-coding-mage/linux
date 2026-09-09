// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	xt_connmark - Netfilter module to operate on connection marks
 *
 *	Copyright (C) 2002,2004 MARA Systems AB <https://www.marasystems.com>
 *	by Henrik Nordstrom <hno@marasystems.com>
 *	Copyright © CC Computer Consultants GmbH, 2007 - 2008
 *	Jan Engelhardt <jengelh@medozas.de>
 */

// Dependencies supplied by the surrounding kernel translation.

static unsigned int connmark_tg_shift(
    skb: *mut sk_buff,
    info: *const xt_connmark_tginfo2,
) -> c_uint {
    let mut ctinfo: ip_conntrack_info = unsafe { core::mem::zeroed() };
    let mut new_targetmark: u32;
    let ct: *mut nf_conn;
    let mut newmark: u32;
    let mut oldmark: u32;

    ct = unsafe { nf_ct_get(skb, &mut ctinfo) };
    if ct.is_null() {
        return XT_CONTINUE;
    }

    unsafe {
        match (*info).mode {
            XT_CONNMARK_SET => {
                oldmark = core::ptr::read_volatile(&(*ct).mark);
                newmark = (oldmark & !(*info).ctmask) ^ (*info).ctmark;
                if (*info).shift_dir == D_SHIFT_RIGHT {
                    newmark >>= (*info).shift_bits;
                } else {
                    newmark <<= (*info).shift_bits;
                }

                if core::ptr::read_volatile(&(*ct).mark) != newmark {
                    core::ptr::write_volatile(&mut (*ct).mark, newmark);
                    nf_conntrack_event_cache(IPCT_MARK, ct);
                }
            }
            XT_CONNMARK_SAVE => {
                new_targetmark = (*skb).mark & (*info).nfmask;
                if (*info).shift_dir == D_SHIFT_RIGHT {
                    new_targetmark >>= (*info).shift_bits;
                } else {
                    new_targetmark <<= (*info).shift_bits;
                }

                newmark = (core::ptr::read_volatile(&(*ct).mark) & !(*info).ctmask)
                    ^ new_targetmark;
                if core::ptr::read_volatile(&(*ct).mark) != newmark {
                    core::ptr::write_volatile(&mut (*ct).mark, newmark);
                    nf_conntrack_event_cache(IPCT_MARK, ct);
                }
            }
            XT_CONNMARK_RESTORE => {
                new_targetmark = core::ptr::read_volatile(&(*ct).mark) & (*info).ctmask;
                if (*info).shift_dir == D_SHIFT_RIGHT {
                    new_targetmark >>= (*info).shift_bits;
                } else {
                    new_targetmark <<= (*info).shift_bits;
                }

                newmark = ((*skb).mark & !(*info).nfmask) ^ new_targetmark;
                (*skb).mark = newmark;
            }
            _ => {}
        }
    }
    XT_CONTINUE
}

static unsigned int connmark_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info: *const xt_connmark_tginfo1 = unsafe { (*par).targinfo as *const xt_connmark_tginfo1 };
    let info2 = xt_connmark_tginfo2 {
        ctmark: unsafe { (*info).ctmark },
        ctmask: unsafe { (*info).ctmask },
        nfmask: unsafe { (*info).nfmask },
        mode: unsafe { (*info).mode },
        ..unsafe { core::mem::zeroed() }
    };

    connmark_tg_shift(skb, &info2)
}

static unsigned int connmark_tg_v2(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info: *const xt_connmark_tginfo2 = unsafe { (*par).targinfo as *const xt_connmark_tginfo2 };
    connmark_tg_shift(skb, info)
}

static fn connmark_tg_check(par: *const xt_tgchk_param) -> c_int {
    let ret = unsafe { nf_ct_netns_get((*par).net, (*par).family) };
    if ret < 0 {
        unsafe { pr_info_ratelimited!("cannot load conntrack support for proto=%u\n", (*par).family); }
    }
    ret
}

static fn connmark_tg_check_v2(par: *const xt_tgchk_param) -> c_int {
    let info: *const xt_connmark_tginfo2 = unsafe { (*par).targinfo as *const xt_connmark_tginfo2 };
    unsafe {
        if (*info).shift_dir > D_SHIFT_RIGHT || (*info).shift_bits >= 32 {
            return -EINVAL;
        }
    }
    connmark_tg_check(par)
}

static fn connmark_tg_destroy(par: *const xt_tgdtor_param) {
    unsafe { nf_ct_netns_put((*par).net, (*par).family); }
}

static fn connmark_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info: *const xt_connmark_mtinfo1 = unsafe { (*par).matchinfo as *const xt_connmark_mtinfo1 };
    let mut ctinfo: ip_conntrack_info = unsafe { core::mem::zeroed() };
    let ct: *const nf_conn;

    ct = unsafe { nf_ct_get(skb as *mut sk_buff, &mut ctinfo) };
    if ct.is_null() {
        return false;
    }

    unsafe {
        ((core::ptr::read_volatile(&(*ct).mark) & (*info).mask) == (*info).mark) ^ (*info).invert
    }
}

static fn connmark_mt_check(par: *const xt_mtchk_param) -> c_int {
    let ret = unsafe { nf_ct_netns_get((*par).net, (*par).family) };
    if ret < 0 {
        unsafe { pr_info_ratelimited!("cannot load conntrack support for proto=%u\n", (*par).family); }
    }
    ret
}

static fn connmark_mt_destroy(par: *const xt_mtdtor_param) {
    unsafe { nf_ct_netns_put((*par).net, (*par).family); }
}

// The registration objects and module entry points retain the C module's external interface.
static mut connmark_tg_reg: [xt_target; 4] = [unsafe { core::mem::zeroed() }; 4];
static mut connmark_mt_reg: xt_match = unsafe { core::mem::zeroed() };

static fn connmark_mt_init() -> c_int {
    let mut ret = unsafe { xt_register_targets(connmark_tg_reg.as_mut_ptr(), connmark_tg_reg.len()) };
    if ret < 0 { return ret; }
    ret = unsafe { xt_register_match(&mut connmark_mt_reg) };
    if ret < 0 {
        unsafe { xt_unregister_targets(connmark_tg_reg.as_mut_ptr(), connmark_tg_reg.len()); }
        return ret;
    }
    0
}

static fn connmark_mt_exit() {
    unsafe {
        xt_unregister_match(&mut connmark_mt_reg);
        xt_unregister_targets(connmark_tg_reg.as_mut_ptr(), connmark_tg_reg.len());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
