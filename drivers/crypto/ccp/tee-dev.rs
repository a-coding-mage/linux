// SPDX-License-Identifier: MIT
/*
 * AMD Trusted Execution Environment (TEE) interface
 *
 * Author: Rijo Thomas <Rijo-john.Thomas@amd.com>
 * Author: Devaraj Rangasamy <Devaraj.Rangasamy@amd.com>
 *
 * Copyright (C) 2019,2021 Advanced Micro Devices, Inc.
 */

// Linux headers and local headers from the C implementation are external
// dependencies of this translation.

static mut psp_dead: bool = false;

unsafe fn tee_alloc_ring(tee: *mut psp_tee_device, ring_size: i32) -> i32 {
    let rb_mgr: *mut ring_buf_manager = &mut (*tee).rb_mgr;
    let mut start_addr: *mut core::ffi::c_void;

    if ring_size == 0 {
        return -EINVAL;
    }

    /* We need actual physical address instead of DMA address, since
     * Trusted OS running on AMD Secure Processor will map this region
     */
    start_addr = __get_free_pages(GFP_KERNEL, get_order(ring_size as usize)) as *mut core::ffi::c_void;
    if start_addr.is_null() {
        return -ENOMEM;
    }

    memset(start_addr, 0x0, ring_size as usize);
    (*rb_mgr).ring_start = start_addr;
    (*rb_mgr).ring_size = ring_size;
    (*rb_mgr).ring_pa = __psp_pa(start_addr);
    mutex_init(&mut (*rb_mgr).mutex);

    0
}

unsafe fn tee_free_ring(tee: *mut psp_tee_device) {
    let rb_mgr: *mut ring_buf_manager = &mut (*tee).rb_mgr;

    if (*rb_mgr).ring_start.is_null() {
        return;
    }

    free_pages((*rb_mgr).ring_start as usize, get_order((*rb_mgr).ring_size as usize));

    (*rb_mgr).ring_start = core::ptr::null_mut();
    (*rb_mgr).ring_size = 0;
    (*rb_mgr).ring_pa = 0;
    mutex_destroy(&mut (*rb_mgr).mutex);
}

unsafe fn tee_alloc_cmd_buffer(tee: *mut psp_tee_device) -> *mut tee_init_ring_cmd {
    let cmd = kzalloc_obj::<tee_init_ring_cmd>();
    if cmd.is_null() {
        return core::ptr::null_mut();
    }

    (*cmd).hi_addr = upper_32_bits((*tee).rb_mgr.ring_pa);
    (*cmd).low_addr = lower_32_bits((*tee).rb_mgr.ring_pa);
    (*cmd).size = (*tee).rb_mgr.ring_size;

    dev_dbg((*tee).dev, "tee: ring address: high = 0x%x low = 0x%x size = %u\n",
            (*cmd).hi_addr, (*cmd).low_addr, (*cmd).size);

    cmd
}

unsafe fn tee_free_cmd_buffer(cmd: *mut tee_init_ring_cmd) {
    kfree(cmd);
}

unsafe fn tee_send_destroy_cmd(tee: *mut psp_tee_device) -> bool {
    let mut reg: u32 = 0;
    let ret: i32;

    ret = psp_mailbox_command((*tee).psp, PSP_CMD_TEE_RING_DESTROY, core::ptr::null_mut(),
                              TEE_DEFAULT_CMD_TIMEOUT, &mut reg);
    if ret != 0 {
        dev_err((*tee).dev, "tee: ring destroy command timed out, disabling TEE support");
        psp_dead = true;
        return false;
    }

    if FIELD_GET(PSP_CMDRESP_STS, reg) != 0 {
        dev_err((*tee).dev, "tee: ring destroy command failed (%#010lx)\n",
                FIELD_GET(PSP_CMDRESP_STS, reg));
        psp_dead = true;
        return false;
    }

    true
}

unsafe fn tee_init_ring(tee: *mut psp_tee_device) -> i32 {
    let ring_size: i32 = MAX_RING_BUFFER_ENTRIES * core::mem::size_of::<tee_ring_cmd>() as i32;
    let mut cmd: *mut tee_init_ring_cmd;
    let mut retry = false;
    let mut reg: u32 = 0;
    let mut ret: i32;

    // BUILD_BUG_ON(sizeof(struct tee_ring_cmd) != 1024);

    ret = tee_alloc_ring(tee, ring_size);
    if ret != 0 {
        dev_err((*tee).dev, "tee: ring allocation failed %d", ret);
        return ret;
    }

    (*tee).rb_mgr.wptr = 0;

    cmd = tee_alloc_cmd_buffer(tee);
    if cmd.is_null() {
        tee_free_ring(tee);
        return -ENOMEM;
    }

    /* Send command buffer details to Trusted OS by writing to
     * CPU-PSP message registers
     */
    loop {
        ret = psp_mailbox_command((*tee).psp, PSP_CMD_TEE_RING_INIT, cmd,
                                  TEE_DEFAULT_CMD_TIMEOUT, &mut reg);
        if ret != 0 {
            dev_err((*tee).dev, "tee: ring init command timed out, disabling TEE support");
            tee_free_ring(tee);
            psp_dead = true;
            break;
        }

        if FIELD_GET(PSP_CMDRESP_STS, reg) != 0 {
            /*
             * During the hibernate resume sequence driver may have gotten loaded
             * but the ring not properly destroyed. If the ring doesn't work, try
             * to destroy and re-init once.
             */
            if !retry && FIELD_GET(PSP_CMDRESP_STS, reg) == PSP_TEE_STS_RING_BUSY {
                dev_info((*tee).dev, "tee: ring init command failed with busy status, retrying\n");
                if tee_send_destroy_cmd(tee) {
                    retry = true;
                    continue;
                }
            }
            dev_err((*tee).dev, "tee: ring init command failed (%#010lx)\n",
                    FIELD_GET(PSP_CMDRESP_STS, reg));
            tee_free_ring(tee);
            psp_dead = true;
            ret = -EIO;
        }
        break;
    }

    tee_free_cmd_buffer(cmd);
    ret
}

unsafe fn tee_destroy_ring(tee: *mut psp_tee_device) {
    if (*tee).rb_mgr.ring_start.is_null() {
        return;
    }

    if !psp_dead {
        tee_send_destroy_cmd(tee);
    }

    tee_free_ring(tee);
}

pub unsafe fn tee_dev_init(psp: *mut psp_device) -> i32 {
    let dev = (*psp).dev;
    let mut tee: *mut psp_tee_device;
    let mut ret: i32 = -ENOMEM;

    tee = devm_kzalloc(dev, core::mem::size_of::<psp_tee_device>(), GFP_KERNEL) as *mut psp_tee_device;
    if tee.is_null() {
        goto_e_err(psp, dev);
        return ret;
    }

    (*psp).tee_data = tee;
    (*tee).dev = dev;
    (*tee).psp = psp;
    (*tee).io_regs = (*psp).io_regs;
    (*tee).vdata = (*psp).vdata.tee as *mut tee_vdata;
    if (*tee).vdata.is_null() {
        ret = -ENODEV;
        dev_err(dev, "tee: missing driver data");
        goto_e_err(psp, dev);
        return ret;
    }

    ret = tee_init_ring(tee);
    if ret != 0 {
        dev_err(dev, "tee: failed to init ring buffer");
        goto_e_err(psp, dev);
        return ret;
    }

    dev_notice(dev, "tee enabled");
    return 0;
}

unsafe fn goto_e_err(psp: *mut psp_device, dev: *mut device) {
    (*psp).tee_data = core::ptr::null_mut();
    dev_notice(dev, "tee initialization failed");
}

pub unsafe fn tee_dev_destroy(psp: *mut psp_device) {
    let tee = (*psp).tee_data;
    if tee.is_null() {
        return;
    }
    tee_destroy_ring(tee);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
