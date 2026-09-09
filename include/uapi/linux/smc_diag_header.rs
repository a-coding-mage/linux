/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies supplied by the corresponding Linux headers. */

/* Request structure */
#[repr(C)]
pub struct smc_diag_req {
    pub diag_family: u8,
    pub pad: [u8; 2],
    pub diag_ext: u8, /* Query extended information */
    pub id: inet_diag_sockid,
}

/* Base info structure. It contains socket identity (addrs/ports/cookie) based
 * on the internal clcsock, and more SMC-related socket data
 */
#[repr(C)]
pub union smc_diag_msg__bindgen_ty_1 {
    pub diag_mode: u8,
    pub diag_fallback: u8, /* the old name of the field */
}

#[repr(C)]
pub struct smc_diag_msg {
    pub diag_family: u8,
    pub diag_state: u8,
    pub __bindgen_anon_1: smc_diag_msg__bindgen_ty_1,
    pub diag_shutdown: u8,
    pub id: inet_diag_sockid,
    pub diag_uid: u32,
    pub diag_inode: u64,
}

/* Mode of a connection */
pub const SMC_DIAG_MODE_SMCR: u32 = 0;
pub const SMC_DIAG_MODE_FALLBACK_TCP: u32 = 1;
pub const SMC_DIAG_MODE_SMCD: u32 = 2;

/* Extensions */
pub const SMC_DIAG_NONE: u32 = 0;
pub const SMC_DIAG_CONNINFO: u32 = 1;
pub const SMC_DIAG_LGRINFO: u32 = 2;
pub const SMC_DIAG_SHUTDOWN: u32 = 3;
pub const SMC_DIAG_DMBINFO: u32 = 4;
pub const SMC_DIAG_FALLBACK: u32 = 5;
pub const __SMC_DIAG_MAX: u32 = 6;
pub const SMC_DIAG_MAX: u32 = __SMC_DIAG_MAX - 1;

/* SMC_DIAG_CONNINFO */

#[repr(C)]
pub struct smc_diag_cursor {
    pub reserved: u16,
    pub wrap: u16,
    pub count: u32,
}

#[repr(C)]
pub struct smc_diag_conninfo {
    pub token: u32, /* unique connection id */
    pub sndbuf_size: u32, /* size of send buffer */
    pub rmbe_size: u32, /* size of RMB element */
    pub peer_rmbe_size: u32, /* size of peer RMB element */
    /* local RMB element cursors */
    pub rx_prod: smc_diag_cursor, /* received producer cursor */
    pub rx_cons: smc_diag_cursor, /* received consumer cursor */
    /* peer RMB element cursors */
    pub tx_prod: smc_diag_cursor, /* sent producer cursor */
    pub tx_cons: smc_diag_cursor, /* sent consumer cursor */
    pub rx_prod_flags: u8, /* received producer flags */
    pub rx_conn_state_flags: u8, /* recvd connection flags*/
    pub tx_prod_flags: u8, /* sent producer flags */
    pub tx_conn_state_flags: u8, /* sent connection flags*/
    /* send buffer cursors */
    pub tx_prep: smc_diag_cursor, /* prepared to be sent cursor */
    pub tx_sent: smc_diag_cursor, /* sent cursor */
    pub tx_fin: smc_diag_cursor, /* confirmed sent cursor */
}

/* SMC_DIAG_LINKINFO */

#[repr(C)]
pub struct smc_diag_linkinfo {
    pub link_id: u8, /* link identifier */
    pub ibname: [u8; IB_DEVICE_NAME_MAX], /* name of the RDMA device */
    pub ibport: u8, /* RDMA device port number */
    pub gid: [u8; 40], /* local GID */
    pub peer_gid: [u8; 40], /* peer GID */
}

#[repr(C)]
pub struct smc_diag_lgrinfo {
    pub lnk: [smc_diag_linkinfo; 1],
    pub role: u8,
}

#[repr(C)]
pub struct smc_diag_fallback {
    pub reason: u32,
    pub peer_diagnosis: u32,
}

#[repr(C)]
pub struct smcd_diag_dmbinfo { /* SMC-D Socket internals */
    pub linkid: u32, /* Link identifier */
    pub peer_gid: u64, /* Peer GID */
    pub my_gid: u64, /* My GID */
    pub token: u64, /* Token of DMB */
    pub peer_token: u64, /* Token of remote DMBE */
    pub peer_gid_ext: u64, /* Peer GID (extended part) */
    pub my_gid_ext: u64, /* My GID (extended part) */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
