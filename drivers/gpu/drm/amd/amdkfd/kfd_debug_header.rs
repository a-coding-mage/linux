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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency declarations and preprocessor definitions are supplied by kfd_priv.h.

use core::ffi::c_void;

extern "C" {
    pub fn kfd_dbg_trap_deactivate(target: *mut kfd_process, unwind: bool, unwind_count: i32);
    pub fn kfd_dbg_trap_activate(target: *mut kfd_process) -> i32;
    pub fn kfd_dbg_ev_query_debug_event(
        process: *mut kfd_process,
        queue_id: *mut u32,
        gpu_id: *mut u32,
        exception_clear_mask: u64,
        event_status: *mut u64,
    ) -> i32;
    pub fn kfd_set_dbg_ev_from_interrupt(
        dev: *mut kfd_node,
        pasid: u32,
        doorbell_id: u32,
        trap_mask: u64,
        exception_data: *mut c_void,
        exception_data_size: usize,
    ) -> bool;
    pub fn kfd_dbg_ev_raise(
        event_mask: u64,
        process: *mut kfd_process,
        dev: *mut kfd_node,
        source_id: u32,
        use_worker: bool,
        exception_data: *mut c_void,
        exception_data_size: usize,
    ) -> bool;
    pub fn kfd_dbg_trap_disable(target: *mut kfd_process) -> i32;
    pub fn kfd_dbg_trap_enable(
        target: *mut kfd_process,
        fd: u32,
        runtime_info: *mut c_void,
        runtime_info_size: *mut u32,
    ) -> i32;
    pub fn kfd_dbg_trap_set_wave_launch_override(
        target: *mut kfd_process,
        trap_override: u32,
        trap_mask_bits: u32,
        trap_mask_request: u32,
        trap_mask_prev: *mut u32,
        trap_mask_supported: *mut u32,
    ) -> i32;
    pub fn kfd_dbg_trap_set_wave_launch_mode(target: *mut kfd_process, wave_launch_mode: u8) -> i32;
    pub fn kfd_dbg_trap_clear_dev_address_watch(
        pdd: *mut kfd_process_device,
        watch_id: u32,
    ) -> i32;
    pub fn kfd_dbg_trap_set_dev_address_watch(
        pdd: *mut kfd_process_device,
        watch_address: u64,
        watch_address_mask: u32,
        watch_id: *mut u32,
        watch_mode: u32,
    ) -> i32;
    pub fn kfd_dbg_trap_set_flags(target: *mut kfd_process, flags: *mut u32) -> i32;
    pub fn kfd_dbg_trap_query_exception_info(
        target: *mut kfd_process,
        source_id: u32,
        exception_code: u32,
        clear_exception: bool,
        info: *mut c_void,
        info_size: *mut u32,
    ) -> i32;
    pub fn kfd_dbg_send_exception_to_runtime(
        p: *mut kfd_process,
        dev_id: u32,
        queue_id: u32,
        error_reason: u64,
    ) -> i32;

    pub fn debug_event_write_work_handler(work: *mut work_struct);
    pub fn kfd_dbg_trap_device_snapshot(
        target: *mut kfd_process,
        exception_clear_mask: u64,
        user_info: *mut c_void,
        number_of_device_infos: *mut u32,
        entry_size: *mut u32,
    ) -> i32;
    pub fn kfd_dbg_set_enabled_debug_exception_mask(target: *mut kfd_process, exception_set_mask: u64);
    pub fn kfd_dbg_set_mes_debug_mode(pdd: *mut kfd_process_device, sq_trap_en: bool) -> i32;
}

pub unsafe fn kfd_dbg_is_per_vmid_supported(dev: *mut kfd_node) -> bool {
    KFD_GC_VERSION(dev) == IP_VERSION(9, 4, 2)
        || KFD_GC_VERSION(dev) == IP_VERSION(9, 4, 3)
        || KFD_GC_VERSION(dev) == IP_VERSION(9, 4, 4)
        || KFD_GC_VERSION(dev) == IP_VERSION(9, 5, 0)
        || KFD_GC_VERSION(dev) >= IP_VERSION(11, 0, 0)
}

/*
 * If GFX off is enabled, chips that do not support RLC restore for the debug
 * registers will disable GFX off temporarily for the entire debug session.
 * See disable_on_trap_action_entry and enable_on_trap_action_exit for details.
 */
pub unsafe fn kfd_dbg_is_rlc_restore_supported(dev: *mut kfd_node) -> bool {
    !(KFD_GC_VERSION(dev) == IP_VERSION(10, 1, 10)
        || KFD_GC_VERSION(dev) == IP_VERSION(10, 1, 1))
}

pub unsafe fn kfd_dbg_has_cwsr_workaround(dev: *mut kfd_node) -> bool {
    KFD_GC_VERSION(dev) >= IP_VERSION(11, 0, 0)
        && KFD_GC_VERSION(dev) <= IP_VERSION(11, 0, 3)
}

pub unsafe fn kfd_dbg_has_gws_support(dev: *mut kfd_node) -> bool {
    if (KFD_GC_VERSION(dev) == IP_VERSION(9, 0, 1) && (*dev).kfd.mec2_fw_version < 0x81b6)
        || (KFD_GC_VERSION(dev) >= IP_VERSION(9, 1, 0)
            && KFD_GC_VERSION(dev) <= IP_VERSION(9, 2, 2)
            && (*dev).kfd.mec2_fw_version < 0x1b6)
        || (KFD_GC_VERSION(dev) == IP_VERSION(9, 4, 0) && (*dev).kfd.mec2_fw_version < 0x1b6)
        || (KFD_GC_VERSION(dev) == IP_VERSION(9, 4, 1) && (*dev).kfd.mec2_fw_version < 0x30)
        || kfd_dbg_has_cwsr_workaround(dev)
    {
        return false;
    }

    /* Assume debugging and cooperative launch supported otherwise. */
    true
}

pub unsafe fn kfd_dbg_has_ttmps_always_setup(dev: *mut kfd_node) -> bool {
    (KFD_GC_VERSION(dev) < IP_VERSION(11, 0, 0) && KFD_GC_VERSION(dev) != IP_VERSION(9, 4, 2))
        || (KFD_GC_VERSION(dev) >= IP_VERSION(11, 0, 0)
            && KFD_GC_VERSION(dev) < IP_VERSION(12, 0, 0)
            && ((*dev).adev.mes.sched_version & AMDGPU_MES_VERSION_MASK) >= 70)
        || KFD_GC_VERSION(dev) >= IP_VERSION(12, 0, 0)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
