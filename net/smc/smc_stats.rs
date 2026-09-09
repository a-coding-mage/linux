// SPDX-License-Identifier: GPL-2.0-only
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * SMC statistics netlink routines
 *
 * Copyright IBM Corp. 2021
 *
 * Author(s):  Guvenc Gulce
 */

// Linux kernel dependencies and build-time declarations are supplied externally.

pub unsafe fn smc_stats_init(net: *mut net) -> c_int {
    (*net).smc.fback_rsn = kzalloc_obj::<_>();
    if (*net).smc.fback_rsn.is_null() { return -ENOMEM; }
    (*net).smc.smc_stats = alloc_percpu::<smc_stats>();
    if (*net).smc.smc_stats.is_null() {
        kfree((*net).smc.fback_rsn);
        return -ENOMEM;
    }
    mutex_init(&mut (*net).smc.mutex_fback_rsn);
    0
}

pub unsafe fn smc_stats_exit(net: *mut net) {
    kfree((*net).smc.fback_rsn);
    if !(*net).smc.smc_stats.is_null() { free_percpu((*net).smc.smc_stats); }
}

unsafe fn smc_nl_fill_stats_rmb_data(skb: *mut sk_buff, stats: *mut smc_stats, tech: c_int, typ: c_int) -> c_int {
    let stats_rmb_cnt = if typ == SMC_NLA_STATS_T_TX_RMB_STATS { &mut (*stats).smc[tech as usize].rmb_tx } else { &mut (*stats).smc[tech as usize].rmb_rx };
    let attrs = nla_nest_start(skb, typ);
    if attrs.is_null() { return -EMSGSIZE; }
    macro_rules! put { ($a:expr, $v:expr) => { if nla_put_u64_64bit(skb, $a, $v, SMC_NLA_STATS_RMB_PAD) != 0 { nla_nest_cancel(skb, attrs); return -EMSGSIZE; } }; }
    put!(SMC_NLA_STATS_RMB_REUSE_CNT, stats_rmb_cnt.reuse_cnt);
    put!(SMC_NLA_STATS_RMB_SIZE_SM_PEER_CNT, stats_rmb_cnt.buf_size_small_peer_cnt);
    put!(SMC_NLA_STATS_RMB_SIZE_SM_CNT, stats_rmb_cnt.buf_size_small_cnt);
    put!(SMC_NLA_STATS_RMB_FULL_PEER_CNT, stats_rmb_cnt.buf_full_peer_cnt);
    put!(SMC_NLA_STATS_RMB_FULL_CNT, stats_rmb_cnt.buf_full_cnt);
    put!(SMC_NLA_STATS_RMB_ALLOC_CNT, stats_rmb_cnt.alloc_cnt);
    put!(SMC_NLA_STATS_RMB_DGRADE_CNT, stats_rmb_cnt.dgrade_cnt);
    nla_nest_end(skb, attrs); 0
}

unsafe fn smc_nl_fill_stats_bufsize_data(skb: *mut sk_buff, stats: *mut smc_stats, tech: c_int, typ: c_int) -> c_int {
    let stats_pload = if typ == SMC_NLA_STATS_T_TXPLOAD_SIZE { &mut (*stats).smc[tech as usize].tx_pd } else if typ == SMC_NLA_STATS_T_RXPLOAD_SIZE { &mut (*stats).smc[tech as usize].rx_pd } else if typ == SMC_NLA_STATS_T_TX_RMB_SIZE { &mut (*stats).smc[tech as usize].tx_rmbsize } else if typ == SMC_NLA_STATS_T_RX_RMB_SIZE { &mut (*stats).smc[tech as usize].rx_rmbsize } else { return -EMSGSIZE; };
    let attrs = nla_nest_start(skb, typ); if attrs.is_null() { return -EMSGSIZE; }
    macro_rules! put { ($a:expr, $i:expr) => { if nla_put_u64_64bit(skb, $a, stats_pload.buf[$i], SMC_NLA_STATS_PLOAD_PAD) != 0 { nla_nest_cancel(skb, attrs); return -EMSGSIZE; } }; }
    put!(SMC_NLA_STATS_PLOAD_8K, SMC_BUF_8K); put!(SMC_NLA_STATS_PLOAD_16K, SMC_BUF_16K); put!(SMC_NLA_STATS_PLOAD_32K, SMC_BUF_32K); put!(SMC_NLA_STATS_PLOAD_64K, SMC_BUF_64K); put!(SMC_NLA_STATS_PLOAD_128K, SMC_BUF_128K); put!(SMC_NLA_STATS_PLOAD_256K, SMC_BUF_256K); put!(SMC_NLA_STATS_PLOAD_512K, SMC_BUF_512K); put!(SMC_NLA_STATS_PLOAD_1024K, SMC_BUF_1024K); put!(SMC_NLA_STATS_PLOAD_G_1024K, SMC_BUF_G_1024K);
    nla_nest_end(skb, attrs); 0
}

unsafe fn smc_nl_fill_stats_tech_data(skb: *mut sk_buff, stats: *mut smc_stats, tech: c_int) -> c_int {
    let smc_tech = &mut (*stats).smc[tech as usize];
    let attrs = nla_nest_start(skb, if tech == SMC_TYPE_D { SMC_NLA_STATS_SMCD_TECH } else { SMC_NLA_STATS_SMCR_TECH });
    if attrs.is_null() { return -EMSGSIZE; }
    if smc_nl_fill_stats_rmb_data(skb, stats, tech, SMC_NLA_STATS_T_TX_RMB_STATS) != 0 || smc_nl_fill_stats_rmb_data(skb, stats, tech, SMC_NLA_STATS_T_RX_RMB_STATS) != 0 || smc_nl_fill_stats_bufsize_data(skb, stats, tech, SMC_NLA_STATS_T_TXPLOAD_SIZE) != 0 || smc_nl_fill_stats_bufsize_data(skb, stats, tech, SMC_NLA_STATS_T_RXPLOAD_SIZE) != 0 || smc_nl_fill_stats_bufsize_data(skb, stats, tech, SMC_NLA_STATS_T_TX_RMB_SIZE) != 0 || smc_nl_fill_stats_bufsize_data(skb, stats, tech, SMC_NLA_STATS_T_RX_RMB_SIZE) != 0 { nla_nest_cancel(skb, attrs); return -EMSGSIZE; }
    macro_rules! put { ($a:expr, $v:expr) => { if nla_put_u64_64bit(skb, $a, $v, SMC_NLA_STATS_PAD) != 0 { nla_nest_cancel(skb, attrs); return -EMSGSIZE; } }; }
    put!(SMC_NLA_STATS_T_CLNT_V1_SUCC, smc_tech.clnt_v1_succ_cnt); put!(SMC_NLA_STATS_T_CLNT_V2_SUCC, smc_tech.clnt_v2_succ_cnt); put!(SMC_NLA_STATS_T_SRV_V1_SUCC, smc_tech.srv_v1_succ_cnt); put!(SMC_NLA_STATS_T_SRV_V2_SUCC, smc_tech.srv_v2_succ_cnt); put!(SMC_NLA_STATS_T_RX_BYTES, smc_tech.rx_bytes); put!(SMC_NLA_STATS_T_TX_BYTES, smc_tech.tx_bytes); put!(SMC_NLA_STATS_T_RX_CNT, smc_tech.rx_cnt); put!(SMC_NLA_STATS_T_TX_CNT, smc_tech.tx_cnt); put!(SMC_NLA_STATS_T_SENDPAGE_CNT, 0); put!(SMC_NLA_STATS_T_CORK_CNT, smc_tech.cork_cnt); put!(SMC_NLA_STATS_T_NDLY_CNT, smc_tech.ndly_cnt); put!(SMC_NLA_STATS_T_SPLICE_CNT, smc_tech.splice_cnt); put!(SMC_NLA_STATS_T_URG_DATA_CNT, smc_tech.urg_data_cnt);
    if nla_put_uint(skb, SMC_NLA_STATS_T_RX_RMB_USAGE, smc_tech.rx_rmbuse) != 0 || nla_put_uint(skb, SMC_NLA_STATS_T_TX_RMB_USAGE, smc_tech.tx_rmbuse) != 0 { nla_nest_cancel(skb, attrs); return -EMSGSIZE; }
    nla_nest_end(skb, attrs); 0
}

pub unsafe fn smc_nl_get_stats(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let ctx = smc_nl_dmp_ctx(cb); if (*ctx).pos[0] != 0 { return (*skb).len; }
    let net = sock_net((*skb).sk); let nlh = genlmsg_put(skb, NETLINK_CB((*cb).skb).portid, (*cb).nlh.nlmsg_seq, &smc_gen_nl_family, NLM_F_MULTI, SMC_NETLINK_GET_STATS); if nlh.is_null() { return (*skb).len; }
    let attrs = nla_nest_start(skb, SMC_GEN_STATS); if attrs.is_null() { genlmsg_cancel(skb, nlh); return (*skb).len; }
    let stats = kzalloc_obj::<smc_stats>(); if stats.is_null() { nla_nest_cancel(skb, attrs); genlmsg_cancel(skb, nlh); return (*skb).len; }
    let size = core::mem::size_of::<smc_stats>() / core::mem::size_of::<u64>();
    for_each_possible_cpu!(cpu, { let src = per_cpu_ptr((*net).smc.smc_stats, cpu) as *mut u64; let sum = stats as *mut u64; for i in 0..size { *sum.add(i) += *src.add(i); } });
    if smc_nl_fill_stats_tech_data(skb, stats, SMC_TYPE_D) != 0 || smc_nl_fill_stats_tech_data(skb, stats, SMC_TYPE_R) != 0 || nla_put_u64_64bit(skb, SMC_NLA_STATS_CLNT_HS_ERR_CNT, (*stats).clnt_hshake_err_cnt, SMC_NLA_STATS_PAD) != 0 || nla_put_u64_64bit(skb, SMC_NLA_STATS_SRV_HS_ERR_CNT, (*stats).srv_hshake_err_cnt, SMC_NLA_STATS_PAD) != 0 { kfree(stats); nla_nest_cancel(skb, attrs); genlmsg_cancel(skb, nlh); return (*skb).len; }
    nla_nest_end(skb, attrs); genlmsg_end(skb, nlh); (*ctx).pos[0] = 1; kfree(stats); (*skb).len
}

unsafe fn smc_nl_get_fback_details(skb: *mut sk_buff, cb: *mut netlink_callback, pos: c_int, is_srv: bool) -> c_int {
    let ctx = smc_nl_dmp_ctx(cb); let net = sock_net((*skb).sk); let mut cnt = (*ctx).pos[2]; let arr = if is_srv { &(*net).smc.fback_rsn.srv[0] } else { &(*net).smc.fback_rsn.clnt[0] }; if arr[pos as usize].fback_code == 0 { return -ENODATA; }
    let nlh = genlmsg_put(skb, NETLINK_CB((*cb).skb).portid, (*cb).nlh.nlmsg_seq, &smc_gen_nl_family, NLM_F_MULTI, SMC_NETLINK_GET_FBACK_STATS); if nlh.is_null() { return -EMSGSIZE; } let attrs = nla_nest_start(skb, SMC_GEN_FBACK_STATS); if attrs.is_null() { genlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if nla_put_u8(skb, SMC_NLA_FBACK_STATS_TYPE, is_srv as u8) != 0 { nla_nest_cancel(skb, attrs); genlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if cnt == 0 { if nla_put_u64_64bit(skb, SMC_NLA_FBACK_STATS_SRV_CNT, (*net).smc.fback_rsn.srv_fback_cnt, SMC_NLA_FBACK_STATS_PAD) != 0 || nla_put_u64_64bit(skb, SMC_NLA_FBACK_STATS_CLNT_CNT, (*net).smc.fback_rsn.clnt_fback_cnt, SMC_NLA_FBACK_STATS_PAD) != 0 { nla_nest_cancel(skb, attrs); genlmsg_cancel(skb, nlh); return -EMSGSIZE; } cnt = 1; }
    if nla_put_u32(skb, SMC_NLA_FBACK_STATS_RSN_CODE, arr[pos as usize].fback_code) != 0 || nla_put_u16(skb, SMC_NLA_FBACK_STATS_RSN_CNT, arr[pos as usize].count) != 0 { nla_nest_cancel(skb, attrs); genlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    (*ctx).pos[2] = cnt; nla_nest_end(skb, attrs); genlmsg_end(skb, nlh); 0
}

pub unsafe fn smc_nl_get_fback_stats(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    let ctx = smc_nl_dmp_ctx(cb); let net = sock_net((*skb).sk); let mut skip = (*ctx).pos[1]; let mut k = (*ctx).pos[0]; mutex_lock(&mut (*net).smc.mutex_fback_rsn);
    while k < SMC_MAX_FBACK_RSN_CNT { if k < (*ctx).pos[0] { k += 1; continue; } if skip == 0 { let rc = smc_nl_get_fback_details(skb, cb, k, true); if rc != 0 && rc != -ENODATA { break; } } else { skip = 0; } let rc = smc_nl_get_fback_details(skb, cb, k, false); if rc != 0 && rc != -ENODATA { skip = 1; break; } if rc == -ENODATA { break; } k += 1; }
    mutex_unlock(&mut (*net).smc.mutex_fback_rsn); (*ctx).pos[1] = skip; (*ctx).pos[0] = k; (*skb).len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
