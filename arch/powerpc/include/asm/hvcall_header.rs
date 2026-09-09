/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of asm/powerpc/hvcall.h. */

pub const HVSC: i32 = 0x44000022;

/* Hypervisor return codes. */
pub const H_SUCCESS: i64 = 0; pub const H_BUSY: i64 = 1; pub const H_CLOSED: i64 = 2;
pub const H_NOT_AVAILABLE: i64 = 3; pub const H_CONSTRAINED: i64 = 4; pub const H_PARTIAL: i64 = 5;
pub const H_IN_PROGRESS: i64 = 14; pub const H_PAGE_REGISTERED: i64 = 15; pub const H_PARTIAL_STORE: i64 = 16;
pub const H_PENDING: i64 = 17; pub const H_CONTINUE: i64 = 18;
pub const H_LONG_BUSY_START_RANGE: i64 = 9900; pub const H_LONG_BUSY_ORDER_1_MSEC: i64 = 9900;
pub const H_LONG_BUSY_ORDER_10_MSEC: i64 = 9901; pub const H_LONG_BUSY_ORDER_100_MSEC: i64 = 9902;
pub const H_LONG_BUSY_ORDER_1_SEC: i64 = 9903; pub const H_LONG_BUSY_ORDER_10_SEC: i64 = 9904;
pub const H_LONG_BUSY_ORDER_100_SEC: i64 = 9905; pub const H_LONG_BUSY_END_RANGE: i64 = 9905;
pub const H_TOO_HARD: i64 = 9999;
pub const H_HARDWARE: i64 = -1; pub const H_FUNCTION: i64 = -2; pub const H_PRIVILEGE: i64 = -3;
pub const H_PARAMETER: i64 = -4; pub const H_BAD_MODE: i64 = -5; pub const H_PTEG_FULL: i64 = -6;
pub const H_NOT_FOUND: i64 = -7; pub const H_RESERVED_DABR: i64 = -8; pub const H_NO_MEM: i64 = -9;
pub const H_AUTHORITY: i64 = -10; pub const H_PERMISSION: i64 = -11; pub const H_DROPPED: i64 = -12;
pub const H_SOURCE_PARM: i64 = -13; pub const H_DEST_PARM: i64 = -14; pub const H_REMOTE_PARM: i64 = -15;
pub const H_RESOURCE: i64 = -16; pub const H_ADAPTER_PARM: i64 = -17; pub const H_RH_PARM: i64 = -18;
pub const H_RCQ_PARM: i64 = -19; pub const H_SCQ_PARM: i64 = -20; pub const H_EQ_PARM: i64 = -21;
pub const H_RT_PARM: i64 = -22; pub const H_ST_PARM: i64 = -23; pub const H_SIGT_PARM: i64 = -24;
pub const H_TOKEN_PARM: i64 = -25; pub const H_MLENGTH_PARM: i64 = -27; pub const H_MEM_PARM: i64 = -28;
pub const H_MEM_ACCESS_PARM: i64 = -29; pub const H_ATTR_PARM: i64 = -30; pub const H_PORT_PARM: i64 = -31;
pub const H_MCG_PARM: i64 = -32; pub const H_VL_PARM: i64 = -33; pub const H_TSIZE_PARM: i64 = -34;
pub const H_TRACE_PARM: i64 = -35; pub const H_MASK_PARM: i64 = -37; pub const H_MCG_FULL: i64 = -38;
pub const H_ALIAS_EXIST: i64 = -39; pub const H_P_COUNTER: i64 = -40; pub const H_TABLE_FULL: i64 = -41;
pub const H_ALT_TABLE: i64 = -42; pub const H_MR_CONDITION: i64 = -43; pub const H_NOT_ENOUGH_RESOURCES: i64 = -44;
pub const H_R_STATE: i64 = -45; pub const H_RESCINDED: i64 = -46; pub const H_ABORTED: i64 = -54;
pub const H_P2: i64 = -55; pub const H_P3: i64 = -56; pub const H_P4: i64 = -57; pub const H_P5: i64 = -58;
pub const H_P6: i64 = -59; pub const H_P7: i64 = -60; pub const H_P8: i64 = -61; pub const H_P9: i64 = -62;
pub const H_NOOP: i64 = -63; pub const H_TOO_BIG: i64 = -64; pub const H_UNSUPPORTED: i64 = -67;
pub const H_OVERLAP: i64 = -68; pub const H_INTERRUPT: i64 = -69; pub const H_BAD_DATA: i64 = -70;
pub const H_NOT_ACTIVE: i64 = -71; pub const H_SG_LIST: i64 = -72; pub const H_OP_MODE: i64 = -73;
pub const H_COP_HW: i64 = -74; pub const H_STATE: i64 = -75; pub const H_IN_USE: i64 = -77;
pub const H_INVALID_ELEMENT_ID: i64 = -79; pub const H_INVALID_ELEMENT_SIZE: i64 = -80; pub const H_INVALID_ELEMENT_VALUE: i64 = -81;
pub const H_INPUT_BUFFER_NOT_DEFINED: i64 = -82; pub const H_INPUT_BUFFER_TOO_SMALL: i64 = -83;
pub const H_OUTPUT_BUFFER_NOT_DEFINED: i64 = -84; pub const H_OUTPUT_BUFFER_TOO_SMALL: i64 = -85;
pub const H_PARTITION_PAGE_TABLE_NOT_DEFINED: i64 = -86; pub const H_GUEST_VCPU_STATE_NOT_HV_OWNED: i64 = -87;
pub const H_UNSUPPORTED_FLAG_START: i64 = -256; pub const H_UNSUPPORTED_FLAG_END: i64 = -511;
pub const H_MULTI_THREADS_ACTIVE: i64 = -9005; pub const H_OUTSTANDING_COP_OPS: i64 = -9006;
#[inline] pub const fn h_is_long_busy(x: i64) -> bool { x >= H_LONG_BUSY_START_RANGE && x <= H_LONG_BUSY_END_RANGE }

/* Flags. */
pub const H_LARGE_PAGE:u64=1u64<<(63-16); pub const H_EXACT:u64=1u64<<(63-24); pub const H_R_XLATE:u64=1u64<<(63-25);
pub const H_READ_4:u64=1u64<<(63-26); pub const H_PAGE_STATE_CHANGE:u64=1u64<<(63-28); pub const H_PAGE_UNUSED:u64=(1u64<<(63-29))|(1u64<<(63-30));
pub const H_PAGE_SET_UNUSED:u64=H_PAGE_STATE_CHANGE|H_PAGE_UNUSED; pub const H_PAGE_SET_LOANED:u64=H_PAGE_SET_UNUSED|(1u64<<(63-31)); pub const H_PAGE_SET_ACTIVE:u64=H_PAGE_STATE_CHANGE;
pub const H_AVPN:u64=1u64<<(63-32); pub const H_ANDCOND:u64=1u64<<(63-33); pub const H_LOCAL:u64=1u64<<(63-35);
pub const H_ICACHE_INVALIDATE:u64=1u64<<(63-40); pub const H_ICACHE_SYNCHRONIZE:u64=1u64<<(63-41); pub const H_COALESCE_CAND:u64=1u64<<(63-42);
pub const H_ZERO_PAGE:u64=1u64<<(63-48); pub const H_COPY_PAGE:u64=1u64<<(63-49); pub const H_N:u64=1u64<<(63-61); pub const H_PP1:u64=1u64<<(63-62); pub const H_PP2:u64=1u64<<(63-63);
pub const H_VPA_FUNC_SHIFT:u32=63-18; pub const H_VPA_FUNC_MASK:u64=7; pub const H_VPA_REG_VPA:u64=1; pub const H_VPA_REG_DTL:u64=2; pub const H_VPA_REG_SLB:u64=3; pub const H_VPA_DEREG_VPA:u64=5; pub const H_VPA_DEREG_DTL:u64=6; pub const H_VPA_DEREG_SLB:u64=7;
pub const H_VASI_INVALID:u32=0; pub const H_VASI_ENABLED:u32=1; pub const H_VASI_ABORTED:u32=2; pub const H_VASI_SUSPENDING:u32=3; pub const H_VASI_SUSPENDED:u32=4; pub const H_VASI_RESUMED:u32=5; pub const H_VASI_COMPLETED:u32=6;
pub const H_VASI_SIGNAL_CANCEL:u32=1; pub const H_VASI_SIGNAL_ABORT:u32=2; pub const H_VASI_SIGNAL_SUSPEND:u32=3; pub const H_VASI_SIGNAL_COMPLETE:u32=4; pub const H_VASI_SIGNAL_ENABLE:u32=5; pub const H_VASI_SIGNAL_FAILOVER:u32=6;
pub const H_CB_ALIGNMENT:usize=4096;

/* pSeries hypervisor opcodes. */
pub const H_REMOVE:u64=0x04; pub const H_ENTER:u64=0x08; pub const H_READ:u64=0x0c; pub const H_CLEAR_MOD:u64=0x10; pub const H_CLEAR_REF:u64=0x14; pub const H_PROTECT:u64=0x18; pub const H_GET_TCE:u64=0x1c; pub const H_PUT_TCE:u64=0x20; pub const H_SET_SPRG0:u64=0x24; pub const H_SET_DABR:u64=0x28; pub const H_PAGE_INIT:u64=0x2c; pub const H_SET_ASR:u64=0x30; pub const H_ASR_ON:u64=0x34; pub const H_ASR_OFF:u64=0x38;
pub const H_LOGICAL_CI_LOAD:u64=0x3c; pub const H_LOGICAL_CI_STORE:u64=0x40; pub const H_LOGICAL_CACHE_LOAD:u64=0x44; pub const H_LOGICAL_CACHE_STORE:u64=0x48; pub const H_LOGICAL_ICBI:u64=0x4c; pub const H_LOGICAL_DCBF:u64=0x50; pub const H_GET_TERM_CHAR:u64=0x54; pub const H_PUT_TERM_CHAR:u64=0x58; pub const H_REAL_TO_LOGICAL:u64=0x5c; pub const H_HYPERVISOR_DATA:u64=0x60; pub const H_EOI:u64=0x64; pub const H_CPPR:u64=0x68; pub const H_IPI:u64=0x6c; pub const H_IPOLL:u64=0x70; pub const H_XIRR:u64=0x74; pub const H_MIGRATE_DMA:u64=0x78; pub const H_PERFMON:u64=0x7c;
pub const H_REGISTER_VPA:u64=0xdc; pub const H_CEDE:u64=0xe0; pub const H_CONFER:u64=0xe4; pub const H_PROD:u64=0xe8; pub const H_GET_PPP:u64=0xec; pub const H_SET_PPP:u64=0xf0; pub const H_PURR:u64=0xf4; pub const H_PIC:u64=0xf8; pub const H_REG_CRQ:u64=0xfc; pub const H_FREE_CRQ:u64=0x100; pub const H_VIO_SIGNAL:u64=0x104; pub const H_SEND_CRQ:u64=0x108; pub const H_COPY_RDMA:u64=0x110; pub const H_REGISTER_LOGICAL_LAN:u64=0x114; pub const H_FREE_LOGICAL_LAN:u64=0x118; pub const H_ADD_LOGICAL_LAN_BUFFER:u64=0x11c; pub const H_SEND_LOGICAL_LAN:u64=0x120; pub const H_BULK_REMOVE:u64=0x124; pub const H_MULTICAST_CTRL:u64=0x130; pub const H_SET_XDABR:u64=0x134; pub const H_STUFF_TCE:u64=0x138; pub const H_PUT_TCE_INDIRECT:u64=0x13c; pub const H_CHANGE_LOGICAL_LAN_MAC:u64=0x14c; pub const H_VTERM_PARTNER_INFO:u64=0x150; pub const H_REGISTER_VTERM:u64=0x154; pub const H_FREE_VTERM:u64=0x158;
pub const H_RESET_EVENTS:u64=0x15c; pub const H_ALLOC_RESOURCE:u64=0x160; pub const H_FREE_RESOURCE:u64=0x164; pub const H_MODIFY_QP:u64=0x168; pub const H_QUERY_QP:u64=0x16c; pub const H_REREGISTER_PMR:u64=0x170; pub const H_REGISTER_SMR:u64=0x174; pub const H_QUERY_MR:u64=0x178; pub const H_QUERY_MW:u64=0x17c; pub const H_QUERY_HCA:u64=0x180; pub const H_QUERY_PORT:u64=0x184; pub const H_MODIFY_PORT:u64=0x188; pub const H_DEFINE_AQP1:u64=0x18c; pub const H_GET_TRACE_BUFFER:u64=0x190; pub const H_DEFINE_AQP0:u64=0x194; pub const H_RESIZE_MR:u64=0x198; pub const H_ATTACH_MCQP:u64=0x19c; pub const H_DETACH_MCQP:u64=0x1a0; pub const H_CREATE_RPT:u64=0x1a4; pub const H_REMOVE_RPT:u64=0x1a8; pub const H_REGISTER_RPAGES:u64=0x1ac; pub const H_DISABLE_AND_GET:u64=0x1b0; pub const H_ERROR_DATA:u64=0x1b4; pub const H_GET_HCA_INFO:u64=0x1b8; pub const H_GET_PERF_COUNT:u64=0x1bc; pub const H_MANAGE_TRACE:u64=0x1c0; pub const H_GET_CPU_CHARACTERISTICS:u64=0x1c8; pub const H_FREE_LOGICAL_LAN_BUFFER:u64=0x1d4; pub const H_POLL_PENDING:u64=0x1d8; pub const H_QUERY_INT_STATE:u64=0x1e4;
pub const H_ILLAN_ATTRIBUTES:u64=0x244; pub const H_ADD_LOGICAL_LAN_BUFFERS:u64=0x248; pub const H_MODIFY_HEA_QP:u64=0x250; pub const H_QUERY_HEA_QP:u64=0x254; pub const H_QUERY_HEA:u64=0x258; pub const H_QUERY_HEA_PORT:u64=0x25c; pub const H_MODIFY_HEA_PORT:u64=0x260; pub const H_REG_BCMC:u64=0x264; pub const H_DEREG_BCMC:u64=0x268; pub const H_REGISTER_HEA_RPAGES:u64=0x26c; pub const H_DISABLE_AND_GET_HEA:u64=0x270; pub const H_GET_HEA_INFO:u64=0x274; pub const H_ALLOC_HEA_RESOURCE:u64=0x278; pub const H_ADD_CONN:u64=0x284; pub const H_DEL_CONN:u64=0x288; pub const H_JOIN:u64=0x298; pub const H_VASI_SIGNAL:u64=0x2a0; pub const H_VASI_STATE:u64=0x2a4; pub const H_VIOCTL:u64=0x2a8; pub const H_ENABLE_CRQ:u64=0x2b0; pub const H_GET_EM_PARMS:u64=0x2b8; pub const H_SET_MPP:u64=0x2d0; pub const H_GET_MPP:u64=0x2d4; pub const H_REG_SUB_CRQ:u64=0x2dc; pub const H_FREE_SUB_CRQ:u64=0x2e0; pub const H_SEND_SUB_CRQ:u64=0x2e4; pub const H_SEND_SUB_CRQ_INDIRECT:u64=0x2e8; pub const H_HOME_NODE_ASSOCIATIVITY:u64=0x2ec; pub const H_BEST_ENERGY:u64=0x2f4; pub const H_XIRR_X:u64=0x2fc; pub const H_RANDOM:u64=0x300; pub const H_COP:u64=0x304; pub const H_GET_MPP_X:u64=0x314; pub const H_SET_MODE:u64=0x31c; pub const H_BLOCK_REMOVE:u64=0x328;
pub const H_CLEAR_HPT:u64=0x358; pub const H_REQUEST_VMC:u64=0x360; pub const H_RESIZE_HPT_PREPARE:u64=0x36c; pub const H_RESIZE_HPT_COMMIT:u64=0x370; pub const H_REGISTER_PROC_TBL:u64=0x37c; pub const H_SIGNAL_SYS_RESET:u64=0x380; pub const H_ALLOCATE_VAS_WINDOW:u64=0x388; pub const H_MODIFY_VAS_WINDOW:u64=0x38c; pub const H_DEALLOCATE_VAS_WINDOW:u64=0x390; pub const H_QUERY_VAS_WINDOW:u64=0x394; pub const H_QUERY_VAS_CAPABILITIES:u64=0x398; pub const H_QUERY_NX_CAPABILITIES:u64=0x39c; pub const H_GET_NX_FAULT:u64=0x3a0; pub const H_INT_GET_SOURCE_INFO:u64=0x3a8; pub const H_INT_SET_SOURCE_CONFIG:u64=0x3ac; pub const H_INT_GET_SOURCE_CONFIG:u64=0x3b0; pub const H_INT_GET_QUEUE_INFO:u64=0x3b4; pub const H_INT_SET_QUEUE_CONFIG:u64=0x3b8; pub const H_INT_GET_QUEUE_CONFIG:u64=0x3bc; pub const H_INT_SET_OS_REPORTING_LINE:u64=0x3c0; pub const H_INT_GET_OS_REPORTING_LINE:u64=0x3c4; pub const H_INT_ESB:u64=0x3c8; pub const H_INT_SYNC:u64=0x3cc; pub const H_INT_RESET:u64=0x3d0;
pub const H_SCM_READ_METADATA:u64=0x3e4; pub const H_SCM_WRITE_METADATA:u64=0x3e8; pub const H_SCM_BIND_MEM:u64=0x3ec; pub const H_SCM_UNBIND_MEM:u64=0x3f0; pub const H_SCM_QUERY_BLOCK_MEM_BINDING:u64=0x3f4; pub const H_SCM_QUERY_LOGICAL_MEM_BINDING:u64=0x3f8; pub const H_SCM_UNBIND_ALL:u64=0x3fc; pub const H_SCM_HEALTH:u64=0x400; pub const H_SCM_PERFORMANCE_STATS:u64=0x418; pub const H_PKS_GET_CONFIG:u64=0x41c; pub const H_PKS_SET_PASSWORD:u64=0x420; pub const H_PKS_GEN_PASSWORD:u64=0x424; pub const H_PKS_WRITE_OBJECT:u64=0x42c; pub const H_PKS_GEN_KEY:u64=0x430; pub const H_PKS_READ_OBJECT:u64=0x434; pub const H_PKS_REMOVE_OBJECT:u64=0x438; pub const H_PKS_CONFIRM_OBJECT_FLUSHED:u64=0x43c; pub const H_RPT_INVALIDATE:u64=0x448; pub const H_SCM_FLUSH:u64=0x44c; pub const H_GET_ENERGY_SCALE_INFO:u64=0x450; pub const H_PKS_SIGNED_UPDATE:u64=0x454; pub const H_HTM:u64=0x458; pub const H_WATCHDOG:u64=0x45c; pub const H_GUEST_GET_CAPABILITIES:u64=0x460; pub const H_GUEST_SET_CAPABILITIES:u64=0x464; pub const H_GUEST_CREATE:u64=0x470; pub const H_GUEST_CREATE_VCPU:u64=0x474; pub const H_GUEST_GET_STATE:u64=0x478; pub const H_GUEST_SET_STATE:u64=0x47c; pub const H_GUEST_RUN_VCPU:u64=0x480; pub const H_GUEST_COPY_MEMORY:u64=0x484; pub const H_GUEST_DELETE:u64=0x488; pub const H_PKS_WRAP_OBJECT:u64=0x490; pub const H_PKS_UNWRAP_OBJECT:u64=0x494; pub const MAX_HCALL_OPCODE:u64=H_PKS_UNWRAP_OBJECT;

pub const H_UNBIND_SCOPE_ALL:u32=1; pub const H_UNBIND_SCOPE_DRC:u32=2;
pub const H_GET_VIOA_DUMP_SIZE:u32=1; pub const H_GET_VIOA_DUMP:u32=2; pub const H_GET_ILLAN_NUM_VLAN_IDS:u32=3; pub const H_GET_ILLAN_VLAN_ID_LIST:u32=4; pub const H_GET_ILLAN_SWITCH_ID:u32=5; pub const H_DISABLE_MIGRATION:u32=6; pub const H_ENABLE_MIGRATION:u32=7; pub const H_GET_PARTNER_INFO:u32=8; pub const H_GET_PARTNER_WWPN_LIST:u32=9; pub const H_DISABLE_ALL_VIO_INTS:u32=0xa; pub const H_DISABLE_VIO_INTERRUPT:u32=0xb; pub const H_ENABLE_VIO_INTERRUPT:u32=0xc; pub const H_GET_SESSION_TOKEN:u32=0x19; pub const H_SESSION_ERR_DETECTED:u32=0x1a;
pub const H_RTAS:u32=0xf000; pub const H_LOGICAL_MEMOP:u32=0xf001; pub const H_CAS:u32=0xf002; pub const H_UPDATE_DT:u32=0xf003; pub const H_GET_24X7_CATALOG_PAGE:u32=0xf078; pub const H_GET_24X7_DATA:u32=0xf07c; pub const H_GET_PERF_COUNTER_INFO:u32=0xf080;
pub const H_SET_PARTITION_TABLE:u32=0xf800; pub const H_ENTER_NESTED:u32=0xf804; pub const H_TLB_INVALIDATE:u32=0xf808; pub const H_COPY_TOFROM_GUEST:u32=0xf80c;
pub const H_PAGE_IN_SHARED:u32=1; pub const H_SVM_PAGE_IN:u32=0xef00; pub const H_SVM_PAGE_OUT:u32=0xef04; pub const H_SVM_INIT_START:u32=0xef08; pub const H_SVM_INIT_DONE:u32=0xef0c; pub const H_SVM_INIT_ABORT:u32=0xef14;
pub const H_SET_MODE_RESOURCE_SET_CIABR:u32=1; pub const H_SET_MODE_RESOURCE_SET_DAWR0:u32=2; pub const H_SET_MODE_RESOURCE_ADDR_TRANS_MODE:u32=3; pub const H_SET_MODE_RESOURCE_LE:u32=4; pub const H_SET_MODE_RESOURCE_SET_DAWR1:u32=5; pub const H_SIGNAL_SYS_RESET_ALL:i32=-1; pub const H_SIGNAL_SYS_RESET_ALL_OTHERS:i32=-2;
pub const H_CPU_CHAR_SPEC_BAR_ORI31:u64=1u64<<63; pub const H_CPU_CHAR_BCCTRL_SERIALISED:u64=1u64<<62; pub const H_CPU_CHAR_L1D_FLUSH_ORI30:u64=1u64<<61; pub const H_CPU_CHAR_L1D_FLUSH_TRIG2:u64=1u64<<60; pub const H_CPU_CHAR_L1D_THREAD_PRIV:u64=1u64<<59; pub const H_CPU_CHAR_BRANCH_HINTS_HONORED:u64=1u64<<58; pub const H_CPU_CHAR_THREAD_RECONFIG_CTRL:u64=1u64<<57; pub const H_CPU_CHAR_COUNT_CACHE_DISABLED:u64=1u64<<56; pub const H_CPU_CHAR_BCCTR_FLUSH_ASSIST:u64=1u64<<54; pub const H_CPU_CHAR_BCCTR_LINK_FLUSH_ASSIST:u64=1u64<<52;
pub const H_CPU_BEHAV_FAVOUR_SECURITY:u64=1u64<<63; pub const H_CPU_BEHAV_L1D_FLUSH_PR:u64=1u64<<62; pub const H_CPU_BEHAV_BNDS_CHK_SPEC_BAR:u64=1u64<<61; pub const H_CPU_BEHAV_FAVOUR_SECURITY_H:u64=1u64<<60; pub const H_CPU_BEHAV_FLUSH_COUNT_CACHE:u64=1u64<<58; pub const H_CPU_BEHAV_FLUSH_LINK_STACK:u64=1u64<<57; pub const H_CPU_BEHAV_NO_L1D_FLUSH_ENTRY:u64=1u64<<56; pub const H_CPU_BEHAV_NO_L1D_FLUSH_UACCESS:u64=1u64<<55; pub const H_CPU_BEHAV_NO_STF_BARRIER:u64=1u64<<54;
pub const PROC_TABLE_OP_MASK:u32=0x18; pub const PROC_TABLE_DEREG:u32=0x10; pub const PROC_TABLE_NEW:u32=0x18; pub const PROC_TABLE_TYPE_MASK:u32=6; pub const PROC_TABLE_HPT_SLB:u32=0; pub const PROC_TABLE_HPT_PT:u32=2; pub const PROC_TABLE_RADIX:u32=4; pub const PROC_TABLE_GTSE:u32=1;
pub const H_RPTI_TYPE_NESTED:u32=1; pub const H_RPTI_TYPE_TLB:u32=2; pub const H_RPTI_TYPE_PWC:u32=4; pub const H_RPTI_TYPE_PRT:u32=8; pub const H_RPTI_TYPE_PAT:u32=8; pub const H_RPTI_TYPE_ALL:u32=H_RPTI_TYPE_TLB|H_RPTI_TYPE_PWC|H_RPTI_TYPE_PRT; pub const H_RPTI_TYPE_NESTED_ALL:u32=H_RPTI_TYPE_TLB|H_RPTI_TYPE_PWC|H_RPTI_TYPE_PAT; pub const H_RPTI_TARGET_CMMU:u32=1; pub const H_RPTI_TARGET_CMMU_LOCAL:u32=2; pub const H_RPTI_TARGET_NMMU:u32=4; pub const H_RPTI_PAGE_4K:u32=1; pub const H_RPTI_PAGE_64K:u32=2; pub const H_RPTI_PAGE_2M:u32=4; pub const H_RPTI_PAGE_1G:u32=8; pub const H_RPTI_PAGE_ALL:u64=!0;
pub const H_GUEST_FLAGS_WIDE:u64=1u64<<63; pub const H_GUEST_FLAGS_HOST_WIDE:u64=1u64<<62; pub const H_GUEST_CAP_COPY_MEM:u64=1u64<<63; pub const H_GUEST_CAP_POWER9:u64=1u64<<62; pub const H_GUEST_CAP_POWER10:u64=1u64<<61; pub const H_GUEST_CAP_POWER11:u64=1u64<<60; pub const H_GUEST_CAP_BITMAP2:u64=1;
pub const H_HTM_FLAGS_HARDWARE_TARGET:u64=1u64<<63; pub const H_HTM_FLAGS_LOGICAL_TARGET:u64=1u64<<62; pub const H_HTM_FLAGS_PROCID_TARGET:u64=1u64<<61; pub const H_HTM_FLAGS_NOWRAP:u64=1u64<<60; pub const H_HTM_OP_SHIFT:u32=48; pub const H_HTM_TYPE_SHIFT:u32=32;
#[inline] pub const fn h_htm_op(x:u64)->u64{x<<H_HTM_OP_SHIFT} #[inline] pub const fn h_htm_type(x:u64)->u64{x<<H_HTM_TYPE_SHIFT} #[inline] pub const fn h_htm_target_node_index(x:u64)->u64{x<<48} #[inline] pub const fn h_htm_target_nodal_chip_index(x:u64)->u64{x<<32} #[inline] pub const fn h_htm_target_core_index_on_chip(x:u64)->u64{x<<16}
pub const H_HTM_OP_CAPABILITIES:u32=1; pub const H_HTM_OP_STATUS:u32=2; pub const H_HTM_OP_SETUP:u32=3; pub const H_HTM_OP_CONFIGURE:u32=4; pub const H_HTM_OP_START:u32=5; pub const H_HTM_OP_STOP:u32=6; pub const H_HTM_OP_DECONFIGURE:u32=7; pub const H_HTM_OP_DUMP_DETAILS:u32=8; pub const H_HTM_OP_DUMP_DATA:u32=9; pub const H_HTM_OP_DUMP_SYSMEM_CONF:u32=0xa; pub const H_HTM_OP_DUMP_SYSPROC_CONF:u32=0xb; pub const H_HTM_TYPE_NEST:u32=1; pub const H_HTM_TYPE_CORE:u32=2; pub const H_HTM_TYPE_LLAT:u32=3; pub const H_HTM_TYPE_GLOBAL:u32=0xff;

extern "C" {
    pub fn plpar_hcall_norets(opcode: libc::c_ulong, ...) -> libc::c_long;
    pub fn plpar_hcall_norets_notrace(opcode: libc::c_ulong, ...) -> libc::c_long;
    pub fn plpar_hcall(opcode: libc::c_ulong, retbuf: *mut libc::c_ulong, ...) -> libc::c_long;
    pub fn plpar_hcall_raw(opcode: libc::c_ulong, retbuf: *mut libc::c_ulong, ...) -> libc::c_long;
    pub fn plpar_hcall9(opcode: libc::c_ulong, retbuf: *mut libc::c_ulong, ...) -> libc::c_long;
    pub fn plpar_hcall9_raw(opcode: libc::c_ulong, retbuf: *mut libc::c_ulong, ...) -> libc::c_long;
    pub fn __trace_hcall_entry(opcode: libc::c_ulong, args: *mut libc::c_ulong);
    pub fn __trace_hcall_exit(opcode: libc::c_long, retval: libc::c_long, retbuf: *mut libc::c_ulong);
    pub fn h_get_mpp(data: *mut HvcallMppData) -> libc::c_long;
    pub fn h_get_mpp_x(data: *mut HvcallMppXData) -> libc::c_int;
}
pub const PLPAR_HCALL_BUFSIZE: usize=4; pub const PLPAR_HCALL9_BUFSIZE: usize=9;
#[repr(C)] pub struct HvcallMppData { pub entitled_mem: libc::c_ulong, pub mapped_mem: libc::c_ulong, pub group_num:u16, pub pool_num:u16, pub mem_weight:u8, pub unallocated_mem_weight:u8, pub unallocated_entitlement:libc::c_ulong, pub pool_size:libc::c_ulong, pub loan_request:libc::c_long, pub backing_mem:libc::c_ulong }
#[repr(C)] pub struct HvcallMppXData { pub coalesced_bytes:libc::c_ulong, pub pool_coalesced_bytes:libc::c_ulong, pub pool_purr_cycles:libc::c_ulong, pub pool_spurr_cycles:libc::c_ulong, pub reserved:[libc::c_ulong;3] }
pub fn get_longbusy_msecs(rc:i32)->u32 { match rc { 9900=>1,9901=>10,9902=>100,9903=>1000,9904=>10000,9905=>100000,_=>1 } }
#[repr(C)] pub struct HCpuCharResult { pub character:u64, pub behaviour:u64 }
#[repr(C)] pub struct HvGuestState { pub version:u64, pub lpid:u32, pub vcpu_token:u32, pub lpcr:u64, pub pcr:u64, pub amor:u64, pub dpdes:u64, pub hfscr:u64, pub tb_offset:i64, pub dawr0:u64, pub dawrx0:u64, pub ciabr:u64, pub hdec_expiry:u64, pub purr:u64, pub spurr:u64, pub ic:u64, pub vtb:u64, pub hdar:u64, pub hdsisr:u64, pub heir:u64, pub asdr:u64, pub srr0:u64, pub srr1:u64, pub sprg:[u64;4], pub pidr:u64, pub cfar:u64, pub ppr:u64, pub dawr1:u64, pub dawrx1:u64 }
pub const HV_GUEST_STATE_VERSION:u32=2;
pub fn hv_guest_state_size(version:u32)->isize { match version { 1 => (core::mem::offset_of!(HvGuestState,ppr)+8) as isize, 2 => core::mem::size_of::<HvGuestState>() as isize, _ => -1 } }
#[repr(C,packed)] pub struct HvGetPerfCounterInfoParams { pub counter_request:u32, pub starting_index:u32, pub secondary_index:u16, pub returned_values:u16, pub detail_rc:u32, pub cv_element_size:u16, pub counter_info_version_in:u8, pub counter_info_version_out:u8, pub reserved:[u8;0xc], pub counter_value:[u8;0] }
pub const HGPCI_REQ_BUFFER_SIZE:usize=4096; pub const HGPCI_MAX_DATA_BYTES:usize=HGPCI_REQ_BUFFER_SIZE-core::mem::size_of::<HvGetPerfCounterInfoParams>();
#[repr(C,packed)] pub struct HvGpciRequestBuffer { pub params:HvGetPerfCounterInfoParams, pub bytes:[u8;HGPCI_MAX_DATA_BYTES] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
