/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/* Copyright 2016-2022 Advanced Micro Devices, Inc. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union PM4_MES_TYPE_3_HEADER {
    pub bits: PM4_MES_TYPE_3_HEADER_bits,
    pub u32All: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PM4_MES_TYPE_3_HEADER_bits { pub raw: u32 }

macro_rules! raw_bits { ($($n:ident),* $(,)?) => { $(
    #[repr(C)] #[derive(Copy, Clone)] pub struct $n { pub raw: u32 }
)* } }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mes_set_resources_queue_type_enum { queue_type__mes_set_resources__kernel_interface_queue_kiq=0, queue_type__mes_set_resources__hsa_interface_queue_hiq=1, queue_type__mes_set_resources__hsa_debug_interface_queue=4 }
raw_bits!(pm4_mes_set_resources_bitfields2, pm4_mes_set_resources_bitfields7, pm4_mes_set_resources_bitfields8);
#[repr(C)] pub union pm4_mes_set_resources_header { pub header: PM4_MES_TYPE_3_HEADER, pub ordinal1: u32 }
#[repr(C)] pub union pm4_mes_set_resources_word2 { pub bitfields2: pm4_mes_set_resources_bitfields2, pub ordinal2: u32 }
#[repr(C)] pub union pm4_mes_set_resources_word7 { pub bitfields7: pm4_mes_set_resources_bitfields7, pub ordinal7: u32 }
#[repr(C)] pub union pm4_mes_set_resources_word8 { pub bitfields8: pm4_mes_set_resources_bitfields8, pub ordinal8: u32 }
#[repr(C)] pub struct pm4_mes_set_resources { pub word1: pm4_mes_set_resources_header, pub word2: pm4_mes_set_resources_word2, pub queue_mask_lo:u32, pub queue_mask_hi:u32, pub gws_mask_lo:u32, pub gws_mask_hi:u32, pub word7:pm4_mes_set_resources_word7, pub word8:pm4_mes_set_resources_word8 }

raw_bits!(pm4_mes_runlist_bitfields2, pm4_mes_runlist_bitfields4);
#[repr(C)] pub union pm4_mes_runlist_word1 { pub header: PM4_MES_TYPE_3_HEADER, pub ordinal1:u32 }
#[repr(C)] pub union pm4_mes_runlist_word2 { pub bitfields2:pm4_mes_runlist_bitfields2, pub ordinal2:u32 }
#[repr(C)] pub union pm4_mes_runlist_word4 { pub bitfields4:pm4_mes_runlist_bitfields4, pub ordinal4:u32 }
#[repr(C)] pub struct pm4_mes_runlist { pub word1:pm4_mes_runlist_word1, pub word2:pm4_mes_runlist_word2, pub ib_base_hi:u32, pub word4:pm4_mes_runlist_word4 }

raw_bits!(pm4_mes_map_process_bitfields2, pm4_mes_map_process_bitfields14);
#[repr(C)] pub union pm4_mes_map_process_word1 { pub header:PM4_MES_TYPE_3_HEADER, pub ordinal1:u32 }
#[repr(C)] pub union pm4_mes_map_process_word2 { pub bitfields2:pm4_mes_map_process_bitfields2, pub ordinal2:u32 }
#[repr(C)] pub union pm4_mes_map_process_word14 { pub bitfields14:pm4_mes_map_process_bitfields14, pub ordinal14:u32 }
#[repr(C)] pub struct pm4_mes_map_process { pub word1:pm4_mes_map_process_word1, pub word2:pm4_mes_map_process_word2, pub vm_context_page_table_base_addr_lo32:u32, pub vm_context_page_table_base_addr_hi32:u32, pub sh_mem_bases:u32, pub sh_mem_config:u32, pub sq_shader_tba_lo:u32, pub sq_shader_tba_hi:u32, pub sq_shader_tma_lo:u32, pub sq_shader_tma_hi:u32, pub reserved6:u32, pub gds_addr_lo:u32, pub gds_addr_hi:u32, pub word14:pm4_mes_map_process_word14, pub completion_signal_lo:u32, pub completion_signal_hi:u32 }

#[repr(C)] pub struct PM4_MES_MAP_PROCESS_VM { pub word1:PM4_MES_TYPE_3_HEADER, pub reserved1:u32, pub vm_context_cntl:u32, pub reserved2:u32, pub vm_context_page_table_end_addr_lo32:u32, pub vm_context_page_table_end_addr_hi32:u32, pub vm_context_page_table_start_addr_lo32:u32, pub vm_context_page_table_start_addr_hi32:u32, pub reserved3:u32, pub reserved4:u32, pub reserved5:u32, pub reserved6:u32, pub reserved7:u32, pub reserved8:u32, pub completion_signal_lo32:u32, pub completion_signal_hi32:u32 }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_map_queues_queue_sel_enum { queue_sel__mes_map_queues__map_to_specified_queue_slots_vi=0, queue_sel__mes_map_queues__map_to_hws_determined_queue_slots_vi=1 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_map_queues_queue_type_enum { queue_type__mes_map_queues__normal_compute_vi=0, queue_type__mes_map_queues__debug_interface_queue_vi=1, queue_type__mes_map_queues__normal_latency_static_queue_vi=2, queue_type__mes_map_queues__low_latency_static_queue_vi=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_map_queues_engine_sel_enum { engine_sel__mes_map_queues__compute_vi=0, engine_sel__mes_map_queues__sdma0_vi=2, engine_sel__mes_map_queues__sdma1_vi=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_map_queues_extended_engine_sel_enum { extended_engine_sel__mes_map_queues__legacy_engine_sel=0, extended_engine_sel__mes_map_queues__sdma0_to_7_sel=1, extended_engine_sel__mes_map_queues__sdma8_to_15_sel=2 }
raw_bits!(pm4_mes_map_queues_bitfields2, pm4_mes_map_queues_bitfields3);
#[repr(C)] pub struct pm4_mes_map_queues { pub header:PM4_MES_TYPE_3_HEADER, pub word2:pm4_mes_map_queues_bitfields2, pub word3:pm4_mes_map_queues_bitfields3, pub mqd_addr_lo:u32, pub mqd_addr_hi:u32, pub wptr_addr_lo:u32, pub wptr_addr_hi:u32 }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_query_status_interrupt_sel_enum { interrupt_sel__mes_query_status__completion_status=0, interrupt_sel__mes_query_status__process_status=1, interrupt_sel__mes_query_status__queue_status=2 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_query_status_command_enum { command__mes_query_status__interrupt_only=0, command__mes_query_status__fence_only_immediate=1, command__mes_query_status__fence_only_after_write_ack=2, command__mes_query_status__fence_wait_for_write_ack_send_interrupt=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_query_status_engine_sel_enum { engine_sel__mes_query_status__compute=0, engine_sel__mes_query_status__sdma0_queue=2, engine_sel__mes_query_status__sdma1_queue=3 }
raw_bits!(pm4_mes_query_status_bitfields2, pm4_mes_query_status_bitfields3a, pm4_mes_query_status_bitfields3b);
#[repr(C)] pub struct pm4_mes_query_status { pub header:PM4_MES_TYPE_3_HEADER, pub bitfields2:pm4_mes_query_status_bitfields2, pub bitfields3:pm4_mes_query_status_bitfields3a, pub addr_lo:u32, pub addr_hi:u32, pub data_lo:u32, pub data_hi:u32 }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_unmap_queues_action_enum { action__mes_unmap_queues__preempt_queues=0, action__mes_unmap_queues__reset_queues=1, action__mes_unmap_queues__disable_process_queues=2, action__mes_unmap_queues__reserved=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_unmap_queues_queue_sel_enum { queue_sel__mes_unmap_queues__perform_request_on_specified_queues=0, queue_sel__mes_unmap_queues__perform_request_on_pasid_queues=1, queue_sel__mes_unmap_queues__unmap_all_queues=2, queue_sel__mes_unmap_queues__unmap_all_non_static_queues=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_unmap_queues_engine_sel_enum { engine_sel__mes_unmap_queues__compute=0, engine_sel__mes_unmap_queues__sdma0=2, engine_sel__mes_unmap_queues__sdmal=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mes_unmap_queues_extended_engine_sel_enum { extended_engine_sel__mes_unmap_queues__legacy_engine_sel=0, extended_engine_sel__mes_unmap_queues__sdma0_to_7_sel=1 }
raw_bits!(pm4_mes_unmap_queues_bitfields2, pm4_mes_unmap_queues_bitfields3a, pm4_mes_unmap_queues_bitfields3b, pm4_mes_unmap_queues_bitfields4, pm4_mes_unmap_queues_bitfields5, pm4_mes_unmap_queues_bitfields6);
#[repr(C)] pub struct pm4_mes_unmap_queues { pub header:PM4_MES_TYPE_3_HEADER, pub bitfields2:pm4_mes_unmap_queues_bitfields2, pub bitfields3:pm4_mes_unmap_queues_bitfields3a, pub bitfields4:pm4_mes_unmap_queues_bitfields4, pub bitfields5:pm4_mes_unmap_queues_bitfields5, pub bitfields6:pm4_mes_unmap_queues_bitfields6 }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mec_release_mem_event_index_enum { event_index__mec_release_mem__end_of_pipe=5, event_index__mec_release_mem__shader_done=6 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mec_release_mem_cache_policy_enum { cache_policy__mec_release_mem__lru=0, cache_policy__mec_release_mem__stream=1 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mec_release_mem_pq_exe_status_enum { pq_exe_status__mec_release_mem__default=0, pq_exe_status__mec_release_mem__phase_update=1 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mec_release_mem_dst_sel_enum { dst_sel__mec_release_mem__memory_controller=0, dst_sel__mec_release_mem__tc_l2=1, dst_sel__mec_release_mem__queue_write_pointer_register=2, dst_sel__mec_release_mem__queue_write_pointer_poll_mask_bit=3 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mec_release_mem_int_sel_enum { int_sel__mec_release_mem__none=0, int_sel__mec_release_mem__send_interrupt_only=1, int_sel__mec_release_mem__send_interrupt_after_write_confirm=2, int_sel__mec_release_mem__send_data_after_write_confirm=3, int_sel__mec_release_mem__unconditionally_send_int_ctxid=4, int_sel__mec_release_mem__conditionally_send_int_ctxid_based_on_32_bit_compare=5, int_sel__mec_release_mem__conditionally_send_int_ctxid_based_on_64_bit_compare=6 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum mec_release_mem_data_sel_enum { data_sel__mec_release_mem__none=0, data_sel__mec_release_mem__send_32_bit_low=1, data_sel__mec_release_mem__send_64_bit_data=2, data_sel__mec_release_mem__send_gpu_clock_counter=3, data_sel__mec_release_mem__send_cp_perfcounter_hi_lo=4, data_sel__mec_release_mem__store_gds_data_to_memory=5 }
raw_bits!(pm4_mec_release_mem_bitfields2, pm4_mec_release_mem_bitfields3, pm4_mec_release_mem_bitfields4, pm4_mec_release_mem_bitfields4b, pm4_mec_release_mem_bitfields6c);
#[repr(C)] pub struct pm4_mec_release_mem { pub header:PM4_MES_TYPE_3_HEADER, pub bitfields2:pm4_mec_release_mem_bitfields2, pub bitfields3:pm4_mec_release_mem_bitfields3, pub bitfields4:pm4_mec_release_mem_bitfields4, pub address_hi:u32, pub data_lo:u32, pub data_hi:u32, pub int_ctxid:u32 }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum WRITE_DATA_dst_sel_enum { dst_sel___write_data__mem_mapped_register=0, dst_sel___write_data__tc_l2=2, dst_sel___write_data__gds=3, dst_sel___write_data__memory=5, dst_sel___write_data__memory_mapped_adc_persistent_state=6 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum WRITE_DATA_addr_incr_enum { addr_incr___write_data__increment_address=0, addr_incr___write_data__do_not_increment_address=1 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum WRITE_DATA_wr_confirm_enum { wr_confirm___write_data__do_not_wait_for_write_confirmation=0, wr_confirm___write_data__wait_for_write_confirmation=1 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum WRITE_DATA_cache_policy_enum { cache_policy___write_data__lru=0, cache_policy___write_data__stream=1 }
raw_bits!(pm4_mec_write_data_mmio_bitfields2, pm4_mec_write_data_mmio_bitfields3);
#[repr(C)] pub struct pm4_mec_write_data_mmio { pub header:PM4_MES_TYPE_3_HEADER, pub bitfields2:pm4_mec_write_data_mmio_bitfields2, pub bitfields3:pm4_mec_write_data_mmio_bitfields3, pub reserved7:u32, pub data:u32 }

pub const CACHE_FLUSH_AND_INV_TS_EVENT: u32 = 0x00000014;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
