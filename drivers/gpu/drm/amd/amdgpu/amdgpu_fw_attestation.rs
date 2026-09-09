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

const FW_ATTESTATION_DB_COOKIE: u32 = 0x143b6a37;
const FW_ATTESTATION_RECORD_VALID: u8 = 1;
const FW_ATTESTATION_MAX_SIZE: i64 = 4096;

#[repr(C)]
struct FW_ATT_DB_HEADER {
    AttDbVersion: u32,
    AttDbCookie: u32,
}

#[repr(C)]
struct FW_ATT_RECORD {
    AttFwIdV1: u16,
    AttFwIdV2: u16,
    AttFWVersion: u32,
    AttFWActiveFunctionID: u16,
    AttSource: u8,
    RecordValid: u8,
    AttFwTaId: u32,
}

unsafe fn amdgpu_fw_attestation_debugfs_read(
    f: *mut file,
    buf: *mut core::ffi::c_char,
    size: usize,
    pos: *mut loff_t,
) -> ssize_t {
    let adev = (*file_inode(f)).i_private as *mut amdgpu_device;
    let mut records_addr: u64 = 0;
    let mut vram_pos: u64 = 0;
    let mut fw_att_hdr: FW_ATT_DB_HEADER = core::mem::zeroed();
    let mut fw_att_record: FW_ATT_RECORD = core::mem::zeroed();

    if size < core::mem::size_of::<FW_ATT_RECORD>() {
        drm_warn(adev_to_drm(adev), "FW attestation input buffer not enough memory");
        return -EINVAL;
    }

    if (*pos + core::mem::size_of::<FW_ATT_DB_HEADER>() as i64) >= FW_ATTESTATION_MAX_SIZE {
        drm_warn(adev_to_drm(adev), "FW attestation out of bounds");
        return 0;
    }

    if psp_get_fw_attestation_records_addr(&mut (*adev).psp, &mut records_addr) != 0 {
        drm_warn(adev_to_drm(adev), "Failed to get FW attestation record address");
        return -EINVAL;
    }

    vram_pos = records_addr - (*adev).gmc.vram_start;

    if *pos == 0 {
        amdgpu_device_vram_access(
            adev,
            vram_pos,
            &mut fw_att_hdr as *mut FW_ATT_DB_HEADER as *mut u32,
            core::mem::size_of::<FW_ATT_DB_HEADER>() as u32,
            false,
        );

        if fw_att_hdr.AttDbCookie != FW_ATTESTATION_DB_COOKIE {
            drm_warn(adev_to_drm(adev), "Invalid FW attestation cookie");
            return -EINVAL;
        }

        drm_info!(adev_to_drm(adev), "FW attestation version = 0x%X", fw_att_hdr.AttDbVersion);
    }

    amdgpu_device_vram_access(
        adev,
        vram_pos + core::mem::size_of::<FW_ATT_DB_HEADER>() as u64 + *pos as u64,
        &mut fw_att_record as *mut FW_ATT_RECORD as *mut u32,
        core::mem::size_of::<FW_ATT_RECORD>() as u32,
        false,
    );

    if fw_att_record.RecordValid != FW_ATTESTATION_RECORD_VALID {
        return 0;
    }

    if copy_to_user(buf as *mut core::ffi::c_void, &fw_att_record as *const FW_ATT_RECORD as *const core::ffi::c_void, core::mem::size_of::<FW_ATT_RECORD>()) != 0 {
        return -EINVAL;
    }

    *pos += core::mem::size_of::<FW_ATT_RECORD>() as i64;
    core::mem::size_of::<FW_ATT_RECORD>() as ssize_t
}

static amdgpu_fw_attestation_debugfs_ops: file_operations = file_operations {
    owner: THIS_MODULE,
    read: Some(amdgpu_fw_attestation_debugfs_read),
    write: None,
    llseek: Some(default_llseek),
};

unsafe fn amdgpu_is_fw_attestation_supported(adev: *mut amdgpu_device) -> i32 {
    if (*adev).flags & AMD_IS_APU != 0 {
        return 0;
    }

    if amdgpu_ip_version(adev, MP0_HWIP, 0) == IP_VERSION(14, 0, 2)
        || amdgpu_ip_version(adev, MP0_HWIP, 0) == IP_VERSION(14, 0, 3)
    {
        return 0;
    }

    if (*adev).asic_type >= CHIP_SIENNA_CICHLID {
        return 1;
    }

    0
}

pub unsafe fn amdgpu_fw_attestation_debugfs_init(adev: *mut amdgpu_device) {
    if amdgpu_is_fw_attestation_supported(adev) == 0 {
        return;
    }

    debugfs_create_file(
        "amdgpu_fw_attestation",
        0o400,
        (*adev_to_drm(adev)).primary.debugfs_root,
        adev as *mut core::ffi::c_void,
        &amdgpu_fw_attestation_debugfs_ops,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
