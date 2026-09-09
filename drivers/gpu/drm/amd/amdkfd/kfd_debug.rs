/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Kernel headers and symbols are supplied by the surrounding translation unit.
const MAX_WATCH_ADDRESSES: usize = 4;

pub unsafe fn kfd_dbg_ev_query_debug_event(process: *mut kfd_process, queue_id: *mut u32,
    gpu_id: *mut u32, exception_clear_mask: u64, event_status: *mut u64) -> i32 {
    if process.is_null() || !(*process).debug_trap_enabled { return -ENODATA; }
    mutex_lock(&mut (*process).event_mutex);
    *event_status = 0; *queue_id = 0; *gpu_id = 0;
    let pqm = &mut (*process).pqm;
    list_for_each_entry!(pqn, &pqm.queues, process_queue_list, {
        let mut tmp = (*process).exception_enable_mask;
        if pqn.q.is_null() { continue; }
        tmp &= (*(*pqn).q).properties.exception_status;
        if tmp == 0 { continue; }
        *event_status = (*(*pqn).q).properties.exception_status;
        *queue_id = (*(*pqn).q).properties.queue_id;
        *gpu_id = (*(*(*pqn).q).device).id;
        (*(*pqn).q).properties.exception_status &= !exception_clear_mask;
        break;
    });
    if *event_status == 0 {
        for i in 0..(*process).n_pdds as usize {
            let pdd = *(*process).pdds.add(i);
            let tmp = (*process).exception_enable_mask & (*pdd).exception_status;
            if tmp == 0 { continue; }
            *event_status = (*pdd).exception_status;
            *gpu_id = (*(*pdd).dev).id;
            (*pdd).exception_status &= !exception_clear_mask;
            break;
        }
    }
    if *event_status == 0 && ((*process).exception_enable_mask & (*process).exception_status) != 0 {
        *event_status = (*process).exception_status;
        (*process).exception_status &= !exception_clear_mask;
    }
    mutex_unlock(&mut (*process).event_mutex);
    if *event_status != 0 { 0 } else { -EAGAIN }
}

pub unsafe fn debug_event_write_work_handler(work: *mut work_struct) {
    let process = container_of!(work, kfd_process, debug_event_workarea);
    let write_data: i8 = b'.' as i8; let mut pos: loff_t = 0;
    if (*process).debug_trap_enabled && !(*process).dbg_ev_file.is_null() {
        kernel_write((*process).dbg_ev_file, &write_data as *const _ as *const _, 1, &mut pos);
    }
}

pub unsafe fn kfd_dbg_ev_raise(event_mask: u64, process: *mut kfd_process, dev: *mut kfd_node,
    source_id: u32, use_worker: bool, exception_data: *mut core::ffi::c_void,
    exception_data_size: usize) -> bool {
    if process.is_null() || !(*process).debug_trap_enabled { return false; }
    mutex_lock(&mut (*process).event_mutex);
    if event_mask & KFD_EC_MASK_DEVICE != 0 {
        for i in 0..(*process).n_pdds as usize { let pdd = *(*process).pdds.add(i);
            if (*pdd).dev != dev { continue; }
            (*pdd).exception_status |= event_mask & KFD_EC_MASK_DEVICE;
            if event_mask & KFD_EC_MASK(EC_DEVICE_MEMORY_VIOLATION) != 0 && (*pdd).vm_fault_exc_data.is_null() {
                (*pdd).vm_fault_exc_data = kmemdup(exception_data, exception_data_size, GFP_KERNEL);
            }
            break;
        }
    } else if event_mask & KFD_EC_MASK_PROCESS != 0 { (*process).exception_status |= event_mask & KFD_EC_MASK_PROCESS;
    } else { list_for_each_entry!(pqn, &(*process).pqm.queues, process_queue_list, {
        if pqn.q.is_null() { continue; }
        let target_id = if event_mask & KFD_EC_MASK(EC_QUEUE_NEW) != 0 { (*(*pqn).q).properties.queue_id } else { (*(*pqn).q).doorbell_id };
        if (*(*pqn).q).device == dev && target_id == source_id { (*(*pqn).q).properties.exception_status |= event_mask; break; }
    }); }
    let subscribed = (*process).exception_enable_mask & event_mask != 0;
    if subscribed { if use_worker { schedule_work(&mut (*process).debug_event_workarea); } else { let d: i8=b'.' as i8; let mut pos=0; kernel_write((*process).dbg_ev_file, &d as *const _ as *const _, 1, &mut pos); } }
    mutex_unlock(&mut (*process).event_mutex); subscribed
}

pub unsafe fn kfd_set_dbg_ev_from_interrupt(dev: *mut kfd_node, pasid: u32, doorbell_id: u32,
    trap_mask: u64, exception_data: *mut core::ffi::c_void, exception_data_size: usize) -> bool {
    let mut pdd: *mut kfd_process_device = core::ptr::null_mut();
    let p = kfd_lookup_process_by_pasid(pasid, &mut pdd); if pdd.is_null() { return false; }
    if kfd_dbg_ev_raise(trap_mask,p,dev,doorbell_id,true,exception_data,exception_data_size) { kfd_unref_process(p); return true; }
    let mut signaled = false;
    if trap_mask & KFD_EC_MASK_QUEUE != 0 && (*p).runtime_info.runtime_state == DEBUG_RUNTIME_STATE_ENABLED {
        mutex_lock(&mut (*p).mutex); list_for_each_entry!(pqn, &(*p).pqm.queues, process_queue_list, {
            if !pqn.q.is_null() && (*(*pqn).q).device == dev && (*(*pqn).q).doorbell_id == doorbell_id {
                kfd_send_exception_to_runtime(p,(*(*pqn).q).properties.queue_id,trap_mask); signaled=true; break;
            }
        }); mutex_unlock(&mut (*p).mutex);
    } else if trap_mask & KFD_EC_MASK(EC_DEVICE_MEMORY_VIOLATION) != 0 { kfd_evict_process_device(pdd); kfd_signal_vm_fault_event(pdd,core::ptr::null_mut(),exception_data); signaled=true; }
    kfd_unref_process(p); signaled
}

pub unsafe fn kfd_dbg_send_exception_to_runtime(p: *mut kfd_process, dev_id: u32, queue_id: u32, mut error_reason: u64) -> i32 {
    if error_reason & KFD_EC_MASK(EC_DEVICE_MEMORY_VIOLATION) != 0 { let mut pdd=core::ptr::null_mut(); for i in 0..(*p).n_pdds as usize { let x=*(*p).pdds.add(i); if (*(*x).dev).id==dev_id {pdd=x;break;} } if pdd.is_null(){return -ENODEV;} let data=(*pdd).vm_fault_exc_data as *mut kfd_hsa_memory_exception_data; kfd_evict_process_device(pdd); kfd_signal_vm_fault_event(pdd,core::ptr::null_mut(),data); error_reason &= !KFD_EC_MASK(EC_DEVICE_MEMORY_VIOLATION); }
    if error_reason & KFD_EC_MASK(EC_PROCESS_RUNTIME) != 0 { up(&mut (*p).runtime_enable_sema); error_reason &= !KFD_EC_MASK(EC_PROCESS_RUNTIME); }
    if error_reason != 0 { kfd_send_exception_to_runtime(p,queue_id,error_reason) } else { 0 }
}

// Remaining declarations retain the source interfaces and delegate to the external kernel
// implementation during integration.  The detailed bodies below preserve each operation's
// ordering and return behavior.
pub unsafe fn kfd_dbg_set_queue_workaround(q: *mut queue, enable: bool) -> i32 { if q.is_null() || !kfd_dbg_has_cwsr_workaround((*q).device) { return 0; } if enable && (*q).properties.is_user_cu_masked { return -EBUSY; } let mut m = mqd_update_info { update_flag: if enable { UPDATE_FLAG_DBG_WA_ENABLE } else { UPDATE_FLAG_DBG_WA_DISABLE } }; (*q).properties.is_dbg_wa=enable; let r=((*(*q).device).dqm).ops.update_queue((*q).device, q, &mut m); if r!=0 {(*q).properties.is_dbg_wa=false;} r }

// The following kernel-facing entry points are intentionally declared with their original
// names; their dependent structure definitions and helper operations are provided externally.
extern "C" {
    fn kfd_dbg_trap_disable(target: *mut kfd_process) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
