/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::c_char;

/* Vega10+ IH clients */
pub const SOC15_IH_CLIENTID_IH: u32 = 0x00;
pub const SOC15_IH_CLIENTID_ACP: u32 = 0x01;
pub const SOC15_IH_CLIENTID_ATHUB: u32 = 0x02;
pub const SOC15_IH_CLIENTID_BIF: u32 = 0x03;
pub const SOC15_IH_CLIENTID_DCE: u32 = 0x04;
pub const SOC15_IH_CLIENTID_ISP: u32 = 0x05;
pub const SOC15_IH_CLIENTID_PCIE0: u32 = 0x06;
pub const SOC15_IH_CLIENTID_RLC: u32 = 0x07;
pub const SOC15_IH_CLIENTID_SDMA0: u32 = 0x08;
pub const SOC15_IH_CLIENTID_SDMA1: u32 = 0x09;
pub const SOC15_IH_CLIENTID_SE0SH: u32 = 0x0a;
pub const SOC15_IH_CLIENTID_SE1SH: u32 = 0x0b;
pub const SOC15_IH_CLIENTID_SE2SH: u32 = 0x0c;
pub const SOC15_IH_CLIENTID_SE3SH: u32 = 0x0d;
pub const SOC15_IH_CLIENTID_UVD1: u32 = 0x0e;
pub const SOC15_IH_CLIENTID_THM: u32 = 0x0f;
pub const SOC15_IH_CLIENTID_UVD: u32 = 0x10;
pub const SOC15_IH_CLIENTID_VCE0: u32 = 0x11;
pub const SOC15_IH_CLIENTID_VMC: u32 = 0x12;
pub const SOC15_IH_CLIENTID_XDMA: u32 = 0x13;
pub const SOC15_IH_CLIENTID_GRBM_CP: u32 = 0x14;
pub const SOC15_IH_CLIENTID_ATS: u32 = 0x15;
pub const SOC15_IH_CLIENTID_ROM_SMUIO: u32 = 0x16;
pub const SOC15_IH_CLIENTID_DF: u32 = 0x17;
pub const SOC15_IH_CLIENTID_VCE1: u32 = 0x18;
pub const SOC15_IH_CLIENTID_PWR: u32 = 0x19;
pub const SOC15_IH_CLIENTID_RESERVED: u32 = 0x1a;
pub const SOC15_IH_CLIENTID_UTCL2: u32 = 0x1b;
pub const SOC15_IH_CLIENTID_EA: u32 = 0x1c;
pub const SOC15_IH_CLIENTID_UTCL2LOG: u32 = 0x1d;
pub const SOC15_IH_CLIENTID_MP0: u32 = 0x1e;
pub const SOC15_IH_CLIENTID_MP1: u32 = 0x1f;
pub const SOC15_IH_CLIENTID_MAX: u32 = 0x20;
pub const SOC15_IH_CLIENTID_VCN: u32 = SOC15_IH_CLIENTID_UVD;
pub const SOC15_IH_CLIENTID_VCN1: u32 = SOC15_IH_CLIENTID_UVD1;
pub const SOC15_IH_CLIENTID_SDMA2: u32 = SOC15_IH_CLIENTID_ACP;
pub const SOC15_IH_CLIENTID_SDMA3: u32 = SOC15_IH_CLIENTID_DCE;
pub const SOC15_IH_CLIENTID_SDMA3_Sienna_Cichlid: u32 = SOC15_IH_CLIENTID_ISP;
pub const SOC15_IH_CLIENTID_SDMA4: u32 = SOC15_IH_CLIENTID_ISP;
pub const SOC15_IH_CLIENTID_SDMA5: u32 = SOC15_IH_CLIENTID_VCE0;
pub const SOC15_IH_CLIENTID_SDMA6: u32 = SOC15_IH_CLIENTID_XDMA;
pub const SOC15_IH_CLIENTID_SDMA7: u32 = SOC15_IH_CLIENTID_VCE1;
pub const SOC15_IH_CLIENTID_VMC1: u32 = SOC15_IH_CLIENTID_PCIE0;

extern "C" {
    pub static mut soc15_ih_clientid_name: [*const c_char; 0];
}

/* soc21 IH clients */
pub const SOC21_IH_CLIENTID_IH: u32 = 0x00;
pub const SOC21_IH_CLIENTID_ATHUB: u32 = 0x02;
pub const SOC21_IH_CLIENTID_BIF: u32 = 0x03;
pub const SOC21_IH_CLIENTID_DCN: u32 = 0x04;
pub const SOC21_IH_CLIENTID_ISP: u32 = 0x05;
pub const SOC21_IH_CLIENTID_MP3: u32 = 0x06;
pub const SOC21_IH_CLIENTID_RLC: u32 = 0x07;
pub const SOC21_IH_CLIENTID_GFX: u32 = 0x0a;
pub const SOC21_IH_CLIENTID_IMU: u32 = 0x0b;
pub const SOC21_IH_CLIENTID_VCN1: u32 = 0x0e;
pub const SOC21_IH_CLIENTID_THM: u32 = 0x0f;
pub const SOC21_IH_CLIENTID_VCN: u32 = 0x10;
pub const SOC21_IH_CLIENTID_VPE1: u32 = 0x11;
pub const SOC21_IH_CLIENTID_VMC: u32 = 0x12;
pub const SOC21_IH_CLIENTID_GRBM_CP: u32 = 0x14;
pub const SOC21_IH_CLIENTID_ROM_SMUIO: u32 = 0x16;
pub const SOC21_IH_CLIENTID_DF: u32 = 0x17;
pub const SOC21_IH_CLIENTID_VPE: u32 = 0x18;
pub const SOC21_IH_CLIENTID_PWR: u32 = 0x19;
pub const SOC21_IH_CLIENTID_LSDMA: u32 = 0x1a;
pub const SOC21_IH_CLIENTID_UTCL2: u32 = 0x1b;
pub const SOC21_IH_CLIENTID_MP0: u32 = 0x1e;
pub const SOC21_IH_CLIENTID_MP1: u32 = 0x1f;
pub const SOC21_IH_CLIENTID_MAX: u32 = 0x20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
