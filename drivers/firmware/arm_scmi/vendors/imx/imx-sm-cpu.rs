// SPDX-License-Identifier: GPL-2.0
/*
 * System control and Management Interface (SCMI) NXP CPU Protocol
 *
 * Copyright 2025 NXP
 */

// Linux dependencies and sibling protocol declarations are supplied externally.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x10000;

#[repr(u8)]
enum ScmiImxCpuProtocolCmd {
    ScmiImxCpuAttributes = 0x3,
    ScmiImxCpuStart = 0x4,
    ScmiImxCpuStop = 0x5,
    ScmiImxCpuResetVectorSet = 0x6,
    ScmiImxCpuInfoGet = 0xC,
}

#[repr(C)]
struct ScmiImxCpuInfo {
    nr_cpu: u32,
}

const SCMI_IMX_CPU_NR_CPU_MASK: u32 = 0xffff;

#[repr(C)]
struct ScmiMsgImxCpuProtocolAttributes {
    attributes: u32,
}

const CPU_MAX_NAME: usize = 16;

#[repr(C)]
struct ScmiMsgImxCpuAttributesOut {
    attributes: u32,
    name: [u8; CPU_MAX_NAME],
}

const CPU_VEC_FLAGS_RESUME: u32 = 1 << 31;
const CPU_VEC_FLAGS_START: u32 = 1 << 30;
const CPU_VEC_FLAGS_BOOT: u32 = 1 << 29;

#[repr(C)]
struct ScmiImxCpuResetVectorSetIn {
    cpuid: u32,
    flags: u32,
    resetvectorlow: u32,
    resetvectorhigh: u32,
}

const CPU_RUN_MODE_START: u32 = 0;
const CPU_RUN_MODE_HOLD: u32 = 1;
const CPU_RUN_MODE_STOP: u32 = 2;
const CPU_RUN_MODE_SLEEP: u32 = 3;

#[repr(C)]
struct ScmiImxCpuInfoGetOut {
    runmode: u32,
    sleepmode: u32,
    resetvectorlow: u32,
    resetvectorhigh: u32,
}

unsafe fn scmi_imx_cpu_validate_cpuid(
    ph: *const ScmiProtocolHandle,
    cpuid: u32,
) -> i32 {
    let info = ((*ph).get_priv.unwrap())(ph);
    let info = info as *const ScmiImxCpuInfo;

    if cpuid >= (*info).nr_cpu {
        return -22;
    }
    0
}

unsafe fn scmi_imx_cpu_start(
    ph: *const ScmiProtocolHandle,
    cpuid: u32,
    start: bool,
) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let msg_id = if start {
        ScmiImxCpuProtocolCmd::ScmiImxCpuStart as u8
    } else {
        ScmiImxCpuProtocolCmd::ScmiImxCpuStop as u8
    };
    let mut ret = scmi_imx_cpu_validate_cpuid(ph, cpuid);
    if ret != 0 { return ret; }
    ret = ((*(*ph).xops).xfer_get_init.unwrap())(ph, msg_id, 4, 0, &mut t);
    if ret != 0 { return ret; }
    core::ptr::write_unaligned((*t).tx.buf as *mut u32, cpuid.to_le());
    ret = ((*(*ph).xops).do_xfer.unwrap())(ph, t);
    ((*(*ph).xops).xfer_put.unwrap())(ph, t);
    ret
}

unsafe fn scmi_imx_cpu_reset_vector_set(
    ph: *const ScmiProtocolHandle, cpuid: u32, vector: u64,
    start: bool, boot: bool, resume: bool,
) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = scmi_imx_cpu_validate_cpuid(ph, cpuid);
    if ret != 0 { return ret; }
    ret = ((*(*ph).xops).xfer_get_init.unwrap())(
        ph, ScmiImxCpuProtocolCmd::ScmiImxCpuResetVectorSet as u8,
        core::mem::size_of::<ScmiImxCpuResetVectorSetIn>(), 0, &mut t,
    );
    if ret != 0 { return ret; }
    let input = (*t).tx.buf as *mut ScmiImxCpuResetVectorSetIn;
    (*input).cpuid = cpuid.to_le();
    (*input).flags = 0;
    if start { (*input).flags |= (1u32 << 30).to_le(); }
    if boot { (*input).flags |= (1u32 << 29).to_le(); }
    if resume { (*input).flags |= (1u32 << 31).to_le(); }
    (*input).resetvectorlow = (vector as u32).to_le();
    (*input).resetvectorhigh = ((vector >> 32) as u32).to_le();
    ret = ((*(*ph).xops).do_xfer.unwrap())(ph, t);
    ((*(*ph).xops).xfer_put.unwrap())(ph, t);
    ret
}

unsafe fn scmi_imx_cpu_started(
    ph: *const ScmiProtocolHandle, cpuid: u32, started: *mut bool,
) -> i32 {
    if started.is_null() { return -22; }
    *started = false;
    let mut ret = scmi_imx_cpu_validate_cpuid(ph, cpuid);
    if ret != 0 { return ret; }
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    ret = ((*(*ph).xops).xfer_get_init.unwrap())(
        ph, ScmiImxCpuProtocolCmd::ScmiImxCpuInfoGet as u8, 4, 0, &mut t,
    );
    if ret != 0 { return ret; }
    core::ptr::write_unaligned((*t).tx.buf as *mut u32, cpuid.to_le());
    ret = ((*(*ph).xops).do_xfer.unwrap())(ph, t);
    if ret == 0 {
        let out = (*t).rx.buf as *const ScmiImxCpuInfoGetOut;
        let mode = u32::from_le((*out).runmode);
        if mode == CPU_RUN_MODE_START || mode == CPU_RUN_MODE_SLEEP { *started = true; }
    }
    ((*(*ph).xops).xfer_put.unwrap())(ph, t);
    ret
}

// Function table and protocol registration are provided by the SCMI framework.
const SCMI_IMX_CPU_PROTO_OPS: ScmiImxCpuProtoOps = ScmiImxCpuProtoOps {
    cpu_reset_vector_set: scmi_imx_cpu_reset_vector_set,
    cpu_start: scmi_imx_cpu_start,
    cpu_started: scmi_imx_cpu_started,
};

unsafe fn scmi_imx_cpu_protocol_attributes_get(
    ph: *const ScmiProtocolHandle, info: *mut ScmiImxCpuInfo,
) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init.unwrap())(
        ph, PROTOCOL_ATTRIBUTES, 0,
        core::mem::size_of::<ScmiMsgImxCpuProtocolAttributes>(), &mut t,
    );
    if ret != 0 { return ret; }
    ret = ((*(*ph).xops).do_xfer.unwrap())(ph, t);
    if ret == 0 {
        let attr = (*t).rx.buf as *const ScmiMsgImxCpuProtocolAttributes;
        (*info).nr_cpu = u32::from_le((*attr).attributes) & SCMI_IMX_CPU_NR_CPU_MASK;
    }
    ((*(*ph).xops).xfer_put.unwrap())(ph, t);
    ret
}

unsafe fn scmi_imx_cpu_attributes_get(ph: *const ScmiProtocolHandle, cpuid: u32) -> i32 {
    let mut t: *mut ScmiXfer = core::ptr::null_mut();
    let mut ret = ((*(*ph).xops).xfer_get_init.unwrap())(
        ph, ScmiImxCpuProtocolCmd::ScmiImxCpuAttributes as u8, 4, 0, &mut t,
    );
    if ret != 0 { return ret; }
    core::ptr::write_unaligned((*t).tx.buf as *mut u32, cpuid.to_le());
    ret = ((*(*ph).xops).do_xfer.unwrap())(ph, t);
    ((*(*ph).xops).xfer_put.unwrap())(ph, t);
    ret
}

unsafe fn scmi_imx_cpu_protocol_init(ph: *const ScmiProtocolHandle) -> i32 {
    let info = devm_kzalloc((*ph).dev, core::mem::size_of::<ScmiImxCpuInfo>(), GFP_KERNEL)
        as *mut ScmiImxCpuInfo;
    if info.is_null() { return -12; }
    let mut ret = scmi_imx_cpu_protocol_attributes_get(ph, info);
    if ret != 0 { return ret; }
    let mut i = 0;
    while i < (*info).nr_cpu {
        ret = scmi_imx_cpu_attributes_get(ph, i);
        if ret != 0 { return ret; }
        i += 1;
    }
    ((*ph).set_priv.unwrap())(ph, info as *mut core::ffi::c_void)
}

// Module metadata and the concrete protocol descriptor are supplied by the kernel SCMI bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
