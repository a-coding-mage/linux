// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

// Rust translation of soc/qcom/qdsp6/q6core.c.
// External Linux/APR symbols and types are declared as dependencies supplied by
// other translated files.

pub const ADSP_STATE_READY_TIMEOUT_MS: u32 = 3000;
pub const Q6_READY_TIMEOUT_MS: u32 = 100;
pub const AVCS_CMD_ADSP_EVENT_GET_STATE: u32 = 0x0001_290C;
pub const AVCS_CMDRSP_ADSP_EVENT_GET_STATE: u32 = 0x0001_290D;
pub const AVCS_GET_VERSIONS: u32 = 0x0001_2905;
pub const AVCS_GET_VERSIONS_RSP: u32 = 0x0001_2906;
pub const AVCS_CMD_GET_FWK_VERSION: u32 = 0x0001_292c;
pub const AVCS_CMDRSP_GET_FWK_VERSION: u32 = 0x0001_292d;

extern "C" {
    static mut jiffies: c_ulong;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn apr_send_pkt(adev: *mut apr_device, pkt: *mut apr_pkt) -> c_int;
    fn kmemdup(src: *const c_void, len: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn init_waitqueue_head(wq: *mut wait_queue_head_t);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> bool;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn wait_event_timeout_q6core(
        wq: *mut wait_queue_head_t,
        condition: *mut bool,
        timeout: c_ulong,
    ) -> c_int;
}

pub type c_char = i8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type gfp_t = u32;

pub const GFP_ATOMIC: gfp_t = 0;
pub const GFP_KERNEL: gfp_t = 0;
pub const ENOMEM: c_int = 12;
pub const ENOTSUPP: c_int = 524;
pub const ADSP_EUNSUPPORTED: u32 = 0;
pub const APR_BASIC_RSP_RESULT: u32 = 0;
pub const APR_MSG_TYPE_SEQ_CMD: u32 = 0;
pub const APR_HDR_SIZE: u32 = 0;
pub const APR_PKT_VER: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct apr_device {
    pub dev: device,
}

#[repr(C)]
pub struct apr_hdr {
    pub hdr_field: u32,
    pub pkt_size: u32,
    pub opcode: u32,
}

#[repr(C)]
pub struct apr_pkt {
    pub hdr: apr_hdr,
}

#[repr(C)]
pub struct apr_resp_pkt {
    pub hdr: apr_hdr,
    pub payload: *mut c_void,
}

#[repr(C)]
pub struct aprv2_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: u32,
}

#[repr(C)]
pub struct q6core_svc_api_info {
    pub api_version: u32,
    pub api_branch_version: u32,
}

#[repr(C, packed)]
pub struct avcs_svc_info {
    pub service_id: u32,
    pub version: u32,
}

#[repr(C, packed)]
pub struct avcs_cmdrsp_get_version {
    pub build_id: u32,
    pub num_services: u32,
    pub svc_api_info: [avcs_svc_info; 0],
}

/* for ADSP2.8 and above */
#[repr(C, packed)]
pub struct avcs_svc_api_info {
    pub service_id: u32,
    pub api_version: u32,
    pub api_branch_version: u32,
}

#[repr(C, packed)]
pub struct avcs_cmdrsp_get_fwk_version {
    pub build_major_version: u32,
    pub build_minor_version: u32,
    pub build_branch_version: u32,
    pub build_subbranch_version: u32,
    pub num_services: u32,
    pub svc_api_info: [avcs_svc_api_info; 0],
}

#[repr(C)]
pub struct q6core {
    pub adev: *mut apr_device,
    pub wait: wait_queue_head_t,
    pub avcs_state: u32,
    pub lock: mutex,
    pub resp_received: bool,
    pub num_services: u32,
    pub fwk_version: *mut avcs_cmdrsp_get_fwk_version,
    pub svc_version: *mut avcs_cmdrsp_get_version,
    pub fwk_version_supported: bool,
    pub get_state_supported: bool,
    pub get_version_supported: bool,
    pub is_version_requested: bool,
}

static mut g_core: *mut q6core = core::ptr::null_mut();

#[inline]
const fn APR_HDR_LEN(len: u32) -> u32 {
    len
}

#[inline]
const fn APR_HDR_FIELD(msg_type: u32, hdr_len: u32, ver: u32) -> u32 {
    (msg_type) | (hdr_len) | (ver)
}

#[inline]
const fn struct_size_avcs_cmdrsp_get_fwk_version(n: u32) -> usize {
    core::mem::size_of::<avcs_cmdrsp_get_fwk_version>()
        + (n as usize) * core::mem::size_of::<avcs_svc_api_info>()
}

#[inline]
const fn struct_size_avcs_cmdrsp_get_version(n: u32) -> usize {
    core::mem::size_of::<avcs_cmdrsp_get_version>()
        + (n as usize) * core::mem::size_of::<avcs_svc_info>()
}

unsafe extern "C" fn q6core_callback(
    adev: *mut apr_device,
    data: *const apr_resp_pkt,
) -> c_int {
    let core = dev_get_drvdata(core::ptr::addr_of_mut!((*adev).dev)) as *mut q6core;
    let mut result: *const aprv2_ibasic_rsp_result_t;
    let hdr: *const apr_hdr = core::ptr::addr_of!((*data).hdr);

    result = (*data).payload as *const aprv2_ibasic_rsp_result_t;
    match (*hdr).opcode {
        APR_BASIC_RSP_RESULT => {
            result = (*data).payload as *const aprv2_ibasic_rsp_result_t;
            match (*result).opcode {
                AVCS_GET_VERSIONS => {
                    if (*result).status == ADSP_EUNSUPPORTED {
                        (*core).get_version_supported = false;
                    }
                    (*core).resp_received = true;
                }
                AVCS_CMD_GET_FWK_VERSION => {
                    if (*result).status == ADSP_EUNSUPPORTED {
                        (*core).fwk_version_supported = false;
                    }
                    (*core).resp_received = true;
                }
                AVCS_CMD_ADSP_EVENT_GET_STATE => {
                    if (*result).status == ADSP_EUNSUPPORTED {
                        (*core).get_state_supported = false;
                    }
                    (*core).resp_received = true;
                }
                _ => {}
            }
        }
        AVCS_CMDRSP_GET_FWK_VERSION => {
            let fwk: *mut avcs_cmdrsp_get_fwk_version =
                (*data).payload as *mut avcs_cmdrsp_get_fwk_version;

            (*core).fwk_version = kmemdup(
                (*data).payload as *const c_void,
                struct_size_avcs_cmdrsp_get_fwk_version((*fwk).num_services),
                GFP_ATOMIC,
            ) as *mut avcs_cmdrsp_get_fwk_version;
            if (*core).fwk_version.is_null() {
                return -ENOMEM;
            }

            (*core).fwk_version_supported = true;
            (*core).resp_received = true;
        }
        AVCS_GET_VERSIONS_RSP => {
            let v: *mut avcs_cmdrsp_get_version =
                (*data).payload as *mut avcs_cmdrsp_get_version;

            (*core).svc_version = kmemdup(
                (*data).payload as *const c_void,
                struct_size_avcs_cmdrsp_get_version((*v).num_services),
                GFP_ATOMIC,
            ) as *mut avcs_cmdrsp_get_version;
            if (*core).svc_version.is_null() {
                return -ENOMEM;
            }

            (*core).get_version_supported = true;
            (*core).resp_received = true;
        }
        AVCS_CMDRSP_ADSP_EVENT_GET_STATE => {
            (*core).get_state_supported = true;
            (*core).avcs_state = (*result).opcode;

            (*core).resp_received = true;
        }
        _ => {
            dev_err(
                core::ptr::addr_of_mut!((*adev).dev),
                b"Message id from adsp core svc: 0x%x\n\0".as_ptr() as *const c_char,
                (*hdr).opcode,
            );
        }
    }

    if (*core).resp_received {
        wake_up(core::ptr::addr_of_mut!((*core).wait));
    }

    0
}

unsafe fn q6core_get_fwk_versions(core: *mut q6core) -> c_int {
    let adev: *mut apr_device = (*core).adev;
    let mut pkt: apr_pkt = core::mem::zeroed();
    let mut rc: c_int;

    pkt.hdr.hdr_field = APR_HDR_FIELD(
        APR_MSG_TYPE_SEQ_CMD,
        APR_HDR_LEN(APR_HDR_SIZE),
        APR_PKT_VER,
    );
    pkt.hdr.pkt_size = APR_HDR_SIZE;
    pkt.hdr.opcode = AVCS_CMD_GET_FWK_VERSION;

    rc = apr_send_pkt(adev, core::ptr::addr_of_mut!(pkt));
    if rc < 0 {
        return rc;
    }

    rc = wait_event_timeout_q6core(
        core::ptr::addr_of_mut!((*core).wait),
        core::ptr::addr_of_mut!((*core).resp_received),
        msecs_to_jiffies(Q6_READY_TIMEOUT_MS),
    );
    if rc > 0 && (*core).resp_received {
        (*core).resp_received = false;

        if !(*core).fwk_version_supported {
            return -ENOTSUPP;
        } else {
            return 0;
        }
    }

    rc
}

unsafe fn q6core_get_svc_versions(core: *mut q6core) -> c_int {
    let adev: *mut apr_device = (*core).adev;
    let mut pkt: apr_pkt = core::mem::zeroed();
    let mut rc: c_int;

    pkt.hdr.hdr_field = APR_HDR_FIELD(
        APR_MSG_TYPE_SEQ_CMD,
        APR_HDR_LEN(APR_HDR_SIZE),
        APR_PKT_VER,
    );
    pkt.hdr.pkt_size = APR_HDR_SIZE;
    pkt.hdr.opcode = AVCS_GET_VERSIONS;

    rc = apr_send_pkt(adev, core::ptr::addr_of_mut!(pkt));
    if rc < 0 {
        return rc;
    }

    rc = wait_event_timeout_q6core(
        core::ptr::addr_of_mut!((*core).wait),
        core::ptr::addr_of_mut!((*core).resp_received),
        msecs_to_jiffies(Q6_READY_TIMEOUT_MS),
    );
    if rc > 0 && (*core).resp_received {
        (*core).resp_received = false;
        return 0;
    }

    rc
}

unsafe fn __q6core_is_adsp_ready(core: *mut q6core) -> bool {
    let adev: *mut apr_device = (*core).adev;
    let mut pkt: apr_pkt = core::mem::zeroed();
    let mut rc: c_int;

    (*core).get_state_supported = false;

    pkt.hdr.hdr_field = APR_HDR_FIELD(
        APR_MSG_TYPE_SEQ_CMD,
        APR_HDR_LEN(APR_HDR_SIZE),
        APR_PKT_VER,
    );
    pkt.hdr.pkt_size = APR_HDR_SIZE;
    pkt.hdr.opcode = AVCS_CMD_ADSP_EVENT_GET_STATE;

    rc = apr_send_pkt(adev, core::ptr::addr_of_mut!(pkt));
    if rc < 0 {
        return false;
    }

    rc = wait_event_timeout_q6core(
        core::ptr::addr_of_mut!((*core).wait),
        core::ptr::addr_of_mut!((*core).resp_received),
        msecs_to_jiffies(Q6_READY_TIMEOUT_MS),
    );
    if rc > 0 && (*core).resp_received {
        (*core).resp_received = false;

        if (*core).avcs_state != 0 {
            return true;
        }
    }

    /* assume that the adsp is up if we not support this command */
    if !(*core).get_state_supported {
        return true;
    }

    false
}

/**
 * q6core_get_svc_api_info() - Get version number of a service.
 *
 * @svc_id: service id of the service.
 * @ainfo: Valid struct pointer to fill svc api information.
 *
 * Return: zero on success and error code on failure or unsupported
 */
#[no_mangle]
pub unsafe extern "C" fn q6core_get_svc_api_info(
    svc_id: c_int,
    ainfo: *mut q6core_svc_api_info,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int = -ENOTSUPP;

    if g_core.is_null() || ainfo.is_null() {
        return 0;
    }

    mutex_lock(core::ptr::addr_of_mut!((*g_core).lock));
    if !(*g_core).is_version_requested {
        if q6core_get_fwk_versions(g_core) == -ENOTSUPP {
            q6core_get_svc_versions(g_core);
        }
        (*g_core).is_version_requested = true;
    }

    if (*g_core).fwk_version_supported {
        i = 0;
        while i < (*(*g_core).fwk_version).num_services as c_int {
            let info: *mut avcs_svc_api_info =
                (*(*g_core).fwk_version).svc_api_info.as_mut_ptr().add(i as usize);
            if svc_id != (*info).service_id as c_int {
                i += 1;
                continue;
            }

            (*ainfo).api_version = (*info).api_version;
            (*ainfo).api_branch_version = (*info).api_branch_version;
            ret = 0;
            break;
        }
    } else if (*g_core).get_version_supported {
        i = 0;
        while i < (*(*g_core).svc_version).num_services as c_int {
            let info: *mut avcs_svc_info =
                (*(*g_core).svc_version).svc_api_info.as_mut_ptr().add(i as usize);
            if svc_id != (*info).service_id as c_int {
                i += 1;
                continue;
            }

            (*ainfo).api_version = (*info).version;
            (*ainfo).api_branch_version = 0;
            ret = 0;
            break;
        }
    }

    mutex_unlock(core::ptr::addr_of_mut!((*g_core).lock));

    ret
}
// EXPORT_SYMBOL_GPL(q6core_get_svc_api_info);

/**
 * q6core_is_adsp_ready() - Get status of adsp
 *
 * Return: Will be an true if adsp is ready and false if not.
 */
#[no_mangle]
pub unsafe extern "C" fn q6core_is_adsp_ready() -> bool {
    let timeout: c_ulong;
    let mut ret: bool = false;

    if g_core.is_null() {
        return false;
    }

    mutex_lock(core::ptr::addr_of_mut!((*g_core).lock));
    timeout = jiffies.wrapping_add(msecs_to_jiffies(ADSP_STATE_READY_TIMEOUT_MS));
    loop {
        if __q6core_is_adsp_ready(g_core) {
            ret = true;
            break;
        }

        if !time_after(timeout, jiffies) {
            ret = false;
            break;
        }
    }

    mutex_unlock(core::ptr::addr_of_mut!((*g_core).lock));
    ret
}
// EXPORT_SYMBOL_GPL(q6core_is_adsp_ready);

unsafe extern "C" fn q6core_probe(adev: *mut apr_device) -> c_int {
    g_core = kzalloc(core::mem::size_of::<q6core>(), GFP_KERNEL) as *mut q6core;
    if g_core.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(core::ptr::addr_of_mut!((*adev).dev), g_core as *mut c_void);

    mutex_init(core::ptr::addr_of_mut!((*g_core).lock));
    (*g_core).adev = adev;
    init_waitqueue_head(core::ptr::addr_of_mut!((*g_core).wait));
    0
}

unsafe extern "C" fn q6core_exit(adev: *mut apr_device) {
    let core: *mut q6core = dev_get_drvdata(core::ptr::addr_of_mut!((*adev).dev)) as *mut q6core;

    if (*core).fwk_version_supported {
        kfree((*core).fwk_version as *mut c_void);
    }
    if (*core).get_version_supported {
        kfree((*core).svc_version as *mut c_void);
    }

    g_core = core::ptr::null_mut();
    kfree(core as *mut c_void);
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
pub struct apr_driver {
    pub probe: Option<unsafe extern "C" fn(*mut apr_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut apr_device)>,
    pub callback: Option<unsafe extern "C" fn(*mut apr_device, *const apr_resp_pkt) -> c_int>,
    pub driver: driver,
}

// #ifdef CONFIG_OF
static q6core_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"qcom,q6core\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, q6core_device_id);
// #endif

static mut qcom_q6core_driver: apr_driver = apr_driver {
    probe: Some(q6core_probe),
    remove: Some(q6core_exit),
    callback: Some(q6core_callback),
    driver: driver {
        name: b"qcom-q6core\0".as_ptr() as *const c_char,
        of_match_table: q6core_device_id.as_ptr(),
    },
};

// module_apr_driver(qcom_q6core_driver);
// MODULE_DESCRIPTION("q6 core");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
