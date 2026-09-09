/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

/* AMDGPU RAP debugfs test interface. */

unsafe fn amdgpu_rap_debugfs_write(
    f: *mut file,
    buf: *const u8,
    size: usize,
    pos: *mut loff_t,
) -> isize {
    let adev = (*file_inode(f)).i_private as *mut amdgpu_device;
    let mut rap_shared_mem: *mut ta_rap_shared_memory;
    let mut rap_cmd_output: *mut ta_rap_cmd_output_data;
    let dev = adev_to_drm(adev);
    let mut op: u32 = 0;
    let mut status: ta_rap_status;
    let mut ret: i32;

    if *pos != 0 || size != 2 {
        return -EINVAL as isize;
    }

    ret = kstrtouint_from_user(buf, size, *pos, &mut op);
    if ret != 0 {
        return ret as isize;
    }

    ret = pm_runtime_get_sync((*dev).dev);
    if ret < 0 {
        pm_runtime_put_autosuspend((*dev).dev);
        return ret as isize;
    }

    /* Make sure gfx core is on; RAP TA cannot handle GFX OFF currently. */
    amdgpu_gfx_off_ctrl(adev, false);

    match op {
        2 => {
            ret = psp_rap_invoke(&mut (*adev).psp, op, &mut status);
            if ret == 0 && status == TA_RAP_STATUS__SUCCESS {
                dev_info((*adev).dev, "RAP L0 validate test success.\n");
            } else {
                rap_shared_mem = (*adev).psp.rap_context.context.mem_context.shared_buf
                    as *mut ta_rap_shared_memory;
                rap_cmd_output = &mut (*rap_shared_mem).rap_out_message.output;

                dev_info((*adev).dev, "RAP test failed, the output is:\n");
                dev_info((*adev).dev, "\tlast_subsection: 0x%08x.\n", (*rap_cmd_output).last_subsection);
                dev_info((*adev).dev, "\tnum_total_validate: 0x%08x.\n", (*rap_cmd_output).num_total_validate);
                dev_info((*adev).dev, "\tnum_valid: 0x%08x.\n", (*rap_cmd_output).num_valid);
                dev_info((*adev).dev, "\tlast_validate_addr: 0x%08x.\n", (*rap_cmd_output).last_validate_addr);
                dev_info((*adev).dev, "\tlast_validate_val: 0x%08x.\n", (*rap_cmd_output).last_validate_val);
                dev_info((*adev).dev, "\tlast_validate_val_exptd: 0x%08x.\n", (*rap_cmd_output).last_validate_val_exptd);
            }
        }
        _ => {
            dev_info((*adev).dev, "Unsupported op id: %d, ", op);
            dev_info((*adev).dev, "Only support op 2(L0 validate test).\n");
        }
    }

    amdgpu_gfx_off_ctrl(adev, true);
    pm_runtime_put_autosuspend((*dev).dev);

    size as isize
}

static AMDGPU_RAP_DEBUGFS_OPS: file_operations = file_operations {
    owner: THIS_MODULE,
    read: None,
    write: Some(amdgpu_rap_debugfs_write),
    llseek: Some(default_llseek),
};

pub unsafe fn amdgpu_rap_debugfs_init(adev: *mut amdgpu_device) {
    let minor = (*adev_to_drm(adev)).primary;

    if !(*adev).psp.rap_context.context.initialized {
        return;
    }

    debugfs_create_file(
        "rap_test",
        S_IWUSR,
        (*minor).debugfs_root,
        adev as *mut core::ffi::c_void,
        &AMDGPU_RAP_DEBUGFS_OPS,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
