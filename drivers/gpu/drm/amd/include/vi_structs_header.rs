/*
 * Copyright 2012 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#[repr(C)]
pub struct vi_sdma_mqd {
    pub sdmax_rlcx_rb_cntl: u32, pub sdmax_rlcx_rb_base: u32,
    pub sdmax_rlcx_rb_base_hi: u32, pub sdmax_rlcx_rb_rptr: u32,
    pub sdmax_rlcx_rb_wptr: u32, pub sdmax_rlcx_rb_wptr_poll_cntl: u32,
    pub sdmax_rlcx_rb_wptr_poll_addr_hi: u32, pub sdmax_rlcx_rb_wptr_poll_addr_lo: u32,
    pub sdmax_rlcx_rb_rptr_addr_hi: u32, pub sdmax_rlcx_rb_rptr_addr_lo: u32,
    pub sdmax_rlcx_ib_cntl: u32, pub sdmax_rlcx_ib_rptr: u32,
    pub sdmax_rlcx_ib_offset: u32, pub sdmax_rlcx_ib_base_lo: u32,
    pub sdmax_rlcx_ib_base_hi: u32, pub sdmax_rlcx_ib_size: u32,
    pub sdmax_rlcx_skip_cntl: u32, pub sdmax_rlcx_context_status: u32,
    pub sdmax_rlcx_doorbell: u32, pub sdmax_rlcx_virtual_addr: u32,
    pub sdmax_rlcx_ape1_cntl: u32, pub sdmax_rlcx_doorbell_log: u32,
    pub reserved: [u32; 104],
    /* reserved_126,127: repurposed for driver-internal use */
    pub sdma_engine_id: u32, pub sdma_queue_id: u32,
}

#[repr(C)]
pub struct vi_mqd {
    pub header: u32, pub compute_dispatch_initiator: u32,
    pub compute_dim_x: u32, pub compute_dim_y: u32, pub compute_dim_z: u32,
    pub compute_start_x: u32, pub compute_start_y: u32, pub compute_start_z: u32,
    pub compute_num_thread_x: u32, pub compute_num_thread_y: u32, pub compute_num_thread_z: u32,
    pub compute_pipelinestat_enable: u32, pub compute_perfcount_enable: u32,
    pub compute_pgm_lo: u32, pub compute_pgm_hi: u32, pub compute_tba_lo: u32, pub compute_tba_hi: u32,
    pub compute_tma_lo: u32, pub compute_tma_hi: u32, pub compute_pgm_rsrc1: u32, pub compute_pgm_rsrc2: u32,
    pub compute_vmid: u32, pub compute_resource_limits: u32,
    pub compute_static_thread_mgmt_se0: u32, pub compute_static_thread_mgmt_se1: u32,
    pub compute_tmpring_size: u32, pub compute_static_thread_mgmt_se2: u32, pub compute_static_thread_mgmt_se3: u32,
    pub compute_restart_x: u32, pub compute_restart_y: u32, pub compute_restart_z: u32,
    pub compute_thread_trace_enable: u32, pub compute_misc_reserved: u32, pub compute_dispatch_id: u32,
    pub compute_threadgroup_id: u32, pub compute_relaunch: u32,
    pub compute_wave_restore_addr_lo: u32, pub compute_wave_restore_addr_hi: u32, pub compute_wave_restore_control: u32,
    pub reserved0: [u32; 27],
    pub compute_user_data: [u32; 16],
    pub cp_compute_csinvoc_count_lo: u32, pub cp_compute_csinvoc_count_hi: u32,
    pub reserved35_37: [u32; 3],
    pub cp_mqd_query_time_lo: u32, pub cp_mqd_query_time_hi: u32,
    pub cp_mqd_connect_start_time_lo: u32, pub cp_mqd_connect_start_time_hi: u32,
    pub cp_mqd_connect_end_time_lo: u32, pub cp_mqd_connect_end_time_hi: u32,
    pub cp_mqd_connect_end_wf_count: u32, pub cp_mqd_connect_end_pq_rptr: u32,
    pub cp_mqd_connect_endvi_sdma_mqd_pq_wptr: u32, pub cp_mqd_connect_end_ib_rptr: u32,
    pub reserved38_39: [u32; 2],
    pub cp_mqd_save_start_time_lo: u32, pub cp_mqd_save_start_time_hi: u32,
    pub cp_mqd_save_end_time_lo: u32, pub cp_mqd_save_end_time_hi: u32,
    pub cp_mqd_restore_start_time_lo: u32, pub cp_mqd_restore_start_time_hi: u32,
    pub cp_mqd_restore_end_time_lo: u32, pub cp_mqd_restore_end_time_hi: u32,
    pub disable_queue: u32, pub reserved41: u32,
    pub gds_cs_ctxsw_cnt: [u32; 4], pub reserved42_43: [u32; 2],
    pub cp_pq_exe_status_lo: u32, pub cp_pq_exe_status_hi: u32,
    pub cp_packet_id_lo: u32, pub cp_packet_id_hi: u32,
    pub cp_packet_exe_status_lo: u32, pub cp_packet_exe_status_hi: u32,
    pub gds_save_base_addr_lo: u32, pub gds_save_base_addr_hi: u32,
    pub gds_save_mask_lo: u32, pub gds_save_mask_hi: u32,
    pub ctx_save_base_addr_lo: u32, pub ctx_save_base_addr_hi: u32,
    pub dynamic_cu_mask_addr_lo: u32, pub dynamic_cu_mask_addr_hi: u32,
    pub cp_mqd_base_addr_lo: u32, pub cp_mqd_base_addr_hi: u32,
    pub cp_hqd_active: u32, pub cp_hqd_vmid: u32, pub cp_hqd_persistent_state: u32,
    pub cp_hqd_pipe_priority: u32, pub cp_hqd_queue_priority: u32, pub cp_hqd_quantum: u32,
    pub cp_hqd_pq_base_lo: u32, pub cp_hqd_pq_base_hi: u32, pub cp_hqd_pq_rptr: u32,
    pub cp_hqd_pq_rptr_report_addr_lo: u32, pub cp_hqd_pq_rptr_report_addr_hi: u32,
    pub cp_hqd_pq_wptr_poll_addr_lo: u32, pub cp_hqd_pq_wptr_poll_addr_hi: u32,
    pub cp_hqd_pq_doorbell_control: u32, pub cp_hqd_pq_wptr: u32, pub cp_hqd_pq_control: u32,
    pub cp_hqd_ib_base_addr_lo: u32, pub cp_hqd_ib_base_addr_hi: u32, pub cp_hqd_ib_rptr: u32,
    pub cp_hqd_ib_control: u32, pub cp_hqd_iq_timer: u32, pub cp_hqd_iq_rptr: u32,
    pub cp_hqd_dequeue_request: u32, pub cp_hqd_dma_offload: u32, pub cp_hqd_sema_cmd: u32,
    pub cp_hqd_msg_type: u32, pub cp_hqd_atomic0_preop_lo: u32, pub cp_hqd_atomic0_preop_hi: u32,
    pub cp_hqd_atomic1_preop_lo: u32, pub cp_hqd_atomic1_preop_hi: u32,
    pub cp_hqd_hq_status0: u32, pub cp_hqd_hq_control0: u32, pub cp_mqd_control: u32,
    pub cp_hqd_hq_status1: u32, pub cp_hqd_hq_control1: u32,
    pub cp_hqd_eop_base_addr_lo: u32, pub cp_hqd_eop_base_addr_hi: u32, pub cp_hqd_eop_control: u32,
    pub cp_hqd_eop_rptr: u32, pub cp_hqd_eop_wptr: u32, pub cp_hqd_eop_done_events: u32,
    pub cp_hqd_ctx_save_base_addr_lo: u32, pub cp_hqd_ctx_save_base_addr_hi: u32,
    pub cp_hqd_ctx_save_control: u32, pub cp_hqd_cntl_stack_offset: u32, pub cp_hqd_cntl_stack_size: u32,
    pub cp_hqd_wg_state_offset: u32, pub cp_hqd_ctx_save_size: u32, pub cp_hqd_gds_resource_state: u32,
    pub cp_hqd_error: u32, pub cp_hqd_eop_wptr_mem: u32, pub cp_hqd_eop_dones: u32,
    pub reserved46_58: [u32; 13], pub iqtimer_pkt: [u32; 32], pub reserved56_58: [u32; 3],
    pub set_resources: [u32; 8], pub reserved59_62: [u32; 4], pub queue_doorbell_id: [u32; 16],
    pub reserved_t: [u32; 256],
}

#[repr(C)]
pub struct vi_mqd_allocation { pub mqd: vi_mqd, pub wptr_poll_mem: u32, pub rptr_report_mem: u32, pub dynamic_cu_mask: u32, pub dynamic_rb_mask: u32 }

#[repr(C)]
pub struct vi_ce_ib_state { pub ce_ib_completion_status: u32, pub ce_constegnine_count: u32, pub ce_ibOffset_ib1: u32, pub ce_ibOffset_ib2: u32 }

#[repr(C)]
pub struct vi_de_ib_state {
    pub ib_completion_status: u32, pub de_constEngine_count: u32, pub ib_offset_ib1: u32, pub ib_offset_ib2: u32,
    pub preamble_begin_ib1: u32, pub preamble_begin_ib2: u32, pub preamble_end_ib1: u32, pub preamble_end_ib2: u32,
    pub draw_indirect_baseLo: u32, pub draw_indirect_baseHi: u32, pub disp_indirect_baseLo: u32, pub disp_indirect_baseHi: u32,
    pub gds_backup_addrlo: u32, pub gds_backup_addrhi: u32, pub index_base_addrlo: u32, pub index_base_addrhi: u32, pub sample_cntl: u32,
}

#[repr(C)]
pub struct vi_ce_ib_state_chained_ib {
    pub ce_ib_completion_status: u32, pub ce_constegnine_count: u32, pub ce_ibOffset_ib1: u32, pub ce_ibOffset_ib2: u32,
    pub ce_chainib_addrlo_ib1: u32, pub ce_chainib_addrlo_ib2: u32, pub ce_chainib_addrhi_ib1: u32, pub ce_chainib_addrhi_ib2: u32,
    pub ce_chainib_size_ib1: u32, pub ce_chainib_size_ib2: u32,
}

#[repr(C)]
pub struct vi_de_ib_state_chained_ib {
    pub ib_completion_status: u32, pub de_constEngine_count: u32, pub ib_offset_ib1: u32, pub ib_offset_ib2: u32,
    pub chain_ib_addrlo_ib1: u32, pub chain_ib_addrlo_ib2: u32, pub chain_ib_addrhi_ib1: u32, pub chain_ib_addrhi_ib2: u32,
    pub chain_ib_size_ib1: u32, pub chain_ib_size_ib2: u32,
    pub preamble_begin_ib1: u32, pub preamble_begin_ib2: u32, pub preamble_end_ib1: u32, pub preamble_end_ib2: u32,
    pub chain_ib_pream_addrlo_ib1: u32, pub chain_ib_pream_addrlo_ib2: u32, pub chain_ib_pream_addrhi_ib1: u32, pub chain_ib_pream_addrhi_ib2: u32,
    pub draw_indirect_baseLo: u32, pub draw_indirect_baseHi: u32, pub disp_indirect_baseLo: u32, pub disp_indirect_baseHi: u32,
    pub gds_backup_addrlo: u32, pub gds_backup_addrhi: u32, pub index_base_addrlo: u32, pub index_base_addrhi: u32, pub sample_cntl: u32,
}

#[repr(C)]
pub struct vi_gfx_meta_data {
    pub ce_payload: vi_ce_ib_state, pub reserved1: [u32; 60], pub de_payload: vi_de_ib_state,
    pub DeIbBaseAddrLo: u32, pub DeIbBaseAddrHi: u32, pub reserved2: [u32; 941],
}

#[repr(C)]
pub struct vi_gfx_meta_data_chained_ib {
    pub ce_payload: vi_ce_ib_state_chained_ib, pub reserved1: [u32; 54], pub de_payload: vi_de_ib_state_chained_ib,
    pub DeIbBaseAddrLo: u32, pub DeIbBaseAddrHi: u32, pub reserved2: [u32; 931],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
