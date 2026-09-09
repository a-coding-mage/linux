// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

// Kernel and qcom_pas.h dependencies are supplied by the surrounding crate.

const TA_QCOM_PAS_IS_SUPPORTED: u32 = 1;
const TA_QCOM_PAS_CAPABILITIES: u32 = 2;
const TA_QCOM_PAS_INIT_IMAGE: u32 = 3;
const TA_QCOM_PAS_MEM_SETUP: u32 = 4;
const TA_QCOM_PAS_GET_RESOURCE_TABLE: u32 = 5;
const TA_QCOM_PAS_AUTH_AND_RESET: u32 = 6;
const TA_QCOM_PAS_SET_REMOTE_STATE: u32 = 7;
const TA_QCOM_PAS_SHUTDOWN: u32 = 8;
const TEE_NUM_PARAMS: u32 = 4;

#[repr(C)]
struct QcomPasTeePrivate {
    dev: *mut device,
    ctx: *mut tee_context,
    session_id: u32,
}

unsafe fn qcom_pas_tee_supported(dev: *mut device, pas_id: u32) -> bool {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_IS_SUPPORTED, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4];
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = pas_id;
    let ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 {
        dev_err(dev, "PAS not supported, pas_id: %d, ret: %d, err: 0x%x\n", pas_id, ret, inv_arg.ret);
        return false;
    }
    true
}

unsafe fn qcom_pas_tee_init_image(dev: *mut device, pas_id: u32, metadata: *const core::ffi::c_void, size: usize, ctx: *mut qcom_pas_context) -> i32 {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_INIT_IMAGE, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4];
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = pas_id;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT;
    let mdata_shm = tee_shm_alloc_kernel_buf((*data).ctx, size);
    if IS_ERR(mdata_shm) { dev_err(dev, "mdata_shm allocation failed\n"); return PTR_ERR(mdata_shm); }
    let mdata_buf = tee_shm_get_va(mdata_shm, 0);
    if IS_ERR(mdata_buf) { dev_err(dev, "mdata_buf get VA failed\n"); tee_shm_free(mdata_shm); return PTR_ERR(mdata_buf); }
    core::ptr::copy_nonoverlapping(metadata as *const u8, mdata_buf as *mut u8, size);
    param[1].u.memref.shm = mdata_shm;
    param[1].u.memref.size = size;
    let ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { dev_err(dev, "PAS init image failed, pas_id: %d, ret: %d, err: 0x%x\n", pas_id, ret, inv_arg.ret); tee_shm_free(mdata_shm); return if ret != 0 { ret } else { -EINVAL }; }
    if !ctx.is_null() { (*ctx).ptr = mdata_shm as *mut _; } else { tee_shm_free(mdata_shm); }
    ret
}

unsafe fn qcom_pas_tee_mem_setup(dev: *mut device, pas_id: u32, addr: phys_addr_t, size: phys_addr_t) -> i32 {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_MEM_SETUP, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4];
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[0].u.value.a = pas_id; param[0].u.value.b = size;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[1].u.value.a = lower_32_bits(addr); param[1].u.value.b = upper_32_bits(addr);
    let ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { dev_err(dev, "PAS mem setup failed, pas_id: %d, ret: %d, err: 0x%x\n", pas_id, ret, inv_arg.ret); return if ret != 0 { ret } else { -EINVAL }; } ret
}

unsafe fn __qcom_pas_tee_auth_and_reset(dev: *mut device, pas_id: u32, mem_phys: phys_addr_t, mem_size: usize) -> i32 {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_AUTH_AND_RESET, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4];
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[0].u.value.a = pas_id; param[0].u.value.b = mem_size;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[1].u.value.a = lower_32_bits(mem_phys); param[1].u.value.b = upper_32_bits(mem_phys);
    param[2].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT;
    let ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { dev_err(dev, "PAS auth reset failed, pas_id: %d, ret: %d, err: 0x%x\n", pas_id, ret, inv_arg.ret); return if ret != 0 { ret } else { -EINVAL }; } ret
}

unsafe fn qcom_pas_tee_auth_and_reset(dev: *mut device, pas_id: u32) -> i32 { __qcom_pas_tee_auth_and_reset(dev, pas_id, 0, 0) }
unsafe fn qcom_pas_tee_prepare_and_auth_reset(dev: *mut device, ctx: *mut qcom_pas_context) -> i32 { __qcom_pas_tee_auth_and_reset(dev, (*ctx).pas_id, (*ctx).mem_phys, (*ctx).mem_size) }

unsafe fn qcom_pas_tee_set_remote_state(dev: *mut device, state: u32, pas_id: u32) -> i32 {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_SET_REMOTE_STATE, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4]; param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[0].u.value.a = pas_id; param[0].u.value.b = state;
    let ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { dev_err(dev, "PAS set remote state failed, pas_id: %d, ret: %d, err: 0x%x\n", pas_id, ret, inv_arg.ret); return if ret != 0 { ret } else { -EINVAL }; } ret
}

unsafe fn qcom_pas_tee_shutdown(dev: *mut device, pas_id: u32) -> i32 {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_SHUTDOWN, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4]; param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[0].u.value.a = pas_id;
    let ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { dev_err(dev, "PAS shutdown failed, pas_id: %d, ret: %d, err: 0x%x\n", pas_id, ret, inv_arg.ret); return if ret != 0 { ret } else { -EINVAL }; } ret
}

unsafe fn qcom_pas_tee_metadata_release(_dev: *mut device, ctx: *mut qcom_pas_context) { tee_shm_free((*ctx).ptr as *mut tee_shm); (*ctx).ptr = core::ptr::null_mut(); }

unsafe fn qcom_pas_tee_get_rsc_table(dev: *mut device, ctx: *mut qcom_pas_context, input_rt: *mut core::ffi::c_void, input_rt_size: usize, output_rt_size: *mut usize) -> *mut core::ffi::c_void {
    let data = dev_get_drvdata(dev) as *mut QcomPasTeePrivate;
    let mut inv_arg = tee_ioctl_invoke_arg { func: TA_QCOM_PAS_GET_RESOURCE_TABLE, session: (*data).session_id, num_params: TEE_NUM_PARAMS, ..Default::default() };
    let mut param: [tee_param; 4] = [Default::default(); 4];
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT; param[0].u.value.a = (*ctx).pas_id;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT; param[1].u.memref.size = input_rt_size;
    let mut ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { dev_err(dev, "PAS get RT failed, pas_id: %d, ret: %d, err: 0x%x\n", (*ctx).pas_id, ret, inv_arg.ret); return ERR_PTR(if ret != 0 { ret } else { -EINVAL }); }
    if param[1].u.memref.size < input_rt_size { *output_rt_size = 0; return core::ptr::null_mut(); }
    let rt_shm = tee_shm_alloc_kernel_buf((*data).ctx, param[1].u.memref.size);
    if IS_ERR_OR_NULL(rt_shm) { dev_err(dev, "rt_shm allocation failed\n"); return ERR_PTR(-ENOMEM); }
    let rt_va = tee_shm_get_va(rt_shm, 0);
    if IS_ERR(rt_va) { dev_err(dev, "rt_shm get VA failed\n"); tee_shm_free(rt_shm); return ERR_CAST(rt_va); }
    core::ptr::copy_nonoverlapping(input_rt as *const u8, rt_va as *mut u8, input_rt_size);
    param[1].u.memref.shm = rt_shm;
    ret = tee_client_invoke_func((*data).ctx, &mut inv_arg, param.as_mut_ptr());
    if ret < 0 || inv_arg.ret != 0 { tee_shm_free(rt_shm); return ERR_PTR(if ret != 0 { ret } else { -EINVAL }); }
    if param[1].u.memref.size == 0 { tee_shm_free(rt_shm); return core::ptr::null_mut(); }
    *output_rt_size = param[1].u.memref.size;
    let out = kmemdup(rt_va, *output_rt_size, GFP_KERNEL); tee_shm_free(rt_shm); if out.is_null() { ERR_PTR(-ENOMEM) } else { out }
}

#[allow(dead_code)]
static mut qcom_pas_ops_tee: qcom_pas_ops = qcom_pas_ops { drv_name: "qcom-pas-tee", supported: Some(qcom_pas_tee_supported), init_image: Some(qcom_pas_tee_init_image), mem_setup: Some(qcom_pas_tee_mem_setup), get_rsc_table: Some(qcom_pas_tee_get_rsc_table), auth_and_reset: Some(qcom_pas_tee_auth_and_reset), prepare_and_auth_reset: Some(qcom_pas_tee_prepare_and_auth_reset), set_remote_state: Some(qcom_pas_tee_set_remote_state), shutdown: Some(qcom_pas_tee_shutdown), metadata_release: Some(qcom_pas_tee_metadata_release), ..Default::default() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
