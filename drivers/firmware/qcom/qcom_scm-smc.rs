// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2015,2019 The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux kernel and other translation units:
// linux/cleanup.h, linux/io.h, linux/errno.h, linux/delay.h, linux/mutex.h,
// linux/slab.h, linux/types.h, linux/firmware/qcom/qcom_scm.h,
// linux/firmware/qcom/qcom_tzmem.h, linux/arm-smccc.h, linux/dma-mapping.h,
// qcom_scm.h, and qcom_scm_trace.h.

#[repr(C)]
pub struct arm_smccc_args {
    pub args: [core::ffi::c_ulong; 8],
}

static mut qcom_scm_lock: DEFINE_MUTEX_TYPE = DEFINE_MUTEX!();

const QCOM_SCM_EBUSY_WAIT_MS: u32 = 30;
const QCOM_SCM_EBUSY_MAX_RETRY: i32 = 20;

const SCM_SMC_N_REG_ARGS: usize = 4;
const SCM_SMC_FIRST_EXT_IDX: usize = SCM_SMC_N_REG_ARGS - 1;
const SCM_SMC_N_EXT_ARGS: usize = MAX_QCOM_SCM_ARGS - SCM_SMC_N_REG_ARGS + 1;
const SCM_SMC_FIRST_REG_IDX: usize = 2;
const SCM_SMC_LAST_REG_IDX: usize = SCM_SMC_FIRST_REG_IDX + SCM_SMC_N_REG_ARGS - 1;

unsafe fn __scm_smc_do_quirk(
    smc: *const arm_smccc_args,
    res: *mut arm_smccc_res,
) {
    let mut a0 = (*smc).args[0];
    let mut quirk = arm_smccc_quirk {
        id: ARM_SMCCC_QUIRK_QCOM_A6,
        ..core::mem::zeroed()
    };

    quirk.state.a6 = 0;

    loop {
        trace_scm_smc_request(a0, smc);
        arm_smccc_smc_quirk(
            a0,
            (*smc).args[1],
            (*smc).args[2],
            (*smc).args[3],
            (*smc).args[4],
            (*smc).args[5],
            quirk.state.a6,
            (*smc).args[7],
            res,
            &mut quirk,
        );

        if (*res).a0 == QCOM_SCM_INTERRUPTED {
            a0 = (*res).a0;
        }

        if (*res).a0 != QCOM_SCM_INTERRUPTED {
            break;
        }
    }
}

unsafe fn fill_wq_resume_args(resume: *mut arm_smccc_args, smc_call_ctx: u32) {
    (*resume).args.fill(0);

    (*resume).args[0] = ARM_SMCCC_CALL_VAL(
        ARM_SMCCC_STD_CALL,
        ARM_SMCCC_SMC_64,
        ARM_SMCCC_OWNER_SIP,
        SCM_SMC_FNID(QCOM_SCM_SVC_WAITQ, QCOM_SCM_WAITQ_RESUME),
    );

    (*resume).args[1] = QCOM_SCM_ARGS(1);
    (*resume).args[2] = smc_call_ctx as core::ffi::c_ulong;
}

pub unsafe fn scm_get_wq_ctx(
    wq_ctx: *mut u32,
    flags: *mut u32,
    more_pending: *mut u32,
) -> i32 {
    let mut get_wq_res: arm_smccc_res = core::mem::zeroed();
    let mut get_wq_ctx: arm_smccc_args = core::mem::zeroed();

    get_wq_ctx.args[0] = ARM_SMCCC_CALL_VAL(
        ARM_SMCCC_FAST_CALL,
        ARM_SMCCC_SMC_64,
        ARM_SMCCC_OWNER_SIP,
        SCM_SMC_FNID(QCOM_SCM_SVC_WAITQ, QCOM_SCM_WAITQ_GET_WQ_CTX),
    );

    // Guaranteed to return only success or error, no WAITQ_*
    __scm_smc_do_quirk(&get_wq_ctx, &mut get_wq_res);
    let ret = get_wq_res.a0 as i32;
    if ret != 0 {
        return ret;
    }

    trace_scm_waitq_get_wq_ctx(get_wq_res.a1, get_wq_res.a2, get_wq_res.a3);
    *wq_ctx = get_wq_res.a1 as u32;
    *flags = get_wq_res.a2 as u32;
    *more_pending = get_wq_res.a3 as u32;

    0
}

unsafe fn __scm_smc_do_quirk_handle_waitq(
    dev: *mut device,
    waitq: *mut arm_smccc_args,
    res: *mut arm_smccc_res,
) -> i32 {
    let mut smc_call_ctx: u32;
    let mut resume: arm_smccc_args = core::mem::zeroed();
    let mut smc = waitq;

    loop {
        __scm_smc_do_quirk(smc, res);

        if (*res).a0 == QCOM_SCM_WAITQ_SLEEP {
            let wq_ctx = (*res).a1 as u32;
            smc_call_ctx = (*res).a2 as u32;

            trace_scm_waitq_sleep(wq_ctx, smc_call_ctx);
            let ret = qcom_scm_wait_for_wq_completion(dev, wq_ctx);
            if ret != 0 {
                return ret;
            }

            trace_scm_waitq_resume(smc_call_ctx);
            fill_wq_resume_args(&mut resume, smc_call_ctx);
            smc = &mut resume;
        }

        if (*res).a0 != QCOM_SCM_WAITQ_SLEEP {
            break;
        }
    }

    0
}

unsafe fn __scm_smc_do(
    dev: *mut device,
    smc: *mut arm_smccc_args,
    res: *mut arm_smccc_res,
    atomic: bool,
) -> i32 {
    let mut retry_count: i32 = 0;

    if atomic {
        __scm_smc_do_quirk(smc, res);
        return 0;
    }

    loop {
        mutex_lock(&mut qcom_scm_lock);
        let ret = __scm_smc_do_quirk_handle_waitq(dev, smc, res);
        mutex_unlock(&mut qcom_scm_lock);

        if ret != 0 {
            return ret;
        }

        if (*res).a0 == QCOM_SCM_V2_EBUSY {
            if retry_count > QCOM_SCM_EBUSY_MAX_RETRY {
                break;
            }
            retry_count += 1;
            msleep(QCOM_SCM_EBUSY_WAIT_MS);
        } else {
            break;
        }
    }

    0
}

pub unsafe fn __scm_smc_call(
    dev: *mut device,
    desc: *const qcom_scm_desc,
    qcom_convention: qcom_scm_convention,
    res: *mut qcom_scm_res,
    atomic: bool,
) -> i32 {
    let arglen = ((*desc).arginfo & 0xf) as usize;
    let flag = if atomic { GFP_ATOMIC } else { GFP_KERNEL };
    let smccc_call_type = if atomic { ARM_SMCCC_FAST_CALL } else { ARM_SMCCC_STD_CALL };
    let qcom_smccc_convention = if qcom_convention == SMC_CONVENTION_ARM_32 {
        ARM_SMCCC_SMC_32
    } else {
        ARM_SMCCC_SMC_64
    };
    let mut smc_res: arm_smccc_res = core::mem::zeroed();
    let mut smc: arm_smccc_args = core::mem::zeroed();

    smc.args[0] = ARM_SMCCC_CALL_VAL(
        smccc_call_type,
        qcom_smccc_convention,
        (*desc).owner,
        SCM_SMC_FNID((*desc).svc, (*desc).cmd),
    );
    smc.args[1] = (*desc).arginfo;
    for i in 0..SCM_SMC_N_REG_ARGS {
        smc.args[i + SCM_SMC_FIRST_REG_IDX] = (*desc).args[i];
    }

    if arglen > SCM_SMC_N_REG_ARGS {
        let mempool = qcom_scm_get_tzmem_pool();
        if mempool.is_null() {
            return -EINVAL;
        }

        let args_virt = qcom_tzmem_alloc(
            mempool,
            SCM_SMC_N_EXT_ARGS * core::mem::size_of::<u64>(),
            flag,
        );
        if args_virt.is_null() {
            return -ENOMEM;
        }

        if qcom_smccc_convention == ARM_SMCCC_SMC_32 {
            let args = args_virt as *mut __le32;
            for i in 0..SCM_SMC_N_EXT_ARGS {
                *args.add(i) = cpu_to_le32((*desc).args[i + SCM_SMC_FIRST_EXT_IDX]);
            }
        } else {
            let args = args_virt as *mut __le64;
            for i in 0..SCM_SMC_N_EXT_ARGS {
                *args.add(i) = cpu_to_le64((*desc).args[i + SCM_SMC_FIRST_EXT_IDX]);
            }
        }

        smc.args[SCM_SMC_LAST_REG_IDX] = qcom_tzmem_to_phys(args_virt);
        // C cleanup attribute: qcom_tzmem frees args_virt on scope exit.
        qcom_tzmem_cleanup(args_virt);
    }

    let ret = __scm_smc_do(dev, &mut smc, &mut smc_res, atomic);

    trace_scm_smc_done(ret, smc.args[0], &smc_res);

    if ret != 0 {
        return ret;
    }

    if !res.is_null() {
        (*res).result[0] = smc_res.a1;
        (*res).result[1] = smc_res.a2;
        (*res).result[2] = smc_res.a3;
    }

    if smc_res.a0 != 0 {
        qcom_scm_remap_error(smc_res.a0) as i32
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
