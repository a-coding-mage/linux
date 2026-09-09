/* SPDX-License-Identifier: GPL-2.0 */
/* Direct low-level Rust translation of linux/nvme.h. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;

pub const NVMF_NQN_FIELD_LEN: usize = 256;
pub const NVMF_NQN_SIZE: usize = 223;
pub const NVMF_TRSVCID_SIZE: usize = 32;
pub const NVMF_TRADDR_SIZE: usize = 256;
pub const NVMF_TSAS_SIZE: usize = 256;
pub const NVME_DISC_SUBSYS_NAME: &str = "nqn.2014-08.org.nvmexpress.discovery";
pub const NVME_NSID_ALL: u32 = 0xffff_ffff;
pub const NVME_SUBSYS_RESET: u32 = 0x4e56_4d65;

macro_rules! nvme_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
nvme_consts! {
    NVME_NQN_DISC=1, NVME_NQN_NVME=2, NVME_NQN_CURR=3,
    NVME_CTRL_IO=1, NVME_CTRL_DISC=2, NVME_CTRL_ADMIN=3,
    NVME_DCTYPE_NOT_REPORTED=0, NVME_DCTYPE_DDC=1, NVME_DCTYPE_CDC=2,
    NVMF_ADDR_FAMILY_PCI=0, NVMF_ADDR_FAMILY_IP4=1, NVMF_ADDR_FAMILY_IP6=2,
    NVMF_ADDR_FAMILY_IB=3, NVMF_ADDR_FAMILY_FC=4, NVMF_ADDR_FAMILY_LOOP=254,
    NVMF_TRTYPE_PCI=0, NVMF_TRTYPE_RDMA=1, NVMF_TRTYPE_FC=2, NVMF_TRTYPE_TCP=3,
    NVMF_TRTYPE_LOOP=254, NVMF_TREQ_NOT_SPECIFIED=0, NVMF_TREQ_REQUIRED=1,
    NVMF_TREQ_NOT_REQUIRED=2, NVMF_TREQ_DISABLE_SQFLOW=1<<2,
    NVME_AQ_DEPTH=32, NVME_NR_AEN_COMMANDS=1, NVME_AQ_BLK_MQ_DEPTH=31,
    NVME_AQ_MQ_TAG_DEPTH=30, NVME_ADM_SQES=6, NVME_NVM_IOSQES=6, NVME_NVM_IOCQES=4,
    NVME_REG_CAP=0x0000, NVME_REG_VS=0x0008, NVME_REG_INTMS=0x000c,
    NVME_REG_INTMC=0x0010, NVME_REG_CC=0x0014, NVME_REG_CSTS=0x001c,
    NVME_REG_NSSR=0x0020, NVME_REG_AQA=0x0024, NVME_REG_ASQ=0x0028,
    NVME_REG_ACQ=0x0030, NVME_REG_CMBLOC=0x0038, NVME_REG_CMBSZ=0x003c,
    NVME_REG_DBS=0x1000, NVME_CMBSZ_SQS=1, NVME_CMBSZ_CQS=2, NVME_CMBSZ_LISTS=4,
    NVME_CC_ENABLE=1, NVME_CC_CSS_SHIFT=4, NVME_CC_MPS_SHIFT=7,
    NVME_CSTS_RDY=1, NVME_CSTS_CFS=2, NVME_CSTS_SHST_MASK=3<<2,
    NVME_CAP_CSS_NVM=1, NVME_CAP_CSS_CSI=1<<6,
    NVME_SGL_FMT_ADDRESS=0, NVME_SGL_FMT_OFFSET=1, NVME_SGL_FMT_TRANSPORT_A=0x0a,
    NVME_SGL_FMT_INVALIDATE=0x0f, NVME_SGL_FMT_DATA_DESC=0, NVME_SGL_FMT_SEG_DESC=2,
    NVME_SGL_FMT_LAST_SEG_DESC=3, NVME_KEY_SGL_FMT_DATA_DESC=4,
    NVME_TRANSPORT_SGL_DATA_DESC=5,
    nvme_cmd_flush=0, nvme_cmd_write=1, nvme_cmd_read=2, nvme_cmd_write_uncor=4,
    nvme_cmd_compare=5, nvme_cmd_write_zeroes=8, nvme_cmd_dsm=9, nvme_cmd_verify=0xc,
    nvme_cmd_resv_register=0xd, nvme_cmd_resv_report=0xe, nvme_cmd_resv_acquire=0x11,
    nvme_cmd_resv_release=0x15, nvme_cmd_zone_mgmt_send=0x79,
    nvme_cmd_zone_mgmt_recv=0x7a, nvme_cmd_zone_append=0x7d, nvme_cmd_vendor_start=0x80,
    nvme_fabrics_command=0x7f, NVME_CNTLID_MIN=1, NVME_CNTLID_MAX=0xffef,
    NVME_CNTLID_DYNAMIC=0xffff, NVME_SCT_GENERIC=0, NVME_SC_SUCCESS=0,
    NVME_SC_INVALID_OPCODE=1, NVME_SC_INVALID_FIELD=2, NVME_SC_MASK=0xff,
    NVME_SCT_MASK=0x700, NVME_SCT_SC_MASK=0x7ff, NVME_STATUS_CRD=0x1800,
    NVME_STATUS_MORE=0x2000, NVME_STATUS_DNR=0x4000
}

#[inline] pub const fn NVME_CAP_MQES(cap: u64) -> u64 { cap & 0xffff }
#[inline] pub const fn NVME_CAP_TIMEOUT(cap: u64) -> u64 { (cap >> 24) & 0xff }
#[inline] pub const fn NVME_CAP_STRIDE(cap: u64) -> u64 { (cap >> 32) & 0xf }
#[inline] pub const fn NVME_SCT(status: u16) -> u16 { (status >> 8) & 7 }

#[repr(C)] #[derive(Copy, Clone)] pub struct nvme_id_power_state { pub max_power: __le16, pub rsvd2: __u8, pub flags: __u8, pub entry_lat: __le32, pub exit_lat: __le32, pub read_tput: __u8, pub read_lat: __u8, pub write_tput: __u8, pub write_lat: __u8, pub idle_power: __le16, pub idle_scale: __u8, pub rsvd19: __u8, pub active_power: __le16, pub active_work_scale: __u8, pub rsvd23: [__u8;9] }
#[repr(C)] pub struct nvme_lbaf { pub ms: __le16, pub ds: __u8, pub rp: __u8 }
#[repr(C)] pub struct nvme_sgl_desc { pub addr: __le64, pub length: __le32, pub rsvd: [__u8;3], pub type_: __u8 }
#[repr(C)] pub struct nvme_keyed_sgl_desc { pub addr: __le64, pub length: [__u8;3], pub key: [__u8;4], pub type_: __u8 }
#[repr(C)] pub union nvme_data_ptr { pub prp: [__le64;2], pub sgl: nvme_sgl_desc, pub ksgl: nvme_keyed_sgl_desc }
#[repr(C)] pub struct nvme_common_command { pub opcode: __u8, pub flags: __u8, pub command_id: __u16, pub nsid: __le32, pub cdw2: [__le32;2], pub metadata: __le64, pub dptr: nvme_data_ptr, pub cdw10: __le32, pub cdw11: __le32, pub cdw12: __le32, pub cdw13: __le32, pub cdw14: __le32, pub cdw15: __le32 }
#[repr(C)] pub struct nvme_command { pub common: nvme_common_command }

#[inline] pub unsafe fn nvme_is_fabrics(cmd: *const nvme_command) -> bool { (*cmd).common.opcode == nvme_fabrics_command as u8 }
#[inline] pub unsafe fn nvme_is_write(cmd: *const nvme_command) -> bool { (*cmd).common.opcode & 1 != 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
