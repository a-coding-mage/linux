// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021, Linaro Limited

// C dependencies originally included:
// linux/slab.h, linux/wait.h, linux/kernel.h, linux/module.h, linux/of.h,
// linux/delay.h, linux/of_platform.h, linux/jiffies.h, linux/soc/qcom/apr.h,
// dt-bindings/soc/qcom,gpr.h, dt-bindings/sound/qcom,q6dsp-lpass-ports.h,
// q6apm.h, q6prm.h, audioreach.h

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct gpr_svc {
    pub id: u32,
}

#[repr(C)]
pub struct gpr_device_t {
    pub dev: device,
    pub svc: gpr_svc,
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpr_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: u32,
}

#[repr(C)]
pub struct q6prm {
    pub dev: *mut device,
    pub gdev: *mut gpr_device_t,
    pub wait: wait_queue_head_t,
    pub result: gpr_ibasic_rsp_result_t,
    pub lock: mutex,
}

const PRM_CMD_REQUEST_HW_RSC: u32 = 0x0100100F;
const PRM_CMD_RSP_REQUEST_HW_RSC: u32 = 0x02001002;
const PRM_CMD_RELEASE_HW_RSC: u32 = 0x01001010;
const PRM_CMD_RSP_RELEASE_HW_RSC: u32 = 0x02001003;
const PARAM_ID_RSC_HW_CORE: u32 = 0x08001032;
const PARAM_ID_RSC_LPASS_CORE: u32 = 0x0800102B;
const PARAM_ID_RSC_AUDIO_HW_CLK: u32 = 0x0800102C;
const PARAM_ID_RSC_CPU_LPR: u32 = 0x08001A6E;

const LPR_CPU_SS_SLEEP_DISABLE: u32 = 0x1;

#[repr(C)]
pub struct apm_module_param_data {
    pub module_instance_id: u32,
    pub error_code: u32,
    pub param_id: u32,
    pub param_size: u32,
}

#[repr(C)]
pub union prm_cmd_request_hw_core_union {
    pub hw_clk_id: u32,
    pub lpr_state: u32,
}

#[repr(C, packed)]
pub struct prm_cmd_request_hw_core {
    pub param_data: apm_module_param_data,
    pub u: prm_cmd_request_hw_core_union,
}

#[repr(C)]
pub struct audio_hw_clk_cfg {
    pub clock_id: c_int,
    pub clock_freq: c_uint,
    pub clock_attri: c_int,
    pub clock_root: c_int,
}

#[repr(C)]
pub struct audio_hw_clk_rel_cfg {
    pub clock_id: c_int,
}

#[repr(C, packed)]
pub struct prm_cmd_request_rsc {
    pub param_data: apm_module_param_data,
    pub num_clk_id: u32,
    pub clock_id: audio_hw_clk_cfg,
}

#[repr(C, packed)]
pub struct prm_cmd_release_rsc {
    pub param_data: apm_module_param_data,
    pub num_clk_id: u32,
    pub clock_id: audio_hw_clk_rel_cfg,
}

#[repr(C)]
pub struct gpr_pkt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpr_hdr {
    pub opcode: u32,
}

#[repr(C)]
pub struct gpr_resp_pkt {
    pub hdr: gpr_hdr,
    pub payload: *const gpr_ibasic_rsp_result_t,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct gpr_driver_t {
    pub probe: Option<unsafe extern "C" fn(*mut gpr_device_t) -> c_int>,
    pub gpr_callback: Option<unsafe extern "C" fn(*const gpr_resp_pkt, *mut c_void, c_int) -> c_int>,
    pub driver: driver,
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GPR_HDR_SIZE: usize = 0;
const APM_CMD_HDR_SIZE: usize = 0;
const APM_MODULE_PARAM_DATA_SIZE: usize = core::mem::size_of::<apm_module_param_data>();
const GPR_PRM_MODULE_IID: u32 = 0;
const Q6PRM_HW_LPR_VOTE: u32 = 0;

unsafe extern "C" {
    fn audioreach_send_cmd_sync(
        dev: *mut device,
        gdev: *mut gpr_device_t,
        result: *mut gpr_ibasic_rsp_result_t,
        lock: *mut mutex,
        unused: *mut c_void,
        wait: *mut wait_queue_head_t,
        pkt: *mut gpr_pkt,
        rsp_opcode: u32,
    ) -> c_int;
    fn audioreach_alloc_cmd_pkt(
        size: usize,
        opcode: u32,
        token: u32,
        src_port: u32,
        dest_port: u32,
    ) -> *mut gpr_pkt;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    fn q6apm_is_adsp_ready() -> bool;
    fn devm_of_platform_populate(dev: *mut device) -> c_int;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn kfree(ptr: *mut c_void);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
    fn module_gpr_driver(driver: *mut gpr_driver_t);
}

unsafe fn q6prm_send_cmd_sync(prm: *mut q6prm, pkt: *mut gpr_pkt, rsp_opcode: u32) -> c_int {
    unsafe {
        audioreach_send_cmd_sync(
            (*prm).dev,
            (*prm).gdev,
            &mut (*prm).result,
            &mut (*prm).lock,
            core::ptr::null_mut(),
            &mut (*prm).wait,
            pkt,
            rsp_opcode,
        )
    }
}

unsafe fn q6prm_set_hw_core_req(dev: *mut device, hw_block_id: u32, enable: bool) -> c_int {
    let prm = unsafe { dev_get_drvdata((*dev).parent) as *mut q6prm };
    let gdev = unsafe { (*prm).gdev };
    let lpr_req = hw_block_id == Q6PRM_HW_LPR_VOTE;
    let (opcode, rsp_opcode) = if enable {
        (PRM_CMD_REQUEST_HW_RSC, PRM_CMD_RSP_REQUEST_HW_RSC)
    } else {
        (PRM_CMD_RELEASE_HW_RSC, PRM_CMD_RSP_RELEASE_HW_RSC)
    };

    let pkt = unsafe {
        audioreach_alloc_cmd_pkt(
            core::mem::size_of::<prm_cmd_request_hw_core>(),
            opcode,
            0,
            (*gdev).svc.id,
            GPR_PRM_MODULE_IID,
        )
    };
    if unsafe { IS_ERR(pkt as *const c_void) } {
        return unsafe { PTR_ERR(pkt as *const c_void) };
    }

    let req = unsafe {
        (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut prm_cmd_request_hw_core
    };
    let param_data = unsafe { &mut (*req).param_data };

    param_data.module_instance_id = GPR_PRM_MODULE_IID;
    param_data.error_code = 0;
    param_data.param_id = if lpr_req {
        PARAM_ID_RSC_CPU_LPR
    } else {
        PARAM_ID_RSC_HW_CORE
    };
    param_data.param_size =
        (core::mem::size_of::<prm_cmd_request_hw_core>() - APM_MODULE_PARAM_DATA_SIZE) as u32;

    if lpr_req {
        unsafe {
            (*req).u.lpr_state = LPR_CPU_SS_SLEEP_DISABLE;
        }
    } else {
        unsafe {
            (*req).u.hw_clk_id = hw_block_id;
        }
    }

    let ret = unsafe { q6prm_send_cmd_sync(prm, pkt, rsp_opcode) };
    unsafe { kfree(pkt as *mut c_void) };
    ret
}

#[no_mangle]
pub unsafe extern "C" fn q6prm_vote_lpass_core_hw(
    dev: *mut device,
    hw_block_id: u32,
    _client_name: *const c_char,
    _client_handle: *mut u32,
) -> c_int {
    unsafe { q6prm_set_hw_core_req(dev, hw_block_id, true) }
}
// EXPORT_SYMBOL_GPL(q6prm_vote_lpass_core_hw);

#[no_mangle]
pub unsafe extern "C" fn q6prm_unvote_lpass_core_hw(
    dev: *mut device,
    hw_block_id: u32,
    _client_handle: u32,
) -> c_int {
    unsafe { q6prm_set_hw_core_req(dev, hw_block_id, false) }
}
// EXPORT_SYMBOL_GPL(q6prm_unvote_lpass_core_hw);

unsafe fn q6prm_request_lpass_clock(
    dev: *mut device,
    clk_id: c_int,
    clk_attr: c_int,
    clk_root: c_int,
    freq: c_uint,
) -> c_int {
    let prm = unsafe { dev_get_drvdata((*dev).parent) as *mut q6prm };
    let gdev = unsafe { (*prm).gdev };

    let pkt = unsafe {
        audioreach_alloc_cmd_pkt(
            core::mem::size_of::<prm_cmd_request_rsc>(),
            PRM_CMD_REQUEST_HW_RSC,
            0,
            (*gdev).svc.id,
            GPR_PRM_MODULE_IID,
        )
    };
    if unsafe { IS_ERR(pkt as *const c_void) } {
        return unsafe { PTR_ERR(pkt as *const c_void) };
    }

    let req =
        unsafe { (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut prm_cmd_request_rsc };
    let param_data = unsafe { &mut (*req).param_data };

    param_data.module_instance_id = GPR_PRM_MODULE_IID;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_RSC_AUDIO_HW_CLK;
    param_data.param_size =
        (core::mem::size_of::<prm_cmd_request_rsc>() - APM_MODULE_PARAM_DATA_SIZE) as u32;

    unsafe {
        (*req).num_clk_id = 1;
        (*req).clock_id.clock_id = clk_id;
        (*req).clock_id.clock_freq = freq;
        (*req).clock_id.clock_attri = clk_attr;
        (*req).clock_id.clock_root = clk_root;
    }

    let ret = unsafe { q6prm_send_cmd_sync(prm, pkt, PRM_CMD_RSP_REQUEST_HW_RSC) };
    unsafe { kfree(pkt as *mut c_void) };
    ret
}

unsafe fn q6prm_release_lpass_clock(
    dev: *mut device,
    clk_id: c_int,
    _clk_attr: c_int,
    _clk_root: c_int,
    _freq: c_uint,
) -> c_int {
    let prm = unsafe { dev_get_drvdata((*dev).parent) as *mut q6prm };
    let gdev = unsafe { (*prm).gdev };

    let pkt = unsafe {
        audioreach_alloc_cmd_pkt(
            core::mem::size_of::<prm_cmd_release_rsc>(),
            PRM_CMD_RELEASE_HW_RSC,
            0,
            (*gdev).svc.id,
            GPR_PRM_MODULE_IID,
        )
    };
    if unsafe { IS_ERR(pkt as *const c_void) } {
        return unsafe { PTR_ERR(pkt as *const c_void) };
    }

    let rel =
        unsafe { (pkt as *mut u8).add(GPR_HDR_SIZE + APM_CMD_HDR_SIZE) as *mut prm_cmd_release_rsc };
    let param_data = unsafe { &mut (*rel).param_data };

    param_data.module_instance_id = GPR_PRM_MODULE_IID;
    param_data.error_code = 0;
    param_data.param_id = PARAM_ID_RSC_AUDIO_HW_CLK;
    param_data.param_size =
        (core::mem::size_of::<prm_cmd_release_rsc>() - APM_MODULE_PARAM_DATA_SIZE) as u32;

    unsafe {
        (*rel).num_clk_id = 1;
        (*rel).clock_id.clock_id = clk_id;
    }

    let ret = unsafe { q6prm_send_cmd_sync(prm, pkt, PRM_CMD_RSP_RELEASE_HW_RSC) };
    unsafe { kfree(pkt as *mut c_void) };
    ret
}

#[no_mangle]
pub unsafe extern "C" fn q6prm_set_lpass_clock(
    dev: *mut device,
    clk_id: c_int,
    clk_attr: c_int,
    clk_root: c_int,
    freq: c_uint,
) -> c_int {
    if freq != 0 {
        return unsafe { q6prm_request_lpass_clock(dev, clk_id, clk_attr, clk_root, freq) };
    }

    unsafe { q6prm_release_lpass_clock(dev, clk_id, clk_attr, clk_root, freq) }
}
// EXPORT_SYMBOL_GPL(q6prm_set_lpass_clock);

unsafe extern "C" fn prm_callback(
    data: *const gpr_resp_pkt,
    priv_: *mut c_void,
    _op: c_int,
) -> c_int {
    let gdev = priv_ as *mut gpr_device_t;
    let prm = unsafe { dev_get_drvdata(&mut (*gdev).dev) as *mut q6prm };
    let hdr = unsafe { &(*data).hdr };

    match hdr.opcode {
        PRM_CMD_RSP_REQUEST_HW_RSC | PRM_CMD_RSP_RELEASE_HW_RSC => {
            let result = unsafe { (*data).payload };
            unsafe {
                (*prm).result.opcode = hdr.opcode;
                (*prm).result.status = (*result).status;
                wake_up(&mut (*prm).wait);
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn prm_probe(gdev: *mut gpr_device_t) -> c_int {
    let dev = unsafe { &mut (*gdev).dev as *mut device };
    let cc = unsafe { devm_kzalloc(dev, core::mem::size_of::<q6prm>(), GFP_KERNEL) as *mut q6prm };
    if cc.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*cc).dev = dev;
        (*cc).gdev = gdev;
        mutex_init(&mut (*cc).lock);
        init_waitqueue_head(&mut (*cc).wait);
        dev_set_drvdata(dev, cc as *mut c_void);
    }

    if unsafe { !q6apm_is_adsp_ready() } {
        return -EPROBE_DEFER;
    }

    unsafe { devm_of_platform_populate(dev) }
}

// #ifdef CONFIG_OF
#[used]
static PRM_DEVICE_ID_COMPATIBLE: &[u8; 10] = b"qcom,q6prm";

#[used]
static PRM_DEVICE_ID: [of_device_id; 2] = [
    of_device_id {
        compatible: PRM_DEVICE_ID_COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, prm_device_id);
// #endif

#[used]
static mut PRM_DRIVER: gpr_driver_t = gpr_driver_t {
    probe: Some(prm_probe),
    gpr_callback: Some(prm_callback),
    driver: driver {
        name: b"qcom-prm\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(PRM_DEVICE_ID.as_ptr()) },
    },
};

#[no_mangle]
pub unsafe extern "C" fn q6prm_rs_register_driver() {
    unsafe {
        module_gpr_driver(&raw mut PRM_DRIVER);
    }
}

// module_gpr_driver(prm_driver);
// MODULE_DESCRIPTION("Q6 Proxy Resource Manager");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
