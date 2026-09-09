/* SPDX-License-Identifier: GPL-2.0 */
/* Netburst Performance Events (P4, old Xeon) */

pub const ARCH_P4_TOTAL_ESCR: u32 = 46;
pub const ARCH_P4_RESERVED_ESCR: u32 = 2;
pub const ARCH_P4_MAX_ESCR: u32 = ARCH_P4_TOTAL_ESCR - ARCH_P4_RESERVED_ESCR;
pub const ARCH_P4_MAX_CCCR: u32 = 18;
pub const ARCH_P4_CNTRVAL_BITS: u32 = 40;
pub const ARCH_P4_CNTRVAL_MASK: u64 = (1u64 << ARCH_P4_CNTRVAL_BITS) - 1;
pub const ARCH_P4_UNFLAGGED_BIT: u64 = 1u64 << (ARCH_P4_CNTRVAL_BITS - 1);

pub const P4_ESCR_EVENT_MASK: u64 = 0x7e000000;
pub const P4_ESCR_EVENT_SHIFT: u32 = 25;
pub const P4_ESCR_EVENTMASK_MASK: u64 = 0x01fffe00;
pub const P4_ESCR_EVENTMASK_SHIFT: u32 = 9;
pub const P4_ESCR_TAG_MASK: u64 = 0x1e0;
pub const P4_ESCR_TAG_SHIFT: u32 = 5;
pub const P4_ESCR_TAG_ENABLE: u64 = 0x10;
pub const P4_ESCR_T0_OS: u64 = 8;
pub const P4_ESCR_T0_USR: u64 = 4;
pub const P4_ESCR_T1_OS: u64 = 2;
pub const P4_ESCR_T1_USR: u64 = 1;

#[inline] pub const fn p4_escr_event(v: u64) -> u64 { v << P4_ESCR_EVENT_SHIFT }
#[inline] pub const fn p4_escr_emask(v: u64) -> u64 { v << P4_ESCR_EVENTMASK_SHIFT }
#[inline] pub const fn p4_escr_tag(v: u64) -> u64 { v << P4_ESCR_TAG_SHIFT }

pub const P4_CCCR_OVF: u64 = 0x80000000;
pub const P4_CCCR_CASCADE: u64 = 0x40000000;
pub const P4_CCCR_OVF_PMI_T0: u64 = 0x04000000;
pub const P4_CCCR_OVF_PMI_T1: u64 = 0x08000000;
pub const P4_CCCR_FORCE_OVF: u64 = 0x02000000;
pub const P4_CCCR_EDGE: u64 = 0x01000000;
pub const P4_CCCR_THRESHOLD_MASK: u64 = 0x00f00000;
pub const P4_CCCR_THRESHOLD_SHIFT: u32 = 20;
pub const P4_CCCR_COMPLEMENT: u64 = 0x00080000;
pub const P4_CCCR_COMPARE: u64 = 0x00040000;
pub const P4_CCCR_ESCR_SELECT_MASK: u64 = 0xe000;
pub const P4_CCCR_ESCR_SELECT_SHIFT: u32 = 13;
pub const P4_CCCR_ENABLE: u64 = 0x1000;
pub const P4_CCCR_THREAD_SINGLE: u64 = 0x10000;
pub const P4_CCCR_THREAD_BOTH: u64 = 0x20000;
pub const P4_CCCR_THREAD_ANY: u64 = 0x30000;
pub const P4_CCCR_RESERVED: u64 = 0xfff;
#[inline] pub const fn p4_cccr_threshold(v: u64) -> u64 { v << P4_CCCR_THRESHOLD_SHIFT }
#[inline] pub const fn p4_cccr_esel(v: u64) -> u64 { v << P4_CCCR_ESCR_SELECT_SHIFT }

#[inline] pub const fn p4_config_pack_escr(v: u64) -> u64 { v << 32 }
#[inline] pub const fn p4_config_pack_cccr(v: u64) -> u64 { v & 0xffff_ffff }
#[inline] pub const fn p4_config_unpack_escr(v: u64) -> u64 { v >> 32 }
#[inline] pub const fn p4_config_unpack_cccr(v: u64) -> u64 { v & 0xffff_ffff }
#[inline] pub const fn p4_config_unpack_emask(v: u64) -> u32 { ((p4_config_unpack_escr(v) & P4_ESCR_EVENTMASK_MASK) >> P4_ESCR_EVENTMASK_SHIFT) as u32 }
#[inline] pub const fn p4_config_unpack_event(v: u64) -> u32 { ((p4_config_unpack_escr(v) & P4_ESCR_EVENT_MASK) >> P4_ESCR_EVENT_SHIFT) as u32 }
pub const P4_CONFIG_HT_SHIFT: u32 = 63;
pub const P4_CONFIG_HT: u64 = 1u64 << P4_CONFIG_HT_SHIFT;
pub const P4_CONFIG_ALIASABLE: u64 = 1u64 << 9;
pub const P4_CONFIG_MASK_ESCR: u64 = P4_ESCR_EVENT_MASK | P4_ESCR_EVENTMASK_MASK | P4_ESCR_TAG_MASK | P4_ESCR_TAG_ENABLE;
pub const P4_CONFIG_MASK_CCCR: u64 = P4_CCCR_EDGE | P4_CCCR_THRESHOLD_MASK | P4_CCCR_COMPLEMENT | P4_CCCR_COMPARE | P4_CCCR_THREAD_ANY | P4_CCCR_RESERVED;
pub const P4_CONFIG_MASK: u64 = p4_config_pack_escr(P4_CONFIG_MASK_ESCR) | p4_config_pack_cccr(P4_CONFIG_MASK_CCCR);
pub const P4_CONFIG_EVENT_ALIAS_MASK: u64 = p4_config_pack_escr(P4_CONFIG_MASK_ESCR) | p4_config_pack_cccr(P4_CCCR_EDGE | P4_CCCR_THRESHOLD_MASK | P4_CCCR_COMPLEMENT | P4_CCCR_COMPARE);
pub const P4_CONFIG_EVENT_ALIAS_IMMUTABLE_BITS: u64 = P4_CONFIG_HT | p4_config_pack_escr(P4_ESCR_T0_OS | P4_ESCR_T0_USR | P4_ESCR_T1_OS | P4_ESCR_T1_USR) | p4_config_pack_cccr(P4_CCCR_OVF | P4_CCCR_CASCADE | P4_CCCR_FORCE_OVF | P4_CCCR_THREAD_ANY | P4_CCCR_OVF_PMI_T0 | P4_CCCR_OVF_PMI_T1 | P4_CONFIG_ALIASABLE);

#[inline] pub fn p4_is_event_cascaded(config: u64) -> bool { (p4_config_unpack_cccr(config) & P4_CCCR_CASCADE) != 0 }
#[inline] pub fn p4_ht_config_thread(config: u64) -> i32 { ((config & P4_CONFIG_HT) != 0) as i32 }
#[inline] pub fn p4_set_ht_bit(config: u64) -> u64 { config | P4_CONFIG_HT }
#[inline] pub fn p4_clear_ht_bit(config: u64) -> u64 { config & !P4_CONFIG_HT }
// CONFIG_SMP-dependent kernel state is supplied by the surrounding translation.
#[inline] pub fn p4_ht_active(max_threads_per_core: u32) -> i32 { (max_threads_per_core > 1) as i32 }
#[inline] pub fn p4_should_swap_ts(config: u64, ht_thread: i32) -> i32 { (p4_ht_config_thread(config) ^ ht_thread) }
#[inline] pub fn p4_default_cccr_conf(ht_thread: i32) -> u32 { (P4_CCCR_THREAD_ANY | if ht_thread == 0 { P4_CCCR_OVF_PMI_T0 } else { P4_CCCR_OVF_PMI_T1 }) as u32 }
#[inline] pub fn p4_default_escr_conf(ht_thread: i32, exclude_os: i32, exclude_usr: i32) -> u32 { let mut x=0; if ht_thread==0 { if exclude_os==0{x|=P4_ESCR_T0_OS;} if exclude_usr==0{x|=P4_ESCR_T0_USR;} } else { if exclude_os==0{x|=P4_ESCR_T1_OS;} if exclude_usr==0{x|=P4_ESCR_T1_USR;} } x as u32 }

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum P4_EVENTS { P4_EVENT_TC_DELIVER_MODE, P4_EVENT_BPU_FETCH_REQUEST, P4_EVENT_ITLB_REFERENCE, P4_EVENT_MEMORY_CANCEL, P4_EVENT_MEMORY_COMPLETE, P4_EVENT_LOAD_PORT_REPLAY, P4_EVENT_STORE_PORT_REPLAY, P4_EVENT_MOB_LOAD_REPLAY, P4_EVENT_PAGE_WALK_TYPE, P4_EVENT_BSQ_CACHE_REFERENCE, P4_EVENT_IOQ_ALLOCATION, P4_EVENT_IOQ_ACTIVE_ENTRIES, P4_EVENT_FSB_DATA_ACTIVITY, P4_EVENT_BSQ_ALLOCATION, P4_EVENT_BSQ_ACTIVE_ENTRIES, P4_EVENT_SSE_INPUT_ASSIST, P4_EVENT_PACKED_SP_UOP, P4_EVENT_PACKED_DP_UOP, P4_EVENT_SCALAR_SP_UOP, P4_EVENT_SCALAR_DP_UOP, P4_EVENT_64BIT_MMX_UOP, P4_EVENT_128BIT_MMX_UOP, P4_EVENT_X87_FP_UOP, P4_EVENT_TC_MISC, P4_EVENT_GLOBAL_POWER_EVENTS, P4_EVENT_TC_MS_XFER, P4_EVENT_UOP_QUEUE_WRITES, P4_EVENT_RETIRED_MISPRED_BRANCH_TYPE, P4_EVENT_RETIRED_BRANCH_TYPE, P4_EVENT_RESOURCE_STALL, P4_EVENT_WC_BUFFER, P4_EVENT_B2B_CYCLES, P4_EVENT_BNR, P4_EVENT_SNOOP, P4_EVENT_RESPONSE, P4_EVENT_FRONT_END_EVENT, P4_EVENT_EXECUTION_EVENT, P4_EVENT_REPLAY_EVENT, P4_EVENT_INSTR_RETIRED, P4_EVENT_UOPS_RETIRED, P4_EVENT_UOP_TYPE, P4_EVENT_BRANCH_RETIRED, P4_EVENT_MISPRED_BRANCH_RETIRED, P4_EVENT_X87_ASSIST, P4_EVENT_MACHINE_CLEAR, P4_EVENT_INSTR_COMPLETED }
#[inline] pub const fn p4_opcode_pack(event:u64, sel:u64)->u16 { ((event<<8)|sel) as u16 }
#[inline] pub const fn p4_opcode_esel(opcode:u16)->u16 { opcode & 0xff }
#[inline] pub const fn p4_opcode_evnt(opcode:u16)->u16 { (opcode & 0xff00)>>8 }

pub const P4_PEBS_CONFIG_ENABLE:u64=1<<7; pub const P4_PEBS_CONFIG_UOP_TAG:u64=1<<8; pub const P4_PEBS_CONFIG_METRIC_MASK:u64=0x3f; pub const P4_PEBS_CONFIG_MASK:u64=0xff;
pub const P4_PEBS_ENABLE:u64=0x02000000; pub const P4_PEBS_ENABLE_UOP_TAG:u64=0x01000000;
#[inline] pub const fn p4_config_unpack_metric(v:u64)->u64 { v&P4_PEBS_CONFIG_METRIC_MASK }
#[inline] pub const fn p4_config_unpack_pebs(v:u64)->u64 { v&P4_PEBS_CONFIG_MASK }
#[inline] pub const fn p4_config_pebs_has(v:u64, mask:u64)->u64 { p4_config_unpack_pebs(v)&mask }
#[repr(i32)] pub enum P4_PEBS_METRIC { P4_PEBS_METRIC__none, P4_PEBS_METRIC__1stl_cache_load_miss_retired, P4_PEBS_METRIC__2ndl_cache_load_miss_retired, P4_PEBS_METRIC__dtlb_load_miss_retired, P4_PEBS_METRIC__dtlb_store_miss_retired, P4_PEBS_METRIC__dtlb_all_miss_retired, P4_PEBS_METRIC__tagged_mispred_branch, P4_PEBS_METRIC__mob_load_replay_retired, P4_PEBS_METRIC__split_load_retired, P4_PEBS_METRIC__split_store_retired, P4_PEBS_METRIC__max }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
