// SPDX-License-Identifier: GPL-2.0-only

// C dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
struct RtasPhyAttestParams {
    cmd: papr_phy_attest_io_block,
    work_area: *mut rtas_work_area,
    cmd_len: u32,
    sequence: u32,
    written: u32,
    status: i32,
}

unsafe fn rtas_physical_attestation(params: *mut RtasPhyAttestParams) -> i32 {
    let work_area: *mut rtas_work_area = (*params).work_area;
    let token: i32 = rtas_function_token(RTAS_FN_IBM_PHYSICAL_ATTESTATION);
    if token == RTAS_UNKNOWN_SERVICE {
        return -ENOENT;
    }

    lockdep_assert_held(&rtas_ibm_physical_attestation_lock);

    let mut fwrc: i32;
    let mut rets = [0u32; 2];
    let ret: i32;
    loop {
        fwrc = rtas_call(
            token,
            3,
            3,
            rets.as_mut_ptr(),
            rtas_work_area_phys(work_area),
            (*params).cmd_len,
            (*params).sequence,
        );
        if !rtas_busy_delay(fwrc) {
            break;
        }
    }

    match fwrc {
        RTAS_HARDWARE_ERROR => ret = -EIO,
        RTAS_INVALID_PARAMETER => ret = -EINVAL,
        RTAS_SEQ_MORE_DATA => {
            (*params).sequence = rets[0];
            (*params).written = rets[1];
            if WARN(
                (*params).written > rtas_work_area_size(work_area),
                "possible write beyond end of work area",
            ) {
                ret = -EFAULT;
            } else {
                ret = 0;
            }
        }
        RTAS_SEQ_COMPLETE => {
            (*params).written = rets[1];
            // Kernel or firmware bug, do not continue.
            if WARN(
                (*params).written > rtas_work_area_size(work_area),
                "possible write beyond end of work area",
            ) {
                ret = -EFAULT;
            } else {
                ret = 0;
            }
        }
        _ => {
            ret = -EIO;
            pr_err_ratelimited!("unexpected ibm,get-phy_attest status {}\n", fwrc);
        }
    }

    (*params).status = fwrc;
    ret
}

unsafe fn phy_attest_sequence_begin(seq: *mut papr_rtas_sequence) {
    // Allocate the work area under the function lock to avoid exhausting the
    // limited work area pool with concurrent requests.
    mutex_lock(&rtas_ibm_physical_attestation_lock);
    let param = (*seq).params as *mut RtasPhyAttestParams;
    (*param).work_area = rtas_work_area_alloc(SZ_4K);
    memcpy(
        rtas_work_area_raw_buf((*param).work_area),
        &(*param).cmd as *const papr_phy_attest_io_block as *const _,
        (*param).cmd_len as usize,
    );
    (*param).sequence = 1;
    (*param).status = 0;
}

unsafe fn phy_attest_sequence_end(seq: *mut papr_rtas_sequence) {
    let param = (*seq).params as *mut RtasPhyAttestParams;
    rtas_work_area_free((*param).work_area);
    mutex_unlock(&rtas_ibm_physical_attestation_lock);
    kfree(param as *mut _);
}

unsafe fn phy_attest_sequence_fill_work_area(
    seq: *mut papr_rtas_sequence,
    len: *mut usize,
) -> *const u8 {
    let p = (*seq).params as *mut RtasPhyAttestParams;
    let init_state = (*p).written == 0;

    if papr_rtas_sequence_should_stop(seq, (*p).status, init_state) {
        return core::ptr::null();
    }
    if papr_rtas_sequence_set_err(seq, rtas_physical_attestation(p)) {
        return core::ptr::null();
    }
    *len = (*p).written as usize;
    rtas_work_area_raw_buf((*p).work_area) as *const u8
}

#[repr(C)]
struct FileOperations {
    read: Option<unsafe extern "C" fn()>,
    llseek: Option<unsafe extern "C" fn()>,
    release: Option<unsafe extern "C" fn()>,
}

static papr_phy_attest_handle_ops: FileOperations = FileOperations {
    read: Some(papr_rtas_common_handle_read),
    llseek: Some(papr_rtas_common_handle_seek),
    release: Some(papr_rtas_common_handle_release),
};

unsafe fn papr_phy_attest_create_handle(ulc: *mut papr_phy_attest_io_block) -> i64 {
    let params = kzalloc::<RtasPhyAttestParams>(GFP_KERNEL_ACCOUNT);
    if params.is_null() {
        return -ENOMEM as i64;
    }

    if copy_from_user(
        &mut (*params).cmd as *mut papr_phy_attest_io_block,
        ulc,
        core::mem::size_of::<papr_phy_attest_io_block>(),
    ) != 0 {
        kfree(params as *mut _);
        return -EFAULT as i64;
    }

    (*params).cmd_len = be32_to_cpu((*params).cmd.length);
    if (*params).cmd_len == 0
        || (*params).cmd_len as usize > core::mem::size_of::<papr_phy_attest_io_block>()
    {
        kfree(params as *mut _);
        return -EINVAL as i64;
    }

    let mut seq = papr_rtas_sequence::default();
    seq.begin = Some(phy_attest_sequence_begin);
    seq.end = Some(phy_attest_sequence_end);
    seq.work = Some(phy_attest_sequence_fill_work_area);
    seq.params = params as *mut core::ffi::c_void;

    let fd = papr_rtas_setup_file_interface(
        &mut seq,
        &papr_phy_attest_handle_ops,
        "[papr-physical-attestation]",
    );
    if fd < 0 {
        kfree(params as *mut _);
    }
    fd
}

unsafe fn papr_phy_attest_dev_ioctl(
    _filp: *mut file,
    ioctl: u32,
    arg: usize,
) -> i64 {
    let argp = arg as *mut papr_phy_attest_io_block;
    match ioctl {
        PAPR_PHY_ATTEST_IOC_HANDLE => papr_phy_attest_create_handle(argp),
        _ => -ENOIOCTLCMD as i64,
    }
}

static papr_phy_attest_ops: FileOperations = FileOperations {
    unlocked_ioctl: Some(papr_phy_attest_dev_ioctl),
};

static mut papr_phy_attest_dev: miscdevice = miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: "papr-physical-attestation\0",
    fops: &papr_phy_attest_ops,
};

unsafe fn papr_phy_attest_init() -> i32 {
    if !rtas_function_implemented(RTAS_FN_IBM_PHYSICAL_ATTESTATION) {
        return -ENODEV;
    }
    misc_register(&mut papr_phy_attest_dev)
}

// machine_device_initcall(pseries, papr_phy_attest_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
