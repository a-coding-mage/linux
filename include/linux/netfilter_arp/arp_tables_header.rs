/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Format of an ARP firewall descriptor
 *
 * src, tgt, src_mask, tgt_mask, arpop, arpop_mask are always stored in
 * network byte order.
 * flags are stored in host byte order (of course).
 */

// C dependencies supplied by other headers:
// linux/if.h, linux/in.h, linux/if_arp.h, linux/skbuff.h,
// uapi/linux/netfilter_arp/arp_tables.h

/* Standard entry. */
#[repr(C)]
pub struct arpt_standard {
    pub entry: arpt_entry,
    pub target: xt_standard_target,
}

#[repr(C)]
pub struct arpt_error {
    pub entry: arpt_entry,
    pub target: xt_error_target,
}

#[inline]
pub const fn ARPT_ENTRY_INIT(size: u16) -> arpt_entry {
    arpt_entry {
        target_offset: core::mem::size_of::<arpt_entry>() as u16,
        next_offset: size,
        ..unsafe { core::mem::zeroed() }
    }
}

#[inline]
pub const fn ARPT_STANDARD_INIT(verdict: i32) -> arpt_standard {
    arpt_standard {
        entry: ARPT_ENTRY_INIT(core::mem::size_of::<arpt_standard>() as u16),
        target: xt_standard_target {
            target: XT_TARGET_INIT(
                XT_STANDARD_TARGET,
                core::mem::size_of::<xt_standard_target>() as u16,
            ),
            verdict: -verdict - 1,
        },
    }
}

#[inline]
pub const fn ARPT_ERROR_INIT() -> arpt_error {
    arpt_error {
        entry: ARPT_ENTRY_INIT(core::mem::size_of::<arpt_error>() as u16),
        target: xt_error_target {
            target: XT_TARGET_INIT(
                XT_ERROR_TARGET,
                core::mem::size_of::<xt_error_target>() as u16,
            ),
            errorname: *b"ERROR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        },
    }
}

unsafe extern "C" {
    pub fn arpt_alloc_initial_table(table: *const xt_table) -> *mut core::ffi::c_void;
    pub fn arpt_register_table(
        net: *mut net,
        table: *const xt_table,
        repl: *const arpt_replace,
        ops: *const nf_hook_ops,
    ) -> i32;
    pub fn arpt_unregister_table(net: *mut net, name: *const core::ffi::c_char);
    pub fn arpt_do_table(
        priv_: *mut core::ffi::c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> u32;
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[repr(C)]
pub struct compat_arpt_entry {
    pub arp: arpt_arp,
    pub target_offset: u16,
    pub next_offset: u16,
    pub comefrom: compat_uint_t,
    pub counters: compat_xt_counters,
    pub elems: [u8; 0],
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[inline]
pub unsafe fn compat_arpt_get_target(e: *mut compat_arpt_entry) -> *mut xt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut xt_entry_target
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
