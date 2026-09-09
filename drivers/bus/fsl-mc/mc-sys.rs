// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Copyright 2013-2016 Freescale Semiconductor Inc.
 *
 * I/O services to send MC commands to the MC hardware
 */

const MC_CMD_COMPLETION_TIMEOUT_MS: u32 = 15000;
const MC_CMD_COMPLETION_POLLING_MIN_SLEEP_USECS: u32 = 10;
const MC_CMD_COMPLETION_POLLING_MAX_SLEEP_USECS: u32 = 500;

unsafe fn mc_cmd_hdr_read_status(cmd: *mut fsl_mc_command) -> mc_cmd_status {
    let hdr = &mut (*(cmd as *mut mc_cmd_header));
    hdr.status as mc_cmd_status
}

pub unsafe fn mc_cmd_hdr_read_cmdid(cmd: *mut fsl_mc_command) -> u16 {
    let hdr = &mut (*(cmd as *mut mc_cmd_header));
    le16_to_cpu(hdr.cmd_id)
}

unsafe fn mc_status_to_error(status: mc_cmd_status) -> i32 {
    match status as u32 {
        MC_CMD_STATUS_OK => 0,
        MC_CMD_STATUS_AUTH_ERR => -EACCES,
        MC_CMD_STATUS_NO_PRIVILEGE => -EPERM,
        MC_CMD_STATUS_DMA_ERR => -EIO,
        MC_CMD_STATUS_CONFIG_ERR => -ENXIO,
        MC_CMD_STATUS_TIMEOUT => -ETIMEDOUT,
        MC_CMD_STATUS_NO_RESOURCE => -ENAVAIL,
        MC_CMD_STATUS_NO_MEMORY => -ENOMEM,
        MC_CMD_STATUS_BUSY => -EBUSY,
        MC_CMD_STATUS_UNSUPPORTED_OP => -ENOTSUPP,
        MC_CMD_STATUS_INVALID_STATE => -ENODEV,
        _ => -EINVAL,
    }
}

unsafe fn mc_status_to_string(status: mc_cmd_status) -> *const u8 {
    match status as u32 {
        MC_CMD_STATUS_OK => b"Command completed successfully\0".as_ptr(),
        MC_CMD_STATUS_READY => b"Command ready to be processed\0".as_ptr(),
        MC_CMD_STATUS_AUTH_ERR => b"Authentication error\0".as_ptr(),
        MC_CMD_STATUS_NO_PRIVILEGE => b"No privilege\0".as_ptr(),
        MC_CMD_STATUS_DMA_ERR => b"DMA or I/O error\0".as_ptr(),
        MC_CMD_STATUS_CONFIG_ERR => b"Configuration error\0".as_ptr(),
        MC_CMD_STATUS_TIMEOUT => b"Operation timed out\0".as_ptr(),
        MC_CMD_STATUS_NO_RESOURCE => b"No resources\0".as_ptr(),
        MC_CMD_STATUS_NO_MEMORY => b"No memory available\0".as_ptr(),
        MC_CMD_STATUS_BUSY => b"Device is busy\0".as_ptr(),
        MC_CMD_STATUS_UNSUPPORTED_OP => b"Unsupported operation\0".as_ptr(),
        MC_CMD_STATUS_INVALID_STATE => b"Invalid state\0".as_ptr(),
        _ => b"Unknown MC error\0".as_ptr(),
    }
}

unsafe fn mc_write_command(portal: *mut fsl_mc_command, cmd: *mut fsl_mc_command) {
    for i in 0..MC_CMD_NUM_OF_PARAMS {
        // Data is already in LE byte-order; convert LE -> CPU so the I/O write
        // conversion CPU -> LE puts it back in the expected order.
        writeq_relaxed(le64_to_cpu((*cmd).params[i]), &mut (*portal).params[i]);
    }
    writeq(le64_to_cpu((*cmd).header), &mut (*portal).header);
}

unsafe fn mc_read_response(portal: *mut fsl_mc_command,
                            resp: *mut fsl_mc_command) -> mc_cmd_status {
    (*resp).header = cpu_to_le64(readq_relaxed(&(*portal).header));
    let status = mc_cmd_hdr_read_status(resp);
    if status != MC_CMD_STATUS_OK {
        return status;
    }
    for i in 0..MC_CMD_NUM_OF_PARAMS {
        (*resp).params[i] = cpu_to_le64(readq_relaxed(&(*portal).params[i]));
    }
    status
}

unsafe fn mc_polling_wait_preemptible(mc_io: *mut fsl_mc_io,
                                      cmd: *mut fsl_mc_command,
                                      mc_status: *mut mc_cmd_status) -> i32 {
    let jiffies_until_timeout = jiffies + msecs_to_jiffies(MC_CMD_COMPLETION_TIMEOUT_MS);
    loop {
        let status = mc_read_response((*mc_io).portal_virt_addr, cmd);
        if status != MC_CMD_STATUS_READY {
            *mc_status = status;
            return 0;
        }
        usleep_range(MC_CMD_COMPLETION_POLLING_MIN_SLEEP_USECS,
                     MC_CMD_COMPLETION_POLLING_MAX_SLEEP_USECS);
        if time_after_eq(jiffies, jiffies_until_timeout) {
            dev_dbg((*mc_io).dev, "MC command timed out (portal: %pa, dprc handle: %#x, command: %#x)\n",
                    &(*mc_io).portal_phys_addr,
                    mc_cmd_hdr_read_token(cmd) as u32,
                    mc_cmd_hdr_read_cmdid(cmd) as u32);
            return -ETIMEDOUT;
        }
    }
}

unsafe fn mc_polling_wait_atomic(mc_io: *mut fsl_mc_io,
                                 cmd: *mut fsl_mc_command,
                                 mc_status: *mut mc_cmd_status) -> i32 {
    let mut timeout_usecs = MC_CMD_COMPLETION_TIMEOUT_MS * 1000;
    loop {
        let status = mc_read_response((*mc_io).portal_virt_addr, cmd);
        if status != MC_CMD_STATUS_READY {
            *mc_status = status;
            return 0;
        }
        udelay(MC_CMD_COMPLETION_POLLING_MAX_SLEEP_USECS);
        timeout_usecs -= MC_CMD_COMPLETION_POLLING_MAX_SLEEP_USECS;
        if timeout_usecs == 0 {
            dev_dbg((*mc_io).dev, "MC command timed out (portal: %pa, dprc handle: %#x, command: %#x)\n",
                    &(*mc_io).portal_phys_addr,
                    mc_cmd_hdr_read_token(cmd) as u32,
                    mc_cmd_hdr_read_cmdid(cmd) as u32);
            return -ETIMEDOUT;
        }
    }
}

pub unsafe fn mc_send_command(mc_io: *mut fsl_mc_io, cmd: *mut fsl_mc_command) -> i32 {
    let mut status: mc_cmd_status = MC_CMD_STATUS_READY;
    let mut irq_flags: unsigned_long = 0;
    if in_hardirq() && ((*mc_io).flags & FSL_MC_IO_ATOMIC_CONTEXT_PORTAL) == 0 {
        return -EINVAL;
    }
    if ((*mc_io).flags & FSL_MC_IO_ATOMIC_CONTEXT_PORTAL) != 0 {
        raw_spin_lock_irqsave(&mut (*mc_io).spinlock, &mut irq_flags);
    } else {
        mutex_lock(&mut (*mc_io).mutex);
    }
    mc_write_command((*mc_io).portal_virt_addr, cmd);
    let mut error = if ((*mc_io).flags & FSL_MC_IO_ATOMIC_CONTEXT_PORTAL) == 0 {
        mc_polling_wait_preemptible(mc_io, cmd, &mut status)
    } else {
        mc_polling_wait_atomic(mc_io, cmd, &mut status)
    };
    if error >= 0 && status != MC_CMD_STATUS_OK {
        dev_dbg((*mc_io).dev, "MC command failed: portal: %pa, dprc handle: %#x, command: %#x, status: %s (%#x)\n",
                &(*mc_io).portal_phys_addr, mc_cmd_hdr_read_token(cmd) as u32,
                mc_cmd_hdr_read_cmdid(cmd) as u32, mc_status_to_string(status), status as u32);
        error = mc_status_to_error(status);
    }
    if ((*mc_io).flags & FSL_MC_IO_ATOMIC_CONTEXT_PORTAL) != 0 {
        raw_spin_unlock_irqrestore(&mut (*mc_io).spinlock, irq_flags);
    } else {
        mutex_unlock(&mut (*mc_io).mutex);
    }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
