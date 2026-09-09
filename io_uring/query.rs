// SPDX-License-Identifier: GPL-2.0

// Declarations and constants are supplied by the corresponding kernel headers.

#[repr(C)]
pub union io_query_data {
    pub opcodes: io_uring_query_opcode,
    pub zcrx: io_uring_query_zcrx,
    pub zcrx_notif: io_uring_query_zcrx_event,
    pub scq: io_uring_query_scq,
}

pub const IO_MAX_QUERY_SIZE: usize = core::mem::size_of::<io_query_data>();
pub const IO_MAX_QUERY_ENTRIES: i32 = 1000;

unsafe fn io_query_ops(data: *mut io_query_data) -> isize {
    let e = &mut (*data).opcodes;

    e.nr_request_opcodes = IORING_OP_LAST;
    e.nr_register_opcodes = IORING_REGISTER_LAST;
    e.feature_flags = IORING_FEAT_FLAGS;
    e.ring_setup_flags = IORING_SETUP_FLAGS;
    e.enter_flags = IORING_ENTER_FLAGS;
    e.sqe_flags = SQE_VALID_FLAGS;
    e.nr_query_opcodes = __IO_URING_QUERY_MAX;
    e.__pad = 0;
    core::mem::size_of::<io_uring_query_opcode>() as isize
}

unsafe fn io_query_zcrx(data: *mut io_query_data) -> isize {
    let e = &mut (*data).zcrx;

    e.register_flags = ZCRX_SUPPORTED_REG_FLAGS;
    e.area_flags = IORING_ZCRX_AREA_DMABUF;
    e.nr_ctrl_opcodes = __ZCRX_CTRL_LAST;
    e.rq_hdr_size = core::mem::size_of::<zcrx_rq_hdr>();
    e.rq_hdr_alignment = L1_CACHE_BYTES;
    e.features = ZCRX_FEATURES;
    e.__resv2 = 0;
    core::mem::size_of::<io_uring_query_zcrx>() as isize
}

unsafe fn io_query_zcrx_notif(data: *mut io_query_data) -> isize {
    let e = &mut (*data).zcrx_notif;

    e.event_flags = ZCRX_EVENT_TYPE_MASK;
    e.stats_size = core::mem::size_of::<zcrx_stats>();
    e.stats_off_alignment = core::mem::align_of::<zcrx_stats>();
    e.__resv1 = 0;
    memset(&mut e.__resv2 as *mut _, 0, core::mem::size_of_val(&e.__resv2));
    core::mem::size_of::<io_uring_query_zcrx_event>() as isize
}

unsafe fn io_query_scq(data: *mut io_query_data) -> isize {
    let e = &mut (*data).scq;

    e.hdr_size = core::mem::size_of::<io_rings>();
    e.hdr_alignment = SMP_CACHE_BYTES;
    core::mem::size_of::<io_uring_query_scq>() as isize
}

unsafe fn io_handle_query_entry(
    data: *mut io_query_data,
    uhdr: *mut core::ffi::c_void,
    next_entry: *mut u64,
) -> i32 {
    let mut hdr: io_uring_query_hdr = core::mem::zeroed();
    let mut usize_: usize;
    let mut res_size: usize = 0;
    let mut ret: isize = -EINVAL as isize;
    let udata: *mut core::ffi::c_void;

    if copy_from_user(&mut hdr as *mut _, uhdr, core::mem::size_of::<io_uring_query_hdr>()) != 0 {
        return -EFAULT;
    }
    if hdr.size > PAGE_SIZE {
        return -E2BIG;
    }
    usize_ = hdr.size;
    hdr.size = core::cmp::min(hdr.size, IO_MAX_QUERY_SIZE);
    udata = u64_to_user_ptr(hdr.query_data);

    if hdr.query_op >= __IO_URING_QUERY_MAX {
        ret = -EOPNOTSUPP as isize;
    } else if !mem_is_zero(hdr.__resv.as_ptr() as *const _, core::mem::size_of_val(&hdr.__resv))
        || hdr.result != 0 || hdr.size == 0
    {
    } else {
        if copy_from_user(data as *mut _, udata, hdr.size) != 0 {
            return -EFAULT;
        }
        ret = match hdr.query_op {
            IO_URING_QUERY_OPCODES => io_query_ops(data),
            IO_URING_QUERY_ZCRX => io_query_zcrx(data),
            IO_URING_QUERY_ZCRX_EVENT => io_query_zcrx_notif(data),
            IO_URING_QUERY_SCQ => io_query_scq(data),
            _ => ret,
        };
    }

    if ret >= 0 {
        if WARN_ON_ONCE(ret as usize > IO_MAX_QUERY_SIZE) {
            return -EFAULT;
        }
        res_size = ret as usize;
        ret = 0;
    }
    hdr.result = ret;
    hdr.size = core::cmp::min(usize_, res_size);

    if copy_struct_to_user(udata, usize_, data, hdr.size, core::ptr::null_mut()) != 0 {
        return -EFAULT;
    }
    if copy_to_user(uhdr, &hdr as *const _, core::mem::size_of::<io_uring_query_hdr>()) != 0 {
        return -EFAULT;
    }
    *next_entry = hdr.next_entry;
    0
}

pub unsafe fn io_query(arg: *mut core::ffi::c_void, nr_args: u32) -> i32 {
    let mut entry_buffer: io_query_data = core::mem::zeroed();
    let mut uhdr = arg;
    let mut nr: i32 = 0;

    if nr_args != 0 {
        return -EINVAL;
    }
    while !uhdr.is_null() {
        let mut next_hdr: u64 = 0;
        let ret = io_handle_query_entry(&mut entry_buffer, uhdr, &mut next_hdr);
        if ret != 0 {
            return ret;
        }
        uhdr = u64_to_user_ptr(next_hdr);
        nr += 1;
        if nr >= IO_MAX_QUERY_ENTRIES {
            return -ERANGE;
        }
        if fatal_signal_pending(current) {
            return -EINTR;
        }
        cond_resched();
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
