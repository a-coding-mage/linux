/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux type definitions.
pub use crate::linux_types::{__le32, size_t, u32};

pub struct qcom_smd_rpm;

pub const QCOM_SMD_RPM_ACTIVE_STATE: i32 = 0;
pub const QCOM_SMD_RPM_SLEEP_STATE: i32 = 1;
pub const QCOM_SMD_RPM_STATE_NUM: i32 = 2;

/*
 * Constants used for addressing resources in the RPM.
 */
pub const QCOM_SMD_RPM_BBYB: u32 = 0x6279_6262;
pub const QCOM_SMD_RPM_BOBB: u32 = 0x6262_6f62;
pub const QCOM_SMD_RPM_BOOST: u32 = 0x6174_7362;
pub const QCOM_SMD_RPM_BUS_CLK: u32 = 0x316b_6c63;
pub const QCOM_SMD_RPM_BUS_MASTER: u32 = 0x7361_6d62;
pub const QCOM_SMD_RPM_BUS_SLAVE: u32 = 0x766c_7362;
pub const QCOM_SMD_RPM_CLK_BUF_A: u32 = 0x616b_6c63;
pub const QCOM_SMD_RPM_LDOA: u32 = 0x616f_646c;
pub const QCOM_SMD_RPM_LDOB: u32 = 0x626f_646c;
pub const QCOM_SMD_RPM_LDOE: u32 = 0x656f_646c;
pub const QCOM_SMD_RPM_RWCX: u32 = 0x7863_7772;
pub const QCOM_SMD_RPM_RWMX: u32 = 0x786d_7772;
pub const QCOM_SMD_RPM_RWLC: u32 = 0x636c_7772;
pub const QCOM_SMD_RPM_RWLM: u32 = 0x6d6c_7772;
pub const QCOM_SMD_RPM_MEM_CLK: u32 = 0x326b_6c63;
pub const QCOM_SMD_RPM_MISC_CLK: u32 = 0x306b_6c63;
pub const QCOM_SMD_RPM_NCPA: u32 = 0x6170_636e;
pub const QCOM_SMD_RPM_NCPB: u32 = 0x6270_636e;
pub const QCOM_SMD_RPM_OCMEM_PWR: u32 = 0x706d_636f;
pub const QCOM_SMD_RPM_QPIC_CLK: u32 = 0x6369_7071;
pub const QCOM_SMD_RPM_QUP_CLK: u32 = 0x7075_71;
pub const QCOM_SMD_RPM_SMPA: u32 = 0x6170_6d73;
pub const QCOM_SMD_RPM_SMPB: u32 = 0x6270_6d73;
pub const QCOM_SMD_RPM_SMPE: u32 = 0x6570_6d73;
pub const QCOM_SMD_RPM_SPDM: u32 = 0x6370_7362;
pub const QCOM_SMD_RPM_VSA: u32 = 0x6173_76;
pub const QCOM_SMD_RPM_MMAXI_CLK: u32 = 0x6978_6d6d;
pub const QCOM_SMD_RPM_IPA_CLK: u32 = 0x6170_69;
pub const QCOM_SMD_RPM_CE_CLK: u32 = 0x6563;
pub const QCOM_SMD_RPM_AGGR_CLK: u32 = 0x7267_6761;
pub const QCOM_SMD_RPM_HWKM_CLK: u32 = 0x6d6b_7768;
pub const QCOM_SMD_RPM_PKA_CLK: u32 = 0x616b_70;
pub const QCOM_SMD_RPM_MCFG_CLK: u32 = 0x6766_636d;

pub const QCOM_RPM_KEY_SOFTWARE_ENABLE: u32 = 0x6e65_7773;
pub const QCOM_RPM_KEY_PIN_CTRL_CLK_BUFFER_ENABLE_KEY: u32 = 0x6263_6370;
pub const QCOM_RPM_SMD_KEY_RATE: u32 = 0x7a48_4b;
pub const QCOM_RPM_SMD_KEY_ENABLE: u32 = 0x6261_6e45;
pub const QCOM_RPM_SMD_KEY_STATE: u32 = 0x5441_5453;
pub const QCOM_RPM_SCALING_ENABLE_ID: u32 = 0x2;

#[repr(C)]
pub struct clk_smd_rpm_req {
    pub key: __le32,
    pub nbytes: __le32,
    pub value: __le32,
}

unsafe extern "C" {
    pub fn qcom_rpm_smd_write(
        rpm: *mut qcom_smd_rpm,
        state: i32,
        resource_type: u32,
        resource_id: u32,
        buf: *mut core::ffi::c_void,
        count: size_t,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
