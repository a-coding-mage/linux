// SPDX-License-Identifier: GPL-2.0+
/*
 * Support for dynamic clock devices
 *
 * Copyright (C) 2010 OMICRON electronics GmbH
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn get_posix_clock(fp: *mut file) -> *mut posix_clock {
    let pccontext = (*fp).private_data as *mut posix_clock_context;
    let clk = (*pccontext).clk;

    down_read(&mut (*clk).rwsem);

    if !(*clk).zombie {
        return clk;
    }

    up_read(&mut (*clk).rwsem);
    core::ptr::null_mut()
}

unsafe fn put_posix_clock(clk: *mut posix_clock) {
    up_read(&mut (*clk).rwsem);
}

unsafe extern "C" fn posix_clock_read(
    fp: *mut file,
    buf: *mut core::ffi::c_char,
    count: usize,
    _ppos: *mut loff_t,
) -> isize {
    let pccontext = (*fp).private_data as *mut posix_clock_context;
    let clk = get_posix_clock(fp);
    let mut err: i32 = -EINVAL;

    if clk.is_null() {
        return -ENODEV as isize;
    }

    if let Some(read) = (*clk).ops.read {
        err = read(pccontext, (*fp).f_flags, buf, count);
    }

    put_posix_clock(clk);
    err as isize
}

unsafe extern "C" fn posix_clock_poll(
    fp: *mut file,
    wait: *mut poll_table,
) -> __poll_t {
    let pccontext = (*fp).private_data as *mut posix_clock_context;
    let clk = get_posix_clock(fp);
    let mut result: __poll_t = 0;

    if clk.is_null() {
        return EPOLLERR as __poll_t;
    }

    if let Some(poll) = (*clk).ops.poll {
        result = poll(pccontext, fp, wait);
    }

    put_posix_clock(clk);
    result
}

unsafe extern "C" fn posix_clock_ioctl(
    fp: *mut file,
    cmd: u32,
    arg: libc::c_ulong,
) -> libc::c_long {
    let pccontext = (*fp).private_data as *mut posix_clock_context;
    let clk = get_posix_clock(fp);
    let mut err: i32 = -ENOTTY;

    if clk.is_null() {
        return -ENODEV as libc::c_long;
    }

    if let Some(ioctl) = (*clk).ops.ioctl {
        err = ioctl(pccontext, cmd, arg);
    }

    put_posix_clock(clk);
    err as libc::c_long
}

unsafe extern "C" fn posix_clock_open(inode: *mut inode, fp: *mut file) -> i32 {
    let mut err: i32;
    let clk = container_of((*inode).i_cdev, posix_clock, cdev);
    let pccontext: *mut posix_clock_context;

    down_read(&mut (*clk).rwsem);

    if (*clk).zombie {
        err = -ENODEV;
    } else {
        pccontext = kzalloc_obj::<posix_clock_context>();
        if pccontext.is_null() {
            err = -ENOMEM;
        } else {
            (*pccontext).clk = clk;
            (*pccontext).fp = fp;
            if let Some(open) = (*clk).ops.open {
                err = open(pccontext, (*fp).f_mode);
                if err != 0 {
                    kfree(pccontext);
                } else {
                    (*fp).private_data = pccontext as *mut core::ffi::c_void;
                    get_device((*clk).dev);
                }
            } else {
                (*fp).private_data = pccontext as *mut core::ffi::c_void;
                get_device((*clk).dev);
                err = 0;
            }
        }
    }

    up_read(&mut (*clk).rwsem);
    err
}

unsafe extern "C" fn posix_clock_release(_inode: *mut inode, fp: *mut file) -> i32 {
    let pccontext = (*fp).private_data as *mut posix_clock_context;
    if pccontext.is_null() {
        return -ENODEV;
    }

    let clk = (*pccontext).clk;
    let mut err = 0;
    if let Some(release) = (*clk).ops.release {
        err = release(pccontext);
    }
    put_device((*clk).dev);
    kfree(pccontext);
    (*fp).private_data = core::ptr::null_mut();
    err
}

static POSIX_CLOCK_FILE_OPERATIONS: file_operations = file_operations {
    owner: THIS_MODULE,
    read: Some(posix_clock_read),
    poll: Some(posix_clock_poll),
    unlocked_ioctl: Some(posix_clock_ioctl),
    compat_ioctl: Some(posix_clock_ioctl),
    open: Some(posix_clock_open),
    release: Some(posix_clock_release),
};

#[no_mangle]
pub unsafe extern "C" fn posix_clock_register(
    clk: *mut posix_clock,
    dev: *mut device,
) -> i32 {
    init_rwsem(&mut (*clk).rwsem);
    cdev_init(&mut (*clk).cdev, &POSIX_CLOCK_FILE_OPERATIONS);
    let err = cdev_device_add(&mut (*clk).cdev, dev);
    if err != 0 {
        pr_err("%s unable to add device %d:%d\n", dev_name(dev), MAJOR((*dev).devt), MINOR((*dev).devt));
        return err;
    }
    (*clk).cdev.owner = (*clk).ops.owner;
    (*clk).dev = dev;
    0
}

#[no_mangle]
pub unsafe extern "C" fn posix_clock_unregister(clk: *mut posix_clock) {
    cdev_device_del(&mut (*clk).cdev, (*clk).dev);
    down_write(&mut (*clk).rwsem);
    (*clk).zombie = true;
    up_write(&mut (*clk).rwsem);
    put_device((*clk).dev);
}

#[repr(C)]
pub struct posix_clock_desc {
    fp: *mut file,
    clk: *mut posix_clock,
}

unsafe fn get_clock_desc(id: clockid_t, cd: *mut posix_clock_desc) -> i32 {
    let fp = fget(clockid_to_fd(id));
    let mut err = -EINVAL;
    if fp.is_null() {
        return err;
    }
    if (*fp).f_op != &POSIX_CLOCK_FILE_OPERATIONS || (*fp).private_data.is_null() {
        fput(fp);
        return err;
    }
    (*cd).fp = fp;
    (*cd).clk = get_posix_clock(fp);
    err = if (*cd).clk.is_null() { -ENODEV } else { 0 };
    if err != 0 {
        fput(fp);
    }
    err
}

unsafe fn put_clock_desc(cd: *mut posix_clock_desc) {
    put_posix_clock((*cd).clk);
    fput((*cd).fp);
}

unsafe fn pc_clock_adjtime(id: clockid_t, tx: *mut __kernel_timex) -> i32 {
    let mut cd = core::mem::MaybeUninit::<posix_clock_desc>::uninit();
    let mut err = get_clock_desc(id, cd.as_mut_ptr());
    if err != 0 { return err; }
    let cd = cd.assume_init_mut();
    if (*tx).modes != 0 && ((*cd.fp).f_mode & FMODE_WRITE) == 0 {
        err = -EACCES;
    } else if let Some(adj) = (*cd.clk).ops.clock_adjtime {
        err = adj(cd.clk, tx);
    } else {
        err = -EOPNOTSUPP;
    }
    put_clock_desc(cd);
    err
}

unsafe fn pc_clock_gettime(id: clockid_t, ts: *mut timespec64) -> i32 {
    pc_clock_call_get(id, ts, true)
}

unsafe fn pc_clock_getres(id: clockid_t, ts: *mut timespec64) -> i32 {
    pc_clock_call_get(id, ts, false)
}

unsafe fn pc_clock_call_get(id: clockid_t, ts: *mut timespec64, time: bool) -> i32 {
    let mut cd = core::mem::MaybeUninit::<posix_clock_desc>::uninit();
    let err = get_clock_desc(id, cd.as_mut_ptr());
    if err != 0 { return err; }
    let cd = cd.assume_init_mut();
    let err = if time {
        (*cd.clk).ops.clock_gettime.map(|f| f(cd.clk, ts)).unwrap_or(-EOPNOTSUPP)
    } else {
        (*cd.clk).ops.clock_getres.map(|f| f(cd.clk, ts)).unwrap_or(-EOPNOTSUPP)
    };
    put_clock_desc(cd);
    err
}

unsafe fn pc_clock_settime(id: clockid_t, ts: *const timespec64) -> i32 {
    if !timespec64_valid_strict(ts) { return -EINVAL; }
    let mut cd = core::mem::MaybeUninit::<posix_clock_desc>::uninit();
    let mut err = get_clock_desc(id, cd.as_mut_ptr());
    if err != 0 { return err; }
    let cd = cd.assume_init_mut();
    if ((*cd.fp).f_mode & FMODE_WRITE) == 0 {
        err = -EACCES;
    } else {
        err = (*cd.clk).ops.clock_settime.map(|f| f(cd.clk, ts)).unwrap_or(-EOPNOTSUPP);
    }
    put_clock_desc(cd);
    err
}

pub static mut clock_posix_dynamic: k_clock = k_clock {
    clock_getres: Some(pc_clock_getres),
    clock_set: Some(pc_clock_settime),
    clock_get_timespec: Some(pc_clock_gettime),
    clock_adj: Some(pc_clock_adjtime),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
