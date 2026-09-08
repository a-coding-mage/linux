// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019-2021 Linaro Ltd.
 *
 * Author:
 * Sumit Garg <sumit.garg@linaro.org>
 */

// C dependencies translated as external Rust dependencies:
// linux/err.h, linux/key-type.h, linux/module.h, linux/slab.h,
// linux/string.h, linux/tee_drv.h, linux/uuid.h, keys/trusted_tee.h.

const DRIVER_NAME: &[u8] = b"trusted-key-tee\0";

/*
 * Get random data for symmetric key
 *
 * [out]     memref[0]        Random data
 */
const TA_CMD_GET_RANDOM: u32 = 0x0;

/*
 * Seal trusted key using hardware unique key
 *
 * [in]      memref[0]        Plain key
 * [out]     memref[1]        Sealed key datablob
 */
const TA_CMD_SEAL: u32 = 0x1;

/*
 * Unseal trusted key using hardware unique key
 *
 * [in]      memref[0]        Sealed key datablob
 * [out]     memref[1]        Plain key
 */
const TA_CMD_UNSEAL: u32 = 0x2;

/**
 * struct trusted_key_tee_private - TEE Trusted key private data
 * @dev:		TEE based Trusted key device.
 * @ctx:		TEE context handler.
 * @session_id:		Trusted key TA session identifier.
 * @shm_pool:		Memory pool shared with TEE device.
 */
#[repr(C)]
struct trusted_key_tee_private {
    dev: *mut device,
    ctx: *mut tee_context,
    session_id: u32,
    shm_pool: *mut tee_shm,
}

static mut pvt_data: trusted_key_tee_private = trusted_key_tee_private {
    dev: core::ptr::null_mut(),
    ctx: core::ptr::null_mut(),
    session_id: 0,
    shm_pool: core::ptr::null_mut(),
};

unsafe extern "C" {
    static mut key_type_trusted: key_type;

    fn memset(s: *mut core::ffi::c_void, c: core::ffi::c_int, n: usize) -> *mut core::ffi::c_void;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;

    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_long;

    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);

    fn tee_shm_register_kernel_buf(
        ctx: *mut tee_context,
        addr: *mut core::ffi::c_void,
        length: usize,
    ) -> *mut tee_shm;
    fn tee_shm_free(shm: *mut tee_shm);
    fn tee_client_invoke_func(
        ctx: *mut tee_context,
        arg: *mut tee_ioctl_invoke_arg,
        param: *mut tee_param,
    ) -> core::ffi::c_int;
    fn tee_client_open_context(
        start: *mut tee_context,
        match_fn: Option<
            unsafe extern "C" fn(*mut tee_ioctl_version_data, *const core::ffi::c_void) -> core::ffi::c_int,
        >,
        data: *const core::ffi::c_void,
        vers: *mut tee_ioctl_version_data,
    ) -> *mut tee_context;
    fn tee_client_open_session(
        ctx: *mut tee_context,
        arg: *mut tee_ioctl_open_session_arg,
        param: *mut tee_param,
    ) -> core::ffi::c_int;
    fn tee_client_close_session(ctx: *mut tee_context, session: u32);
    fn tee_client_close_context(ctx: *mut tee_context);
    fn tee_client_driver_register(drv: *mut tee_client_driver) -> core::ffi::c_int;
    fn tee_client_driver_unregister(drv: *mut tee_client_driver);

    fn register_key_type(ktype: *mut key_type) -> core::ffi::c_int;
    fn unregister_key_type(ktype: *mut key_type);
}

/*
 * Have the TEE seal(encrypt) the symmetric key
 */
unsafe extern "C" fn trusted_tee_seal(
    p: *mut trusted_key_payload,
    _datablob: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;
    let mut inv_arg: tee_ioctl_invoke_arg = core::mem::zeroed();
    let mut param: [tee_param; 4] = core::mem::zeroed();
    let mut reg_shm: *mut tee_shm = core::ptr::null_mut();

    memset(
        &mut inv_arg as *mut _ as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&inv_arg),
    );
    memset(
        param.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&param),
    );

    reg_shm = tee_shm_register_kernel_buf(
        pvt_data.ctx,
        (*p).key.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&(*p).key) + core::mem::size_of_val(&(*p).blob),
    );
    if IS_ERR(reg_shm as *const core::ffi::c_void) {
        dev_err(pvt_data.dev, c"shm register failed\n".as_ptr());
        return PTR_ERR(reg_shm as *const core::ffi::c_void) as core::ffi::c_int;
    }

    inv_arg.func = TA_CMD_SEAL;
    inv_arg.session = pvt_data.session_id;
    inv_arg.num_params = 4;

    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT;
    param[0].u.memref.shm = reg_shm;
    param[0].u.memref.size = (*p).key_len;
    param[0].u.memref.shm_offs = 0;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT;
    param[1].u.memref.shm = reg_shm;
    param[1].u.memref.size = core::mem::size_of_val(&(*p).blob);
    param[1].u.memref.shm_offs = core::mem::size_of_val(&(*p).key);

    ret = tee_client_invoke_func(pvt_data.ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 {
        dev_err(
            pvt_data.dev,
            c"TA_CMD_SEAL invoke err: %x\n".as_ptr(),
            inv_arg.ret,
        );
        ret = -EFAULT;
    } else {
        (*p).blob_len = param[1].u.memref.size;
    }

    tee_shm_free(reg_shm);

    ret
}

/*
 * Have the TEE unseal(decrypt) the symmetric key
 */
unsafe extern "C" fn trusted_tee_unseal(
    p: *mut trusted_key_payload,
    _datablob: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;
    let mut inv_arg: tee_ioctl_invoke_arg = core::mem::zeroed();
    let mut param: [tee_param; 4] = core::mem::zeroed();
    let mut reg_shm: *mut tee_shm = core::ptr::null_mut();

    memset(
        &mut inv_arg as *mut _ as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&inv_arg),
    );
    memset(
        param.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&param),
    );

    reg_shm = tee_shm_register_kernel_buf(
        pvt_data.ctx,
        (*p).key.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&(*p).key) + core::mem::size_of_val(&(*p).blob),
    );
    if IS_ERR(reg_shm as *const core::ffi::c_void) {
        dev_err(pvt_data.dev, c"shm register failed\n".as_ptr());
        return PTR_ERR(reg_shm as *const core::ffi::c_void) as core::ffi::c_int;
    }

    inv_arg.func = TA_CMD_UNSEAL;
    inv_arg.session = pvt_data.session_id;
    inv_arg.num_params = 4;

    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT;
    param[0].u.memref.shm = reg_shm;
    param[0].u.memref.size = (*p).blob_len;
    param[0].u.memref.shm_offs = core::mem::size_of_val(&(*p).key);
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT;
    param[1].u.memref.shm = reg_shm;
    param[1].u.memref.size = core::mem::size_of_val(&(*p).key);
    param[1].u.memref.shm_offs = 0;

    ret = tee_client_invoke_func(pvt_data.ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 {
        dev_err(
            pvt_data.dev,
            c"TA_CMD_UNSEAL invoke err: %x\n".as_ptr(),
            inv_arg.ret,
        );
        ret = -EFAULT;
    } else {
        (*p).key_len = param[1].u.memref.size;
    }

    tee_shm_free(reg_shm);

    ret
}

/*
 * Have the TEE generate random symmetric key
 */
unsafe extern "C" fn trusted_tee_get_random(
    key: *mut core::ffi::c_uchar,
    key_len: usize,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;
    let mut inv_arg: tee_ioctl_invoke_arg = core::mem::zeroed();
    let mut param: [tee_param; 4] = core::mem::zeroed();
    let mut reg_shm: *mut tee_shm = core::ptr::null_mut();

    memset(
        &mut inv_arg as *mut _ as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&inv_arg),
    );
    memset(
        param.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&param),
    );

    reg_shm = tee_shm_register_kernel_buf(pvt_data.ctx, key as *mut core::ffi::c_void, key_len);
    if IS_ERR(reg_shm as *const core::ffi::c_void) {
        dev_err(pvt_data.dev, c"key shm register failed\n".as_ptr());
        return PTR_ERR(reg_shm as *const core::ffi::c_void) as core::ffi::c_int;
    }

    inv_arg.func = TA_CMD_GET_RANDOM;
    inv_arg.session = pvt_data.session_id;
    inv_arg.num_params = 4;

    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT;
    param[0].u.memref.shm = reg_shm;
    param[0].u.memref.size = key_len;
    param[0].u.memref.shm_offs = 0;

    ret = tee_client_invoke_func(pvt_data.ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 {
        dev_err(
            pvt_data.dev,
            c"TA_CMD_GET_RANDOM invoke err: %x\n".as_ptr(),
            inv_arg.ret,
        );
        ret = -EFAULT;
    } else {
        ret = param[0].u.memref.size as core::ffi::c_int;
    }

    tee_shm_free(reg_shm);

    ret
}

unsafe extern "C" fn optee_ctx_match(
    ver: *mut tee_ioctl_version_data,
    _data: *const core::ffi::c_void,
) -> core::ffi::c_int {
    if (*ver).impl_id == TEE_IMPL_ID_OPTEE && ((*ver).gen_caps & TEE_GEN_CAP_REG_MEM) != 0 {
        1
    } else {
        0
    }
}

unsafe extern "C" fn trusted_key_probe(rng_device: *mut tee_client_device) -> core::ffi::c_int {
    let dev: *mut device = &mut (*rng_device).dev;
    let mut ret: core::ffi::c_int;
    let mut sess_arg: tee_ioctl_open_session_arg = core::mem::zeroed();

    memset(
        &mut sess_arg as *mut _ as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&sess_arg),
    );

    pvt_data.ctx = tee_client_open_context(
        core::ptr::null_mut(),
        Some(optee_ctx_match),
        core::ptr::null(),
        core::ptr::null_mut(),
    );
    if IS_ERR(pvt_data.ctx as *const core::ffi::c_void) {
        return -ENODEV;
    }

    memcpy(
        sess_arg.uuid.as_mut_ptr() as *mut core::ffi::c_void,
        (*rng_device).id.uuid.b.as_ptr() as *const core::ffi::c_void,
        TEE_IOCTL_UUID_LEN,
    );
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_REE_KERNEL;
    sess_arg.num_params = 0;

    ret = tee_client_open_session(pvt_data.ctx, &mut sess_arg, core::ptr::null_mut());
    if ret < 0 || sess_arg.ret != 0 {
        dev_err(
            dev,
            c"tee_client_open_session failed, err: %x\n".as_ptr(),
            sess_arg.ret,
        );
        ret = -EINVAL;
        tee_client_close_context(pvt_data.ctx);
        return ret;
    }
    pvt_data.session_id = sess_arg.session;

    ret = register_key_type(&mut key_type_trusted);
    if ret < 0 {
        tee_client_close_session(pvt_data.ctx, pvt_data.session_id);
        tee_client_close_context(pvt_data.ctx);
        return ret;
    }

    pvt_data.dev = dev;

    0
}

unsafe extern "C" fn trusted_key_remove(_dev: *mut tee_client_device) {
    unregister_key_type(&mut key_type_trusted);
    tee_client_close_session(pvt_data.ctx, pvt_data.session_id);
    tee_client_close_context(pvt_data.ctx);
}

static trusted_key_id_table: [tee_client_device_id; 2] = [
    tee_client_device_id {
        uuid: uuid_t {
            b: [
                0xf0, 0x4a, 0x0f, 0xe7, 0x1f, 0x5d, 0x4b, 0x9b, 0xab, 0xf7, 0x61, 0x9b, 0x85,
                0xb4, 0xce, 0x8c,
            ],
        },
    },
    tee_client_device_id {
        uuid: uuid_t { b: [0; 16] },
    },
];
// MODULE_DEVICE_TABLE(tee, trusted_key_id_table);

static mut trusted_key_driver: tee_client_driver = tee_client_driver {
    probe: Some(trusted_key_probe),
    remove: Some(trusted_key_remove),
    id_table: trusted_key_id_table.as_ptr(),
    driver: device_driver {
        name: DRIVER_NAME.as_ptr() as *const core::ffi::c_char,
    },
};

unsafe extern "C" fn trusted_tee_init() -> core::ffi::c_int {
    tee_client_driver_register(&mut trusted_key_driver)
}

unsafe extern "C" fn trusted_tee_exit() {
    tee_client_driver_unregister(&mut trusted_key_driver);
}

#[unsafe(no_mangle)]
pub static mut trusted_key_tee_ops: trusted_key_ops = trusted_key_ops {
    migratable: 0, /* non-migratable */
    init: Some(trusted_tee_init),
    seal: Some(trusted_tee_seal),
    unseal: Some(trusted_tee_unseal),
    get_random: Some(trusted_tee_get_random),
    exit: Some(trusted_tee_exit),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
