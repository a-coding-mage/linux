/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 MediaTek Inc.
 */

#[repr(C)]
pub struct irq_top_t {
    pub hwirq_base: ::core::ffi::c_int,
    pub num_int_regs: ::core::ffi::c_uint,
    pub en_reg: ::core::ffi::c_uint,
    pub en_reg_shift: ::core::ffi::c_uint,
    pub sta_reg: ::core::ffi::c_uint,
    pub sta_reg_shift: ::core::ffi::c_uint,
    pub top_offset: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pmic_irq_data {
    pub num_top: ::core::ffi::c_uint,
    pub num_pmic_irqs: ::core::ffi::c_uint,
    pub top_int_status_reg: ::core::ffi::c_ushort,
    pub enable_hwirq: *mut bool,
    pub cache_hwirq: *mut bool,
    pub pmic_ints: *const irq_top_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum mt6358_irq_top_status_shift {
    MT6358_BUCK_TOP = 0,
    MT6358_LDO_TOP,
    MT6358_PSC_TOP,
    MT6358_SCK_TOP,
    MT6358_BM_TOP,
    MT6358_HK_TOP,
    MT6358_AUD_TOP,
    MT6358_MISC_TOP,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum mt6358_irq_numbers {
    MT6358_IRQ_VPROC11_OC = 0,
    MT6358_IRQ_VPROC12_OC,
    MT6358_IRQ_VCORE_OC,
    MT6358_IRQ_VGPU_OC,
    MT6358_IRQ_VMODEM_OC,
    MT6358_IRQ_VDRAM1_OC,
    MT6358_IRQ_VS1_OC,
    MT6358_IRQ_VS2_OC,
    MT6358_IRQ_VPA_OC,
    MT6358_IRQ_VCORE_PREOC,
    MT6358_IRQ_VFE28_OC = 16,
    MT6358_IRQ_VXO22_OC,
    MT6358_IRQ_VRF18_OC,
    MT6358_IRQ_VRF12_OC,
    MT6358_IRQ_VEFUSE_OC,
    MT6358_IRQ_VCN33_OC,
    MT6358_IRQ_VCN28_OC,
    MT6358_IRQ_VCN18_OC,
    MT6358_IRQ_VCAMA1_OC,
    MT6358_IRQ_VCAMA2_OC,
    MT6358_IRQ_VCAMD_OC,
    MT6358_IRQ_VCAMIO_OC,
    MT6358_IRQ_VLDO28_OC,
    MT6358_IRQ_VA12_OC,
    MT6358_IRQ_VAUX18_OC,
    MT6358_IRQ_VAUD28_OC,
    MT6358_IRQ_VIO28_OC,
    MT6358_IRQ_VIO18_OC,
    MT6358_IRQ_VSRAM_PROC11_OC,
    MT6358_IRQ_VSRAM_PROC12_OC,
    MT6358_IRQ_VSRAM_OTHERS_OC,
    MT6358_IRQ_VSRAM_GPU_OC,
    MT6358_IRQ_VDRAM2_OC,
    MT6358_IRQ_VMC_OC,
    MT6358_IRQ_VMCH_OC,
    MT6358_IRQ_VEMC_OC,
    MT6358_IRQ_VSIM1_OC,
    MT6358_IRQ_VSIM2_OC,
    MT6358_IRQ_VIBR_OC,
    MT6358_IRQ_VUSB_OC,
    MT6358_IRQ_VBIF28_OC,
    MT6358_IRQ_PWRKEY = 48,
    MT6358_IRQ_HOMEKEY,
    MT6358_IRQ_PWRKEY_R,
    MT6358_IRQ_HOMEKEY_R,
    MT6358_IRQ_NI_LBAT_INT,
    MT6358_IRQ_CHRDET,
    MT6358_IRQ_CHRDET_EDGE,
    MT6358_IRQ_VCDT_HV_DET,
    MT6358_IRQ_RTC = 64,
    MT6358_IRQ_FG_BAT0_H = 80,
    MT6358_IRQ_FG_BAT0_L,
    MT6358_IRQ_FG_CUR_H,
    MT6358_IRQ_FG_CUR_L,
    MT6358_IRQ_FG_ZCV,
    MT6358_IRQ_FG_BAT1_H,
    MT6358_IRQ_FG_BAT1_L,
    MT6358_IRQ_FG_N_CHARGE_L,
    MT6358_IRQ_FG_IAVG_H,
    MT6358_IRQ_FG_IAVG_L,
    MT6358_IRQ_FG_TIME_H,
    MT6358_IRQ_FG_DISCHARGE,
    MT6358_IRQ_FG_CHARGE,
    MT6358_IRQ_BATON_LV = 96,
    MT6358_IRQ_BATON_HT,
    MT6358_IRQ_BATON_BAT_IN,
    MT6358_IRQ_BATON_BAT_OUT,
    MT6358_IRQ_BIF,
    MT6358_IRQ_BAT_H = 112,
    MT6358_IRQ_BAT_L,
    MT6358_IRQ_BAT2_H,
    MT6358_IRQ_BAT2_L,
    MT6358_IRQ_BAT_TEMP_H,
    MT6358_IRQ_BAT_TEMP_L,
    MT6358_IRQ_AUXADC_IMP,
    MT6358_IRQ_NAG_C_DLTV,
    MT6358_IRQ_AUDIO = 128,
    MT6358_IRQ_ACCDET = 133,
    MT6358_IRQ_ACCDET_EINT0,
    MT6358_IRQ_ACCDET_EINT1,
    MT6358_IRQ_SPI_CMD_ALERT = 144,
    MT6358_IRQ_NR,
}

pub const MT6358_IRQ_BUCK_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_VPROC11_OC;
pub const MT6358_IRQ_LDO_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_VFE28_OC;
pub const MT6358_IRQ_PSC_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_PWRKEY;
pub const MT6358_IRQ_SCK_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_RTC;
pub const MT6358_IRQ_BM_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_FG_BAT0_H;
pub const MT6358_IRQ_HK_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_BAT_H;
pub const MT6358_IRQ_AUD_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_AUDIO;
pub const MT6358_IRQ_MISC_BASE: mt6358_irq_numbers = mt6358_irq_numbers::MT6358_IRQ_SPI_CMD_ALERT;

pub const MT6358_IRQ_BUCK_BITS: usize = 10;
pub const MT6358_IRQ_LDO_BITS: usize = 25;
pub const MT6358_IRQ_PSC_BITS: usize = 9;
pub const MT6358_IRQ_SCK_BITS: usize = 1;
pub const MT6358_IRQ_BM_BITS: usize = 5;
pub const MT6358_IRQ_HK_BITS: usize = 8;
pub const MT6358_IRQ_AUD_BITS: usize = 6;
pub const MT6358_IRQ_MISC_BITS: usize = 1;

/* MTK_PMIC_REG_WIDTH and the register/top symbols are supplied externally. */
#[macro_export]
macro_rules! MT6358_TOP_GEN {
    ($sp_base:expr, $sp_bits:expr, $en_reg:expr, $sta_reg:expr, $top:expr) => {
        irq_top_t {
            hwirq_base: $sp_base,
            num_int_regs: (($sp_bits - 1) / MTK_PMIC_REG_WIDTH) + 1,
            en_reg: $en_reg,
            en_reg_shift: 0x6,
            sta_reg: $sta_reg,
            sta_reg_shift: 0x2,
            top_offset: $top,
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
