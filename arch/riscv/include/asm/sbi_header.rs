/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Regents of the University of California
 * Copyright (c) 2020 Western Digital Corporation or its affiliates.
 */

// C header dependencies: linux/types.h, linux/cpumask.h, linux/jump_label.h.
// Items guarded by CONFIG_RISCV_SBI are retained here unconditionally; build
// configuration may omit them when that configuration is disabled.

#[repr(i32)]
pub enum sbi_ext_id {
    SBI_EXT_0_1_SET_TIMER = 0x0,
    SBI_EXT_0_1_CONSOLE_PUTCHAR = 0x1,
    SBI_EXT_0_1_CONSOLE_GETCHAR = 0x2,
    SBI_EXT_0_1_CLEAR_IPI = 0x3,
    SBI_EXT_0_1_SEND_IPI = 0x4,
    SBI_EXT_0_1_REMOTE_FENCE_I = 0x5,
    SBI_EXT_0_1_REMOTE_SFENCE_VMA = 0x6,
    SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID = 0x7,
    SBI_EXT_0_1_SHUTDOWN = 0x8,
    SBI_EXT_BASE = 0x10,
    SBI_EXT_TIME = 0x54494D45,
    SBI_EXT_IPI = 0x735049,
    SBI_EXT_RFENCE = 0x52464E43,
    SBI_EXT_HSM = 0x48534D,
    SBI_EXT_SRST = 0x53525354,
    SBI_EXT_SUSP = 0x53555350,
    SBI_EXT_PMU = 0x504D55,
    SBI_EXT_DBCN = 0x4442434E,
    SBI_EXT_STA = 0x535441,
    SBI_EXT_NACL = 0x4E41434C,
    SBI_EXT_FWFT = 0x46574654,
    SBI_EXT_MPXY = 0x4D505859,
    SBI_EXT_DBTR = 0x44425452,
    // Experimental extensions must lie within this range.
    SBI_EXT_EXPERIMENTAL_START = 0x08000000,
    SBI_EXT_EXPERIMENTAL_END = 0x08FFFFFF,
    // Vendor extensions must lie within this range.
    SBI_EXT_VENDOR_START = 0x09000000,
    SBI_EXT_VENDOR_END = 0x09FFFFFF,
}

macro_rules! c_enum { ($name:ident { $($item:ident = $value:expr,)* }) => {
    #[repr(i32)] pub enum $name { $($item = $value,)* }
} }
c_enum!(sbi_ext_base_fid { SBI_EXT_BASE_GET_SPEC_VERSION=0, SBI_EXT_BASE_GET_IMP_ID=1, SBI_EXT_BASE_GET_IMP_VERSION=2, SBI_EXT_BASE_PROBE_EXT=3, SBI_EXT_BASE_GET_MVENDORID=4, SBI_EXT_BASE_GET_MARCHID=5, SBI_EXT_BASE_GET_MIMPID=6, });
c_enum!(sbi_ext_time_fid { SBI_EXT_TIME_SET_TIMER=0, });
c_enum!(sbi_ext_ipi_fid { SBI_EXT_IPI_SEND_IPI=0, });
c_enum!(sbi_ext_rfence_fid { SBI_EXT_RFENCE_REMOTE_FENCE_I=0, SBI_EXT_RFENCE_REMOTE_SFENCE_VMA=1, SBI_EXT_RFENCE_REMOTE_SFENCE_VMA_ASID=2, SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA_VMID=3, SBI_EXT_RFENCE_REMOTE_HFENCE_GVMA=4, SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA_ASID=5, SBI_EXT_RFENCE_REMOTE_HFENCE_VVMA=6, });
c_enum!(sbi_ext_hsm_fid { SBI_EXT_HSM_HART_START=0, SBI_EXT_HSM_HART_STOP=1, SBI_EXT_HSM_HART_STATUS=2, SBI_EXT_HSM_HART_SUSPEND=3, });
c_enum!(sbi_hsm_hart_state { SBI_HSM_STATE_STARTED=0, SBI_HSM_STATE_STOPPED=1, SBI_HSM_STATE_START_PENDING=2, SBI_HSM_STATE_STOP_PENDING=3, SBI_HSM_STATE_SUSPENDED=4, SBI_HSM_STATE_SUSPEND_PENDING=5, SBI_HSM_STATE_RESUME_PENDING=6, });

pub const SBI_HSM_SUSP_BASE_MASK: u32 = 0x7fffffff;
pub const SBI_HSM_SUSP_NON_RET_BIT: u32 = 0x80000000;
pub const SBI_HSM_SUSP_PLAT_BASE: u32 = 0x10000000;
pub const SBI_HSM_SUSPEND_RET_DEFAULT: u32 = 0;
pub const SBI_HSM_SUSPEND_RET_PLATFORM: u32 = SBI_HSM_SUSP_PLAT_BASE;
pub const SBI_HSM_SUSPEND_RET_LAST: u32 = SBI_HSM_SUSP_BASE_MASK;
pub const SBI_HSM_SUSPEND_NON_RET_DEFAULT: u32 = SBI_HSM_SUSP_NON_RET_BIT;
pub const SBI_HSM_SUSPEND_NON_RET_PLATFORM: u32 = SBI_HSM_SUSP_NON_RET_BIT | SBI_HSM_SUSP_PLAT_BASE;
pub const SBI_HSM_SUSPEND_NON_RET_LAST: u32 = SBI_HSM_SUSP_NON_RET_BIT | SBI_HSM_SUSP_BASE_MASK;

c_enum!(sbi_ext_srst_fid { SBI_EXT_SRST_RESET=0, });
c_enum!(sbi_srst_reset_type { SBI_SRST_RESET_TYPE_SHUTDOWN=0, SBI_SRST_RESET_TYPE_COLD_REBOOT=1, SBI_SRST_RESET_TYPE_WARM_REBOOT=2, });
c_enum!(sbi_srst_reset_reason { SBI_SRST_RESET_REASON_NONE=0, SBI_SRST_RESET_REASON_SYS_FAILURE=1, });
c_enum!(sbi_ext_susp_fid { SBI_EXT_SUSP_SYSTEM_SUSPEND=0, });
c_enum!(sbi_ext_susp_sleep_type { SBI_SUSP_SLEEP_TYPE_SUSPEND_TO_RAM=0, });
c_enum!(sbi_ext_pmu_fid { SBI_EXT_PMU_NUM_COUNTERS=0, SBI_EXT_PMU_COUNTER_GET_INFO=1, SBI_EXT_PMU_COUNTER_CFG_MATCH=2, SBI_EXT_PMU_COUNTER_START=3, SBI_EXT_PMU_COUNTER_STOP=4, SBI_EXT_PMU_COUNTER_FW_READ=5, SBI_EXT_PMU_COUNTER_FW_READ_HI=6, SBI_EXT_PMU_SNAPSHOT_SET_SHMEM=7, SBI_EXT_PMU_EVENT_GET_INFO=8, });

#[repr(C)]
pub union sbi_pmu_ctr_info { pub value: usize, pub bits: sbi_pmu_ctr_info_bits }
#[repr(C)] pub struct sbi_pmu_ctr_info_bits { pub csr: usize, pub width: usize, pub reserved: usize, pub type_: usize }
#[repr(C)] pub struct riscv_pmu_snapshot_data { pub ctr_overflow_mask: u64, pub ctr_values: [u64; 64], pub reserved: [u64; 447] }
#[repr(C)] pub struct riscv_pmu_event_info { pub event_idx: u32, pub output: u32, pub event_data: u64 }
pub const RISCV_PMU_EVENT_INFO_OUTPUT_MASK: u64 = 0x01;
pub const RISCV_PMU_RAW_EVENT_MASK: u64 = (1u64 << 48) - 1;
pub const RISCV_PMU_PLAT_FW_EVENT_MASK: u64 = (1u64 << 62) - 1;
pub const RISCV_PMU_RAW_EVENT_V2_MASK: u64 = (1u64 << 56) - 1;
pub const RISCV_PMU_RAW_EVENT_IDX: u32 = 0x20000;
pub const RISCV_PMU_RAW_EVENT_V2_IDX: u32 = 0x30000;
pub const RISCV_PLAT_FW_EVENT: u32 = 0xFFFF;

// General PMU event codes specified in the SBI PMU extension.
c_enum!(sbi_pmu_hw_generic_events_t { SBI_PMU_HW_NO_EVENT=0, SBI_PMU_HW_CPU_CYCLES=1, SBI_PMU_HW_INSTRUCTIONS=2, SBI_PMU_HW_CACHE_REFERENCES=3, SBI_PMU_HW_CACHE_MISSES=4, SBI_PMU_HW_BRANCH_INSTRUCTIONS=5, SBI_PMU_HW_BRANCH_MISSES=6, SBI_PMU_HW_BUS_CYCLES=7, SBI_PMU_HW_STALLED_CYCLES_FRONTEND=8, SBI_PMU_HW_STALLED_CYCLES_BACKEND=9, SBI_PMU_HW_REF_CPU_CYCLES=10, SBI_PMU_HW_GENERAL_MAX=11, });
// Special firmware events provided even when hardware lacks performance events.
c_enum!(sbi_pmu_fw_generic_events_t { SBI_PMU_FW_MISALIGNED_LOAD=0, SBI_PMU_FW_MISALIGNED_STORE=1, SBI_PMU_FW_ACCESS_LOAD=2, SBI_PMU_FW_ACCESS_STORE=3, SBI_PMU_FW_ILLEGAL_INSN=4, SBI_PMU_FW_SET_TIMER=5, SBI_PMU_FW_IPI_SENT=6, SBI_PMU_FW_IPI_RCVD=7, SBI_PMU_FW_FENCE_I_SENT=8, SBI_PMU_FW_FENCE_I_RCVD=9, SBI_PMU_FW_SFENCE_VMA_SENT=10, SBI_PMU_FW_SFENCE_VMA_RCVD=11, SBI_PMU_FW_SFENCE_VMA_ASID_SENT=12, SBI_PMU_FW_SFENCE_VMA_ASID_RCVD=13, SBI_PMU_FW_HFENCE_GVMA_SENT=14, SBI_PMU_FW_HFENCE_GVMA_RCVD=15, SBI_PMU_FW_HFENCE_GVMA_VMID_SENT=16, SBI_PMU_FW_HFENCE_GVMA_VMID_RCVD=17, SBI_PMU_FW_HFENCE_VVMA_SENT=18, SBI_PMU_FW_HFENCE_VVMA_RCVD=19, SBI_PMU_FW_HFENCE_VVMA_ASID_SENT=20, SBI_PMU_FW_HFENCE_VVMA_ASID_RCVD=21, SBI_PMU_FW_MAX=22, });
c_enum!(sbi_pmu_event_type { SBI_PMU_EVENT_TYPE_HW=0, SBI_PMU_EVENT_TYPE_CACHE=1, SBI_PMU_EVENT_TYPE_RAW=2, SBI_PMU_EVENT_TYPE_RAW_V2=3, SBI_PMU_EVENT_TYPE_FW=15, });
c_enum!(sbi_pmu_ctr_type { SBI_PMU_CTR_TYPE_HW=0, SBI_PMU_CTR_TYPE_FW=1, });

pub const SBI_PMU_EVENT_IDX_OFFSET: u32=20; pub const SBI_PMU_EVENT_IDX_MASK: u32=0xFFFFF; pub const SBI_PMU_EVENT_IDX_CODE_MASK: u32=0xFFFF; pub const SBI_PMU_EVENT_IDX_TYPE_MASK: u32=0xF0000; pub const SBI_PMU_EVENT_RAW_IDX: u32=0x20000; pub const SBI_PMU_FIXED_CTR_MASK: u32=0x07;
pub const SBI_PMU_EVENT_CACHE_ID_CODE_MASK: u32=0xFFF8; pub const SBI_PMU_EVENT_CACHE_OP_ID_CODE_MASK: u32=0x06; pub const SBI_PMU_EVENT_CACHE_RESULT_ID_CODE_MASK: u32=0x01; pub const SBI_PMU_EVENT_CACHE_ID_SHIFT: u32=3; pub const SBI_PMU_EVENT_CACHE_OP_SHIFT: u32=1; pub const SBI_PMU_EVENT_IDX_INVALID: u32=0xFFFFFFFF;
pub const SBI_PMU_CFG_FLAG_SKIP_MATCH:u32=1<<0; pub const SBI_PMU_CFG_FLAG_CLEAR_VALUE:u32=1<<1; pub const SBI_PMU_CFG_FLAG_AUTO_START:u32=1<<2; pub const SBI_PMU_CFG_FLAG_SET_VUINH:u32=1<<3; pub const SBI_PMU_CFG_FLAG_SET_VSINH:u32=1<<4; pub const SBI_PMU_CFG_FLAG_SET_UINH:u32=1<<5; pub const SBI_PMU_CFG_FLAG_SET_SINH:u32=1<<6; pub const SBI_PMU_CFG_FLAG_SET_MINH:u32=1<<7;
pub const SBI_PMU_START_FLAG_SET_INIT_VALUE:u32=1; pub const SBI_PMU_START_FLAG_INIT_SNAPSHOT:u32=2; pub const SBI_PMU_STOP_FLAG_RESET:u32=1; pub const SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT:u32=2;

c_enum!(sbi_ext_dbcn_fid { SBI_EXT_DBCN_CONSOLE_WRITE=0, SBI_EXT_DBCN_CONSOLE_READ=1, SBI_EXT_DBCN_CONSOLE_WRITE_BYTE=2, });
c_enum!(sbi_ext_sta_fid { SBI_EXT_STA_STEAL_TIME_SET_SHMEM=0, });
#[repr(C, packed)] pub struct sbi_sta_struct { pub sequence: u32, pub flags: u32, pub steal: u64, pub preempted: u8, pub pad: [u8;47] }
pub const SBI_SHMEM_DISABLE: i32 = -1;
c_enum!(sbi_ext_nacl_fid { SBI_EXT_NACL_PROBE_FEATURE=0, SBI_EXT_NACL_SET_SHMEM=1, SBI_EXT_NACL_SYNC_CSR=2, SBI_EXT_NACL_SYNC_HFENCE=3, SBI_EXT_NACL_SYNC_SRET=4, });
c_enum!(sbi_ext_nacl_feature { SBI_NACL_FEAT_SYNC_CSR=0, SBI_NACL_FEAT_SYNC_HFENCE=1, SBI_NACL_FEAT_SYNC_SRET=2, SBI_NACL_FEAT_AUTOSWAP_CSR=3, });
pub const SBI_EXT_FWFT_SET:u32=0; pub const SBI_EXT_FWFT_GET:u32=1;
c_enum!(sbi_fwft_feature_t { SBI_FWFT_MISALIGNED_EXC_DELEG=0, SBI_FWFT_LANDING_PAD=1, SBI_FWFT_SHADOW_STACK=2, SBI_FWFT_DOUBLE_TRAP=3, SBI_FWFT_PTE_AD_HW_UPDATING=4, SBI_FWFT_POINTER_MASKING_PMLEN=5, SBI_FWFT_LOCAL_RESERVED_START=6, SBI_FWFT_LOCAL_RESERVED_END=0x3fffffff, SBI_FWFT_LOCAL_PLATFORM_START=0x40000000, SBI_FWFT_LOCAL_PLATFORM_END=0x7fffffff, SBI_FWFT_GLOBAL_RESERVED_START=0x80000000u32 as i32, SBI_FWFT_GLOBAL_RESERVED_END=0xbfffffff_u32 as i32, SBI_FWFT_GLOBAL_PLATFORM_START=0xc0000000_u32 as i32, SBI_FWFT_GLOBAL_PLATFORM_END=-1, });
pub const SBI_FWFT_PLATFORM_FEATURE_BIT:u32=1<<30; pub const SBI_FWFT_GLOBAL_FEATURE_BIT:u32=1<<31; pub const SBI_FWFT_SET_FLAG_LOCK:u32=1;

pub const SBI_NACL_SHMEM_ADDR_SHIFT:u32=12; pub const SBI_NACL_SHMEM_SCRATCH_OFFSET:u32=0; pub const SBI_NACL_SHMEM_SCRATCH_SIZE:u32=0x1000; pub const SBI_NACL_SHMEM_SRET_OFFSET:u32=0; pub const SBI_NACL_SHMEM_SRET_SIZE:u32=0x0200; pub const SBI_NACL_SHMEM_AUTOSWAP_OFFSET:u32=SBI_NACL_SHMEM_SRET_OFFSET+SBI_NACL_SHMEM_SRET_SIZE; pub const SBI_NACL_SHMEM_AUTOSWAP_SIZE:u32=0x0080; pub const SBI_NACL_SHMEM_UNUSED_OFFSET:u32=SBI_NACL_SHMEM_AUTOSWAP_OFFSET+SBI_NACL_SHMEM_AUTOSWAP_SIZE; pub const SBI_NACL_SHMEM_UNUSED_SIZE:u32=0x0580; pub const SBI_NACL_SHMEM_HFENCE_OFFSET:u32=SBI_NACL_SHMEM_UNUSED_OFFSET+SBI_NACL_SHMEM_UNUSED_SIZE; pub const SBI_NACL_SHMEM_HFENCE_SIZE:u32=0x0780; pub const SBI_NACL_SHMEM_DBITMAP_OFFSET:u32=SBI_NACL_SHMEM_HFENCE_OFFSET+SBI_NACL_SHMEM_HFENCE_SIZE; pub const SBI_NACL_SHMEM_DBITMAP_SIZE:u32=0x0080; pub const SBI_NACL_SHMEM_CSR_OFFSET:u32=SBI_NACL_SHMEM_DBITMAP_OFFSET+SBI_NACL_SHMEM_DBITMAP_SIZE;
// __riscv_xlen-dependent definitions retain the C intent using usize.
pub const SBI_NACL_SHMEM_CSR_SIZE: usize = (core::mem::size_of::<usize>() * 1024); pub const SBI_NACL_SHMEM_SIZE: usize = SBI_NACL_SHMEM_CSR_OFFSET as usize + SBI_NACL_SHMEM_CSR_SIZE;
pub const SBI_NACL_SHMEM_HFENCE_ENTRY_SZ:usize=core::mem::size_of::<usize>()*4; pub const SBI_NACL_SHMEM_HFENCE_ENTRY_MAX:usize=0x0780/SBI_NACL_SHMEM_HFENCE_ENTRY_SZ; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_BITS:usize=1; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_SHIFT:usize=usize::BITS as usize-SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_BITS; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_MASK:usize=(1usize<<SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_BITS)-1; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_PEND:usize=SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_MASK<<SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_SHIFT;
#[inline] pub const fn SBI_NACL_SHMEM_CSR_INDEX(csr_num:usize)->usize { ((csr_num & 0xc00)>>2)|(csr_num&0xff) }
#[inline] pub const fn SBI_NACL_SHMEM_HFENCE_ENTRY(num:usize)->usize { 0x1780 + num*SBI_NACL_SHMEM_HFENCE_ENTRY_SZ }
#[inline] pub const fn SBI_NACL_SHMEM_HFENCE_ENTRY_CONFIG(num:usize)->usize { SBI_NACL_SHMEM_HFENCE_ENTRY(num) }
#[inline] pub const fn SBI_NACL_SHMEM_HFENCE_ENTRY_PNUM(num:usize)->usize { SBI_NACL_SHMEM_HFENCE_ENTRY(num)+core::mem::size_of::<usize>() }
#[inline] pub const fn SBI_NACL_SHMEM_HFENCE_ENTRY_PCOUNT(num:usize)->usize { SBI_NACL_SHMEM_HFENCE_ENTRY(num)+core::mem::size_of::<usize>()*3 }
#[inline] pub const fn SBI_NACL_SHMEM_SRET_X(i:usize)->usize { core::mem::size_of::<usize>()*i }
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD1_BITS:usize=3; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD1_SHIFT:usize=SBI_NACL_SHMEM_HFENCE_CONFIG_PEND_SHIFT-3; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_BITS:usize=4; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_SHIFT:usize=SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD1_SHIFT-4; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_MASK:usize=(1usize<<4)-1;
pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA:usize=0; pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_ALL:usize=1; pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_VMID:usize=2; pub const SBI_NACL_SHMEM_HFENCE_TYPE_GVMA_VMID_ALL:usize=3; pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA:usize=4; pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ALL:usize=5; pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ASID:usize=6; pub const SBI_NACL_SHMEM_HFENCE_TYPE_VVMA_ASID_ALL:usize=7;
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD2_BITS:usize=1; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD2_SHIFT:usize=SBI_NACL_SHMEM_HFENCE_CONFIG_TYPE_SHIFT-1; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ORDER_BITS:usize=7; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ORDER_SHIFT:usize=SBI_NACL_SHMEM_HFENCE_CONFIG_RSVD2_SHIFT-7; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ORDER_MASK:usize=(1usize<<7)-1; pub const SBI_NACL_SHMEM_HFENCE_ORDER_BASE:usize=12;
pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ASID_BITS:usize=16; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_BITS:usize=14; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_SHIFT:usize=16; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_ASID_MASK:usize=(1usize<<16)-1; pub const SBI_NACL_SHMEM_HFENCE_CONFIG_VMID_MASK:usize=(1usize<<14)-1; pub const SBI_NACL_SHMEM_AUTOSWAP_FLAG_HSTATUS:usize=1; pub const SBI_NACL_SHMEM_AUTOSWAP_HSTATUS:usize=core::mem::size_of::<usize>(); pub const SBI_NACL_SHMEM_SRET_X_LAST:usize=31;

c_enum!(sbi_ext_mpxy_fid { SBI_EXT_MPXY_GET_SHMEM_SIZE=0, SBI_EXT_MPXY_SET_SHMEM=1, SBI_EXT_MPXY_GET_CHANNEL_IDS=2, SBI_EXT_MPXY_READ_ATTRS=3, SBI_EXT_MPXY_WRITE_ATTRS=4, SBI_EXT_MPXY_SEND_MSG_WITH_RESP=5, SBI_EXT_MPXY_SEND_MSG_WITHOUT_RESP=6, SBI_EXT_MPXY_GET_NOTIFICATION_EVENTS=7, });
c_enum!(sbi_mpxy_attribute_id { SBI_MPXY_ATTR_MSG_PROT_ID=0, SBI_MPXY_ATTR_MSG_PROT_VER=1, SBI_MPXY_ATTR_MSG_MAX_LEN=2, SBI_MPXY_ATTR_MSG_SEND_TIMEOUT=3, SBI_MPXY_ATTR_MSG_COMPLETION_TIMEOUT=4, SBI_MPXY_ATTR_CHANNEL_CAPABILITY=5, SBI_MPXY_ATTR_SSE_EVENT_ID=6, SBI_MPXY_ATTR_MSI_CONTROL=7, SBI_MPXY_ATTR_MSI_ADDR_LO=8, SBI_MPXY_ATTR_MSI_ADDR_HI=9, SBI_MPXY_ATTR_MSI_DATA=10, SBI_MPXY_ATTR_EVENTS_STATE_CONTROL=11, SBI_MPXY_ATTR_STD_ATTR_MAX_IDX=12, SBI_MPXY_ATTR_MSGPROTO_ATTR_START=0x80000000u32 as i32, SBI_MPXY_ATTR_MSGPROTO_ATTR_END=-1, });
c_enum!(sbi_mpxy_msgproto_id { SBI_MPXY_MSGPROTO_RPMI_ID=0, });
c_enum!(sbi_mpxy_rpmi_attribute_id { SBI_MPXY_RPMI_ATTR_SERVICEGROUP_ID=0x80000000u32 as i32, SBI_MPXY_RPMI_ATTR_SERVICEGROUP_VERSION=0x80000001u32 as i32, SBI_MPXY_RPMI_ATTR_IMPL_ID=0x80000002u32 as i32, SBI_MPXY_RPMI_ATTR_IMPL_VERSION=0x80000003u32 as i32, SBI_MPXY_RPMI_ATTR_MAX_ID=0x80000004u32 as i32, });
pub const SBI_MPXY_CHAN_CAP_MSI:u32=1; pub const SBI_MPXY_CHAN_CAP_SSE:u32=2; pub const SBI_MPXY_CHAN_CAP_EVENTS_STATE:u32=4; pub const SBI_MPXY_CHAN_CAP_SEND_WITH_RESP:u32=8; pub const SBI_MPXY_CHAN_CAP_SEND_WITHOUT_RESP:u32=16; pub const SBI_MPXY_CHAN_CAP_GET_NOTIFICATIONS:u32=32;
#[inline] pub const fn SBI_MPXY_MSG_PROT_VER_MAJOR(ver:u32)->u32 { ver>>16 }
#[inline] pub const fn SBI_MPXY_MSG_PROT_VER_MINOR(ver:u32)->u32 { ver&0xffff }
#[inline] pub const fn SBI_MPXY_MSG_PROT_MKVER(maj:u32,min:u32)->u32 { (maj<<16)|(min&0xffff) }
c_enum!(sbi_ext_dbtr_fid { SBI_EXT_DBTR_NUM_TRIGGERS=0, SBI_EXT_DBTR_SETUP_SHMEM=1, SBI_EXT_DBTR_TRIG_READ=2, SBI_EXT_DBTR_TRIG_INSTALL=3, SBI_EXT_DBTR_TRIG_UPDATE=4, SBI_EXT_DBTR_TRIG_UNINSTALL=5, SBI_EXT_DBTR_TRIG_ENABLE=6, SBI_EXT_DBTR_TRIG_DISABLE=7, });
#[repr(C)] pub struct sbi_dbtr_data_msg { pub tstate: usize, pub tdata1: usize, pub tdata2: usize, pub tdata3: usize }
#[repr(C)] pub struct sbi_dbtr_id_msg { pub idx: usize }
#[repr(C)] pub union sbi_dbtr_shmem_entry { pub data: sbi_dbtr_data_msg, pub id: sbi_dbtr_id_msg }

pub const SBI_SPEC_VERSION_DEFAULT:u32=1; pub const SBI_SPEC_VERSION_MAJOR_SHIFT:u32=24; pub const SBI_SPEC_VERSION_MAJOR_MASK:u32=0x7f; pub const SBI_SPEC_VERSION_MINOR_MASK:u32=0xffffff;
pub const SBI_SUCCESS:i32=0; pub const SBI_ERR_FAILURE:i32=-1; pub const SBI_ERR_NOT_SUPPORTED:i32=-2; pub const SBI_ERR_INVALID_PARAM:i32=-3; pub const SBI_ERR_DENIED:i32=-4; pub const SBI_ERR_INVALID_ADDRESS:i32=-5; pub const SBI_ERR_ALREADY_AVAILABLE:i32=-6; pub const SBI_ERR_ALREADY_STARTED:i32=-7; pub const SBI_ERR_ALREADY_STOPPED:i32=-8; pub const SBI_ERR_NO_SHMEM:i32=-9; pub const SBI_ERR_INVALID_STATE:i32=-10; pub const SBI_ERR_BAD_RANGE:i32=-11; pub const SBI_ERR_TIMEOUT:i32=-12; pub const SBI_ERR_IO:i32=-13; pub const SBI_ERR_DENIED_LOCKED:i32=-14;
extern "C" { pub static mut sbi_spec_version: usize; pub fn sbi_init(); pub fn __sbi_base_ecall(fid:i32)->isize; pub fn __sbi_ecall(arg0:usize,arg1:usize,arg2:usize,arg3:usize,arg4:usize,arg5:usize,fid:i32,ext:i32)->sbiret; }
#[repr(C)] pub struct sbiret { pub error:isize, pub value:isize }
#[inline] pub unsafe fn sbi_ecall(e:i32,f:i32,a0:usize,a1:usize,a2:usize,a3:usize,a4:usize,a5:usize)->sbiret { __sbi_ecall(a0,a1,a2,a3,a4,a5,f,e) }
extern "C" { pub fn sbi_get_mvendorid()->isize; pub fn sbi_get_marchid()->isize; pub fn sbi_get_mimpid()->isize; pub fn sbi_set_timer(stime_value:u64); pub fn sbi_shutdown(); pub fn sbi_send_ipi(cpu:u32); pub fn sbi_probe_extension(ext:i32)->isize; pub fn sbi_debug_console_write(bytes:*const i8,num_bytes:u32)->i32; pub fn sbi_debug_console_read(bytes:*mut i8,num_bytes:u32)->i32; pub static mut sbi_debug_console_available: bool; }
#[cfg(any())] extern "C" { pub fn sbi_console_putchar(ch:i32); pub fn sbi_console_getchar()->i32; }
// Remaining declarations use kernel-provided cpumask_t and error constants.
extern "C" { pub fn sbi_remote_fence_i(cpu_mask:*const core::ffi::c_void)->i32; pub fn sbi_remote_sfence_vma_asid(cpu_mask:*const core::ffi::c_void,start:usize,size:usize,asid:usize)->i32; pub fn sbi_remote_hfence_gvma(cpu_mask:*const core::ffi::c_void,start:usize,size:usize)->i32; pub fn sbi_remote_hfence_gvma_vmid(cpu_mask:*const core::ffi::c_void,start:usize,size:usize,vmid:usize)->i32; pub fn sbi_remote_hfence_vvma(cpu_mask:*const core::ffi::c_void,start:usize,size:usize)->i32; pub fn sbi_remote_hfence_vvma_asid(cpu_mask:*const core::ffi::c_void,start:usize,size:usize,asid:usize)->i32; pub fn sbi_fwft_set(feature:u32,value:usize,flags:usize)->i32; pub fn sbi_fwft_set_cpumask(mask:*const core::ffi::c_void,feature:u32,value:usize,flags:usize)->i32; pub fn sbi_ipi_init(); }
extern "C" { pub fn riscv_get_mvendorid()->usize; pub fn riscv_get_marchid()->usize; pub fn riscv_cached_mvendorid(cpu_id:u32)->usize; pub fn riscv_cached_marchid(cpu_id:u32)->usize; pub fn riscv_cached_mimpid(cpu_id:u32)->usize; }

#[inline] pub unsafe fn sbi_spec_is_0_1()->i32 { (sbi_spec_version == SBI_SPEC_VERSION_DEFAULT as usize) as i32 }
#[inline] pub unsafe fn sbi_major_version()->usize { (sbi_spec_version >> 24) & 0x7f }
#[inline] pub unsafe fn sbi_minor_version()->usize { sbi_spec_version & 0xffffff }
#[inline] pub fn sbi_mk_version(major:usize,minor:usize)->usize { ((major & 0x7f)<<24)|(minor&0xffffff) }

// SBI error to Linux errno mapping; Linux errno constants are external dependencies.
#[inline] pub fn sbi_err_map_linux_errno(err:i32)->i32 { match err { 0=>0, -4|-14=>-1, -3|-10=>-22, -11=>-34, -5=>-14, -9=>-12, -12=>-110, -13=>-5, _=>-95 } }

#[cfg(any())] pub fn sbi_fwft_set_online_cpus(_feature:u32,_value:usize,_flags:usize)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
