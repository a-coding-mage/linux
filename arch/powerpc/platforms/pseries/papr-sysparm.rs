// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the kernel headers are intentionally left external.

pub unsafe fn papr_sysparm_buf_alloc() -> *mut papr_sysparm_buf {
    kzalloc_obj::<papr_sysparm_buf>()
}

pub unsafe fn papr_sysparm_buf_free(buf: *mut papr_sysparm_buf) {
    kfree(buf);
}

unsafe fn papr_sysparm_buf_get_length(buf: *const papr_sysparm_buf) -> usize {
    be16_to_cpu((*buf).len)
}

unsafe fn papr_sysparm_buf_set_length(buf: *mut papr_sysparm_buf, mut length: usize) {
    WARN_ONCE(length > core::mem::size_of_val(&(*buf).val),
              "bogus length %zu, clamping to safe value", length);
    length = core::cmp::min(core::mem::size_of_val(&(*buf).val), length);
    (*buf).len = cpu_to_be16(length as u16);
}

// For use on buffers returned from ibm,get-system-parameter before returning
// them to callers. Ensures the encoded length of valid data cannot overrun val.
unsafe fn papr_sysparm_buf_clamp_length(buf: *mut papr_sysparm_buf) {
    papr_sysparm_buf_set_length(buf, papr_sysparm_buf_get_length(buf));
}

// Perform basic diligence on the system parameter buffer before submitting it.
unsafe fn papr_sysparm_buf_can_submit(buf: *const papr_sysparm_buf) -> bool {
    papr_sysparm_buf_get_length(buf) <= core::mem::size_of_val(&(*buf).val)
}

pub unsafe fn papr_sysparm_get(param: papr_sysparm_t,
                               buf: *mut papr_sysparm_buf) -> i32 {
    let token: i32 = rtas_function_token(RTAS_FN_IBM_GET_SYSTEM_PARAMETER);
    let work_area: *mut rtas_work_area;
    let mut fwrc: i32;
    let ret: i32;

    might_sleep();
    if WARN_ON(!buf.is_null()) == false { return -EFAULT; }
    if token == RTAS_UNKNOWN_SERVICE { return -ENOENT; }
    if !papr_sysparm_buf_can_submit(buf) { return -EINVAL; }

    work_area = rtas_work_area_alloc(core::mem::size_of::<papr_sysparm_buf>());
    memcpy(rtas_work_area_raw_buf(work_area), buf as *const _, core::mem::size_of::<papr_sysparm_buf>());
    loop {
        fwrc = rtas_call(token, 3, 1, core::ptr::null_mut(), param.token,
                         rtas_work_area_phys(work_area), rtas_work_area_size(work_area));
        if !rtas_busy_delay(fwrc) { break; }
    }
    ret = match fwrc {
        0 => {
            memcpy(buf as *mut _, rtas_work_area_raw_buf(work_area), core::mem::size_of::<papr_sysparm_buf>());
            papr_sysparm_buf_clamp_length(buf); 0
        },
        -3 => -EOPNOTSUPP,
        -9002 => -EPERM,
        -9999 => -EINVAL,
        -1 => -EIO,
        _ => { pr_err!("unexpected ibm,get-system-parameter result %d\n", fwrc); -EIO }
    };
    rtas_work_area_free(work_area);
    ret
}

pub unsafe fn papr_sysparm_set(param: papr_sysparm_t,
                               buf: *const papr_sysparm_buf) -> i32 {
    let token: i32 = rtas_function_token(RTAS_FN_IBM_SET_SYSTEM_PARAMETER);
    let work_area: *mut rtas_work_area;
    let mut fwrc: i32;
    let ret: i32;
    might_sleep();
    if WARN_ON(!buf.is_null()) == false { return -EFAULT; }
    if token == RTAS_UNKNOWN_SERVICE { return -ENOENT; }
    if !papr_sysparm_buf_can_submit(buf) { return -EINVAL; }
    work_area = rtas_work_area_alloc(core::mem::size_of::<papr_sysparm_buf>());
    memcpy(rtas_work_area_raw_buf(work_area), buf, core::mem::size_of::<papr_sysparm_buf>());
    loop {
        fwrc = rtas_call(token, 2, 1, core::ptr::null_mut(), param.token,
                         rtas_work_area_phys(work_area));
        if !rtas_busy_delay(fwrc) { break; }
    }
    ret = match fwrc {
        0 => 0, -3 => -EOPNOTSUPP, -9002 => -EPERM, -9999 => -EINVAL,
        -1 => -EIO,
        _ => { pr_err!("unexpected ibm,set-system-parameter result %d\n", fwrc); -EIO }
    };
    rtas_work_area_free(work_area);
    ret
}

unsafe fn papr_sysparm_buf_from_user(user_iob: *const papr_sysparm_io_block) -> *mut papr_sysparm_buf {
    let mut len: u16 = 0;
    if get_user(&mut len, &(*user_iob).length) != 0 { return ERR_PTR(-EFAULT); }
    static_assert!(core::mem::size_of::<[u8; PAPR_SYSPARM_MAX_INPUT]>() >= PAPR_SYSPARM_MAX_INPUT);
    if len as usize > PAPR_SYSPARM_MAX_INPUT { return ERR_PTR(-EINVAL); }
    let kern_spbuf = papr_sysparm_buf_alloc();
    if kern_spbuf.is_null() { return ERR_PTR(-ENOMEM); }
    papr_sysparm_buf_set_length(kern_spbuf, len as usize);
    if len != 0 && copy_from_user((*kern_spbuf).val.as_mut_ptr(), (*user_iob).data.as_ptr(), len as usize) != 0 {
        papr_sysparm_buf_free(kern_spbuf); return ERR_PTR(-EFAULT);
    }
    kern_spbuf
}

unsafe fn papr_sysparm_buf_to_user(kern_spbuf: *const papr_sysparm_buf,
                                   user_iob: *mut papr_sysparm_io_block) -> i32 {
    let len_out = papr_sysparm_buf_get_length(kern_spbuf) as u16;
    if put_user(len_out, &mut (*user_iob).length) != 0 { return -EFAULT; }
    if copy_to_user((*user_iob).data.as_mut_ptr(), (*kern_spbuf).val.as_ptr(), PAPR_SYSPARM_MAX_OUTPUT) != 0 { return -EFAULT; }
    0
}

unsafe fn papr_sysparm_ioctl_get(user_iob: *mut papr_sysparm_io_block) -> i64 {
    let mut param = papr_sysparm_t { token: 0 };
    if get_user(&mut param.token, &(*user_iob).parameter) != 0 { return -EFAULT as i64; }
    let buf = papr_sysparm_buf_from_user(user_iob);
    if IS_ERR(buf) { return PTR_ERR(buf) as i64; }
    let mut ret = papr_sysparm_get(param, buf) as i64;
    if ret == 0 { ret = papr_sysparm_buf_to_user(buf, user_iob) as i64; }
    papr_sysparm_buf_free(buf); ret
}

unsafe fn papr_sysparm_ioctl_set(user_iob: *mut papr_sysparm_io_block) -> i64 {
    let mut param = papr_sysparm_t { token: 0 };
    if get_user(&mut param.token, &(*user_iob).parameter) != 0 { return -EFAULT as i64; }
    let buf = papr_sysparm_buf_from_user(user_iob);
    if IS_ERR(buf) { return PTR_ERR(buf) as i64; }
    let ret = papr_sysparm_set(param, buf) as i64;
    papr_sysparm_buf_free(buf); ret
}

unsafe fn papr_sysparm_ioctl(filp: *mut file, ioctl: u32, arg: usize) -> i64 {
    let argp = arg as *mut papr_sysparm_io_block;
    match ioctl {
        PAPR_SYSPARM_IOC_GET => papr_sysparm_ioctl_get(argp),
        PAPR_SYSPARM_IOC_SET => if (*filp).f_mode & FMODE_WRITE != 0 { papr_sysparm_ioctl_set(argp) } else { -EBADF as i64 },
        _ => -ENOIOCTLCMD as i64,
    }
}

static papr_sysparm_ops: file_operations = file_operations { unlocked_ioctl: Some(papr_sysparm_ioctl) };
static mut papr_sysparm_dev: miscdevice = miscdevice { minor: MISC_DYNAMIC_MINOR, name: b"papr-sysparm\0".as_ptr(), fops: &papr_sysparm_ops };

unsafe fn papr_sysparm_init() -> i32 {
    if !rtas_function_implemented(RTAS_FN_IBM_GET_SYSTEM_PARAMETER) { return -ENODEV; }
    misc_register(&mut papr_sysparm_dev)
}

machine_device_initcall!(pseries, papr_sysparm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
