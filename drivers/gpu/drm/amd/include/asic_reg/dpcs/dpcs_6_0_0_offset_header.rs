/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.
 */

// addressBlock: dpcssys_dcio_dcio_dispdec
// base address: 0x0
pub const regHPD_CTRL: u32 = 0x286c;
pub const regHPD_CTRL_BASE_IDX: u32 = 2;
pub const regDC_PINSTRAPS: u32 = 0x2880;
pub const regDC_PINSTRAPS_BASE_IDX: u32 = 2;

// addressBlock: dpcssys_dcio_dcio_chip_dispdec
// base address: 0x0
pub const regDC_GPIO_DDC1_MASK: u32 = 0x28d0;
pub const regDC_GPIO_DDC1_MASK_BASE_IDX: u32 = 2;
pub const regDC_GPIO_DDC2_MASK: u32 = 0x28d4;
pub const regDC_GPIO_DDC2_MASK_BASE_IDX: u32 = 2;
pub const regDC_GPIO_DDC3_MASK: u32 = 0x28d8;
pub const regDC_GPIO_DDC3_MASK_BASE_IDX: u32 = 2;
pub const regDC_GPIO_DDC4_MASK: u32 = 0x28dc;
pub const regDC_GPIO_DDC4_MASK_BASE_IDX: u32 = 2;
pub const regPHY_AUX_CNTL: u32 = 0x28ff;
pub const regPHY_AUX_CNTL_BASE_IDX: u32 = 2;
pub const regDC_GPIO_AUX_CTRL_5: u32 = 0x291d;
pub const regDC_GPIO_AUX_CTRL_5_BASE_IDX: u32 = 2;

// addressBlock: dpcssys_dcio_i3c_pad_control_ddc1_dc_i3c_dispdec
// base address: 0x0
pub const regDC_I3C0_DC_I3CPAD_CONTROL0: u32 = 0x2f6c;
pub const regDC_I3C0_DC_I3CPAD_CONTROL0_BASE_IDX: u32 = 2;
pub const regDC_I3C0_DC_I3CPAD_CONTROL1: u32 = 0x2f6d;
pub const regDC_I3C0_DC_I3CPAD_CONTROL1_BASE_IDX: u32 = 2;

// addressBlock: dpcssys_dcio_i3c_pad_control_ddc2_dc_i3c_dispdec
// base address: 0x8
pub const regDC_I3C1_DC_I3CPAD_CONTROL0: u32 = 0x2f6e;
pub const regDC_I3C1_DC_I3CPAD_CONTROL0_BASE_IDX: u32 = 2;
pub const regDC_I3C1_DC_I3CPAD_CONTROL1: u32 = 0x2f6f;
pub const regDC_I3C1_DC_I3CPAD_CONTROL1_BASE_IDX: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
