// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018-2019 Linaro Ltd.
 */

// Linux kernel dependencies supplied by other translation units.

pub const DRIVER_NAME: &str = "optee-rng";
pub const TEE_ERROR_HEALTH_TEST_FAIL: u32 = 0x00000001;
pub const TA_CMD_GET_ENTROPY: u32 = 0x0;
pub const TA_CMD_GET_RNG_INFO: u32 = 0x1;
pub const MAX_ENTROPY_REQ_SZ: usize = 4 * 1024;

// TA_CMD_GET_ENTROPY - Get Entropy from RNG
// param[0] (inout memref) - Entropy buffer memory reference
// param[1..3] unused
// TEE_SUCCESS, TEE_ERROR_BAD_PARAMETERS, TEE_ERROR_NOT_SUPPORTED,
// TEE_ERROR_HEALTH_TEST_FAIL are returned by the Trusted App.

// TA_CMD_GET_RNG_INFO - Get RNG information
// param[0] (out value) - value.a: RNG data-rate in bytes per second
//                        value.b: Quality/Entropy per 1024 bit of data
// param[1..3] unused

#[repr(C)]
pub struct Device { _private: [u8; 0] }
#[repr(C)]
pub struct TeeContext { _private: [u8; 0] }
#[repr(C)]
pub struct TeeShm { _private: [u8; 0] }
#[repr(C)]
pub struct TeeClientDevice { pub dev: Device, pub id: TeeClientDeviceId }
#[repr(C)]
pub struct TeeClientDeviceId { pub uuid: TeeUuid }
#[repr(C)]
pub struct TeeUuid { _private: [u8; 16] }
#[repr(C)]
pub struct TeeIoctlVersionData { pub impl_id: u32 }
#[repr(C)]
pub struct TeeIoctlInvokeArg { pub func: u32, pub session: u32, pub num_params: u32, pub ret: u32 }
#[repr(C)]
pub union TeeParamUnion { pub memref: TeeParamMemref, pub value: TeeParamValue }
#[repr(C)]
pub struct TeeParam { pub attr: u64, pub u: TeeParamUnion }
#[repr(C)]
pub struct TeeParamMemref { pub shm: *mut TeeShm, pub size: usize, pub shm_offs: usize }
#[repr(C)]
pub struct TeeParamValue { pub a: u32, pub b: u32, pub c: u32 }
#[repr(C)]
pub struct Hwrng {
    pub name: *const u8,
    pub init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>,
    pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
    pub quality: u32,
}

#[repr(C)]
pub struct OpteeRngPrivate {
    pub dev: *mut Device,
    pub ctx: *mut TeeContext,
    pub session_id: u32,
    pub data_rate: u32,
    pub entropy_shm_pool: *mut TeeShm,
    pub optee_rng: Hwrng,
}

// External kernel interfaces are declarations supplied by other files.
extern "C" {
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize);
    fn memcpy(d: *mut core::ffi::c_void, s: *const core::ffi::c_void, n: usize);
    fn tee_client_invoke_func(ctx: *mut TeeContext, arg: *mut TeeIoctlInvokeArg, param: *mut TeeParam) -> i32;
    fn tee_shm_get_va(shm: *mut TeeShm, offs: usize) -> *mut u8;
    fn tee_shm_alloc_kernel_buf(ctx: *mut TeeContext, size: usize) -> *mut TeeShm;
    fn tee_shm_free(shm: *mut TeeShm);
    fn msleep(msecs: u32);
    fn dev_err(dev: *mut Device, fmt: *const u8, ...);
    fn tee_client_open_context(data: *const core::ffi::c_void, match_fn: Option<unsafe extern "C" fn(*mut TeeIoctlVersionData, *const core::ffi::c_void) -> i32>, a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> *mut TeeContext;
    fn tee_client_open_session(ctx: *mut TeeContext, arg: *mut TeeIoctlOpenSessionArg, param: *mut TeeParam) -> i32;
    fn tee_client_close_session(ctx: *mut TeeContext, session: u32);
    fn tee_client_close_context(ctx: *mut TeeContext);
    fn devm_hwrng_register(dev: *mut Device, rng: *mut Hwrng) -> i32;
}

#[repr(C)]
pub struct TeeIoctlOpenSessionArg { pub uuid: [u8; 16], pub clnt_login: u32, pub num_params: u32, pub session: u32, pub ret: u32 }

pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT: u64 = 0;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT: u64 = 0;
pub const TEE_IOCTL_LOGIN_PUBLIC: u32 = 0;
pub const TEE_IMPL_ID_OPTEE: u32 = 0;
pub const EINVAL: i32 = 22;
pub const ENODEV: i32 = 19;

#[inline]
unsafe fn to_optee_rng_private(r: *mut Hwrng) -> *mut OpteeRngPrivate {
    (r as *mut u8).sub(core::mem::offset_of!(OpteeRngPrivate, optee_rng)) as *mut OpteeRngPrivate
}

static mut PVT_DATA: OpteeRngPrivate = OpteeRngPrivate {
    dev: core::ptr::null_mut(), ctx: core::ptr::null_mut(), session_id: 0,
    data_rate: 0, entropy_shm_pool: core::ptr::null_mut(),
    optee_rng: Hwrng { name: DRIVER_NAME.as_ptr(), init: Some(optee_rng_init), cleanup: Some(optee_rng_cleanup), read: Some(optee_rng_read), quality: 0 },
};

unsafe fn get_optee_rng_data(p: *mut OpteeRngPrivate, buf: *mut u8, req_size: usize) -> usize {
    let mut inv_arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; 4] = core::mem::zeroed();
    inv_arg.func = TA_CMD_GET_ENTROPY; inv_arg.session = (*p).session_id; inv_arg.num_params = 4;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT;
    param[0].u.memref = TeeParamMemref { shm: (*p).entropy_shm_pool, size: req_size, shm_offs: 0 };
    let ret = tee_client_invoke_func((*p).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { return 0; }
    let rng_data = tee_shm_get_va((*p).entropy_shm_pool, 0);
    let rng_size = param[0].u.memref.size;
    memcpy(buf as *mut _, rng_data as *const _, rng_size); rng_size
}

unsafe extern "C" fn optee_rng_read(rng: *mut Hwrng, buf: *mut core::ffi::c_void, mut max: usize, wait: bool) -> i32 {
    let p = to_optee_rng_private(rng); if max > MAX_ENTROPY_REQ_SZ { max = MAX_ENTROPY_REQ_SZ; }
    let mut read = 0usize; let mut timeout = 1i32; let mut data = buf as *mut u8;
    while read < max { let rng_size = get_optee_rng_data(p, data, max - read); data = data.add(rng_size); read += rng_size;
        if wait && (*p).data_rate != 0 { timeout -= 1; if timeout == -1 || read == max { return read as i32; } msleep(((1000 * (max - read)) / (*p).data_rate as usize) as u32); } else { return read as i32; }
    } read as i32
}

unsafe extern "C" fn optee_rng_init(rng: *mut Hwrng) -> i32 { let p = to_optee_rng_private(rng); let s = tee_shm_alloc_kernel_buf((*p).ctx, MAX_ENTROPY_REQ_SZ); (*p).entropy_shm_pool = s; 0 }
unsafe extern "C" fn optee_rng_cleanup(rng: *mut Hwrng) { tee_shm_free((*to_optee_rng_private(rng)).entropy_shm_pool); }

unsafe fn get_optee_rng_info(dev: *mut Device) -> i32 {
    let mut inv_arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; 4] = core::mem::zeroed();
    inv_arg.func = TA_CMD_GET_RNG_INFO; inv_arg.session = PVT_DATA.session_id; inv_arg.num_params = 4;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT;
    let ret = tee_client_invoke_func(PVT_DATA.ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { return -EINVAL; }
    PVT_DATA.data_rate = param[0].u.value.a;
    PVT_DATA.optee_rng.quality = param[0].u.value.b;
    0
}

unsafe extern "C" fn optee_ctx_match(ver: *mut TeeIoctlVersionData, _data: *const core::ffi::c_void) -> i32 {
    ((*ver).impl_id == TEE_IMPL_ID_OPTEE) as i32
}

unsafe extern "C" fn optee_rng_probe(rng_device: *mut TeeClientDevice) -> i32 {
    let dev = &mut (*rng_device).dev as *mut Device;
    let mut sess_arg: TeeIoctlOpenSessionArg = core::mem::zeroed();
    PVT_DATA.ctx = tee_client_open_context(core::ptr::null(), Some(optee_ctx_match), core::ptr::null(), core::ptr::null());
    if PVT_DATA.ctx.is_null() { return -ENODEV; }
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_PUBLIC; sess_arg.num_params = 0;
    let ret = tee_client_open_session(PVT_DATA.ctx, &mut sess_arg, core::ptr::null_mut());
    if ret < 0 || sess_arg.ret != 0 { tee_client_close_context(PVT_DATA.ctx); return -EINVAL; }
    PVT_DATA.session_id = sess_arg.session;
    let err = get_optee_rng_info(dev);
    if err != 0 { tee_client_close_session(PVT_DATA.ctx, PVT_DATA.session_id); tee_client_close_context(PVT_DATA.ctx); return err; }
    let err = devm_hwrng_register(dev, &mut PVT_DATA.optee_rng);
    if err != 0 { tee_client_close_session(PVT_DATA.ctx, PVT_DATA.session_id); tee_client_close_context(PVT_DATA.ctx); return err; }
    PVT_DATA.dev = dev; 0
}

unsafe extern "C" fn optee_rng_remove(_tee_dev: *mut TeeClientDevice) {
    tee_client_close_session(PVT_DATA.ctx, PVT_DATA.session_id);
    tee_client_close_context(PVT_DATA.ctx);
}

#[repr(C)]
pub struct TeeClientDriver { pub probe: Option<unsafe extern "C" fn(*mut TeeClientDevice) -> i32>, pub remove: Option<unsafe extern "C" fn(*mut TeeClientDevice)>, pub id_table: *const TeeClientDeviceId, pub name: *const u8 }

// UUID_INIT(0xab7a617c, 0xb8e7, 0x4d8f, 0x83, 0x01, 0xd0, 0x9b, 0x61, 0x03, 0x6b, 0x64)
static OPTEE_RNG_ID_TABLE: [TeeClientDeviceId; 2] = unsafe { core::mem::zeroed() };
static OPTEE_RNG_DRIVER: TeeClientDriver = TeeClientDriver { probe: Some(optee_rng_probe), remove: Some(optee_rng_remove), id_table: OPTEE_RNG_ID_TABLE.as_ptr(), name: DRIVER_NAME.as_ptr() };

// module_tee_client_driver(optee_rng_driver);
// MODULE_DEVICE_TABLE(tee, optee_rng_id_table);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Sumit Garg <sumit.garg@linaro.org>");
// MODULE_DESCRIPTION("OP-TEE based random number generator driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
