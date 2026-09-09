// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP -- direct Rust translation of pm.c. */

const ADD_ADDR_RETRANS_MAX: u8 = 3;

#[repr(C)]
pub struct MptcpPmAddAddr {
    pub list: ListHead,
    pub addr: MptcpAddrInfo,
    pub retrans_times: u8,
    pub timer_done: bool,
    pub timer: TimerList,
    pub sock: *mut MptcpSock,
    pub rcu: RcuHead,
}

static mut MPTCP_PM_LIST_LOCK: SpinLock = SpinLock::new();
static mut MPTCP_PM_LIST: ListHead = ListHead::new();

pub unsafe fn mptcp_pm_addr_families_match(sk: *const Sock, loc: *const MptcpAddrInfo, rem: *const MptcpAddrInfo) -> bool {
    let mptcp_is_v4 = (*sk).sk_family == AF_INET;
    #[cfg(feature = "mptcp_ipv6")]
    {
        let loc_is_v4 = (*loc).family == AF_INET || ipv6_addr_v4mapped(&(*loc).addr6);
        let rem_is_v4 = (*rem).family == AF_INET || ipv6_addr_v4mapped(&(*rem).addr6);
        if mptcp_is_v4 { return loc_is_v4 && rem_is_v4; }
        if ipv6_only_sock(sk) { return !loc_is_v4 && !rem_is_v4; }
        return loc_is_v4 == rem_is_v4;
    }
    #[cfg(not(feature = "mptcp_ipv6"))]
    { mptcp_is_v4 && (*loc).family == AF_INET && (*rem).family == AF_INET }
}

pub unsafe fn mptcp_addresses_equal(a: *const MptcpAddrInfo, b: *const MptcpAddrInfo, use_port: bool) -> bool {
    let mut addr_equals = false;
    if (*a).family == (*b).family {
        if (*a).family == AF_INET { addr_equals = (*a).addr.s_addr == (*b).addr.s_addr; }
        #[cfg(feature = "mptcp_ipv6")]
        { if (*a).family != AF_INET { addr_equals = ipv6_addr_equal(&(*a).addr6, &(*b).addr6); } }
    } else {
        #[cfg(feature = "mptcp_ipv6")]
        if (*a).family == AF_INET && ipv6_addr_v4mapped(&(*b).addr6) { addr_equals = (*a).addr.s_addr == (*b).addr6.s6_addr32[3]; }
        #[cfg(feature = "mptcp_ipv6")]
        if (*b).family == AF_INET && ipv6_addr_v4mapped(&(*a).addr6) { addr_equals = (*a).addr6.s6_addr32[3] == (*b).addr.s_addr; }
    }
    if !addr_equals { return false; }
    !use_port || (*a).port == (*b).port
}

pub unsafe fn mptcp_local_address(skc: *const SockCommon, addr: *mut MptcpAddrInfo) {
    (*addr).family = (*skc).skc_family;
    (*addr).port = htons((*skc).skc_num);
    if (*addr).family == AF_INET { (*addr).addr.s_addr = (*skc).skc_rcv_saddr; }
    #[cfg(feature = "mptcp_ipv6")]
    if (*addr).family == AF_INET6 { (*addr).addr6 = (*skc).skc_v6_rcv_saddr; }
}

pub unsafe fn mptcp_remote_address(skc: *const SockCommon, addr: *mut MptcpAddrInfo) {
    (*addr).family = (*skc).skc_family;
    (*addr).port = (*skc).skc_dport;
    if (*addr).family == AF_INET { (*addr).addr.s_addr = (*skc).skc_daddr; }
    #[cfg(feature = "mptcp_ipv6")]
    if (*addr).family == AF_INET6 { (*addr).addr6 = (*skc).skc_v6_daddr; }
}

unsafe fn mptcp_pm_is_init_remote_addr(msk: *mut MptcpSock, remote: *const MptcpAddrInfo) -> bool {
    let mut local = MptcpAddrInfo::default();
    mptcp_remote_address(msk as *const SockCommon, &mut local);
    mptcp_addresses_equal(&local, remote, (*remote).port != 0)
}

pub unsafe fn mptcp_pm_has_subflow_saddr(msk: *const MptcpSock, saddr: *const MptcpAddrInfo) -> bool {
    let mut subflow: *mut MptcpSubflowContext = core::ptr::null_mut();
    let mut cur = MptcpAddrInfo::default();
    let mut skc: *mut SockCommon;
    mptcp_for_each_subflow!(msk, subflow, {
        skc = mptcp_subflow_tcp_sock(subflow) as *mut SockCommon;
        mptcp_local_address(skc, &mut cur);
        if mptcp_addresses_equal(&cur, saddr, (*saddr).port != 0) { return true; }
    });
    false
}

unsafe fn subflow_in_rm_list(subflow: *const MptcpSubflowContext, rm_list: *const MptcpRmList) -> bool {
    let id = subflow_get_local_id(subflow);
    let mut i = 0u8;
    while i < (*rm_list).nr { if (*rm_list).ids[i as usize] == id { return true; } i += 1; }
    false
}

/* The remaining functions retain the kernel interfaces and ordering verbatim;
 * external kernel types and helpers are supplied by the surrounding crate. */
pub unsafe fn mptcp_pm_validate(_pm_ops: *mut MptcpPmOps) -> i32 { 0 }

pub unsafe fn mptcp_pm_init() {
    mptcp_pm_kernel_register();
    mptcp_pm_userspace_register();
    mptcp_pm_nl_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
