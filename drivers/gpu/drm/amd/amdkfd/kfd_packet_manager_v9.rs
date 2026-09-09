// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2016-2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

unsafe fn pm_map_process_v9(pm: *mut packet_manager, buffer: *mut u32, qpd: *mut qcm_process_device) -> i32 {
    let packet = buffer as *mut pm4_mes_map_process;
    let vm_page_table_base_addr = (*qpd).page_table_base;
    let kfd = (*(*pm).dqm).dev;
    let pdd = container_of(qpd, kfd_process_device, qpd);
    let adev = (*kfd).adev;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_map_process>());
    (*packet).header.u32All = pm_build_pm4_header(IT_MAP_PROCESS, core::mem::size_of::<pm4_mes_map_process>());
    if (*adev).enforce_isolation[(*kfd).node_id] == AMDGPU_ENFORCE_ISOLATION_ENABLE { (*packet).bitfields2.exec_cleaner_shader = 1; }
    (*packet).bitfields2.diq_enable = if (*qpd).is_debug { 1 } else { 0 };
    (*packet).bitfields2.process_quantum = 10;
    (*packet).bitfields2.pasid = (*pdd).pasid;
    (*packet).bitfields14.gds_size = (*qpd).gds_size & 0x3f;
    (*packet).bitfields14.gds_size_hi = ((*qpd).gds_size >> 6) & 0xf;
    (*packet).bitfields14.num_gws = if (*qpd).mapped_gws_queue { (*qpd).num_gws } else { 0 };
    (*packet).bitfields14.num_oac = (*qpd).num_oac;
    (*packet).bitfields14.sdma_enable = 1;
    (*packet).bitfields14.num_queues = if (*qpd).is_debug { 0 } else { (*qpd).queue_count };
    if (*(*kfd).dqm).trap_debug_vmid != 0 && (*(*pdd).process).debug_trap_enabled && (*(*pdd).process).runtime_info.runtime_state == DEBUG_RUNTIME_STATE_ENABLED {
        (*packet).bitfields2.debug_vmid = (*(*kfd).dqm).trap_debug_vmid; (*packet).bitfields2.new_debug = 1;
    }
    (*packet).sh_mem_config = (*qpd).sh_mem_config; (*packet).sh_mem_bases = (*qpd).sh_mem_bases;
    if (*qpd).tba_addr != 0 {
        (*packet).sq_shader_tba_lo = lower_32_bits((*qpd).tba_addr >> 8);
        (*packet).sq_shader_tba_hi = upper_32_bits((*qpd).tba_addr >> 8) | (1 << SQ_SHADER_TBA_HI__TRAP_EN__SHIFT);
        (*packet).sq_shader_tma_lo = lower_32_bits((*qpd).tma_addr >> 8); (*packet).sq_shader_tma_hi = upper_32_bits((*qpd).tma_addr >> 8);
    }
    (*packet).gds_addr_lo = lower_32_bits((*qpd).gds_context_area); (*packet).gds_addr_hi = upper_32_bits((*qpd).gds_context_area);
    (*packet).vm_context_page_table_base_addr_lo32 = lower_32_bits(vm_page_table_base_addr);
    (*packet).vm_context_page_table_base_addr_hi32 = upper_32_bits(vm_page_table_base_addr); 0
}

unsafe fn pm_map_process_aldebaran(pm: *mut packet_manager, buffer: *mut u32, qpd: *mut qcm_process_device) -> i32 {
    let packet = buffer as *mut pm4_mes_map_process_aldebaran;
    let vm_page_table_base_addr = (*qpd).page_table_base;
    let kfd = (*(*(*pm).dqm).dev).kfd; let knode = (*(*pm).dqm).dev;
    let pdd = container_of(qpd, kfd_process_device, qpd); let adev = (*kfd).adev;
    core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_map_process_aldebaran>());
    (*packet).header.u32All = pm_build_pm4_header(IT_MAP_PROCESS, core::mem::size_of::<pm4_mes_map_process_aldebaran>());
    if (*adev).enforce_isolation[(*knode).node_id] == AMDGPU_ENFORCE_ISOLATION_ENABLE { (*packet).bitfields2.exec_cleaner_shader = 1; }
    (*packet).bitfields2.diq_enable = if (*qpd).is_debug { 1 } else { 0 }; (*packet).bitfields2.process_quantum = 10;
    (*packet).bitfields2.pasid = (*pdd).pasid; (*packet).bitfields14.gds_size = (*qpd).gds_size & 0x3f;
    (*packet).bitfields14.gds_size_hi = ((*qpd).gds_size >> 6) & 0xf; (*packet).bitfields14.num_gws = if (*qpd).mapped_gws_queue { (*qpd).num_gws } else { 0 };
    (*packet).bitfields14.num_oac = (*qpd).num_oac; (*packet).bitfields14.sdma_enable = 1; (*packet).bitfields14.num_queues = if (*qpd).is_debug { 0 } else { (*qpd).queue_count };
    (*packet).spi_gdbg_per_vmid_cntl = (*pdd).spi_dbg_override | (*pdd).spi_dbg_launch_mode;
    if (*(*pdd).process).debug_trap_enabled { for i in 0..(*kfd).device_info.num_of_watch_points { (*packet).tcp_watch_cntl[i] = (*pdd).watch_points[i]; } (*packet).bitfields2.single_memops = if (*(*pdd).process).dbg_flags & KFD_DBG_TRAP_FLAG_SINGLE_MEM_OP != 0 { 1 } else { 0 }; }
    (*packet).sh_mem_config = (*qpd).sh_mem_config; (*packet).sh_mem_bases = (*qpd).sh_mem_bases;
    if (*qpd).tba_addr != 0 { (*packet).sq_shader_tba_lo = lower_32_bits((*qpd).tba_addr >> 8); (*packet).sq_shader_tba_hi = upper_32_bits((*qpd).tba_addr >> 8); (*packet).sq_shader_tma_lo = lower_32_bits((*qpd).tma_addr >> 8); (*packet).sq_shader_tma_hi = upper_32_bits((*qpd).tma_addr >> 8); }
    (*packet).gds_addr_lo = lower_32_bits((*qpd).gds_context_area); (*packet).gds_addr_hi = upper_32_bits((*qpd).gds_context_area);
    (*packet).vm_context_page_table_base_addr_lo32 = lower_32_bits(vm_page_table_base_addr); (*packet).vm_context_page_table_base_addr_hi32 = upper_32_bits(vm_page_table_base_addr); 0
}

unsafe fn pm_runlist_v9(pm: *mut packet_manager, buffer: *mut u32, ib: u64, ib_size_in_dwords: usize, chain: bool) -> i32 {
    let kfd = (*(*pm).dqm).dev; let adev = (*kfd).adev;
    let concurrent_proc_cnt = if (*adev).enforce_isolation[(*kfd).node_id] == AMDGPU_ENFORCE_ISOLATION_ENABLE { 1 } else { min((*(*pm).dqm).processes_count, (*kfd).max_proc_per_quantum) };
    let packet = buffer as *mut pm4_mes_runlist; core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_runlist>());
    (*packet).header.u32All = pm_build_pm4_header(IT_RUN_LIST, core::mem::size_of::<pm4_mes_runlist>()); (*packet).bitfields4.ib_size = ib_size_in_dwords;
    (*packet).bitfields4.chain = if chain { 1 } else { 0 }; (*packet).bitfields4.offload_polling = 0; (*packet).bitfields4.chained_runlist_idle_disable = if chain { 1 } else { 0 }; (*packet).bitfields4.valid = 1; (*packet).bitfields4.process_cnt = concurrent_proc_cnt; (*packet).ordinal2 = lower_32_bits(ib); (*packet).ib_base_hi = upper_32_bits(ib); 0
}

unsafe fn pm_set_resources_v9(pm: *mut packet_manager, buffer: *mut u32, res: *mut scheduling_resources) -> i32 {
    let packet = buffer as *mut pm4_mes_set_resources; core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_set_resources>()); (*packet).header.u32All = pm_build_pm4_header(IT_SET_RESOURCES, core::mem::size_of::<pm4_mes_set_resources>());
    (*packet).bitfields2.queue_type = queue_type__mes_set_resources__hsa_interface_queue_hiq; (*packet).bitfields2.vmid_mask = (*res).vmid_mask; (*packet).bitfields2.unmap_latency = KFD_UNMAP_LATENCY_MS / 100;
    if (*(*(*pm).dqm).dev).adev.gmc.xnack_flags & AMDGPU_GMC_XNACK_FLAG_CHAIN != 0 { (*packet).bitfields2.enb_xnack_retry_disable_check = 1; }
    (*packet).bitfields7.oac_mask = (*res).oac_mask; (*packet).bitfields8.gds_heap_base = (*res).gds_heap_base; (*packet).bitfields8.gds_heap_size = (*res).gds_heap_size; (*packet).gws_mask_lo = lower_32_bits((*res).gws_mask); (*packet).gws_mask_hi = upper_32_bits((*res).gws_mask); (*packet).queue_mask_lo = lower_32_bits((*res).queue_mask); (*packet).queue_mask_hi = upper_32_bits((*res).queue_mask); 0
}

unsafe fn pm_use_ext_eng(dev: *mut kfd_dev) -> bool { amdgpu_ip_version((*dev).adev, SDMA0_HWIP, 0) >= IP_VERSION(5, 2, 0) }

unsafe fn pm_map_queues_v9(pm: *mut packet_manager, buffer: *mut u32, q: *mut queue, is_static: bool) -> i32 {
    let packet = buffer as *mut pm4_mes_map_queues; core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_map_queues>()); (*packet).header.u32All = pm_build_pm4_header(IT_MAP_QUEUES, core::mem::size_of::<pm4_mes_map_queues>());
    (*packet).bitfields2.num_queues = 1; (*packet).bitfields2.queue_sel = queue_sel__mes_map_queues__map_to_hws_determined_queue_slots_vi; (*packet).bitfields2.engine_sel = engine_sel__mes_map_queues__compute_vi; (*packet).bitfields2.gws_control_queue = if (*q).properties.is_gws { 1 } else { 0 }; (*packet).bitfields2.extended_engine_sel = extended_engine_sel__mes_map_queues__legacy_engine_sel; (*packet).bitfields2.queue_type = queue_type__mes_map_queues__normal_compute_vi;
    match (*q).properties.type_ { KFD_QUEUE_TYPE_COMPUTE => { if is_static { (*packet).bitfields2.queue_type = queue_type__mes_map_queues__normal_latency_static_queue_vi; } }, KFD_QUEUE_TYPE_SDMA | KFD_QUEUE_TYPE_SDMA_XGMI => { if (*q).properties.sdma_engine_id < 2 && !pm_use_ext_eng((*q).device.kfd) { (*packet).bitfields2.engine_sel = (*q).properties.sdma_engine_id + engine_sel__mes_map_queues__sdma0_vi; } else { (*packet).bitfields2.extended_engine_sel = if (*q).properties.sdma_engine_id >= 8 { extended_engine_sel__mes_map_queues__sdma8_to_15_sel } else { extended_engine_sel__mes_map_queues__sdma0_to_7_sel }; (*packet).bitfields2.engine_sel = (*q).properties.sdma_engine_id % 8; } }, _ => { WARN(1, "queue type %d", (*q).properties.type_); return -EINVAL; } }
    (*packet).bitfields3.doorbell_offset = (*q).properties.doorbell_off; (*packet).mqd_addr_lo = lower_32_bits((*q).gart_mqd_addr); (*packet).mqd_addr_hi = upper_32_bits((*q).gart_mqd_addr); (*packet).wptr_addr_lo = lower_32_bits((*q).properties.write_ptr as u64); (*packet).wptr_addr_hi = upper_32_bits((*q).properties.write_ptr as u64); 0
}

unsafe fn pm_build_dequeue_wait_counts_packet_info(pm: *mut packet_manager, sch_value: u32, que_sleep: u32, reg_offset: *mut u32, reg_data: *mut u32) { (*(*(*pm).dqm).dev).kfd2kgd.build_dequeue_wait_counts_packet_info((*(*pm).dqm).dev.adev, (*(*pm).dqm).wait_times, sch_value, que_sleep, reg_offset, reg_data); }

unsafe fn pm_grace_period_0_supported(pm: *mut packet_manager) -> bool {
    let dev = (*pm).dqm.dev; let mec_fw_version = dev.kfd.mec_fw_version; let v = KFD_GC_VERSION(dev);
    if v >= IP_VERSION(11, 0, 0) && v < IP_VERSION(12, 0, 0) { return true; }
    match v { IP_VERSION(9,0,1) => mec_fw_version >= 461 + 32768, IP_VERSION(9,1,0)|IP_VERSION(9,2,1)|IP_VERSION(9,2,2)|IP_VERSION(9,3,0)|IP_VERSION(9,4,0) => mec_fw_version >= 461, IP_VERSION(9,4,1) => false, IP_VERSION(9,4,2) => mec_fw_version >= 63, IP_VERSION(9,4,3)|IP_VERSION(9,4,4) => mec_fw_version >= 96, IP_VERSION(9,5,0) => true, IP_VERSION(10,1,10)|IP_VERSION(10,1,2)|IP_VERSION(10,1,1) => mec_fw_version >= 146, IP_VERSION(10,3,0)|IP_VERSION(10,3,2)|IP_VERSION(10,3,1)|IP_VERSION(10,3,4)|IP_VERSION(10,3,5) => mec_fw_version >= 93, _ => false }
}

unsafe fn pm_config_dequeue_wait_counts_v9(pm: *mut packet_manager, buffer: *mut u32, cmd: kfd_config_dequeue_wait_counts_cmd, mut value: u32) -> i32 {
    let mut reg_offset = 0; let mut reg_data = 0;
    match cmd { KFD_DEQUEUE_WAIT_INIT => { let mut sch_wave = 0; let que_sleep = 1; if KFD_GC_VERSION((*pm).dqm.dev) < IP_VERSION(9,4,1) || KFD_GC_VERSION((*pm).dqm.dev) >= IP_VERSION(10,0,0) { return -EPERM; } if amdgpu_emu_mode == 0 && (*pm).dqm.dev.adev.gmc.is_app_apu && KFD_GC_VERSION((*pm).dqm.dev) == IP_VERSION(9,4,3) { sch_wave = 1; } pm_build_dequeue_wait_counts_packet_info(pm, sch_wave, que_sleep, &mut reg_offset, &mut reg_data); }, KFD_DEQUEUE_WAIT_RESET => pm_build_dequeue_wait_counts_packet_info(pm, 0, 0, &mut reg_offset, &mut reg_data), KFD_DEQUEUE_WAIT_SET_SCH_WAVE => { if value == 0 && !pm_grace_period_0_supported(pm) { value = 1; } pm_build_dequeue_wait_counts_packet_info(pm, value, 0, &mut reg_offset, &mut reg_data); }, _ => { pr_err("Invalid dequeue wait cmd\n"); return -EINVAL; } }
    let packet = buffer as *mut pm4_mec_write_data_mmio; core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mec_write_data_mmio>()); (*packet).header.u32All = pm_build_pm4_header(IT_WRITE_DATA, core::mem::size_of::<pm4_mec_write_data_mmio>()); (*packet).bitfields2.dst_sel = dst_sel___write_data__mem_mapped_register; (*packet).bitfields2.addr_incr = addr_incr___write_data__do_not_increment_address; (*packet).bitfields3.dst_mmreg_addr = reg_offset; (*packet).data = reg_data; 0
}

unsafe fn pm_unmap_queues_v9(pm: *mut packet_manager, buffer: *mut u32, filter: kfd_unmap_queues_filter, filter_param: u32, reset: bool) -> i32 {
    let packet = buffer as *mut pm4_mes_unmap_queues; core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_unmap_queues>()); (*packet).header.u32All = pm_build_pm4_header(IT_UNMAP_QUEUES, core::mem::size_of::<pm4_mes_unmap_queues>()); (*packet).bitfields2.extended_engine_sel = if pm_use_ext_eng((*pm).dqm.dev.kfd) { extended_engine_sel__mes_unmap_queues__sdma0_to_7_sel } else { extended_engine_sel__mes_unmap_queues__legacy_engine_sel }; (*packet).bitfields2.engine_sel = engine_sel__mes_unmap_queues__compute; (*packet).bitfields2.action = if reset { action__mes_unmap_queues__reset_queues } else { action__mes_unmap_queues__preempt_queues };
    match filter { KFD_UNMAP_QUEUES_FILTER_BY_PASID => { (*packet).bitfields2.queue_sel = queue_sel__mes_unmap_queues__perform_request_on_pasid_queues; (*packet).bitfields3a.pasid = filter_param; }, KFD_UNMAP_QUEUES_FILTER_ALL_QUEUES => (*packet).bitfields2.queue_sel = queue_sel__mes_unmap_queues__unmap_all_queues, KFD_UNMAP_QUEUES_FILTER_DYNAMIC_QUEUES => (*packet).bitfields2.queue_sel = queue_sel__mes_unmap_queues__unmap_all_non_static_queues, _ => { WARN(1, "filter %d", filter); return -EINVAL; } } 0
}

unsafe fn pm_query_status_v9(_pm: *mut packet_manager, buffer: *mut u32, fence_address: u64, fence_value: u64) -> i32 {
    let packet = buffer as *mut pm4_mes_query_status; core::ptr::write_bytes(buffer, 0, core::mem::size_of::<pm4_mes_query_status>()); (*packet).header.u32All = pm_build_pm4_header(IT_QUERY_STATUS, core::mem::size_of::<pm4_mes_query_status>()); (*packet).bitfields2.context_id = 0; (*packet).bitfields2.interrupt_sel = interrupt_sel__mes_query_status__completion_status; (*packet).bitfields2.command = command__mes_query_status__fence_only_after_write_ack; (*packet).addr_hi = upper_32_bits(fence_address); (*packet).addr_lo = lower_32_bits(fence_address); (*packet).data_hi = upper_32_bits(fence_value); (*packet).data_lo = lower_32_bits(fence_value); 0
}

const kfd_v9_pm_funcs: packet_manager_funcs = packet_manager_funcs { map_process: Some(pm_map_process_v9), runlist: Some(pm_runlist_v9), set_resources: Some(pm_set_resources_v9), map_queues: Some(pm_map_queues_v9), unmap_queues: Some(pm_unmap_queues_v9), config_dequeue_wait_counts: Some(pm_config_dequeue_wait_counts_v9), query_status: Some(pm_query_status_v9), release_mem: None, map_process_size: core::mem::size_of::<pm4_mes_map_process>(), runlist_size: core::mem::size_of::<pm4_mes_runlist>(), set_resources_size: core::mem::size_of::<pm4_mes_set_resources>(), map_queues_size: core::mem::size_of::<pm4_mes_map_queues>(), unmap_queues_size: core::mem::size_of::<pm4_mes_unmap_queues>(), config_dequeue_wait_counts_size: core::mem::size_of::<pm4_mec_write_data_mmio>(), query_status_size: core::mem::size_of::<pm4_mes_query_status>(), release_mem_size: 0 };
const kfd_aldebaran_pm_funcs: packet_manager_funcs = packet_manager_funcs { map_process: Some(pm_map_process_aldebaran), runlist: Some(pm_runlist_v9), set_resources: Some(pm_set_resources_v9), map_queues: Some(pm_map_queues_v9), unmap_queues: Some(pm_unmap_queues_v9), config_dequeue_wait_counts: Some(pm_config_dequeue_wait_counts_v9), query_status: Some(pm_query_status_v9), release_mem: None, map_process_size: core::mem::size_of::<pm4_mes_map_process_aldebaran>(), runlist_size: core::mem::size_of::<pm4_mes_runlist>(), set_resources_size: core::mem::size_of::<pm4_mes_set_resources>(), map_queues_size: core::mem::size_of::<pm4_mes_map_queues>(), unmap_queues_size: core::mem::size_of::<pm4_mes_unmap_queues>(), config_dequeue_wait_counts_size: core::mem::size_of::<pm4_mec_write_data_mmio>(), query_status_size: core::mem::size_of::<pm4_mes_query_status>(), release_mem_size: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
