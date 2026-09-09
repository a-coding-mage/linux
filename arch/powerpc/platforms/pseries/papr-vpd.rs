// SPDX-License-Identifier: GPL-2.0-only

// C dependencies supplied by the surrounding kernel translation unit are intentionally
// referenced here without reimplementation.

/**
 * struct rtas_ibm_get_vpd_params - Parameters (in and out) for ibm,get-vpd.
 * @loc_code:  In: Caller-provided location code buffer. Must be RTAS-addressable.
 * @work_area: In: Caller-provided work area buffer for results.
 * @sequence:  In: Sequence number. Out: Next sequence number.
 * @written:   Out: Bytes written by ibm,get-vpd to @work_area.
 * @status:    Out: RTAS call status.
 */
#[repr(C)]
struct rtas_ibm_get_vpd_params {
    loc_code: *const papr_location_code,
    work_area: *mut rtas_work_area,
    sequence: u32,
    written: u32,
    status: i32,
}

/**
 * rtas_ibm_get_vpd() - Call ibm,get-vpd to fill a work area buffer.
 * @params: See &struct rtas_ibm_get_vpd_params.
 *
 * Calls ibm,get-vpd until it errors or successfully deposits data
 * into the supplied work area. Handles RTAS retry statuses. Maps RTAS
 * error statuses to reasonable errno values.
 */
unsafe fn rtas_ibm_get_vpd(params: *mut rtas_ibm_get_vpd_params) -> i32 {
    let loc_code = (*params).loc_code;
    let work_area = (*params).work_area;
    let mut rets: [u32; 2] = [0; 2];
    let fwrc: i32;
    let ret: i32;

    lockdep_assert_held(&rtas_ibm_get_vpd_lock);

    loop {
        fwrc = rtas_call(
            rtas_function_token(RTAS_FN_IBM_GET_VPD),
            4,
            3,
            rets.as_mut_ptr(),
            __pa(loc_code),
            rtas_work_area_phys(work_area),
            rtas_work_area_size(work_area),
            (*params).sequence,
        );
        if !rtas_busy_delay(fwrc) {
            break;
        }
    }

    ret = match fwrc {
        RTAS_HARDWARE_ERROR => -EIO,
        RTAS_INVALID_PARAMETER => -EINVAL,
        RTAS_SEQ_START_OVER => {
            pr_info_ratelimited!("VPD changed during retrieval, retrying\n");
            -EAGAIN
        }
        RTAS_SEQ_MORE_DATA => {
            (*params).sequence = rets[0];
            (*params).written = rets[1];
            if WARN(
                (*params).written > rtas_work_area_size(work_area),
                "possible write beyond end of work area",
            ) {
                -EFAULT
            } else {
                0
            }
        }
        RTAS_SEQ_COMPLETE => {
            (*params).written = rets[1];
            if WARN(
                (*params).written > rtas_work_area_size(work_area),
                "possible write beyond end of work area",
            ) {
                -EFAULT
            } else {
                0
            }
        }
        _ => {
            pr_err_ratelimited!("unexpected ibm,get-vpd status %d\n", fwrc);
            -EIO
        }
    };

    (*params).status = fwrc;
    ret
}

/* Internal VPD sequence APIs. */
unsafe fn vpd_sequence_begin(seq: *mut papr_rtas_sequence) {
    let mut vpd_params: *mut rtas_ibm_get_vpd_params;
    static mut STATIC_LOC_CODE: papr_location_code = papr_location_code::default();

    vpd_params = (*seq).params as *mut rtas_ibm_get_vpd_params;
    mutex_lock(&rtas_ibm_get_vpd_lock);
    STATIC_LOC_CODE = *(*vpd_params).loc_code;
    vpd_params = (*seq).params as *mut rtas_ibm_get_vpd_params;
    (*vpd_params).work_area = rtas_work_area_alloc(SZ_4K);
    (*vpd_params).loc_code = &STATIC_LOC_CODE;
    (*vpd_params).sequence = 1;
    (*vpd_params).status = 0;
}

unsafe fn vpd_sequence_end(seq: *mut papr_rtas_sequence) {
    let vpd_params = (*seq).params as *mut rtas_ibm_get_vpd_params;
    rtas_work_area_free((*vpd_params).work_area);
    mutex_unlock(&rtas_ibm_get_vpd_lock);
}

unsafe fn vpd_sequence_fill_work_area(
    seq: *mut papr_rtas_sequence,
    len: *mut usize,
) -> *const core::ffi::c_char {
    let p = (*seq).params as *mut rtas_ibm_get_vpd_params;
    let init_state = (*p).written == 0;

    if papr_rtas_sequence_should_stop(seq, (*p).status, init_state) {
        return core::ptr::null();
    }
    if papr_rtas_sequence_set_err(seq, rtas_ibm_get_vpd(p)) {
        return core::ptr::null();
    }
    *len = (*p).written as usize;
    rtas_work_area_raw_buf((*p).work_area)
}

static papr_vpd_handle_ops: file_operations = file_operations {
    read: Some(papr_rtas_common_handle_read),
    llseek: Some(papr_rtas_common_handle_seek),
    release: Some(papr_rtas_common_handle_release),
    ..file_operations::default()
};

unsafe fn papr_vpd_create_handle(ulc: *mut papr_location_code) -> isize {
    let mut vpd_params: rtas_ibm_get_vpd_params = core::mem::zeroed();
    let mut seq: papr_rtas_sequence = core::mem::zeroed();
    let mut klc: papr_location_code = core::mem::zeroed();

    if copy_from_user(
        &mut klc as *mut _ as *mut core::ffi::c_void,
        ulc as *const core::ffi::c_void,
        core::mem::size_of::<papr_location_code>(),
    ) != 0 {
        return -EFAULT as isize;
    }
    if !string_is_terminated(klc.str_.as_ptr(), klc.str_.len()) {
        return -EINVAL as isize;
    }

    seq.begin = Some(vpd_sequence_begin);
    seq.end = Some(vpd_sequence_end);
    seq.work = Some(vpd_sequence_fill_work_area);
    vpd_params.loc_code = &klc;
    seq.params = &mut vpd_params as *mut _ as *mut core::ffi::c_void;
    papr_rtas_setup_file_interface(&mut seq, &papr_vpd_handle_ops, "[papr-vpd]")
}

unsafe fn papr_vpd_dev_ioctl(
    _filp: *mut file,
    ioctl: u32,
    arg: usize,
) -> isize {
    match ioctl {
        PAPR_VPD_IOC_CREATE_HANDLE => papr_vpd_create_handle(arg as *mut papr_location_code),
        _ => -ENOIOCTLCMD as isize,
    }
}

static papr_vpd_ops: file_operations = file_operations {
    unlocked_ioctl: Some(papr_vpd_dev_ioctl),
    ..file_operations::default()
};

static mut papr_vpd_dev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: "papr-vpd",
    fops: &papr_vpd_ops,
    ..miscdevice::default()
};

unsafe fn papr_vpd_init() -> i32 {
    if !rtas_function_implemented(RTAS_FN_IBM_GET_VPD) {
        return -ENODEV;
    }
    misc_register(&mut papr_vpd_dev)
}

machine_device_initcall!(pseries, papr_vpd_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
