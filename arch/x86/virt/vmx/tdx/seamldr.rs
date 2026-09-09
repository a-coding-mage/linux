// SPDX-License-Identifier: GPL-2.0
/*
 * P-SEAMLDR support for TDX module management features like runtime updates
 *
 * Copyright (C) 2025 Intel Corporation
 */

const P_SEAMLDR_INFO: u64 = 0x8000_0000_0000_0000;
const P_SEAMLDR_INSTALL: u64 = 0x8000_0000_0000_0001;

const SEAMLDR_MAX_NR_MODULE_PAGES: usize = 496;
const SEAMLDR_MAX_NR_SIG_PAGES: usize = 1;

/*
 * The seamldr_params "scenario" field specifies the operation mode:
 * 0: Install TDX module from scratch (not used by kernel)
 * 1: Update existing TDX module to a compatible version
 */
const SEAMLDR_SCENARIO_UPDATE: u32 = 1;

/*
 * This is the "SEAMLDR_PARAMS" data structure defined in the
 * "SEAM Loader (SEAMLDR) Interface Specification".
 *
 * It is the in-memory ABI that the kernel passes to the P-SEAMLDR
 * to update the TDX module. It breaks the TDX module image up in
 * page-size pieces.
 */
#[repr(C, packed)]
struct seamldr_params {
    version: u32,
    scenario: u32,
    sigstruct_pages_pa_list: [u64; SEAMLDR_MAX_NR_SIG_PAGES],
    reserved: [u8; 104],
    module_nr_pages: u64,
    module_pages_pa_list: [u64; SEAMLDR_MAX_NR_MODULE_PAGES],
}

/* Serialize P-SEAMLDR calls since the hardware only allows a single CPU to
 * interact with P-SEAMLDR simultaneously. */
static mut seamldr_lock: raw_spinlock_t = raw_spinlock_t::new();

unsafe fn seamldr_call(fn_: u64, args: *mut tdx_module_args) -> i32 {
    /*
     * With this bug, P-SEAMLDR calls corrupt the VMCS
     * pointer and must be avoided. This path should be
     * unreachable since sysfs hides the ABIs.
     */
    if boot_cpu_has_bug(X86_BUG_SEAMRET_INVD_VMCS) {
        WARN_ON(1);
        return -EINVAL;
    }

    let _guard = raw_spinlock_guard(&mut seamldr_lock);
    seamcall_prerr(fn_, args)
}

unsafe fn seamldr_get_info(seamldr_info: *mut seamldr_info) -> i32 {
    let mut args: tdx_module_args = core::mem::zeroed();

    /* Use slow_virt_to_phys() since @seamldr_info may be allocated on the stack. */
    args.rcx = slow_virt_to_phys(seamldr_info as *const core::ffi::c_void);
    seamldr_call(P_SEAMLDR_INFO, &mut args)
}

/* Call into P-SEAMLDR to install a TDX module update */
unsafe fn seamldr_install(params: *const seamldr_params) -> i32 {
    let mut args: tdx_module_args = core::mem::zeroed();
    args.rcx = __pa(params as *const core::ffi::c_void);
    seamldr_call(P_SEAMLDR_INSTALL, &mut args)
}

const TDX_IMAGE_VERSION_2: u16 = 0x200;

/* First page of the on-disk module update image: */
#[repr(C, packed)]
struct tdx_image_header {
    version: u16,
    checksum: u16,
    signature: [u8; 8],
    sigstruct_nr_pages: u32,
    module_nr_pages: u32,
    reserved: [u8; 4076],
}

const TDX_IMAGE_HEADER_SIZE: usize = core::mem::size_of::<tdx_image_header>();

/*
 * Intel TDX module update ABI structure. aka. "TDX module blob".
 * This is the on-disk format that fw_upload lands in a kernel buffer.
 *
 * @payload contains sigstruct pages followed by module pages.
 */
#[repr(C, packed)]
struct tdx_image {
    header: tdx_image_header,
    payload: [u8; 0],
}

/* Given a vmalloc() allocation, write all backing physical addresses to pa_list[]. */
unsafe fn populate_pa_list(pa_list: *mut u64, vmalloc_addr: *const u8, vmalloc_len_pages: u32) {
    for i in 0..vmalloc_len_pages {
        let offset = (i as usize).wrapping_mul(PAGE_SIZE);
        let pfn = vmalloc_to_pfn(vmalloc_addr.add(offset));
        *pa_list.add(i as usize) = pfn << PAGE_SHIFT;
    }
}

unsafe fn populate_seamldr_params(
    params: *mut seamldr_params,
    sig: *const u8,
    sig_nr_pages: u32,
    mod_: *const u8,
    mod_nr_pages: u32,
) {
    (*params).version = 0;
    (*params).scenario = SEAMLDR_SCENARIO_UPDATE;
    (*params).module_nr_pages = mod_nr_pages as u64;

    populate_pa_list((*params).sigstruct_pages_pa_list.as_mut_ptr(), sig, sig_nr_pages);
    populate_pa_list((*params).module_pages_pa_list.as_mut_ptr(), mod_, mod_nr_pages);
}

/* @image points to a vmalloc()'d 'struct tdx_image'. Transform it into @params. */
unsafe fn init_seamldr_params(
    params: *mut seamldr_params,
    image: *const tdx_image,
    image_len: u32,
) -> i32 {
    let header = &(*image).header;
    let sigstruct_len = (header.sigstruct_nr_pages as usize).wrapping_mul(PAGE_SIZE);
    let module_len = (header.module_nr_pages as usize).wrapping_mul(PAGE_SIZE);
    let header_start = header as *const tdx_image_header as *const u8;
    let header_end = header_start.add(TDX_IMAGE_HEADER_SIZE);
    let sigstruct_start = header_end;
    let module_start = sigstruct_start.add(sigstruct_len);

    if TDX_IMAGE_HEADER_SIZE + sigstruct_len + module_len != image_len as usize {
        return -EINVAL;
    }
    if header.version != TDX_IMAGE_VERSION_2 {
        return -EINVAL;
    }
    if header.sigstruct_nr_pages as usize > SEAMLDR_MAX_NR_SIG_PAGES
        || header.module_nr_pages as usize > SEAMLDR_MAX_NR_MODULE_PAGES
    {
        return -EINVAL;
    }
    if core::slice::from_raw_parts(header.signature.as_ptr(), header.signature.len()) != b"TDX-BLOB" {
        return -EINVAL;
    }
    if header.reserved.iter().any(|&v| v != 0) {
        return -EINVAL;
    }

    populate_seamldr_params(params, sigstruct_start, header.sigstruct_nr_pages,
                            module_start, header.module_nr_pages);
    0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum module_update_state {
    MODULE_UPDATE_START,
    MODULE_UPDATE_SHUTDOWN,
    MODULE_UPDATE_CPU_INSTALL,
    MODULE_UPDATE_CPU_INIT,
    MODULE_UPDATE_RUN_UPDATE,
    MODULE_UPDATE_DONE,
}

struct update_ctrl {
    state: module_update_state,
    num_ack: i32,
    num_failed: i32,
    lock: raw_spinlock_t,
}

static mut update_ctrl: update_ctrl = update_ctrl {
    state: module_update_state::MODULE_UPDATE_START,
    num_ack: 0,
    num_failed: 0,
    lock: raw_spinlock_t::new(),
};

unsafe fn __set_target_state(ctrl: *mut update_ctrl, newstate: module_update_state) {
    (*ctrl).num_ack = 0;
    (*ctrl).state = newstate;
}

unsafe fn ack_state(ctrl: *mut update_ctrl, result: i32) {
    raw_spin_lock(&mut (*ctrl).lock);
    (*ctrl).num_failed += (result != 0) as i32;
    (*ctrl).num_ack += 1;
    if (*ctrl).num_ack == num_online_cpus() && (*ctrl).num_failed == 0 {
        __set_target_state(ctrl, ((*ctrl).state as u32 + 1) as module_update_state);
    }
    raw_spin_unlock(&mut (*ctrl).lock);
}

unsafe fn init_state(ctrl: *mut update_ctrl) {
    raw_spin_lock_init(&mut (*ctrl).lock);
    __set_target_state(ctrl, module_update_state::MODULE_UPDATE_SHUTDOWN);
    (*ctrl).num_failed = 0;
}

/* See multi_cpu_stop() from where this multi-cpu state-machine was adopted. */
unsafe fn do_seamldr_install_module(seamldr_params: *mut core::ffi::c_void) -> i32 {
    let mut curstate = module_update_state::MODULE_UPDATE_START;
    let mut ret = 0;
    let is_lead_cpu = smp_processor_id() == 0;

    loop {
        let newstate = core::ptr::read_volatile(&update_ctrl.state);
        if curstate == newstate {
            cpu_relax();
            continue;
        }
        curstate = newstate;
        match curstate {
            module_update_state::MODULE_UPDATE_SHUTDOWN => {
                if is_lead_cpu { ret = tdx_module_shutdown(); }
            }
            module_update_state::MODULE_UPDATE_CPU_INSTALL => {
                ret = seamldr_install(seamldr_params as *const seamldr_params);
            }
            module_update_state::MODULE_UPDATE_CPU_INIT => ret = tdx_cpu_enable(),
            module_update_state::MODULE_UPDATE_RUN_UPDATE => {
                if is_lead_cpu { ret = tdx_module_run_update(); }
            }
            _ => {}
        }
        ack_state(&mut update_ctrl, ret);
        if curstate == module_update_state::MODULE_UPDATE_DONE
            || core::ptr::read_volatile(&update_ctrl.num_failed) != 0 { break; }
    }
    ret
}

/* Install a new TDX module. Returns 0 on success, negative error code on failure. */
unsafe fn seamldr_install_module(data: *const u8, data_len: u32) -> i32 {
    if data_len < TDX_IMAGE_HEADER_SIZE as u32 { return -EINVAL; }
    let image = data as *const tdx_image;
    let params = kzalloc_obj::<seamldr_params>();
    if params.is_null() { return -ENOMEM; }
    let mut ret = init_seamldr_params(params, image, data_len);
    if ret == 0 {
        cpus_read_lock();
        init_state(&mut update_ctrl);
        ret = stop_machine_cpuslocked(do_seamldr_install_module, params as *mut core::ffi::c_void,
                                      cpu_online_mask);
        cpus_read_unlock();
    }
    kfree(params);
    ret
}

/* stop_machine() does not interrupt preemption-disabled regions. */
unsafe fn seamldr_lock_module_update() { preempt_disable(); }
unsafe fn seamldr_unlock_module_update() { preempt_enable(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
