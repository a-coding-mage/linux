// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure Processor Seamless Firmware Servicing support.
 *
 * Copyright (C) 2025 Advanced Micro Devices, Inc.
 *
 * Author: Ashish Kalra <ashish.kalra@amd.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/firmware.h, sfs.h, and sev-dev.h.

const SFS_DEFAULT_TIMEOUT: u32 = 10 * MSEC_PER_SEC;
const SFS_MAX_PAYLOAD_SIZE: usize = 2 * 1024 * 1024;
const SFS_NUM_2MB_PAGES_CMDBUF: usize = SFS_MAX_PAYLOAD_SIZE / PMD_SIZE;
const SFS_NUM_PAGES_CMDBUF: usize = SFS_MAX_PAYLOAD_SIZE / PAGE_SIZE;

static mut SFS_IOCTL_MUTEX: Mutex = DEFINE_MUTEX!();
static mut MISC_DEV: *mut sfs_misc_dev = core::ptr::null_mut();

unsafe fn send_sfs_cmd(sfs_dev: *mut sfs_device, msg: i32) -> i32 {
    let command_buf = (*sfs_dev).command_buf;
    (*command_buf).hdr.status = 0;
    (*command_buf).hdr.sub_cmd_id = msg;

    let ret = psp_extended_mailbox_cmd(
        (*sfs_dev).psp,
        SFS_DEFAULT_TIMEOUT,
        command_buf as *mut psp_ext_request,
    );
    if ret == -EIO {
        dev_dbg(
            (*sfs_dev).dev,
            "msg 0x{:x} failed with PSP error: 0x{:x}, extended status: 0x{:x}\n",
            msg,
            (*command_buf).hdr.status,
            *( (*command_buf).buf.as_ptr() as *const u32),
        );
    }
    ret
}

unsafe fn send_sfs_get_fw_versions(sfs_dev: *mut sfs_device) -> i32 {
    /*
     * SFS_GET_FW_VERSIONS command needs the output buffer to be
     * initialized to 0xC7 in every byte.
     */
    core::ptr::write_bytes((*sfs_dev).command_buf.as_mut().unwrap().sfs_buffer.as_mut_ptr(), 0xc7, PAGE_SIZE);
    (*sfs_dev).command_buf.hdr.payload_size = 2 * PAGE_SIZE;
    send_sfs_cmd(sfs_dev, PSP_SFS_GET_FW_VERSIONS)
}

unsafe fn send_sfs_update_package(sfs_dev: *mut sfs_device, payload_name: *const c_char) -> i32 {
    let mut payload_path = [0 as c_char; PAYLOAD_NAME_SIZE + core::mem::size_of_val(&b"amd/"[..])];
    let firmware: *const firmware;
    let package_size: usize;

    /* Sanitize userspace provided payload name */
    if strnchr(payload_name, PAYLOAD_NAME_SIZE, 0).is_null() {
        return -EINVAL;
    }

    snprintf(payload_path.as_mut_ptr(), payload_path.len(), b"amd/%s\0".as_ptr() as *const c_char, payload_name);

    let mut fw: *const firmware = core::ptr::null();
    let ret = firmware_request_nowarn(&mut fw, payload_path.as_ptr(), (*sfs_dev).dev);
    firmware = fw;
    if ret < 0 {
        dev_warn_ratelimited((*sfs_dev).dev, "firmware request failed for %s (%d)\n", payload_path.as_ptr(), ret);
        return -ENOENT;
    }

    /*
     * SFS Update Package command's input buffer contains TEE_EXT_CMD_BUFFER
     * followed by the Update Package and it should be 64KB aligned.
     */
    package_size = ALIGN((*firmware).size + PAGE_SIZE, 0x10000usize);

    /* SFS command buffer is a pre-allocated 2MB buffer. */
    if package_size > SFS_MAX_PAYLOAD_SIZE {
        dev_warn_ratelimited((*sfs_dev).dev, "SFS payload size %ld larger than maximum supported payload size of %u\n", package_size, SFS_MAX_PAYLOAD_SIZE);
        release_firmware(firmware);
        return -E2BIG;
    }

    /* Copy firmware data to a HV_Fixed memory region. */
    core::ptr::copy_nonoverlapping((*firmware).data, (*sfs_dev).command_buf.sfs_buffer.as_mut_ptr(), (*firmware).size);
    (*sfs_dev).command_buf.hdr.payload_size = package_size;
    release_firmware(firmware);
    send_sfs_cmd(sfs_dev, PSP_SFS_UPDATE)
}

unsafe fn sfs_ioctl(_filp: *mut file, cmd: u32, arg: c_ulong) -> c_long {
    let psp_master = psp_get_master_device();
    if psp_master.is_null() || (*psp_master).sfs_data.is_null() { return -ENODEV as c_long; }
    let sfs_dev = (*psp_master).sfs_data;
    let _guard = guard_mutex(&mut SFS_IOCTL_MUTEX);
    match cmd {
        SFSIOCFWVERS => {
            dev_dbg((*sfs_dev).dev, "in SFSIOCFWVERS\n");
            let user = arg as *mut sfs_user_get_fw_versions;
            let ret = send_sfs_get_fw_versions(sfs_dev);
            if ret != 0 && ret != -EIO { return ret as c_long; }
            if copy_to_user(core::ptr::addr_of_mut!((*user).blob), (*sfs_dev).command_buf.sfs_buffer.as_ptr(), PAGE_SIZE) != 0 { return -EFAULT as c_long; }
            if copy_to_user(core::ptr::addr_of_mut!((*user).sfs_status), core::ptr::addr_of!((*sfs_dev).command_buf.hdr.status), core::mem::size_of_val(&(*user).sfs_status)) != 0 { return -EFAULT as c_long; }
            if copy_to_user(core::ptr::addr_of_mut!((*user).sfs_extended_status), core::ptr::addr_of!((*sfs_dev).command_buf.buf), core::mem::size_of_val(&(*user).sfs_extended_status)) != 0 { return -EFAULT as c_long; }
            0
        }
        SFSIOCUPDATEPKG => {
            dev_dbg((*sfs_dev).dev, "in SFSIOCUPDATEPKG\n");
            let user = arg as *mut sfs_user_update_package;
            let mut payload_name = [0 as c_char; PAYLOAD_NAME_SIZE];
            if copy_from_user(payload_name.as_mut_ptr(), (*user).payload_name.as_ptr(), PAYLOAD_NAME_SIZE) != 0 { return -EFAULT as c_long; }
            let ret = send_sfs_update_package(sfs_dev, payload_name.as_ptr());
            if ret != 0 && ret != -EIO { return ret as c_long; }
            if copy_to_user(core::ptr::addr_of_mut!((*user).sfs_status), core::ptr::addr_of!((*sfs_dev).command_buf.hdr.status), core::mem::size_of_val(&(*user).sfs_status)) != 0 { return -EFAULT as c_long; }
            if copy_to_user(core::ptr::addr_of_mut!((*user).sfs_extended_status), core::ptr::addr_of!((*sfs_dev).command_buf.buf), core::mem::size_of_val(&(*user).sfs_extended_status)) != 0 { return -EFAULT as c_long; }
            0
        }
        _ => -EINVAL as c_long,
    }
}

static SFS_FOPS: file_operations = file_operations { owner: THIS_MODULE, unlocked_ioctl: Some(sfs_ioctl) };

unsafe fn sfs_exit(_ref_: *mut kref) {
    misc_deregister(&mut (*MISC_DEV).misc);
    kfree(MISC_DEV as *mut c_void);
    MISC_DEV = core::ptr::null_mut();
}

pub unsafe fn sfs_dev_destroy(psp: *mut psp_device) {
    let sfs_dev = (*psp).sfs_data;
    if sfs_dev.is_null() { return; }
    set_memory_wb((*sfs_dev).command_buf as usize, SFS_NUM_PAGES_CMDBUF);
    snp_free_hv_fixed_pages((*sfs_dev).page);
    if !(*sfs_dev).misc.is_null() { kref_put(&mut (*MISC_DEV).refcount, sfs_exit); }
    (*psp).sfs_data = core::ptr::null_mut();
}

unsafe fn sfs_misc_init(sfs: *mut sfs_device) -> i32 {
    let dev = (*sfs).dev;
    if MISC_DEV.is_null() {
        MISC_DEV = kzalloc_obj::<sfs_misc_dev>();
        if MISC_DEV.is_null() { return -ENOMEM; }
        (*MISC_DEV).misc.minor = MISC_DYNAMIC_MINOR;
        (*MISC_DEV).misc.name = b"sfs\0".as_ptr() as *const c_char;
        (*MISC_DEV).misc.fops = &SFS_FOPS;
        (*MISC_DEV).misc.mode = 0o600;
        let ret = misc_register(&mut (*MISC_DEV).misc);
        if ret != 0 { return ret; }
        kref_init(&mut (*MISC_DEV).refcount);
    } else { kref_get(&mut (*MISC_DEV).refcount); }
    (*sfs).misc = MISC_DEV;
    dev_dbg(dev, "registered SFS device\n");
    0
}

pub unsafe fn sfs_dev_init(psp: *mut psp_device) -> i32 {
    let dev = (*psp).dev;
    let sfs_dev = devm_kzalloc(dev, core::mem::size_of::<sfs_device>(), GFP_KERNEL);
    if sfs_dev.is_null() { return -ENOMEM; }
    let page = snp_alloc_hv_fixed_pages(SFS_NUM_2MB_PAGES_CMDBUF);
    if page.is_null() { dev_dbg(dev, "Command Buffer HV-Fixed page allocation failed\n"); goto_cleanup_dev(psp, dev, sfs_dev, -ENOMEM); }
    (*sfs_dev).page = page;
    (*sfs_dev).command_buf = page_address(page);
    dev_dbg(dev, "Command buffer 0x%px to be marked as HV_Fixed\n", (*sfs_dev).command_buf);
    let ret = set_memory_uc((*sfs_dev).command_buf as usize, SFS_NUM_PAGES_CMDBUF);
    if ret != 0 { dev_dbg(dev, "Set memory uc failed\n"); snp_free_hv_fixed_pages(page); goto_cleanup_dev(psp, dev, sfs_dev, ret); }
    dev_dbg(dev, "Command buffer 0x%px marked uncacheable\n", (*sfs_dev).command_buf);
    (*psp).sfs_data = sfs_dev; (*sfs_dev).dev = dev; (*sfs_dev).psp = psp;
    let ret = sfs_misc_init(sfs_dev);
    if ret != 0 { set_memory_wb((*sfs_dev).command_buf as usize, SFS_NUM_PAGES_CMDBUF); snp_free_hv_fixed_pages(page); goto_cleanup_dev(psp, dev, sfs_dev, ret); }
    dev_notice((*sfs_dev).dev, "SFS support is available\n");
    0
}

// External kernel symbols and types are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
