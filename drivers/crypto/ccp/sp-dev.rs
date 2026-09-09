// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure Processor driver
 *
 * Copyright (C) 2017-2018 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 * Author: Gary R Hook <gary.hook@amd.com>
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 */

// Linux kernel dependencies supplied by other translation units.

static mut SP_UNIT_LOCK: RwLock = DEFINE_RWLOCK!();
static mut SP_UNITS: ListHead = LIST_HEAD_INIT!();

/* Ever-increasing value to produce unique unit numbers */
static mut SP_ORDINAL: Atomic = ATOMIC_INIT!(0);

static unsafe fn sp_add_device(sp: *mut sp_device) {
    let mut flags: c_ulong = 0;

    write_lock_irqsave(&mut SP_UNIT_LOCK, &mut flags);
    list_add_tail(&mut (*sp).entry, &mut SP_UNITS);
    write_unlock_irqrestore(&mut SP_UNIT_LOCK, flags);
}

static unsafe fn sp_del_device(sp: *mut sp_device) {
    let mut flags: c_ulong = 0;

    write_lock_irqsave(&mut SP_UNIT_LOCK, &mut flags);
    list_del(&mut (*sp).entry);
    write_unlock_irqrestore(&mut SP_UNIT_LOCK, flags);
}

static unsafe extern "C" fn sp_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let sp = data as *mut sp_device;

    if let Some(handler) = (*sp).ccp_irq_handler {
        handler(irq, (*sp).ccp_irq_data);
    }

    if let Some(handler) = (*sp).psp_irq_handler {
        handler(irq, (*sp).psp_irq_data);
    }

    IRQ_HANDLED
}

pub unsafe fn sp_request_ccp_irq(
    sp: *mut sp_device,
    handler: irq_handler_t,
    name: *const c_char,
    data: *mut c_void,
) -> c_int {
    let ret: c_int;

    if (*sp).psp_irq == (*sp).ccp_irq && !(*sp).dev_vdata.is_null()
        && !(*(*sp).dev_vdata).psp_vdata.is_null()
    {
        /* Need a common routine to manage all interrupts */
        (*sp).ccp_irq_data = data;
        (*sp).ccp_irq_handler = Some(handler);

        if !(*sp).irq_registered {
            ret = request_irq((*sp).ccp_irq, Some(sp_irq_handler), 0, (*sp).name.as_ptr(), sp as *mut c_void);
            if ret != 0 { return ret; }
            (*sp).irq_registered = true;
        }
    } else {
        /* Each sub-device can manage it's own interrupt */
        ret = request_irq((*sp).ccp_irq, Some(handler), 0, name, data);
        if ret != 0 { return ret; }
    }
    0
}

pub unsafe fn sp_request_psp_irq(
    sp: *mut sp_device,
    handler: irq_handler_t,
    name: *const c_char,
    data: *mut c_void,
) -> c_int {
    let ret: c_int;

    if (*sp).psp_irq == (*sp).ccp_irq && !(*sp).dev_vdata.is_null()
        && !(*(*sp).dev_vdata).ccp_vdata.is_null()
    {
        /* Need a common routine to manage all interrupts */
        (*sp).psp_irq_data = data;
        (*sp).psp_irq_handler = Some(handler);
        if !(*sp).irq_registered {
            ret = request_irq((*sp).psp_irq, Some(sp_irq_handler), 0, (*sp).name.as_ptr(), sp as *mut c_void);
            if ret != 0 { return ret; }
            (*sp).irq_registered = true;
        }
    } else {
        /* Each sub-device can manage it's own interrupt */
        ret = request_irq((*sp).psp_irq, Some(handler), 0, name, data);
        if ret != 0 { return ret; }
    }
    0
}

pub unsafe fn sp_free_ccp_irq(sp: *mut sp_device, data: *mut c_void) {
    if (*sp).psp_irq == (*sp).ccp_irq && !(*sp).dev_vdata.is_null()
        && !(*(*sp).dev_vdata).psp_vdata.is_null()
    {
        /* Using common routine to manage all interrupts */
        if (*sp).psp_irq_handler.is_none() {
            /* Nothing else using it, so free it */
            free_irq((*sp).ccp_irq, sp as *mut c_void);
            (*sp).irq_registered = false;
        }
        (*sp).ccp_irq_handler = None;
        (*sp).ccp_irq_data = core::ptr::null_mut();
    } else {
        /* Each sub-device can manage it's own interrupt */
        free_irq((*sp).ccp_irq, data);
    }
}

pub unsafe fn sp_free_psp_irq(sp: *mut sp_device, data: *mut c_void) {
    if (*sp).psp_irq == (*sp).ccp_irq && !(*sp).dev_vdata.is_null()
        && !(*(*sp).dev_vdata).ccp_vdata.is_null()
    {
        /* Using common routine to manage all interrupts */
        if (*sp).ccp_irq_handler.is_none() {
            /* Nothing else using it, so free it */
            free_irq((*sp).psp_irq, sp as *mut c_void);
            (*sp).irq_registered = false;
        }
        (*sp).psp_irq_handler = None;
        (*sp).psp_irq_data = core::ptr::null_mut();
    } else {
        /* Each sub-device can manage it's own interrupt */
        free_irq((*sp).psp_irq, data);
    }
}

/**
 * sp_alloc_struct - allocate and initialize the sp_device struct
 *
 * @dev: device struct of the SP
 */
pub unsafe fn sp_alloc_struct(dev: *mut device) -> *mut sp_device {
    let sp = devm_kzalloc(dev, core::mem::size_of::<sp_device>(), GFP_KERNEL) as *mut sp_device;
    if sp.is_null() { return core::ptr::null_mut(); }
    (*sp).dev = dev;
    (*sp).ord = atomic_inc_return(&mut SP_ORDINAL);
    snprintf((*sp).name.as_mut_ptr(), SP_MAX_NAME_LEN, c"sp-%u".as_ptr(), (*sp).ord);
    sp
}

pub unsafe fn sp_init(sp: *mut sp_device) -> c_int {
    sp_add_device(sp);
    if !(*(*sp).dev_vdata).ccp_vdata.is_null() { ccp_dev_init(sp); }
    if !(*(*sp).dev_vdata).psp_vdata.is_null() { psp_dev_init(sp); }
    0
}

pub unsafe fn sp_destroy(sp: *mut sp_device) {
    if !(*(*sp).dev_vdata).ccp_vdata.is_null() { ccp_dev_destroy(sp); }
    if !(*(*sp).dev_vdata).psp_vdata.is_null() { psp_dev_destroy(sp); }
    sp_del_device(sp);
}

pub unsafe fn sp_suspend(sp: *mut sp_device) -> c_int {
    if !(*(*sp).dev_vdata).ccp_vdata.is_null() { ccp_dev_suspend(sp); }
    0
}

pub unsafe fn sp_resume(sp: *mut sp_device) -> c_int {
    if !(*(*sp).dev_vdata).ccp_vdata.is_null() { ccp_dev_resume(sp); }
    0
}

pub unsafe fn sp_restore(sp: *mut sp_device) -> c_int {
    if !(*sp).psp_data.is_null() {
        let ret = psp_restore(sp);
        if ret != 0 { return ret; }
    }
    sp_resume(sp)
}

pub unsafe fn sp_get_psp_master_device() -> *mut sp_device {
    let mut i: *mut sp_device = core::ptr::null_mut();
    let mut ret: *mut sp_device = core::ptr::null_mut();
    let mut flags: c_ulong = 0;
    write_lock_irqsave(&mut SP_UNIT_LOCK, &mut flags);
    if !list_empty(&SP_UNITS) {
        list_for_each_entry!(i, &SP_UNITS, entry, {
            if !(*i).psp_data.is_null() && (*i).get_psp_master_device.is_some() {
                ret = ((*i).get_psp_master_device.unwrap())();
                break;
            }
        });
    }
    write_unlock_irqrestore(&mut SP_UNIT_LOCK, flags);
    ret
}

#[cfg(CONFIG_X86)]
static mut INITIALIZED: bool = false;

unsafe fn sp_mod_init() -> c_int {
    #[cfg(CONFIG_X86)] {
        if INITIALIZED { return 0; }
        let ret = sp_pci_init();
        if ret != 0 { return ret; }
        #[cfg(CONFIG_CRYPTO_DEV_SP_PSP)] psp_pci_init();
        INITIALIZED = true;
        return 0;
    }
    #[cfg(CONFIG_ARM64)] {
        let ret = sp_platform_init();
        if ret != 0 { return ret; }
        return 0;
    }
    -ENODEV
}

#[cfg(all(CONFIG_KVM_AMD, CONFIG_KVM_AMD_SEV))]
pub unsafe fn sev_module_init() -> c_int { sp_mod_init() }

unsafe fn sp_mod_exit() {
    #[cfg(CONFIG_X86)] {
        #[cfg(CONFIG_CRYPTO_DEV_SP_PSP)] psp_pci_exit();
        sp_pci_exit();
    }
    #[cfg(CONFIG_ARM64)] sp_platform_exit();
}

// module_init(sp_mod_init);
// module_exit(sp_mod_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
