// SPDX-License-Identifier: GPL-2.0-only

// Dependency intent from the C source: Linux kernel anon inodes, filesystems,
// RTAS, machine-dependent initialization, and the PAPR platform-dump UAPI.

const RTAS_IBM_PLATFORM_DUMP_COMPLETE: i32 = 0;
const RTAS_IBM_PLATFORM_DUMP_CONTINUE: i32 = 1;
const RTAS_NOT_AUTHORIZED: i32 = -9002;
const RTAS_IBM_PLATFORM_DUMP_START: i32 = 2;

#[repr(C)]
struct IbmPlatformDumpParams {
    work_area: *mut RtasWorkArea,
    buf_length: u32,
    dump_tag_hi: u32,
    dump_tag_lo: u32,
    sequence_hi: u32,
    sequence_lo: u32,
    bytes_ret_hi: u32,
    bytes_ret_lo: u32,
    status: i32,
    list: ListHead,
}

// Multiple dumps with different dump IDs can be retrieved at the same time,
// but not with the same dump ID.
static mut PLATFORM_DUMP_LIST_MUTEX: Mutex = Mutex::new();
static mut PLATFORM_DUMP_LIST: ListHead = ListHead::new();

unsafe fn rtas_ibm_platform_dump(
    params: *mut IbmPlatformDumpParams,
    buf_addr: PhysAddr,
    buf_length: u32,
) -> i32 {
    let mut rets = [0u32; 4];
    let mut fwrc: i32;
    let mut ret = 0i32;

    loop {
        fwrc = rtas_call(
            rtas_function_token(RTAS_FN_IBM_PLATFORM_DUMP),
            6,
            5,
            rets.as_mut_ptr(),
            (*params).dump_tag_hi as u64,
            (*params).dump_tag_lo as u64,
            (*params).sequence_hi as u64,
            (*params).sequence_lo as u64,
            buf_addr,
            buf_length as u64,
        );
        if !rtas_busy_delay(fwrc) {
            break;
        }
    }

    match fwrc {
        RTAS_HARDWARE_ERROR => ret = -EIO,
        RTAS_NOT_AUTHORIZED => ret = -EPERM,
        RTAS_IBM_PLATFORM_DUMP_CONTINUE | RTAS_IBM_PLATFORM_DUMP_COMPLETE => {
            (*params).sequence_hi = rets[0];
            (*params).sequence_lo = rets[1];
            (*params).bytes_ret_hi = rets[2];
            (*params).bytes_ret_lo = rets[3];
        }
        _ => {
            ret = -EIO;
            pr_err_ratelimited!("unexpected ibm,platform-dump status {}\n", fwrc);
        }
    }

    (*params).status = fwrc;
    ret
}

unsafe extern "C" fn papr_platform_dump_handle_read(
    file: *mut File,
    buf: *mut c_char,
    mut size: usize,
    _off: *mut LoffT,
) -> Isize {
    let params = (*file).private_data as *mut IbmPlatformDumpParams;
    let total_bytes: u64;
    let fwrc: i32;

    if (*params).buf_length == 0 {
        pr_warn_once!(
            "Platform dump completed for dump ID {}\n",
            (((*params).dump_tag_hi as u64) << 32) | (*params).dump_tag_lo as u64
        );
        return -EINVAL as Isize;
    }

    if (*params).status == RTAS_IBM_PLATFORM_DUMP_COMPLETE {
        (*params).buf_length = 0;
        return 0;
    }

    if size < SZ_1K as usize {
        pr_err_once!("Buffer length should be minimum 1024 bytes\n");
        return -EINVAL as Isize;
    } else if size > (*params).buf_length as usize {
        size = (*params).buf_length as usize;
    }

    fwrc = rtas_ibm_platform_dump(
        params,
        rtas_work_area_phys((*params).work_area),
        size as u32,
    );
    if fwrc < 0 {
        return fwrc as Isize;
    }

    total_bytes = (((*params).bytes_ret_hi as u64) << 32) | (*params).bytes_ret_lo as u64;
    if WARN!(total_bytes > size as u64, "possible write beyond end of work area") {
        return -EFAULT as Isize;
    }

    if copy_to_user(buf as *mut c_void, rtas_work_area_raw_buf((*params).work_area), total_bytes as usize) != 0 {
        return -EFAULT as Isize;
    }
    total_bytes as Isize
}

unsafe extern "C" fn papr_platform_dump_handle_release(
    _inode: *mut Inode,
    file: *mut File,
) -> i32 {
    let params = (*file).private_data as *mut IbmPlatformDumpParams;
    if !(*params).work_area.is_null() {
        rtas_work_area_free((*params).work_area);
    }
    mutex_lock(&mut PLATFORM_DUMP_LIST_MUTEX);
    list_del(&mut (*params).list);
    mutex_unlock(&mut PLATFORM_DUMP_LIST_MUTEX);
    kfree(params);
    (*file).private_data = core::ptr::null_mut();
    0
}

unsafe extern "C" fn papr_platform_dump_invalidate_ioctl(
    file: *mut File,
    ioctl: u32,
    arg: CULong,
) -> CLong {
    let argp = arg as *mut u64;
    let dump_tag = get_user(argp);
    if dump_tag.is_err() {
        return -EFAULT as CLong;
    }
    if ioctl != PAPR_PLATFORM_DUMP_IOC_INVALIDATE {
        return -ENOIOCTLCMD as CLong;
    }
    if (*file).private_data.is_null() {
        pr_err!("No valid FD to invalidate dump for the ID({})\n", dump_tag.unwrap());
        return -EINVAL as CLong;
    }
    let params = (*file).private_data as *mut IbmPlatformDumpParams;
    let dump_tag = dump_tag.unwrap();
    let param_dump_tag = (((*params).dump_tag_hi as u64) << 32) | (*params).dump_tag_lo as u64;
    if dump_tag != param_dump_tag {
        pr_err!("Invalid dump ID({}) to invalidate dump\n", dump_tag);
        return -EINVAL as CLong;
    }
    if (*params).status != RTAS_IBM_PLATFORM_DUMP_COMPLETE {
        pr_err!("Platform dump is not complete, but requested to invalidate dump for ID({})\n", dump_tag);
        return -EINPROGRESS as CLong;
    }
    rtas_ibm_platform_dump(params, 0, 0) as CLong
}

static PAPr_PLATFORM_DUMP_HANDLE_OPS: FileOperations = FileOperations {
    read: Some(papr_platform_dump_handle_read),
    release: Some(papr_platform_dump_handle_release),
    unlocked_ioctl: Some(papr_platform_dump_invalidate_ioctl),
};

unsafe fn papr_platform_dump_create_handle(dump_tag: u64) -> CLong {
    let mut params: *mut IbmPlatformDumpParams;
    let mut param_dump_tag: u64;
    let mut pos = 0usize;
    while let Some(entry) = list_next_entry::<IbmPlatformDumpParams>(&PLATFORM_DUMP_LIST, &mut pos) {
        params = entry;
        param_dump_tag = (((*params).dump_tag_hi as u64) << 32) | (*params).dump_tag_lo as u64;
        if dump_tag == param_dump_tag {
            pr_err!("Platform dump for ID({}) is already in progress\n", dump_tag);
            return -EALREADY as CLong;
        }
    }
    params = kzalloc::<IbmPlatformDumpParams>(GFP_KERNEL_ACCOUNT);
    if params.is_null() {
        return -ENOMEM as CLong;
    }
    (*params).work_area = rtas_work_area_alloc(SZ_4K);
    (*params).buf_length = SZ_4K;
    (*params).dump_tag_hi = (dump_tag >> 32) as u32;
    (*params).dump_tag_lo = dump_tag as u32;
    (*params).status = RTAS_IBM_PLATFORM_DUMP_START;
    let fd = FD_ADD(
        O_RDONLY | O_CLOEXEC,
        anon_inode_getfile_fmode("[papr-platform-dump]", &PAPr_PLATFORM_DUMP_HANDLE_OPS, params as *mut c_void, O_RDONLY, FMODE_LSEEK | FMODE_PREAD),
    );
    if fd < 0 {
        rtas_work_area_free((*params).work_area);
        kfree(params);
        return fd as CLong;
    }
    list_add(&mut (*params).list, &mut PLATFORM_DUMP_LIST);
    pr_info!("{} ({}) initiated platform dump for dump tag {}\n", current_comm(), current_pid(), dump_tag);
    fd as CLong
}

unsafe extern "C" fn papr_platform_dump_dev_ioctl(
    _filp: *mut File,
    ioctl: u32,
    arg: CULong,
) -> CLong {
    let dump_tag = get_user(arg as *mut u64);
    if dump_tag.is_err() {
        return -EFAULT as CLong;
    }
    let ret = match ioctl {
        PAPR_PLATFORM_DUMP_IOC_CREATE_HANDLE => {
            mutex_lock(&mut PLATFORM_DUMP_LIST_MUTEX);
            let result = papr_platform_dump_create_handle(dump_tag.unwrap());
            mutex_unlock(&mut PLATFORM_DUMP_LIST_MUTEX);
            result
        }
        _ => -ENOIOCTLCMD as CLong,
    };
    ret
}

static PAPr_PLATFORM_DUMP_OPS: FileOperations = FileOperations {
    unlocked_ioctl: Some(papr_platform_dump_dev_ioctl),
};

static mut PAPr_PLATFORM_DUMP_DEV: MiscDevice = MiscDevice {
    minor: MISC_DYNAMIC_MINOR,
    name: "papr-platform-dump",
    fops: &PAPr_PLATFORM_DUMP_OPS,
};

unsafe fn papr_platform_dump_init() -> i32 {
    if !rtas_function_implemented(RTAS_FN_IBM_PLATFORM_DUMP) {
        return -ENODEV;
    }
    misc_register(&mut PAPr_PLATFORM_DUMP_DEV)
}

machine_device_initcall!(pseries, papr_platform_dump_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
