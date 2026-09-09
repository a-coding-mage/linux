/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2004-2007 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 */

/* Register offsets */
pub const MXC_IIMSTAT: u32 = 0x0000;
pub const MXC_IIMSTATM: u32 = 0x0004;
pub const MXC_IIMERR: u32 = 0x0008;
pub const MXC_IIMEMASK: u32 = 0x000C;
pub const MXC_IIMFCTL: u32 = 0x0010;
pub const MXC_IIMUA: u32 = 0x0014;
pub const MXC_IIMLA: u32 = 0x0018;
pub const MXC_IIMSDAT: u32 = 0x001C;
pub const MXC_IIMPREV: u32 = 0x0020;
pub const MXC_IIMSREV: u32 = 0x0024;
pub const MXC_IIMPRG_P: u32 = 0x0028;
pub const MXC_IIMSCS0: u32 = 0x002C;
pub const MXC_IIMSCS1: u32 = 0x0030;
pub const MXC_IIMSCS2: u32 = 0x0034;
pub const MXC_IIMSCS3: u32 = 0x0038;
pub const MXC_IIMFBAC0: u32 = 0x0800;
pub const MXC_IIMJAC: u32 = 0x0804;
pub const MXC_IIMHWV1: u32 = 0x0808;
pub const MXC_IIMHWV2: u32 = 0x080C;
pub const MXC_IIMHAB0: u32 = 0x0810;
pub const MXC_IIMHAB1: u32 = 0x0814;
/* Definitions for i.MX27 TO2 */
pub const MXC_IIMMAC: u32 = 0x0814;
pub const MXC_IIMPREV_FUSE: u32 = 0x0818;
pub const MXC_IIMSREV_FUSE: u32 = 0x081C;
pub const MXC_IIMSJC_CHALL_0: u32 = 0x0820;
pub const MXC_IIMSJC_CHALL_7: u32 = 0x083C;
pub const MXC_IIMFB0UC17: u32 = 0x0840;
pub const MXC_IIMFB0UC255: u32 = 0x0BFC;
pub const MXC_IIMFBAC1: u32 = 0x0C00;
/* Definitions for i.MX27 TO2 */
pub const MXC_IIMSUID: u32 = 0x0C04;
pub const MXC_IIMKEY0: u32 = 0x0C04;
pub const MXC_IIMKEY20: u32 = 0x0C54;
pub const MXC_IIMSJC_RESP_0: u32 = 0x0C58;
pub const MXC_IIMSJC_RESP_7: u32 = 0x0C74;
pub const MXC_IIMFB1UC30: u32 = 0x0C78;
pub const MXC_IIMFB1UC255: u32 = 0x0FFC;

/* Bit definitions */
pub const MXC_IIMHWV1_WLOCK: u32 = 0x1 << 7;
pub const MXC_IIMHWV1_MCU_ENDIAN: u32 = 0x1 << 6;
pub const MXC_IIMHWV1_DSP_ENDIAN: u32 = 0x1 << 5;
pub const MXC_IIMHWV1_BOOT_INT: u32 = 0x1 << 4;
pub const MXC_IIMHWV1_SCC_DISABLE: u32 = 0x1 << 3;
pub const MXC_IIMHWV1_HANTRO_DISABLE: u32 = 0x1 << 2;
pub const MXC_IIMHWV1_MEMSTICK_DIS: u32 = 0x1 << 1;

pub const MXC_IIMHWV2_WLOCK: u32 = 0x1 << 7;
pub const MXC_IIMHWV2_BP_SDMA: u32 = 0x1 << 6;
pub const MXC_IIMHWV2_SCM_DCM: u32 = 0x1 << 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
