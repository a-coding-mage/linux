// SPDX-License-Identifier: GPL-2.0

// External kernel dependencies supplied by other translation units.

unsafe fn nf_hooks_lwtunnel_get() -> i32 {
    if static_branch_unlikely(&raw mut nf_hooks_lwtunnel_enabled) {
        1
    } else {
        0
    }
}

unsafe fn nf_hooks_lwtunnel_set(enable: i32) -> i32 {
    if static_branch_unlikely(&raw mut nf_hooks_lwtunnel_enabled) {
        if enable == 0 {
            return -EBUSY;
        }
    } else if enable != 0 {
        static_branch_enable(&raw mut nf_hooks_lwtunnel_enabled);
    }

    0
}

// CONFIG_SYSCTL conditional: the following implementation is present when
// the kernel is built with sysctl support.
#[cfg(CONFIG_SYSCTL)]
pub unsafe extern "C" fn nf_hooks_lwtunnel_sysctl_handler(
    table: *const ctl_table,
    write: i32,
    buffer: *mut core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> i32 {
    let mut proc_nf_hooks_lwtunnel_enabled: i32 = 0;
    let mut tmp = ctl_table {
        procname: (*table).procname,
        data: &mut proc_nf_hooks_lwtunnel_enabled as *mut i32 as *mut core::ffi::c_void,
        maxlen: core::mem::size_of::<i32>(),
        mode: (*table).mode,
        extra1: SYSCTL_ZERO,
        extra2: SYSCTL_ONE,
        ..core::mem::zeroed()
    };
    let mut ret: i32;

    if write == 0 {
        proc_nf_hooks_lwtunnel_enabled = nf_hooks_lwtunnel_get();
    }

    ret = proc_dointvec_minmax(&mut tmp, write, buffer, lenp, ppos);

    if write != 0 && ret == 0 {
        ret = nf_hooks_lwtunnel_set(proc_nf_hooks_lwtunnel_enabled);
    }

    ret
}

#[cfg(CONFIG_SYSCTL)]
static mut nf_lwtunnel_sysctl_table: [ctl_table; 1] = [ctl_table {
    procname: c"nf_hooks_lwtunnel".as_ptr(),
    data: core::ptr::null_mut(),
    maxlen: core::mem::size_of::<i32>(),
    mode: 0o644,
    proc_handler: Some(nf_hooks_lwtunnel_sysctl_handler),
    ..unsafe { core::mem::zeroed() }
}];

#[cfg(CONFIG_SYSCTL)]
unsafe fn nf_lwtunnel_net_init(net: *mut net) -> i32 {
    let mut table: *const ctl_table;
    let hdr: *mut ctl_table_header;

    table = nf_lwtunnel_sysctl_table.as_ptr();
    if !net_eq(net, &raw const init_net) {
        table = kmemdup(
            nf_lwtunnel_sysctl_table.as_ptr() as *const core::ffi::c_void,
            core::mem::size_of_val(&nf_lwtunnel_sysctl_table),
            GFP_KERNEL,
        ) as *const ctl_table;
        if table.is_null() {
            return -ENOMEM;
        }
    }

    hdr = register_net_sysctl_sz(
        net,
        c"net/netfilter".as_ptr(),
        table,
        nf_lwtunnel_sysctl_table.len(),
    );
    if hdr.is_null() {
        if !net_eq(net, &raw const init_net) {
            kfree(table as *mut core::ffi::c_void);
        }
        return -ENOMEM;
    }

    (*net).nf.nf_lwtnl_dir_header = hdr;
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn nf_lwtunnel_net_exit(net: *mut net) {
    let table = (*(*net).nf.nf_lwtnl_dir_header).ctl_table_arg;
    unregister_net_sysctl_table((*net).nf.nf_lwtnl_dir_header);
    if !net_eq(net, &raw const init_net) {
        kfree(table as *mut core::ffi::c_void);
    }
}

#[cfg(CONFIG_SYSCTL)]
static mut nf_lwtunnel_net_ops: pernet_operations = pernet_operations {
    init: Some(nf_lwtunnel_net_init),
    exit: Some(nf_lwtunnel_net_exit),
    ..unsafe { core::mem::zeroed() }
};

#[cfg(CONFIG_SYSCTL)]
pub unsafe extern "C" fn netfilter_lwtunnel_init() -> i32 {
    register_pernet_subsys(&raw mut nf_lwtunnel_net_ops)
}

#[cfg(CONFIG_SYSCTL)]
pub unsafe extern "C" fn netfilter_lwtunnel_fini() {
    unregister_pernet_subsys(&raw mut nf_lwtunnel_net_ops);
}

#[cfg(not(CONFIG_SYSCTL))]
pub extern "C" fn netfilter_lwtunnel_init() -> i32 {
    0
}

#[cfg(not(CONFIG_SYSCTL))]
pub extern "C" fn netfilter_lwtunnel_fini() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
