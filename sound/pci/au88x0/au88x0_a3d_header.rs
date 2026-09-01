/* SPDX-License-Identifier: GPL-2.0-or-later */
/***************************************************************************
 *            au88x0_a3d.h
 *
 *  Fri Jul 18 14:16:03 2003
 *  Copyright  2003  mjander
 *  mjander@users.sourceforge.net
 ****************************************************************************/

/*
 */

// C header dependency note: the original header had a disabled include of
// <openal.h>.

pub const HRTF_SZ: usize = 0x38;
pub const DLINE_SZ: usize = 0x28;

pub const CTRLID_HRTF: u32 = 1;
pub const CTRLID_ITD: u32 = 2;
pub const CTRLID_ILD: u32 = 4;
pub const CTRLID_FILTER: u32 = 8;
pub const CTRLID_GAINS: u32 = 16;

/* 3D parameter structs */
pub type a3d_Hrtf_t = [u16; HRTF_SZ];
pub type a3d_ItdDline_t = [u16; DLINE_SZ];
pub type a3d_atmos_t = [u16; 5];
pub type a3d_LRGains_t = [u16; 2];
pub type a3d_Itd_t = [u16; 2];
pub type a3d_Ild_t = [u16; 2];

#[repr(C)]
pub struct a3dsrc_t {
    pub vortex: *mut core::ffi::c_void, /* Formerly CAsp4HwIO*, now vortex_t*. */
    pub source: u32,                    /* this_04 */
    pub slice: u32,                     /* this_08 */
    pub hrtf: [a3d_Hrtf_t; 2],
    pub itd: a3d_Itd_t,
    pub ild: a3d_Ild_t,
    pub dline: a3d_ItdDline_t,
    pub filter: a3d_atmos_t,
}

/* First Register bank */

pub const A3D_A_HrtfCurrent: u32 = 0x18000; /* 56 ULONG */
pub const A3D_A_GainCurrent: u32 = 0x180E0;
pub const A3D_A_GainTarget: u32 = 0x180E4;
pub const A3D_A_A12Current: u32 = 0x180E8; /* Atmospheric current. */
pub const A3D_A_A21Target: u32 = 0x180EC; /* Atmospheric target */
pub const A3D_A_B01Current: u32 = 0x180F0; /* Atmospheric current */
pub const A3D_A_B10Target: u32 = 0x180F4; /* Atmospheric target */
pub const A3D_A_B2Current: u32 = 0x180F8; /* Atmospheric current */
pub const A3D_A_B2Target: u32 = 0x180FC; /* Atmospheric target */
pub const A3D_A_HrtfTarget: u32 = 0x18100; /* 56 ULONG */
pub const A3D_A_ITDCurrent: u32 = 0x181E0;
pub const A3D_A_ITDTarget: u32 = 0x181E4;
pub const A3D_A_HrtfDelayLine: u32 = 0x181E8; /* 56 ULONG */
pub const A3D_A_ITDDelayLine: u32 = 0x182C8; /* 40/45 ULONG */
pub const A3D_A_HrtfTrackTC: u32 = 0x1837C; /* Time Constants */
pub const A3D_A_GainTrackTC: u32 = 0x18380;
pub const A3D_A_CoeffTrackTC: u32 = 0x18384;
pub const A3D_A_ITDTrackTC: u32 = 0x18388;
pub const A3D_A_x1: u32 = 0x1838C;
pub const A3D_A_x2: u32 = 0x18390;
pub const A3D_A_y1: u32 = 0x18394;
pub const A3D_A_y2: u32 = 0x18398;
pub const A3D_A_HrtfOutL: u32 = 0x1839C;
pub const A3D_A_HrtfOutR: u32 = 0x183A0;
pub const A3D_A_TAIL: u32 = 0x183A4;

/* Second register bank */
pub const A3D_B_HrtfCurrent: u32 = 0x19000; /* 56 ULONG */
pub const A3D_B_GainCurrent: u32 = 0x190E0;
pub const A3D_B_GainTarget: u32 = 0x190E4;
pub const A3D_B_A12Current: u32 = 0x190E8;
pub const A3D_B_A21Target: u32 = 0x190EC;
pub const A3D_B_B01Current: u32 = 0x190F0;
pub const A3D_B_B10Target: u32 = 0x190F4;
pub const A3D_B_B2Current: u32 = 0x190F8;
pub const A3D_B_B2Target: u32 = 0x190FC;
pub const A3D_B_HrtfTarget: u32 = 0x19100; /* 56 ULONG */
pub const A3D_B_ITDCurrent: u32 = 0x191E0;
pub const A3D_B_ITDTarget: u32 = 0x191E4;
pub const A3D_B_HrtfDelayLine: u32 = 0x191E8; /* 56 ULONG */
pub const A3D_B_TAIL: u32 = 0x192C8;

/* There are 4 slices, 4 a3d each = 16 a3d sources. */
pub const A3D_SLICE_BANK_A: u32 = 0x18000; /* 4 sources */
pub const A3D_SLICE_BANK_B: u32 = 0x19000; /* 4 sources */
pub const A3D_SLICE_VDBDest: u32 = 0x19C00; /* 8 ULONG */
pub const A3D_SLICE_VDBSource: u32 = 0x19C20; /* 4 ULONG */
pub const A3D_SLICE_ABReg: u32 = 0x19C30;
pub const A3D_SLICE_CReg: u32 = 0x19C34;
pub const A3D_SLICE_Control: u32 = 0x19C38;
pub const A3D_SLICE_DebugReserved: u32 = 0x19C3c; /* Dangerous! */
pub const A3D_SLICE_Pointers: u32 = 0x19C40;
pub const A3D_SLICE_TAIL: u32 = 0x1A000;

// Slice size: 0x2000
// Source size: 0x3A4, 0x2C8

/* Address generator macro. */
pub const fn a3d_addrA(slice: u32, source: u32, reg: u32) -> u32 {
    (slice << 0xd)
        .wrapping_add(source.wrapping_mul(0x3A4))
        .wrapping_add(reg)
}

pub const fn a3d_addrB(slice: u32, source: u32, reg: u32) -> u32 {
    (slice << 0xd)
        .wrapping_add(source.wrapping_mul(0x2C8))
        .wrapping_add(reg)
}

pub const fn a3d_addrS(slice: u32, reg: u32) -> u32 {
    (slice << 0xd).wrapping_add(reg)
}

// Original disabled macro:
// #define a3d_addr(slice,source,reg) (((reg)>=0x19000) ? a3d_addr2((slice),(source),(reg)) : a3d_addr1((slice),(source),(reg)))

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
