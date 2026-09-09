// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Platform Security Processor (PSP) interface
 *
 * Copyright (C) 2016,2019 Advanced Micro Devices, Inc.
 *
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 */

// External kernel and driver dependencies supplied by the surrounding tree.

pub static mut psp_master: *mut psp_device = core::ptr::null_mut();

const PSP_C2PMSG_17_CMDRESP_CMD: u32 = 0x000f0000;

unsafe fn psp_mailbox_poll(
    cmdresp_reg: *const core::ffi::c_void,
    cmdresp: *mut u32,
    mut timeout_msecs: u32,
) -> i32 {
    loop {
        *cmdresp = ioread32(cmdresp_reg);
        if ((*cmdresp & PSP_CMDRESP_RESP) >> PSP_CMDRESP_RESP_SHIFT) != 0 {
            return 0;
        }

        if timeout_msecs == 0 {
            break;
        }
        timeout_msecs = timeout_msecs.wrapping_sub(1);
        usleep_range(1000, 1100);
    }

    -ETIMEDOUT
}

pub unsafe fn psp_mailbox_command(
    psp: *mut psp_device,
    cmd: psp_cmd,
    cmdbuff: *mut core::ffi::c_void,
    timeout_msecs: u32,
    cmdresp: *mut u32,
) -> i32 {
    if psp.is_null()
        || (*psp).vdata.is_null()
        || (*(*psp).vdata).cmdresp_reg == 0
        || (*(*psp).vdata).cmdbuff_addr_lo_reg == 0
        || (*(*psp).vdata).cmdbuff_addr_hi_reg == 0
    {
        return -ENODEV;
    }

    let cmdresp_reg = (*psp).io_regs.add((*(*psp).vdata).cmdresp_reg as usize);
    let cmdbuff_lo_reg = (*psp).io_regs.add((*(*psp).vdata).cmdbuff_addr_lo_reg as usize);
    let cmdbuff_hi_reg = (*psp).io_regs.add((*(*psp).vdata).cmdbuff_addr_hi_reg as usize);

    mutex_lock(&mut (*psp).mailbox_mutex);

    let mut ret = -EBUSY;
    if psp_mailbox_poll(cmdresp_reg as *const _, cmdresp, 0) != 0 {
        mutex_unlock(&mut (*psp).mailbox_mutex);
        return ret;
    }

    if !cmdbuff.is_null() {
        let addr = __psp_pa(cmdbuff);
        iowrite32(addr as u32, cmdbuff_lo_reg);
        iowrite32((addr >> 32) as u32, cmdbuff_hi_reg);
    }

    *cmdresp = ((cmd as u32) << 16) & PSP_C2PMSG_17_CMDRESP_CMD;
    iowrite32(*cmdresp, cmdresp_reg);
    ret = psp_mailbox_poll(cmdresp_reg as *const _, cmdresp, timeout_msecs);

    mutex_unlock(&mut (*psp).mailbox_mutex);
    ret
}

pub unsafe fn psp_extended_mailbox_cmd(
    psp: *mut psp_device,
    timeout_msecs: u32,
    req: *mut psp_ext_request,
) -> i32 {
    let mut reg = 0u32;
    print_hex_dump_debug(b"->psp \0".as_ptr(), DUMP_PREFIX_OFFSET, 16, 2, req as *const _, (*req).header.payload_size, false);

    let ret = psp_mailbox_command(psp, PSP_CMD_TEE_EXTENDED_CMD, req as *mut _, timeout_msecs, &mut reg);
    if ret != 0 {
        return ret;
    } else if ((reg & PSP_CMDRESP_STS) >> PSP_CMDRESP_STS_SHIFT) != 0 {
        (*req).header.status = (reg & PSP_CMDRESP_STS) >> PSP_CMDRESP_STS_SHIFT;
        return -EIO;
    }

    print_hex_dump_debug(b"<-psp \0".as_ptr(), DUMP_PREFIX_OFFSET, 16, 2, req as *const _, (*req).header.payload_size, false);
    0
}

unsafe fn psp_alloc_struct(sp: *mut sp_device) -> *mut psp_device {
    let dev = (*sp).dev;
    let psp = devm_kzalloc(dev, core::mem::size_of::<psp_device>(), GFP_KERNEL) as *mut psp_device;
    if psp.is_null() { return core::ptr::null_mut(); }
    (*psp).dev = dev;
    (*psp).sp = sp;
    snprintf((*psp).name.as_mut_ptr(), (*psp).name.len(), b"psp-%u\0".as_ptr(), (*sp).ord);
    psp
}

unsafe fn psp_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let psp = data as *mut psp_device;
    let status = ioread32((*psp).io_regs.add((*(*psp).vdata).intsts_reg as usize));
    iowrite32(status, (*psp).io_regs.add((*(*psp).vdata).intsts_reg as usize));
    if status != 0 {
        if let Some(handler) = (*psp).sev_irq_handler {
            handler(irq, (*psp).sev_irq_data, status);
        }
    }
    IRQ_HANDLED
}

unsafe fn psp_get_capability(psp: *mut psp_device) -> i32 {
    let val = ioread32((*psp).io_regs.add((*(*psp).vdata).feature_reg as usize));
    if val == 0xffff_ffff {
        dev_notice((*psp).dev, b"psp: unable to access the device: you might be running a broken BIOS.\n\0".as_ptr());
        return -ENODEV;
    }
    (*psp).capability.raw = val;
    0
}

unsafe fn psp_check_sev_support(psp: *mut psp_device) -> i32 { if !(*psp).capability.sev { dev_dbg((*psp).dev, b"psp does not support SEV\n\0".as_ptr()); -ENODEV } else { 0 } }
unsafe fn psp_check_tee_support(psp: *mut psp_device) -> i32 { if !(*psp).capability.tee { dev_dbg((*psp).dev, b"psp does not support TEE\n\0".as_ptr()); -ENODEV } else { 0 } }
unsafe fn psp_check_sfs_support(psp: *mut psp_device) -> i32 { if !(*psp).capability.sfs { dev_dbg((*psp).dev, b"psp does not support SFS\n\0".as_ptr()); -ENODEV } else { 0 } }

unsafe fn psp_init(psp: *mut psp_device) -> i32 {
    let mut ret;
    if psp_check_sev_support(psp) == 0 { ret = sev_dev_init(psp); if ret != 0 { return ret; } }
    if psp_check_tee_support(psp) == 0 { ret = tee_dev_init(psp); if ret != 0 { return ret; } }
    if psp_check_sfs_support(psp) == 0 { ret = sfs_dev_init(psp); if ret != 0 { return ret; } }
    if (*psp).vdata.platform_access { ret = platform_access_dev_init(psp); if ret != 0 { return ret; } }
    if PSP_FEATURE(psp, DBC) || (*psp).capability.dbc_thru_ext { ret = dbc_dev_init(psp); if ret != 0 { return ret; } }
    ret = psp_init_hsti(psp); if ret != 0 { return ret; }
    0
}

pub unsafe fn psp_dev_init(sp: *mut sp_device) -> i32 {
    let dev = (*sp).dev;
    let psp = psp_alloc_struct(sp);
    if psp.is_null() { return -ENOMEM; }
    (*sp).psp_data = psp;
    (*psp).vdata = (*sp).dev_vdata.psp_vdata as *mut psp_vdata;
    if (*psp).vdata.is_null() { dev_err(dev, b"missing driver data\n\0".as_ptr()); (*sp).psp_data = core::ptr::null_mut(); return -ENODEV; }
    (*psp).io_regs = (*sp).io_map;
    mutex_init(&mut (*psp).mailbox_mutex);
    let ret = psp_get_capability(psp); if ret != 0 { (*sp).psp_data = core::ptr::null_mut(); return ret; }
    iowrite32(0, (*psp).io_regs.add((*(*psp).vdata).inten_reg as usize));
    iowrite32(u32::MAX, (*psp).io_regs.add((*(*psp).vdata).intsts_reg as usize));
    let ret = sp_request_psp_irq((*psp).sp, Some(psp_irq_handler), (*psp).name.as_ptr(), psp as *mut _);
    if ret != 0 { (*sp).psp_data = core::ptr::null_mut(); return ret; }
    if let Some(setter) = (*sp).set_psp_master_device { setter(sp); }
    let ret = psp_init(psp);
    if ret != 0 { if let Some(clearer) = (*sp).clear_psp_master_device { clearer(sp); } sp_free_psp_irq((*psp).sp, psp); (*sp).psp_data = core::ptr::null_mut(); return ret; }
    iowrite32(u32::MAX, (*psp).io_regs.add((*(*psp).vdata).inten_reg as usize));
    dev_notice(dev, b"psp enabled\n\0".as_ptr());
    0
}

pub unsafe fn psp_dev_destroy(sp: *mut sp_device) { let psp = (*sp).psp_data; if psp.is_null() { return; } dbc_dev_destroy(psp); platform_access_dev_destroy(psp); sfs_dev_destroy(psp); tee_dev_destroy(psp); sev_dev_destroy(psp); sp_free_psp_irq(sp, psp); if let Some(f) = (*sp).clear_psp_master_device { f(sp); } }
pub unsafe fn psp_set_sev_irq_handler(psp: *mut psp_device, handler: psp_irq_handler_t, data: *mut core::ffi::c_void) { (*psp).sev_irq_data = data; (*psp).sev_irq_handler = handler; }
pub unsafe fn psp_clear_sev_irq_handler(psp: *mut psp_device) { psp_set_sev_irq_handler(psp, None, core::ptr::null_mut()); }
pub unsafe fn psp_get_master_device() -> *mut psp_device { let sp = sp_get_psp_master_device(); if sp.is_null() { core::ptr::null_mut() } else { (*sp).psp_data } }
pub unsafe fn psp_restore(sp: *mut sp_device) -> i32 { let psp = (*sp).psp_data; if !(*psp).tee_data.is_null() { tee_restore(psp) } else { 0 } }
pub unsafe fn psp_pci_init() { psp_master = psp_get_master_device(); if !psp_master.is_null() { sev_pci_init(); } }
pub unsafe fn psp_pci_exit() { if !psp_master.is_null() { sev_pci_exit(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
