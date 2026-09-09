// SPDX-License-Identifier: GPL-2.0
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * smc_sysctl.c: sysctl interface to SMC subsystem.
 *
 * Copyright (c) 2022, Alibaba Inc.
 * Author: Tony Lu <tonylu@linux.alibaba.com>
 */

// C headers and SMC subsystem headers are supplied by the surrounding kernel
// translation unit.

static mut min_sndbuf: i32 = SMC_BUF_MIN_SIZE;
static mut min_rcvbuf: i32 = SMC_BUF_MIN_SIZE;
static mut max_sndbuf: i32 = INT_MAX / 2;
static mut max_rcvbuf: i32 = INT_MAX / 2;
static net_smc_wmem_init: i32 = 64 * 1024;
static net_smc_rmem_init: i32 = 64 * 1024;
static mut links_per_lgr_min: i32 = SMC_LINKS_ADD_LNK_MIN;
static mut links_per_lgr_max: i32 = SMC_LINKS_ADD_LNK_MAX;
static mut conns_per_lgr_min: i32 = SMC_CONN_PER_LGR_MIN;
static mut conns_per_lgr_max: i32 = SMC_CONN_PER_LGR_MAX;
static mut smcr_max_wr_min: u32 = 2;
static mut smcr_max_wr_max: u32 = 2048;

// CONFIG_SMC_HS_CTRL_BPF conditional section preserved from the C source.
#[cfg(CONFIG_SMC_HS_CTRL_BPF)]
unsafe fn smc_net_replace_smc_hs_ctrl(net: *mut net, name: *const c_char) -> i32 {
    let mut ctrl: *mut smc_hs_ctrl = core::ptr::null_mut();

    rcu_read_lock();
    // null or empty name ask to clear current ctrl
    if !name.is_null() && *name != 0 {
        ctrl = smc_hs_ctrl_find_by_name(name);
        if ctrl.is_null() {
            rcu_read_unlock();
            return -EINVAL;
        }
        // no change, just return
        if ctrl == rcu_dereference((*net).smc.hs_ctrl) {
            rcu_read_unlock();
            return 0;
        }
        if !bpf_try_module_get(ctrl, (*ctrl).owner) {
            rcu_read_unlock();
            return -EBUSY;
        }
    }
    // xchg old ctrl with the new one atomically
    ctrl = unrcu_pointer(xchg(&mut (*net).smc.hs_ctrl, rcu_initializer(ctrl)));
    // release old ctrl
    if !ctrl.is_null() {
        bpf_module_put(ctrl, (*ctrl).owner);
    }

    rcu_read_unlock();
    0
}

#[cfg(CONFIG_SMC_HS_CTRL_BPF)]
unsafe fn proc_smc_hs_ctrl(ctl: *const ctl_table, write: i32, buffer: *mut c_void,
                           lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    let net = container_of((*ctl).data, net, smc.hs_ctrl);
    let mut val = [0i8; SMC_HS_CTRL_NAME_MAX as usize];
    let tbl = ctl_table { data: val.as_mut_ptr() as *mut c_void,
        maxlen: SMC_HS_CTRL_NAME_MAX, ..core::mem::zeroed() };
    let ctrl: *mut smc_hs_ctrl;

    rcu_read_lock();
    ctrl = rcu_dereference((*net).smc.hs_ctrl);
    if !ctrl.is_null() {
        memcpy(val.as_mut_ptr() as *mut c_void, (*ctrl).name.as_ptr() as *const c_void,
               core::mem::size_of_val(&(*ctrl).name));
    } else { val[0] = 0; }
    rcu_read_unlock();

    let mut ret = proc_dostring(&tbl, write, buffer, lenp, ppos);
    if ret != 0 { return ret; }
    if write != 0 { ret = smc_net_replace_smc_hs_ctrl(net, val.as_ptr()); }
    ret
}

// The ctl_table array is represented using the kernel-provided Rust binding.
static smc_table: [ctl_table; 10] = [
    ctl_table { procname: b"autocorking_size\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_autocorking_size), maxlen: core::mem::size_of::<u32>(), mode: 0o644, proc_handler: Some(proc_douintvec), ..ctl_table::ZERO },
    ctl_table { procname: b"smcr_buf_type\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_smcr_buf_type), maxlen: core::mem::size_of::<u32>(), mode: 0o644, proc_handler: Some(proc_douintvec_minmax), extra1: SYSCTL_ZERO, extra2: SYSCTL_TWO, ..ctl_table::ZERO },
    ctl_table { procname: b"smcr_testlink_time\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_smcr_testlink_time), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_jiffies), ..ctl_table::ZERO },
    ctl_table { procname: b"wmem\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_wmem), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: addr_of!(min_sndbuf), extra2: addr_of!(max_sndbuf), ..ctl_table::ZERO },
    ctl_table { procname: b"rmem\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_rmem), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: addr_of!(min_rcvbuf), extra2: addr_of!(max_rcvbuf), ..ctl_table::ZERO },
    ctl_table { procname: b"smcr_max_links_per_lgr\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_max_links_per_lgr), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: addr_of!(links_per_lgr_min), extra2: addr_of!(links_per_lgr_max), ..ctl_table::ZERO },
    ctl_table { procname: b"smcr_max_conns_per_lgr\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_max_conns_per_lgr), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: addr_of!(conns_per_lgr_min), extra2: addr_of!(conns_per_lgr_max), ..ctl_table::ZERO },
    ctl_table { procname: b"limit_smc_hs\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.limit_smc_hs), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE, ..ctl_table::ZERO },
    ctl_table { procname: b"smcr_max_send_wr\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_smcr_max_send_wr), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: addr_of!(smcr_max_wr_min), extra2: addr_of!(smcr_max_wr_max), ..ctl_table::ZERO },
    ctl_table { procname: b"smcr_max_recv_wr\0".as_ptr() as *const c_char, data: addr_of!(init_net.smc.sysctl_smcr_max_recv_wr), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: addr_of!(smcr_max_wr_min), extra2: addr_of!(smcr_max_wr_max), ..ctl_table::ZERO },
];

unsafe fn smc_table_dup(net: *mut net) -> *const ctl_table {
    let table_size = smc_table.len();
    let table = kmemdup(smc_table.as_ptr() as *const c_void, core::mem::size_of_val(&smc_table), GFP_KERNEL) as *mut ctl_table;
    if table.is_null() { return core::ptr::null(); }
    for i in 0..table_size { (*table.add(i)).data = ((*table.add(i)).data as usize + net as usize - &init_net as *const _ as usize) as *mut c_void; }
    table
}

pub unsafe fn smc_sysctl_net_init(net: *mut net) -> i32 {
    let table_size = smc_table.len();
    let mut table = smc_table.as_ptr();
    if !net_eq(net, &init_net) { table = smc_table_dup(net); if table.is_null() { return -ENOMEM; } }
    (*net).smc.smc_hdr = register_net_sysctl_sz(net, b"net/smc\0".as_ptr() as *const c_char, table, table_size);
    if (*net).smc.smc_hdr.is_null() { if !net_eq(net, &init_net) { kfree(table as *mut c_void); } return -ENOMEM; }
    (*net).smc.sysctl_autocorking_size = SMC_AUTOCORKING_DEFAULT_SIZE;
    (*net).smc.sysctl_smcr_buf_type = SMCR_PHYS_CONT_BUFS;
    (*net).smc.sysctl_smcr_testlink_time = SMC_LLC_TESTLINK_DEFAULT_TIME;
    write_once(&mut (*net).smc.sysctl_wmem, net_smc_wmem_init);
    write_once(&mut (*net).smc.sysctl_rmem, net_smc_rmem_init);
    (*net).smc.sysctl_max_links_per_lgr = SMC_LINKS_PER_LGR_MAX_PREFER;
    (*net).smc.sysctl_max_conns_per_lgr = SMC_CONN_PER_LGR_PREFER;
    (*net).smc.sysctl_smcr_max_send_wr = SMCR_MAX_SEND_WR_DEF;
    (*net).smc.sysctl_smcr_max_recv_wr = SMCR_MAX_RECV_WR_DEF;
    // disable handshake limitation by default
    (*net).smc.limit_smc_hs = 0;
    0
}

pub unsafe fn smc_sysctl_net_exit(net: *mut net) {
    let table = (*net).smc.smc_hdr.ctl_table_arg;
    unregister_net_sysctl_table((*net).smc.smc_hdr);
    if !net_eq(net, &init_net) { kfree(table as *mut c_void); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
