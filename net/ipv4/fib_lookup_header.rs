/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding kernel headers:
// linux/types.h, linux/list.h, net/inet_dscp.h, net/ip_fib.h, net/nexthop.h

#[repr(C)]
pub struct fib_alias {
    pub fa_list: hlist_node,
    pub fa_info: *mut fib_info,
    pub fa_dscp: dscp_t,
    pub fa_type: u8,
    pub fa_state: u8,
    pub fa_slen: u8,
    pub tb_id: u32,
    pub fa_default: i16,
    pub offload: u8,
    pub trap: u8,
    pub offload_failed: u8,
    pub rcu: rcu_head,
}

pub const FA_S_ACCESSED: u8 = 0x01;

/* Don't write on fa_state unless needed, to keep it shared on all cpus */
#[inline]
pub unsafe fn fib_alias_accessed(fa: *mut fib_alias) {
    let fa_state: u8 = core::ptr::read_volatile(core::ptr::addr_of!((*fa).fa_state));

    if (fa_state & FA_S_ACCESSED) == 0 {
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!((*fa).fa_state),
            fa_state | FA_S_ACCESSED,
        );
    }
}

/* Exported by fib_semantics.c */
unsafe extern "C" {
    pub fn fib_release_info(fi: *mut fib_info);
    pub fn fib_create_info(
        cfg: *mut fib_config,
        extack: *mut netlink_ext_ack,
    ) -> *mut fib_info;
    pub fn fib_nh_match(
        net: *mut net,
        cfg: *mut fib_config,
        fi: *mut fib_info,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn fib_metrics_match(cfg: *mut fib_config, fi: *mut fib_info) -> bool;
    pub fn fib_dump_info(
        skb: *mut sk_buff,
        pid: u32,
        seq: u32,
        event: i32,
        fri: *const fib_rt_info,
        flags: u32,
    ) -> i32;
    pub fn rtmsg_fib(
        event: i32,
        key: __be32,
        fa: *mut fib_alias,
        dst_len: i32,
        tb_id: u32,
        info: *const nl_info,
        nlm_flags: u32,
    );
    pub fn fib_nlmsg_size(fi: *mut fib_info) -> usize;
}

#[inline]
pub unsafe fn fib_result_assign(res: *mut fib_result, fi: *mut fib_info) {
    /* we used to play games with refcounts, but we now use RCU */
    (*res).fi = fi;
    (*res).nhc = fib_info_nhc(fi, 0);
}

#[repr(C)]
pub struct fib_prop {
    pub error: i32,
    pub scope: u8,
}

unsafe extern "C" {
    pub static fib_props: [fib_prop; (RTN_MAX + 1) as usize];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
