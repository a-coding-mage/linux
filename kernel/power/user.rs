// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/kernel/power/user.c
 *
 * This file provides the user space interface for software suspend/resume.
 *
 * Copyright (C) 2006 Rafael J. Wysocki <rjw@sisk.pl>
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

static mut NEED_WAIT: bool = false;

#[repr(C)]
struct SnapshotData {
    handle: SnapshotHandle,
    swap: i32,
    mode: i32,
    frozen: bool,
    ready: bool,
    platform_support: bool,
    free_bitmaps: bool,
    dev: DevT,
}

static mut SNAPSHOT_STATE: SnapshotData = SnapshotData {
    handle: SnapshotHandle::zeroed(),
    swap: 0,
    mode: 0,
    frozen: false,
    ready: false,
    platform_support: false,
    free_bitmaps: false,
    dev: 0,
};

pub unsafe fn is_hibernate_resume_dev(dev: DevT) -> i32 {
    (hibernation_available() && SNAPSHOT_STATE.dev == dev) as i32
}

unsafe fn snapshot_open(inode: *mut Inode, filp: *mut File) -> i32 {
    let data: *mut SnapshotData;
    let sleep_flags: u32;
    let mut error: i32;

    if !hibernation_available() { return -EPERM; }
    sleep_flags = lock_system_sleep();
    if !hibernate_acquire() {
        error = -EBUSY;
        unlock_system_sleep(sleep_flags);
        return error;
    }
    if ((*filp).f_flags & O_ACCMODE) == O_RDWR {
        hibernate_release();
        error = -ENOSYS;
        unlock_system_sleep(sleep_flags);
        return error;
    }
    nonseekable_open(inode, filp);
    data = &raw mut SNAPSHOT_STATE;
    (*filp).private_data = data as *mut core::ffi::c_void;
    core::ptr::write_bytes(&mut (*data).handle as *mut SnapshotHandle as *mut u8, 0, core::mem::size_of::<SnapshotHandle>());
    if ((*filp).f_flags & O_ACCMODE) == O_RDONLY {
        (*data).swap = pin_hibernation_swap_type(swsusp_resume_device, 0);
        (*data).mode = O_RDONLY;
        (*data).free_bitmaps = false;
        error = pm_notifier_call_chain_robust(PM_HIBERNATION_PREPARE, PM_POST_HIBERNATION);
    } else {
        NEED_WAIT = true;
        (*data).swap = -1;
        (*data).mode = O_WRONLY;
        error = pm_notifier_call_chain_robust(PM_RESTORE_PREPARE, PM_POST_RESTORE);
        if error == 0 {
            error = create_basic_memory_bitmaps();
            (*data).free_bitmaps = error == 0;
        }
    }
    if error != 0 { unpin_hibernation_swap_type((*data).swap); hibernate_release(); }
    (*data).frozen = false;
    (*data).ready = false;
    (*data).platform_support = false;
    (*data).dev = 0;
    unlock_system_sleep(sleep_flags);
    error
}

unsafe fn snapshot_release(_inode: *mut Inode, filp: *mut File) -> i32 {
    let sleep_flags = lock_system_sleep();
    swsusp_free();
    let data = (*filp).private_data as *mut SnapshotData;
    (*data).dev = 0;
    free_all_swap_pages((*data).swap);
    unpin_hibernation_swap_type((*data).swap);
    if (*data).frozen {
        pm_restore_gfp_mask(); free_basic_memory_bitmaps(); thaw_processes();
    } else if (*data).free_bitmaps { free_basic_memory_bitmaps(); }
    pm_notifier_call_chain(if (*data).mode == O_RDONLY { PM_POST_HIBERNATION } else { PM_POST_RESTORE });
    hibernate_release();
    unlock_system_sleep(sleep_flags);
    0
}

unsafe fn snapshot_read(filp: *mut File, buf: *mut u8, count: usize, offp: *mut LoFF) -> isize {
    let mut pg_offp = *offp & !PAGE_MASK;
    let sleep_flags = lock_system_sleep();
    let data = (*filp).private_data as *mut SnapshotData;
    let res: isize;
    if !(*data).ready { res = -ENODATA as isize; }
    else {
        if pg_offp == 0 { res = snapshot_read_next(&mut (*data).handle); if res <= 0 { unlock_system_sleep(sleep_flags); return res; } }
        else { res = PAGE_SIZE as isize - pg_offp as isize; }
        let result = simple_read_from_buffer(buf, count, &mut pg_offp, data_of((*data).handle), res);
        if result > 0 { *offp += result as LoFF; }
        unlock_system_sleep(sleep_flags); return result;
    }
    unlock_system_sleep(sleep_flags); res
}

unsafe fn snapshot_write(filp: *mut File, buf: *const u8, count: usize, offp: *mut LoFF) -> isize {
    if NEED_WAIT { wait_for_device_probe(); NEED_WAIT = false; }
    let sleep_flags = lock_system_sleep();
    let data = (*filp).private_data as *mut SnapshotData;
    let pg_offp = *offp & !PAGE_MASK;
    let mut res = if pg_offp == 0 { snapshot_write_next(&mut (*data).handle) } else { PAGE_SIZE as isize };
    if res <= 0 { unlock_system_sleep(sleep_flags); return res; }
    if data_of((*data).handle).is_null() { unlock_system_sleep(sleep_flags); return -EINVAL as isize; }
    res = simple_write_to_buffer(data_of((*data).handle), res, &pg_offp, buf, count);
    if res > 0 { *offp += res as LoFF; }
    unlock_system_sleep(sleep_flags); res
}

#[repr(C, packed)]
struct CompatResumeSwapArea { offset: CompatLoFF, dev: u32 }

unsafe fn snapshot_set_swap_area(data: *mut SnapshotData, argp: *mut core::ffi::c_void) -> i32 {
    if swsusp_swap_in_use() { return -EPERM; }
    let (swdev, offset) = if in_compat_syscall() {
        let mut area = CompatResumeSwapArea { offset: 0, dev: 0 };
        if copy_from_user(&mut area as *mut _ as *mut u8, argp as *const u8, core::mem::size_of_val(&area)) != 0 { return -EFAULT; }
        (new_decode_dev(area.dev), area.offset as SectorT)
    } else {
        let mut area = ResumeSwapArea { offset: 0, dev: 0 };
        if copy_from_user(&mut area as *mut _ as *mut u8, argp as *const u8, core::mem::size_of_val(&area)) != 0 { return -EFAULT; }
        (new_decode_dev(area.dev), area.offset as SectorT)
    };
    unpin_hibernation_swap_type((*data).swap);
    (*data).swap = pin_hibernation_swap_type(swdev, offset);
    if (*data).swap < 0 { return if swdev != 0 { -ENODEV } else { -EINVAL }; }
    (*data).dev = swdev;
    0
}

// The ioctl switch and file-operation registration retain the C ABI and are
// declared through the kernel types/constants supplied by the build.
unsafe fn snapshot_ioctl(filp: *mut File, cmd: u32, arg: usize) -> isize {
    if NEED_WAIT { wait_for_device_probe(); NEED_WAIT = false; }
    if _IOC_TYPE(cmd) != SNAPSHOT_IOC_MAGIC || _IOC_NR(cmd) > SNAPSHOT_IOC_MAXNR { return -ENOTTY as isize; }
    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    if !mutex_trylock(&raw mut system_transition_mutex) { return -EBUSY as isize; }
    lock_device_hotplug();
    let data = (*filp).private_data as *mut SnapshotData;
    let mut error = 0;
    match cmd {
        SNAPSHOT_FREEZE => { if !(*data).frozen { error = pm_sleep_fs_sync(); if error == 0 { error = freeze_processes(); } if error == 0 { error = create_basic_memory_bitmaps(); if error == 0 { (*data).frozen = true; } else { thaw_processes(); } } }
        SNAPSHOT_UNFREEZE => { if (*data).frozen && !(*data).ready { pm_restore_gfp_mask(); free_basic_memory_bitmaps(); (*data).free_bitmaps = false; thaw_processes(); (*data).frozen = false; } }
        SNAPSHOT_CREATE_IMAGE => { if (*data).mode != O_RDONLY || !(*data).frozen || (*data).ready { error = -EPERM; } else { pm_restore_gfp_mask(); error = hibernation_snapshot((*data).platform_support); if error == 0 { error = put_user(in_suspend, arg as *mut i32); (*data).ready = !freezer_test_done && error == 0; freezer_test_done = false; } } }
        SNAPSHOT_ATOMIC_RESTORE => { error = snapshot_write_finalize(&mut (*data).handle); if error == 0 { if (*data).mode != O_WRONLY || !(*data).frozen { error = -EPERM; } else if !snapshot_image_loaded(&(*data).handle) { error = -ENODATA; } else { error = hibernation_restore((*data).platform_support); } } }
        SNAPSHOT_FREE => { swsusp_free(); core::ptr::write_bytes(&mut (*data).handle as *mut _ as *mut u8, 0, core::mem::size_of::<SnapshotHandle>()); (*data).ready = false; thaw_kernel_threads(); }
        SNAPSHOT_PREF_IMAGE_SIZE => image_size = arg,
        SNAPSHOT_GET_IMAGE_SIZE => { if !(*data).ready { error = -ENODATA; } else { let size = snapshot_get_image_size() << PAGE_SHIFT; error = put_user(size, arg as *mut LoFF); } }
        SNAPSHOT_AVAIL_SWAP_SIZE => { let size = count_swap_pages((*data).swap, 1) << PAGE_SHIFT; error = put_user(size, arg as *mut LoFF); }
        SNAPSHOT_ALLOC_SWAP_PAGE => { if (*data).swap < 0 || (*data).swap >= MAX_SWAPFILES { error = -ENODEV; } else { let offset = alloc_swapdev_block((*data).swap); if offset != 0 { error = put_user(offset << PAGE_SHIFT, arg as *mut LoFF); } else { error = -ENOSPC; } } }
        SNAPSHOT_FREE_SWAP_PAGES => { if (*data).swap < 0 || (*data).swap >= MAX_SWAPFILES { error = -ENODEV; } else { free_all_swap_pages((*data).swap); } }
        SNAPSHOT_S2RAM => { if !(*data).frozen { error = -EPERM; } else { error = suspend_devices_and_enter(PM_SUSPEND_MEM); (*data).ready = false; } }
        SNAPSHOT_PLATFORM_SUPPORT => (*data).platform_support = arg != 0,
        SNAPSHOT_POWER_OFF => if (*data).platform_support { error = hibernation_platform_enter(); },
        SNAPSHOT_SET_SWAP_AREA => error = snapshot_set_swap_area(data, arg as *mut core::ffi::c_void),
        _ => error = -ENOTTY,
    }
    unlock_device_hotplug(); mutex_unlock(&raw mut system_transition_mutex); error as isize
}

#[cfg(CONFIG_COMPAT)]
unsafe fn snapshot_compat_ioctl(file: *mut File, cmd: u32, arg: usize) -> isize {
    match cmd {
        SNAPSHOT_GET_IMAGE_SIZE | SNAPSHOT_AVAIL_SWAP_SIZE | SNAPSHOT_ALLOC_SWAP_PAGE |
        SNAPSHOT_CREATE_IMAGE | SNAPSHOT_SET_SWAP_AREA => snapshot_ioctl(file, cmd, compat_ptr(arg) as usize),
        _ => snapshot_ioctl(file, cmd, arg),
    }
}

#[repr(C)]
struct FileOperations {
    open: Option<unsafe fn(*mut Inode, *mut File) -> i32>,
    release: Option<unsafe fn(*mut Inode, *mut File) -> i32>,
    read: Option<unsafe fn(*mut File, *mut u8, usize, *mut LoFF) -> isize>,
    write: Option<unsafe fn(*mut File, *const u8, usize, *mut LoFF) -> isize>,
    unlocked_ioctl: Option<unsafe fn(*mut File, u32, usize) -> isize>,
}

static SNAPSHOT_FOPS: FileOperations = FileOperations {
    open: Some(snapshot_open),
    release: Some(snapshot_release),
    read: Some(snapshot_read),
    write: Some(snapshot_write),
    unlocked_ioctl: Some(snapshot_ioctl),
};

#[repr(C)]
struct MiscDevice { minor: i32, name: *const u8, fops: *const FileOperations }

static mut SNAPSHOT_DEVICE: MiscDevice = MiscDevice {
    minor: SNAPSHOT_MINOR,
    name: b"snapshot\0".as_ptr(),
    fops: &SNAPSHOT_FOPS,
};

unsafe fn snapshot_device_init() -> i32 { misc_register(&raw mut SNAPSHOT_DEVICE) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
