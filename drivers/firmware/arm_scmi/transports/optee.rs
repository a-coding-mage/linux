// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019-2021 Linaro Ltd. */

// Linux/TEE symbols below are supplied by the surrounding kernel bindings.

#[repr(u32)]
enum ScmiOpteePtaCmd {
    PtaScmiCmdCapabilities = 0,
    PtaScmiCmdProcessSmtChannel = 1,
    PtaScmiCmdProcessSmtChannelMessage = 2,
    PtaScmiCmdGetChannel = 3,
    PtaScmiCmdProcessMsgChannel = 4,
}

const PTA_SCMI_CAPS_NONE: u32 = 0;
const PTA_SCMI_CAPS_SMT_HEADER: u32 = 1 << 0;
const PTA_SCMI_CAPS_MSG_HEADER: u32 = 1 << 1;
const PTA_SCMI_CAPS_MASK: u32 = PTA_SCMI_CAPS_SMT_HEADER | PTA_SCMI_CAPS_MSG_HEADER;

#[repr(C)]
union ScmiOpteeReq {
    shmem: *mut ScmiSharedMem,
    msg: *mut ScmiMsgPayld,
}

#[repr(C)]
struct ScmiOpteeChannel {
    channel_id: u32,
    tee_session: u32,
    caps: u32,
    rx_len: u32,
    mu: Mutex,
    cinfo: *mut ScmiChanInfo,
    req: ScmiOpteeReq,
    io_ops: *mut ScmiShmemIoOps,
    tee_shm: *mut TeeShm,
    link: ListHead,
}

#[repr(C)]
struct ScmiOpteeAgent {
    dev: *mut Device,
    tee_ctx: *mut TeeContext,
    caps: u32,
    mu: Mutex,
    channel_list: ListHead,
}

static mut CORE: *mut ScmiTransportCoreOperations = core::ptr::null_mut();
static mut SCMI_OPTEE_PRIVATE: *mut ScmiOpteeAgent = core::ptr::null_mut();
static mut SCMI_OPTEE_SUPPLIER: ScmiTransportSupplier = ScmiTransportSupplier::default();

unsafe fn open_session(agent: *mut ScmiOpteeAgent, tee_session: *mut u32) -> i32 {
    let dev = (*agent).dev;
    let scmi_pta = to_tee_client_device(dev);
    let mut arg: TeeIoctlOpenSessionArg = core::mem::zeroed();
    core::ptr::copy_nonoverlapping((*scmi_pta).id.uuid.b.as_ptr(), arg.uuid.as_mut_ptr(), TEE_IOCTL_UUID_LEN);
    arg.clnt_login = TEE_IOCTL_LOGIN_REE_KERNEL;
    let ret = tee_client_open_session((*agent).tee_ctx, &mut arg, core::ptr::null_mut());
    if ret < 0 || arg.ret != 0 {
        dev_err(dev, "Can't open tee session: %d / %#x\n", ret, arg.ret);
        return -EOPNOTSUPP;
    }
    *tee_session = arg.session;
    0
}

unsafe fn close_session(agent: *mut ScmiOpteeAgent, tee_session: u32) {
    tee_client_close_session((*agent).tee_ctx, tee_session);
}

unsafe fn get_capabilities(agent: *mut ScmiOpteeAgent) -> i32 {
    let mut arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; 1] = core::mem::zeroed();
    let mut tee_session = 0;
    let ret = open_session(agent, &mut tee_session);
    if ret != 0 { return ret; }
    arg.func = ScmiOpteePtaCmd::PtaScmiCmdCapabilities as u32;
    arg.session = tee_session;
    arg.num_params = 1;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT;
    let ret = tee_client_invoke_func((*agent).tee_ctx, &mut arg, param.as_mut_ptr());
    close_session(agent, tee_session);
    if ret < 0 || arg.ret != 0 {
        dev_err((*agent).dev, "Can't get capabilities: %d / %#x\n", ret, arg.ret);
        return -EOPNOTSUPP;
    }
    let caps = param[0].u.value.a;
    if caps & (PTA_SCMI_CAPS_SMT_HEADER | PTA_SCMI_CAPS_MSG_HEADER) == 0 {
        dev_err((*agent).dev, "OP-TEE SCMI PTA doesn't support SMT and MSG\n");
        return -EOPNOTSUPP;
    }
    (*agent).caps = caps;
    0
}

unsafe fn get_channel(channel: *mut ScmiOpteeChannel) -> i32 {
    let dev = (*SCMI_OPTEE_PRIVATE).dev;
    let mut arg: TeeIoctlInvokeArg = core::mem::zeroed();
    let mut param: [TeeParam; 1] = core::mem::zeroed();
    let caps = if !(*channel).tee_shm.is_null() { PTA_SCMI_CAPS_MSG_HEADER } else { PTA_SCMI_CAPS_SMT_HEADER };
    arg.func = ScmiOpteePtaCmd::PtaScmiCmdGetChannel as u32;
    arg.session = (*channel).tee_session;
    arg.num_params = 1;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INOUT;
    param[0].u.value.a = (*channel).channel_id;
    param[0].u.value.b = caps;
    let ret = tee_client_invoke_func((*SCMI_OPTEE_PRIVATE).tee_ctx, &mut arg, param.as_mut_ptr());
    if ret != 0 || arg.ret != 0 {
        dev_err(dev, "Can't get channel with caps %#x: %d / %#x\n", caps, ret, arg.ret);
        return -EOPNOTSUPP;
    }
    (*channel).channel_id = param[0].u.value.a;
    (*channel).caps = caps;
    0
}

unsafe fn invoke_process_smt_channel(channel: *mut ScmiOpteeChannel) -> i32 {
    let mut arg = TeeIoctlInvokeArg { func: ScmiOpteePtaCmd::PtaScmiCmdProcessSmtChannel as u32, session: (*channel).tee_session, num_params: 1, ..core::mem::zeroed() };
    let mut param: [TeeParam; 1] = core::mem::zeroed();
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = (*channel).channel_id;
    let ret = tee_client_invoke_func((*SCMI_OPTEE_PRIVATE).tee_ctx, &mut arg, param.as_mut_ptr());
    if ret < 0 || arg.ret != 0 { dev_err((*SCMI_OPTEE_PRIVATE).dev, "Can't invoke channel %u: %d / %#x\n", (*channel).channel_id, ret, arg.ret); return -EIO; }
    0
}

unsafe fn invoke_process_msg_channel(channel: *mut ScmiOpteeChannel, msg_size: usize) -> i32 {
    let mut arg = TeeIoctlInvokeArg { func: ScmiOpteePtaCmd::PtaScmiCmdProcessMsgChannel as u32, session: (*channel).tee_session, num_params: 3, ..core::mem::zeroed() };
    let mut param: [TeeParam; 3] = core::mem::zeroed();
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = (*channel).channel_id;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT;
    param[1].u.memref.shm = (*channel).tee_shm;
    param[1].u.memref.size = msg_size;
    param[2].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT;
    param[2].u.memref.shm = (*channel).tee_shm;
    param[2].u.memref.size = SCMI_SHMEM_MAX_PAYLOAD_SIZE;
    let ret = tee_client_invoke_func((*SCMI_OPTEE_PRIVATE).tee_ctx, &mut arg, param.as_mut_ptr());
    if ret < 0 || arg.ret != 0 { dev_err((*SCMI_OPTEE_PRIVATE).dev, "Can't invoke channel %u: %d / %#x\n", (*channel).channel_id, ret, arg.ret); return -EIO; }
    (*channel).rx_len = param[2].u.memref.size as u32;
    0
}

unsafe fn scmi_optee_chan_available(of_node: *mut DeviceNode, idx: i32) -> bool {
    let mut channel_id = 0u32;
    of_property_read_u32_index(of_node, "linaro,optee-channel-id", idx, &mut channel_id) == 0
}

unsafe fn scmi_optee_clear_channel(cinfo: *mut ScmiChanInfo) {
    let channel = (*cinfo).transport_info as *mut ScmiOpteeChannel;
    if !(*channel).tee_shm.is_null() { return; }
    ((*CORE).shmem.as_ref().unwrap().clear_channel)((*channel).req.shmem);
}

unsafe fn setup_dynamic_shmem(_dev: *mut Device, channel: *mut ScmiOpteeChannel) -> i32 {
    let msg_size = SCMI_SHMEM_MAX_PAYLOAD_SIZE;
    (*channel).tee_shm = tee_shm_alloc_kernel_buf((*SCMI_OPTEE_PRIVATE).tee_ctx, msg_size);
    if is_err((*channel).tee_shm) { dev_err((*channel).cinfo.as_ref().unwrap().dev, "shmem allocation failed\n"); return -ENOMEM; }
    let shbuf = tee_shm_get_va((*channel).tee_shm, 0);
    core::ptr::write_bytes(shbuf, 0, msg_size);
    (*channel).req.msg = shbuf as *mut ScmiMsgPayld;
    (*channel).rx_len = msg_size as u32;
    0
}

unsafe fn setup_static_shmem(dev: *mut Device, cinfo: *mut ScmiChanInfo, channel: *mut ScmiOpteeChannel) -> i32 {
    (*channel).req.shmem = ((*CORE).shmem.as_ref().unwrap().setup_iomap)(cinfo, dev, true, core::ptr::null_mut(), &mut (*channel).io_ops);
    if is_err((*channel).req.shmem) { return ptr_err((*channel).req.shmem); }
    0
}

unsafe fn setup_shmem(dev: *mut Device, cinfo: *mut ScmiChanInfo, channel: *mut ScmiOpteeChannel) -> i32 {
    if of_property_present((*cinfo).dev.as_ref().unwrap().of_node, "shmem") { setup_static_shmem(dev, cinfo, channel) } else { setup_dynamic_shmem(dev, channel) }
}

unsafe fn scmi_optee_chan_setup(cinfo: *mut ScmiChanInfo, dev: *mut Device, tx: bool) -> i32 {
    if !tx { return -ENODEV; }
    let channel = devm_kzalloc(dev, core::mem::size_of::<ScmiOpteeChannel>(), GFP_KERNEL) as *mut ScmiOpteeChannel;
    if channel.is_null() { return -ENOMEM; }
    let mut channel_id = 0u32;
    let ret = of_property_read_u32_index((*cinfo).dev.as_ref().unwrap().of_node, "linaro,optee-channel-id", 0, &mut channel_id);
    if ret != 0 { return ret; }
    (*cinfo).transport_info = channel as *mut _;
    (*channel).cinfo = cinfo; (*channel).channel_id = channel_id; mutex_init(&mut (*channel).mu);
    let ret = setup_shmem(dev, cinfo, channel); if ret != 0 { return ret; }
    let ret = open_session(SCMI_OPTEE_PRIVATE, &mut (*channel).tee_session); if ret != 0 { goto_err_free_shm(channel, ret); }
    let ret = tee_client_system_session((*SCMI_OPTEE_PRIVATE).tee_ctx, (*channel).tee_session);
    if ret != 0 { dev_warn(dev, "Could not switch to system session, do best effort\n"); }
    let ret = get_channel(channel); if ret != 0 { close_session(SCMI_OPTEE_PRIVATE, (*channel).tee_session); goto_err_free_shm(channel, ret); }
    (*cinfo).no_completion_irq = true;
    mutex_lock(&mut (*SCMI_OPTEE_PRIVATE).mu); list_add(&mut (*channel).link, &mut (*SCMI_OPTEE_PRIVATE).channel_list); mutex_unlock(&mut (*SCMI_OPTEE_PRIVATE).mu);
    0
}

unsafe fn goto_err_free_shm(channel: *mut ScmiOpteeChannel, ret: i32) -> i32 { if !(*channel).tee_shm.is_null() { tee_shm_free((*channel).tee_shm); } ret }

unsafe fn scmi_optee_chan_free(_id: i32, p: *mut core::ffi::c_void, _data: *mut core::ffi::c_void) -> i32 {
    let cinfo = p as *mut ScmiChanInfo; let channel = (*cinfo).transport_info as *mut ScmiOpteeChannel;
    if channel.is_null() { return 0; }
    mutex_lock(&mut (*SCMI_OPTEE_PRIVATE).mu); list_del(&mut (*channel).link); mutex_unlock(&mut (*SCMI_OPTEE_PRIVATE).mu);
    close_session(SCMI_OPTEE_PRIVATE, (*channel).tee_session);
    if !(*channel).tee_shm.is_null() { tee_shm_free((*channel).tee_shm); (*channel).tee_shm = core::ptr::null_mut(); }
    (*cinfo).transport_info = core::ptr::null_mut(); (*channel).cinfo = core::ptr::null_mut(); 0
}

unsafe fn scmi_optee_send_message(cinfo: *mut ScmiChanInfo, xfer: *mut ScmiXfer) -> i32 {
    let channel = (*cinfo).transport_info as *mut ScmiOpteeChannel; mutex_lock(&mut (*channel).mu);
    let ret = if !(*channel).tee_shm.is_null() { ((*CORE).msg.as_ref().unwrap().tx_prepare)((*channel).req.msg, xfer); invoke_process_msg_channel(channel, ((*CORE).msg.as_ref().unwrap().command_size)(xfer)) } else { ((*CORE).shmem.as_ref().unwrap().tx_prepare)((*channel).req.shmem, xfer, cinfo, (*(*channel).io_ops).toio); invoke_process_smt_channel(channel) };
    if ret != 0 { mutex_unlock(&mut (*channel).mu); } ret
}

unsafe fn scmi_optee_fetch_response(cinfo: *mut ScmiChanInfo, xfer: *mut ScmiXfer) { let channel = (*cinfo).transport_info as *mut ScmiOpteeChannel; if !(*channel).tee_shm.is_null() { ((*CORE).msg.as_ref().unwrap().fetch_response)((*channel).req.msg, (*channel).rx_len as usize, xfer); } else { ((*CORE).shmem.as_ref().unwrap().fetch_response)((*channel).req.shmem, xfer, (*(*channel).io_ops).fromio); } }
unsafe fn scmi_optee_mark_txdone(cinfo: *mut ScmiChanInfo, _ret: i32, _unused: *mut ScmiXfer) { mutex_unlock(&mut (*((*cinfo).transport_info as *mut ScmiOpteeChannel)).mu); }

static SCMI_OPTEE_OPS: ScmiTransportOps = ScmiTransportOps { chan_available: Some(scmi_optee_chan_available), chan_setup: Some(scmi_optee_chan_setup), chan_free: Some(scmi_optee_chan_free), send_message: Some(scmi_optee_send_message), mark_txdone: Some(scmi_optee_mark_txdone), fetch_response: Some(scmi_optee_fetch_response), clear_channel: Some(scmi_optee_clear_channel) };

unsafe fn scmi_optee_service_probe(scmi_pta: *mut TeeClientDevice) -> i32 {
    let dev = &mut (*scmi_pta).dev;
    if !SCMI_OPTEE_PRIVATE.is_null() { dev_err(dev, "An SCMI OP-TEE device was already initialized: only one allowed\n"); return -EBUSY; }
    let tee_ctx = tee_client_open_context(core::ptr::null_mut(), Some(scmi_optee_ctx_match), core::ptr::null_mut(), core::ptr::null_mut());
    if is_err(tee_ctx) { return -ENODEV; }
    let agent = devm_kzalloc(dev, core::mem::size_of::<ScmiOpteeAgent>(), GFP_KERNEL) as *mut ScmiOpteeAgent;
    if agent.is_null() { tee_client_close_context(tee_ctx); return -ENOMEM; }
    (*agent).dev = dev; (*agent).tee_ctx = tee_ctx; init_list_head(&mut (*agent).channel_list); mutex_init(&mut (*agent).mu);
    let ret = get_capabilities(agent); if ret != 0 { tee_client_close_context(tee_ctx); return ret; }
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); SCMI_OPTEE_PRIVATE = agent;
    let ret = scmi_transport_supplier_put(&mut SCMI_OPTEE_SUPPLIER.th, (*agent).dev);
    if ret != 0 { SCMI_OPTEE_PRIVATE = core::ptr::null_mut(); tee_client_close_context(tee_ctx); return ret; } 0
}

unsafe fn scmi_optee_service_remove(_scmi_pta: *mut TeeClientDevice) {
    let agent = SCMI_OPTEE_PRIVATE; if agent.is_null() { return; }
    if !list_empty(&(*agent).channel_list) { return; }
    SCMI_OPTEE_PRIVATE = core::ptr::null_mut(); scmi_transport_supplier_put(&mut SCMI_OPTEE_SUPPLIER.th, (*agent).dev); tee_client_close_context((*agent).tee_ctx);
}

unsafe fn scmi_optee_ctx_match(ver: *mut TeeIoctlVersionData, _data: *const core::ffi::c_void) -> i32 { ((*ver).impl_id == TEE_IMPL_ID_OPTEE) as i32 }

unsafe fn scmi_transport_optee_init() -> i32 { let ret = tee_client_driver_register(&SCMI_OPTEE_SERVICE_DRIVER); if ret != 0 { return ret; } let ret = platform_driver_register(&SCMI_OPTEE_DRIVER); if ret != 0 { tee_client_driver_unregister(&SCMI_OPTEE_SERVICE_DRIVER); } ret }
unsafe fn scmi_transport_optee_exit() { platform_driver_unregister(&SCMI_OPTEE_DRIVER); tee_client_driver_unregister(&SCMI_OPTEE_SERVICE_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
