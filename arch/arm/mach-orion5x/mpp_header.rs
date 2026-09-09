/* SPDX-License-Identifier: GPL-2.0 */

pub const fn MPP(
    num: u32,
    sel: u32,
    input: u32,
    output: u32,
    f5181l: u32,
    f5182: u32,
    f5281: u32,
) -> u32 {
    (num & 0xff)
        | ((sel & 0xf) << 8)
        | (((input != 0) as u32) << 12)
        | (((output != 0) as u32) << 13)
        | (((f5181l != 0) as u32) << 14)
        | (((f5182 != 0) as u32) << 15)
        | (((f5281 != 0) as u32) << 16)
}

pub const MPP_F5181_MASK: u32 = MPP(0, 0x0, 0, 0, 1, 0, 0);
pub const MPP_F5182_MASK: u32 = MPP(0, 0x0, 0, 0, 0, 1, 0);
pub const MPP_F5281_MASK: u32 = MPP(0, 0x0, 0, 0, 0, 0, 1);

pub const MPP0_UNUSED: u32 = MPP(0, 0x3, 0, 0, 1, 1, 1);
pub const MPP0_GPIO: u32 = MPP(0, 0x3, 1, 1, 1, 1, 1);
pub const MPP0_PCIE_RST_OUTn: u32 = MPP(0, 0x0, 0, 0, 1, 1, 1);
pub const MPP0_PCI_ARB: u32 = MPP(0, 0x2, 0, 0, 1, 1, 1);

pub const MPP1_UNUSED: u32 = MPP(1, 0x0, 0, 0, 1, 1, 1);
pub const MPP1_GPIO: u32 = MPP(1, 0x0, 1, 1, 1, 1, 1);
pub const MPP1_PCI_ARB: u32 = MPP(1, 0x2, 0, 0, 1, 1, 1);

pub const MPP2_UNUSED: u32 = MPP(2, 0x0, 0, 0, 1, 1, 1);
pub const MPP2_GPIO: u32 = MPP(2, 0x0, 1, 1, 1, 1, 1);
pub const MPP2_PCI_ARB: u32 = MPP(2, 0x2, 0, 0, 1, 1, 1);
pub const MPP2_PCI_PMEn: u32 = MPP(2, 0x3, 0, 0, 1, 1, 1);

pub const MPP3_UNUSED: u32 = MPP(3, 0x0, 0, 0, 1, 1, 1);
pub const MPP3_GPIO: u32 = MPP(3, 0x0, 1, 1, 1, 1, 1);
pub const MPP3_PCI_ARB: u32 = MPP(3, 0x2, 0, 0, 1, 1, 1);

pub const MPP4_UNUSED: u32 = MPP(4, 0x0, 0, 0, 1, 1, 1);
pub const MPP4_GPIO: u32 = MPP(4, 0x0, 1, 1, 1, 1, 1);
pub const MPP4_PCI_ARB: u32 = MPP(4, 0x2, 0, 0, 1, 1, 1);
pub const MPP4_NAND: u32 = MPP(4, 0x4, 0, 0, 0, 1, 1);
pub const MPP4_SATA_LED: u32 = MPP(4, 0x5, 0, 0, 0, 1, 0);

pub const MPP5_UNUSED: u32 = MPP(5, 0x0, 0, 0, 1, 1, 1);
pub const MPP5_GPIO: u32 = MPP(5, 0x0, 1, 1, 1, 1, 1);
pub const MPP5_PCI_ARB: u32 = MPP(5, 0x2, 0, 0, 1, 1, 1);
pub const MPP5_NAND: u32 = MPP(5, 0x4, 0, 0, 0, 1, 1);
pub const MPP5_SATA_LED: u32 = MPP(5, 0x5, 0, 0, 0, 1, 0);

pub const MPP6_UNUSED: u32 = MPP(6, 0x0, 0, 0, 1, 1, 1);
pub const MPP6_GPIO: u32 = MPP(6, 0x0, 1, 1, 1, 1, 1);
pub const MPP6_PCI_ARB: u32 = MPP(6, 0x2, 0, 0, 1, 1, 1);
pub const MPP6_NAND: u32 = MPP(6, 0x4, 0, 0, 0, 1, 1);
pub const MPP6_PCI_CLK: u32 = MPP(6, 0x5, 0, 0, 1, 0, 0);
pub const MPP6_SATA_LED: u32 = MPP(6, 0x5, 0, 0, 0, 1, 0);

pub const MPP7_UNUSED: u32 = MPP(7, 0x0, 0, 0, 1, 1, 1);
pub const MPP7_GPIO: u32 = MPP(7, 0x0, 1, 1, 1, 1, 1);
pub const MPP7_PCI_ARB: u32 = MPP(7, 0x2, 0, 0, 1, 1, 1);
pub const MPP7_NAND: u32 = MPP(7, 0x4, 0, 0, 0, 1, 1);
pub const MPP7_PCI_CLK: u32 = MPP(7, 0x5, 0, 0, 1, 0, 0);
pub const MPP7_SATA_LED: u32 = MPP(7, 0x5, 0, 0, 0, 1, 0);

pub const MPP8_UNUSED: u32 = MPP(8, 0x0, 0, 0, 1, 1, 1);
pub const MPP8_GPIO: u32 = MPP(8, 0x0, 1, 1, 1, 1, 1);
pub const MPP8_GIGE: u32 = MPP(8, 0x1, 0, 0, 1, 1, 1);

pub const MPP9_UNUSED: u32 = MPP(9, 0x0, 0, 0, 1, 1, 1);
pub const MPP9_GPIO: u32 = MPP(9, 0x0, 1, 1, 1, 1, 1);
pub const MPP9_GIGE: u32 = MPP(9, 0x1, 0, 0, 1, 1, 1);

pub const MPP10_UNUSED: u32 = MPP(10, 0x0, 0, 0, 1, 1, 1);
pub const MPP10_GPIO: u32 = MPP(10, 0x0, 1, 1, 1, 1, 1);
pub const MPP10_GIGE: u32 = MPP(10, 0x1, 0, 0, 1, 1, 1);

pub const MPP11_UNUSED: u32 = MPP(11, 0x0, 0, 0, 1, 1, 1);
pub const MPP11_GPIO: u32 = MPP(11, 0x0, 1, 1, 1, 1, 1);
pub const MPP11_GIGE: u32 = MPP(11, 0x1, 0, 0, 1, 1, 1);

pub const MPP12_UNUSED: u32 = MPP(12, 0x0, 0, 0, 1, 1, 1);
pub const MPP12_GPIO: u32 = MPP(12, 0x0, 1, 1, 1, 1, 1);
pub const MPP12_GIGE: u32 = MPP(12, 0x1, 0, 0, 1, 1, 1);
pub const MPP12_NAND: u32 = MPP(12, 0x4, 0, 0, 0, 1, 1);
pub const MPP12_SATA_LED: u32 = MPP(12, 0x5, 0, 0, 0, 1, 0);

pub const MPP13_UNUSED: u32 = MPP(13, 0x0, 0, 0, 1, 1, 1);
pub const MPP13_GPIO: u32 = MPP(13, 0x0, 1, 1, 1, 1, 1);
pub const MPP13_GIGE: u32 = MPP(13, 0x1, 0, 0, 1, 1, 1);
pub const MPP13_NAND: u32 = MPP(13, 0x4, 0, 0, 0, 1, 1);
pub const MPP13_SATA_LED: u32 = MPP(13, 0x5, 0, 0, 0, 1, 0);

pub const MPP14_UNUSED: u32 = MPP(14, 0x0, 0, 0, 1, 1, 1);
pub const MPP14_GPIO: u32 = MPP(14, 0x0, 1, 1, 1, 1, 1);
pub const MPP14_GIGE: u32 = MPP(14, 0x1, 0, 0, 1, 1, 1);
pub const MPP14_NAND: u32 = MPP(14, 0x4, 0, 0, 0, 1, 1);
pub const MPP14_SATA_LED: u32 = MPP(14, 0x5, 0, 0, 0, 1, 0);

pub const MPP15_UNUSED: u32 = MPP(15, 0x0, 0, 0, 1, 1, 1);
pub const MPP15_GPIO: u32 = MPP(15, 0x0, 1, 1, 1, 1, 1);
pub const MPP15_GIGE: u32 = MPP(15, 0x1, 0, 0, 1, 1, 1);
pub const MPP15_NAND: u32 = MPP(15, 0x4, 0, 0, 0, 1, 1);
pub const MPP15_SATA_LED: u32 = MPP(15, 0x5, 0, 0, 0, 1, 0);

pub const MPP16_UNUSED: u32 = MPP(16, 0x0, 0, 0, 1, 1, 1);
pub const MPP16_GPIO: u32 = MPP(16, 0x5, 1, 1, 0, 1, 0);
pub const MPP16_GIGE: u32 = MPP(16, 0x1, 0, 0, 1, 1, 1);
pub const MPP16_NAND: u32 = MPP(16, 0x4, 0, 0, 0, 1, 1);
pub const MPP16_UART: u32 = MPP(16, 0x0, 0, 0, 0, 1, 1);

pub const MPP17_UNUSED: u32 = MPP(17, 0x0, 0, 0, 1, 1, 1);
pub const MPP17_GPIO: u32 = MPP(17, 0x5, 1, 1, 0, 1, 0);
pub const MPP17_GIGE: u32 = MPP(17, 0x1, 0, 0, 1, 1, 1);
pub const MPP17_NAND: u32 = MPP(17, 0x4, 0, 0, 0, 1, 1);
pub const MPP17_UART: u32 = MPP(17, 0x0, 0, 0, 0, 1, 1);

pub const MPP18_UNUSED: u32 = MPP(18, 0x0, 0, 0, 1, 1, 1);
pub const MPP18_GPIO: u32 = MPP(18, 0x5, 1, 1, 0, 1, 0);
pub const MPP18_GIGE: u32 = MPP(18, 0x1, 0, 0, 1, 1, 1);
pub const MPP18_UART: u32 = MPP(18, 0x0, 0, 0, 0, 1, 1);

pub const MPP19_UNUSED: u32 = MPP(19, 0x0, 0, 0, 1, 1, 1);
pub const MPP19_GPIO: u32 = MPP(19, 0x5, 1, 1, 0, 1, 0);
pub const MPP19_GIGE: u32 = MPP(19, 0x1, 0, 0, 1, 1, 1);
pub const MPP19_UART: u32 = MPP(19, 0x0, 0, 0, 0, 1, 1);

pub const MPP_MAX: u32 = 19;

unsafe extern "C" {
    pub fn orion5x_mpp_conf(mpp_list: *mut u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
