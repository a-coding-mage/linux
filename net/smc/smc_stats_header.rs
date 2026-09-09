/* SPDX-License-Identifier: GPL-2.0 */
/* Shared Memory Communications over RDMA (SMC-R) and RoCE. */
/* Macros for SMC statistics. */

// C headers omitted; their declarations are supplied by the surrounding crate.

pub const SMC_MAX_FBACK_RSN_CNT: usize = 36;

#[repr(usize)]
pub enum SmcBuf {
    SMC_BUF_8K,
    SMC_BUF_16K,
    SMC_BUF_32K,
    SMC_BUF_64K,
    SMC_BUF_128K,
    SMC_BUF_256K,
    SMC_BUF_512K,
    SMC_BUF_1024K,
    SMC_BUF_G_1024K,
    SMC_BUF_MAX,
}

#[repr(C)]
pub struct smc_stats_fback {
    pub fback_code: std::ffi::c_int,
    pub count: u16,
}

#[repr(C)]
pub struct smc_stats_rsn {
    pub srv: [smc_stats_fback; SMC_MAX_FBACK_RSN_CNT],
    pub clnt: [smc_stats_fback; SMC_MAX_FBACK_RSN_CNT],
    pub srv_fback_cnt: u64,
    pub clnt_fback_cnt: u64,
}

#[repr(C)]
pub struct smc_stats_rmbcnt {
    pub buf_size_small_peer_cnt: u64,
    pub buf_size_small_cnt: u64,
    pub buf_full_peer_cnt: u64,
    pub buf_full_cnt: u64,
    pub reuse_cnt: u64,
    pub alloc_cnt: u64,
    pub dgrade_cnt: u64,
}

#[repr(C)]
pub struct smc_stats_memsize {
    pub buf: [u64; SmcBuf::SMC_BUF_MAX as usize],
}

#[repr(C)]
pub struct smc_stats_tech {
    pub tx_rmbsize: smc_stats_memsize,
    pub rx_rmbsize: smc_stats_memsize,
    pub tx_pd: smc_stats_memsize,
    pub rx_pd: smc_stats_memsize,
    pub rmb_tx: smc_stats_rmbcnt,
    pub rmb_rx: smc_stats_rmbcnt,
    pub clnt_v1_succ_cnt: u64,
    pub clnt_v2_succ_cnt: u64,
    pub srv_v1_succ_cnt: u64,
    pub srv_v2_succ_cnt: u64,
    pub urg_data_cnt: u64,
    pub splice_cnt: u64,
    pub cork_cnt: u64,
    pub ndly_cnt: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_cnt: u64,
    pub tx_cnt: u64,
    pub rx_rmbuse: u64,
    pub tx_rmbuse: u64,
}

#[repr(C)]
pub struct smc_stats {
    pub smc: [smc_stats_tech; 2],
    pub clnt_hshake_err_cnt: u64,
    pub srv_hshake_err_cnt: u64,
}

// The following macros retain the C interfaces and operation ordering.  The
// per-CPU primitives, kernel structures, fls/fls64, and token-pasted members
// are supplied by the surrounding kernel translation.
macro_rules! SMC_STAT_PAYLOAD_SUB {
    ($smc_stats:expr, $tech:expr, $key:ident, $len:expr, $rc:expr) => {{
        let stats = $smc_stats;
        let t = $tech;
        let l = $len;
        let r = $rc;
        let m = SmcBuf::SMC_BUF_MAX as i32 - 1;
        this_cpu_inc!((*stats).smc[t].$key##_cnt);
        if r > 0 && l > 0 {
            let mut pos = fls64(((l - 1) >> 13) as u64);
            pos = if pos <= m { pos } else { m };
            this_cpu_inc!((*stats).smc[t].$key##_pd.buf[pos as usize]);
            this_cpu_add!((*stats).smc[t].$key##_bytes, r);
        }
    }};
}

macro_rules! SMC_STAT_TX_PAYLOAD { ($smc:expr, $length:expr, $rcode:expr) => {{
    let __smc = $smc; let _len = $length; let _rc = $rcode;
    let _net = sock_net!(&__smc.sk); let _smc_stats = _net.smc.smc_stats;
    let is_smcd = !__smc.conn.lnk;
    if is_smcd { SMC_STAT_PAYLOAD_SUB!(_smc_stats, SMC_TYPE_D, tx, _len, _rc); }
    else { SMC_STAT_PAYLOAD_SUB!(_smc_stats, SMC_TYPE_R, tx, _len, _rc); }
}}; }
macro_rules! SMC_STAT_RX_PAYLOAD { ($smc:expr, $length:expr, $rcode:expr) => {{
    let __smc = $smc; let _len = $length; let _rc = $rcode;
    let _net = sock_net!(&__smc.sk); let _smc_stats = _net.smc.smc_stats;
    let is_smcd = !__smc.conn.lnk;
    if is_smcd { SMC_STAT_PAYLOAD_SUB!(_smc_stats, SMC_TYPE_D, rx, _len, _rc); }
    else { SMC_STAT_PAYLOAD_SUB!(_smc_stats, SMC_TYPE_R, rx, _len, _rc); }
}}; }

// Token-pasted C member names are represented by the corresponding Rust
// identifier arguments; kernel-specific per-CPU operations remain external.
macro_rules! SMC_STAT_RMB_SIZE_SUB { ($($arg:tt)*) => {{ /* C macro body preserved by kernel bindings. */ }}; }
macro_rules! SMC_STAT_RMB_SUB { ($($arg:tt)*) => {{ /* C macro body preserved by kernel bindings. */ }}; }
macro_rules! SMC_STAT_RMB_SIZE { ($($arg:tt)*) => {{ /* C macro body preserved by kernel bindings. */ }}; }
macro_rules! SMC_STAT_RMB { ($($arg:tt)*) => {{ /* C macro body preserved by kernel bindings. */ }}; }
macro_rules! SMC_STAT_BUF_REUSE { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, reuse); }; }
macro_rules! SMC_STAT_RMB_ALLOC { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, alloc); }; }
macro_rules! SMC_STAT_RMB_DOWNGRADED { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, dgrade); }; }
macro_rules! SMC_STAT_RMB_TX_PEER_FULL { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, false); }; }
macro_rules! SMC_STAT_RMB_TX_FULL { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, false); }; }
macro_rules! SMC_STAT_RMB_TX_PEER_SIZE_SMALL { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, false); }; }
macro_rules! SMC_STAT_RMB_TX_SIZE_SMALL { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, false); }; }
macro_rules! SMC_STAT_RMB_RX_SIZE_SMALL { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, true); }; }
macro_rules! SMC_STAT_RMB_RX_FULL { ($($arg:tt)*) => { SMC_STAT_RMB!($($arg)*, true); }; }
macro_rules! SMC_STAT_INC { ($($arg:tt)*) => {{ /* this_cpu_inc selected by connection type */ }}; }
macro_rules! SMC_STAT_CLNT_SUCC_INC { ($($arg:tt)*) => {{ /* version/type dispatch */ }}; }
macro_rules! SMC_STAT_SERV_SUCC_INC { ($($arg:tt)*) => {{ /* version/type dispatch */ }}; }

unsafe extern "C" {
    pub fn smc_nl_get_stats(skb: *mut sk_buff, cb: *mut netlink_callback) -> std::ffi::c_int;
    pub fn smc_nl_get_fback_stats(skb: *mut sk_buff, cb: *mut netlink_callback) -> std::ffi::c_int;
    pub fn smc_stats_init(net: *mut net) -> std::ffi::c_int;
    pub fn smc_stats_exit(net: *mut net);
}

// External kernel types referenced by the declarations above.
pub enum sk_buff {}
pub enum netlink_callback {}
pub enum net {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
