// SPDX-License-Identifier: GPL-2.0
/*
 * System control and Management Interface (SCMI) NXP LMM Protocol
 *
 * Copyright 2025 NXP
 */

// C includes and symbols supplied by the surrounding SCMI/kernel sources are
// intentionally represented as external dependencies.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x10000;

#[repr(u8)]
enum ScmiImxLmmProtocolCmd {
    ScmiImxLmmAttributes = 0x3,
    ScmiImxLmmBoot = 0x4,
    ScmiImxLmmReset = 0x5,
    ScmiImxLmmShutdown = 0x6,
    ScmiImxLmmWake = 0x7,
    ScmiImxLmmSuspend = 0x8,
    ScmiImxLmmNotify = 0x9,
    ScmiImxLmmResetReason = 0xa,
    ScmiImxLmmPowerOn = 0xb,
    ScmiImxLmmResetVectorSet = 0xc,
}

#[repr(C)]
struct ScmiImxLmmPriv {
    nr_lmm: u32,
}

const SCMI_IMX_LMM_NR_LM_MASK: u32 = 0x3f;
const SCMI_IMX_LMM_NR_MAX: u32 = 16;
const SCMI_IMX_LMM_SHUTDOWN_GRACEFUL: u32 = 1 << 0;

#[repr(C)]
struct ScmiMsgImxLmmProtocolAttributes {
    attributes: u32,
}

#[repr(C)]
struct ScmiMsgImxLmmAttributesOut {
    lmid: u32,
    attributes: u32,
    state: u32,
    errstatus: u32,
    name: [u8; LMM_MAX_NAME],
}

#[repr(C)]
struct ScmiImxLmmResetVectorSetIn {
    lmid: u32,
    cpuid: u32,
    flags: u32, // reserved for future extension
    resetvectorlow: u32,
    resetvectorhigh: u32,
}

#[repr(C)]
struct ScmiImxLmmShutdownIn {
    lmid: u32,
    flags: u32,
}

unsafe fn scmi_imx_lmm_validate_lmid(ph: *const ScmiProtocolHandle, lmid: u32) -> i32 {
    let priv_: *mut ScmiImxLmmPriv = ((*ph).get_priv)(ph) as *mut ScmiImxLmmPriv;
    if lmid >= (*priv_).nr_lmm { return -EINVAL; }
    0
}

unsafe fn scmi_imx_lmm_attributes(
    ph: *const ScmiProtocolHandle, lmid: u32, info: *mut ScmiImxLmmInfo,
) -> i32 {
    let mut out: *mut ScmiMsgImxLmmAttributesOut;
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiImxLmmProtocolCmd::ScmiImxLmmAttributes as u8,
                                                core::mem::size_of::<u32>(), 0, &mut t);
    if ret != 0 { return ret; }
    put_unaligned_le32(lmid, (*t).tx.buf);
    ret = ((*(*ph).xops).do_xfer)(ph, t);
    if ret == 0 {
        out = (*t).rx.buf as *mut ScmiMsgImxLmmAttributesOut;
        (*info).lmid = le32_to_cpu((*out).lmid);
        (*info).state = le32_to_cpu((*out).state);
        (*info).errstatus = le32_to_cpu((*out).errstatus);
        strscpy((*info).name.as_mut_ptr(), (*out).name.as_ptr());
        dev_dbg((*ph).dev, "i.MX LMM: Logical Machine(%d), name: %s\n", (*info).lmid, (*info).name.as_ptr());
    } else {
        dev_err((*ph).dev, "i.MX LMM: Failed to get info of Logical Machine(%u)\n", lmid);
    }
    ((*(*ph).xops).xfer_put)(ph, t);
    ret
}

unsafe fn scmi_imx_lmm_power_boot(ph: *const ScmiProtocolHandle, lmid: u32, boot: bool) -> i32 {
    let ret = scmi_imx_lmm_validate_lmid(ph, lmid); if ret != 0 { return ret; }
    let msg_id = if boot { ScmiImxLmmProtocolCmd::ScmiImxLmmBoot as u8 } else { ScmiImxLmmProtocolCmd::ScmiImxLmmPowerOn as u8 };
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, msg_id, core::mem::size_of::<u32>(), 0, &mut t);
    if ret != 0 { return ret; }
    put_unaligned_le32(lmid, (*t).tx.buf);
    ret = ((*(*ph).xops).do_xfer)(ph, t);
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_imx_lmm_reset_vector_set(ph: *const ScmiProtocolHandle, lmid: u32, cpuid: u32, _flags: u32, vector: u64) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiImxLmmProtocolCmd::ScmiImxLmmResetVectorSet as u8, core::mem::size_of::<ScmiImxLmmResetVectorSetIn>(), 0, &mut t);
    if ret != 0 { return ret; }
    let in_ = (*t).tx.buf as *mut ScmiImxLmmResetVectorSetIn;
    (*in_).lmid = cpu_to_le32(lmid); (*in_).cpuid = cpu_to_le32(cpuid); (*in_).flags = cpu_to_le32(0);
    (*in_).resetvectorlow = cpu_to_le32(vector as u32); (*in_).resetvectorhigh = cpu_to_le32((vector >> 32) as u32);
    ret = ((*(*ph).xops).do_xfer)(ph, t); ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_imx_lmm_shutdown(ph: *const ScmiProtocolHandle, lmid: u32, flags: u32) -> i32 {
    let mut ret = scmi_imx_lmm_validate_lmid(ph, lmid); if ret != 0 { return ret; }
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    ret = ((*(*ph).xops).xfer_get_init)(ph, ScmiImxLmmProtocolCmd::ScmiImxLmmShutdown as u8, core::mem::size_of::<ScmiImxLmmShutdownIn>(), 0, &mut t);
    if ret != 0 { return ret; }
    let in_ = (*t).tx.buf as *mut ScmiImxLmmShutdownIn; (*in_).lmid = cpu_to_le32(lmid);
    (*in_).flags = cpu_to_le32(if flags & SCMI_IMX_LMM_SHUTDOWN_GRACEFUL != 0 { SCMI_IMX_LMM_SHUTDOWN_GRACEFUL } else { 0 });
    ret = ((*(*ph).xops).do_xfer)(ph, t); ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_imx_lmm_protocol_attributes_get(ph: *const ScmiProtocolHandle, priv_: *mut ScmiImxLmmPriv) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init)(ph, PROTOCOL_ATTRIBUTES, 0,
                                                core::mem::size_of::<ScmiMsgImxLmmProtocolAttributes>(), &mut t);
    if ret != 0 { return ret; }
    let attr = (*t).rx.buf as *mut ScmiMsgImxLmmProtocolAttributes;
    ret = ((*(*ph).xops).do_xfer)(ph, t);
    if ret == 0 {
        (*priv_).nr_lmm = le32_get_bits((*attr).attributes, SCMI_IMX_LMM_NR_LM_MASK);
        if (*priv_).nr_lmm > SCMI_IMX_LMM_NR_MAX {
            dev_err((*ph).dev, "i.MX LMM: %d:Exceed max supported Logical Machines\n", (*priv_).nr_lmm);
            ret = -EINVAL;
        } else {
            dev_info((*ph).dev, "i.MX LMM: %d Logical Machines\n", (*priv_).nr_lmm);
        }
    }
    ((*(*ph).xops).xfer_put)(ph, t); ret
}

unsafe fn scmi_imx_lmm_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    dev_info((*ph).dev, "NXP SM LMM Version %d.%d\n", PROTOCOL_REV_MAJOR((*ph).version), PROTOCOL_REV_MINOR((*ph).version));
    let info = devm_kzalloc((*ph).dev, core::mem::size_of::<ScmiImxLmmPriv>(), GFP_KERNEL) as *mut ScmiImxLmmPriv;
    if info.is_null() { return -ENOMEM; }
    let ret = scmi_imx_lmm_protocol_attributes_get(ph, info); if ret != 0 { return ret; }
    ((*ph).set_priv)(ph, info as *mut core::ffi::c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
