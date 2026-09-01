/* SPDX-License-Identifier: GPL-2.0 */
/*
 * HiSilicon PCIe Trace and Tuning (PTT) support
 * Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
 */

// C header dependencies: <stddef.h>, <stdint.h>, <linux/bits.h>,
// and <linux/bitfield.h>.

pub const HISI_PTT_8DW_CHECK_MASK: u32 = (((!0u32) - ((1u32 << 11) - 1)) & (!0u32 >> (31 - 31)));
pub const HISI_PTT_IS_8DW_PKT: u32 = (((!0u32) - ((1u32 << 11) - 1)) & (!0u32 >> (31 - 31)));
pub const HISI_PTT_MAX_SPACE_LEN: i32 = 10;
pub const HISI_PTT_FIELD_LENGTH: i32 = 4;

/* Header DW0 fields for 4DW format */
pub const HISI_PTT_HEAD0_4DW_TIME: u32 = (((!0u32) - ((1u32 << 0) - 1)) & (!0u32 >> (31 - 10)));
pub const HISI_PTT_HEAD0_4DW_LEN: u32 = (((!0u32) - ((1u32 << 11) - 1)) & (!0u32 >> (31 - 20)));
pub const HISI_PTT_HEAD0_4DW_SO: u32 = 1u32 << 21;
pub const HISI_PTT_HEAD0_4DW_TH: u32 = 1u32 << 22;
pub const HISI_PTT_HEAD0_4DW_T8: u32 = 1u32 << 23;
pub const HISI_PTT_HEAD0_4DW_T9: u32 = 1u32 << 24;
pub const HISI_PTT_HEAD0_4DW_TYPE: u32 = (((!0u32) - ((1u32 << 25) - 1)) & (!0u32 >> (31 - 29)));
pub const HISI_PTT_HEAD0_4DW_FORMAT: u32 = (((!0u32) - ((1u32 << 30) - 1)) & (!0u32 >> (31 - 31)));

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hisi_ptt_pkt_type {
    HISI_PTT_4DW_PKT = 0,
    HISI_PTT_8DW_PKT = 1,
    HISI_PTT_PKT_MAX = 2,
}

pub static hisi_ptt_pkt_size: [i32; hisi_ptt_pkt_type::HISI_PTT_PKT_MAX as usize] = {
    let mut sizes = [0; hisi_ptt_pkt_type::HISI_PTT_PKT_MAX as usize];
    sizes[hisi_ptt_pkt_type::HISI_PTT_4DW_PKT as usize] = 16;
    sizes[hisi_ptt_pkt_type::HISI_PTT_8DW_PKT as usize] = 32;
    sizes
};

unsafe extern "C" {
    pub fn hisi_ptt_pkt_desc(
        buf: *const ::std::os::raw::c_uchar,
        pos: ::std::os::raw::c_int,
        type_: hisi_ptt_pkt_type,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
