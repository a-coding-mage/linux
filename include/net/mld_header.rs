/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding Linux translation. */

#[repr(C)]
pub struct mld_msg {
    pub mld_hdr: icmp6hdr,
    pub mld_mca: in6_addr,
}

/* mld_type, mld_code, mld_cksum, mld_maxdelay, and mld_reserved are
 * projections into mld_hdr, as in the original C preprocessor aliases. */

#[repr(C)]
pub struct mld2_grec {
    pub grec_type: u8,
    pub grec_auxwords: u8,
    pub grec_nsrcs: u16,
    pub grec_mca: in6_addr,
    pub grec_src: [in6_addr; 0],
}

#[repr(C)]
pub struct mld2_report {
    pub mld2r_hdr: icmp6hdr,
    pub mld2r_grec: [mld2_grec; 0],
}

/* mld2r_type, mld2r_resv1, mld2r_cksum, mld2r_resv2, and mld2r_ngrec
 * are projections into mld2r_hdr, as in the original C aliases. */

#[repr(C)]
pub struct mld2_query {
    pub mld2q_hdr: icmp6hdr,
    pub mld2q_mca: in6_addr,
    /* __LITTLE_ENDIAN_BITFIELD: qrv:3, suppress:1, resv2:4;
     * __BIG_ENDIAN_BITFIELD: resv2:4, suppress:1, qrv:3. */
    pub mld2q_qrv_suppress_resv2: u8,
    pub mld2q_qqic: u8,
    pub mld2q_nsrcs: u16,
    pub mld2q_srcs: [in6_addr; 0],
}

/* mld2q_type, mld2q_code, mld2q_cksum, mld2q_mrc, and mld2q_resv1
 * are projections into mld2q_hdr, as in the original C aliases. */

#[inline]
pub const fn MLDV2_MRC_EXP(value: u16) -> u16 { (value >> 12) & 0x0007 }
#[inline]
pub const fn MLDV2_MRC_MAN(value: u16) -> u16 { value & 0x0fff }
#[inline]
pub const fn MLDV2_QQIC_EXP(value: u8) -> u8 { (value >> 4) & 0x07 }
#[inline]
pub const fn MLDV2_QQIC_MAN(value: u8) -> u8 { value & 0x0f }

pub const MLD_QQIC_MIN_THRESHOLD: u8 = 128;
pub const MLD_QQIC_MAX_THRESHOLD: u64 = 31744;
pub const MLD_MRC_MIN_THRESHOLD: u64 = 32768;
pub const MLD_MRC_MAX_THRESHOLD: u64 = 8387584;
pub const MLDV1_MRD_MAX_COMPAT: u64 = MLD_MRC_MIN_THRESHOLD - 1;
pub const MLD_MAX_QUEUE: u32 = 8;
pub const MLD_MAX_SKBS: u32 = 32;

extern "C" {
    fn fls(x: u64) -> i32;
    fn ntohs(x: u16) -> u16;
}

#[inline]
pub unsafe fn mldv2_mrc(mrd: u64) -> u16 {
    if mrd < MLD_MRC_MIN_THRESHOLD { return mrd as u16; }
    if mrd >= MLD_MRC_MAX_THRESHOLD { return 0xFFFF; }
    let mc_exp = (fls(mrd) - 16) as u16;
    let mc_man = ((mrd >> (mc_exp + 3)) & 0x0FFF) as u16;
    0x8000 | (mc_exp << 12) | mc_man
}

#[inline]
pub unsafe fn mldv2_qqic(value: u64) -> u8 {
    if value < MLD_QQIC_MIN_THRESHOLD as u64 { return value as u8; }
    if value >= MLD_QQIC_MAX_THRESHOLD { return 0xFF; }
    let mc_exp = (fls(value) - 8) as u8;
    let mc_man = ((value >> (mc_exp + 3)) & 0x0F) as u8;
    0x80 | (mc_exp << 4) | mc_man
}

#[inline]
pub unsafe fn mldv2_mrd(mlh2: *const mld2_query) -> u64 {
    let mc_mrc = ntohs((*mlh2).mld2q_hdr.icmp6_maxdelay);
    if (mc_mrc as u64) < MLD_MRC_MIN_THRESHOLD {
        mc_mrc as u64
    } else {
        let mc_exp = MLDV2_MRC_EXP(mc_mrc) as u32;
        let mc_man = MLDV2_MRC_MAN(mc_mrc) as u64;
        (mc_man | 0x1000) << (mc_exp + 3)
    }
}

#[inline]
pub unsafe fn mldv2_qqi(mlh2: *const mld2_query) -> u64 {
    let qqic = (*mlh2).mld2q_qqic;
    if qqic < MLD_QQIC_MIN_THRESHOLD {
        qqic as u64
    } else {
        let mc_exp = MLDV2_QQIC_EXP(qqic) as u32;
        let mc_man = MLDV2_QQIC_MAN(qqic) as u64;
        (mc_man | 0x10) << (mc_exp + 3)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
