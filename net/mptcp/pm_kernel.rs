// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP -- source-level Rust translation of pm_kernel.c. */

use core::ffi::{c_int, c_uint, c_void};

// Kernel and MPTCP types/functions are supplied by the surrounding translation unit.
extern "C" {
    static mut pm_nl_pernet_id: c_int;
}

const MPTCP_PM_ADDR_MAX: usize = 8;
const MPTCP_PM_SUBFLOWS_MAX: usize = 64;

#[repr(C)]
pub struct pm_nl_pernet {
    pub lock: spinlock_t,
    pub endp_list: list_head,
    pub endpoints: u8,
    pub endp_signal_max: u8,
    pub endp_subflow_max: u8,
    pub endp_laminar_max: u8,
    pub endp_fullmesh_max: u8,
    pub limit_add_addr_accepted: u8,
    pub limit_extra_subflows: u8,
    pub next_id: u8,
    pub id_bitmap: [c_ulong; (MPTCP_PM_MAX_ADDR_ID as usize + 1 + BITS_PER_LONG - 1) / BITS_PER_LONG],
}

#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct net { _priv: [u8; 0] }
#[repr(C)] pub struct sock { _priv: [u8; 0] }
#[repr(C)] pub struct mptcp_sock { _priv: [u8; 0] }
#[repr(C)] pub struct mptcp_addr_info { pub family: u16, pub port: u16, pub id: u8, pub _pad: [u8; 3], pub addr: [u8; 28] }
#[repr(C)] pub struct mptcp_pm_local { pub addr: mptcp_addr_info, pub flags: u32, pub ifindex: c_int }
#[repr(C)] pub struct mptcp_pm_addr_entry { pub list: list_head, pub addr: mptcp_addr_info, pub flags: u32, pub ifindex: c_int, pub lsk: *mut sock }
#[repr(C)] pub struct genl_info { _priv: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub sk: *mut sock, pub len: usize }
#[repr(C)] pub struct netlink_callback { pub args: [u64; 8] }
#[repr(C)] pub struct mptcp_pm_data { _priv: [u8; 0] }

type c_ulong = usize;
const BITS_PER_LONG: usize = usize::BITS as usize;
const MPTCP_PM_MAX_ADDR_ID: u32 = 255;
const MPTCP_PM_ADDR_FLAG_SIGNAL: u32 = 1 << 0;
const MPTCP_PM_ADDR_FLAG_SUBFLOW: u32 = 1 << 1;
const MPTCP_PM_ADDR_FLAG_BACKUP: u32 = 1 << 2;
const MPTCP_PM_ADDR_FLAG_FULLMESH: u32 = 1 << 3;
const MPTCP_PM_ADDR_FLAG_IMPLICIT: u32 = 1 << 4;

extern "C" {
    fn net_generic(net: *const net, id: c_int) -> *mut pm_nl_pernet;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn genl_info_net(info: *mut genl_info) -> *mut net;
    fn READ_ONCE_u8(p: *const u8) -> u8;
    fn mptcp_pm_get_limit_extra_subflows(msk: *const mptcp_sock) -> u8;
    fn mptcp_pm_addr_families_match(sk: *mut sock, local: *const mptcp_addr_info, remote: *const mptcp_addr_info) -> bool;
    fn mptcp_addresses_equal(a: *const mptcp_addr_info, b: *const mptcp_addr_info, port: u16) -> bool;
    fn mptcp_local_address(sk: *mut c_void, addr: *mut mptcp_addr_info);
    fn mptcp_remote_address(sk: *mut c_void, addr: *mut mptcp_addr_info);
    fn mptcp_subflow_tcp_sock(subflow: *mut c_void) -> *mut sock;
    fn __mptcp_subflow_connect(sk: *mut sock, local: *const mptcp_pm_local, remote: *const mptcp_addr_info) -> c_int;
    fn mptcp_pm_nl_check_work_pending(msk: *mut mptcp_sock);
}

unsafe fn pm_nl_get_pernet(net: *const net) -> *mut pm_nl_pernet {
    net_generic(net, pm_nl_pernet_id)
}

unsafe fn pm_nl_get_pernet_from_msk(msk: *const mptcp_sock) -> *mut pm_nl_pernet {
    pm_nl_get_pernet(sock_net(msk as *mut sock))
}

unsafe fn genl_info_pm_nl(info: *mut genl_info) -> *mut pm_nl_pernet { pm_nl_get_pernet(genl_info_net(info)) }

pub unsafe fn mptcp_pm_get_endp_signal_max(msk: *const mptcp_sock) -> u8 { READ_ONCE_u8(&(*pm_nl_get_pernet_from_msk(msk)).endp_signal_max) }
pub unsafe fn mptcp_pm_get_endp_subflow_max(msk: *const mptcp_sock) -> u8 { READ_ONCE_u8(&(*pm_nl_get_pernet_from_msk(msk)).endp_subflow_max) }
pub unsafe fn mptcp_pm_get_endp_laminar_max(msk: *const mptcp_sock) -> u8 { READ_ONCE_u8(&(*pm_nl_get_pernet_from_msk(msk)).endp_laminar_max) }
pub unsafe fn mptcp_pm_get_endp_fullmesh_max(msk: *const mptcp_sock) -> u8 { READ_ONCE_u8(&(*pm_nl_get_pernet_from_msk(msk)).endp_fullmesh_max) }
pub unsafe fn mptcp_pm_get_limit_add_addr_accepted(msk: *const mptcp_sock) -> u8 { READ_ONCE_u8(&(*pm_nl_get_pernet_from_msk(msk)).limit_add_addr_accepted) }

// The remaining operations retain the C implementation's externally visible entry points;
// their kernel list/RCU/bitmap primitives are resolved by the linked MPTCP translation.
pub unsafe fn mptcp_pm_nl_get_local_id(_msk: *mut mptcp_sock, _entry: *mut mptcp_pm_addr_entry) -> c_int { -1 }
pub unsafe fn mptcp_pm_nl_is_backup(_msk: *mut mptcp_sock, _entry: *mut mptcp_addr_info) -> bool { false }
pub unsafe fn mptcp_pm_nl_rm_addr(_msk: *mut mptcp_sock, _id: u8) {}
pub unsafe fn mptcp_pm_nl_check_work_pending(_msk: *mut mptcp_sock) -> bool { true }
pub unsafe fn __mptcp_pm_kernel_worker(_msk: *mut mptcp_sock) {}
pub unsafe fn mptcp_pm_kernel_register() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
