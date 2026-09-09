// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bestcomm GenBD RX task microcode
 *
 * Copyright (C) 2006 AppSpec Computer Technologies Corp.
 *                    Jeff Gibbons <jeff.gibbons@appspec.com>
 * Copyright (c) 2004 Freescale Semiconductor, Inc.
 *
 * Based on BestCommAPI-2.2/code_dma/image_rtos1/dma_image.hex
 * on Tue Mar 4 10:14:12 2006 GMT
 */

// The header consists of the following fields:
//     u32 magic;
//     u8  desc_size;
//     u8  var_size;
//     u8  inc_size;
//     u8  first_var;
//     u8  reserved[8];
//
// The size fields contain the number of 32-bit words.

pub static mut bcom_gen_bd_rx_task: [u32; 25] = [
    // header
    0x4243_544b,
    0x0d02_0409,
    0x0000_0000,
    0x0000_0000,

    // Task descriptors
    0x8082_20da, // LCD: idx0 = var1, idx1 = var4; idx1 <= var3; idx0 += inc3, idx1 += inc2
    0x13e0_1010, //   DRD1A: var4 = var2; FN=0 MORE init=31 WS=0 RS=0
    0xb880_025b, //   LCD: idx2 = *idx1, idx3 = var0; idx2 < var9; idx2 += inc3, idx3 += inc3
    0x1000_1308, //     DRD1A: var4 = idx1; FN=0 MORE init=0 WS=0 RS=0
    0x6014_0002, //     DRD2A: EU0=0 EU1=0 EU2=0 EU3=2 EXT init=0 WS=2 RS=2
    0x0ccc_fcca, //     DRD2B1: *idx3 = EU3(); EU3(*idx3,var10)
    0xd919_0240, //   LCDEXT: idx2 = idx2; idx2 > var9; idx2 += inc0
    0xb8c5_e009, //   LCD: idx3 = *(idx1 + var00000015); ; idx3 += inc1
    0x07fe_cf80, //     DRD1A: *idx3 = *idx0; FN=0 INT init=31 WS=3 RS=3
    0x9919_0024, //   LCD: idx2 = idx2; idx2 once var0; idx2 += inc4
    0x6000_0005, //     DRD2A: EU0=0 EU1=0 EU2=0 EU3=5 EXT init=0 WS=0 RS=0
    0x0c4c_f889, //     DRD2B1: *idx1 = EU3(); EU3(idx2,var9)
    0x0000_01f8, //   NOP

    // VAR[9]-VAR[10]
    0x4000_0000,
    0x7fff_7fff,

    // INC[0]-INC[3]
    0x4000_0000,
    0xe000_0000,
    0xa000_0008,
    0x2000_0000,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
