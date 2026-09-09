/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/timer.h and net/snmp.h

use core::ffi::c_int;

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sctp_mib {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

// External kernel types supplied by other translated headers.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netns_sctp {
    pub sctp_statistics: sctp_mib,

    // CONFIG_PROC_FS
    pub proc_net_sctp: *mut proc_dir_entry,

    // CONFIG_SYSCTL
    pub sysctl_header: *mut ctl_table_header,

    /* This is the global socket data structure used for responding to
     * the Out-of-the-blue (OOTB) packets.  A control sock will be created
     * for this socket at the initialization time.
     */
    pub ctl_sock: *mut sock,

    /* UDP tunneling listening sock. */
    pub udp4_sock: *mut sock,
    pub udp6_sock: *mut sock,
    /* UDP tunneling listening port. */
    pub udp_port: c_int,
    /* UDP tunneling remote encap port. */
    pub encap_port: c_int,

    /* This is the global local address list.
     * We actively maintain this complete list of addresses on
     * the system by catching address add/delete events.
     *
     * It is a list of sctp_sockaddr_entry.
     */
    pub local_addr_list: list_head,
    pub addr_waitq: list_head,
    pub addr_wq_timer: timer_list,
    pub auto_asconf_splist: list_head,
    /* Lock that protects both addr_waitq and auto_asconf_splist */
    pub addr_wq_lock: spinlock_t,

    /* Lock that protects the local_addr_list writers */
    pub local_addr_lock: spinlock_t,

    /* RFC2960 Section 14. Suggested SCTP Protocol Parameter Values
     *
     * The following protocol parameters are RECOMMENDED:
     *
     * RTO.Initial          - 3  seconds
     * RTO.Min              - 1  second
     * RTO.Max              -  60 seconds
     * RTO.Alpha            - 1/8  (3 when converted to right shifts.)
     * RTO.Beta             - 1/4  (2 when converted to right shifts.)
     */
    pub rto_initial: u32,
    pub rto_min: u32,
    pub rto_max: u32,

    /* Note: rto_alpha and rto_beta are really defined as inverse
     * powers of two to facilitate integer operations.
     */
    pub rto_alpha: c_int,
    pub rto_beta: c_int,

    /* Max.Burst - 4 */
    pub max_burst: c_int,
    /* Whether Cookie Preservative is enabled(1) or not(0) */
    pub cookie_preserve_enable: c_int,
    /* Whether cookie authentication is enabled(1) or not(0) */
    pub cookie_auth_enable: c_int,
    /* Valid.Cookie.Life - 60 seconds */
    pub valid_cookie_life: u32,
    /* Delayed SACK timeout 200ms default */
    pub sack_timeout: u32,
    /* HB.interval - 30 seconds */
    pub hb_interval: u32,
    /* The interval for PLPMTUD probe timer */
    pub probe_interval: u32,

    /* Association.Max.Retrans - 10 attempts
     * Path.Max.Retrans - 5 attempts (per destination address)
     * Max.Init.Retransmits - 8 attempts
     */
    pub max_retrans_association: c_int,
    pub max_retrans_path: c_int,
    pub max_retrans_init: c_int,
    /* Potentially-Failed.Max.Retrans sysctl value
     * taken from:
     * http://tools.ietf.org/html/draft-nishida-tsvwg-sctp-failover-05
     */
    pub pf_retrans: c_int,
    /* Primary.Switchover.Max.Retrans sysctl value
     * taken from:
     * https://tools.ietf.org/html/rfc7829
     */
    pub ps_retrans: c_int,
    /*
     * Disable Potentially-Failed feature, the feature is enabled by default
     * pf_enable -  0  : disable pf
     *           - >0  : enable pf
     */
    pub pf_enable: c_int,
    /*
     * Disable Potentially-Failed state exposure, ignored by default
     * pf_expose -  0  : compatible with old applications (by default)
     *           -  1  : disable pf state exposure
     *           -  2  : enable  pf state exposure
     */
    pub pf_expose: c_int,
    /*
     * Policy for performing sctp/socket accounting
     * 0 - do socket level accounting, all assocs share sk_sndbuf
     * 1 - do sctp accounting, each asoc may use sk_sndbuf bytes
     */
    pub sndbuf_policy: c_int,
    /*
     * Policy for performing sctp/socket accounting
     * 0 - do socket level accounting, all assocs share sk_rcvbuf
     * 1 - do sctp accounting, each asoc may use sk_rcvbuf bytes
     */
    pub rcvbuf_policy: c_int,
    pub default_auto_asconf: c_int,
    /* Flag to indicate if addip is enabled. */
    pub addip_enable: c_int,
    pub addip_noauth: c_int,
    /* Flag to indicate if PR-SCTP is enabled. */
    pub prsctp_enable: c_int,
    /* Flag to indicate if PR-CONFIG is enabled. */
    pub reconf_enable: c_int,
    /* Flag to indicate if SCTP-AUTH is enabled */
    pub auth_enable: c_int,
    /* Flag to indicate if stream interleave is enabled */
    pub intl_enable: c_int,
    /* Flag to indicate if ecn is enabled */
    pub ecn_enable: c_int,
    /*
     * Policy to control SCTP IPv4 address scoping
     * 0 - Disable IPv4 address scoping
     * 1 - Enable IPv4 address scoping
     * 2 - Selectively allow only IPv4 private addresses
     * 3 - Selectively allow only IPv4 link local address
     */
    pub scope_policy: c_int,
    /* Threshold for rwnd update SACKS. Receive buffer shifted this many
     * bits is an indicator of when to send and window update SACK.
     */
    pub rwnd_upd_shift: c_int,
    /* Threshold for autoclose timeout, in seconds. */
    pub max_autoclose: usize,

    // CONFIG_NET_L3_MASTER_DEV
    pub l3mdev_accept: c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
