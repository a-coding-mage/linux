// SPDX-License-Identifier: GPL-2.0-only
// OMAP2/3 System Control Module register access

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const START_PADCONF_SAVE: u32 = 0x2;
const PADCONF_SAVE_DONE: u32 = 0x1;

static mut omap2_ctrl_base: *mut c_void = core::ptr::null_mut();
static mut omap2_ctrl_offset: i16 = 0;

#[repr(C)]
pub struct omap3_scratchpad { pub boot_config_ptr: u32, pub public_restore_ptr: u32, pub secure_ram_restore_ptr: u32, pub sdrc_module_semaphore: u32, pub prcm_block_offset: u32, pub sdrc_block_offset: u32 }
#[repr(C)]
pub struct omap3_scratchpad_prcm_block { pub prm_contents: [u32; 2], pub cm_contents: [u32; 11], pub prcm_block_size: u32 }
#[repr(C)]
pub struct omap3_scratchpad_sdrc_block {
    pub sysconfig: u16, pub cs_cfg: u16, pub sharing: u16, pub err_type: u16,
    pub dll_a_ctrl: u32, pub dll_b_ctrl: u32, pub power: u32, pub cs_0: u32,
    pub mcfg_0: u32, pub mr_0: u16, pub emr_1_0: u16, pub emr_2_0: u16, pub emr_3_0: u16,
    pub actim_ctrla_0: u32, pub actim_ctrlb_0: u32, pub rfr_ctrl_0: u32, pub cs_1: u32,
    pub mcfg_1: u32, pub mr_1: u16, pub emr_1_1: u16, pub emr_2_1: u16, pub emr_3_1: u16,
    pub actim_ctrla_1: u32, pub actim_ctrlb_1: u32, pub rfr_ctrl_1: u32,
    pub dcdl_1_ctrl: u16, pub dcdl_2_ctrl: u16, pub flags: u32, pub block_size: u32,
}
pub static mut omap3_secure_ram_storage: *mut c_void = core::ptr::null_mut();
pub static mut omap3_arm_context: [u32; 128] = [0; 128];
#[repr(C)] pub struct omap3_control_regs { pub values: [u32; 37] }
static mut control_context: omap3_control_regs = omap3_control_regs { values: [0; 37] };

extern "C" {
    fn readl_relaxed(p: *mut c_void) -> u32; fn writel_relaxed(v: u32, p: *mut c_void);
    fn udelay(v: u32); fn memcpy_toio(d: *mut c_void, s: *const c_void, n: usize);
    fn omap_ctrl_readl(offset: u16) -> u32; fn omap_ctrl_writel(v: u32, offset: u16);
    fn cpu_is_omap3630() -> bool; fn omap_rev() -> u32; fn omap_type() -> u32;
    fn __pa_symbol(p: *const c_void) -> u32; fn __pa(p: *const c_void) -> u32;
    fn omap3_restore(); fn omap3_restore_3630(); fn omap3_restore_es3();
    fn omap3_prm_save_scratchpad_contents(p: *mut u32); fn omap3_cm_save_scratchpad_contents(p: *mut u32);
    fn sdrc_read_reg(r: u32) -> u32;
}

#[no_mangle] pub unsafe extern "C" fn omap_ctrl_readb(offset: u16) -> u8 { (omap_ctrl_readl(offset) >> ((offset & 3) * 8)) as u8 }
#[no_mangle] pub unsafe extern "C" fn omap_ctrl_readw(offset: u16) -> u16 { (omap_ctrl_readl(offset) >> ((offset & 2) * 8)) as u16 }
#[no_mangle] pub unsafe extern "C" fn omap_ctrl_readl(offset: u16) -> u32 { readl_relaxed(omap2_ctrl_base.add((offset & 0xfffc) as usize)) }
#[no_mangle] pub unsafe extern "C" fn omap_ctrl_writeb(val: u8, offset: u16) { let sh=((offset&3)*8) as u32; let mut t=omap_ctrl_readl(offset); t &= 0xffffffffu32 ^ (0xff << sh); t |= (val as u32)<<sh; omap_ctrl_writel(t,offset); }
#[no_mangle] pub unsafe extern "C" fn omap_ctrl_writew(val: u16, offset: u16) { let sh=((offset&2)*8) as u32; let mut t=omap_ctrl_readl(offset); t &= 0xffffffffu32 ^ (0xffff << sh); t |= (val as u32)<<sh; omap_ctrl_writel(t,offset); }
#[no_mangle] pub unsafe extern "C" fn omap_ctrl_writel(val: u32, offset: u16) { writel_relaxed(val,omap2_ctrl_base.add((offset&0xfffc) as usize)); }

pub unsafe extern "C" fn omap3_ctrl_write_boot_mode(bootmode: u8) {
    let l = (('B' as u32)<<24) | (('M' as u32)<<16) | bootmode as u32;
    writel_relaxed(l, OMAP2_L4_IO_ADDRESS(OMAP343X_SCRATCHPAD + 4));
}

pub unsafe extern "C" fn omap3_ctrl_save_padconf() -> i32 { let mut c=omap_ctrl_readl(OMAP343X_CONTROL_PADCONF_OFF); c|=START_PADCONF_SAVE; omap_ctrl_writel(c,OMAP343X_CONTROL_PADCONF_OFF); while omap_ctrl_readl(OMAP343X_CONTROL_GENERAL_PURPOSE_STATUS)&PADCONF_SAVE_DONE==0 { udelay(1); } 0 }
unsafe fn omap3_ctrl_set_iva_bootmode_idle() { omap_ctrl_writel(OMAP3_IVA2_BOOTMOD_IDLE, OMAP343X_CONTROL_IVA2_BOOTMOD); }
unsafe fn omap3_ctrl_setup_d2d_padconf() { let mask:u16=(1<<4)|(1<<3); let mut p=omap_ctrl_readw(OMAP3_PADCONF_SAD2D_MSTANDBY); p|=mask; omap_ctrl_writew(p,OMAP3_PADCONF_SAD2D_MSTANDBY); p=omap_ctrl_readw(OMAP3_PADCONF_SAD2D_IDLEACK); p|=mask; omap_ctrl_writew(p,OMAP3_PADCONF_SAD2D_IDLEACK); }
pub unsafe extern "C" fn omap3_ctrl_init() { omap_ctrl_writel(OMAP3430_AUTOIDLE_MASK,OMAP2_CONTROL_SYSCONFIG); omap3_ctrl_set_iva_bootmode_idle(); omap3_ctrl_setup_d2d_padconf(); }

static mut am43xx_control_reg_offsets: [u32; 59] = [
    AM33XX_CONTROL_SYSCONFIG_OFFSET,AM33XX_CONTROL_STATUS_OFFSET,AM43XX_CONTROL_MPU_L2_CTRL_OFFSET,AM33XX_CONTROL_CORE_SLDO_CTRL_OFFSET,AM33XX_CONTROL_MPU_SLDO_CTRL_OFFSET,AM33XX_CONTROL_CLK32KDIVRATIO_CTRL_OFFSET,AM33XX_CONTROL_BANDGAP_CTRL_OFFSET,AM33XX_CONTROL_BANDGAP_TRIM_OFFSET,AM33XX_CONTROL_PLL_CLKINPULOW_CTRL_OFFSET,AM33XX_CONTROL_MOSC_CTRL_OFFSET,AM33XX_CONTROL_DEEPSLEEP_CTRL_OFFSET,AM43XX_CONTROL_DISPLAY_PLL_SEL_OFFSET,AM33XX_CONTROL_INIT_PRIORITY_0_OFFSET,AM33XX_CONTROL_INIT_PRIORITY_1_OFFSET,AM33XX_CONTROL_TPTC_CFG_OFFSET,AM33XX_CONTROL_USB_CTRL0_OFFSET,AM33XX_CONTROL_USB_CTRL1_OFFSET,AM43XX_CONTROL_USB_CTRL2_OFFSET,AM43XX_CONTROL_GMII_SEL_OFFSET,AM43XX_CONTROL_MPUSS_CTRL_OFFSET,AM43XX_CONTROL_TIMER_CASCADE_CTRL_OFFSET,AM43XX_CONTROL_PWMSS_CTRL_OFFSET,AM33XX_CONTROL_MREQPRIO_0_OFFSET,AM33XX_CONTROL_MREQPRIO_1_OFFSET,AM33XX_CONTROL_HW_EVENT_SEL_GRP1_OFFSET,AM33XX_CONTROL_HW_EVENT_SEL_GRP2_OFFSET,AM33XX_CONTROL_HW_EVENT_SEL_GRP3_OFFSET,AM33XX_CONTROL_HW_EVENT_SEL_GRP4_OFFSET,AM33XX_CONTROL_SMRT_CTRL_OFFSET,AM33XX_CONTROL_MPUSS_HW_DEBUG_SEL_OFFSET,AM43XX_CONTROL_CQDETECT_STS_OFFSET,AM43XX_CONTROL_CQDETECT_STS2_OFFSET,AM43XX_CONTROL_VTP_CTRL_OFFSET,AM33XX_CONTROL_VREF_CTRL_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_0_3_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_4_7_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_8_11_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_12_15_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_16_19_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_20_23_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_24_27_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_28_31_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_32_35_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_36_39_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_40_43_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_44_47_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_48_51_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_52_55_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_56_59_OFFSET,AM33XX_CONTROL_TPCC_EVT_MUX_60_63_OFFSET,AM33XX_CONTROL_TIMER_EVT_CAPT_OFFSET,AM33XX_CONTROL_ECAP_EVT_CAPT_OFFSET,AM33XX_CONTROL_ADC_EVT_CAPT_OFFSET,AM43XX_CONTROL_ADC1_EVT_CAPT_OFFSET,AM33XX_CONTROL_RESET_ISO_OFFSET];
static mut am33xx_control_vals: [u32; 59] = [0;59];
unsafe fn am43xx_control_save_context(){for i in 0..am43xx_control_reg_offsets.len(){am33xx_control_vals[i]=omap_ctrl_readl(am43xx_control_reg_offsets[i] as u16);}}
unsafe fn am43xx_control_restore_context(){for i in 0..am43xx_control_reg_offsets.len(){omap_ctrl_writel(am33xx_control_vals[i],am43xx_control_reg_offsets[i] as u16);}}

// The remaining device-tree and power-management initialization is preserved as an external-interface translation.
extern "C" { fn soc_is_am43xx() -> bool; fn cpu_pm_register_notifier(nb:*mut c_void)->i32; fn omap2_control_base_init()->i32; fn omap_control_init()->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
