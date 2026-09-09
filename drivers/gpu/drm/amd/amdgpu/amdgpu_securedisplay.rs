/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/* Dependency declarations are supplied by the surrounding kernel translation. */

pub unsafe fn psp_securedisplay_parse_resp_status(
    psp: *mut psp_context,
    status: ta_securedisplay_status,
) {
    match status {
        TA_SECUREDISPLAY_STATUS__SUCCESS => {}
        TA_SECUREDISPLAY_STATUS__GENERIC_FAILURE => {
            dev_err((*psp).adev.dev, "Secure display: Generic Failure.");
        }
        TA_SECUREDISPLAY_STATUS__INVALID_PARAMETER => {
            dev_err((*psp).adev.dev, "Secure display: Invalid Parameter.");
        }
        TA_SECUREDISPLAY_STATUS__NULL_POINTER => {
            dev_err((*psp).adev.dev, "Secure display: Null Pointer.");
        }
        TA_SECUREDISPLAY_STATUS__I2C_WRITE_ERROR => {
            dev_err((*psp).adev.dev, "Secure display: Failed to write to I2C.");
        }
        TA_SECUREDISPLAY_STATUS__READ_DIO_SCRATCH_ERROR => {
            dev_err((*psp).adev.dev, "Secure display: Failed to Read DIO Scratch Register.");
        }
        TA_SECUREDISPLAY_STATUS__READ_CRC_ERROR => {
            dev_err((*psp).adev.dev, "Secure display: Failed to Read CRC");
        }
        TA_SECUREDISPLAY_STATUS__I2C_INIT_ERROR => {
            dev_err((*psp).adev.dev, "Secure display: Failed to initialize I2C.");
        }
        _ => {
            dev_err((*psp).adev.dev, "Secure display: Failed to parse status: %d\n", status);
        }
    }
}

pub unsafe fn psp_prep_securedisplay_cmd_buf(
    psp: *mut psp_context,
    cmd: *mut *mut ta_securedisplay_cmd,
    command_id: ta_securedisplay_command,
) {
    *cmd = (*psp).securedisplay_context.context.mem_context.shared_buf
        as *mut ta_securedisplay_cmd;
    core::ptr::write_bytes(*cmd as *mut u8, 0, core::mem::size_of::<ta_securedisplay_cmd>());
    (**cmd).status = TA_SECUREDISPLAY_STATUS__GENERIC_FAILURE;
    (**cmd).cmd_id = command_id;
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn amdgpu_securedisplay_debugfs_write(
    f: *mut file,
    buf: *const core::ffi::c_char,
    size: usize,
    pos: *mut loff_t,
) -> isize {
    let adev = (*file_inode(f)).i_private as *mut amdgpu_device;
    let psp = &mut (*adev).psp as *mut psp_context;
    let mut securedisplay_cmd: *mut ta_securedisplay_cmd = core::ptr::null_mut();
    let dev = adev_to_drm(adev);
    let mut phy_id: u32 = 0;
    let mut op: u32 = 0;
    let mut str_buf = [0i8; 64];
    let len: isize;
    let mut write_ret = size as isize;
    let ret: isize;
    let mut nargs: i32;

    if *pos != 0 { return -EINVAL; }
    len = strncpy_from_user(str_buf.as_mut_ptr(), buf, str_buf.len());
    if len < 0 { return -EFAULT; }
    if len == 0 || len >= str_buf.len() as isize { return -EINVAL; }

    ret = pm_runtime_get_sync((*dev).dev);
    if ret < 0 {
        pm_runtime_put_autosuspend((*dev).dev);
        return ret;
    }

    if len < 3 {
        nargs = sscanf(str_buf.as_ptr(), c"%u", &mut op);
    } else {
        nargs = sscanf(str_buf.as_ptr(), c"%u %u", &mut op, &mut phy_id);
    }
    if nargs < 1 { write_ret = -EINVAL; }
    else {
        match op {
            1 => {
                mutex_lock(&mut (*psp).securedisplay_context.mutex);
                psp_prep_securedisplay_cmd_buf(psp, &mut securedisplay_cmd,
                    TA_SECUREDISPLAY_COMMAND__QUERY_TA);
                let invoke_ret = psp_securedisplay_invoke(psp, TA_SECUREDISPLAY_COMMAND__QUERY_TA);
                if invoke_ret == 0 {
                    if (*securedisplay_cmd).status == TA_SECUREDISPLAY_STATUS__SUCCESS {
                        dev_info((*adev).dev, "SECUREDISPLAY: query securedisplay TA ret is 0x%X\n",
                            (*securedisplay_cmd).securedisplay_out_message.query_ta.query_cmd_ret);
                    } else {
                        psp_securedisplay_parse_resp_status(psp, (*securedisplay_cmd).status);
                    }
                }
                mutex_unlock(&mut (*psp).securedisplay_context.mutex);
            }
            2 => {
                if nargs < 2 || phy_id >= TA_SECUREDISPLAY_MAX_PHY {
                    dev_err((*adev).dev, "Invalid input: %s\n", str_buf.as_ptr());
                    write_ret = -EINVAL;
                } else {
                    mutex_lock(&mut (*psp).securedisplay_context.mutex);
                    psp_prep_securedisplay_cmd_buf(psp, &mut securedisplay_cmd,
                        TA_SECUREDISPLAY_COMMAND__SEND_ROI_CRC);
                    (*securedisplay_cmd).securedisplay_in_message.send_roi_crc.phy_id = phy_id;
                    let invoke_ret = psp_securedisplay_invoke(psp, TA_SECUREDISPLAY_COMMAND__SEND_ROI_CRC);
                    if invoke_ret == 0 {
                        if (*securedisplay_cmd).status == TA_SECUREDISPLAY_STATUS__SUCCESS {
                            dev_info((*adev).dev, "SECUREDISPLAY: I2C buffer out put is: %*ph\n",
                                TA_SECUREDISPLAY_I2C_BUFFER_SIZE,
                                (*securedisplay_cmd).securedisplay_out_message.send_roi_crc.i2c_buf);
                        } else {
                            psp_securedisplay_parse_resp_status(psp, (*securedisplay_cmd).status);
                        }
                    }
                    mutex_unlock(&mut (*psp).securedisplay_context.mutex);
                }
            }
            _ => {
                dev_err((*adev).dev, "Invalid input: %s\n", str_buf.as_ptr());
                write_ret = -EINVAL;
            }
        }
    }
    pm_runtime_put_autosuspend((*dev).dev);
    write_ret
}

#[cfg(CONFIG_DEBUG_FS)]
static amdgpu_securedisplay_debugfs_ops: file_operations = file_operations {
    owner: THIS_MODULE,
    read: None,
    write: Some(amdgpu_securedisplay_debugfs_write),
    llseek: Some(default_llseek),
};

pub unsafe fn amdgpu_securedisplay_debugfs_init(adev: *mut amdgpu_device) {
    #[cfg(CONFIG_DEBUG_FS)]
    {
        if !(*adev).psp.securedisplay_context.context.initialized { return; }
        debugfs_create_file(
            c"securedisplay_test", S_IWUSR,
            adev_to_drm(adev).primary.debugfs_root,
            adev, &amdgpu_securedisplay_debugfs_ops,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
