/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const PSP_GFX_CMD_BUF_VERSION: u32 = 0x00000001;
pub const GFX_CMD_STATUS_MASK: u32 = 0x0000FFFF;
pub const GFX_CMD_ID_MASK: u32 = 0x000F0000;
pub const GFX_CMD_RESERVED_MASK: u32 = 0x7FF00000;
pub const GFX_CMD_RESPONSE_MASK: u32 = 0x80000000;
pub const C2PMSG_CMD_GFX_USB_PD_FW_VER: u32 = 0x2000000;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psp_gfx_crtl_cmd_id {
    GFX_CTRL_CMD_ID_INIT_RBI_RING = 0x00010000,
    GFX_CTRL_CMD_ID_INIT_GPCOM_RING = 0x00020000,
    GFX_CTRL_CMD_ID_DESTROY_RINGS = 0x00030000,
    GFX_CTRL_CMD_ID_CAN_INIT_RINGS = 0x00040000,
    GFX_CTRL_CMD_ID_ENABLE_INT = 0x00050000,
    GFX_CTRL_CMD_ID_DISABLE_INT = 0x00060000,
    GFX_CTRL_CMD_ID_MODE1_RST = 0x00070000,
    GFX_CTRL_CMD_ID_GBR_IH_SET = 0x00080000,
    GFX_CTRL_CMD_ID_CONSUME_CMD = 0x00090000,
    GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING = 0x000C0000,
    GFX_CTRL_CMD_ID_MAX = 0x000F0000,
}

#[repr(C)]
pub struct psp_gfx_ctrl {
    pub cmd_resp: u32,
    pub rbi_wptr: u32,
    pub rbi_rptr: u32,
    pub gpcom_wptr: u32,
    pub gpcom_rptr: u32,
    pub ring_addr_lo: u32,
    pub ring_addr_hi: u32,
    pub ring_buf_size: u32,
}

pub const GFX_FLAG_RESPONSE: u32 = 0x80000000;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psp_gfx_cmd_id {
    GFX_CMD_ID_LOAD_TA = 1, GFX_CMD_ID_UNLOAD_TA = 2, GFX_CMD_ID_INVOKE_CMD = 3,
    GFX_CMD_ID_LOAD_ASD = 4, GFX_CMD_ID_SETUP_TMR = 5, GFX_CMD_ID_LOAD_IP_FW = 6,
    GFX_CMD_ID_DESTROY_TMR = 7, GFX_CMD_ID_SAVE_RESTORE = 8, GFX_CMD_ID_SETUP_VMR = 9,
    GFX_CMD_ID_DESTROY_VMR = 0xA, GFX_CMD_ID_PROG_REG = 0xB,
    GFX_CMD_ID_GET_FW_ATTESTATION = 0xF, GFX_CMD_ID_LOAD_TOC = 0x20,
    GFX_CMD_ID_AUTOLOAD_RLC = 0x21, GFX_CMD_ID_BOOT_CFG = 0x22,
    GFX_CMD_ID_SRIOV_SPATIAL_PART = 0x27, GFX_CMD_ID_CONFIG_SQ_PERFMON = 0x46,
    GFX_CMD_ID_FB_NPS_MODE = 0x48, GFX_CMD_ID_PERF_HW = 0x4C,
    GFX_CMD_ID_FB_FW_RESERV_ADDR = 0x50, GFX_CMD_ID_FB_FW_RESERV_EXT_ADDR = 0x51,
    GFX_CMD_ID_SET_MMHUB_ECO_SEC_LEVEL = 0x5D,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psp_gfx_boot_config_cmd { BOOTCFG_CMD_SET = 1, BOOTCFG_CMD_GET = 2, BOOTCFG_CMD_INVALIDATE = 3 }
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psp_gfx_boot_config { BOOT_CONFIG_GECC = 1 }

#[repr(C)] pub struct psp_gfx_cmd_load_ta { pub app_phy_addr_lo:u32, pub app_phy_addr_hi:u32, pub app_len:u32, pub cmd_buf_phy_addr_lo:u32, pub cmd_buf_phy_addr_hi:u32, pub cmd_buf_len:u32 }
#[repr(C)] pub struct psp_gfx_cmd_unload_ta { pub session_id:u32 }
#[repr(C)] pub struct psp_gfx_buf_desc { pub buf_phy_addr_lo:u32, pub buf_phy_addr_hi:u32, pub buf_size:u32 }
pub const GFX_BUF_MAX_DESC: usize = 64;
#[repr(C)] pub struct psp_gfx_buf_list { pub num_desc:u32, pub total_size:u32, pub buf_desc:[psp_gfx_buf_desc; GFX_BUF_MAX_DESC] }
#[repr(C)] pub struct psp_gfx_cmd_invoke_cmd { pub session_id:u32, pub ta_cmd_id:u32, pub buf:psp_gfx_buf_list }

#[repr(C)] pub struct psp_gfx_cmd_setup_tmr { pub buf_phy_addr_lo:u32, pub buf_phy_addr_hi:u32, pub buf_size:u32, pub flags:psp_gfx_cmd_setup_tmr_flags, pub system_phy_addr_lo:u32, pub system_phy_addr_hi:u32 }
#[repr(C)] pub union psp_gfx_cmd_setup_tmr_flags { pub bitfield: psp_gfx_cmd_setup_tmr_bitfield, pub tmr_flags:u32 }
#[repr(C)] pub struct psp_gfx_cmd_setup_tmr_bitfield { pub sriov_enabled:u32, pub virt_phy_addr:u32, pub reserved:u32 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psp_gfx_fw_type {
    GFX_FW_TYPE_NONE=0, GFX_FW_TYPE_CP_ME=1, GFX_FW_TYPE_CP_PFP=2, GFX_FW_TYPE_CP_CE=3, GFX_FW_TYPE_CP_MEC=4, GFX_FW_TYPE_CP_MEC_ME1=5, GFX_FW_TYPE_CP_MEC_ME2=6, GFX_FW_TYPE_RLC_V=7, GFX_FW_TYPE_RLC_G=8, GFX_FW_TYPE_SDMA0=9, GFX_FW_TYPE_SDMA1=10, GFX_FW_TYPE_DMCU_ERAM=11, GFX_FW_TYPE_DMCU_ISR=12, GFX_FW_TYPE_VCN=13, GFX_FW_TYPE_UVD=14, GFX_FW_TYPE_VCE=15, GFX_FW_TYPE_ISP=16, GFX_FW_TYPE_ACP=17, GFX_FW_TYPE_SMU=18, GFX_FW_TYPE_MMSCH=19,
    GFX_FW_TYPE_RLC_RESTORE_LIST_GPM_MEM=20, GFX_FW_TYPE_RLC_RESTORE_LIST_SRM_MEM=21, GFX_FW_TYPE_RLC_RESTORE_LIST_SRM_CNTL=22, GFX_FW_TYPE_UVD1=23, GFX_FW_TYPE_TOC=24, GFX_FW_TYPE_RLC_P=25, GFX_FW_TYPE_RLC_IRAM=26, GFX_FW_TYPE_GLOBAL_TAP_DELAYS=27, GFX_FW_TYPE_SE0_TAP_DELAYS=28, GFX_FW_TYPE_SE1_TAP_DELAYS=29, GFX_FW_TYPE_GLOBAL_SE0_SE1_SKEW_DELAYS=30, GFX_FW_TYPE_SDMA0_JT=31, GFX_FW_TYPE_SDMA1_JT=32, GFX_FW_TYPE_CP_MES=33, GFX_FW_TYPE_MES_STACK=34, GFX_FW_TYPE_RLC_SRM_DRAM_SR=35, GFX_FW_TYPE_RLCG_SCRATCH_SR=36, GFX_FW_TYPE_RLCP_SCRATCH_SR=37, GFX_FW_TYPE_RLCV_SCRATCH_SR=38, GFX_FW_TYPE_RLX6_DRAM_SR=39, GFX_FW_TYPE_SDMA0_PG_CONTEXT=40, GFX_FW_TYPE_SDMA1_PG_CONTEXT=41, GFX_FW_TYPE_GLOBAL_MUX_SELECT_RAM=42, GFX_FW_TYPE_SE0_MUX_SELECT_RAM=43, GFX_FW_TYPE_SE1_MUX_SELECT_RAM=44, GFX_FW_TYPE_ACCUM_CTRL_RAM=45, GFX_FW_TYPE_RLCP_CAM=46, GFX_FW_TYPE_RLC_SPP_CAM_EXT=47, GFX_FW_TYPE_RLC_DRAM_BOOT=48, GFX_FW_TYPE_VCN0_RAM=49, GFX_FW_TYPE_VCN1_RAM=50, GFX_FW_TYPE_DMUB=51, GFX_FW_TYPE_SDMA2=52, GFX_FW_TYPE_SDMA3=53, GFX_FW_TYPE_SDMA4=54, GFX_FW_TYPE_SDMA5=55, GFX_FW_TYPE_SDMA6=56, GFX_FW_TYPE_SDMA7=57, GFX_FW_TYPE_VCN1=58, GFX_FW_TYPE_CAP=62, GFX_FW_TYPE_SE2_TAP_DELAYS=65, GFX_FW_TYPE_SE3_TAP_DELAYS=66, GFX_FW_TYPE_REG_LIST=67, GFX_FW_TYPE_IMU_I=68, GFX_FW_TYPE_IMU_D=69, GFX_FW_TYPE_LSDMA=70, GFX_FW_TYPE_SDMA_UCODE_TH0=71, GFX_FW_TYPE_SDMA_UCODE_TH1=72, GFX_FW_TYPE_PPTABLE=73, GFX_FW_TYPE_DISCRETE_USB4=74, GFX_FW_TYPE_TA=75, GFX_FW_TYPE_RS64_MES=76, GFX_FW_TYPE_RS64_MES_STACK=77, GFX_FW_TYPE_RS64_KIQ=78, GFX_FW_TYPE_RS64_KIQ_STACK=79, GFX_FW_TYPE_ISP_DATA=80, GFX_FW_TYPE_CP_MES_KIQ=81, GFX_FW_TYPE_MES_KIQ_STACK=82, GFX_FW_TYPE_UMSCH_DATA=83, GFX_FW_TYPE_UMSCH_UCODE=84, GFX_FW_TYPE_UMSCH_CMD_BUFFER=85, GFX_FW_TYPE_USB_DP_COMBO_PHY=86, GFX_FW_TYPE_RS64_PFP=87, GFX_FW_TYPE_RS64_ME=88, GFX_FW_TYPE_RS64_MEC=89, GFX_FW_TYPE_RS64_PFP_P0_STACK=90, GFX_FW_TYPE_RS64_PFP_P1_STACK=91, GFX_FW_TYPE_RS64_ME_P0_STACK=92, GFX_FW_TYPE_RS64_ME_P1_STACK=93, GFX_FW_TYPE_RS64_MEC_P0_STACK=94, GFX_FW_TYPE_RS64_MEC_P1_STACK=95, GFX_FW_TYPE_RS64_MEC_P2_STACK=96, GFX_FW_TYPE_RS64_MEC_P3_STACK=97, GFX_FW_TYPE_RLX6_UCODE_CORE1=98, GFX_FW_TYPE_RLX6_DRAM_BOOT_CORE1=99, GFX_FW_TYPE_VPEC_FW1=100, GFX_FW_TYPE_VPEC_FW2=101, GFX_FW_TYPE_VPE=102, GFX_FW_TYPE_JPEG_RAM=128, GFX_FW_TYPE_P2S_TABLE=129, GFX_FW_TYPE_MAX=130,
}

#[repr(C)] pub struct psp_gfx_cmd_load_ip_fw { pub fw_phy_addr_lo:u32, pub fw_phy_addr_hi:u32, pub fw_size:u32, pub fw_type:psp_gfx_fw_type }
#[repr(C)] pub struct psp_gfx_cmd_save_restore_ip_fw { pub save_fw:u32, pub save_restore_addr_lo:u32, pub save_restore_addr_hi:u32, pub buf_size:u32, pub fw_type:psp_gfx_fw_type }
#[repr(C)] pub struct psp_gfx_cmd_reg_prog { pub reg_value:u32, pub reg_id:u32 }
#[repr(C)] pub struct psp_gfx_cmd_load_toc { pub toc_phy_addr_lo:u32, pub toc_phy_addr_hi:u32, pub toc_size:u32 }
#[repr(C)] pub struct psp_gfx_cmd_boot_cfg { pub timestamp:u32, pub sub_cmd:psp_gfx_boot_config_cmd, pub boot_config:u32, pub boot_config_valid:u32 }
#[repr(C)] pub struct psp_gfx_cmd_sriov_spatial_part { pub mode:u32, pub override_ips:u32, pub override_xcds_avail:u32, pub override_this_aid:u32 }
#[repr(C)] pub struct psp_gfx_cmd_config_sq_perfmon { pub gfx_xcp_mask:u32, pub core_override:u8, pub reg_override:u8, pub perfmon_override:u8, pub reserved:[u8;5] }
#[repr(C)] pub struct psp_gfx_cmd_fb_memory_part { pub mode:u32, pub resvd:u32 }
#[repr(C)] pub struct psp_gfx_cmd_req_perf_hw { pub req:u32, pub ptl_state:u32, pub pref_format1:u32, pub pref_format2:u32 }

#[repr(C)] pub union psp_gfx_commands { pub cmd_load_ta:psp_gfx_cmd_load_ta, pub cmd_unload_ta:psp_gfx_cmd_unload_ta, pub cmd_invoke_cmd:psp_gfx_cmd_invoke_cmd, pub cmd_setup_tmr:psp_gfx_cmd_setup_tmr, pub cmd_load_ip_fw:psp_gfx_cmd_load_ip_fw, pub cmd_save_restore_ip_fw:psp_gfx_cmd_save_restore_ip_fw, pub cmd_setup_reg_prog:psp_gfx_cmd_reg_prog, pub cmd_setup_vmr:psp_gfx_cmd_setup_tmr, pub cmd_load_toc:psp_gfx_cmd_load_toc, pub boot_cfg:psp_gfx_cmd_boot_cfg, pub cmd_spatial_part:psp_gfx_cmd_sriov_spatial_part, pub config_sq_perfmon:psp_gfx_cmd_config_sq_perfmon, pub cmd_memory_part:psp_gfx_cmd_fb_memory_part, pub cmd_req_perf_hw:psp_gfx_cmd_req_perf_hw }
#[repr(C)] pub struct psp_gfx_uresp_reserved { pub reserved:[u32;8] }
#[repr(C)] pub struct psp_gfx_uresp_fwar_db_info { pub fwar_db_addr_lo:u32, pub fwar_db_addr_hi:u32 }
#[repr(C)] pub struct psp_gfx_uresp_bootcfg { pub boot_cfg:u32 }
#[repr(C)] pub struct psp_gfx_uresp_fw_reserve_info { pub reserve_base_address_hi:u32, pub reserve_base_address_lo:u32, pub reserve_size:u32 }
#[repr(C)] pub struct psp_gfx_uresp_perf_hw { pub resp:u32, pub ptl_state:u32, pub pref_format1:u32, pub pref_format2:u32 }
#[repr(C)] pub union psp_gfx_uresp { pub reserved:psp_gfx_uresp_reserved, pub boot_cfg:psp_gfx_uresp_bootcfg, pub fwar_db_info:psp_gfx_uresp_fwar_db_info, pub fw_reserve_info:psp_gfx_uresp_fw_reserve_info, pub perf_hw_info:psp_gfx_uresp_perf_hw }
#[repr(C)] pub struct psp_gfx_resp { pub status:u32, pub session_id:u32, pub fw_addr_lo:u32, pub fw_addr_hi:u32, pub tmr_size:u32, pub reserved:[u32;11], pub uresp:psp_gfx_uresp }
#[repr(C)] pub struct psp_gfx_cmd_resp { pub buf_size:u32, pub buf_version:u32, pub cmd_id:u32, pub resp_buf_addr_lo:u32, pub resp_buf_addr_hi:u32, pub resp_offset:u32, pub resp_buf_size:u32, pub cmd:psp_gfx_commands, pub reserved_1:[u8;864 - core::mem::size_of::<psp_gfx_commands>() - 28], pub resp:psp_gfx_resp, pub reserved_2:[u8;1024 - 864 - core::mem::size_of::<psp_gfx_resp>()] }
pub const FRAME_TYPE_DESTROY: u8 = 1;
#[repr(C)] pub struct psp_gfx_rb_frame { pub cmd_buf_addr_lo:u32, pub cmd_buf_addr_hi:u32, pub cmd_buf_size:u32, pub fence_addr_lo:u32, pub fence_addr_hi:u32, pub fence_value:u32, pub sid_lo:u32, pub sid_hi:u32, pub vmid:u8, pub frame_type:u8, pub reserved1:[u8;2], pub reserved2:[u32;7] }
pub const PSP_ERR_UNKNOWN_COMMAND: u32 = 0x00000100;
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tee_error_code { TEE_SUCCESS=0, TEE_ERROR_CANCEL=0xFFFF0002, TEE_ERROR_NOT_SUPPORTED=0xFFFF000A }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
