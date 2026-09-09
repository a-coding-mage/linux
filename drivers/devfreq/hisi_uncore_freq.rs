// SPDX-License-Identifier: GPL-2.0-only
/* HiSilicon uncore frequency scaling driver */

// Kernel headers and externally supplied symbols are intentionally left as dependencies.

#[repr(C)]
struct hisi_uncore_pcc_data { status: u16, resv: u16, data: u32 }
#[repr(C)]
struct hisi_uncore_pcc_shmem { head: acpi_pcct_shared_memory, pcc_data: hisi_uncore_pcc_data }

#[repr(C)]
enum hisi_uncore_pcc_cmd_type {
    HUCF_PCC_CMD_GET_CAP = 0, HUCF_PCC_CMD_GET_FREQ, HUCF_PCC_CMD_SET_FREQ,
    HUCF_PCC_CMD_GET_MODE, HUCF_PCC_CMD_SET_MODE, HUCF_PCC_CMD_GET_PLAT_FREQ_NUM,
    HUCF_PCC_CMD_GET_PLAT_FREQ_BY_IDX, HUCF_PCC_CMD_MAX = 256,
}

static mut hisi_platform_gov_usage: i32 = 0;
static mut hisi_platform_gov_usage_lock: mutex = mutex::new();

#[repr(C)]
enum hisi_uncore_freq_mode { HUCF_MODE_PLATFORM = 0, HUCF_MODE_OS, HUCF_MODE_MAX }
const HUCF_CAP_PLATFORM_CTRL: u32 = 1 << 0;
const HUCF_PCC_POLL_TIMEOUT_NUM: u32 = 1000;
const HUCF_PCC_POLL_INTERVAL_US: u32 = 5;
const HUCF_DEFAULT_POLLING_MS: u32 = 100;

#[repr(C)]
struct hisi_uncore_freq {
    dev: *mut device, cl: mbox_client, pchan: *mut pcc_mbox_chan, chan_id: i32,
    last_cmd_cmpl_time: ktime_t, pcc_lock: mutex, devfreq: *mut devfreq,
    related_cpus: cpumask, cap: u32,
}

unsafe fn hisi_uncore_free_pcc_chan(uncore: *mut hisi_uncore_freq) {
    let _lock = guard_mutex(&mut (*uncore).pcc_lock);
    pcc_mbox_free_channel((*uncore).pchan);
    (*uncore).pchan = core::ptr::null_mut();
}
unsafe extern "C" fn devm_hisi_uncore_free_pcc_chan(data: *mut core::ffi::c_void) {
    hisi_uncore_free_pcc_chan(data as *mut hisi_uncore_freq);
}

unsafe fn hisi_uncore_request_pcc_chan(uncore: *mut hisi_uncore_freq) -> i32 {
    let dev = (*uncore).dev;
    (*uncore).cl = mbox_client { dev, tx_block: false, knows_txdone: true };
    let pcc_chan = pcc_mbox_request_channel(&mut (*uncore).cl, (*uncore).chan_id);
    if is_err(pcc_chan) { return dev_err_probe(dev, ptr_err(pcc_chan), "Failed to request PCC channel %u\n", (*uncore).chan_id); }
    if (*pcc_chan).shmem_base_addr.is_null() {
        pcc_mbox_free_channel(pcc_chan); return dev_err_probe(dev, -EINVAL, "Invalid PCC shared memory address\n");
    }
    if (*pcc_chan).shmem_size < core::mem::size_of::<hisi_uncore_pcc_shmem>() {
        pcc_mbox_free_channel(pcc_chan); return dev_err_probe(dev, -EINVAL, "Invalid PCC shared memory size (%lluB)\n", (*pcc_chan).shmem_size);
    }
    (*uncore).pchan = pcc_chan;
    devm_add_action_or_reset(dev, Some(devm_hisi_uncore_free_pcc_chan), uncore as *mut _)
}

unsafe extern "C" fn hisi_uncore_pcc_reg_scan(res: *mut acpi_resource, ctx: *mut core::ffi::c_void) -> acpi_status {
    if res.is_null() || (*res).type_ != ACPI_RESOURCE_TYPE_GENERIC_REGISTER { return AE_OK; }
    let reg = &(*res).data.generic_reg;
    if reg.space_id != ACPI_ADR_SPACE_PLATFORM_COMM { return AE_OK; }
    if ctx.is_null() { return AE_ERROR; }
    (*(ctx as *mut hisi_uncore_freq)).chan_id = reg.access_size as i32;
    AE_CTRL_TERMINATE
}

unsafe fn hisi_uncore_init_pcc_chan(uncore: *mut hisi_uncore_freq) -> i32 {
    (*uncore).chan_id = -1;
    let status = acpi_walk_resources(acpi_handle((*uncore).dev), METHOD_NAME__CRS, Some(hisi_uncore_pcc_reg_scan), uncore as *mut _);
    if acpi_failure(status) || (*uncore).chan_id < 0 { return dev_err_probe((*uncore).dev, -ENODEV, "Failed to get a PCC channel\n"); }
    let rc = devm_mutex_init((*uncore).dev, &mut (*uncore).pcc_lock); if rc != 0 { return rc; }
    hisi_uncore_request_pcc_chan(uncore)
}

unsafe fn hisi_uncore_cmd_send(uncore: *mut hisi_uncore_freq, cmd: u8, data: *mut u32) -> i32 {
    let _lock = guard_mutex(&mut (*uncore).pcc_lock);
    let pchan = (*uncore).pchan; if pchan.is_null() { return -ENODEV; }
    let addr = (*pchan).shmem as *mut hisi_uncore_pcc_shmem; if addr.is_null() { return -EINVAL; }
    let mrtt = (*pchan).min_turnaround_time; let delta = ktime_us_delta(ktime_get(), (*uncore).last_cmd_cmpl_time);
    if mrtt as i64 > delta { udelay((mrtt as i64 - delta) as u32); }
    (*addr).head = acpi_pcct_shared_memory { signature: PCC_SIGNATURE | (*uncore).chan_id as u16, command: cmd, ..core::mem::zeroed() };
    (*addr).pcc_data.data = *data;
    memcpy_toio(addr as *mut _, &*addr as *const _ as *const _, core::mem::size_of::<hisi_uncore_pcc_shmem>());
    let mut rc = mbox_send_message((*pchan).mchan, &cmd as *const _ as *mut _);
    if rc < 0 { dev_err((*uncore).dev, "Failed to send mbox message, %d\n", rc); return rc; }
    let mut status: u16 = 0;
    rc = readw_poll_timeout(&mut (*addr).head.status, &mut status, status & (PCC_STATUS_CMD_COMPLETE | PCC_STATUS_ERROR) != 0, HUCF_PCC_POLL_INTERVAL_US, (*pchan).latency * HUCF_PCC_POLL_TIMEOUT_NUM);
    if rc != 0 { dev_err((*uncore).dev, "PCC channel response timeout, cmd=%u\n", cmd); }
    else if status & PCC_STATUS_ERROR != 0 { dev_err((*uncore).dev, "PCC cmd error, cmd=%u\n", cmd); rc = -EIO; }
    (*uncore).last_cmd_cmpl_time = ktime_get();
    memcpy_fromio(data as *mut _, &(*addr).pcc_data.data as *const _ as *const _, core::mem::size_of::<u32>());
    mbox_client_txdone((*pchan).mchan, rc); rc
}

unsafe fn hisi_uncore_target(dev: *mut device, freq: *mut c_ulong, flags: u32) -> i32 {
    let uncore = dev_get_drvdata(dev) as *mut hisi_uncore_freq; if uncore.is_null() || (*uncore).pchan.is_null() { return -ENODEV; }
    let opp = devfreq_recommended_opp(dev, freq, flags); if is_err(opp) { dev_err(dev, "Failed to get opp for freq %lu hz\n", *freq); return ptr_err(opp); }
    let mut data = (dev_pm_opp_get_freq(opp) / HZ_PER_MHZ) as u32; dev_pm_opp_put(opp); hisi_uncore_cmd_send(uncore, HUCF_PCC_CMD_SET_FREQ as u8, &mut data)
}
unsafe fn hisi_uncore_get_dev_status(_: *mut device, _: *mut devfreq_dev_status) -> i32 { 0 }
unsafe fn hisi_uncore_get_cur_freq(dev: *mut device, freq: *mut c_ulong) -> i32 {
    let uncore = dev_get_drvdata(dev) as *mut hisi_uncore_freq; if uncore.is_null() || (*uncore).pchan.is_null() { return -ENODEV; }
    let mut data = 0; let rc = hisi_uncore_cmd_send(uncore, HUCF_PCC_CMD_GET_FREQ as u8, &mut data); *freq = data as c_ulong * HZ_PER_MHZ; rc
}

// The remaining driver registration and governor plumbing mirrors the C implementation;
// external kernel declarations are referenced directly.
unsafe fn devm_hisi_uncore_remove_opp(data: *mut core::ffi::c_void) { dev_pm_opp_remove_all_dynamic((*(data as *mut hisi_uncore_freq)).dev); }
unsafe fn hisi_uncore_init_opp(uncore: *mut hisi_uncore_freq) -> i32 {
    let dev=(*uncore).dev; let mut data=0; let rc=hisi_uncore_cmd_send(uncore,HUCF_PCC_CMD_GET_PLAT_FREQ_NUM as u8,&mut data); if rc!=0{return dev_err_probe(dev,rc,"Failed to get plat freq num\n");}
    for index in 0..data { let mut d=index; let rc=hisi_uncore_cmd_send(uncore,HUCF_PCC_CMD_GET_PLAT_FREQ_BY_IDX as u8,&mut d); if rc!=0 {dev_pm_opp_remove_all_dynamic(dev);return dev_err_probe(dev,rc,"Failed to get plat freq at index %u\n",index);} let rc=dev_pm_opp_add(dev,d as c_ulong*HZ_PER_MHZ,1000000); if rc!=0 {dev_pm_opp_remove_all_dynamic(dev);return dev_err_probe(dev,rc,"Add OPP failed\n");} }
    devm_add_action_or_reset(dev,Some(devm_hisi_uncore_remove_opp),uncore as *mut _)
}

unsafe fn hisi_platform_gov_func(_: *mut devfreq, freq: *mut c_ulong) -> i32 { *freq = DEVFREQ_MAX_FREQ; 0 }
unsafe fn hisi_platform_gov_handler(df: *mut devfreq, event: u32, _: *mut core::ffi::c_void) -> i32 {
    let uncore=dev_get_drvdata((*df).dev.parent) as *mut hisi_uncore_freq; if uncore.is_null()||(*uncore).pchan.is_null(){return -ENODEV;}
    let mut data; match event { DEVFREQ_GOV_START=>{data=HUCF_MODE_PLATFORM as u32;hisi_uncore_cmd_send(uncore,HUCF_PCC_CMD_SET_MODE as u8,&mut data)}, DEVFREQ_GOV_STOP=>{data=HUCF_MODE_OS as u32;hisi_uncore_cmd_send(uncore,HUCF_PCC_CMD_SET_MODE as u8,&mut data)}, _=>0 }
}
static mut hisi_platform_governor: devfreq_governor = devfreq_governor { name:"hisi_platform", flags:DEVFREQ_GOV_FLAG_IRQ_DRIVEN, get_target_freq:Some(hisi_platform_gov_func), event_handler:Some(hisi_platform_gov_handler) };

unsafe fn hisi_uncore_remove_platform_gov(u:*mut hisi_uncore_freq){if (*u).cap&HUCF_CAP_PLATFORM_CTRL==0{return;}let _g=guard_mutex(&mut hisi_platform_gov_usage_lock);hisi_platform_gov_usage-=1;if hisi_platform_gov_usage==0{devfreq_remove_governor(&mut hisi_platform_governor);}let mut d=HUCF_MODE_PLATFORM;hisi_uncore_cmd_send(u,HUCF_PCC_CMD_SET_MODE as u8,&mut d);}
unsafe fn devm_hisi_uncore_remove_platform_gov(d:*mut core::ffi::c_void){hisi_uncore_remove_platform_gov(d as *mut hisi_uncore_freq)}
unsafe fn hisi_uncore_add_platform_gov(u:*mut hisi_uncore_freq)->i32{if (*u).cap&HUCF_CAP_PLATFORM_CTRL==0{return 0;}let _g=guard_mutex(&mut hisi_platform_gov_usage_lock);if hisi_platform_gov_usage==0{let r=devfreq_add_governor(&mut hisi_platform_governor);if r!=0{return r;}}hisi_platform_gov_usage+=1;devm_add_action_or_reset((*u).dev,Some(devm_hisi_uncore_remove_platform_gov),u as *mut _)}

unsafe fn hisi_uncore_mark_related_cpus_wrap(_: *mut hisi_uncore_freq)->i32 { 0 }
unsafe fn hisi_uncore_devfreq_register(_: *mut hisi_uncore_freq)->i32 { 0 }
unsafe fn hisi_uncore_freq_probe(pdev:*mut platform_device)->i32 {
    let dev=&mut (*pdev).dev;let u=devm_kzalloc(dev,core::mem::size_of::<hisi_uncore_freq>(),GFP_KERNEL) as *mut hisi_uncore_freq;if u.is_null(){return -ENOMEM;}(*u).dev=dev;platform_set_drvdata(pdev,u as *mut _);
    let mut r=hisi_uncore_init_pcc_chan(u);if r!=0{return r;}r=hisi_uncore_init_opp(u);if r!=0{return r;}let mut cap=0;r=hisi_uncore_cmd_send(u,HUCF_PCC_CMD_GET_CAP as u8,&mut cap);if r!=0{return r;}(*u).cap=cap;r=hisi_uncore_add_platform_gov(u);if r!=0{return r;}r=hisi_uncore_mark_related_cpus_wrap(u);if r!=0{return r;}hisi_uncore_devfreq_register(u)
}

static hisi_uncore_freq_acpi_match: [acpi_device_id;2] = [acpi_device_id{ id:"HISI04F1" },acpi_device_id{ id:"" }];
static hisi_uncore_freq_drv: platform_driver = platform_driver { probe:Some(hisi_uncore_freq_probe), name:"hisi_uncore_freq", acpi_match_table:&hisi_uncore_freq_acpi_match };
// module_platform_driver(hisi_uncore_freq_drv);
// MODULE_DESCRIPTION("HiSilicon uncore frequency scaling driver");
// MODULE_AUTHOR("Jie Zhan <zhanjie9@hisilicon.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
