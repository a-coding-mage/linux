/*
 * Definitions for tcp compression routines.
 *
 * Translated from slhc_vj.h.  Linux IP/TCP header types are supplied by
 * other dependencies.
 */

/* SLIP compression masks for len/vers byte */
pub const SL_TYPE_IP: u8 = 0x40;
pub const SL_TYPE_UNCOMPRESSED_TCP: u8 = 0x70;
pub const SL_TYPE_COMPRESSED_TCP: u8 = 0x80;
pub const SL_TYPE_ERROR: u8 = 0x00;

/* Bits in first octet of compressed packet */
pub const NEW_C: u8 = 0x40;
pub const NEW_I: u8 = 0x20;
pub const NEW_S: u8 = 0x08;
pub const NEW_A: u8 = 0x04;
pub const NEW_W: u8 = 0x02;
pub const NEW_U: u8 = 0x01;

/* reserved, special-case values of above */
pub const SPECIAL_I: u8 = NEW_S | NEW_W | NEW_U;
pub const SPECIAL_D: u8 = NEW_S | NEW_A | NEW_W | NEW_U;
pub const SPECIALS_MASK: u8 = NEW_S | NEW_A | NEW_W | NEW_U;

pub const TCP_PUSH_BIT: u8 = 0x10;

pub type byte_t = u8;
pub type int32 = u32;

/* Linux IP/TCP header declarations are external dependencies. */
extern "C" {
    pub type iphdr;
    pub type tcphdr;
}

#[repr(C)]
pub struct cstate {
    pub cs_this: byte_t,
    pub initialized: bool,
    pub next: *mut cstate,
    pub cs_ip: iphdr,
    pub cs_tcp: tcphdr,
    pub cs_ipopt: [u8; 64],
    pub cs_tcpopt: [u8; 64],
    pub cs_hsize: i32,
}

pub const NULLSLSTATE: *mut cstate = core::ptr::null_mut();

#[repr(C)]
pub struct slcompress {
    pub tstate: *mut cstate,
    pub rstate: *mut cstate,

    pub tslot_limit: byte_t,
    pub rslot_limit: byte_t,

    pub xmit_oldest: byte_t,
    pub xmit_current: byte_t,
    pub recv_current: byte_t,

    pub flags: byte_t,
    /* tossing rcvd frames until id received */

    pub sls_o_nontcp: int32,
    pub sls_o_tcp: int32,
    pub sls_o_uncompressed: int32,
    pub sls_o_compressed: int32,
    pub sls_o_searches: int32,
    pub sls_o_misses: int32,

    pub sls_i_uncompressed: int32,
    pub sls_i_compressed: int32,
    pub sls_i_error: int32,
    pub sls_i_tossed: int32,

    pub sls_i_runt: int32,
    pub sls_i_badcheck: int32,
}

pub const SLF_TOSS: u8 = 0x01;
pub const NULLSLCOMPR: *mut slcompress = core::ptr::null_mut();

extern "C" {
    pub fn slhc_init(rslots: i32, tslots: i32) -> *mut slcompress;
    pub fn slhc_free(comp: *mut slcompress);

    pub fn slhc_compress(
        comp: *mut slcompress,
        icp: *mut u8,
        isize: i32,
        ocp: *mut u8,
        cpp: *mut *mut u8,
        compress_cid: i32,
    ) -> i32;
    pub fn slhc_uncompress(comp: *mut slcompress, icp: *mut u8, isize: i32) -> i32;
    pub fn slhc_remember(comp: *mut slcompress, icp: *mut u8, isize: i32) -> i32;
    pub fn slhc_toss(comp: *mut slcompress) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
