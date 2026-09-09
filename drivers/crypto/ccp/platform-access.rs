// SPDX-License-Identifier: GPL-2.0
/*
 * AMD Platform Security Processor (PSP) Platform Access interface
 *
 * Copyright (C) 2023 Advanced Micro Devices, Inc.
 *
 * Author: Mario Limonciello <mario.limonciello@amd.com>
 *
 * Some of this code is adapted from drivers/i2c/busses/i2c-designware-amdpsp.c
 * developed by Jan Dabros <jsd@semihalf.com> and Copyright (C) 2022 Google Inc.
 *
 */

// Linux dependencies supplied by the surrounding translation unit:
// bitfield helpers, errno values, polling, mutexes, and platform-access types.

const PSP_CMD_TIMEOUT_US: u32 = 500 * USEC_PER_MSEC;
const DOORBELL_CMDRESP_STS: u32 = GENMASK(7, 0);

/* Recovery field should be equal 0 to start sending commands */
unsafe fn check_recovery(cmd: *mut u32) -> i32 {
    FIELD_GET(PSP_CMDRESP_RECOVERY, ioread32(cmd))
}

unsafe fn wait_cmd(cmd: *mut u32) -> i32 {
    let mut tmp: u32 = 0;
    let expected: u32;

    /* Expect mbox_cmd to be cleared and ready bit to be set by PSP */
    expected = FIELD_PREP(PSP_CMDRESP_RESP, 1);

    /*
     * Check for readiness of PSP mailbox in a tight loop in order to
     * process further as soon as command was consumed.
     */
    readl_poll_timeout(cmd, &mut tmp, (tmp & expected) != 0, 0,
                       PSP_CMD_TIMEOUT_US)
}

pub unsafe fn psp_check_platform_access_status() -> i32 {
    let psp = psp_get_master_device();

    if psp.is_null() || (*psp).platform_access_data.is_null() {
        return -ENODEV;
    }

    0
}
// EXPORT_SYMBOL(psp_check_platform_access_status);

pub unsafe fn psp_send_platform_access_msg(
    msg: psp_platform_access_msg,
    req: *mut psp_request,
) -> i32 {
    let psp = psp_get_master_device();
    let mut cmd: *mut u32;
    let mut lo: *mut u32;
    let mut hi: *mut u32;
    let pa_dev: *mut psp_platform_access_device;
    let req_addr: phys_addr_t;
    let mut cmd_reg: u32;
    let mut ret: i32;

    if psp.is_null() || (*psp).platform_access_data.is_null() {
        return -ENODEV;
    }

    pa_dev = (*psp).platform_access_data;

    if (*(*pa_dev).vdata).cmdresp_reg == 0
        || (*(*pa_dev).vdata).cmdbuff_addr_lo_reg == 0
        || (*(*pa_dev).vdata).cmdbuff_addr_hi_reg == 0
    {
        return -ENODEV;
    }

    cmd = (*psp).io_regs.add((*(*pa_dev).vdata).cmdresp_reg as usize);
    lo = (*psp).io_regs.add((*(*pa_dev).vdata).cmdbuff_addr_lo_reg as usize);
    hi = (*psp).io_regs.add((*(*pa_dev).vdata).cmdbuff_addr_hi_reg as usize);

    mutex_lock(&mut (*pa_dev).mailbox_mutex);

    'mailbox: loop {
    if check_recovery(cmd) != 0 {
        dev_dbg((*psp).dev, "platform mailbox is in recovery\n");
        ret = -EBUSY;
        break 'mailbox;
    }

    if wait_cmd(cmd) != 0 {
        dev_dbg((*psp).dev, "platform mailbox is not done processing command\n");
        ret = -EBUSY;
        break 'mailbox;
    }

    /*
     * Fill mailbox with address of command-response buffer, which will be
     * used for sending i2c requests as well as reading status returned by
     * PSP. Use physical address of buffer, since PSP will map this region.
     */
    req_addr = __psp_pa(req);
    iowrite32(lower_32_bits(req_addr), lo);
    iowrite32(upper_32_bits(req_addr), hi);

    print_hex_dump_debug("->psp ", DUMP_PREFIX_OFFSET, 16, 2, req,
                         (*req).header.payload_size, false);

    /* Write command register to trigger processing */
    cmd_reg = FIELD_PREP(PSP_CMDRESP_CMD, msg);
    iowrite32(cmd_reg, cmd);

    if wait_cmd(cmd) != 0 {
        ret = -ETIMEDOUT;
        break 'mailbox;
    }

    /* Ensure it was triggered by this driver */
    if ioread32(lo) != lower_32_bits(req_addr)
        || ioread32(hi) != upper_32_bits(req_addr)
    {
        ret = -EBUSY;
        break 'mailbox;
    }

    /*
     * Read status from PSP. If status is non-zero, it indicates an error
     * occurred during "processing" of the command.
     * If status is zero, it indicates the command was "processed"
     * successfully, but the result of the command is in the payload.
     * Return both cases to the caller as -EIO to investigate.
     */
    cmd_reg = ioread32(cmd);
    if FIELD_GET(PSP_CMDRESP_STS, cmd_reg) != 0 {
        (*req).header.status = FIELD_GET(PSP_CMDRESP_STS, cmd_reg);
    }
    if (*req).header.status != 0 {
        ret = -EIO;
        break 'mailbox;
    }

    print_hex_dump_debug("<-psp ", DUMP_PREFIX_OFFSET, 16, 2, req,
                         (*req).header.payload_size, false);

    ret = 0;
    break 'mailbox;
    }
    mutex_unlock(&mut (*pa_dev).mailbox_mutex);
    ret
}
// EXPORT_SYMBOL_GPL(psp_send_platform_access_msg);

pub unsafe fn psp_ring_platform_doorbell(msg: i32, result: *mut u32) -> i32 {
    let psp = psp_get_master_device();
    let pa_dev: *mut psp_platform_access_device;
    let button: *mut u32;
    let cmd: *mut u32;
    let mut ret: i32;
    let val: i32;

    if psp.is_null() || (*psp).platform_access_data.is_null() {
        return -ENODEV;
    }

    pa_dev = (*psp).platform_access_data;
    button = (*psp).io_regs.add((*(*pa_dev).vdata).doorbell_button_reg as usize);
    cmd = (*psp).io_regs.add((*(*pa_dev).vdata).doorbell_cmd_reg as usize);

    mutex_lock(&mut (*pa_dev).doorbell_mutex);

    'doorbell: loop {
    if wait_cmd(cmd) != 0 {
        dev_err((*psp).dev, "doorbell command not done processing\n");
        ret = -EBUSY;
        break 'doorbell;
    }

    iowrite32(FIELD_PREP(DOORBELL_CMDRESP_STS, msg), cmd);
    iowrite32(PSP_DRBL_RING, button);

    if wait_cmd(cmd) != 0 {
        ret = -ETIMEDOUT;
        break 'doorbell;
    }

    val = FIELD_GET(DOORBELL_CMDRESP_STS, ioread32(cmd));
    if val != 0 {
        if !result.is_null() {
            *result = val as u32;
        }
        ret = -EIO;
        break 'doorbell;
    }

    ret = 0;
    break 'doorbell;
    }
    mutex_unlock(&mut (*pa_dev).doorbell_mutex);
    ret
}
// EXPORT_SYMBOL_GPL(psp_ring_platform_doorbell);

pub unsafe fn platform_access_dev_destroy(psp: *mut psp_device) {
    let pa_dev = (*psp).platform_access_data;

    if pa_dev.is_null() {
        return;
    }

    mutex_destroy(&mut (*pa_dev).mailbox_mutex);
    mutex_destroy(&mut (*pa_dev).doorbell_mutex);
    (*psp).platform_access_data = core::ptr::null_mut();
}

pub unsafe fn platform_access_dev_init(psp: *mut psp_device) -> i32 {
    let dev = (*psp).dev;
    let pa_dev: *mut psp_platform_access_device;

    pa_dev = devm_kzalloc(dev, core::mem::size_of::<psp_platform_access_device>(), GFP_KERNEL)
        as *mut psp_platform_access_device;
    if pa_dev.is_null() {
        return -ENOMEM;
    }

    (*psp).platform_access_data = pa_dev;
    (*pa_dev).psp = psp;
    (*pa_dev).dev = dev;

    (*pa_dev).vdata = (*psp).vdata.platform_access as *mut platform_access_vdata;

    mutex_init(&mut (*pa_dev).mailbox_mutex);
    mutex_init(&mut (*pa_dev).doorbell_mutex);

    dev_dbg(dev, "platform access enabled\n");

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
