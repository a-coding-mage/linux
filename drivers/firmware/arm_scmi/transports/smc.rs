// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) Message SMC/HVC
 * Transport driver
 *
 * Copyright 2020 NXP
 */

// Linux kernel dependencies are supplied by the surrounding repository.

const SHMEM_SIZE: usize = 0x1000;
const SHMEM_SHIFT: usize = 12;
#[inline]
const fn shmem_page(x: usize) -> usize { x >> SHMEM_SHIFT }
#[inline]
const fn shmem_offset(x: usize) -> usize { x & (SHMEM_SIZE - 1) }

/*
 * The shmem address is split into 4K page and offset.
 * This is to make sure the parameters fit in 32bit arguments of the
 * smc/hvc call to keep it uniform across smc32/smc64 conventions.
 * This however limits the shmem address to 44 bit.
 *
 * These optional parameters can be used to distinguish among multiple
 * scmi instances that are using the same smc-id.
 * The page parameter is passed in r1/x1/w1 register and the offset parameter
 * is passed in r2/x2/w2 register.
 */

#[repr(C)]
pub struct ScmiSmc {
    pub irq: i32,
    pub cinfo: *mut ScmiChanInfo,
    pub shmem: *mut ScmiSharedMem,
    pub io_ops: *mut ScmiShmemIoOps,
    /* Protect access to shmem area */
    pub shmem_lock: Mutex,
    pub inflight: Atomic,
    pub func_id: usize,
    pub param_page: usize,
    pub param_offset: usize,
    pub cap_id: usize,
}

const INFLIGHT_NONE: i32 = MSG_TOKEN_MAX;

static mut CORE: *mut ScmiTransportCoreOperations = core::ptr::null_mut();

unsafe extern "C" fn smc_msg_done_isr(_irq: i32, data: *mut core::ffi::c_void) -> IrqReturn {
    let scmi_info = data as *mut ScmiSmc;
    ((*CORE).rx_callback)((*scmi_info).cinfo,
        ((*(*CORE).shmem).read_header)((*scmi_info).shmem), core::ptr::null_mut());
    IRQ_HANDLED
}

unsafe extern "C" fn smc_chan_available(_of_node: *mut DeviceNode, _idx: i32) -> bool {
    let np = of_parse_phandle(_of_node, b"shmem\0".as_ptr() as *const _, 0);
    !np.is_null()
}

#[inline]
unsafe fn smc_channel_lock_init(scmi_info: *mut ScmiSmc) {
    if IS_ENABLED_ATOMIC {
        atomic_set(&mut (*scmi_info).inflight, INFLIGHT_NONE);
    } else {
        mutex_init(&mut (*scmi_info).shmem_lock);
    }
}

unsafe fn smc_xfer_inflight(xfer: *mut ScmiXfer, inflight: *mut Atomic) -> bool {
    atomic_cmpxchg(inflight, INFLIGHT_NONE, (*xfer).hdr.seq) == INFLIGHT_NONE
}

#[inline]
unsafe fn smc_channel_lock_acquire(scmi_info: *mut ScmiSmc, xfer: *mut ScmiXfer) {
    if IS_ENABLED_ATOMIC {
        spin_until_cond(|| smc_xfer_inflight(xfer, &mut (*scmi_info).inflight));
    } else {
        mutex_lock(&mut (*scmi_info).shmem_lock);
    }
}

#[inline]
unsafe fn smc_channel_lock_release(scmi_info: *mut ScmiSmc) {
    if IS_ENABLED_ATOMIC {
        atomic_set(&mut (*scmi_info).inflight, INFLIGHT_NONE);
    } else {
        mutex_unlock(&mut (*scmi_info).shmem_lock);
    }
}

unsafe extern "C" fn smc_chan_setup(cinfo: *mut ScmiChanInfo, dev: *mut Device, tx: bool) -> i32 {
    if !tx { return -ENODEV; }
    let scmi_info = devm_kzalloc(dev, core::mem::size_of::<ScmiSmc>(), GFP_KERNEL) as *mut ScmiSmc;
    if scmi_info.is_null() { return -ENOMEM; }
    let mut res = Resource::default();
    let mut func_id: u32 = 0;
    (*scmi_info).shmem = ((*CORE).shmem).setup_iomap(cinfo, dev, tx, &mut res, &mut (*scmi_info).io_ops);
    if IS_ERR((*scmi_info).shmem) { return PTR_ERR((*scmi_info).shmem); }
    let ret = of_property_read_u32((*dev).of_node, b"arm,smc-id\0".as_ptr() as *const _, &mut func_id);
    if ret < 0 { return ret; }
    let mut cap_id = usize::MAX;
    if of_device_is_compatible((*dev).of_node, b"qcom,scmi-smc\0".as_ptr() as *const _) {
        let size = resource_size(&res);
        let ptr = ((*scmi_info).shmem as *mut u8).add(size - 8);
        core::ptr::copy_nonoverlapping(ptr, &mut cap_id as *mut usize as *mut u8, core::mem::size_of::<usize>());
    }
    if of_device_is_compatible((*dev).of_node, b"arm,scmi-smc-param\0".as_ptr() as *const _) {
        (*scmi_info).param_page = shmem_page(res.start);
        (*scmi_info).param_offset = shmem_offset(res.start);
    }
    (*scmi_info).func_id = func_id as usize; (*scmi_info).cap_id = cap_id; (*scmi_info).cinfo = cinfo;
    smc_channel_lock_init(scmi_info); (*cinfo).transport_info = scmi_info as *mut _;
    (*scmi_info).irq = of_irq_get_byname((*(*cinfo).dev).of_node, b"a2p\0".as_ptr() as *const _);
    if (*scmi_info).irq > 0 {
        let ret = request_irq((*scmi_info).irq, smc_msg_done_isr, IRQF_NO_SUSPEND, dev_name(dev), scmi_info as *mut _);
        if ret != 0 { (*cinfo).transport_info = core::ptr::null_mut(); (*scmi_info).cinfo = core::ptr::null_mut(); return ret; }
    } else { (*cinfo).no_completion_irq = true; }
    0
}

unsafe extern "C" fn smc_chan_free(_id: i32, p: *mut core::ffi::c_void, _data: *mut core::ffi::c_void) -> i32 {
    let cinfo = p as *mut ScmiChanInfo; let scmi_info = (*cinfo).transport_info as *mut ScmiSmc;
    if scmi_info.is_null() { return 0; }
    if (*scmi_info).irq > 0 { free_irq((*scmi_info).irq, scmi_info as *mut _); }
    (*cinfo).transport_info = core::ptr::null_mut(); (*scmi_info).cinfo = core::ptr::null_mut(); 0
}

unsafe extern "C" fn smc_send_message(cinfo: *mut ScmiChanInfo, xfer: *mut ScmiXfer) -> i32 {
    let scmi_info = (*cinfo).transport_info as *mut ScmiSmc; let mut res = ArmSmcccRes::default();
    smc_channel_lock_acquire(scmi_info, xfer);
    ((*CORE).shmem).tx_prepare((*scmi_info).shmem, xfer, cinfo, (*(*scmi_info).io_ops).toio);
    if (*scmi_info).cap_id != usize::MAX { arm_smccc_1_1_invoke((*scmi_info).func_id, (*scmi_info).cap_id, 0, 0, 0, 0, 0, 0, &mut res); }
    else { arm_smccc_1_1_invoke((*scmi_info).func_id, (*scmi_info).param_page, (*scmi_info).param_offset, 0, 0, 0, 0, 0, &mut res); }
    if res.a0 != 0 { smc_channel_lock_release(scmi_info); return -EOPNOTSUPP; } 0
}

unsafe extern "C" fn smc_fetch_response(cinfo: *mut ScmiChanInfo, xfer: *mut ScmiXfer) {
    let s = (*cinfo).transport_info as *mut ScmiSmc;
    ((*CORE).shmem).fetch_response((*s).shmem, xfer, (*(*s).io_ops).fromio);
}
unsafe extern "C" fn smc_mark_txdone(cinfo: *mut ScmiChanInfo, _ret: i32, _xfer: *mut ScmiXfer) {
    smc_channel_lock_release((*cinfo).transport_info as *mut ScmiSmc);
}

static SCMI_SMC_OPS: ScmiTransportOps = ScmiTransportOps {
    chan_available: Some(smc_chan_available),
    chan_setup: Some(smc_chan_setup),
    chan_free: Some(smc_chan_free),
    send_message: Some(smc_send_message),
    mark_txdone: Some(smc_mark_txdone),
    fetch_response: Some(smc_fetch_response),
};

static SCMI_SMC_DESC: ScmiDesc = ScmiDesc {
    ops: &SCMI_SMC_OPS,
    max_rx_timeout_ms: 30,
    max_msg: 20,
    max_msg_size: SCMI_SHMEM_MAX_PAYLOAD_SIZE,
    /*
     * Setting .sync_cmds_atomic_replies to true for SMC assumes that,
     * once the SMC instruction has completed successfully, the issued
     * SCMI command would have been already fully processed by the SCMI
     * platform firmware and so any possible response value expected
     * for the issued command will be immmediately ready to be fetched
     * from the shared memory area.
     */
    sync_cmds_completed_on_ret: true,
    atomic_enabled: IS_ENABLED_ATOMIC,
};

static SCMI_OF_MATCH: [OfDeviceId; 4] = [
    OfDeviceId::compatible(b"arm,scmi-smc\0"),
    OfDeviceId::compatible(b"arm,scmi-smc-param\0"),
    OfDeviceId::compatible(b"qcom,scmi-smc\0"),
    OfDeviceId::sentinel(),
];

// DEFINE_SCMI_TRANSPORT_DRIVER(scmi_smc, scmi_smc_driver, scmi_smc_desc,
//                              scmi_of_match, core);
// module_platform_driver(scmi_smc_driver);
// MODULE_DEVICE_TABLE(of, scmi_of_match);
// MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
// MODULE_AUTHOR("Nikunj Kela <quic_nkela@quicinc.com>");
// MODULE_DESCRIPTION("SCMI SMC Transport driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
