/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C, packed)]
struct cop_symcpb_aes_ecb {
    key: [u8; 32],
    __rsvd: [u8; 80],
}

#[repr(C, packed)]
struct cop_symcpb_aes_cbc {
    iv: [u8; 16],
    key: [u8; 32],
    cv: [u8; 16],
    spbc: u32,
    __rsvd: [u8; 44],
}

#[repr(C, packed)]
struct cop_symcpb_aes_gca {
    in_pat: [u8; 16],
    key: [u8; 32],
    out_pat: [u8; 16],
    spbc: u32,
    __rsvd: [u8; 44],
}

#[repr(C, packed)]
struct cop_symcpb_aes_gcm {
    in_pat_or_aad: [u8; 16],
    iv_or_cnt: [u8; 16],
    bit_length_aad: u64,
    bit_length_data: u64,
    in_s0: [u8; 16],
    key: [u8; 32],
    __rsvd1: [u8; 16],
    out_pat_or_mac: [u8; 16],
    out_s0: [u8; 16],
    out_cnt: [u8; 16],
    spbc: u32,
    __rsvd2: [u8; 12],
}

#[repr(C, packed)]
struct cop_symcpb_aes_ctr {
    iv: [u8; 16],
    key: [u8; 32],
    cv: [u8; 16],
    spbc: u32,
    __rsvd2: [u8; 44],
}

#[repr(C, packed)]
struct cop_symcpb_aes_cca {
    b0: [u8; 16],
    b1: [u8; 16],
    key: [u8; 16],
    out_pat_or_b0: [u8; 16],
    spbc: u32,
    __rsvd: [u8; 44],
}

#[repr(C, packed)]
struct cop_symcpb_aes_ccm {
    in_pat_or_b0: [u8; 16],
    iv_or_ctr: [u8; 16],
    in_s0: [u8; 16],
    key: [u8; 16],
    __rsvd1: [u8; 48],
    out_pat_or_mac: [u8; 16],
    out_s0: [u8; 16],
    out_ctr: [u8; 16],
    spbc: u32,
    __rsvd2: [u8; 12],
}

#[repr(C, packed)]
struct cop_symcpb_aes_xcbc {
    cv: [u8; 16],
    key: [u8; 16],
    __rsvd1: [u8; 16],
    out_cv_mac: [u8; 16],
    spbc: u32,
    __rsvd2: [u8; 44],
}

#[repr(C, packed)]
struct cop_symcpb_sha256 {
    message_bit_length: u64,
    __rsvd1: u64,
    input_partial_digest: [u8; 32],
    message_digest: [u8; 32],
    spbc: u32,
    __rsvd2: [u8; 44],
}

#[repr(C, packed)]
struct cop_symcpb_sha512 {
    message_bit_length_hi: u64,
    message_bit_length_lo: u64,
    input_partial_digest: [u8; 64],
    __rsvd1: [u8; 32],
    message_digest: [u8; 64],
    spbc: u32,
    __rsvd2: [u8; 76],
}

const NX_FDM_INTERMEDIATE: u32 = 0x01;
const NX_FDM_CONTINUATION: u32 = 0x02;
const NX_FDM_ENDE_ENCRYPT: u32 = 0x80;

macro_rules! NX_CPB_FDM { ($c:expr) => { ($c).cpb.hdr.fdm }; }
macro_rules! NX_CPB_KS_DS { ($c:expr) => { ($c).cpb.hdr.ks_ds }; }
macro_rules! NX_CPB_KEY_SIZE { ($c:expr) => { NX_CPB_KS_DS!($c) >> 4 }; }
macro_rules! NX_CPB_SET_KEY_SIZE { ($c:expr, $x:expr) => { NX_CPB_KS_DS!($c) |= ($x) << 4 }; }
macro_rules! NX_CPB_SET_DIGEST_SIZE { ($c:expr, $x:expr) => { NX_CPB_KS_DS!($c) |= $x }; }

#[repr(C, packed)]
struct cop_symcpb_header {
    mode: u8,
    fdm: u8,
    ks_ds: u8,
    pad_byte: u8,
    __rsvd: [u8; 12],
}

#[repr(C)]
union cop_parameter_block_union {
    aes_ecb: cop_symcpb_aes_ecb,
    aes_cbc: cop_symcpb_aes_cbc,
    aes_gca: cop_symcpb_aes_gca,
    aes_gcm: cop_symcpb_aes_gcm,
    aes_cca: cop_symcpb_aes_cca,
    aes_ccm: cop_symcpb_aes_ccm,
    aes_ctr: cop_symcpb_aes_ctr,
    aes_xcbc: cop_symcpb_aes_xcbc,
    sha256: cop_symcpb_sha256,
    sha512: cop_symcpb_sha512,
}

#[repr(C, packed)]
struct cop_parameter_block {
    hdr: cop_symcpb_header,
    data: cop_parameter_block_union,
}

const NX_CSB_VALID_BIT: u32 = 0x80;

/* co-processor status block */
#[repr(C, packed)]
struct cop_status_block {
    valid: u8,
    crb_seq_number: u8,
    completion_code: u8,
    completion_extension: u8,
    processed_byte_count: __be32,
    address: __be64,
}

/* Nest accelerator workbook section 4.4 */
#[repr(C, packed)]
struct nx_csbcpb {
    __rsvd: [u8; 112],
    csb: cop_status_block,
    cpb: cop_parameter_block,
}

/* nx_csbcpb related definitions */
const NX_MODE_AES_ECB: u32 = 0;
const NX_MODE_AES_CBC: u32 = 1;
const NX_MODE_AES_GMAC: u32 = 2;
const NX_MODE_AES_GCA: u32 = 3;
const NX_MODE_AES_GCM: u32 = 4;
const NX_MODE_AES_CCA: u32 = 5;
const NX_MODE_AES_CCM: u32 = 6;
const NX_MODE_AES_CTR: u32 = 7;
const NX_MODE_AES_XCBC_MAC: u32 = 20;
const NX_MODE_SHA: u32 = 0;
const NX_MODE_SHA_HMAC: u32 = 1;
const NX_MODE_AES_CBC_HMAC_ETA: u32 = 8;
const NX_MODE_AES_CBC_HMAC_ATE: u32 = 9;
const NX_MODE_AES_CBC_HMAC_EAA: u32 = 10;
const NX_MODE_AES_CTR_HMAC_ETA: u32 = 12;
const NX_MODE_AES_CTR_HMAC_ATE: u32 = 13;
const NX_MODE_AES_CTR_HMAC_EAA: u32 = 14;

const NX_FDM_CI_FULL: u32 = 0;
const NX_FDM_CI_FIRST: u32 = 1;
const NX_FDM_CI_LAST: u32 = 2;
const NX_FDM_CI_MIDDLE: u32 = 3;

const NX_FDM_PR_NONE: u32 = 0;
const NX_FDM_PR_PAD: u32 = 1;

const NX_KS_AES_128: u32 = 1;
const NX_KS_AES_192: u32 = 2;
const NX_KS_AES_256: u32 = 3;

const NX_DS_SHA256: u32 = 2;
const NX_DS_SHA512: u32 = 3;

const NX_FC_AES: u32 = 0;
const NX_FC_SHA: u32 = 2;
const NX_FC_AES_HMAC: u32 = 6;

const NX_MAX_FC: u32 = NX_FC_AES_HMAC + 1;
const NX_MAX_MODE: u32 = NX_MODE_AES_XCBC_MAC + 1;

const HCOP_FC_AES: u32 = NX_FC_AES;
const HCOP_FC_SHA: u32 = NX_FC_SHA;
const HCOP_FC_AES_HMAC: u32 = NX_FC_AES_HMAC;

/* indices into the array of algorithm properties */
const NX_PROPS_AES_128: u32 = 0;
const NX_PROPS_AES_192: u32 = 1;
const NX_PROPS_AES_256: u32 = 2;
const NX_PROPS_SHA256: u32 = 1;
const NX_PROPS_SHA512: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
