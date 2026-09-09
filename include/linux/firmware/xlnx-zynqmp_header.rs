// SPDX-License-Identifier: GPL-2.0
// Xilinx Zynq MPSoC Firmware layer.
// C header dependencies and CONFIG_ZYNQMP_FIRMWARE conditionality are preserved
// conceptually; dependent kernel symbols are supplied by other translations.

pub const ZYNQMP_PM_VERSION_MAJOR: u32 = 1;
pub const ZYNQMP_PM_VERSION_MINOR: u32 = 0;
pub const ZYNQMP_PM_VERSION: u32 = (ZYNQMP_PM_VERSION_MAJOR << 16) | ZYNQMP_PM_VERSION_MINOR;
pub const ZYNQMP_TZ_VERSION_MAJOR: u32 = 1;
pub const ZYNQMP_TZ_VERSION_MINOR: u32 = 0;
pub const ZYNQMP_TZ_VERSION: u32 = (ZYNQMP_TZ_VERSION_MAJOR << 16) | ZYNQMP_TZ_VERSION_MINOR;
pub const PM_SIP_SVC: u32 = 0xC2000000;
pub const GET_SIP_SVC_VERSION: u32 = 0x8200ff03;
pub const SIP_SVC_VERSION_MAJOR: u32 = 0;
pub const SIP_SVC_VERSION_MINOR: u32 = 2;
pub const SIP_SVC_PASSTHROUGH_VERSION: u32 = (SIP_SVC_VERSION_MAJOR << 16) | SIP_SVC_VERSION_MINOR;
pub const PASS_THROUGH_FW_CMD_ID: u32 = 0xfff;
pub const PM_API_VERSION_1: u32 = 1;
pub const PM_API_VERSION_2: u32 = 2;
pub const PM_API_VERSION_3: u32 = 3;
pub const PM_PINCTRL_PARAM_SET_VERSION: u32 = 2;
pub const PM_ZYNQMP_FAMILY_CODE: u32 = 1;
pub const PM_VERSAL_FAMILY_CODE: u32 = 2;
pub const PM_VERSAL_NET_FAMILY_CODE: u32 = 3;
pub const API_ID_MASK: u32 = 0xff;
pub const MODULE_ID_MASK: u32 = 0xf00;
pub const PLM_MODULE_ID_MASK: u32 = 0xff00;
pub const FIRMWARE_VERSION_MASK: u32 = 0xffff;
pub const TF_A_CLEAR_PM_STATE: u32 = 0xa05;
pub const TF_A_PM_REGISTER_SGI: u32 = 0xa04;
pub const PM_GET_TRUSTZONE_VERSION: u32 = 0xa03;
pub const PM_SET_SUSPEND_MODE: u32 = 0xa02;
pub const GET_CALLBACK_DATA: u32 = 0xa01;
pub const PAYLOAD_ARG_CNT: u32 = 7;
pub const SMC_ARG_CNT_64: u32 = 8;
pub const SMC_ARG_CNT_32: u32 = 13;
pub const CB_ARG_CNT: u32 = 4;
pub const CB_PAYLOAD_SIZE: u32 = CB_ARG_CNT + 1;
pub const ZYNQMP_PM_MAX_QOS: u32 = 100;
pub const GSS_NUM_REGS: u32 = 4;
pub const ZYNQMP_PM_CAPABILITY_ACCESS: u32 = 1;
pub const ZYNQMP_PM_CAPABILITY_CONTEXT: u32 = 2;
pub const ZYNQMP_PM_CAPABILITY_WAKEUP: u32 = 4;
pub const ZYNQMP_PM_CAPABILITY_UNUSABLE: u32 = 8;
pub const PM_LOAD_PDI: u32 = 0x701;
pub const PDI_SRC_DDR: u32 = 0xf;
pub const XILINX_ZYNQMP_PM_FPGA_FULL: u32 = 0;
pub const XILINX_ZYNQMP_PM_FPGA_PARTIAL: u32 = 1;
pub const XILINX_ZYNQMP_PM_FPGA_CONFIG_STAT_OFFSET: u32 = 7;
pub const XILINX_ZYNQMP_PM_FPGA_READ_CONFIG_REG: u32 = 0;
pub const VERSAL_EVENT_ERROR_PMC_ERR1: u32 = 0x28100000;
pub const VERSAL_EVENT_ERROR_PMC_ERR2: u32 = 0x28104000;
pub const VERSAL_EVENT_ERROR_PSM_ERR1: u32 = 0x28108000;
pub const VERSAL_EVENT_ERROR_PSM_ERR2: u32 = 0x2810c000;
pub const VERSAL_NET_EVENT_ERROR_PMC_ERR1: u32 = 0x28100000;
pub const VERSAL_NET_EVENT_ERROR_PMC_ERR2: u32 = 0x28104000;
pub const VERSAL_NET_EVENT_ERROR_PMC_ERR3: u32 = 0x28108000;
pub const VERSAL_NET_EVENT_ERROR_PSM_ERR1: u32 = 0x2810c000;
pub const VERSAL_NET_EVENT_ERROR_PSM_ERR2: u32 = 0x28110000;
pub const VERSAL_NET_EVENT_ERROR_PSM_ERR3: u32 = 0x28114000;
pub const VERSAL_NET_EVENT_ERROR_PSM_ERR4: u32 = 0x28118000;
pub const SD_ITAPDLY: u32 = 0xff180314;
pub const SD_OTAPDLYSEL: u32 = 0xff180318;
pub const XPM_EVENT_ERROR_MASK_DDRMC_CR: u32 = 1 << 18;
pub const XPM_EVENT_ERROR_MASK_DDRMC_NCR: u32 = 1 << 19;
pub const XPM_EVENT_ERROR_MASK_NOC_NCR: u32 = 1 << 13;
pub const XPM_EVENT_ERROR_MASK_NOC_CR: u32 = 1 << 12;
pub const PM_DEV_ALL_PERIPH: u32 = 0x18224fff;
pub const PM_ALL_NOTIFIERS: u32 = 0xffff_ffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_module_id { PM_MODULE_ID=0, XPM_MODULE_ID=2, XSEM_MODULE_ID=3, TF_A_MODULE_ID=10 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_api_cb_id { PM_INIT_SUSPEND_CB=30, PM_ACKNOWLEDGE_CB=31, PM_NOTIFY_CB=32 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_api_id { PM_API_FEATURES=0, PM_GET_API_VERSION=1, PM_GET_NODE_STATUS=3, PM_REGISTER_NOTIFIER=5, PM_FORCE_POWERDOWN=8, PM_REQUEST_WAKEUP=10, PM_SYSTEM_SHUTDOWN=12, PM_REQUEST_NODE=13, PM_RELEASE_NODE=14, PM_SET_REQUIREMENT=15, PM_RESET_ASSERT=17, PM_RESET_GET_STATUS=18, PM_MMIO_WRITE=19, PM_MMIO_READ=20, PM_PM_INIT_FINALIZE=21, PM_FPGA_LOAD=22, PM_FPGA_GET_STATUS=23, PM_GET_CHIPID=24, PM_SECURE_SHA=26, PM_PINCTRL_REQUEST=28, PM_PINCTRL_RELEASE=29, PM_PINCTRL_SET_FUNCTION=31, PM_PINCTRL_CONFIG_PARAM_GET=32, PM_PINCTRL_CONFIG_PARAM_SET=33, PM_IOCTL=34, PM_QUERY_DATA=35, PM_CLOCK_ENABLE=36, PM_CLOCK_DISABLE=37, PM_CLOCK_GETSTATE=38, PM_CLOCK_SETDIVIDER=39, PM_CLOCK_GETDIVIDER=40, PM_CLOCK_SETPARENT=43, PM_CLOCK_GETPARENT=44, PM_FPGA_READ=46, PM_SECURE_AES=47, PM_EFUSE_ACCESS=53, PM_FEATURE_CHECK=63 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_ret_status { XST_PM_SUCCESS=0, XST_PM_INVALID_VERSION=4, XST_PM_NO_FEATURE=19, XST_PM_INVALID_CRC=301, XST_PM_INTERNAL=2000, XST_PM_CONFLICT=2001, XST_PM_NO_ACCESS=2002, XST_PM_INVALID_NODE=2003, XST_PM_DOUBLE_REQ=2004, XST_PM_ABORT_SUSPEND=2005, XST_PM_MULT_USER=2008 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_ioctl_id { IOCTL_GET_RPU_OPER_MODE=0, IOCTL_SET_RPU_OPER_MODE=1, IOCTL_RPU_BOOT_ADDR_CONFIG=2, IOCTL_TCM_COMB_CONFIG=3, IOCTL_SET_TAPDELAY_BYPASS=4, IOCTL_SD_DLL_RESET=6, IOCTL_SET_SD_TAPDELAY=7, IOCTL_SET_PLL_FRAC_MODE=8, IOCTL_GET_PLL_FRAC_MODE=9, IOCTL_SET_PLL_FRAC_DATA=10, IOCTL_GET_PLL_FRAC_DATA=11, IOCTL_WRITE_GGS=12, IOCTL_READ_GGS=13, IOCTL_WRITE_PGGS=14, IOCTL_READ_PGGS=15, IOCTL_SET_BOOT_HEALTH_STATUS=17, IOCTL_OSPI_MUX_SELECT=21, IOCTL_REGISTER_SGI=25, IOCTL_SET_FEATURE_CONFIG=26, IOCTL_GET_FEATURE_CONFIG=27, IOCTL_READ_REG=28, IOCTL_MASK_WRITE_REG=29, IOCTL_SET_SD_CONFIG=30, IOCTL_SET_GEM_CONFIG=31, IOCTL_GET_QOS=34 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_pm_query_data { pub qid: u32, pub arg1: u32, pub arg2: u32, pub arg3: u32 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpu_oper_mode { PM_RPU_MODE_LOCKSTEP=0, PM_RPU_MODE_SPLIT=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpu_boot_mem { PM_RPU_BOOTMEM_LOVEC=0, PM_RPU_BOOTMEM_HIVEC=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpu_tcm_comb { PM_RPU_TCM_SPLIT=0, PM_RPU_TCM_COMB=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_reset_action { PM_RESET_ACTION_RELEASE=0, PM_RESET_ACTION_ASSERT=1, PM_RESET_ACTION_PULSE=2 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_request_ack { ZYNQMP_PM_REQUEST_ACK_NO=1, ZYNQMP_PM_REQUEST_ACK_BLOCKING=2, ZYNQMP_PM_REQUEST_ACK_NON_BLOCKING=3 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_node_id { NODE_SD_0=39, NODE_SD_1=40 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tap_delay_type { PM_TAPDELAY_INPUT=0, PM_TAPDELAY_OUTPUT=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dll_reset_type { PM_DLL_RESET_ASSERT=0, PM_DLL_RESET_RELEASE=1, PM_DLL_RESET_PULSE=2 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_feature_config_id { PM_FEATURE_INVALID=0, PM_FEATURE_OVERTEMP_STATUS=1, PM_FEATURE_OVERTEMP_VALUE=2, PM_FEATURE_EXTWDT_STATUS=3, PM_FEATURE_EXTWDT_VALUE=4 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_sd_config_type { SD_CONFIG_EMMC_SEL=1, SD_CONFIG_BASECLK=2, SD_CONFIG_8BIT=3, SD_CONFIG_FIXED=4 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_gem_config_type { GEM_CONFIG_SGMII_MODE=1, GEM_CONFIG_FIXED=2 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_node_status { PM_NODE_UNUSED=0, PM_NODE_RUNNING=1, PM_NODE_HALT=12 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_query_id { PM_QID_INVALID=0, PM_QID_CLOCK_GET_NAME=1, PM_QID_CLOCK_GET_TOPOLOGY=2, PM_QID_CLOCK_GET_FIXEDFACTOR_PARAMS=3, PM_QID_CLOCK_GET_PARENTS=4, PM_QID_CLOCK_GET_ATTRIBUTES=5, PM_QID_PINCTRL_GET_NUM_PINS=6, PM_QID_PINCTRL_GET_NUM_FUNCTIONS=7, PM_QID_PINCTRL_GET_NUM_FUNCTION_GROUPS=8, PM_QID_PINCTRL_GET_FUNCTION_NAME=9, PM_QID_PINCTRL_GET_FUNCTION_GROUPS=10, PM_QID_PINCTRL_GET_PIN_GROUPS=11, PM_QID_CLOCK_GET_NUM_CLOCKS=12, PM_QID_CLOCK_GET_MAX_DIVISOR=13, PM_QID_PINCTRL_GET_ATTRIBUTES=15 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_config_param { PM_PINCTRL_CONFIG_SLEW_RATE=0, PM_PINCTRL_CONFIG_BIAS_STATUS=1, PM_PINCTRL_CONFIG_PULL_CTRL=2, PM_PINCTRL_CONFIG_SCHMITT_CMOS=3, PM_PINCTRL_CONFIG_DRIVE_STRENGTH=4, PM_PINCTRL_CONFIG_VOLTAGE_STATUS=5, PM_PINCTRL_CONFIG_TRI_STATE=6, PM_PINCTRL_CONFIG_MAX=7 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_slew_rate { PM_PINCTRL_SLEW_RATE_FAST=0, PM_PINCTRL_SLEW_RATE_SLOW=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_bias_status { PM_PINCTRL_BIAS_DISABLE=0, PM_PINCTRL_BIAS_ENABLE=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_pull_ctrl { PM_PINCTRL_BIAS_PULL_DOWN=0, PM_PINCTRL_BIAS_PULL_UP=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_schmitt_cmos { PM_PINCTRL_INPUT_TYPE_CMOS=0, PM_PINCTRL_INPUT_TYPE_SCHMITT=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_drive_strength { PM_PINCTRL_DRIVE_STRENGTH_2MA=0, PM_PINCTRL_DRIVE_STRENGTH_4MA=1, PM_PINCTRL_DRIVE_STRENGTH_8MA=2, PM_PINCTRL_DRIVE_STRENGTH_12MA=3 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_pinctrl_tri_state { PM_PINCTRL_TRI_STATE_DISABLE=0, PM_PINCTRL_TRI_STATE_ENABLE=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_shutdown_type { ZYNQMP_PM_SHUTDOWN_TYPE_SHUTDOWN=0, ZYNQMP_PM_SHUTDOWN_TYPE_RESET=1, ZYNQMP_PM_SHUTDOWN_TYPE_SETSCOPE_ONLY=2 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_pm_shutdown_subtype { ZYNQMP_PM_SHUTDOWN_SUBTYPE_SUBSYSTEM=0, ZYNQMP_PM_SHUTDOWN_SUBTYPE_PS_ONLY=1, ZYNQMP_PM_SHUTDOWN_SUBTYPE_SYSTEM=2 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tap_delay_signal_type { PM_TAPDELAY_NAND_DQS_IN=0, PM_TAPDELAY_NAND_DQS_OUT=1, PM_TAPDELAY_QSPI=2, PM_TAPDELAY_MAX=3 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tap_delay_bypass_ctrl { PM_TAPDELAY_BYPASS_DISABLE=0, PM_TAPDELAY_BYPASS_ENABLE=1 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ospi_mux_select_type { PM_OSPI_MUX_SEL_DMA=0, PM_OSPI_MUX_SEL_LINEAR=1 }

// The reset identifiers are a contiguous ABI range, with the named aliases below.
pub const ZYNQMP_PM_RESET_START: u32 = 1000;
pub const ZYNQMP_PM_RESET_PCIE_CFG: u32 = 1000;
pub const ZYNQMP_PM_RESET_PS_PL3: u32 = 1119;
pub const ZYNQMP_PM_RESET_END: u32 = ZYNQMP_PM_RESET_PS_PL3;

extern "C" {
    pub fn zynqmp_pm_invoke_fn(pm_api_id: u32, ret_payload: *mut u32, num_args: u32, ... ) -> i32;
    pub fn zynqmp_pm_invoke_fw_fn(pm_api_id: u32, ret_payload: *mut u32, num_args: u32, ... ) -> i32;
    pub fn zynqmp_pm_get_api_version(version: *mut u32) -> i32;
    pub fn zynqmp_pm_get_chipid(idcode: *mut u32, version: *mut u32) -> i32;
    pub fn zynqmp_pm_get_family_info(family: *mut u32) -> i32;
    pub fn zynqmp_pm_query_data(qdata: zynqmp_pm_query_data, out: *mut u32) -> i32;
    pub fn zynqmp_pm_clock_enable(clock_id: u32) -> i32;
    pub fn zynqmp_pm_clock_disable(clock_id: u32) -> i32;
    pub fn zynqmp_pm_clock_getstate(clock_id: u32, state: *mut u32) -> i32;
    pub fn zynqmp_pm_clock_setdivider(clock_id: u32, divider: u32) -> i32;
    pub fn zynqmp_pm_clock_getdivider(clock_id: u32, divider: *mut u32) -> i32;
    pub fn zynqmp_pm_clock_setparent(clock_id: u32, parent_id: u32) -> i32;
    pub fn zynqmp_pm_clock_getparent(clock_id: u32, parent_id: *mut u32) -> i32;
    pub fn zynqmp_pm_set_pll_frac_mode(clk_id: u32, mode: u32) -> i32;
    pub fn zynqmp_pm_get_pll_frac_mode(clk_id: u32, mode: *mut u32) -> i32;
    pub fn zynqmp_pm_set_pll_frac_data(clk_id: u32, data: u32) -> i32;
    pub fn zynqmp_pm_get_pll_frac_data(clk_id: u32, data: *mut u32) -> i32;
    pub fn zynqmp_pm_set_sd_tapdelay(node_id: u32, typ: u32, value: u32) -> i32;
    pub fn zynqmp_pm_sd_dll_reset(node_id: u32, typ: u32) -> i32;
    pub fn zynqmp_pm_ospi_mux_select(dev_id: u32, select: u32) -> i32;
    pub fn zynqmp_pm_reset_assert(reset: u32, assert_flag: zynqmp_pm_reset_action) -> i32;
    pub fn zynqmp_pm_reset_get_status(reset: u32, status: *mut u32) -> i32;
    pub fn zynqmp_pm_bootmode_read(ps_mode: *mut u32) -> u32;
    pub fn zynqmp_pm_bootmode_write(ps_mode: u32) -> i32;
    pub fn zynqmp_pm_set_suspend_mode(mode: u32) -> i32;
    pub fn zynqmp_pm_request_node(node: u32, capabilities: u32, qos: u32, ack: zynqmp_pm_request_ack) -> i32;
    pub fn zynqmp_pm_release_node(node: u32) -> i32;
    pub fn zynqmp_pm_set_requirement(node: u32, capabilities: u32, qos: u32, ack: zynqmp_pm_request_ack) -> i32;
    pub fn zynqmp_pm_efuse_access(address: u64, out: *mut u32) -> i32;
    pub fn zynqmp_pm_fpga_load(address: u64, size: u32, flags: u32) -> i32;
    pub fn zynqmp_pm_fpga_get_status(value: *mut u32) -> i32;
    pub fn zynqmp_pm_fpga_get_config_status(value: *mut u32) -> i32;
    pub fn zynqmp_pm_write_ggs(index: u32, value: u32) -> i32;
    pub fn zynqmp_pm_read_ggs(index: u32, value: *mut u32) -> i32;
    pub fn zynqmp_pm_write_pggs(index: u32, value: u32) -> i32;
    pub fn zynqmp_pm_read_pggs(index: u32, value: *mut u32) -> i32;
    pub fn zynqmp_pm_set_tapdelay_bypass(index: u32, value: u32) -> i32;
    pub fn zynqmp_pm_system_shutdown(typ: u32, subtype: u32) -> i32;
    pub fn zynqmp_pm_set_boot_health_status(value: u32) -> i32;
    pub fn zynqmp_pm_pinctrl_request(pin: u32) -> i32;
    pub fn zynqmp_pm_pinctrl_release(pin: u32) -> i32;
    pub fn zynqmp_pm_pinctrl_set_function(pin: u32, id: u32) -> i32;
    pub fn zynqmp_pm_pinctrl_get_config(pin: u32, param: u32, value: *mut u32) -> i32;
    pub fn zynqmp_pm_pinctrl_set_config(pin: u32, param: u32, value: u32) -> i32;
    pub fn zynqmp_pm_load_pdi(src: u32, address: u64) -> i32;
    pub fn zynqmp_pm_register_notifier(node: u32, event: u32, wake: u32, enable: u32) -> i32;
    pub fn zynqmp_pm_feature(api_id: u32) -> i32;
    pub fn zynqmp_pm_is_function_supported(api_id: u32, id: u32) -> i32;
    pub fn zynqmp_pm_set_feature_config(id: pm_feature_config_id, value: u32) -> i32;
    pub fn zynqmp_pm_get_feature_config(id: pm_feature_config_id, payload: *mut u32) -> i32;
    pub fn zynqmp_pm_sec_read_reg(node_id: u32, offset: u32, ret_value: *mut u32) -> i32;
    pub fn zynqmp_pm_sec_mask_write_reg(node_id: u32, offset: u32, mask: u32, value: u32) -> i32;
    pub fn zynqmp_pm_register_sgi(sgi_num: u32, reset: u32) -> i32;
    pub fn zynqmp_pm_force_pwrdwn(target: u32, ack: zynqmp_pm_request_ack) -> i32;
    pub fn zynqmp_pm_request_wake(node: u32, set_addr: bool, address: u64, ack: zynqmp_pm_request_ack) -> i32;
    pub fn zynqmp_pm_get_rpu_mode(node_id: u32, rpu_mode: *mut rpu_oper_mode) -> i32;
    pub fn zynqmp_pm_set_rpu_mode(node_id: u32, rpu_mode: rpu_oper_mode) -> i32;
    pub fn zynqmp_pm_set_tcm_config(node_id: u32, tcm_mode: rpu_tcm_comb) -> i32;
    pub fn zynqmp_pm_get_node_status(node: u32, status: *mut u32, requirements: *mut u32, usage: *mut u32) -> i32;
    pub fn zynqmp_pm_get_rpu_node_status(node: u32, status: *mut u32, requirements: *mut u32, usage: *mut u32) -> i32;
    pub fn zynqmp_pm_start_rpu(node: u32, bootaddr: u64) -> i32;
    pub fn zynqmp_pm_stop_rpu(node: u32) -> i32;
    pub fn zynqmp_pm_set_sd_config(node: u32, config: pm_sd_config_type, value: u32) -> i32;
    pub fn zynqmp_pm_set_gem_config(node: u32, config: pm_gem_config_type, value: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
