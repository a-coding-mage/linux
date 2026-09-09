// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub unsafe fn pm_build_pm4_header(opcode: u32, packet_size: usize) -> u32 {
    let mut header: PM4_MES_TYPE_3_HEADER = core::mem::zeroed();
    header.u32All = 0;
    header.opcode = opcode;
    header.count = (packet_size / 4 - 2) as _;
    header.type_ = PM4_TYPE_3;
    header.u32All
}

unsafe fn pm_map_process_vi(
    pm: *mut packet_manager,
    buffer: *mut u32,
    qpd: *mut qcm_process_device,
) -> i32 {
    let pdd = qpd_to_pdd(qpd);
    let packet = buffer as *mut pm4_mes_map_process;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_map_process>());
    (*packet).header.u32All = pm_build_pm4_header(IT_MAP_PROCESS, core::mem::size_of::<pm4_mes_map_process>());
    (*packet).bitfields2.diq_enable = if (*qpd).is_debug { 1 } else { 0 };
    (*packet).bitfields2.process_quantum = 10;
    (*packet).bitfields2.pasid = (*pdd).pasid;
    (*packet).bitfields3.page_table_base = (*qpd).page_table_base;
    (*packet).bitfields10.gds_size = (*qpd).gds_size;
    (*packet).bitfields10.num_gws = (*qpd).num_gws;
    (*packet).bitfields10.num_oac = (*qpd).num_oac;
    (*packet).bitfields10.num_queues = if (*qpd).is_debug { 0 } else { (*qpd).queue_count };
    (*packet).sh_mem_config = (*qpd).sh_mem_config;
    (*packet).sh_mem_bases = (*qpd).sh_mem_bases;
    (*packet).sh_mem_ape1_base = (*qpd).sh_mem_ape1_base;
    (*packet).sh_mem_ape1_limit = (*qpd).sh_mem_ape1_limit;
    (*packet).sh_hidden_private_base_vmid = (*qpd).sh_hidden_private_base;
    (*packet).gds_addr_lo = lower_32_bits((*qpd).gds_context_area);
    (*packet).gds_addr_hi = upper_32_bits((*qpd).gds_context_area);
    0
}

unsafe fn pm_runlist_vi(pm: *mut packet_manager, buffer: *mut u32, ib: u64, ib_size_in_dwords: usize, chain: bool) -> i32 {
    if ib == 0 { return -EFAULT; }
    let concurrent_proc_cnt = core::cmp::min((*(*pm).dqm).processes_count, (*(*pm).dqm).dev.as_ref().unwrap().max_proc_per_quantum);
    let packet = buffer as *mut pm4_mes_runlist;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_runlist>());
    (*packet).header.u32All = pm_build_pm4_header(IT_RUN_LIST, core::mem::size_of::<pm4_mes_runlist>());
    (*packet).bitfields4.ib_size = ib_size_in_dwords;
    (*packet).bitfields4.chain = if chain { 1 } else { 0 };
    (*packet).bitfields4.offload_polling = 0;
    (*packet).bitfields4.valid = 1;
    (*packet).bitfields4.process_cnt = concurrent_proc_cnt;
    (*packet).ordinal2 = lower_32_bits(ib);
    (*packet).bitfields3.ib_base_hi = upper_32_bits(ib);
    0
}

unsafe fn pm_set_resources_vi(pm: *mut packet_manager, buffer: *mut u32, res: *mut scheduling_resources) -> i32 {
    let packet = buffer as *mut pm4_mes_set_resources;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_set_resources>());
    (*packet).header.u32All = pm_build_pm4_header(IT_SET_RESOURCES, core::mem::size_of::<pm4_mes_set_resources>());
    (*packet).bitfields2.queue_type = queue_type__mes_set_resources__hsa_interface_queue_hiq;
    (*packet).bitfields2.vmid_mask = (*res).vmid_mask;
    (*packet).bitfields2.unmap_latency = KFD_UNMAP_LATENCY_MS / 100;
    (*packet).bitfields7.oac_mask = (*res).oac_mask;
    (*packet).bitfields8.gds_heap_base = (*res).gds_heap_base;
    (*packet).bitfields8.gds_heap_size = (*res).gds_heap_size;
    (*packet).gws_mask_lo = lower_32_bits((*res).gws_mask);
    (*packet).gws_mask_hi = upper_32_bits((*res).gws_mask);
    (*packet).queue_mask_lo = lower_32_bits((*res).queue_mask);
    (*packet).queue_mask_hi = upper_32_bits((*res).queue_mask);
    0
}

unsafe fn pm_map_queues_vi(pm: *mut packet_manager, buffer: *mut u32, q: *mut queue, is_static: bool) -> i32 {
    let packet = buffer as *mut pm4_mes_map_queues;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_map_queues>());
    (*packet).header.u32All = pm_build_pm4_header(IT_MAP_QUEUES, core::mem::size_of::<pm4_mes_map_queues>());
    (*packet).bitfields2.num_queues = 1;
    (*packet).bitfields2.queue_sel = queue_sel__mes_map_queues__map_to_hws_determined_queue_slots_vi;
    (*packet).bitfields2.engine_sel = engine_sel__mes_map_queues__compute_vi;
    (*packet).bitfields2.queue_type = queue_type__mes_map_queues__normal_compute_vi;
    match (*q).properties.type_ {
        KFD_QUEUE_TYPE_COMPUTE => if is_static { (*packet).bitfields2.queue_type = queue_type__mes_map_queues__normal_latency_static_queue_vi },
        KFD_QUEUE_TYPE_SDMA | KFD_QUEUE_TYPE_SDMA_XGMI => (*packet).bitfields2.engine_sel = (*q).properties.sdma_engine_id + engine_sel__mes_map_queues__sdma0_vi,
        _ => return -EINVAL,
    }
    (*packet).bitfields3.doorbell_offset = (*q).properties.doorbell_off;
    (*packet).mqd_addr_lo = lower_32_bits((*q).gart_mqd_addr);
    (*packet).mqd_addr_hi = upper_32_bits((*q).gart_mqd_addr);
    (*packet).wptr_addr_lo = lower_32_bits((*q).properties.write_ptr as u64);
    (*packet).wptr_addr_hi = upper_32_bits((*q).properties.write_ptr as u64);
    0
}

unsafe fn pm_unmap_queues_vi(pm: *mut packet_manager, buffer: *mut u32, filter: kfd_unmap_queues_filter, filter_param: u32, reset: bool) -> i32 {
    let packet = buffer as *mut pm4_mes_unmap_queues;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_unmap_queues>());
    (*packet).header.u32All = pm_build_pm4_header(IT_UNMAP_QUEUES, core::mem::size_of::<pm4_mes_unmap_queues>());
    (*packet).bitfields2.engine_sel = engine_sel__mes_unmap_queues__compute;
    (*packet).bitfields2.action = if reset { action__mes_unmap_queues__reset_queues } else { action__mes_unmap_queues__preempt_queues };
    match filter {
        KFD_UNMAP_QUEUES_FILTER_BY_PASID => { (*packet).bitfields2.queue_sel = queue_sel__mes_unmap_queues__perform_request_on_pasid_queues; (*packet).bitfields3a.pasid = filter_param; },
        KFD_UNMAP_QUEUES_FILTER_ALL_QUEUES => (*packet).bitfields2.queue_sel = queue_sel__mes_unmap_queues__unmap_all_queues,
        KFD_UNMAP_QUEUES_FILTER_DYNAMIC_QUEUES => (*packet).bitfields2.queue_sel = queue_sel__mes_unmap_queues__unmap_all_non_static_queues,
        _ => return -EINVAL,
    }
    0
}

unsafe fn pm_query_status_vi(pm: *mut packet_manager, buffer: *mut u32, fence_address: u64, fence_value: u64) -> i32 {
    let packet = buffer as *mut pm4_mes_query_status;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_query_status>());
    (*packet).header.u32All = pm_build_pm4_header(IT_QUERY_STATUS, core::mem::size_of::<pm4_mes_query_status>());
    (*packet).bitfields2.context_id = 0;
    (*packet).bitfields2.interrupt_sel = interrupt_sel__mes_query_status__completion_status;
    (*packet).bitfields2.command = command__mes_query_status__fence_only_after_write_ack;
    (*packet).addr_hi = upper_32_bits(fence_address); (*packet).addr_lo = lower_32_bits(fence_address);
    (*packet).data_hi = upper_32_bits(fence_value); (*packet).data_lo = lower_32_bits(fence_value);
    0
}

unsafe fn pm_release_mem_vi(gpu_addr: u64, buffer: *mut u32) -> i32 {
    let packet = buffer as *mut pm4_mec_release_mem;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mec_release_mem>());
    (*packet).header.u32All = pm_build_pm4_header(IT_RELEASE_MEM, core::mem::size_of::<pm4_mec_release_mem>());
    (*packet).bitfields2.event_type = CACHE_FLUSH_AND_INV_TS_EVENT;
    (*packet).bitfields2.event_index = event_index___release_mem__end_of_pipe;
    (*packet).bitfields2.tcl1_action_ena = 1; (*packet).bitfields2.tc_action_ena = 1;
    (*packet).bitfields2.cache_policy = cache_policy___release_mem__lru; (*packet).bitfields2.atc = 0;
    (*packet).bitfields3.data_sel = data_sel___release_mem__send_32_bit_low;
    (*packet).bitfields3.int_sel = int_sel___release_mem__send_interrupt_after_write_confirm;
    (*packet).bitfields4.address_lo_32b = ((gpu_addr & 0xffff_ffff) >> 2) as _;
    (*packet).address_hi = upper_32_bits(gpu_addr); (*packet).data_lo = 0;
    0
}

pub static kfd_vi_pm_funcs: packet_manager_funcs = packet_manager_funcs {
    map_process: Some(pm_map_process_vi), runlist: Some(pm_runlist_vi), set_resources: Some(pm_set_resources_vi),
    map_queues: Some(pm_map_queues_vi), unmap_queues: Some(pm_unmap_queues_vi), config_dequeue_wait_counts: None,
    query_status: Some(pm_query_status_vi), release_mem: Some(pm_release_mem_vi),
    map_process_size: core::mem::size_of::<pm4_mes_map_process>(), runlist_size: core::mem::size_of::<pm4_mes_runlist>(),
    set_resources_size: core::mem::size_of::<pm4_mes_set_resources>(), map_queues_size: core::mem::size_of::<pm4_mes_map_queues>(),
    unmap_queues_size: core::mem::size_of::<pm4_mes_unmap_queues>(), config_dequeue_wait_counts_size: 0,
    query_status_size: core::mem::size_of::<pm4_mes_query_status>(), release_mem_size: core::mem::size_of::<pm4_mec_release_mem>(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
