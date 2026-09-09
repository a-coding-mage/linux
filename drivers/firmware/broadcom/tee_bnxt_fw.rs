// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Broadcom.
 */

// Linux kernel and Broadcom TEE dependencies supplied externally.

const MAX_SHM_MEM_SZ: usize = 4 * 1024 * 1024;
const MAX_TEE_PARAM_ARRY_MEMB: usize = 4;

#[repr(C)]
enum TaCmd {
    /*
     * TA_CMD_BNXT_FASTBOOT - boot bnxt device by copying f/w into sram
     *
     * param[0] unused
     * param[1] unused
     * param[2] unused
     * param[3] unused
     *
     * Result:
     *  TEE_SUCCESS - Invoke command success
     *  TEE_ERROR_ITEM_NOT_FOUND - Corrupt f/w image found on memory
     */
    TA_CMD_BNXT_FASTBOOT = 0,

    /*
     * TA_CMD_BNXT_COPY_COREDUMP - copy the core dump into shm
     *
     * param[0] (inout memref) - Coredump buffer memory reference
     * param[1] (in value) - value.a: offset, data to be copied from
     *                       value.b: size of data to be copied
     * param[2] unused
     * param[3] unused
     *
     * Result:
     *  TEE_SUCCESS - Invoke command success
     *  TEE_ERROR_BAD_PARAMETERS - Incorrect input param
     *  TEE_ERROR_ITEM_NOT_FOUND - Corrupt core dump
     */
    TA_CMD_BNXT_COPY_COREDUMP = 3,
}

// struct tee_bnxt_fw_private - OP-TEE bnxt private data
// @dev: OP-TEE based bnxt device.
// @ctx: OP-TEE context handler.
// @session_id: TA session identifier.
#[repr(C)]
struct TeeBnxtFwPrivate {
    dev: *mut Device,
    ctx: *mut TeeContext,
    session_id: u32,
    fw_shm_pool: *mut TeeShm,
}

static mut PVT_DATA: TeeBnxtFwPrivate = TeeBnxtFwPrivate {
    dev: core::ptr::null_mut(),
    ctx: core::ptr::null_mut(),
    session_id: 0,
    fw_shm_pool: core::ptr::null_mut(),
};

unsafe fn prepare_args(cmd: i32, arg: *mut TeeIoctlInvokeArg, param: *mut TeeParam) {
    core::ptr::write_bytes(arg, 0, 1);
    core::ptr::write_bytes(param, 0, MAX_TEE_PARAM_ARRY_MEMB);

    (*arg).func = cmd;
    (*arg).session = PVT_DATA.session_id;
    (*arg).num_params = MAX_TEE_PARAM_ARRY_MEMB as u32;

    match cmd {
        TA_CMD_BNXT_COPY_COREDUMP => {
            (*param.add(0)).attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT;
            (*param.add(0)).u.memref.shm = PVT_DATA.fw_shm_pool;
            (*param.add(0)).u.memref.size = MAX_SHM_MEM_SZ;
            (*param.add(0)).u.memref.shm_offs = 0;
            (*param.add(1)).attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
        }
        TA_CMD_BNXT_FASTBOOT => {}
        _ => {}
    }
}

// tee_bnxt_fw_load() - Load the bnxt firmware using an OP-TEE call.
pub unsafe fn tee_bnxt_fw_load() -> i32 {
    let mut ret: i32 = 0;
    let mut arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; MAX_TEE_PARAM_ARRY_MEMB] = core::mem::zeroed();

    if PVT_DATA.ctx.is_null() {
        return -ENODEV;
    }

    prepare_args(TA_CMD_BNXT_FASTBOOT, &mut arg, param.as_mut_ptr());
    ret = tee_client_invoke_func(PVT_DATA.ctx, &mut arg, param.as_mut_ptr());
    if ret < 0 || arg.ret != 0 {
        dev_err(PVT_DATA.dev, "TA_CMD_BNXT_FASTBOOT invoke failed TEE err: %x, ret:%x\n", arg.ret, ret);
        return -EINVAL;
    }
    0
}

// EXPORT_SYMBOL(tee_bnxt_fw_load);

// tee_bnxt_copy_coredump() - Copy coredump from the allocated memory.
pub unsafe fn tee_bnxt_copy_coredump(mut buf: *mut core::ffi::c_void, mut offset: u32, size: u32) -> i32 {
    let mut arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; MAX_TEE_PARAM_ARRY_MEMB] = core::mem::zeroed();
    let mut rbytes = size;
    let mut nbytes: u32;
    let mut ret: i32;

    if PVT_DATA.ctx.is_null() {
        return -ENODEV;
    }
    prepare_args(TA_CMD_BNXT_COPY_COREDUMP, &mut arg, param.as_mut_ptr());
    while rbytes != 0 {
        nbytes = core::cmp::min(rbytes, (*param.as_ptr()).u.memref.size as u32);
        (*param.add(1)).u.value.a = offset;
        (*param.add(1)).u.value.b = nbytes;
        ret = tee_client_invoke_func(PVT_DATA.ctx, &mut arg, param.as_mut_ptr());
        if ret < 0 || arg.ret != 0 {
            dev_err(PVT_DATA.dev, "TA_CMD_BNXT_COPY_COREDUMP invoke failed TEE err: %x, ret:%x\n", arg.ret, ret);
            return -EINVAL;
        }
        let core_data = tee_shm_get_va(PVT_DATA.fw_shm_pool, 0);
        if is_err(core_data) {
            dev_err(PVT_DATA.dev, "tee_shm_get_va failed\n");
            return ptr_err(core_data);
        }
        core::ptr::copy_nonoverlapping(core_data as *const u8, buf as *mut u8, nbytes as usize);
        rbytes -= nbytes;
        buf = (buf as *mut u8).add(nbytes as usize) as *mut core::ffi::c_void;
        offset += nbytes;
    }
    0
}

// EXPORT_SYMBOL(tee_bnxt_copy_coredump);

unsafe fn optee_ctx_match(ver: *mut TeeIoctlVersionData, _data: *const core::ffi::c_void) -> i32 {
    ((*ver).impl_id == TEE_IMPL_ID_OPTEE) as i32
}

// The remaining driver registration declarations mirror the C driver and use
// kernel-provided structure and helper definitions.
unsafe fn tee_bnxt_fw_probe(bnxt_device: *mut TeeClientDevice) -> i32 {
    let dev = &mut (*bnxt_device).dev;
    let mut err = -ENODEV;
    let mut sess_arg: TeeIoctlOpenSessionArg = core::mem::zeroed();

    PVT_DATA.ctx = tee_client_open_context(core::ptr::null_mut(), Some(optee_ctx_match), core::ptr::null(), core::ptr::null());
    if is_err(PVT_DATA.ctx) { return -ENODEV; }

    export_uuid(&mut sess_arg.uuid, &(*bnxt_device).id.uuid);
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_PUBLIC;
    sess_arg.num_params = 0;
    let ret = tee_client_open_session(PVT_DATA.ctx, &mut sess_arg, core::ptr::null_mut());
    if ret < 0 || sess_arg.ret != 0 {
        dev_err(dev, "tee_client_open_session failed, err: %x\n", sess_arg.ret);
        err = -EINVAL;
        tee_client_close_context(PVT_DATA.ctx);
        return err;
    }
    PVT_DATA.session_id = sess_arg.session;
    PVT_DATA.dev = dev;

    let fw_shm_pool = tee_shm_alloc_kernel_buf(PVT_DATA.ctx, MAX_SHM_MEM_SZ);
    if is_err(fw_shm_pool) {
        dev_err(PVT_DATA.dev, "tee_shm_alloc_kernel_buf failed\n");
        err = ptr_err(fw_shm_pool);
        tee_client_close_session(PVT_DATA.ctx, PVT_DATA.session_id);
        tee_client_close_context(PVT_DATA.ctx);
        return err;
    }
    PVT_DATA.fw_shm_pool = fw_shm_pool;
    0
}

unsafe fn tee_bnxt_fw_remove(_bnxt_device: *mut TeeClientDevice) {
    tee_shm_free(PVT_DATA.fw_shm_pool);
    tee_client_close_session(PVT_DATA.ctx, PVT_DATA.session_id);
    tee_client_close_context(PVT_DATA.ctx);
    PVT_DATA.ctx = core::ptr::null_mut();
}

unsafe fn tee_bnxt_fw_shutdown(_bnxt_device: *mut TeeClientDevice) {
    tee_shm_free(PVT_DATA.fw_shm_pool);
    tee_client_close_session(PVT_DATA.ctx, PVT_DATA.session_id);
    tee_client_close_context(PVT_DATA.ctx);
    PVT_DATA.ctx = core::ptr::null_mut();
}

// UUID_INIT(0x6272636D, 0x2019, 0x0716, 0x42, 0x43, 0x4D, 0x5F, 0x53, 0x43, 0x48, 0x49)
static TEE_BNXT_FW_ID_TABLE: [TeeClientDeviceId; 2] = [
    TeeClientDeviceId { uuid: Uuid { b: [0x62, 0x72, 0x63, 0x6d, 0x20, 0x19, 0x07, 0x16, 0x42, 0x43, 0x4d, 0x5f, 0x53, 0x43, 0x48, 0x49] } },
    TeeClientDeviceId { uuid: Uuid { b: [0; 16] } },
];

// MODULE_DEVICE_TABLE(tee, tee_bnxt_fw_id_table);
// module_tee_client_driver(tee_bnxt_fw_driver);
// MODULE_AUTHOR("Vikas Gupta <vikas.gupta@broadcom.com>");
// MODULE_DESCRIPTION("Broadcom bnxt firmware manager");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
