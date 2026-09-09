/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2024 NXP
 */

/* C dependencies: linux/bitfield.h, linux/errno.h, linux/scmi_imx_protocol.h,
 * and linux/types.h. */

pub const SCMI_IMX95_CTRL_PDM_CLK_SEL: u32 = 0; /* AON PDM clock sel */
pub const SCMI_IMX95_CTRL_MQS1_SETTINGS: u32 = 1; /* AON MQS settings */
pub const SCMI_IMX95_CTRL_SAI1_MCLK: u32 = 2; /* AON SAI1 MCLK */
pub const SCMI_IMX95_CTRL_SAI3_MCLK: u32 = 3; /* WAKE SAI3 MCLK */
pub const SCMI_IMX95_CTRL_SAI4_MCLK: u32 = 4; /* WAKE SAI4 MCLK */
pub const SCMI_IMX95_CTRL_SAI5_MCLK: u32 = 5; /* WAKE SAI5 MCLK */

pub const SCMI_IMX94_CTRL_PDM_CLK_SEL: u32 = 0; /* AON PDM clock sel */
pub const SCMI_IMX94_CTRL_MQS1_SETTINGS: u32 = 1; /* AON MQS settings */
pub const SCMI_IMX94_CTRL_MQS2_SETTINGS: u32 = 2; /* WAKE MQS settings */
pub const SCMI_IMX94_CTRL_SAI1_MCLK: u32 = 3; /* AON SAI1 MCLK */
pub const SCMI_IMX94_CTRL_SAI2_MCLK: u32 = 4; /* WAKE SAI2 MCLK */
pub const SCMI_IMX94_CTRL_SAI3_MCLK: u32 = 5; /* WAKE SAI3 MCLK */
pub const SCMI_IMX94_CTRL_SAI4_MCLK: u32 = 6; /* WAKE SAI4 MCLK */

pub const SCMI_IMX952_CTRL_BYPASS_AUDMIX: u32 = 8; /* WAKE AUDMIX */

/* CONFIG_IMX_SCMI_MISC_DRV controls whether these are external functions or
 * unsupported inline fallbacks in the C header. */
#[cfg(CONFIG_IMX_SCMI_MISC_DRV)]
unsafe extern "C" {
    pub fn scmi_imx_misc_ctrl_get(id: u32, num: *mut u32, val: *mut u32) -> i32;
    pub fn scmi_imx_misc_ctrl_set(id: u32, val: u32) -> i32;
}

#[cfg(not(CONFIG_IMX_SCMI_MISC_DRV))]
pub unsafe fn scmi_imx_misc_ctrl_get(_id: u32, _num: *mut u32, _val: *mut u32) -> i32 {
    -95
}

#[cfg(not(CONFIG_IMX_SCMI_MISC_DRV))]
pub unsafe fn scmi_imx_misc_ctrl_set(_id: u32, _val: u32) -> i32 {
    -95
}

#[cfg(CONFIG_IMX_SCMI_CPU_DRV)]
unsafe extern "C" {
    pub fn scmi_imx_cpu_start(cpuid: u32, start: bool) -> i32;
    pub fn scmi_imx_cpu_started(cpuid: u32, started: *mut bool) -> i32;
    pub fn scmi_imx_cpu_reset_vector_set(
        cpuid: u32,
        vector: u64,
        start: bool,
        boot: bool,
        resume: bool,
    ) -> i32;
}

#[cfg(not(CONFIG_IMX_SCMI_CPU_DRV))]
pub unsafe fn scmi_imx_cpu_start(_cpuid: u32, _start: bool) -> i32 { -95 }

#[cfg(not(CONFIG_IMX_SCMI_CPU_DRV))]
pub unsafe fn scmi_imx_cpu_started(_cpuid: u32, _started: *mut bool) -> i32 { -95 }

#[cfg(not(CONFIG_IMX_SCMI_CPU_DRV))]
pub unsafe fn scmi_imx_cpu_reset_vector_set(
    _cpuid: u32,
    _vector: u64,
    _start: bool,
    _boot: bool,
    _resume: bool,
) -> i32 { -95 }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum scmi_imx_lmm_op {
    SCMI_IMX_LMM_BOOT,
    SCMI_IMX_LMM_POWER_ON,
    SCMI_IMX_LMM_SHUTDOWN,
}

/* For shutdown pperation */
pub const SCMI_IMX_LMM_OP_FORCEFUL: u32 = 0;
pub const SCMI_IMX_LMM_OP_GRACEFUL: u32 = 1;

#[cfg(CONFIG_IMX_SCMI_LMM_DRV)]
unsafe extern "C" {
    pub fn scmi_imx_lmm_operation(lmid: u32, op: scmi_imx_lmm_op, flags: u32) -> i32;
    pub fn scmi_imx_lmm_info(lmid: u32, info: *mut scmi_imx_lmm_info) -> i32;
    pub fn scmi_imx_lmm_reset_vector_set(lmid: u32, cpuid: u32, flags: u32, vector: u64) -> i32;
}

#[cfg(not(CONFIG_IMX_SCMI_LMM_DRV))]
pub unsafe fn scmi_imx_lmm_operation(_lmid: u32, _op: scmi_imx_lmm_op, _flags: u32) -> i32 { -95 }

#[cfg(not(CONFIG_IMX_SCMI_LMM_DRV))]
pub unsafe fn scmi_imx_lmm_info(_lmid: u32, _info: *mut scmi_imx_lmm_info) -> i32 { -95 }

#[cfg(not(CONFIG_IMX_SCMI_LMM_DRV))]
pub unsafe fn scmi_imx_lmm_reset_vector_set(
    _lmid: u32,
    _cpuid: u32,
    _flags: u32,
    _vector: u64,
) -> i32 { -95 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
