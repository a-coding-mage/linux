// Translated from nfnetlink_osf.h.
//
// Original dependencies: <linux/types.h>, <linux/ip.h>, and <linux/tcp.h>.

pub const MAXGENRELEN: usize = 32;

pub const NF_OSF_GENRE: u32 = 1 << 0;
pub const NF_OSF_TTL: u32 = 1 << 1;
pub const NF_OSF_LOG: u32 = 1 << 2;
pub const NF_OSF_INVERT: u32 = 1 << 3;

pub const NF_OSF_LOGLEVEL_ALL: u32 = 0; // log all matched fingerprints
pub const NF_OSF_LOGLEVEL_FIRST: u32 = 1; // log only the first matced fingerprint
pub const NF_OSF_LOGLEVEL_ALL_KNOWN: u32 = 2; // do not log unknown packets

pub const NF_OSF_TTL_TRUE: u32 = 0; // True ip and fingerprint TTL comparison

// Check if ip TTL is less than fingerprint one
pub const NF_OSF_TTL_LESS: u32 = 1;

// Do not compare ip and fingerprint TTL at all
pub const NF_OSF_TTL_NOCHECK: u32 = 2;

pub const NF_OSF_FLAGMASK: u32 =
    NF_OSF_GENRE | NF_OSF_TTL | NF_OSF_LOG | NF_OSF_INVERT;

/* Wildcard MSS (kind of).
 * It is used to implement a state machine for the different wildcard values
 * of the MSS and window sizes.
 */
#[repr(C)]
pub struct nf_osf_wc {
    pub wc: u32,
    pub val: u32,
}

/* This struct represents IANA options
 * http://www.iana.org/assignments/tcp-parameters
 */
#[repr(C)]
pub struct nf_osf_opt {
    pub kind: u16,
    pub length: u16,
    pub wc: nf_osf_wc,
}

#[repr(C)]
pub struct nf_osf_info {
    pub genre: [core::ffi::c_char; MAXGENRELEN],
    pub len: u32,
    pub flags: u32,
    pub loglevel: u32,
    pub ttl: u32,
}

#[repr(C)]
pub struct nf_osf_user_finger {
    pub wss: nf_osf_wc,
    pub ttl: u8,
    pub df: u8,
    pub ss: u16,
    pub mss: u16,
    pub opt_num: u16,
    pub genre: [core::ffi::c_char; MAXGENRELEN],
    pub version: [core::ffi::c_char; MAXGENRELEN],
    pub subtype: [core::ffi::c_char; MAXGENRELEN],
    // MAX_IPOPTLEN is maximum if all options are NOPs or EOLs
    pub opt: [nf_osf_opt; MAX_IPOPTLEN],
}

#[repr(C)]
pub struct nf_osf_nlmsg {
    pub f: nf_osf_user_finger,
    pub ip: iphdr,
    pub tcp: tcphdr,
}

/* Defines for IANA option kinds */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum iana_options {
    OSFOPT_EOL = 0, // End of options
    OSFOPT_NOP, // NOP
    OSFOPT_MSS, // Maximum segment size
    OSFOPT_WSO, // Window scale option
    OSFOPT_SACKP, // SACK permitted
    OSFOPT_SACK, // SACK
    OSFOPT_ECHO,
    OSFOPT_ECHOREPLY,
    OSFOPT_TS, // Timestamp option
    OSFOPT_POCP, // Partial Order Connection Permitted
    OSFOPT_POSP, // Partial Order Service Profile

    /* Others are not used in the current OSF */
    OSFOPT_EMPTY = 255,
}

/* Initial window size option state machine: multiple of mss, mtu or
 * plain numeric value. Can also be made as plain numeric value which
 * is not a multiple of specified value.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nf_osf_window_size_options {
    OSF_WSS_PLAIN = 0,
    OSF_WSS_MSS,
    OSF_WSS_MTU,
    OSF_WSS_MODULO,
    OSF_WSS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nf_osf_attr_type {
    OSF_ATTR_UNSPEC = 0,
    OSF_ATTR_FINGER,
    OSF_ATTR_MAX,
}

/*
 * Add/remove fingerprint from the kernel.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nf_osf_msg_types {
    OSF_MSG_ADD = 0,
    OSF_MSG_REMOVE,
    OSF_MSG_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
