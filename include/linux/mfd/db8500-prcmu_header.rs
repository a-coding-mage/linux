/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of db8500-prcmu.h. */

// External kernel types and symbols are supplied by the surrounding translation.
pub type bool = core::ffi::c_bool;
pub type s32 = i32;
pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type unsigned_long = core::ffi::c_ulong;

pub const DB8500_PRCM_LINE_VALUE: u32 = 0x170;
pub const DB8500_PRCM_LINE_VALUE_HSI_CAWAKE0: u32 = 1 << 3;
pub const DB8500_PRCM_DSI_SW_RESET: u32 = 0x324;
pub const DB8500_PRCM_DSI_SW_RESET_DSI0_SW_RESETN: u32 = 1 << 0;
pub const DB8500_PRCM_DSI_SW_RESET_DSI1_SW_RESETN: u32 = 1 << 1;
pub const DB8500_PRCM_DSI_SW_RESET_DSI2_SW_RESETN: u32 = 1 << 2;
pub const DB8500_PRCMU_FW_VERSION_OFFSET: u32 = 0xA4;
pub const DB8500_PRCMU_LEGACY_OFFSET: u32 = 0xDD4;

pub const PRCMU_CLKSRC_CLK38M: u8 = 0x00; pub const PRCMU_CLKSRC_ACLK: u8 = 0x01;
pub const PRCMU_CLKSRC_SYSCLK: u8 = 0x02; pub const PRCMU_CLKSRC_LCDCLK: u8 = 0x03;
pub const PRCMU_CLKSRC_SDMMCCLK: u8 = 0x04; pub const PRCMU_CLKSRC_TVCLK: u8 = 0x05;
pub const PRCMU_CLKSRC_TIMCLK: u8 = 0x06; pub const PRCMU_CLKSRC_CLK009: u8 = 0x07;
pub const PRCMU_CLKSRC_SIAMMDSPCLK: u8 = 0x40; pub const PRCMU_CLKSRC_I2CCLK: u8 = 0x41;
pub const PRCMU_CLKSRC_MSP02CLK: u8 = 0x42; pub const PRCMU_CLKSRC_ARMPLL_OBSCLK: u8 = 0x43;
pub const PRCMU_CLKSRC_HSIRXCLK: u8 = 0x44; pub const PRCMU_CLKSRC_HSITXCLK: u8 = 0x45;
pub const PRCMU_CLKSRC_ARMCLKFIX: u8 = 0x46; pub const PRCMU_CLKSRC_HDMICLK: u8 = 0x47;
pub const ESRAM0_DEEP_SLEEP_STATE_OFF: u8 = 1; pub const ESRAM0_DEEP_SLEEP_STATE_RET: u8 = 2;

pub type state = u32; pub const OFF: state = 0; pub const ON: state = 1;
pub type ret_state = u32; pub const OFFST: ret_state = 0; pub const ONST: ret_state = 1; pub const RETST: ret_state = 2;
pub type clk_arm = u32; pub const A9_OFF: clk_arm = 0; pub const A9_BOOT: clk_arm = 1; pub const A9_OPPT1: clk_arm = 2; pub const A9_OPPT2: clk_arm = 3; pub const A9_EXTCLK: clk_arm = 4;
pub type clk_gen = u32; pub const GEN_OFF: clk_gen = 0; pub const GEN_BOOT: clk_gen = 1; pub const GEN_OPPT1: clk_gen = 2;
pub type romcode_write = u32; pub const RDY_2_DS: romcode_write = 0x09; pub const RDY_2_XP70_RST: romcode_write = 0x10;
pub type romcode_read = u32; pub const INIT: romcode_read = 0; pub const FS_2_DS: romcode_read = 0x0A; pub const END_DS: romcode_read = 0x0B; pub const DS_TO_FS: romcode_read = 0x0C; pub const END_FS: romcode_read = 0x0D; pub const SWR: romcode_read = 0x0E; pub const END_SWR: romcode_read = 0x0F;
pub type ap_pwrst = u32; pub const NO_PWRST: ap_pwrst = 0; pub const AP_BOOT: ap_pwrst = 1; pub const AP_EXECUTE: ap_pwrst = 2; pub const AP_DEEP_SLEEP: ap_pwrst = 3; pub const AP_SLEEP: ap_pwrst = 4; pub const AP_IDLE: ap_pwrst = 5; pub const AP_RESET: ap_pwrst = 6;
pub type ap_pwrst_trans = u32; pub const PRCMU_AP_NO_CHANGE: ap_pwrst_trans = 0; pub const APEXECUTE_TO_APSLEEP: ap_pwrst_trans = 1; pub const APIDLE_TO_APSLEEP: ap_pwrst_trans = 2; pub const PRCMU_AP_SLEEP: ap_pwrst_trans = 1; pub const APBOOT_TO_APEXECUTE: ap_pwrst_trans = 3; pub const APEXECUTE_TO_APDEEPSLEEP: ap_pwrst_trans = 4; pub const PRCMU_AP_DEEP_SLEEP: ap_pwrst_trans = 4; pub const APEXECUTE_TO_APIDLE: ap_pwrst_trans = 5; pub const PRCMU_AP_IDLE: ap_pwrst_trans = 5; pub const PRCMU_AP_DEEP_IDLE: ap_pwrst_trans = 7;
pub type hw_acc_state = u32; pub const HW_NO_CHANGE: hw_acc_state = 0; pub const HW_OFF: hw_acc_state = 1; pub const HW_OFF_RAMRET: hw_acc_state = 2; pub const HW_ON: hw_acc_state = 4;
pub type ap_pwrsttr_status = u32;
pub const BOOT_TO_EXECUTEOK: ap_pwrsttr_status = 0xff; pub const DEEPSLEEPOK: ap_pwrsttr_status = 0xfe; pub const SLEEPOK: ap_pwrsttr_status = 0xfd; pub const IDLEOK: ap_pwrsttr_status = 0xfc; pub const SOFTRESETOK: ap_pwrsttr_status = 0xfb; pub const SOFTRESETGO: ap_pwrsttr_status = 0xfa; pub const BOOT_TO_EXECUTE: ap_pwrsttr_status = 0xf9; pub const EXECUTE_TO_DEEPSLEEP: ap_pwrsttr_status = 0xf8; pub const DEEPSLEEP_TO_EXECUTE: ap_pwrsttr_status = 0xf7; pub const DEEPSLEEP_TO_EXECUTEOK: ap_pwrsttr_status = 0xf6; pub const EXECUTE_TO_SLEEP: ap_pwrsttr_status = 0xf5; pub const SLEEP_TO_EXECUTE: ap_pwrsttr_status = 0xf4; pub const SLEEP_TO_EXECUTEOK: ap_pwrsttr_status = 0xf3; pub const EXECUTE_TO_IDLE: ap_pwrsttr_status = 0xf2; pub const IDLE_TO_EXECUTE: ap_pwrsttr_status = 0xf1; pub const IDLE_TO_EXECUTEOK: ap_pwrsttr_status = 0xf0; pub const RDYTODS_RETURNTOEXE: ap_pwrsttr_status = 0xef; pub const NORDYTODS_RETURNTOEXE: ap_pwrsttr_status = 0xee; pub const EXETOSLEEP_RETURNTOEXE: ap_pwrsttr_status = 0xed; pub const EXETOIDLE_RETURNTOEXE: ap_pwrsttr_status = 0xec; pub const INIT_STATUS: ap_pwrsttr_status = 0xeb;
pub const INITERROR: ap_pwrsttr_status = 0; pub const PLLARMLOCKP_ER: ap_pwrsttr_status = 1; pub const PLLDDRLOCKP_ER: ap_pwrsttr_status = 2; pub const PLLSOCLOCKP_ER: ap_pwrsttr_status = 3; pub const PLLSOCK1LOCKP_ER: ap_pwrsttr_status = 4; pub const ARMWFI_ER: ap_pwrsttr_status = 5; pub const SYSCLKOK_ER: ap_pwrsttr_status = 6; pub const I2C_NACK_DATA_ER: ap_pwrsttr_status = 7; pub const BOOT_ER: ap_pwrsttr_status = 8; pub const I2C_STATUS_ALWAYS_1: ap_pwrsttr_status = 0x0a; pub const I2C_NACK_REG_ADDR_ER: ap_pwrsttr_status = 0x0b; pub const I2C_NACK_DATA0123_ER: ap_pwrsttr_status = 0x1b; pub const I2C_NACK_ADDR_ER: ap_pwrsttr_status = 0x1f; pub const CURAPPWRSTISNOT_BOOT: ap_pwrsttr_status = 0x20; pub const CURAPPWRSTISNOT_EXECUTE: ap_pwrsttr_status = 0x21; pub const CURAPPWRSTISNOT_SLEEPMODE: ap_pwrsttr_status = 0x22; pub const CURAPPWRSTISNOT_CORRECTFORIT10: ap_pwrsttr_status = 0x23; pub const FIFO4500WUISNOT_WUPEVENT: ap_pwrsttr_status = 0x24; pub const PLL32KLOCKP_ER: ap_pwrsttr_status = 0x29; pub const DDRDEEPSLEEPOK_ER: ap_pwrsttr_status = 0x2a; pub const ROMCODEREADY_ER: ap_pwrsttr_status = 0x50; pub const WUPBEFOREDS: ap_pwrsttr_status = 0x51; pub const DDRCONFIG_ER: ap_pwrsttr_status = 0x52; pub const WUPBEFORESLEEP: ap_pwrsttr_status = 0x53; pub const WUPBEFOREIDLE: ap_pwrsttr_status = 0x54;
pub type mbox_to_arm_err = u32;
pub const INIT_ERR: mbox_to_arm_err = 0; pub const PLLARMLOCKP_ERR: mbox_to_arm_err = 1; pub const PLLDDRLOCKP_ERR: mbox_to_arm_err = 2; pub const PLLSOC0LOCKP_ERR: mbox_to_arm_err = 3; pub const PLLSOC1LOCKP_ERR: mbox_to_arm_err = 4; pub const ARMWFI_ERR: mbox_to_arm_err = 5; pub const SYSCLKOK_ERR: mbox_to_arm_err = 6; pub const BOOT_ERR: mbox_to_arm_err = 7; pub const ROMCODESAVECONTEXT: mbox_to_arm_err = 8; pub const VARMHIGHSPEEDVALTO_ERR: mbox_to_arm_err = 0x10; pub const VARMHIGHSPEEDACCESS_ERR: mbox_to_arm_err = 0x11; pub const VARMLOWSPEEDVALTO_ERR: mbox_to_arm_err = 0x12; pub const VARMLOWSPEEDACCESS_ERR: mbox_to_arm_err = 0x13; pub const VARMRETENTIONVALTO_ERR: mbox_to_arm_err = 0x14; pub const VARMRETENTIONACCESS_ERR: mbox_to_arm_err = 0x15; pub const VAPEHIGHSPEEDVALTO_ERR: mbox_to_arm_err = 0x16; pub const VSAFEHPVALTO_ERR: mbox_to_arm_err = 0x17; pub const VMODSEL1VALTO_ERR: mbox_to_arm_err = 0x18; pub const VMODSEL2VALTO_ERR: mbox_to_arm_err = 0x19; pub const VARMOFFACCESS_ERR: mbox_to_arm_err = 0x1a; pub const VAPEOFFACCESS_ERR: mbox_to_arm_err = 0x1b; pub const VARMRETACCES_ERR: mbox_to_arm_err = 0x1c; pub const CURAPPWRSTISNOTBOOT: mbox_to_arm_err = 0x20; pub const CURAPPWRSTISNOTEXECUTE: mbox_to_arm_err = 0x21; pub const CURAPPWRSTISNOTSLEEPMODE: mbox_to_arm_err = 0x22; pub const CURAPPWRSTISNOTCORRECTDBG: mbox_to_arm_err = 0x23; pub const ARMREGU1VALTO_ERR: mbox_to_arm_err = 0x24; pub const ARMREGU2VALTO_ERR: mbox_to_arm_err = 0x25; pub const VAPEREGUVALTO_ERR: mbox_to_arm_err = 0x26; pub const VSMPS3REGUVALTO_ERR: mbox_to_arm_err = 0x27; pub const VMODREGUVALTO_ERR: mbox_to_arm_err = 0x28;
pub type dvfs_stat = u32; pub const DVFS_GO: dvfs_stat = 0xff; pub const DVFS_ARM100OPPOK: dvfs_stat = 0xfe; pub const DVFS_ARM50OPPOK: dvfs_stat = 0xfd; pub const DVFS_ARMEXTCLKOK: dvfs_stat = 0xfc; pub const DVFS_NOCHGTCLKOK: dvfs_stat = 0xfb; pub const DVFS_INITSTATUS: dvfs_stat = 0;
pub type sva_mmdsp_stat = u32; pub const SVA_MMDSP_GO: sva_mmdsp_stat = 0xff; pub const SVA_MMDSP_INIT: sva_mmdsp_stat = 0;
pub type sia_mmdsp_stat = u32; pub const SIA_MMDSP_GO: sia_mmdsp_stat = 0xff; pub const SIA_MMDSP_INIT: sia_mmdsp_stat = 0;
pub type hw_acc = u32; pub const SVAMMDSP: hw_acc = 0; pub const SVAPIPE: hw_acc = 1; pub const SIAMMDSP: hw_acc = 2; pub const SIAPIPE: hw_acc = 3; pub const SGA: hw_acc = 4; pub const B2R2MCDE: hw_acc = 5; pub const ESRAM12: hw_acc = 6; pub const ESRAM34: hw_acc = 7;
pub type cs_pwrmgt = u32; pub const PWRDNCS0: cs_pwrmgt = 0; pub const WKUPCS0: cs_pwrmgt = 1; pub const PWRDNCS1: cs_pwrmgt = 2; pub const WKUPCS1: cs_pwrmgt = 3;
pub type sia_sva_pwr_policy = u32; pub const NO_CHGT: sia_sva_pwr_policy = 0; pub const DSPOFF_HWPOFF: sia_sva_pwr_policy = 1; pub const DSPOFFRAMRET_HWPOFF: sia_sva_pwr_policy = 2; pub const DSPCLKOFF_HWPOFF: sia_sva_pwr_policy = 3; pub const DSPCLKOFF_HWPCLKOFF: sia_sva_pwr_policy = 4;
pub type auto_enable = u32; pub const AUTO_OFF: auto_enable = 0; pub const AUTO_ON: auto_enable = 1;

pub type prcmu_power_status = u32;
pub const PRCMU_SLEEP_OK: prcmu_power_status = 0xf3; pub const PRCMU_DEEP_SLEEP_OK: prcmu_power_status = 0xf6; pub const PRCMU_IDLE_OK: prcmu_power_status = 0xf0; pub const PRCMU_DEEPIDLE_OK: prcmu_power_status = 0xe3; pub const PRCMU_PRCMU2ARMPENDINGIT_ER: prcmu_power_status = 0x91; pub const PRCMU_ARMPENDINGIT_ER: prcmu_power_status = 0x93;
pub type prcmu_wakeup_index = u32;
pub const PRCMU_WAKEUP_INDEX_RTC: prcmu_wakeup_index = 0; pub const PRCMU_WAKEUP_INDEX_RTT0: prcmu_wakeup_index = 1; pub const PRCMU_WAKEUP_INDEX_RTT1: prcmu_wakeup_index = 2; pub const PRCMU_WAKEUP_INDEX_HSI0: prcmu_wakeup_index = 3; pub const PRCMU_WAKEUP_INDEX_HSI1: prcmu_wakeup_index = 4; pub const PRCMU_WAKEUP_INDEX_USB: prcmu_wakeup_index = 5; pub const PRCMU_WAKEUP_INDEX_ABB: prcmu_wakeup_index = 6; pub const PRCMU_WAKEUP_INDEX_ABB_FIFO: prcmu_wakeup_index = 7; pub const PRCMU_WAKEUP_INDEX_ARM: prcmu_wakeup_index = 8; pub const PRCMU_WAKEUP_INDEX_CD_IRQ: prcmu_wakeup_index = 9; pub const NUM_PRCMU_WAKEUP_INDICES: u32 = 10;
#[inline] pub const fn PRCMU_WAKEUP(index: u32) -> u32 { 1u32 << index }
pub type prcmu_wdog_id = u32; pub const PRCMU_WDOG_ALL: prcmu_wdog_id = 0; pub const PRCMU_WDOG_CPU1: prcmu_wdog_id = 1; pub const PRCMU_WDOG_CPU2: prcmu_wdog_id = 2;
pub type ape_opp = u32; pub const APE_OPP_INIT: ape_opp = 0; pub const APE_NO_CHANGE: ape_opp = 1; pub const APE_100_OPP: ape_opp = 2; pub const APE_50_OPP: ape_opp = 3; pub const APE_50_PARTLY_25_OPP: ape_opp = 0xff;
pub type arm_opp = u32; pub const ARM_OPP_INIT: arm_opp = 0; pub const ARM_NO_CHANGE: arm_opp = 1; pub const ARM_100_OPP: arm_opp = 2; pub const ARM_50_OPP: arm_opp = 3; pub const ARM_MAX_OPP: arm_opp = 4; pub const ARM_MAX_FREQ100OPP: arm_opp = 5; pub const ARM_EXTCLK: arm_opp = 7;
pub type ddr_opp = u32; pub const DDR_100_OPP: ddr_opp = 0; pub const DDR_50_OPP: ddr_opp = 1; pub const DDR_25_OPP: ddr_opp = 2;
pub type ddr_pwrst = u32; pub const DDR_PWR_STATE_UNCHANGED: ddr_pwrst = 0; pub const DDR_PWR_STATE_ON: ddr_pwrst = 1; pub const DDR_PWR_STATE_OFFLOWLAT: ddr_pwrst = 2; pub const DDR_PWR_STATE_OFFHIGHLAT: ddr_pwrst = 3;

pub const EPOD_ID_SVAMMDSP: u32 = 0; pub const EPOD_ID_SVAPIPE: u32 = 1; pub const EPOD_ID_SIAMMDSP: u32 = 2; pub const EPOD_ID_SIAPIPE: u32 = 3; pub const EPOD_ID_SGA: u32 = 4; pub const EPOD_ID_B2R2_MCDE: u32 = 5; pub const EPOD_ID_ESRAM12: u32 = 6; pub const EPOD_ID_ESRAM34: u32 = 7; pub const NUM_EPOD_ID: u32 = 8;
pub const EPOD_STATE_NO_CHANGE: u8 = 0; pub const EPOD_STATE_OFF: u8 = 1; pub const EPOD_STATE_RAMRET: u8 = 2; pub const EPOD_STATE_ON_CLK_OFF: u8 = 3; pub const EPOD_STATE_ON: u8 = 4;
pub const PRCMU_FW_PROJECT_U8500: u32 = 2; pub const PRCMU_FW_PROJECT_U8400: u32 = 3; pub const PRCMU_FW_PROJECT_U9500: u32 = 4; pub const PRCMU_FW_PROJECT_U8500_MBB: u32 = 5; pub const PRCMU_FW_PROJECT_U8500_C1: u32 = 6; pub const PRCMU_FW_PROJECT_U8500_C2: u32 = 7; pub const PRCMU_FW_PROJECT_U8500_C3: u32 = 8; pub const PRCMU_FW_PROJECT_U8500_C4: u32 = 9; pub const PRCMU_FW_PROJECT_U9500_MBL: u32 = 10; pub const PRCMU_FW_PROJECT_U8500_SSG1: u32 = 11; pub const PRCMU_FW_PROJECT_U8500_MBL2: u32 = 12; pub const PRCMU_FW_PROJECT_U8520: u32 = 13; pub const PRCMU_FW_PROJECT_U8420: u32 = 14; pub const PRCMU_FW_PROJECT_U8500_SSG2: u32 = 15; pub const PRCMU_FW_PROJECT_U8420_SYSCLK: u32 = 17; pub const PRCMU_FW_PROJECT_A9420: u32 = 20; pub const PRCMU_FW_PROJECT_U9540: u32 = 32; pub const PRCMU_FW_PROJECT_L8540: u32 = 64; pub const PRCMU_FW_PROJECT_L8580: u32 = 96; pub const PRCMU_FW_PROJECT_NAME_LEN: usize = 20;
pub const PRCMU_QOS_APE_OPP: i32 = 1; pub const PRCMU_QOS_DDR_OPP: i32 = 2; pub const PRCMU_QOS_ARM_OPP: i32 = 3; pub const PRCMU_QOS_DEFAULT_VALUE: i32 = -1; pub const PRCMU_AUTO_PM_OFF: u8 = 0; pub const PRCMU_AUTO_PM_ON: u8 = 1; pub const PRCMU_AUTO_PM_POWER_ON_HSEM: u8 = 1; pub const PRCMU_AUTO_PM_POWER_ON_ABB_FIFO_IT: u8 = 2;
pub type prcmu_auto_pm_policy = u32; pub const PRCMU_AUTO_PM_POLICY_NO_CHANGE: prcmu_auto_pm_policy = 0; pub const PRCMU_AUTO_PM_POLICY_DSP_OFF_HWP_OFF: prcmu_auto_pm_policy = 1; pub const PRCMU_AUTO_PM_POLICY_DSP_OFF_RAMRET_HWP_OFF: prcmu_auto_pm_policy = 2; pub const PRCMU_AUTO_PM_POLICY_DSP_CLK_OFF_HWP_OFF: prcmu_auto_pm_policy = 3; pub const PRCMU_AUTO_PM_POLICY_DSP_CLK_OFF_HWP_CLK_OFF: prcmu_auto_pm_policy = 4;

#[repr(C)] pub struct prcmu_fw_version { pub project: u32, pub api_version: u8, pub func_version: u8, pub errata: u8, pub project_name: [core::ffi::c_char; PRCMU_FW_PROJECT_NAME_LEN] }
#[repr(C)] pub struct prcmu_auto_pm_config { pub sia_auto_pm_enable: u8, pub sia_power_on: u8, pub sia_policy: u8, pub sva_auto_pm_enable: u8, pub sva_power_on: u8, pub sva_policy: u8 }

// CONFIG_MFD_DB8500_PRCMU controls whether these are external kernel functions or inline no-op stubs.
extern "C" {
    pub fn db8500_prcmu_early_init();
    pub fn prcmu_set_rc_a2p(code: romcode_write) -> i32;
    pub fn prcmu_get_rc_p2a() -> romcode_read;
    pub fn prcmu_get_xp70_current_state() -> ap_pwrst;
    pub fn prcmu_has_arm_maxopp() -> bool;
    pub fn prcmu_get_fw_version() -> *mut prcmu_fw_version;
    pub fn prcmu_release_usb_wakeup_state() -> i32;
    pub fn prcmu_configure_auto_pm(sleep: *mut prcmu_auto_pm_config, idle: *mut prcmu_auto_pm_config);
    pub fn prcmu_is_auto_pm_enabled() -> bool;
    pub fn prcmu_config_clkout(clkout: u8, source: u8, div: u8) -> i32;
    pub fn prcmu_clock_rate(clock: u8) -> unsigned_long;
    pub fn prcmu_round_clock_rate(clock: u8, rate: unsigned_long) -> i64;
    pub fn prcmu_set_clock_rate(clock: u8, rate: unsigned_long) -> i32;
    pub fn prcmu_set_clock_divider(clock: u8, divider: u8) -> i32;
    pub fn db8500_prcmu_config_hotdog(threshold: u8) -> i32;
    pub fn db8500_prcmu_config_hotmon(low: u8, high: u8) -> i32;
    pub fn db8500_prcmu_start_temp_sense(cycles32k: u16) -> i32;
    pub fn db8500_prcmu_stop_temp_sense() -> i32;
    pub fn prcmu_abb_read(slave: u8, reg: u8, value: *mut u8, size: u8) -> i32;
    pub fn prcmu_abb_write(slave: u8, reg: u8, value: *mut u8, size: u8) -> i32;
    pub fn prcmu_abb_write_masked(slave: u8, reg: u8, value: *mut u8, mask: *mut u8, size: u8) -> i32;
    pub fn prcmu_ac_wake_req() -> i32; pub fn prcmu_ac_sleep_req(); pub fn db8500_prcmu_modem_reset();
    pub fn db8500_prcmu_config_a9wdog(num: u8, sleep_auto_off: bool) -> i32; pub fn db8500_prcmu_enable_a9wdog(id: u8) -> i32; pub fn db8500_prcmu_disable_a9wdog(id: u8) -> i32; pub fn db8500_prcmu_kick_a9wdog(id: u8) -> i32; pub fn db8500_prcmu_load_a9wdog(id: u8, val: u32) -> i32;
    pub fn db8500_prcmu_system_reset(reset_code: u16); pub fn db8500_prcmu_set_power_state(state: u8, keep_ulp_clk: bool, keep_ap_pll: bool) -> i32; pub fn db8500_prcmu_get_power_state_result() -> u8; pub fn db8500_prcmu_enable_wakeups(wakeups: u32); pub fn db8500_prcmu_set_epod(epod_id: u16, epod_state: u8) -> i32; pub fn db8500_prcmu_request_clock(clock: u8, enable: bool) -> i32; pub fn db8500_prcmu_config_abb_event_readout(abb_events: u32); pub fn db8500_prcmu_get_abb_event_buffer(buf: *mut *mut core::ffi::c_void); pub fn db8500_prcmu_config_esram0_deep_sleep(state: u8) -> i32; pub fn db8500_prcmu_get_reset_code() -> u16; pub fn db8500_prcmu_is_ac_wake_requested() -> bool; pub fn db8500_prcmu_set_arm_opp(opp: u8) -> i32; pub fn db8500_prcmu_get_arm_opp() -> i32; pub fn db8500_prcmu_set_ape_opp(opp: u8) -> i32; pub fn db8500_prcmu_get_ape_opp() -> i32; pub fn db8500_prcmu_request_ape_opp_100_voltage(enable: bool) -> i32; pub fn db8500_prcmu_get_ddr_opp() -> i32;
    pub fn db8500_prcmu_read(reg: u32) -> u32; pub fn db8500_prcmu_write(reg: u32, value: u32); pub fn db8500_prcmu_write_masked(reg: u32, mask: u32, value: u32);
}

#[inline] pub fn prcmu_qos_add_requirement(_: i32, _: *mut core::ffi::c_char, _: i32) -> i32 { 0 }
#[inline] pub fn prcmu_qos_update_requirement(_: i32, _: *mut core::ffi::c_char, _: i32) -> i32 { 0 }
#[inline] pub fn prcmu_qos_remove_requirement(_: i32, _: *mut core::ffi::c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
