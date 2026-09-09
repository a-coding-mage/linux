// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Generic netlink for energy model.
 *
 * Copyright (c) 2025 Valve Corporation.
 * Author: Changwoo Min <changwoo@igalia.com>
 */

// C dependencies supplied by the kernel and generated headers are referenced
// below as external Rust symbols.

#[repr(C)]
struct dump_ctx {
    idx: i32,
    start: i32,
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
}

unsafe fn __em_nl_get_pd_size(pd: *mut em_perf_domain, data: *mut core::ffi::c_void) -> i32 {
    let nr_cpus: i32;
    let msg_sz: i32;
    let cpus_sz: i32;
    let tot_msg_sz = data as *mut i32;

    nr_cpus = cpumask_weight(to_cpumask((*pd).cpus));
    cpus_sz = nla_total_size_64bit(core::mem::size_of::<u64>() as i32) * nr_cpus;

    msg_sz = nla_total_size(0)
        /* DEV_ENERGYMODEL_A_PERF_DOMAINS_PERF_DOMAIN */
        + nla_total_size(core::mem::size_of::<u32>() as i32)
        /* DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID */
        + nla_total_size_64bit(core::mem::size_of::<u64>() as i32)
        /* DEV_ENERGYMODEL_A_PERF_DOMAIN_FLAGS */
        + nla_total_size(cpus_sz);
        /* DEV_ENERGYMODEL_A_PERF_DOMAIN_CPUS */

    *tot_msg_sz += nlmsg_total_size(genlmsg_msg_size(msg_sz));
    0
}

unsafe fn __em_nl_get_pd(pd: *mut em_perf_domain, data: *mut core::ffi::c_void) -> i32 {
    let msg = data as *mut sk_buff;
    let cpumask: *mut cpumask;
    let mut cpu: i32;

    if nla_put_u32(msg, DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID, (*pd).id) != 0 {
        return -EMSGSIZE;
    }
    if nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_DOMAIN_FLAGS, (*pd).flags,
                         DEV_ENERGYMODEL_A_PERF_DOMAIN_PAD) != 0 {
        return -EMSGSIZE;
    }

    cpumask = to_cpumask((*pd).cpus);
    for_each_cpu!(cpu, cpumask) {
        if nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_DOMAIN_CPUS, cpu as u64,
                             DEV_ENERGYMODEL_A_PERF_DOMAIN_PAD) != 0 {
            return -EMSGSIZE;
        }
    }
    0
}

unsafe fn __em_nl_get_pd_for_dump(pd: *mut em_perf_domain, data: *mut core::ffi::c_void) -> i32 {
    let ctx = data as *mut dump_ctx;
    let info: *const genl_info;
    let hdr: *mut core::ffi::c_void;
    let ret: i32;

    (*ctx).idx += 1;
    if (*ctx).idx - 1 < (*ctx).start {
        return 0;
    }
    info = genl_info_dump((*ctx).cb);
    hdr = genlmsg_iput((*ctx).skb, info);
    if hdr.is_null() {
        genlmsg_cancel((*ctx).skb, hdr);
        return -EMSGSIZE;
    }
    ret = __em_nl_get_pd(pd, (*ctx).skb as *mut core::ffi::c_void);
    genlmsg_end((*ctx).skb, hdr);
    ret
}

unsafe fn dev_energymodel_nl_get_perf_domains_doit(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut id: i32;
    let mut ret: i32 = -EMSGSIZE;
    let msg_sz: i32 = 0;
    let cmd = (*(*info).genlhdr).cmd;
    let pd: *mut em_perf_domain;
    let msg: *mut sk_buff;
    let hdr: *mut core::ffi::c_void;

    if (*info).attrs[DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID].is_null() { return -EINVAL; }
    id = nla_get_u32((*info).attrs[DEV_ENERGYMODEL_A_PERF_DOMAIN_PERF_DOMAIN_ID]);
    pd = em_perf_domain_get_by_id(id);
    if pd.is_null() { return -EINVAL; }
    __em_nl_get_pd_size(pd, &msg_sz as *const i32 as *mut core::ffi::c_void);
    msg = genlmsg_new(msg_sz, GFP_KERNEL);
    if msg.is_null() { return -ENOMEM; }
    hdr = genlmsg_put_reply(msg, info, &dev_energymodel_nl_family, 0, cmd);
    if hdr.is_null() { nlmsg_free(msg); return ret; }
    ret = __em_nl_get_pd(pd, msg as *mut core::ffi::c_void);
    if ret != 0 { genlmsg_cancel(msg, hdr); nlmsg_free(msg); return ret; }
    genlmsg_end(msg, hdr);
    genlmsg_reply(msg, info)
}

unsafe fn dev_energymodel_nl_get_perf_domains_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let mut ctx = dump_ctx { idx: 0, start: (*cb).args[0], skb, cb };
    for_each_em_perf_domain(__em_nl_get_pd_for_dump, &mut ctx as *mut _ as *mut core::ffi::c_void)
}

unsafe fn __em_nl_get_pd_table_id(attrs: *mut *mut nlattr) -> *mut em_perf_domain {
    if (*attrs.add(DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID)).is_null() { return core::ptr::null_mut(); }
    em_perf_domain_get_by_id(nla_get_u32(*attrs.add(DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID)))
}

unsafe fn __em_nl_get_pd_table_size(pd: *const em_perf_domain) -> i32 {
    let id_sz = nla_total_size(core::mem::size_of::<u32>() as i32);
    let mut ps_sz = nla_total_size(0)
        + nla_total_size_64bit(8) + nla_total_size_64bit(8)
        + nla_total_size_64bit(8) + nla_total_size_64bit(8)
        + nla_total_size_64bit(8);
    ps_sz *= (*pd).nr_perf_states;
    nlmsg_total_size(genlmsg_msg_size(id_sz + ps_sz))
}

unsafe fn __em_nl_get_pd_table(msg: *mut sk_buff, pd: *const em_perf_domain) -> i32 {
    if nla_put_u32(msg, DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID, (*pd).id) != 0 { return -EMSGSIZE; }
    rcu_read_lock();
    let table = em_perf_state_from_pd(pd as *mut em_perf_domain);
    for i in 0..(*pd).nr_perf_states {
        let ps = &*table.add(i as usize);
        let entry = nla_nest_start(msg, DEV_ENERGYMODEL_A_PERF_TABLE_PERF_STATE);
        if entry.is_null() { rcu_read_unlock(); return -EMSGSIZE; }
        if nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_STATE_PERFORMANCE, ps.performance, DEV_ENERGYMODEL_A_PERF_STATE_PAD) != 0
            || nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_STATE_FREQUENCY, ps.frequency, DEV_ENERGYMODEL_A_PERF_STATE_PAD) != 0
            || nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_STATE_POWER, ps.power, DEV_ENERGYMODEL_A_PERF_STATE_PAD) != 0
            || nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_STATE_COST, ps.cost, DEV_ENERGYMODEL_A_PERF_STATE_PAD) != 0
            || nla_put_u64_64bit(msg, DEV_ENERGYMODEL_A_PERF_STATE_FLAGS, ps.flags, DEV_ENERGYMODEL_A_PERF_STATE_PAD) != 0 {
            nla_nest_cancel(msg, entry); rcu_read_unlock(); return -EMSGSIZE;
        }
        nla_nest_end(msg, entry);
    }
    rcu_read_unlock();
    0
}

unsafe fn dev_energymodel_nl_get_perf_table_doit(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let pd = __em_nl_get_pd_table_id((*info).attrs);
    if pd.is_null() { return -EINVAL; }
    let msg = genlmsg_new(__em_nl_get_pd_table_size(pd), GFP_KERNEL);
    if msg.is_null() { return -ENOMEM; }
    let hdr = genlmsg_put_reply(msg, info, &dev_energymodel_nl_family, 0, (*(*info).genlhdr).cmd);
    if hdr.is_null() { nlmsg_free(msg); return -EMSGSIZE; }
    let ret = __em_nl_get_pd_table(msg, pd);
    if ret != 0 { nlmsg_free(msg); return ret; }
    genlmsg_end(msg, hdr); genlmsg_reply(msg, info)
}

unsafe fn __em_notify_pd_table(pd: *const em_perf_domain, ntf_type: i32) {
    if genl_has_listeners(&dev_energymodel_nl_family, &init_net, DEV_ENERGYMODEL_NLGRP_EVENT) == 0 { return; }
    let msg = genlmsg_new(__em_nl_get_pd_table_size(pd), GFP_KERNEL);
    if msg.is_null() { return; }
    let hdr = genlmsg_put(msg, 0, 0, &dev_energymodel_nl_family, 0, ntf_type);
    if hdr.is_null() { nlmsg_free(msg); return; }
    if __em_nl_get_pd_table(msg, pd) != 0 { nlmsg_free(msg); return; }
    genlmsg_end(msg, hdr);
    genlmsg_multicast(&dev_energymodel_nl_family, msg, 0, DEV_ENERGYMODEL_NLGRP_EVENT, GFP_KERNEL);
}

unsafe fn em_notify_pd_created(pd: *const em_perf_domain) { __em_notify_pd_table(pd, DEV_ENERGYMODEL_CMD_PERF_DOMAIN_CREATED); }
unsafe fn em_notify_pd_updated(pd: *const em_perf_domain) { __em_notify_pd_table(pd, DEV_ENERGYMODEL_CMD_PERF_DOMAIN_UPDATED); }

unsafe fn __em_notify_pd_deleted_size(_pd: *const em_perf_domain) -> i32 {
    nlmsg_total_size(genlmsg_msg_size(nla_total_size(core::mem::size_of::<u32>() as i32)))
}

unsafe fn em_notify_pd_deleted(pd: *const em_perf_domain) {
    if genl_has_listeners(&dev_energymodel_nl_family, &init_net, DEV_ENERGYMODEL_NLGRP_EVENT) == 0 { return; }
    let msg = genlmsg_new(__em_notify_pd_deleted_size(pd), GFP_KERNEL);
    if msg.is_null() { return; }
    let hdr = genlmsg_put(msg, 0, 0, &dev_energymodel_nl_family, 0, DEV_ENERGYMODEL_CMD_PERF_DOMAIN_DELETED);
    if hdr.is_null() || nla_put_u32(msg, DEV_ENERGYMODEL_A_PERF_TABLE_PERF_DOMAIN_ID, (*pd).id) != 0 { nlmsg_free(msg); return; }
    genlmsg_end(msg, hdr);
    genlmsg_multicast(&dev_energymodel_nl_family, msg, 0, DEV_ENERGYMODEL_NLGRP_EVENT, GFP_KERNEL);
}

unsafe fn em_netlink_init() -> i32 { genl_register_family(&dev_energymodel_nl_family) }

// C: postcore_initcall(em_netlink_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
