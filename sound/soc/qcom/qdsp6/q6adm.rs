// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2011-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2018, Linaro Limited

// Rust translation of q6adm.c. C include dependencies are expected to be
// supplied by the surrounding kernel/audio/APR bindings.

pub const ADM_CMD_DEVICE_OPEN_V5: u32 = 0x00010326;
pub const ADM_CMDRSP_DEVICE_OPEN_V5: u32 = 0x00010329;
pub const ADM_CMD_DEVICE_CLOSE_V5: u32 = 0x00010327;
pub const ADM_CMD_MATRIX_MAP_ROUTINGS_V5: u32 = 0x00010325;

pub const TIMEOUT_MS: u32 = 1000;
pub const RESET_COPP_ID: i32 = 99;
pub const INVALID_COPP_ID: u16 = 0xFF;
/* Definition for a legacy device session. */
pub const ADM_LEGACY_DEVICE_SESSION: u16 = 0;
pub const ADM_MATRIX_ID_AUDIO_RX: u32 = 0;
pub const ADM_MATRIX_ID_AUDIO_TX: u32 = 1;

#[repr(C)]
pub struct q6copp {
    pub afe_port: c_int,
    pub copp_idx: c_int,
    pub id: c_int,
    pub topology: c_int,
    pub mode: c_int,
    pub rate: c_int,
    pub bit_width: c_int,
    pub channels: c_int,
    pub app_type: c_int,
    pub acdb_id: c_int,

    pub result: aprv2_ibasic_rsp_result_t,
    pub refcount: kref,
    pub wait: wait_queue_head_t,
    pub node: list_head,
    pub adm: *mut q6adm,
}

#[repr(C)]
pub struct q6adm {
    pub apr: *mut apr_device,
    pub dev: *mut device,
    pub ainfo: q6core_svc_api_info,
    pub copp_bitmap: [c_ulong; AFE_MAX_PORTS],
    pub copps_list: list_head,
    pub copps_list_lock: spinlock_t,
    pub result: aprv2_ibasic_rsp_result_t,
    pub lock: mutex,
    pub matrix_map_wait: wait_queue_head_t,
}

#[repr(C, packed)]
pub struct q6adm_cmd_device_open_v5 {
    pub flags: u16,
    pub mode_of_operation: u16,
    pub endpoint_id_1: u16,
    pub endpoint_id_2: u16,
    pub topology_id: u32,
    pub dev_num_channel: u16,
    pub bit_width: u16,
    pub sample_rate: u32,
    pub dev_channel_mapping: [u8; 8],
}

#[repr(C, packed)]
pub struct q6adm_cmd_matrix_map_routings_v5 {
    pub matrix_id: u32,
    pub num_sessions: u32,
}

#[repr(C, packed)]
pub struct q6adm_session_map_node_v5 {
    pub session_id: u16,
    pub num_copps: u16,
}

#[repr(C, packed)]
pub struct adm_cmd_rsp_device_open_v5 {
    pub status: u32,
    pub copp_id: u16,
    pub reserved: u16,
}

extern "C" {
    pub static q6adm_device_id: [of_device_id; 2];
    pub static mut qcom_q6adm_driver: apr_driver;

    pub fn q6afe_get_port_id(port_id: c_int) -> c_int;
    pub fn q6dsp_map_channels(map: *mut u8, channel_mode: c_int) -> c_int;
    pub fn q6core_get_svc_api_info(svc_id: c_uint, info: *mut q6core_svc_api_info);
    pub fn apr_send_pkt(apr: *mut apr_device, pkt: *mut apr_pkt) -> c_int;
    pub fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    pub fn devm_of_platform_populate(dev: *mut device) -> c_int;
    pub fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_unlock(lock: *mut mutex);
    pub fn mutex_init(lock: *mut mutex);
    pub fn init_waitqueue_head(wait: *mut wait_queue_head_t);
    pub fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    pub fn spin_lock_init(lock: *mut spinlock_t);
    pub fn find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong;
    pub fn set_bit(nr: c_int, addr: *mut c_ulong);
    pub fn clear_bit(nr: c_int, addr: *mut c_ulong);
    pub fn kref_get(kref: *mut kref);
    pub fn kref_init(kref: *mut kref);
    pub fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref)) -> c_int;
    pub fn wake_up(wait: *mut wait_queue_head_t);
    pub fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    pub fn wait_event_timeout(wait: *mut wait_queue_head_t, condition: bool, timeout: c_ulong) -> c_long;
    pub fn INIT_LIST_HEAD(list: *mut list_head);
    pub fn list_add_tail(new: *mut list_head, head: *mut list_head);
    pub fn list_del(entry: *mut list_head);
    pub fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
    pub fn module_apr_driver(driver: *mut apr_driver);
    pub fn dev_err(dev: *const device, fmt: *const c_char, ...);
}

unsafe fn list_entry_q6copp(ptr: *mut list_head) -> *mut q6copp {
    container_of_q6copp_node(ptr)
}

unsafe fn for_each_copp<F>(adm: *mut q6adm, mut f: F)
where
    F: FnMut(*mut q6copp) -> bool,
{
    let mut pos = (*adm).copps_list.next;
    while pos != &mut (*adm).copps_list as *mut list_head {
        let c = list_entry_q6copp(pos);
        pos = (*pos).next;
        if !f(c) {
            break;
        }
    }
}

unsafe extern "C" fn q6adm_find_copp(adm: *mut q6adm, port_idx: c_int, copp_idx: c_int) -> *mut q6copp {
    let mut ret: *mut q6copp = core::ptr::null_mut();
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*adm).copps_list_lock, &mut flags);
    for_each_copp(adm, |c| {
        if port_idx == (*c).afe_port && copp_idx == (*c).copp_idx {
            ret = c;
            kref_get(&mut (*c).refcount);
            false
        } else {
            true
        }
    });
    spin_unlock_irqrestore(&mut (*adm).copps_list_lock, flags);

    ret
}

unsafe extern "C" fn q6adm_apr_send_copp_pkt(
    adm: *mut q6adm,
    copp: *mut q6copp,
    pkt: *mut apr_pkt,
    rsp_opcode: u32,
) -> c_int {
    let dev = (*adm).dev;
    let opcode = (*pkt).hdr.opcode;
    let mut ret: c_int;

    mutex_lock(&mut (*adm).lock);
    (*copp).result.opcode = 0;
    (*copp).result.status = 0;
    ret = apr_send_pkt((*adm).apr, pkt);
    if ret < 0 {
        dev_err(dev, b"Failed to send APR packet\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
        mutex_unlock(&mut (*adm).lock);
        return ret;
    }

    /* Wait for the callback with copp id */
    if rsp_opcode != 0 {
        ret = wait_event_timeout(
            &mut (*copp).wait,
            (*copp).result.opcode == opcode || (*copp).result.opcode == rsp_opcode,
            msecs_to_jiffies(TIMEOUT_MS),
        ) as c_int;
    } else {
        ret = wait_event_timeout(
            &mut (*copp).wait,
            (*copp).result.opcode == opcode,
            msecs_to_jiffies(TIMEOUT_MS),
        ) as c_int;
    }

    if ret == 0 {
        dev_err(dev, b"ADM copp cmd timedout\n\0".as_ptr() as *const c_char);
        ret = -ETIMEDOUT;
    } else if (*copp).result.status > 0 {
        dev_err(
            dev,
            b"DSP returned error[%d]\n\0".as_ptr() as *const c_char,
            (*copp).result.status,
        );
        ret = -EINVAL;
    }

    mutex_unlock(&mut (*adm).lock);
    ret
}

unsafe extern "C" fn q6adm_device_close(
    adm: *mut q6adm,
    copp: *mut q6copp,
    port_id: c_int,
    copp_idx: c_int,
) -> c_int {
    let mut close: apr_pkt = core::mem::zeroed();

    close.hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    close.hdr.pkt_size = core::mem::size_of_val(&close) as u32;
    close.hdr.src_port = port_id as u16;
    close.hdr.dest_port = (*copp).id as u16;
    close.hdr.token = ((port_id << 16) | copp_idx) as u32;
    close.hdr.opcode = ADM_CMD_DEVICE_CLOSE_V5;

    q6adm_apr_send_copp_pkt(adm, copp, &mut close, 0)
}

unsafe extern "C" fn q6adm_free_copp(ref_: *mut kref) {
    let c = container_of_q6copp_refcount(ref_);
    let adm = (*c).adm;
    let mut flags: c_ulong = 0;

    let ret = q6adm_device_close(adm, c, (*c).afe_port, (*c).copp_idx);
    if ret < 0 {
        dev_err(
            (*adm).dev,
            b"Failed to close copp %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    spin_lock_irqsave(&mut (*adm).copps_list_lock, &mut flags);
    clear_bit((*c).copp_idx, &mut (*adm).copp_bitmap[(*c).afe_port as usize]);
    list_del(&mut (*c).node);
    spin_unlock_irqrestore(&mut (*adm).copps_list_lock, flags);
    kfree(c as *mut c_void);
}

unsafe extern "C" fn q6adm_callback(adev: *mut apr_device, data: *const apr_resp_pkt) -> c_int {
    let result = (*data).payload as *const aprv2_ibasic_rsp_result_t;
    let mut port_idx: c_int;
    let mut copp_idx: c_int;
    let hdr = &(*data).hdr as *const apr_hdr;
    let mut copp: *mut q6copp;
    let adm = dev_get_drvdata(&(*adev).dev) as *mut q6adm;

    if (*data).payload_size == 0 {
        return 0;
    }

    copp_idx = ((*hdr).token & 0xFF) as c_int;
    port_idx = (((*hdr).token >> 16) & 0xFF) as c_int;
    if port_idx < 0 || port_idx >= AFE_MAX_PORTS as c_int {
        dev_err(
            &(*adev).dev,
            b"Invalid port idx %d token %d\n\0".as_ptr() as *const c_char,
            port_idx,
            (*hdr).token,
        );
        return 0;
    }
    if copp_idx < 0 || copp_idx >= MAX_COPPS_PER_PORT as c_int {
        dev_err(
            &(*adev).dev,
            b"Invalid copp idx %d token %d\n\0".as_ptr() as *const c_char,
            copp_idx,
            (*hdr).token,
        );
        return 0;
    }

    match (*hdr).opcode {
        APR_BASIC_RSP_RESULT => {
            if (*result).status != 0 {
                dev_err(
                    &(*adev).dev,
                    b"cmd = 0x%x return error = 0x%x\n\0".as_ptr() as *const c_char,
                    (*result).opcode,
                    (*result).status,
                );
            }
            match (*result).opcode {
                ADM_CMD_DEVICE_OPEN_V5 | ADM_CMD_DEVICE_CLOSE_V5 => {
                    for_each_copp(adm, |c| {
                        if port_idx == (*c).afe_port && copp_idx == (*c).copp_idx {
                            (*c).result = *result;
                            wake_up(&mut (*c).wait);
                            false
                        } else {
                            true
                        }
                    });
                }
                ADM_CMD_MATRIX_MAP_ROUTINGS_V5 => {
                    (*adm).result = *result;
                    wake_up(&mut (*adm).matrix_map_wait);
                }
                _ => {
                    dev_err(
                        &(*adev).dev,
                        b"Unknown Cmd: 0x%x\n\0".as_ptr() as *const c_char,
                        (*result).opcode,
                    );
                }
            }
            return 0;
        }
        ADM_CMDRSP_DEVICE_OPEN_V5 => {
            let open = (*data).payload as *mut adm_cmd_rsp_device_open_v5;

            copp = q6adm_find_copp(adm, port_idx, copp_idx);
            if copp.is_null() {
                return 0;
            }

            if (*open).copp_id == INVALID_COPP_ID {
                dev_err(
                    &(*adev).dev,
                    b"Invalid coppid rxed %d\n\0".as_ptr() as *const c_char,
                    (*open).copp_id as c_int,
                );
                (*copp).result.status = ADSP_EBADPARAM;
                wake_up(&mut (*copp).wait);
                kref_put(&mut (*copp).refcount, q6adm_free_copp);
            } else {
                (*copp).result.opcode = (*hdr).opcode;
                (*copp).id = (*open).copp_id as c_int;
                wake_up(&mut (*copp).wait);
                kref_put(&mut (*copp).refcount, q6adm_free_copp);
            }
        }
        _ => {
            dev_err(
                &(*adev).dev,
                b"Unknown cmd:0x%x\n\0".as_ptr() as *const c_char,
                (*hdr).opcode,
            );
        }
    }

    0
}

unsafe extern "C" fn q6adm_alloc_copp(adm: *mut q6adm, port_idx: c_int) -> *mut q6copp {
    let idx = find_first_zero_bit(
        &(*adm).copp_bitmap[port_idx as usize],
        MAX_COPPS_PER_PORT as c_ulong,
    ) as c_int;

    if idx >= MAX_COPPS_PER_PORT as c_int {
        return ERR_PTR(-EBUSY) as *mut q6copp;
    }

    let c = kzalloc(core::mem::size_of::<q6copp>(), GFP_ATOMIC) as *mut q6copp;
    if c.is_null() {
        return ERR_PTR(-ENOMEM) as *mut q6copp;
    }

    set_bit(idx, &mut (*adm).copp_bitmap[port_idx as usize]);
    (*c).copp_idx = idx;
    (*c).afe_port = port_idx;
    (*c).adm = adm;

    init_waitqueue_head(&mut (*c).wait);

    c
}

unsafe extern "C" fn q6adm_find_matching_copp(
    adm: *mut q6adm,
    port_id: c_int,
    topology: c_int,
    mode: c_int,
    rate: c_int,
    channel_mode: c_int,
    bit_width: c_int,
    app_type: c_int,
) -> *mut q6copp {
    let mut ret: *mut q6copp = core::ptr::null_mut();
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*adm).copps_list_lock, &mut flags);

    for_each_copp(adm, |c| {
        if port_id == (*c).afe_port
            && topology == (*c).topology
            && mode == (*c).mode
            && rate == (*c).rate
            && bit_width == (*c).bit_width
            && app_type == (*c).app_type
        {
            ret = c;
            kref_get(&mut (*c).refcount);
        }
        true
    });
    spin_unlock_irqrestore(&mut (*adm).copps_list_lock, flags);

    ret
}

unsafe extern "C" fn q6adm_device_open(
    adm: *mut q6adm,
    copp: *mut q6copp,
    port_id: c_int,
    path: c_int,
    topology: c_int,
    channel_mode: c_int,
    bit_width: c_int,
    rate: c_int,
) -> c_int {
    let mut open: *mut q6adm_cmd_device_open_v5;
    let afe_port = q6afe_get_port_id(port_id);
    let mut pkt: *mut apr_pkt;
    let pkt_size = (APR_HDR_SIZE + core::mem::size_of::<q6adm_cmd_device_open_v5>()) as c_int;

    let p = kzalloc(pkt_size as usize, GFP_KERNEL);
    if p.is_null() {
        return -ENOMEM;
    }

    pkt = p as *mut apr_pkt;
    open = (p as *mut u8).add(APR_HDR_SIZE) as *mut q6adm_cmd_device_open_v5;
    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32;
    (*pkt).hdr.src_port = afe_port as u16;
    (*pkt).hdr.dest_port = afe_port as u16;
    (*pkt).hdr.token = ((port_id << 16) | (*copp).copp_idx) as u32;
    (*pkt).hdr.opcode = ADM_CMD_DEVICE_OPEN_V5;
    (*open).flags = ADM_LEGACY_DEVICE_SESSION;
    (*open).mode_of_operation = path as u16;
    (*open).endpoint_id_1 = afe_port as u16;
    (*open).topology_id = topology as u32;
    (*open).dev_num_channel = (channel_mode & 0x00FF) as u16;
    (*open).bit_width = bit_width as u16;
    (*open).sample_rate = rate as u32;

    let ret = q6dsp_map_channels((*open).dev_channel_mapping.as_mut_ptr(), channel_mode);
    if ret != 0 {
        kfree(p);
        return ret;
    }

    let ret = q6adm_apr_send_copp_pkt(adm, copp, pkt, ADM_CMDRSP_DEVICE_OPEN_V5);
    kfree(p);
    ret
}

/**
 * q6adm_open() - open adm and grab a free copp
 *
 * @dev: Pointer to adm child device.
 * @port_id: port id
 * @path: playback or capture path.
 * @rate: rate at which copp is required.
 * @channel_mode: channel mode
 * @topology: adm topology id
 * @perf_mode: performace mode.
 * @bit_width: audio sample bit width
 * @app_type: Application type.
 * @acdb_id: ACDB id
 *
 * Return: Will be an negative on error or a valid copp pointer on success.
 */
#[no_mangle]
pub unsafe extern "C" fn q6adm_open(
    dev: *mut device,
    port_id: c_int,
    path: c_int,
    rate: c_int,
    channel_mode: c_int,
    topology: c_int,
    perf_mode: c_int,
    bit_width: u16,
    app_type: c_int,
    acdb_id: c_int,
) -> *mut q6copp {
    let adm = dev_get_drvdata((*dev).parent) as *mut q6adm;
    let mut copp: *mut q6copp;
    let mut flags: c_ulong = 0;
    let mut ret: c_int = 0;

    if port_id < 0 {
        dev_err(dev, b"Invalid port_id %d\n\0".as_ptr() as *const c_char, port_id);
        return ERR_PTR(-EINVAL) as *mut q6copp;
    }

    copp = q6adm_find_matching_copp(
        adm,
        port_id,
        topology,
        perf_mode,
        rate,
        channel_mode,
        bit_width as c_int,
        app_type,
    );
    if !copp.is_null() {
        dev_err(
            dev,
            b"Found Matching Copp 0x%x\n\0".as_ptr() as *const c_char,
            (*copp).copp_idx,
        );
        return copp;
    }

    spin_lock_irqsave(&mut (*adm).copps_list_lock, &mut flags);
    copp = q6adm_alloc_copp(adm, port_id);
    if IS_ERR(copp as *const c_void) {
        spin_unlock_irqrestore(&mut (*adm).copps_list_lock, flags);
        return ERR_CAST(copp as *const c_void) as *mut q6copp;
    }

    list_add_tail(&mut (*copp).node, &mut (*adm).copps_list);
    spin_unlock_irqrestore(&mut (*adm).copps_list_lock, flags);

    kref_init(&mut (*copp).refcount);
    (*copp).topology = topology;
    (*copp).mode = perf_mode;
    (*copp).rate = rate;
    (*copp).channels = channel_mode;
    (*copp).bit_width = bit_width as c_int;
    (*copp).app_type = app_type;

    ret = q6adm_device_open(adm, copp, port_id, path, topology, channel_mode, bit_width as c_int, rate);
    if ret < 0 {
        kref_put(&mut (*copp).refcount, q6adm_free_copp);
        return ERR_PTR(ret) as *mut q6copp;
    }

    copp
}
// EXPORT_SYMBOL_GPL(q6adm_open);

/**
 * q6adm_get_copp_id() - get copp index
 *
 * @copp: Pointer to valid copp
 *
 * Return: Will be an negative on error or a valid copp index on success.
 **/
#[no_mangle]
pub unsafe extern "C" fn q6adm_get_copp_id(copp: *mut q6copp) -> c_int {
    if copp.is_null() {
        return -EINVAL;
    }

    (*copp).copp_idx
}
// EXPORT_SYMBOL_GPL(q6adm_get_copp_id);

/**
 * q6adm_matrix_map() - Map asm streams and afe ports using payload
 *
 * @dev: Pointer to adm child device.
 * @path: playback or capture path.
 * @payload_map: map between session id and afe ports.
 * @perf_mode: Performace mode.
 *
 * Return: Will be an negative on error or a zero on success.
 */
#[no_mangle]
pub unsafe extern "C" fn q6adm_matrix_map(
    dev: *mut device,
    path: c_int,
    payload_map: route_payload,
    perf_mode: c_int,
) -> c_int {
    let adm = dev_get_drvdata((*dev).parent) as *mut q6adm;
    let mut route: *mut q6adm_cmd_matrix_map_routings_v5;
    let mut node: *mut q6adm_session_map_node_v5;
    let mut pkt: *mut apr_pkt;
    let mut copps_list: *mut u16;
    let mut copp_idx: c_int;
    /* Assumes port_ids have already been validated during adm_open */
    let mut copp: *mut q6copp;
    let pkt_size = APR_HDR_SIZE
        + core::mem::size_of::<q6adm_cmd_matrix_map_routings_v5>()
        + core::mem::size_of::<q6adm_session_map_node_v5>()
        + core::mem::size_of::<u32>() * payload_map.num_copps as usize;

    let matrix_map = kzalloc(pkt_size, GFP_KERNEL);
    if matrix_map.is_null() {
        return -ENOMEM;
    }

    pkt = matrix_map as *mut apr_pkt;
    route = (matrix_map as *mut u8).add(APR_HDR_SIZE) as *mut q6adm_cmd_matrix_map_routings_v5;
    node = (matrix_map as *mut u8)
        .add(APR_HDR_SIZE + core::mem::size_of::<q6adm_cmd_matrix_map_routings_v5>())
        as *mut q6adm_session_map_node_v5;
    copps_list = (matrix_map as *mut u8)
        .add(
            APR_HDR_SIZE
                + core::mem::size_of::<q6adm_cmd_matrix_map_routings_v5>()
                + core::mem::size_of::<q6adm_session_map_node_v5>(),
        ) as *mut u16;

    (*pkt).hdr.hdr_field = APR_HDR_FIELD(APR_MSG_TYPE_SEQ_CMD, APR_HDR_LEN(APR_HDR_SIZE), APR_PKT_VER);
    (*pkt).hdr.pkt_size = pkt_size as u32;
    (*pkt).hdr.token = 0;
    (*pkt).hdr.opcode = ADM_CMD_MATRIX_MAP_ROUTINGS_V5;
    (*route).num_sessions = 1;

    match path {
        ADM_PATH_PLAYBACK => {
            (*route).matrix_id = ADM_MATRIX_ID_AUDIO_RX;
        }
        ADM_PATH_LIVE_REC => {
            (*route).matrix_id = ADM_MATRIX_ID_AUDIO_TX;
        }
        _ => {
            dev_err(dev, b"Wrong path set[%d]\n\0".as_ptr() as *const c_char, path);
        }
    }

    (*node).session_id = payload_map.session_id;
    (*node).num_copps = payload_map.num_copps;

    let mut i: c_int = 0;
    while i < payload_map.num_copps as c_int {
        let port_idx = payload_map.port_id[i as usize];

        if port_idx < 0 {
            dev_err(
                dev,
                b"Invalid port_id %d\n\0".as_ptr() as *const c_char,
                payload_map.port_id[i as usize],
            );
            kfree(matrix_map);
            return -EINVAL;
        }
        copp_idx = payload_map.copp_idx[i as usize];

        copp = q6adm_find_copp(adm, port_idx, copp_idx);
        if copp.is_null() {
            kfree(matrix_map);
            return -EINVAL;
        }

        *copps_list.add(i as usize) = (*copp).id as u16;
        kref_put(&mut (*copp).refcount, q6adm_free_copp);
        i += 1;
    }

    mutex_lock(&mut (*adm).lock);
    (*adm).result.status = 0;
    (*adm).result.opcode = 0;

    let mut ret = apr_send_pkt((*adm).apr, pkt);
    if ret < 0 {
        dev_err(
            dev,
            b"routing for stream %d failed ret %d\n\0".as_ptr() as *const c_char,
            payload_map.session_id as c_int,
            ret,
        );
        mutex_unlock(&mut (*adm).lock);
        kfree(matrix_map);
        return ret;
    }
    ret = wait_event_timeout(
        &mut (*adm).matrix_map_wait,
        (*adm).result.opcode == (*pkt).hdr.opcode,
        msecs_to_jiffies(TIMEOUT_MS),
    ) as c_int;
    if ret == 0 {
        dev_err(
            dev,
            b"routing for stream %d failed\n\0".as_ptr() as *const c_char,
            payload_map.session_id as c_int,
        );
        ret = -ETIMEDOUT;
    } else if (*adm).result.status > 0 {
        dev_err(
            dev,
            b"DSP returned error[%d]\n\0".as_ptr() as *const c_char,
            (*adm).result.status,
        );
        ret = -EINVAL;
    }

    mutex_unlock(&mut (*adm).lock);
    kfree(matrix_map);
    ret
}
// EXPORT_SYMBOL_GPL(q6adm_matrix_map);

/**
 * q6adm_close() - Close adm copp
 *
 * @dev: Pointer to adm child device.
 * @copp: pointer to previously opened copp
 *
 * Return: Will be an negative on error or a zero on success.
 */
#[no_mangle]
pub unsafe extern "C" fn q6adm_close(dev: *mut device, copp: *mut q6copp) -> c_int {
    kref_put(&mut (*copp).refcount, q6adm_free_copp);

    0
}
// EXPORT_SYMBOL_GPL(q6adm_close);

unsafe extern "C" fn q6adm_probe(adev: *mut apr_device) -> c_int {
    let dev = &mut (*adev).dev as *mut device;
    let mut adm: *mut q6adm;

    adm = devm_kzalloc(dev, core::mem::size_of::<q6adm>(), GFP_KERNEL) as *mut q6adm;
    if adm.is_null() {
        return -ENOMEM;
    }

    (*adm).apr = adev;
    dev_set_drvdata(dev, adm as *mut c_void);
    (*adm).dev = dev;
    q6core_get_svc_api_info((*adev).svc_id, &mut (*adm).ainfo);
    mutex_init(&mut (*adm).lock);
    init_waitqueue_head(&mut (*adm).matrix_map_wait);

    INIT_LIST_HEAD(&mut (*adm).copps_list);
    spin_lock_init(&mut (*adm).copps_list_lock);

    devm_of_platform_populate(dev)
}

// CONFIG_OF:
// static const struct of_device_id q6adm_device_id[] = {
//     { .compatible = "qcom,q6adm" },
//     {},
// };
// MODULE_DEVICE_TABLE(of, q6adm_device_id);

// static struct apr_driver qcom_q6adm_driver = {
//     .probe = q6adm_probe,
//     .callback = q6adm_callback,
//     .driver = {
//         .name = "qcom-q6adm",
//         .of_match_table = of_match_ptr(q6adm_device_id),
//     },
// };

// module_apr_driver(qcom_q6adm_driver);
// MODULE_DESCRIPTION("Q6 Audio Device Manager");
// MODULE_LICENSE("GPL v2");


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
