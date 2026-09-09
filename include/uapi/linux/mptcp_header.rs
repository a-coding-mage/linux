/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux UAPI headers are intentionally
// left external to this translation.

pub const MPTCP_SUBFLOW_FLAG_MCAP_REM: usize = _BITUL(0);
pub const MPTCP_SUBFLOW_FLAG_MCAP_LOC: usize = _BITUL(1);
pub const MPTCP_SUBFLOW_FLAG_JOIN_REM: usize = _BITUL(2);
pub const MPTCP_SUBFLOW_FLAG_JOIN_LOC: usize = _BITUL(3);
pub const MPTCP_SUBFLOW_FLAG_BKUP_REM: usize = _BITUL(4);
pub const MPTCP_SUBFLOW_FLAG_BKUP_LOC: usize = _BITUL(5);
pub const MPTCP_SUBFLOW_FLAG_FULLY_ESTABLISHED: usize = _BITUL(6);
pub const MPTCP_SUBFLOW_FLAG_CONNECTED: usize = _BITUL(7);
pub const MPTCP_SUBFLOW_FLAG_MAPVALID: usize = _BITUL(8);

pub const MPTCP_PM_CMD_GRP_NAME: &str = "mptcp_pm_cmds";
pub const MPTCP_PM_EV_GRP_NAME: &str = "mptcp_pm_events";

pub const MPTCP_INFO_FLAG_FALLBACK: usize = _BITUL(0);
pub const MPTCP_INFO_FLAG_REMOTE_KEY_RECEIVED: usize = _BITUL(1);

pub const MPTCP_PM_EV_FLAG_DENY_JOIN_ID0: usize = _BITUL(0);
pub const MPTCP_PM_EV_FLAG_SERVER_SIDE: usize = _BITUL(1);

pub const MPTCP_PM_ADDR_FLAG_SIGNAL: usize = _BITUL(0);
pub const MPTCP_PM_ADDR_FLAG_SUBFLOW: usize = _BITUL(1);
pub const MPTCP_PM_ADDR_FLAG_BACKUP: usize = _BITUL(2);
pub const MPTCP_PM_ADDR_FLAG_FULLMESH: usize = _BITUL(3);
pub const MPTCP_PM_ADDR_FLAG_IMPLICIT: usize = _BITUL(4);
pub const MPTCP_PM_ADDR_FLAG_LAMINAR: usize = _BITUL(5);
pub const MPTCP_PM_ADDR_FLAGS_MASK: usize = GENMASK(5, 0);

#[repr(C)]
pub struct mptcp_info {
    pub mptcpi_subflows: __u8,
    pub mptcpi_add_addr_signal: __u8,
    pub mptcpi_add_addr_accepted: __u8,
    pub mptcpi_subflows_max: __u8,
    pub mptcpi_add_addr_signal_max: __u8,
    pub mptcpi_add_addr_accepted_max: __u8,
    // 16-bit hole that can no longer be filled
    pub mptcpi_flags: __u32,
    pub mptcpi_token: __u32,
    pub mptcpi_write_seq: __u64,
    pub mptcpi_snd_una: __u64,
    pub mptcpi_rcv_nxt: __u64,
    pub mptcpi_local_addr_used: __u8,
    pub mptcpi_local_addr_max: __u8,
    pub mptcpi_csum_enabled: __u8,
    // 8-bit hole that can no longer be filled
    pub mptcpi_retransmits: __u32,
    pub mptcpi_bytes_retrans: __u64,
    pub mptcpi_bytes_sent: __u64,
    pub mptcpi_bytes_received: __u64,
    pub mptcpi_bytes_acked: __u64,
    pub mptcpi_subflows_total: __u8,
    pub mptcpi_endp_laminar_max: __u8,
    pub mptcpi_endp_fullmesh_max: __u8,
    pub reserved: __u8,
    pub mptcpi_last_data_sent: __u32,
    pub mptcpi_last_data_recv: __u32,
    pub mptcpi_last_ack_recv: __u32,
}

// Field aliases from the C header:
// mptcpi_extra_subflows = mptcpi_subflows
// mptcpi_limit_extra_subflows = mptcpi_subflows_max
// mptcpi_endp_signal_max = mptcpi_add_addr_signal_max
// mptcpi_limit_add_addr_accepted = mptcpi_add_addr_accepted_max
// mptcpi_endp_subflow_max = mptcpi_local_addr_max

/* MPTCP Reset reason codes, rfc8684 */
pub const MPTCP_RST_EUNSPEC: __u32 = 0;
pub const MPTCP_RST_EMPTCP: __u32 = 1;
pub const MPTCP_RST_ERESOURCE: __u32 = 2;
pub const MPTCP_RST_EPROHIBIT: __u32 = 3;
pub const MPTCP_RST_EWQ2BIG: __u32 = 4;
pub const MPTCP_RST_EBADPERF: __u32 = 5;
pub const MPTCP_RST_EMIDDLEBOX: __u32 = 6;

#[repr(C, align(8))]
pub struct mptcp_subflow_data {
    pub size_subflow_data: __u32, // size of this structure in userspace
    pub num_subflows: __u32,      // must be 0, set by kernel
    pub size_kernel: __u32,       // must be 0, set by kernel
    pub size_user: __u32,         // size of one element in data[]
}

#[repr(C)]
pub union mptcp_subflow_addrs__bindgen_ty_1 {
    pub sa_family: __kernel_sa_family_t,
    pub sa_local: sockaddr,
    pub sin_local: sockaddr_in,
    pub sin6_local: sockaddr_in6,
    pub ss_local: __kernel_sockaddr_storage,
}

#[repr(C)]
pub union mptcp_subflow_addrs__bindgen_ty_2 {
    pub sa_remote: sockaddr,
    pub sin_remote: sockaddr_in,
    pub sin6_remote: sockaddr_in6,
    pub ss_remote: __kernel_sockaddr_storage,
}

#[repr(C)]
pub struct mptcp_subflow_addrs {
    pub __bindgen_anon_1: mptcp_subflow_addrs__bindgen_ty_1,
    pub __bindgen_anon_2: mptcp_subflow_addrs__bindgen_ty_2,
}

#[repr(C)]
pub struct mptcp_subflow_info {
    pub id: __u32,
    pub addrs: mptcp_subflow_addrs,
}

#[repr(C)]
pub struct mptcp_full_info {
    pub size_tcpinfo_kernel: __u32, // must be 0, set by kernel
    pub size_tcpinfo_user: __u32,
    pub size_sfinfo_kernel: __u32, // must be 0, set by kernel
    pub size_sfinfo_user: __u32,
    pub num_subflows: __u32, // must be 0, set by kernel (real subflow count)
    pub size_arrays_user: __u32,
    pub subflow_info: __aligned_u64,
    pub tcp_info: __aligned_u64,
    pub mptcp_info: mptcp_info,
}

pub const MPTCP_INFO: __u32 = 1;
pub const MPTCP_TCPINFO: __u32 = 2;
pub const MPTCP_SUBFLOW_ADDRS: __u32 = 3;
pub const MPTCP_FULL_INFO: __u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
