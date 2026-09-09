// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2019-2021, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

// Linux dependencies: delay, err, memblock, mhi, moduleparam, pci, sizes.
// Local dependencies: mhi_controller.h and qaic.h.

const MAX_RESET_TIME_SEC: u8 = 25;

static mut MHI_TIMEOUT_MS: u32 = 2000; // 2 sec default

static FW_IMAGE_PATHS: [*const u8; FAMILY_MAX] = [
    b"qcom/aic100/sbl.bin\0".as_ptr(),
    b"qcom/aic200/sbl.bin\0".as_ptr(),
];

const fn channel(name: &'static [u8], num: u32, elements: u32, dir: u32, ee: u32) -> mhi_channel_config {
    mhi_channel_config { name: name.as_ptr(), num, num_elements: elements, local_elements: 0,
        event_ring: 0, dir, ee_mask: ee, pollcfg: 0, doorbell: MHI_DB_BRST_DISABLE,
        lpm_notify: false, offload_channel: false, doorbell_mode_switch: false, wake_capable: false }
}

static AIC100_CHANNELS: [mhi_channel_config; 26] = [
    channel(b"QAIC_LOOPBACK\0",0,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_LOOPBACK\0",1,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_SAHARA\0",2,32,DMA_TO_DEVICE,MHI_CH_EE_SBL), channel(b"QAIC_SAHARA\0",3,32,DMA_FROM_DEVICE,MHI_CH_EE_SBL),
    channel(b"QAIC_DIAG\0",4,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_DIAG\0",5,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_SSR\0",6,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_SSR\0",7,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_QDSS\0",8,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_QDSS\0",9,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_CONTROL\0",10,128,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_CONTROL\0",11,128,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_LOGGING\0",12,32,DMA_TO_DEVICE,MHI_CH_EE_SBL), channel(b"QAIC_LOGGING\0",13,32,DMA_FROM_DEVICE,MHI_CH_EE_SBL),
    channel(b"QAIC_STATUS\0",14,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_STATUS\0",15,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_TELEMETRY\0",16,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_TELEMETRY\0",17,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_DEBUG\0",18,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_DEBUG\0",19,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"QAIC_TIMESYNC\0",20,32,DMA_TO_DEVICE,MHI_CH_EE_SBL), channel(b"QAIC_TIMESYNC\0",21,32,DMA_FROM_DEVICE,MHI_CH_EE_SBL),
    channel(b"QAIC_TIMESYNC_PERIODIC\0",22,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"QAIC_TIMESYNC_PERIODIC\0",23,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
    channel(b"IPCR\0",24,32,DMA_TO_DEVICE,MHI_CH_EE_AMSS), channel(b"IPCR\0",25,32,DMA_FROM_DEVICE,MHI_CH_EE_AMSS),
];

static AIC200_CHANNELS: [mhi_channel_config; 16] = [
    AIC100_CHANNELS[0], AIC100_CHANNELS[1], AIC100_CHANNELS[2], AIC100_CHANNELS[3],
    AIC100_CHANNELS[6], AIC100_CHANNELS[7], AIC100_CHANNELS[10], AIC100_CHANNELS[11],
    AIC100_CHANNELS[12], AIC100_CHANNELS[13], AIC100_CHANNELS[14], AIC100_CHANNELS[15],
    AIC100_CHANNELS[16], AIC100_CHANNELS[17], AIC100_CHANNELS[22], AIC100_CHANNELS[23],
];

const fn event_config() -> mhi_event_config { mhi_event_config { num_elements: 32, irq_moderation_ms: 0, irq: 0,
    channel: u32::MAX, priority: 1, mode: MHI_DB_BRST_DISABLE, data_type: MHI_ER_CTRL,
    hardware_event: false, client_managed: false, offload_channel: false } }
static mut AIC100_EVENTS: [mhi_event_config; 1] = [event_config()];
static mut AIC200_EVENTS: [mhi_event_config; 1] = [event_config()];

static mut MHI_CNTRL_CONFIGS: [mhi_controller_config; FAMILY_MAX] = [
    mhi_controller_config { max_channels: 128, timeout_ms: 0, buf_len: 0, num_channels: AIC100_CHANNELS.len(), ch_cfg: AIC100_CHANNELS.as_ptr(), num_events: 1, event_cfg: AIC100_EVENTS.as_ptr(), use_bounce_buf: false, m2_no_db: false },
    mhi_controller_config { max_channels: 128, timeout_ms: 0, buf_len: 0, num_channels: AIC200_CHANNELS.len(), ch_cfg: AIC200_CHANNELS.as_ptr(), num_events: 1, event_cfg: AIC200_EVENTS.as_ptr(), use_bounce_buf: false, m2_no_db: false },
];

unsafe extern "C" fn mhi_read_reg(mhi_cntrl: *mut mhi_controller, addr: *mut core::ffi::c_void, out: *mut u32) -> i32 {
    if (addr as usize).wrapping_sub((*mhi_cntrl).regs as usize) == 0x224 { *out = 0x60110200; return 0; }
    let tmp = readl_relaxed(addr);
    if tmp == u32::MAX { return -EIO; }
    *out = tmp; 0
}
unsafe extern "C" fn mhi_write_reg(_mhi_cntrl: *mut mhi_controller, addr: *mut core::ffi::c_void, val: u32) { writel_relaxed(val, addr); }
unsafe extern "C" fn mhi_runtime_get(_mhi_cntrl: *mut mhi_controller) -> i32 { 0 }
unsafe extern "C" fn mhi_runtime_put(_mhi_cntrl: *mut mhi_controller) {}

unsafe extern "C" fn mhi_status_cb(mhi_cntrl: *mut mhi_controller, reason: mhi_callback) {
    let qdev = pci_get_drvdata(to_pci_dev((*mhi_cntrl).cntrl_dev));
    if reason == MHI_CB_FATAL_ERROR { pci_err((*qdev).pdev, "Fatal error received from device. Attempting to recover\n"); }
    if reason == MHI_CB_SYS_ERROR { qaic_dev_reset_clean_local_state(qdev); }
}

unsafe fn mhi_reset_and_async_power_up(mhi_cntrl: *mut mhi_controller) -> i32 {
    let mut time_sec: u8 = 1; let mut current_ee: i32;
    mhi_soc_reset(mhi_cntrl);
    loop { msleep(1000); current_ee = mhi_get_exec_env(mhi_cntrl); if current_ee == MHI_EE_PBL || { let old=time_sec; time_sec = time_sec.wrapping_add(1); old } > MAX_RESET_TIME_SEC { break; } }
    if current_ee == MHI_EE_PBL { mhi_async_power_up(mhi_cntrl) } else { -EIO }
}

pub unsafe fn qaic_mhi_register_controller(pci_dev: *mut pci_dev, mhi_bar: *mut core::ffi::c_void, mhi_irq: i32, shared_msi: bool, family: i32) -> *mut mhi_controller {
    let mut mhi_config = MHI_CNTRL_CONFIGS[family as usize];
    let mhi_cntrl = devm_kzalloc(&mut (*pci_dev).dev, core::mem::size_of::<mhi_controller>(), GFP_KERNEL) as *mut mhi_controller;
    if mhi_cntrl.is_null() { return ERR_PTR(-ENOMEM); }
    (*mhi_cntrl).cntrl_dev = &mut (*pci_dev).dev; (*mhi_cntrl).iova_start = 0; (*mhi_cntrl).iova_stop = PHYS_ADDR_MAX - 1;
    (*mhi_cntrl).status_cb = Some(mhi_status_cb); (*mhi_cntrl).runtime_get = Some(mhi_runtime_get); (*mhi_cntrl).runtime_put = Some(mhi_runtime_put);
    (*mhi_cntrl).read_reg = Some(mhi_read_reg); (*mhi_cntrl).write_reg = Some(mhi_write_reg); (*mhi_cntrl).regs = mhi_bar; (*mhi_cntrl).reg_len = SZ_4K; (*mhi_cntrl).nr_irqs = 1;
    (*mhi_cntrl).irq = devm_kmalloc(&mut (*pci_dev).dev, core::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
    if (*mhi_cntrl).irq.is_null() { return ERR_PTR(-ENOMEM); } *(*mhi_cntrl).irq = mhi_irq;
    if shared_msi { (*mhi_cntrl).irq_flags = IRQF_SHARED; }
    (*mhi_cntrl).fw_image = FW_IMAGE_PATHS[family as usize];
    if family == FAMILY_AIC200 { (*mhi_cntrl).name = b"AIC200\0".as_ptr(); (*mhi_cntrl).seg_len = SZ_512K; } else { (*mhi_cntrl).name = b"AIC100\0".as_ptr(); }
    (*mhi_cntrl).timeout_ms = MHI_TIMEOUT_MS; mhi_config.timeout_ms = MHI_TIMEOUT_MS;
    let mut ret = mhi_register_controller(mhi_cntrl, &mut mhi_config); if ret != 0 { pci_err(pci_dev, "mhi_register_controller failed %d\n", ret); return ERR_PTR(ret); }
    ret = mhi_prepare_for_power_up(mhi_cntrl); if ret != 0 { pci_err(pci_dev, "mhi_prepare_for_power_up failed %d\n", ret); mhi_unregister_controller(mhi_cntrl); return ERR_PTR(ret); }
    ret = mhi_async_power_up(mhi_cntrl);
    if ret == -EIO && MHI_EE_SBL == mhi_get_exec_env(mhi_cntrl) { pci_err(pci_dev, "Found device in SBL at MHI init. Attempting a reset.\n"); ret = mhi_reset_and_async_power_up(mhi_cntrl); }
    if ret != 0 { pci_err(pci_dev, "mhi_async_power_up failed %d\n", ret); mhi_unprepare_after_power_down(mhi_cntrl); mhi_unregister_controller(mhi_cntrl); return ERR_PTR(ret); }
    mhi_cntrl
}

pub unsafe fn qaic_mhi_free_controller(mhi_cntrl: *mut mhi_controller, link_up: bool) { mhi_power_down(mhi_cntrl, link_up); mhi_unprepare_after_power_down(mhi_cntrl); mhi_unregister_controller(mhi_cntrl); }
pub unsafe fn qaic_mhi_start_reset(mhi_cntrl: *mut mhi_controller) { mhi_power_down(mhi_cntrl, true); }
pub unsafe fn qaic_mhi_reset_done(mhi_cntrl: *mut mhi_controller) { let pci_dev = container_of((*mhi_cntrl).cntrl_dev, pci_dev, dev); let ret = mhi_async_power_up(mhi_cntrl); if ret != 0 { pci_err(pci_dev, "mhi_async_power_up failed after reset %d\n", ret); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
