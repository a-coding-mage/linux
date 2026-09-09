// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
 *                         Patrick Schaaf <bof@bof.de>
 *                         Martin Josefsson <gandalf@wlug.westbo.se>
 * Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org>
 */

/* Kernel module which implements the set match and SET target
 * for netfilter/iptables.
 */

// External kernel, netfilter, and ipset declarations are supplied by dependencies.

unsafe fn match_set(index: ip_set_id_t, skb: *const sk_buff,
                    par: *const xt_action_param,
                    opt: *mut ip_set_adt_opt, mut inv: i32) -> i32 {
    if ip_set_test(index, skb, par, opt) != 0 { inv = !inv; }
    inv
}

macro_rules! ADT_OPT {
    ($n:ident, $f:expr, $d:expr, $fs:expr, $cfs:expr, $t:expr, $p:expr, $b:expr, $po:expr, $bo:expr) => {
        let mut $n: ip_set_adt_opt = ip_set_adt_opt {
            family: $f, dim: $d, flags: $fs, cmdflags: $cfs,
            ext: ip_set_adt_opt_ext { timeout: $t, packets: $p, bytes: $b,
                packets_op: $po, bytes_op: $bo, ..unsafe { core::mem::zeroed() } },
            ..unsafe { core::mem::zeroed() }
        };
    };
}

unsafe fn set_match_v0(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_set_info_match_v0;
    ADT_OPT!(opt, xt_family(par), (*info).match_set.u_.compat.dim,
             (*info).match_set.u_.compat.flags, 0, UINT_MAX, 0, 0, 0, 0);
    match_set((*info).match_set.index, skb, par, &mut opt,
              ((*info).match_set.u_.compat.flags & IPSET_INV_MATCH) as i32) != 0
}

unsafe fn compat_flags(info: *mut xt_set_info_v0) {
    let mut i: u8 = 0;
    (*info).u_.compat.dim = IPSET_DIM_ZERO;
    if (*info).u_.flags[0] & IPSET_MATCH_INV != 0 { (*info).u_.compat.flags |= IPSET_INV_MATCH; }
    while i < IPSET_DIM_MAX - 1 && (*info).u_.flags[i as usize] != 0 {
        (*info).u_.compat.dim += 1;
        if (*info).u_.flags[i as usize] & IPSET_SRC != 0 {
            (*info).u_.compat.flags |= 1 << (*info).u_.compat.dim;
        }
        i += 1;
    }
}

unsafe fn set_match_v0_checkentry(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *mut xt_set_info_match_v0;
    let index = ip_set_nfnl_get_byindex((*par).net, (*info).match_set.index);
    if index == IPSET_INVALID_ID { pr_info_ratelimited("Cannot find set identified by id %u to match\n", (*info).match_set.index); return -ENOENT; }
    if (*info).match_set.u_.flags[IPSET_DIM_MAX as usize - 1] != 0 {
        pr_info_ratelimited("set match dimension is over the limit!\n");
        ip_set_nfnl_put((*par).net, (*info).match_set.index); return -ERANGE;
    }
    compat_flags(&mut (*info).match_set); 0
}

unsafe fn set_match_v0_destroy(par: *const xt_mtdtor_param) {
    let info = (*par).matchinfo as *const xt_set_info_match_v0;
    ip_set_nfnl_put((*par).net, (*info).match_set.index);
}

unsafe fn set_match_v1(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_set_info_match_v1;
    ADT_OPT!(opt, xt_family(par), (*info).match_set.dim, (*info).match_set.flags, 0, UINT_MAX, 0, 0, 0, 0);
    if opt.flags & IPSET_RETURN_NOMATCH != 0 { opt.cmdflags |= IPSET_FLAG_RETURN_NOMATCH; }
    match_set((*info).match_set.index, skb, par, &mut opt, ((*info).match_set.flags & IPSET_INV_MATCH) as i32) != 0
}

unsafe fn set_match_v1_checkentry(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_set_info_match_v1;
    let index = ip_set_nfnl_get_byindex((*par).net, (*info).match_set.index);
    if index == IPSET_INVALID_ID { pr_info_ratelimited("Cannot find set identified by id %u to match\n", (*info).match_set.index); return -ENOENT; }
    if (*info).match_set.dim > IPSET_DIM_MAX { pr_info_ratelimited("set match dimension is over the limit!\n"); ip_set_nfnl_put((*par).net, (*info).match_set.index); return -ERANGE; } 0
}
unsafe fn set_match_v1_destroy(par: *const xt_mtdtor_param) { let info = (*par).matchinfo as *const xt_set_info_match_v1; ip_set_nfnl_put((*par).net, (*info).match_set.index); }

unsafe fn set_match_v3(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const xt_set_info_match_v3;
    ADT_OPT!(opt, xt_family(par), (*info).match_set.dim, (*info).match_set.flags, (*info).flags, UINT_MAX, (*info).packets.value, (*info).bytes.value, (*info).packets.op, (*info).bytes.op);
    if (*info).packets.op != IPSET_COUNTER_NONE || (*info).bytes.op != IPSET_COUNTER_NONE { opt.cmdflags |= IPSET_FLAG_MATCH_COUNTERS; }
    match_set((*info).match_set.index, skb, par, &mut opt, ((*info).match_set.flags & IPSET_INV_MATCH) as i32) != 0
}
unsafe fn set_match_v4(skb: *const sk_buff, par: *mut xt_action_param) -> bool { set_match_v3(skb, par) }

unsafe fn set_target_v0(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let info = (*par).targinfo as *const xt_set_info_target_v0;
    ADT_OPT!(add_opt, xt_family(par), (*info).add_set.u_.compat.dim, (*info).add_set.u_.compat.flags, 0, UINT_MAX, 0, 0, 0, 0);
    ADT_OPT!(del_opt, xt_family(par), (*info).del_set.u_.compat.dim, (*info).del_set.u_.compat.flags, 0, UINT_MAX, 0, 0, 0, 0);
    if (*info).add_set.index != IPSET_INVALID_ID { ip_set_add((*info).add_set.index, skb, par, &mut add_opt); }
    if (*info).del_set.index != IPSET_INVALID_ID { ip_set_del((*info).del_set.index, skb, par, &mut del_opt); }
    XT_CONTINUE
}

unsafe fn set_target_v1(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let info = (*par).targinfo as *const xt_set_info_target_v1;
    ADT_OPT!(add_opt, xt_family(par), (*info).add_set.dim, (*info).add_set.flags, 0, UINT_MAX, 0, 0, 0, 0);
    ADT_OPT!(del_opt, xt_family(par), (*info).del_set.dim, (*info).del_set.flags, 0, UINT_MAX, 0, 0, 0, 0);
    if (*info).add_set.index != IPSET_INVALID_ID { ip_set_add((*info).add_set.index, skb, par, &mut add_opt); }
    if (*info).del_set.index != IPSET_INVALID_ID { ip_set_del((*info).del_set.index, skb, par, &mut del_opt); } XT_CONTINUE
}

unsafe fn set_target_v2(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let info = (*par).targinfo as *const xt_set_info_target_v2;
    ADT_OPT!(add_opt, xt_family(par), (*info).add_set.dim, (*info).add_set.flags, (*info).flags, (*info).timeout, 0, 0, 0, 0);
    ADT_OPT!(del_opt, xt_family(par), (*info).del_set.dim, (*info).del_set.flags, 0, UINT_MAX, 0, 0, 0, 0);
    if add_opt.ext.timeout != IPSET_NO_TIMEOUT && add_opt.ext.timeout > IPSET_MAX_TIMEOUT { add_opt.ext.timeout = IPSET_MAX_TIMEOUT; }
    if (*info).add_set.index != IPSET_INVALID_ID { ip_set_add((*info).add_set.index, skb, par, &mut add_opt); }
    if (*info).del_set.index != IPSET_INVALID_ID { ip_set_del((*info).del_set.index, skb, par, &mut del_opt); } XT_CONTINUE
}

unsafe fn set_target_v3(skb: *mut sk_buff, par: *const xt_action_param) -> u32 {
    let info = (*par).targinfo as *const xt_set_info_target_v3;
    ADT_OPT!(add_opt, xt_family(par), (*info).add_set.dim, (*info).add_set.flags, (*info).flags, (*info).timeout, 0, 0, 0, 0);
    ADT_OPT!(del_opt, xt_family(par), (*info).del_set.dim, (*info).del_set.flags, 0, UINT_MAX, 0, 0, 0, 0);
    ADT_OPT!(map_opt, xt_family(par), (*info).map_set.dim, (*info).map_set.flags, 0, UINT_MAX, 0, 0, 0, 0);
    if add_opt.ext.timeout != IPSET_NO_TIMEOUT && add_opt.ext.timeout > IPSET_MAX_TIMEOUT { add_opt.ext.timeout = IPSET_MAX_TIMEOUT; }
    if (*info).add_set.index != IPSET_INVALID_ID { ip_set_add((*info).add_set.index, skb, par, &mut add_opt); }
    if (*info).del_set.index != IPSET_INVALID_ID { ip_set_del((*info).del_set.index, skb, par, &mut del_opt); }
    if (*info).map_set.index != IPSET_INVALID_ID {
        map_opt.cmdflags |= (*info).flags & (IPSET_FLAG_MAP_SKBMARK | IPSET_FLAG_MAP_SKBPRIO | IPSET_FLAG_MAP_SKBQUEUE);
        if match_set((*info).map_set.index, skb, par, &mut map_opt, ((*info).map_set.flags & IPSET_INV_MATCH) as i32) != 0 {
            if map_opt.cmdflags & IPSET_FLAG_MAP_SKBMARK != 0 { (*skb).mark = ((*skb).mark & !map_opt.ext.skbinfo.skbmarkmask) ^ map_opt.ext.skbinfo.skbmark; }
            if map_opt.cmdflags & IPSET_FLAG_MAP_SKBPRIO != 0 { (*skb).priority = map_opt.ext.skbinfo.skbprio; }
            if map_opt.cmdflags & IPSET_FLAG_MAP_SKBQUEUE != 0 && !(*skb).dev.is_null() && (*skb).dev.as_ref().unwrap().real_num_tx_queues > map_opt.ext.skbinfo.skbqueue { skb_set_queue_mapping(skb, map_opt.ext.skbinfo.skbqueue); }
        }
    } XT_CONTINUE
}

unsafe fn set_target_v0_checkentry(par: *const xt_tgchk_param) -> i32 { set_target_v1_checkentry(par) }
unsafe fn set_target_v1_checkentry(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *const xt_set_info_target_v1;
    let mut index: ip_set_id_t;
    if (*info).add_set.index != IPSET_INVALID_ID { index = ip_set_nfnl_get_byindex((*par).net, (*info).add_set.index); if index == IPSET_INVALID_ID { pr_info_ratelimited("Cannot find add_set index %u as target\n", (*info).add_set.index); return -ENOENT; } }
    if (*info).del_set.index != IPSET_INVALID_ID { index = ip_set_nfnl_get_byindex((*par).net, (*info).del_set.index); if index == IPSET_INVALID_ID { pr_info_ratelimited("Cannot find del_set index %u as target\n", (*info).del_set.index); if (*info).add_set.index != IPSET_INVALID_ID { ip_set_nfnl_put((*par).net, (*info).add_set.index); } return -ENOENT; } }
    if (*info).add_set.dim > IPSET_DIM_MAX || (*info).del_set.dim > IPSET_DIM_MAX { pr_info_ratelimited("SET target dimension over the limit!\n"); if (*info).add_set.index != IPSET_INVALID_ID { ip_set_nfnl_put((*par).net, (*info).add_set.index); } if (*info).del_set.index != IPSET_INVALID_ID { ip_set_nfnl_put((*par).net, (*info).del_set.index); } return -ERANGE; } 0
}
unsafe fn set_target_v0_destroy(par: *const xt_tgdtor_param) { let info = (*par).targinfo as *const xt_set_info_target_v0; if (*info).add_set.index != IPSET_INVALID_ID { ip_set_nfnl_put((*par).net, (*info).add_set.index); } if (*info).del_set.index != IPSET_INVALID_ID { ip_set_nfnl_put((*par).net, (*info).del_set.index); } }
unsafe fn set_target_v1_destroy(par: *const xt_tgdtor_param) { set_target_v0_destroy(par) }
unsafe fn set_target_v2_checkentry(par: *const xt_tgchk_param) -> i32 { set_target_v1_checkentry(par) }
unsafe fn set_target_v2_destroy(par: *const xt_tgdtor_param) { set_target_v1_destroy(par) }
unsafe fn set_target_v3_check_hooks(par: *const xt_tgchk_param) -> i32 {
    let info = (*par).targinfo as *const xt_set_info_target_v3;
    if (*info).map_set.index != IPSET_INVALID_ID { if strncmp((*par).table, b"mangle\0".as_ptr(), 7) != 0 { pr_info_ratelimited("--map-set only usable from mangle table\n"); return -EINVAL; } if ((*info).flags & (IPSET_FLAG_MAP_SKBPRIO | IPSET_FLAG_MAP_SKBQUEUE)) != 0 && ((*par).hook_mask & !(1 << NF_INET_FORWARD | 1 << NF_INET_LOCAL_OUT | 1 << NF_INET_POST_ROUTING)) != 0 { pr_info_ratelimited("mapping of prio or/and queue is allowed only from OUTPUT/FORWARD/POSTROUTING chains\n"); return -EINVAL; } } 0
}
unsafe fn set_target_v3_checkentry(par: *const xt_tgchk_param) -> i32 { set_target_v1_checkentry(par) }
unsafe fn set_target_v3_destroy(par: *const xt_tgdtor_param) { set_target_v0_destroy(par) }

static mut set_matches: [xt_match; 0] = [];
static mut set_targets: [xt_target; 0] = [];
unsafe fn xt_set_init() -> i32 { let mut ret = xt_register_matches(set_matches.as_mut_ptr(), 0); if ret == 0 { ret = xt_register_targets(set_targets.as_mut_ptr(), 0); if ret != 0 { xt_unregister_matches(set_matches.as_mut_ptr(), 0); } } ret }
unsafe fn xt_set_fini() { xt_unregister_matches(set_matches.as_mut_ptr(), 0); xt_unregister_targets(set_targets.as_mut_ptr(), 0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
