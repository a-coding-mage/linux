/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  ebtables
 *
 *	Authors:
 *	Bart De Schuymer		<bdschuym@pandora.be>
 *
 *  ebtables.c,v 2.0, April, 2002
 *
 *  This code is strongly inspired by the iptables code which is
 *  Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
 */

/* C dependencies: linux/if.h, linux/if_ether.h, and
 * uapi/linux/netfilter_bridge/ebtables.h. Their declarations are supplied
 * by the surrounding translation unit. */

#[repr(C)]
pub struct ebt_match {
    pub list: list_head,
    pub name: [core::ffi::c_char; EBT_FUNCTION_MAXNAMELEN],
    pub r#match: Option<unsafe extern "C" fn(
        skb: *const sk_buff, in_: *const net_device, out: *const net_device,
        r#match: *const xt_match, matchinfo: *const core::ffi::c_void,
        offset: core::ffi::c_int, protoff: core::ffi::c_uint,
        hotdrop: *mut bool,
    ) -> bool>,
    pub checkentry: Option<unsafe extern "C" fn(
        table: *const core::ffi::c_char, entry: *const core::ffi::c_void,
        r#match: *const xt_match, matchinfo: *mut core::ffi::c_void,
        hook_mask: core::ffi::c_uint,
    ) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(
        r#match: *const xt_match, matchinfo: *mut core::ffi::c_void,
    )>,
    pub matchsize: core::ffi::c_uint,
    pub revision: u8,
    pub family: u8,
    pub me: *mut module,
}

#[repr(C)]
pub struct ebt_watcher {
    pub list: list_head,
    pub name: [core::ffi::c_char; EBT_FUNCTION_MAXNAMELEN],
    pub target: Option<unsafe extern "C" fn(
        skb: *mut sk_buff, in_: *const net_device, out: *const net_device,
        hook_num: core::ffi::c_uint, target: *const xt_target,
        targinfo: *const core::ffi::c_void,
    ) -> core::ffi::c_uint>,
    pub checkentry: Option<unsafe extern "C" fn(
        table: *const core::ffi::c_char, entry: *const core::ffi::c_void,
        target: *const xt_target, targinfo: *mut core::ffi::c_void,
        hook_mask: core::ffi::c_uint,
    ) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(
        target: *const xt_target, targinfo: *mut core::ffi::c_void,
    )>,
    pub targetsize: core::ffi::c_uint,
    pub revision: u8,
    pub family: u8,
    pub me: *mut module,
}

#[repr(C)]
pub struct ebt_target {
    pub list: list_head,
    pub name: [core::ffi::c_char; EBT_FUNCTION_MAXNAMELEN],
    /* returns one of the standard EBT_* verdicts */
    pub target: Option<unsafe extern "C" fn(
        skb: *mut sk_buff, in_: *const net_device, out: *const net_device,
        hook_num: core::ffi::c_uint, target: *const xt_target,
        targinfo: *const core::ffi::c_void,
    ) -> core::ffi::c_uint>,
    pub checkentry: Option<unsafe extern "C" fn(
        table: *const core::ffi::c_char, entry: *const core::ffi::c_void,
        target: *const xt_target, targinfo: *mut core::ffi::c_void,
        hook_mask: core::ffi::c_uint,
    ) -> bool>,
    pub destroy: Option<unsafe extern "C" fn(
        target: *const xt_target, targinfo: *mut core::ffi::c_void,
    )>,
    pub targetsize: core::ffi::c_uint,
    pub revision: u8,
    pub family: u8,
    pub me: *mut module,
}

#[repr(C)]
pub struct ebt_chainstack {
    pub chaininfo: *mut ebt_entries, /* pointer to chain data */
    pub e: *mut ebt_entry, /* pointer to entry data */
    pub n: core::ffi::c_uint, /* n'th entry */
}

#[repr(C)]
pub struct ebt_table_info {
    /* total size of the entries */
    pub entries_size: core::ffi::c_uint,
    pub nentries: core::ffi::c_uint,
    /* pointers to the start of the chains */
    pub hook_entry: [*mut ebt_entries; NF_BR_NUMHOOKS],
    /* room to maintain the stack used for jumping from and into udc */
    pub chainstack: *mut *mut ebt_chainstack,
    pub entries: *mut core::ffi::c_char,
    pub counters: [ebt_counter; 0],
}

#[repr(C)]
pub struct ebt_table {
    pub list: list_head,
    pub name: [core::ffi::c_char; EBT_TABLE_MAXNAMELEN],
    pub table: *mut ebt_replace_kernel,
    pub valid_hooks: core::ffi::c_uint,
    pub lock: rwlock_t,
    /* the data used by the kernel */
    pub private: *mut ebt_table_info,
    pub ops: *mut nf_hook_ops,
    pub me: *mut module,
}

#[macro_export]
macro_rules! EBT_ALIGN {
    ($s:expr) => {
        (($s + (core::mem::align_of::<_xt_align>() - 1))
            & !(core::mem::align_of::<_xt_align>() - 1))
    };
}

extern "C" {
    pub fn ebt_register_table(
        net: *mut net, table: *const ebt_table, ops: *const nf_hook_ops,
    ) -> core::ffi::c_int;
    pub fn ebt_unregister_table(net: *mut net, tablename: *const core::ffi::c_char);
    pub fn ebt_unregister_table_pre_exit(
        net: *mut net, tablename: *const core::ffi::c_char,
    );
    pub fn ebt_do_table(
        priv_: *mut core::ffi::c_void, skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> core::ffi::c_uint;
    pub fn ebt_register_template(
        t: *const ebt_table,
        table_init: Option<unsafe extern "C" fn(net: *mut net) -> core::ffi::c_int>,
    ) -> core::ffi::c_int;
    pub fn ebt_unregister_template(t: *const ebt_table);
}

/* True if the hook mask denotes that the rule is in a base chain,
 * used in the check() functions */
#[macro_export]
macro_rules! BASE_CHAIN {
    ($par:expr) => { (($par).hook_mask & (1 << NF_BR_NUMHOOKS)) };
}

/* Clear the bit in the hook mask that tells if the rule is on a base chain */
#[macro_export]
macro_rules! CLEAR_BASE_CHAIN_BIT {
    ($par:expr) => { (($par).hook_mask &= !(1 << NF_BR_NUMHOOKS)) };
}

#[inline]
pub unsafe fn ebt_invalid_target(target: core::ffi::c_int) -> bool {
    target < -NUM_STANDARD_TARGETS || target >= 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
