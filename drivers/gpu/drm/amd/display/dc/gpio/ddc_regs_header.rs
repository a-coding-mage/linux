/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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
 *
 * Authors: AMD
 */

// Dependency supplied by gpio_regs.h in the original header.

#[repr(C)]
pub struct ddc_registers {
    pub gpio: gpio_registers,
    pub ddc_setup: u32,
    pub phy_aux_cntl: u32,
    pub dc_gpio_aux_ctrl_5: u32,
    pub dc_i3cpad_control0: u32,
    pub dc_i3cpad_control1: u32,
}

#[repr(C)]
pub struct ddc_sh_mask {
    /* i2c_dd_setup */
    pub DC_I2C_DDC1_ENABLE: u32,
    pub DC_I2C_DDC1_EDID_DETECT_ENABLE: u32,
    pub DC_I2C_DDC1_EDID_DETECT_MODE: u32,
    /* ddc1_mask */
    pub DC_GPIO_DDC1DATA_PD_EN: u32,
    pub DC_GPIO_DDC1CLK_PD_EN: u32,
    pub AUX_PAD1_MODE: u32,
    /* i2cpad_mask */
    pub DC_GPIO_SDA_PD_DIS: u32,
    pub DC_GPIO_SCL_PD_DIS: u32,
    // phy_aux_cntl
    pub AUX_PAD_RXSEL: u32,
    pub DDC_PAD_I2CMODE: u32,
    // dc_i3cpad_control0
    pub DC_I3CPAD_DDCCLK_MASK: u32,
    pub DC_I3CPAD_DDCDATA_MASK: u32,
    pub DC_I3CPAD_PD_EN: u32,
    pub DC_I3CPAD_CLK_A: u32,
    pub DC_I3CPAD_DATA_A: u32,
    pub DC_I3CPAD_CLK_EN: u32,
    pub DC_I3CPAD_DATA_EN: u32,
    pub DC_I3CPAD_CLK_Y: u32,
    pub DC_I3CPAD_DATA_Y: u32,
    // dc_i3cpad_control1
    pub DC_I3CPAD_STR: u32,
    pub DC_I3CPAD_RXSEL: u32,
}

// The following macros preserve the original C preprocessor token-pasting
// interfaces. Their expanded register symbols are supplied by dependencies.
#[macro_export]
macro_rules! DDC_GPIO_REG_LIST_ENTRY { ($type:ident, $cd:ident, $id:ident) => {
    /* .type_reg = REG(DC_GPIO_DDC##id##_##type),
       .type_mask = DC_GPIO_DDC##id##_##type##__DC_GPIO_DDC##id##cd##_##type##_MASK,
       .type_shift = DC_GPIO_DDC##id##_##type##__DC_GPIO_DDC##id##cd##_##type##__SHIFT */
}; }
#[macro_export]
macro_rules! DDC_GPIO_REG_LIST { ($cd:ident, $id:ident) => { DDC_GPIO_REG_LIST_ENTRY!(MASK,$cd,$id); DDC_GPIO_REG_LIST_ENTRY!(A,$cd,$id); DDC_GPIO_REG_LIST_ENTRY!(EN,$cd,$id); DDC_GPIO_REG_LIST_ENTRY!(Y,$cd,$id); }; }
#[macro_export]
macro_rules! DDC_REG_LIST { ($cd:ident, $id:ident) => { DDC_GPIO_REG_LIST!($cd,$id); /* .ddc_setup = REG(DC_I2C_DDC##id##_SETUP) */ }; }
#[macro_export]
macro_rules! DDC_REG_LIST_DCN2 { ($cd:ident, $id:ident) => { DDC_REG_LIST!($cd,$id); /* .phy_aux_cntl = REG(PHY_AUX_CNTL), .dc_gpio_aux_ctrl_5 = REG(DC_GPIO_AUX_CTRL_5) */ }; }
#[macro_export]
macro_rules! DDC_GPIO_VGA_REG_LIST_ENTRY { ($type:ident, $cd:ident) => { /* VGA register token-pasted entry */ }; }
#[macro_export]
macro_rules! DDC_GPIO_VGA_REG_LIST { ($cd:ident) => { DDC_GPIO_VGA_REG_LIST_ENTRY!(MASK,$cd); DDC_GPIO_VGA_REG_LIST_ENTRY!(A,$cd); DDC_GPIO_VGA_REG_LIST_ENTRY!(EN,$cd); DDC_GPIO_VGA_REG_LIST_ENTRY!(Y,$cd); }; }
#[macro_export]
macro_rules! DDC_VGA_REG_LIST { ($cd:ident) => { DDC_GPIO_VGA_REG_LIST!($cd); /* .ddc_setup = mmDC_I2C_DDCVGA_SETUP */ }; }
#[macro_export]
macro_rules! DDC_GPIO_I2C_REG_LIST_ENTRY { ($type:ident, $cd:ident) => { /* I2C PAD register token-pasted entry */ }; }
#[macro_export]
macro_rules! DDC_GPIO_I2C_REG_LIST { ($cd:ident) => { DDC_GPIO_I2C_REG_LIST_ENTRY!(MASK,$cd); DDC_GPIO_I2C_REG_LIST_ENTRY!(A,$cd); DDC_GPIO_I2C_REG_LIST_ENTRY!(EN,$cd); DDC_GPIO_I2C_REG_LIST_ENTRY!(Y,$cd); }; }
#[macro_export]
macro_rules! DDC_I2C_REG_LIST { ($cd:ident) => { DDC_GPIO_I2C_REG_LIST!($cd); /* .ddc_setup = 0 */ }; }
#[macro_export]
macro_rules! DDC_I2C_REG_LIST_DCN2 { ($cd:ident) => { DDC_I2C_REG_LIST!($cd); /* .phy_aux_cntl = REG(PHY_AUX_CNTL), .dc_gpio_aux_ctrl_5 = REG(DC_GPIO_AUX_CTRL_5) */ }; }

#[macro_export]
macro_rules! DDC_MASK_SH_LIST_COMMON { ($mask_sh:ident) => {
    /* SF_DDC(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_ENABLE, mask_sh),
       SF_DDC(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_EDID_DETECT_ENABLE, mask_sh),
       SF_DDC(DC_I2C_DDC1_SETUP, DC_I2C_DDC1_EDID_DETECT_MODE, mask_sh),
       SF_DDC(DC_GPIO_DDC1_MASK, DC_GPIO_DDC1DATA_PD_EN, mask_sh),
       SF_DDC(DC_GPIO_DDC1_MASK, DC_GPIO_DDC1CLK_PD_EN, mask_sh),
       SF_DDC(DC_GPIO_DDC1_MASK, AUX_PAD1_MODE, mask_sh) */
}; }
#[macro_export]
macro_rules! DDC_MASK_SH_LIST { ($mask_sh:ident) => {
    DDC_MASK_SH_LIST_COMMON!($mask_sh);
    /* SF_DDC(DC_GPIO_I2CPAD_MASK, DC_GPIO_SDA_PD_DIS, mask_sh),
       SF_DDC(DC_GPIO_I2CPAD_MASK, DC_GPIO_SCL_PD_DIS, mask_sh) */
}; }
#[macro_export]
macro_rules! DDC_MASK_SH_LIST_DCN2 { ($mask_sh:ident, $cd:ident) => {
    DDC_MASK_SH_LIST_COMMON!($mask_sh);
    /* 0, 0, (PHY_AUX_CNTL__AUX##cd##_PAD_RXSEL##mask_sh),
       (DC_GPIO_AUX_CTRL_5__DDC_PAD##cd##_I2CMODE##mask_sh) */
}; }
#[macro_export]
macro_rules! DDC_MASK_SH_LIST_DCN2_VGA { ($mask_sh:ident) => {
    DDC_MASK_SH_LIST_COMMON!($mask_sh);
    /* 0, 0, 0, 0 */
}; }

// Register-list construction macros retained as source-level declarations.
#[macro_export] macro_rules! ddc_data_regs { ($id:ident) => { DDC_REG_LIST!(DATA,$id) }; }
#[macro_export] macro_rules! ddc_clk_regs { ($id:ident) => { DDC_REG_LIST!(CLK,$id) }; }
#[macro_export] macro_rules! ddc_vga_data_regs { () => { DDC_VGA_REG_LIST!(DATA) }; }
#[macro_export] macro_rules! ddc_vga_clk_regs { () => { DDC_VGA_REG_LIST!(CLK) }; }
#[macro_export] macro_rules! ddc_i2c_data_regs { () => { DDC_I2C_REG_LIST!(SDA) }; }
#[macro_export] macro_rules! ddc_i2c_clk_regs { () => { DDC_I2C_REG_LIST!(SCL) }; }
#[macro_export] macro_rules! ddc_data_regs_dcn2 { ($id:ident) => { DDC_REG_LIST_DCN2!(DATA,$id) }; }
#[macro_export] macro_rules! ddc_clk_regs_dcn2 { ($id:ident) => { DDC_REG_LIST_DCN2!(CLK,$id) }; }
#[macro_export] macro_rules! ddc_i2c_data_regs_dcn2 { () => { DDC_I2C_REG_LIST_DCN2!(SDA) }; }
#[macro_export] macro_rules! ddc_i2c_clk_regs_dcn2 { () => { DDC_I2C_REG_LIST_DCN2!(SCL) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
