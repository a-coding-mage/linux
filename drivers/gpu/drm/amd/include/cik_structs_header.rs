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
pub struct cik_mqd {
    pub header: u32,
    pub compute_dispatch_initiator: u32,
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
    pub compute_thread_trace_enable: u32, pub compute_misc_reserved: u32,
    pub compute_user_data_0: u32, pub compute_user_data_1: u32, pub compute_user_data_2: u32, pub compute_user_data_3: u32,
    pub compute_user_data_4: u32, pub compute_user_data_5: u32, pub compute_user_data_6: u32, pub compute_user_data_7: u32,
    pub compute_user_data_8: u32, pub compute_user_data_9: u32, pub compute_user_data_10: u32, pub compute_user_data_11: u32,
    pub compute_user_data_12: u32, pub compute_user_data_13: u32, pub compute_user_data_14: u32, pub compute_user_data_15: u32,
    pub cp_compute_csinvoc_count_lo: u32, pub cp_compute_csinvoc_count_hi: u32,
    pub cp_mqd_base_addr_lo: u32, pub cp_mqd_base_addr_hi: u32, pub cp_hqd_active: u32, pub cp_hqd_vmid: u32,
    pub cp_hqd_persistent_state: u32, pub cp_hqd_pipe_priority: u32, pub cp_hqd_queue_priority: u32, pub cp_hqd_quantum: u32,
    pub cp_hqd_pq_base_lo: u32, pub cp_hqd_pq_base_hi: u32, pub cp_hqd_pq_rptr: u32,
    pub cp_hqd_pq_rptr_report_addr_lo: u32, pub cp_hqd_pq_rptr_report_addr_hi: u32,
    pub cp_hqd_pq_wptr_poll_addr_lo: u32, pub cp_hqd_pq_wptr_poll_addr_hi: u32,
    pub cp_hqd_pq_doorbell_control: u32, pub cp_hqd_pq_wptr: u32, pub cp_hqd_pq_control: u32,
    pub cp_hqd_ib_base_addr_lo: u32, pub cp_hqd_ib_base_addr_hi: u32, pub cp_hqd_ib_rptr: u32,
    pub cp_hqd_ib_control: u32, pub cp_hqd_iq_timer: u32, pub cp_hqd_iq_rptr: u32,
    pub cp_hqd_dequeue_request: u32, pub cp_hqd_dma_offload: u32, pub cp_hqd_sema_cmd: u32, pub cp_hqd_msg_type: u32,
    pub cp_hqd_atomic0_preop_lo: u32, pub cp_hqd_atomic0_preop_hi: u32, pub cp_hqd_atomic1_preop_lo: u32, pub cp_hqd_atomic1_preop_hi: u32,
    pub cp_hqd_hq_status0: u32, pub cp_hqd_hq_control0: u32, pub cp_mqd_control: u32,
    pub cp_mqd_query_time_lo: u32, pub cp_mqd_query_time_hi: u32,
    pub cp_mqd_connect_start_time_lo: u32, pub cp_mqd_connect_start_time_hi: u32,
    pub cp_mqd_connect_end_time_lo: u32, pub cp_mqd_connect_end_time_hi: u32,
    pub cp_mqd_connect_end_wf_count: u32, pub cp_mqd_connect_end_pq_rptr: u32, pub cp_mqd_connect_end_pq_wptr: u32,
    pub cp_mqd_connect_end_ib_rptr: u32,
    pub reserved_96: u32, pub reserved_97: u32, pub reserved_98: u32, pub reserved_99: u32,
    pub iqtimer_pkt_header: u32, pub iqtimer_pkt_dw0: u32, pub iqtimer_pkt_dw1: u32, pub iqtimer_pkt_dw2: u32,
    pub iqtimer_pkt_dw3: u32, pub iqtimer_pkt_dw4: u32, pub iqtimer_pkt_dw5: u32, pub iqtimer_pkt_dw6: u32,
    pub reserved_108: u32, pub reserved_109: u32, pub reserved_110: u32, pub reserved_111: u32,
    pub queue_doorbell_id0: u32, pub queue_doorbell_id1: u32, pub queue_doorbell_id2: u32, pub queue_doorbell_id3: u32,
    pub queue_doorbell_id4: u32, pub queue_doorbell_id5: u32, pub queue_doorbell_id6: u32, pub queue_doorbell_id7: u32,
    pub queue_doorbell_id8: u32, pub queue_doorbell_id9: u32, pub queue_doorbell_id10: u32, pub queue_doorbell_id11: u32,
    pub queue_doorbell_id12: u32, pub queue_doorbell_id13: u32, pub queue_doorbell_id14: u32, pub queue_doorbell_id15: u32,
}

#[repr(C)]
pub struct cik_sdma_rlc_registers {
    pub sdma_rlc_rb_cntl: u32, pub sdma_rlc_rb_base: u32, pub sdma_rlc_rb_base_hi: u32,
    pub sdma_rlc_rb_rptr: u32, pub sdma_rlc_rb_wptr: u32, pub sdma_rlc_rb_wptr_poll_cntl: u32,
    pub sdma_rlc_rb_wptr_poll_addr_hi: u32, pub sdma_rlc_rb_wptr_poll_addr_lo: u32,
    pub sdma_rlc_rb_rptr_addr_hi: u32, pub sdma_rlc_rb_rptr_addr_lo: u32,
    pub sdma_rlc_ib_cntl: u32, pub sdma_rlc_ib_rptr: u32, pub sdma_rlc_ib_offset: u32,
    pub sdma_rlc_ib_base_lo: u32, pub sdma_rlc_ib_base_hi: u32, pub sdma_rlc_ib_size: u32,
    pub sdma_rlc_skip_cntl: u32, pub sdma_rlc_context_status: u32, pub sdma_rlc_doorbell: u32,
    pub sdma_rlc_virtual_addr: u32, pub sdma_rlc_ape1_cntl: u32, pub sdma_rlc_doorbell_log: u32,
    pub reserved_22: u32, pub reserved_23: u32, pub reserved_24: u32, pub reserved_25: u32,
    pub reserved_26: u32, pub reserved_27: u32, pub reserved_28: u32, pub reserved_29: u32,
    pub reserved_30: u32, pub reserved_31: u32, pub reserved_32: u32, pub reserved_33: u32,
    pub reserved_34: u32, pub reserved_35: u32, pub reserved_36: u32, pub reserved_37: u32,
    pub reserved_38: u32, pub reserved_39: u32, pub reserved_40: u32, pub reserved_41: u32,
    pub reserved_42: u32, pub reserved_43: u32, pub reserved_44: u32, pub reserved_45: u32,
    pub reserved_46: u32, pub reserved_47: u32, pub reserved_48: u32, pub reserved_49: u32,
    pub reserved_50: u32, pub reserved_51: u32, pub reserved_52: u32, pub reserved_53: u32,
    pub reserved_54: u32, pub reserved_55: u32, pub reserved_56: u32, pub reserved_57: u32,
    pub reserved_58: u32, pub reserved_59: u32, pub reserved_60: u32, pub reserved_61: u32,
    pub reserved_62: u32, pub reserved_63: u32, pub reserved_64: u32, pub reserved_65: u32,
    pub reserved_66: u32, pub reserved_67: u32, pub reserved_68: u32, pub reserved_69: u32,
    pub reserved_70: u32, pub reserved_71: u32, pub reserved_72: u32, pub reserved_73: u32,
    pub reserved_74: u32, pub reserved_75: u32, pub reserved_76: u32, pub reserved_77: u32,
    pub reserved_78: u32, pub reserved_79: u32, pub reserved_80: u32, pub reserved_81: u32,
    pub reserved_82: u32, pub reserved_83: u32, pub reserved_84: u32, pub reserved_85: u32,
    pub reserved_86: u32, pub reserved_87: u32, pub reserved_88: u32, pub reserved_89: u32,
    pub reserved_90: u32, pub reserved_91: u32, pub reserved_92: u32, pub reserved_93: u32,
    pub reserved_94: u32, pub reserved_95: u32, pub reserved_96: u32, pub reserved_97: u32,
    pub reserved_98: u32, pub reserved_99: u32, pub reserved_100: u32, pub reserved_101: u32,
    pub reserved_102: u32, pub reserved_103: u32, pub reserved_104: u32, pub reserved_105: u32,
    pub reserved_106: u32, pub reserved_107: u32, pub reserved_108: u32, pub reserved_109: u32,
    pub reserved_110: u32, pub reserved_111: u32, pub reserved_112: u32, pub reserved_113: u32,
    pub reserved_114: u32, pub reserved_115: u32, pub reserved_116: u32, pub reserved_117: u32,
    pub reserved_118: u32, pub reserved_119: u32, pub reserved_120: u32, pub reserved_121: u32,
    pub reserved_122: u32, pub reserved_123: u32, pub reserved_124: u32, pub reserved_125: u32,
    /* reserved_126,127: repurposed for driver-internal use */
    pub sdma_engine_id: u32,
    pub sdma_queue_id: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
