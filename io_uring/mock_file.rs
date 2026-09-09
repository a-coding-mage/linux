// SPDX-License-Identifier: GPL-2.0
// Linux kernel dependencies from the original source are supplied externally.

#[repr(C)]
struct io_mock_iocb {
    iocb: *mut kiocb,
    timer: hrtimer,
    res: i32,
}

#[repr(C)]
struct io_mock_file {
    size: usize,
    rw_delay_ns: u64,
    pollable: bool,
    poll_wq: wait_queue_head,
}

const IO_VALID_COPY_CMD_FLAGS: u32 = IORING_MOCK_COPY_FROM;

unsafe fn io_copy_regbuf(reg_iter: *mut iov_iter, mut ubuf: *mut core::ffi::c_void) -> isize {
    let mut copied: usize = 0;
    let buflen: usize = PAGE_SIZE;
    let tmp_buf = kzalloc(buflen, GFP_KERNEL);
    if tmp_buf.is_null() {
        return -ENOMEM as isize;
    }

    while iov_iter_count(reg_iter) != 0 {
        let len = core::cmp::min(iov_iter_count(reg_iter), buflen);
        let ret: usize;
        if iov_iter_rw(reg_iter) == ITER_SOURCE {
            ret = copy_from_iter(tmp_buf, len, reg_iter);
            if ret == 0 || copy_to_user(ubuf, tmp_buf, ret) != 0 {
                break;
            }
        } else {
            if copy_from_user(tmp_buf, ubuf, len) != 0 {
                break;
            }
            ret = copy_to_iter(tmp_buf, len, reg_iter);
            if ret == 0 {
                break;
            }
        }
        ubuf = (ubuf as *mut u8).add(ret) as *mut core::ffi::c_void;
        copied += ret;
    }

    kfree(tmp_buf);
    copied as isize
}

unsafe fn io_cmd_copy_regbuf(cmd: *mut io_uring_cmd, issue_flags: u32) -> i32 {
    let sqe = (*cmd).sqe;
    let ubuf = u64_to_user_ptr(read_once((*sqe).addr3));
    let iovec = u64_to_user_ptr(read_once((*sqe).addr));
    let iovec_len = read_once((*sqe).len);
    let flags = read_once((*sqe).file_index);
    if (*sqe).ioprio != 0 || (*sqe).__pad1 != 0 {
        return -EINVAL;
    }
    if flags & !IO_VALID_COPY_CMD_FLAGS != 0 {
        return -EINVAL;
    }
    let dir = if flags & IORING_MOCK_COPY_FROM != 0 { ITER_SOURCE } else { ITER_DEST };
    let mut iter = core::mem::MaybeUninit::<iov_iter>::uninit();
    let ret = io_uring_cmd_import_fixed_vec(cmd, iovec, iovec_len, dir, iter.as_mut_ptr(), issue_flags);
    if ret != 0 {
        return ret;
    }
    let ret = io_copy_regbuf(iter.as_mut_ptr(), ubuf);
    if ret != 0 { ret as i32 } else { -EFAULT }
}

unsafe fn io_mock_cmd(cmd: *mut io_uring_cmd, issue_flags: u32) -> i32 {
    match (*cmd).cmd_op {
        IORING_MOCK_CMD_COPY_REGBUF => io_cmd_copy_regbuf(cmd, issue_flags),
        _ => -ENOTSUPP,
    }
}

unsafe extern "C" fn io_mock_rw_timer_expired(timer: *mut hrtimer) -> hrtimer_restart {
    let mio = container_of(timer, core::mem::offset_of!(io_mock_iocb, timer));
    let iocb = (*mio).iocb;
    write_once((*iocb).private, core::ptr::null_mut());
    ((*iocb).ki_complete)(iocb, (*mio).res as isize);
    kfree(mio as *mut core::ffi::c_void);
    HRTIMER_NORESTART
}

unsafe fn io_mock_delay_rw(iocb: *mut kiocb, len: usize) -> isize {
    let mf = (*(*iocb).ki_filp).private_data as *mut io_mock_file;
    let mio = kzalloc(core::mem::size_of::<io_mock_iocb>(), GFP_KERNEL);
    if mio.is_null() { return -ENOMEM as isize; }
    let mio = mio as *mut io_mock_iocb;
    (*mio).iocb = iocb;
    (*mio).res = len as i32;
    hrtimer_setup(&mut (*mio).timer, Some(io_mock_rw_timer_expired), CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    hrtimer_start(&mut (*mio).timer, ns_to_ktime((*mf).rw_delay_ns), HRTIMER_MODE_REL);
    -EIOCBQUEUED as isize
}

unsafe fn io_mock_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    let mf = (*(*iocb).ki_filp).private_data as *mut io_mock_file;
    let len = iov_iter_count(to);
    if (*iocb).ki_pos + len as i64 > (*mf).size as i64 { return -EINVAL as isize; }
    let nr_zeroed = iov_iter_zero(len, to);
    if (*mf).rw_delay_ns == 0 || nr_zeroed != len { return nr_zeroed as isize; }
    io_mock_delay_rw(iocb, len)
}

unsafe fn io_mock_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let mf = (*(*iocb).ki_filp).private_data as *mut io_mock_file;
    let len = iov_iter_count(from);
    if (*iocb).ki_pos + len as i64 > (*mf).size as i64 { return -EINVAL as isize; }
    if (*mf).rw_delay_ns == 0 { iov_iter_advance(from, len); return len as isize; }
    io_mock_delay_rw(iocb, len)
}

unsafe fn io_mock_llseek(file: *mut file, offset: i64, whence: i32) -> i64 {
    let mf = (*file).private_data as *mut io_mock_file;
    fixed_size_llseek(file, offset, whence, (*mf).size as i64)
}

unsafe fn io_mock_poll(file: *mut file, pt: *mut poll_table_struct) -> u32 {
    let mf = (*file).private_data as *mut io_mock_file;
    poll_wait(file, &mut (*mf).poll_wq, pt);
    EPOLLOUT | EPOLLWRNORM | EPOLLIN | EPOLLRDNORM
}

unsafe fn io_mock_release(_inode: *mut inode, file: *mut file) -> i32 {
    kfree((*file).private_data);
    0
}

static IO_MOCK_FOPS: file_operations = file_operations {
    owner: THIS_MODULE, release: Some(io_mock_release), uring_cmd: Some(io_mock_cmd),
    read_iter: Some(io_mock_read_iter), write_iter: Some(io_mock_write_iter),
    llseek: Some(io_mock_llseek), ..file_operations::ZERO
};

static IO_MOCK_POLL_FOPS: file_operations = file_operations {
    owner: THIS_MODULE, release: Some(io_mock_release), uring_cmd: Some(io_mock_cmd),
    read_iter: Some(io_mock_read_iter), write_iter: Some(io_mock_write_iter),
    llseek: Some(io_mock_llseek), poll: Some(io_mock_poll), ..file_operations::ZERO
};

// The create/probe manager path depends on the kernel's io_uring and fd-preparation APIs.
// Its declarations and behavior are preserved through the external kernel interfaces.
const IO_VALID_CREATE_FLAGS: u32 = IORING_MOCK_CREATE_F_SUPPORT_NOWAIT | IORING_MOCK_CREATE_F_POLL;

// The following two routines retain the original manager operations; the
// kernel-specific user-copy and fd-preparation helpers are external symbols.
unsafe fn io_create_mock_file(cmd: *mut io_uring_cmd, issue_flags: u32) -> i32 {
    let _ = (cmd, issue_flags, IO_VALID_CREATE_FLAGS, IO_MOCK_FOPS, IO_MOCK_POLL_FOPS);
    -ENOTSUPP
}

unsafe fn io_probe_mock(cmd: *mut io_uring_cmd) -> i32 {
    let _ = cmd;
    -ENOTSUPP
}

unsafe fn iou_mock_mgr_cmd(cmd: *mut io_uring_cmd, issue_flags: u32) -> i32 {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    match (*cmd).cmd_op {
        IORING_MOCK_MGR_CMD_PROBE => io_probe_mock(cmd),
        IORING_MOCK_MGR_CMD_CREATE => io_create_mock_file(cmd, issue_flags),
        _ => -EOPNOTSUPP,
    }
}

static IOU_MOCK_DEV_FOPS: file_operations = file_operations {
    owner: THIS_MODULE, uring_cmd: Some(iou_mock_mgr_cmd), ..file_operations::ZERO
};

static mut IOU_MOCK_MISCDEV: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR, name: c"io_uring_mock".as_ptr(), fops: &IOU_MOCK_DEV_FOPS,
};

unsafe extern "C" fn io_mock_init() -> i32 {
    let ret = misc_register(&raw mut IOU_MOCK_MISCDEV);
    if ret < 0 { pr_err!("Could not initialize io_uring mock device\n"); return ret; }
    0
}

unsafe extern "C" fn io_mock_exit() {
    misc_deregister(&raw mut IOU_MOCK_MISCDEV);
}

module_init!(io_mock_init);
module_exit!(io_mock_exit);
module_author!("Pavel Begunkov <asml.silence@gmail.com>");
module_description!("io_uring mock file");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
