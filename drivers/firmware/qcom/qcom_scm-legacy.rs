// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2010,2015,2019 The Linux Foundation. All rights reserved.
 * Copyright (C) 2015 Linaro Ltd.
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

#[repr(C)]
pub struct ArmSmcccArgs {
    pub args: [usize; 8],
}

#[repr(C)]
pub struct ScmLegacyCommand {
    pub len: u32,
    pub buf_offset: u32,
    pub resp_hdr_offset: u32,
    pub id: u32,
    pub buf: [u32; 0],
}

#[repr(C)]
pub struct ScmLegacyResponse {
    pub len: u32,
    pub buf_offset: u32,
    pub is_complete: u32,
}

#[inline]
unsafe fn scm_legacy_command_to_response(
    cmd: *const ScmLegacyCommand,
) -> *mut ScmLegacyResponse {
    (cmd as *mut u8).add(le32_to_cpu((*cmd).resp_hdr_offset) as usize)
        as *mut ScmLegacyResponse
}

#[inline]
unsafe fn scm_legacy_get_command_buffer(cmd: *const ScmLegacyCommand) -> *mut u32 {
    (*cmd).buf.as_ptr() as *mut u32
}

#[inline]
unsafe fn scm_legacy_get_response_buffer(rsp: *const ScmLegacyResponse) -> *const u32 {
    (rsp as *const u8).add(le32_to_cpu((*rsp).buf_offset) as usize) as *const u32
}

unsafe fn __scm_legacy_do(smc: *const ArmSmcccArgs, res: *mut ArmSmcccRes) {
    loop {
        arm_smccc_smc(
            (*smc).args[0], (*smc).args[1], (*smc).args[2], (*smc).args[3],
            (*smc).args[4], (*smc).args[5], (*smc).args[6], (*smc).args[7], res,
        );
        if (*res).a0 != QCOM_SCM_INTERRUPTED {
            break;
        }
    }
}

pub unsafe fn scm_legacy_call(
    dev: *mut Device,
    desc: *const QcomScmDesc,
    res: *mut QcomScmRes,
) -> i32 {
    let arglen = (*desc).arginfo & 0xf;
    let mut ret: i32 = 0;
    let mut context_id: i32 = 0;
    let cmd_len = (arglen as usize) * core::mem::size_of::<u32>();
    let resp_len = (MAX_QCOM_SCM_RETS as usize) * core::mem::size_of::<u32>();
    let alloc_len = core::mem::size_of::<ScmLegacyCommand>() + cmd_len
        + core::mem::size_of::<ScmLegacyResponse>() + resp_len;
    let mut cmd = kzalloc(page_align(alloc_len), GFP_KERNEL) as *mut ScmLegacyCommand;
    if cmd.is_null() {
        return -12;
    }

    (*cmd).len = cpu_to_le32(alloc_len as u32);
    (*cmd).buf_offset = cpu_to_le32(core::mem::size_of::<ScmLegacyCommand>() as u32);
    (*cmd).resp_hdr_offset = cpu_to_le32(
        (core::mem::size_of::<ScmLegacyCommand>() + cmd_len) as u32,
    );
    (*cmd).id = cpu_to_le32(SCM_LEGACY_FNID((*desc).svc, (*desc).cmd));

    let arg_buf = scm_legacy_get_command_buffer(cmd);
    for i in 0..arglen as usize {
        *arg_buf.add(i) = cpu_to_le32((*desc).args[i]);
    }

    let rsp = scm_legacy_command_to_response(cmd);
    let cmd_phys = dma_map_single(dev, cmd as *mut core::ffi::c_void, alloc_len, DMA_TO_DEVICE);
    if dma_mapping_error(dev, cmd_phys) {
        kfree(cmd as *mut core::ffi::c_void);
        return -12;
    }

    let smc = ArmSmcccArgs { args: [1, &mut context_id as *mut i32 as usize, cmd_phys, 0, 0, 0, 0, 0] };
    let mut smc_res = ArmSmcccRes::default();
    mutex_lock(&qcom_scm_lock);
    __scm_legacy_do(&smc, &mut smc_res);
    if smc_res.a0 != 0 {
        ret = qcom_scm_remap_error(smc_res.a0);
    }
    mutex_unlock(&qcom_scm_lock);
    if ret != 0 {
        dma_unmap_single(dev, cmd_phys, alloc_len, DMA_TO_DEVICE);
        kfree(cmd as *mut core::ffi::c_void);
        return ret;
    }

    while (*rsp).is_complete == 0 {
        dma_sync_single_for_cpu(dev, cmd_phys + core::mem::size_of::<ScmLegacyCommand>() + cmd_len,
            core::mem::size_of::<ScmLegacyResponse>(), DMA_FROM_DEVICE);
    }
    dma_sync_single_for_cpu(dev, cmd_phys + core::mem::size_of::<ScmLegacyCommand>() + cmd_len
        + le32_to_cpu((*rsp).buf_offset) as usize, resp_len, DMA_FROM_DEVICE);

    if !res.is_null() {
        let res_buf = scm_legacy_get_response_buffer(rsp);
        for i in 0..MAX_QCOM_SCM_RETS as usize {
            (*res).result[i] = le32_to_cpu(*res_buf.add(i));
        }
    }
    dma_unmap_single(dev, cmd_phys, alloc_len, DMA_TO_DEVICE);
    kfree(cmd as *mut core::ffi::c_void);
    ret
}

pub const SCM_LEGACY_ATOMIC_N_REG_ARGS: usize = 5;
pub const SCM_LEGACY_ATOMIC_FIRST_REG_IDX: usize = 2;
pub const SCM_LEGACY_CLASS_REGISTER: usize = 0x2 << 8;
pub const SCM_LEGACY_MASK_IRQS: usize = 1 << 5;

#[inline]
pub const fn scm_legacy_atomic_id(svc: usize, cmd: usize, n: usize) -> usize {
    (SCM_LEGACY_FNID(svc, cmd) << 12) | SCM_LEGACY_CLASS_REGISTER | SCM_LEGACY_MASK_IRQS | (n & 0xf)
}

pub unsafe fn scm_legacy_call_atomic(
    _unused: *mut Device,
    desc: *const QcomScmDesc,
    res: *mut QcomScmRes,
) -> usize {
    let mut context_id: i32 = 0;
    let arglen = ((*desc).arginfo & 0xf) as usize;
    BUG_ON(arglen > SCM_LEGACY_ATOMIC_N_REG_ARGS);
    let mut smc_res = ArmSmcccRes::default();
    arm_smccc_smc(
        scm_legacy_atomic_id((*desc).svc, (*desc).cmd, arglen),
        &mut context_id as *mut i32 as usize,
        (*desc).args[0], (*desc).args[1], (*desc).args[2], (*desc).args[3], (*desc).args[4], 0,
        &mut smc_res,
    );
    if !res.is_null() {
        (*res).result[0] = smc_res.a1;
        (*res).result[1] = smc_res.a2;
        (*res).result[2] = smc_res.a3;
    }
    smc_res.a0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
