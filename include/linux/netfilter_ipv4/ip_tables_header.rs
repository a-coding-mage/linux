/* SPDX-License-Identifier: GPL-2.0 */
/*
 * 25-Jul-1998 Major changes to allow for ip chain table
 *
 * 3-Jan-2000 Named tables to allow packet selection for different uses.
 */

/*
 * 	Format of an IP firewall descriptor
 *
 * 	src, dst, src_mask, dst_mask are always stored in network byte order.
 * 	flags are stored in host byte order (of course).
 * 	Port numbers are stored in HOST byte order.
 */

// C dependencies: linux/if.h, linux/in.h, linux/init.h, linux/ip.h,
// linux/skbuff.h, uapi/linux/netfilter_ipv4/ip_tables.h.

extern "C" {
    pub fn ipt_register_table(
        net: *mut net,
        table: *const xt_table,
        repl: *const ipt_replace,
        ops: *const nf_hook_ops,
    ) -> core::ffi::c_int;

    pub fn ipt_unregister_table_exit(net: *mut net, name: *const core::ffi::c_char);
}

/* Standard entry. */
#[repr(C)]
pub struct ipt_standard {
    pub entry: ipt_entry,
    pub target: xt_standard_target,
}

#[repr(C)]
pub struct ipt_error {
    pub entry: ipt_entry,
    pub target: xt_error_target,
}

#[macro_export]
macro_rules! IPT_ENTRY_INIT {
    ($size:expr) => {
        ipt_entry {
            target_offset: core::mem::size_of::<ipt_entry>() as _,
            next_offset: $size,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

#[macro_export]
macro_rules! IPT_STANDARD_INIT {
    ($verdict:expr) => {
        {
            let mut value = ipt_standard {
            entry: ipt_entry {
                target_offset: core::mem::size_of::<ipt_entry>() as _,
                next_offset: core::mem::size_of::<ipt_standard>() as _,
                ..unsafe { core::mem::zeroed() }
            },
                target: unsafe { core::mem::zeroed() },
            };
            value.target = XT_TARGET_INIT!(XT_STANDARD_TARGET, core::mem::size_of::<xt_standard_target>());
            value.target.verdict = (-(($verdict) as i32) - 1) as _;
            value
        }
    };
}

#[macro_export]
macro_rules! IPT_ERROR_INIT {
    () => {
        {
            let mut value = ipt_error {
            entry: ipt_entry {
                target_offset: core::mem::size_of::<ipt_entry>() as _,
                next_offset: core::mem::size_of::<ipt_error>() as _,
                ..unsafe { core::mem::zeroed() }
            },
                target: unsafe { core::mem::zeroed() },
            };
            value.target = XT_TARGET_INIT!(XT_ERROR_TARGET, core::mem::size_of::<xt_error_target>());
            value.target.errorname = *b"ERROR\0";
            value
        }
    };
}

extern "C" {
    pub fn ipt_alloc_initial_table(table: *const xt_table) -> *mut core::ffi::c_void;
    pub fn ipt_do_table(
        priv_: *mut core::ffi::c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> core::ffi::c_uint;
}

// Preserved from CONFIG_NETFILTER_XTABLES_COMPAT. Enable this block when the
// corresponding kernel configuration condition is enabled.
#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[repr(C)]
pub struct compat_ipt_entry {
    pub ip: ipt_ip,
    pub nfcache: compat_uint_t,
    pub target_offset: u16,
    pub next_offset: u16,
    pub comefrom: compat_uint_t,
    pub counters: compat_xt_counters,
    pub elems: [u8; 0],
}

#[cfg(CONFIG_NETFILTER_XTABLES_COMPAT)]
#[inline]
pub unsafe fn compat_ipt_get_target(e: *mut compat_ipt_entry) -> *mut xt_entry_target {
    (e as *mut u8).add((*e).target_offset as usize) as *mut xt_entry_target
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
