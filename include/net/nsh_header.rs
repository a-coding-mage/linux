// Network Service Header definitions translated from nsh.h.
// Dependency intent: `sk_buff`, `skb_network_header`, `ntohs`, and `htons`
// are supplied by the surrounding networking implementation.

#[repr(C)]
pub struct nsh_md1_ctx {
    pub context: [u32; 4],
}

#[repr(C)]
pub struct nsh_md2_tlv {
    pub md_class: u16,
    pub r#type: u8,
    pub length: u8,
    pub md_value: [u8; 0],
}

#[repr(C)]
pub union nshhdr_md {
    pub md1: nsh_md1_ctx,
    pub md2: nsh_md2_tlv,
}

#[repr(C)]
pub struct nshhdr {
    pub ver_flags_ttl_len: u16,
    pub mdtype: u8,
    pub np: u8,
    pub path_hdr: u32,
    pub md: nshhdr_md,
}

/* Masking NSH header fields. */
pub const NSH_VER_MASK: u16 = 0xc000;
pub const NSH_VER_SHIFT: u32 = 14;
pub const NSH_FLAGS_MASK: u16 = 0x3000;
pub const NSH_FLAGS_SHIFT: u32 = 12;
pub const NSH_TTL_MASK: u16 = 0x0fc0;
pub const NSH_TTL_SHIFT: u32 = 6;
pub const NSH_LEN_MASK: u16 = 0x003f;
pub const NSH_LEN_SHIFT: u32 = 0;

pub const NSH_MDTYPE_MASK: u8 = 0x0f;
pub const NSH_MDTYPE_SHIFT: u32 = 0;

pub const NSH_SPI_MASK: u32 = 0xffffff00;
pub const NSH_SPI_SHIFT: u32 = 8;
pub const NSH_SI_MASK: u32 = 0x000000ff;
pub const NSH_SI_SHIFT: u32 = 0;

/* MD Type Registry. */
pub const NSH_M_TYPE1: u8 = 0x01;
pub const NSH_M_TYPE2: u8 = 0x02;
pub const NSH_M_EXP1: u8 = 0xFE;
pub const NSH_M_EXP2: u8 = 0xFF;

/* NSH Base Header Length */
pub const NSH_BASE_HDR_LEN: u32 = 8;

/* NSH MD Type 1 header Length. */
pub const NSH_M_TYPE1_LEN: u32 = 24;

/* NSH header maximum Length. */
pub const NSH_HDR_MAX_LEN: u32 = ((NSH_LEN_MASK as u32 >> NSH_LEN_SHIFT) * 4);

/* NSH context headers maximum Length. */
pub const NSH_CTX_HDRS_MAX_LEN: u32 = NSH_HDR_MAX_LEN - NSH_BASE_HDR_LEN;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

extern "C" {
    pub fn skb_network_header(skb: *mut sk_buff) -> *mut core::ffi::c_void;
    pub fn ntohs(value: u16) -> u16;
    pub fn htons(value: u16) -> u16;
}

pub unsafe fn nsh_hdr(skb: *mut sk_buff) -> *mut nshhdr {
    skb_network_header(skb) as *mut nshhdr
}

pub unsafe fn nsh_hdr_len(nsh: *const nshhdr) -> u16 {
    (((ntohs((*nsh).ver_flags_ttl_len) & NSH_LEN_MASK) >> NSH_LEN_SHIFT) << 2)
}

pub unsafe fn nsh_get_ver(nsh: *const nshhdr) -> u8 {
    ((ntohs((*nsh).ver_flags_ttl_len) & NSH_VER_MASK) >> NSH_VER_SHIFT) as u8
}

pub unsafe fn nsh_get_flags(nsh: *const nshhdr) -> u8 {
    ((ntohs((*nsh).ver_flags_ttl_len) & NSH_FLAGS_MASK) >> NSH_FLAGS_SHIFT) as u8
}

pub unsafe fn nsh_get_ttl(nsh: *const nshhdr) -> u8 {
    ((ntohs((*nsh).ver_flags_ttl_len) & NSH_TTL_MASK) >> NSH_TTL_SHIFT) as u8
}

pub unsafe fn __nsh_set_xflag(nsh: *mut nshhdr, xflag: u16, xmask: u16) {
    (*nsh).ver_flags_ttl_len =
        ((*nsh).ver_flags_ttl_len & !htons(xmask)) | htons(xflag);
}

pub unsafe fn nsh_set_flags_and_ttl(nsh: *mut nshhdr, flags: u8, ttl: u8) {
    __nsh_set_xflag(
        nsh,
        (((flags as u16) << NSH_FLAGS_SHIFT) & NSH_FLAGS_MASK)
            | (((ttl as u16) << NSH_TTL_SHIFT) & NSH_TTL_MASK),
        NSH_FLAGS_MASK | NSH_TTL_MASK,
    );
}

pub unsafe fn nsh_set_flags_ttl_len(nsh: *mut nshhdr, flags: u8, ttl: u8, mut len: u8) {
    len >>= 2;
    __nsh_set_xflag(
        nsh,
        (((flags as u16) << NSH_FLAGS_SHIFT) & NSH_FLAGS_MASK)
            | (((ttl as u16) << NSH_TTL_SHIFT) & NSH_TTL_MASK)
            | (((len as u16) << NSH_LEN_SHIFT) & NSH_LEN_MASK),
        NSH_FLAGS_MASK | NSH_TTL_MASK | NSH_LEN_MASK,
    );
}

extern "C" {
    pub fn nsh_push(skb: *mut sk_buff, pushed_nh: *const nshhdr) -> i32;
    pub fn nsh_pop(skb: *mut sk_buff) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
