// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match running CPU */

/*
 * Might be used to distribute connections on several daemons, if
 * RPS (Remote Packet Steering) is enabled or NIC is multiqueue capable,
 * each RX queue IRQ affined to one CPU (1:1 mapping)
 */

/* (C) 2010 Eric Dumazet
 */

// C dependencies supplied by the surrounding kernel translation.

const EINVAL: i32 = 22;
const NFPROTO_UNSPEC: u8 = 0;

#[repr(C)]
pub struct xt_mtchk_param {
    pub matchinfo: *const core::ffi::c_void,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xt_action_param {
    pub matchinfo: *const core::ffi::c_void,
}

#[repr(C)]
pub struct xt_cpu_info {
    pub cpu: u32,
    pub invert: u32,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub revision: u8,
    pub family: u8,
    pub checkentry: Option<unsafe extern "C" fn(*const xt_mtchk_param) -> i32>,
    pub r#match: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub matchsize: usize,
    pub me: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn raw_smp_processor_id() -> u32;
    fn xt_register_match(m: *mut xt_match) -> i32;
    fn xt_unregister_match(m: *mut xt_match);
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Eric Dumazet <eric.dumazet@gmail.com>");
// MODULE_DESCRIPTION("Xtables: CPU match");
// MODULE_ALIAS("ipt_cpu");
// MODULE_ALIAS("ip6t_cpu");

unsafe extern "C" fn cpu_mt_check(par: *const xt_mtchk_param) -> i32 {
    let info = (*par).matchinfo as *const xt_cpu_info;

    if (*info).invert & !1 != 0 {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn cpu_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let _ = skb;
    let info = (*par).matchinfo as *const xt_cpu_info;

    ((*info).cpu == raw_smp_processor_id()) ^ ((*info).invert != 0)
}

static mut CPU_MT_REG: xt_match = xt_match {
    name: b"cpu\0".as_ptr(),
    revision: 0,
    family: NFPROTO_UNSPEC,
    checkentry: Some(cpu_mt_check),
    r#match: Some(cpu_mt),
    matchsize: core::mem::size_of::<xt_cpu_info>(),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn cpu_mt_init() -> i32 {
    xt_register_match(&raw mut CPU_MT_REG)
}

unsafe extern "C" fn cpu_mt_exit() {
    xt_unregister_match(&raw mut CPU_MT_REG);
}

// module_init(cpu_mt_init);
// module_exit(cpu_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
