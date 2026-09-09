/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2007 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

/*
 * Protocol definitions from RFC 3643 - Fibre Channel Frame Encapsulation.
 *
 * The frame length field is the number of 32-bit words in the encapsulation
 * including the header, CRC and EOF words.
 */
pub const FC_ENCAPS_MIN_FRAME_LEN: u32 = 64;
pub const FC_ENCAPS_MAX_FRAME_LEN: u32 = FC_ENCAPS_MIN_FRAME_LEN + FC_MAX_PAYLOAD;
pub const FC_ENCAPS_VER: u8 = 1;

#[repr(C)]
pub struct fc_encaps_hdr {
    pub fc_proto: __u8,
    pub fc_ver: __u8,
    pub fc_proto_n: __u8,
    pub fc_ver_n: __u8,
    pub fc_proto_data: [u8; 8],
    pub fc_len_flags: __be16,
    pub fc_len_flags_n: __be16,
    pub fc_time: [__be32; 2],
    pub fc_crc: __be32,
    pub fc_sof: __be32,
}

pub const FCIP_ENCAPS_HDR_LEN: usize = 0x20;

#[inline]
pub const fn FC_XY(x: u32, y: u32) -> u32 {
    (((x & 0xff) << 8) | (y & 0xff))
}

/* The C source spells these invocations FCIP_XY; the intended local macro is FC_XY. */
#[inline]
pub const fn FC_XYXY(x: u32, y: u32) -> u32 {
    (FC_XY(x, y) << 16) | FC_XY(x, y)
}

#[inline]
pub const fn FC_XYNN(x: u32, y: u32) -> u32 {
    FC_XYXY(x, y) ^ 0xffff
}

#[inline]
pub const fn FC_SOF_ENCODE(n: u32) -> u32 {
    FC_XYNN(n, n)
}

#[inline]
pub const fn FC_EOF_ENCODE(n: u32) -> u32 {
    FC_XYNN(n, n)
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_sof {
    FC_SOF_F = 0x28,
    FC_SOF_I4 = 0x29,
    FC_SOF_I2 = 0x2d,
    FC_SOF_I3 = 0x2e,
    FC_SOF_N4 = 0x31,
    FC_SOF_N2 = 0x35,
    FC_SOF_N3 = 0x36,
    FC_SOF_C4 = 0x39,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_eof {
    FC_EOF_N = 0x41,
    FC_EOF_T = 0x42,
    FC_EOF_RT = 0x44,
    FC_EOF_DT = 0x46,
    FC_EOF_NI = 0x49,
    FC_EOF_DTI = 0x4e,
    FC_EOF_RTI = 0x4f,
    FC_EOF_A = 0x50,
}

pub const FC_SOF_CLASS_MASK: u8 = 0x06;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_class {
    FC_CLASS_NONE = 0,
    FC_CLASS_2 = fc_sof::FC_SOF_I2 as u8,
    FC_CLASS_3 = fc_sof::FC_SOF_I3 as u8,
    FC_CLASS_4 = fc_sof::FC_SOF_I4 as u8,
    FC_CLASS_F = fc_sof::FC_SOF_F as u8,
}

#[inline]
pub const fn fc_sof_needs_ack(sof: fc_sof) -> i32 {
    ((!((sof as u8) as i32)) & 0x02)
}

#[inline]
pub const fn fc_sof_normal(class: fc_class) -> fc_sof {
    unsafe { core::mem::transmute((class as u8 + 0x36 - 0x2e) as u8) }
}

#[inline]
pub const fn fc_sof_class(sof: fc_sof) -> fc_class {
    unsafe { core::mem::transmute((((sof as u8) & 0x7) | 0x28) as u8) }
}

#[inline]
pub const fn fc_sof_is_init(sof: fc_sof) -> bool {
    (sof as u8) < 0x30
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
