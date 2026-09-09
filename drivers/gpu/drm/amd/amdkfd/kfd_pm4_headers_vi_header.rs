/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/* Rust translation of kfd_pm4_headers_vi.h. */

#[repr(C)]
pub union PM4_MES_TYPE_3_HEADER { pub u32All: u32, pub bits: u32 }

#[repr(u32)] pub enum mes_set_resources_queue_type_enum { queue_type__mes_set_resources__kernel_interface_queue_kiq = 0, queue_type__mes_set_resources__hsa_interface_queue_hiq = 1, queue_type__mes_set_resources__hsa_debug_interface_queue = 4 }
#[repr(C)] pub union pm4_mes_set_resources_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mes_set_resources { pub header: pm4_mes_set_resources_word, pub word2: pm4_mes_set_resources_word, pub queue_mask_lo: u32, pub queue_mask_hi: u32, pub gws_mask_lo: u32, pub gws_mask_hi: u32, pub word7: pm4_mes_set_resources_word, pub word8: pm4_mes_set_resources_word }

#[repr(C)] pub union pm4_mes_runlist_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mes_runlist { pub header: pm4_mes_runlist_word, pub word2: pm4_mes_runlist_word, pub word3: pm4_mes_runlist_word, pub word4: pm4_mes_runlist_word }

#[repr(C)] pub union pm4_mes_map_process_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mes_map_process { pub header: pm4_mes_map_process_word, pub word2: pm4_mes_map_process_word, pub word3: pm4_mes_map_process_word, pub reserved: u32, pub sh_mem_bases: u32, pub sh_mem_config: u32, pub sh_mem_ape1_base: u32, pub sh_mem_ape1_limit: u32, pub sh_hidden_private_base_vmid: u32, pub reserved2: u32, pub reserved3: u32, pub gds_addr_lo: u32, pub gds_addr_hi: u32, pub word10: pm4_mes_map_process_word, pub completion_signal_lo: u32, pub completion_signal_hi: u32 }

#[repr(u32)] pub enum mes_map_queues_queue_sel_vi_enum { queue_sel__mes_map_queues__map_to_specified_queue_slots_vi = 0, queue_sel__mes_map_queues__map_to_hws_determined_queue_slots_vi = 1 }
#[repr(u32)] pub enum mes_map_queues_queue_type_vi_enum { queue_type__mes_map_queues__normal_compute_vi = 0, queue_type__mes_map_queues__debug_interface_queue_vi = 1, queue_type__mes_map_queues__normal_latency_static_queue_vi = 2, queue_type__mes_map_queues__low_latency_static_queue_vi = 3 }
#[repr(u32)] pub enum mes_map_queues_engine_sel_vi_enum { engine_sel__mes_map_queues__compute_vi = 0, engine_sel__mes_map_queues__sdma0_vi = 2, engine_sel__mes_map_queues__sdma1_vi = 3 }
#[repr(C)] pub union pm4_mes_map_queues_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mes_map_queues { pub header: pm4_mes_map_queues_word, pub word2: pm4_mes_map_queues_word, pub word3: pm4_mes_map_queues_word, pub mqd_addr_lo: u32, pub mqd_addr_hi: u32, pub wptr_addr_lo: u32, pub wptr_addr_hi: u32 }

#[repr(u32)] pub enum mes_query_status_interrupt_sel_enum { interrupt_sel__mes_query_status__completion_status = 0, interrupt_sel__mes_query_status__process_status = 1, interrupt_sel__mes_query_status__queue_status = 2 }
#[repr(u32)] pub enum mes_query_status_command_enum { command__mes_query_status__interrupt_only = 0, command__mes_query_status__fence_only_immediate = 1, command__mes_query_status__fence_only_after_write_ack = 2, command__mes_query_status__fence_wait_for_write_ack_send_interrupt = 3 }
#[repr(u32)] pub enum mes_query_status_engine_sel_enum { engine_sel__mes_query_status__compute = 0, engine_sel__mes_query_status__sdma0_queue = 2, engine_sel__mes_query_status__sdma1_queue = 3 }
#[repr(C)] pub union pm4_mes_query_status_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mes_query_status { pub header: pm4_mes_query_status_word, pub word2: pm4_mes_query_status_word, pub word3: pm4_mes_query_status_word, pub addr_lo: u32, pub addr_hi: u32, pub data_lo: u32, pub data_hi: u32 }

#[repr(u32)] pub enum mes_unmap_queues_action_enum { action__mes_unmap_queues__preempt_queues = 0, action__mes_unmap_queues__reset_queues = 1, action__mes_unmap_queues__disable_process_queues = 2, action__mes_unmap_queues__reserved = 3 }
#[repr(u32)] pub enum mes_unmap_queues_queue_sel_enum { queue_sel__mes_unmap_queues__perform_request_on_specified_queues = 0, queue_sel__mes_unmap_queues__perform_request_on_pasid_queues = 1, queue_sel__mes_unmap_queues__unmap_all_queues = 2, queue_sel__mes_unmap_queues__unmap_all_non_static_queues = 3 }
#[repr(u32)] pub enum mes_unmap_queues_engine_sel_enum { engine_sel__mes_unmap_queues__compute = 0, engine_sel__mes_unmap_queues__sdma0 = 2, engine_sel__mes_unmap_queues__sdmal = 3 }
#[repr(C)] pub union pm4_mes_unmap_queues_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mes_unmap_queues { pub header: pm4_mes_unmap_queues_word, pub word2: pm4_mes_unmap_queues_word, pub word3: pm4_mes_unmap_queues_word, pub word4: pm4_mes_unmap_queues_word, pub word5: pm4_mes_unmap_queues_word, pub word6: pm4_mes_unmap_queues_word }

#[repr(u32)] pub enum RELEASE_MEM_event_index_enum { event_index___release_mem__end_of_pipe = 5, event_index___release_mem__shader_done = 6 }
#[repr(u32)] pub enum RELEASE_MEM_cache_policy_enum { cache_policy___release_mem__lru = 0, cache_policy___release_mem__stream = 1, cache_policy___release_mem__bypass = 2 }
#[repr(u32)] pub enum RELEASE_MEM_dst_sel_enum { dst_sel___release_mem__memory_controller = 0, dst_sel___release_mem__tc_l2 = 1, dst_sel___release_mem__queue_write_pointer_register = 2, dst_sel___release_mem__queue_write_pointer_poll_mask_bit = 3 }
#[repr(u32)] pub enum RELEASE_MEM_int_sel_enum { int_sel___release_mem__none = 0, int_sel___release_mem__send_interrupt_only = 1, int_sel___release_mem__send_interrupt_after_write_confirm = 2, int_sel___release_mem__send_data_after_write_confirm = 3 }
#[repr(u32)] pub enum RELEASE_MEM_data_sel_enum { data_sel___release_mem__none = 0, data_sel___release_mem__send_32_bit_low = 1, data_sel___release_mem__send_64_bit_data = 2, data_sel___release_mem__send_gpu_clock_counter = 3, data_sel___release_mem__send_cp_perfcounter_hi_lo = 4, data_sel___release_mem__store_gds_data_to_memory = 5 }
#[repr(C)] pub union pm4_mec_release_mem_word { pub ordinal: u32, pub bitfields: u32 }
#[repr(C)] pub struct pm4_mec_release_mem { pub header: pm4_mec_release_mem_word, pub word2: pm4_mec_release_mem_word, pub word3: pm4_mec_release_mem_word, pub word4: pm4_mec_release_mem_word, pub address_hi: u32, pub data_lo: u32, pub data_hi: u32 }

pub const CACHE_FLUSH_AND_INV_TS_EVENT: u32 = 0x00000014;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
