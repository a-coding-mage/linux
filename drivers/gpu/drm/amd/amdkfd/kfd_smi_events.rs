// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2020-2022 Advanced Micro Devices, Inc.
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

// Kernel types, constants, functions, and event-format macros are supplied by
// the surrounding kernel translation unit.

#[repr(C)]
struct KfdSmiClient {
    list: ListHead,
    fifo: Kfifo,
    wait_queue: WaitQueueHead,
    events: u64,
    dev: *mut KfdNode,
    lock: Spinlock,
    rcu: RcuHead,
    pid: PidT,
    suser: bool,
}

const KFD_MAX_KFIFO_SIZE: usize = 8192;

extern "C" {
    fn poll_wait(file: *mut File, queue: *mut WaitQueueHead, wait: *mut PollTableStruct);
    fn spin_lock(lock: *mut Spinlock);
    fn spin_unlock(lock: *mut Spinlock);
    fn kfifo_is_empty(fifo: *mut Kfifo) -> bool;
    fn kfifo_len(fifo: *mut Kfifo) -> usize;
    fn kfifo_out(fifo: *mut Kfifo, buf: *mut u8, len: usize) -> i32;
    fn kfifo_in(fifo: *mut Kfifo, buf: *const u8, len: usize) -> usize;
    fn kfifo_avail(fifo: *mut Kfifo) -> usize;
    fn kfifo_free(fifo: *mut Kfifo);
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn copy_to_user(dst: *mut u8, src: *const u8, len: usize) -> usize;
    fn copy_from_user(dst: *mut u64, src: *const u64, len: usize) -> usize;
    fn access_ok(ptr: *const core::ffi::c_void, size: usize) -> bool;
    fn wake_up_all(queue: *mut WaitQueueHead);
    fn call_rcu(head: *mut RcuHead, func: unsafe extern "C" fn(*mut RcuHead));
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn task_tgid_nr_ns(task: *mut TaskStruct, ns: *mut PidNamespace) -> PidT;
    fn task_active_pid_ns(task: *mut TaskStruct) -> *mut PidNamespace;
    fn ktime_to_ns(ts: KtimeT) -> i64;
    fn ktime_get_boottime_ns() -> i64;
    fn amdgpu_reset_get_desc(ctx: *mut AmdgpuResetContext, buf: *mut i8, len: usize);
    fn amdgpu_dpm_get_thermal_throttling_counter(adev: *mut AmdgpuDevice) -> u64;
    fn amdgpu_vm_get_task_info_pasid(adev: *mut AmdgpuDevice, pasid: u16) -> *mut AmdgpuTaskInfo;
    fn amdgpu_vm_get_task_info_vm(vm: *mut AmdgpuVm) -> *mut AmdgpuTaskInfo;
    fn amdgpu_vm_put_task_info(info: *mut AmdgpuTaskInfo);
    fn kfd_lookup_process_by_mm(mm: *mut MmStruct) -> *mut KfdProcess;
    fn kfd_unref_process(process: *mut KfdProcess);
    fn capable(cap: u32) -> bool;
}

unsafe extern "C" fn kfd_smi_ev_client_free(p: *mut RcuHead) {
    let ev = container_of_client(p);
    kfifo_free(&mut (*ev).fifo);
    kfree(ev.cast());
}

unsafe fn kfd_smi_task_to_pid(task: *mut TaskStruct) -> PidT {
    if !task.is_null() { task_tgid_nr_ns(task, task_active_pid_ns(task)) } else { 0 }
}

unsafe fn kfd_smi_ev_enabled(pid: PidT, client: *mut KfdSmiClient, event: u32) -> bool {
    let events = core::ptr::read_volatile(&(*client).events);
    if pid != 0 && (*client).pid != pid && !(*client).suser { return false; }
    (events & KFD_SMI_EVENT_MASK_FROM_INDEX(event)) != 0
}

unsafe fn add_event_to_kfifo(pid: PidT, dev: *mut KfdNode, smi_event: u32, event_msg: *mut i8, len: i32) {
    rcu_read_lock();
    list_for_each_entry_rcu!(client, &mut (*dev).smi_clients, list, {
        if !kfd_smi_ev_enabled(pid, client, smi_event) { continue; }
        spin_lock(&mut (*client).lock);
        if kfifo_avail(&mut (*client).fifo) >= len as usize {
            kfifo_in(&mut (*client).fifo, event_msg.cast(), len as usize);
            wake_up_all(&mut (*client).wait_queue);
        }
        spin_unlock(&mut (*client).lock);
    });
    rcu_read_unlock();
}

unsafe fn kfd_smi_event_add(task: *mut TaskStruct, dev: *mut KfdNode, event: u32, fmt: *const i8, args: ...) {
    let mut fifo_in = [0i8; KFD_SMI_EVENT_MSG_SIZE];
    if list_empty!(&(*dev).smi_clients) { return; }
    let pid = kfd_smi_task_to_pid(task);
    let mut len = scnprintf!(fifo_in.as_mut_ptr(), fifo_in.len(), "%x ", event);
    len += vscnprintf!(fifo_in.as_mut_ptr().add(len as usize), fifo_in.len() - len as usize, fmt, args);
    add_event_to_kfifo(pid, dev, event, fifo_in.as_mut_ptr(), len);
}

pub unsafe fn kfd_smi_event_update_gpu_reset(dev: *mut KfdNode, post_reset: bool, reset_context: *mut AmdgpuResetContext) {
    let event;
    let mut reset_cause = [0i8; 64];
    if post_reset { event = KFD_SMI_EVENT_GPU_POST_RESET; } else { event = KFD_SMI_EVENT_GPU_PRE_RESET; (*dev).reset_seq_num += 1; }
    if !reset_context.is_null() { amdgpu_reset_get_desc(reset_context, reset_cause.as_mut_ptr(), reset_cause.len()); }
    kfd_smi_event_add(core::ptr::null_mut(), dev, event, KFD_EVENT_FMT_UPDATE_GPU_RESET!((*dev).reset_seq_num, reset_cause.as_ptr()));
}

pub unsafe fn kfd_smi_event_update_thermal_throttling(dev: *mut KfdNode, throttle_bitmask: u64) {
    kfd_smi_event_add(core::ptr::null_mut(), dev, KFD_SMI_EVENT_THERMAL_THROTTLE,
        KFD_EVENT_FMT_THERMAL_THROTTLING!(throttle_bitmask, amdgpu_dpm_get_thermal_throttling_counter((*dev).adev)));
}

pub unsafe fn kfd_smi_event_update_vmfault(dev: *mut KfdNode, pasid: u16) {
    let task_info = amdgpu_vm_get_task_info_pasid((*dev).adev, pasid);
    if !task_info.is_null() { if (*task_info).task.pid != 0 { kfd_smi_event_add(core::ptr::null_mut(), dev, KFD_SMI_EVENT_VMFAULT, KFD_EVENT_FMT_VMFAULT!((*task_info).task.pid, (*task_info).task.comm)); } amdgpu_vm_put_task_info(task_info); }
}

pub unsafe fn kfd_smi_event_page_fault_start(node: *mut KfdNode, task: *mut TaskStruct, address: usize, write_fault: bool, ts: KtimeT) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_PAGE_FAULT_START, KFD_EVENT_FMT_PAGEFAULT_START!(ktime_to_ns(ts), kfd_smi_task_to_pid(task), address, (*node).id, if write_fault { 'W' } else { 'R' })); }
pub unsafe fn kfd_smi_event_page_fault_end(node: *mut KfdNode, task: *mut TaskStruct, address: usize, migration: bool) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_PAGE_FAULT_END, KFD_EVENT_FMT_PAGEFAULT_END!(ktime_get_boottime_ns(), kfd_smi_task_to_pid(task), address, (*node).id, if migration { 'M' } else { 'U' })); }
pub unsafe fn kfd_smi_event_migration_start(node: *mut KfdNode, task: *mut TaskStruct, start: usize, end: usize, from: u32, to: u32, prefetch_loc: u32, preferred_loc: u32, trigger: u32) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_MIGRATE_START, KFD_EVENT_FMT_MIGRATE_START!(ktime_get_boottime_ns(), kfd_smi_task_to_pid(task), start, end - start, from, to, prefetch_loc, preferred_loc, trigger)); }
pub unsafe fn kfd_smi_event_migration_end(node: *mut KfdNode, task: *mut TaskStruct, start: usize, end: usize, from: u32, to: u32, trigger: u32, error_code: i32) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_MIGRATE_END, KFD_EVENT_FMT_MIGRATE_END!(ktime_get_boottime_ns(), kfd_smi_task_to_pid(task), start, end - start, from, to, trigger, error_code)); }
pub unsafe fn kfd_smi_event_queue_eviction(node: *mut KfdNode, task: *mut TaskStruct, trigger: u32) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_QUEUE_EVICTION, KFD_EVENT_FMT_QUEUE_EVICTION!(ktime_get_boottime_ns(), kfd_smi_task_to_pid(task), (*node).id, trigger)); }
pub unsafe fn kfd_smi_event_queue_restore(node: *mut KfdNode, task: *mut TaskStruct) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_QUEUE_RESTORE, KFD_EVENT_FMT_QUEUE_RESTORE!(ktime_get_boottime_ns(), kfd_smi_task_to_pid(task), (*node).id, '0')); }
pub unsafe fn kfd_smi_event_queue_restore_rescheduled(mm: *mut MmStruct) {
    let p = kfd_lookup_process_by_mm(mm);
    if p.is_null() { return; }
    for i in 0..(*p).n_pdds as usize {
        let pdd = *(*p).pdds.add(i);
        kfd_smi_event_add((*p).lead_thread, (*pdd).dev, KFD_SMI_EVENT_QUEUE_RESTORE,
            KFD_EVENT_FMT_QUEUE_RESTORE!(ktime_get_boottime_ns(), kfd_smi_task_to_pid((*p).lead_thread), (*(*pdd).dev).id, 'R'));
    }
    kfd_unref_process(p);
}
pub unsafe fn kfd_smi_event_unmap_from_gpu(node: *mut KfdNode, task: *mut TaskStruct, address: usize, last: usize, trigger: u32) { kfd_smi_event_add(task, node, KFD_SMI_EVENT_UNMAP_FROM_GPU, KFD_EVENT_FMT_UNMAP_FROM_GPU!(ktime_get_boottime_ns(), kfd_smi_task_to_pid(task), address, last - address + 1, (*node).id, trigger)); }

pub unsafe fn kfd_smi_event_process(pdd: *mut KfdProcessDevice, start: bool) {
    if (*pdd).drm_priv.is_null() { return; }
    let avm = drm_priv_to_vm((*pdd).drm_priv);
    let task_info = amdgpu_vm_get_task_info_vm(avm);
    if !task_info.is_null() {
        kfd_smi_event_add(core::ptr::null_mut(), (*pdd).dev,
            if start { KFD_SMI_EVENT_PROCESS_START } else { KFD_SMI_EVENT_PROCESS_END },
            KFD_EVENT_FMT_PROCESS!((*task_info).task.pid, (*task_info).task.comm));
        amdgpu_vm_put_task_info(task_info);
    }
}

pub unsafe fn kfd_smi_event_open(dev: *mut KfdNode, fd: *mut u32) -> i32 {
    let client = kzalloc_kfd_smi_client();
    if client.is_null() { return -12; }
    init_list_head(&mut (*client).list);
    let ret = kfifo_alloc(&mut (*client).fifo, KFD_MAX_KFIFO_SIZE, 0x10);
    if ret != 0 { kfree(client.cast()); return ret; }
    init_waitqueue_head(&mut (*client).wait_queue);
    spin_lock_init(&mut (*client).lock);
    (*client).events = 0;
    (*client).dev = dev;
    (*client).pid = kfd_smi_task_to_pid(current());
    (*client).suser = capable(21);
    spin_lock(&mut (*dev).smi_lock);
    list_add_rcu(&mut (*client).list, &mut (*dev).smi_clients);
    spin_unlock(&mut (*dev).smi_lock);
    let ret = anon_inode_getfd(kfd_smi_name(), core::ptr::null_mut(), client.cast(), 2);
    if ret < 0 { spin_lock(&mut (*dev).smi_lock); list_del_rcu(&mut (*client).list); spin_unlock(&mut (*dev).smi_lock); synchronize_rcu(); kfifo_free(&mut (*client).fifo); kfree(client.cast()); return ret; }
    *fd = ret as u32;
    0
}

// The following declarations preserve the C implementation's externally
// supplied kernel structures and helpers.
#[repr(C)] struct ListHead { _private: [u8; 0] }
#[repr(C)] struct Kfifo { _private: [u8; 0] }
#[repr(C)] struct WaitQueueHead { _private: [u8; 0] }
#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct RcuHead { _private: [u8; 0] }
#[repr(C)] struct File { private_data: *mut core::ffi::c_void }
#[repr(C)] struct PollTableStruct { _private: [u8; 0] }
#[repr(C)] struct TaskStruct { _private: [u8; 0] }
#[repr(C)] struct PidNamespace { _private: [u8; 0] }
#[repr(C)] struct MmStruct { _private: [u8; 0] }
#[repr(C)] struct AmdgpuResetContext { _private: [u8; 0] }
#[repr(C)] struct AmdgpuDevice { _private: [u8; 0] }
#[repr(C)] struct AmdgpuVm { _private: [u8; 0] }
type PidT = i32;
type KtimeT = i64;

#[repr(C)] struct AmdgpuTask { pid: PidT, comm: *mut i8 }
#[repr(C)] struct AmdgpuTaskInfo { task: AmdgpuTask }
#[repr(C)] struct KfdNode { smi_clients: ListHead, reset_seq_num: u64, adev: *mut AmdgpuDevice, id: u32 }
// Remaining kernel list, allocation, inode, and synchronization operations are external dependencies.
extern "C" {
    fn kzalloc_kfd_smi_client() -> *mut KfdSmiClient;
    fn init_list_head(list: *mut ListHead); fn kfifo_alloc(fifo: *mut Kfifo, size: usize, flags: u32) -> i32;
    fn init_waitqueue_head(queue: *mut WaitQueueHead); fn spin_lock_init(lock: *mut Spinlock);
    fn current() -> *mut TaskStruct; fn list_add_rcu(item: *mut ListHead, head: *mut ListHead); fn list_del_rcu(item: *mut ListHead); fn synchronize_rcu();
    fn anon_inode_getfd(name: *const i8, fops: *mut core::ffi::c_void, data: *mut core::ffi::c_void, flags: i32) -> i32;
    fn kfd_smi_name() -> *const i8;
}
#[repr(C)] struct KfdProcessDevice { drm_priv: *mut core::ffi::c_void, dev: *mut KfdNode }
#[repr(C)] struct KfdProcess { n_pdds: i32, pdds: *mut *mut KfdProcessDevice, lead_thread: *mut TaskStruct }

extern "C" {
    fn drm_priv_to_vm(priv_data: *mut core::ffi::c_void) -> *mut AmdgpuVm;
}

extern "C" { fn container_of_client(p: *mut RcuHead) -> *mut KfdSmiClient; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
