// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 - 2024 Intel Corporation
 */

// Dependencies supplied by the surrounding driver and kernel bindings are intentionally external.

#[cfg(feature = "config_fault_injection")]
static mut IVPU_FAIL_HW: *mut ::core::ffi::c_char = ::core::ptr::null_mut();

const FW_SHARED_MEM_ALIGNMENT: u64 = SZ_512K;
const ECC_MCA_SIGNAL_ENABLE_MASK: u64 = 0xff;

unsafe fn platform_to_str(platform: u32) -> *const ::core::ffi::c_char {
    match platform {
        IVPU_PLATFORM_SILICON => b"SILICON\0".as_ptr() as *const _,
        IVPU_PLATFORM_SIMICS => b"SIMICS\0".as_ptr() as *const _,
        IVPU_PLATFORM_FPGA => b"FPGA\0".as_ptr() as *const _,
        IVPU_PLATFORM_HSLE => b"HSLE\0".as_ptr() as *const _,
        _ => b"Invalid platform\0".as_ptr() as *const _,
    }
}

unsafe fn platform_init(vdev: *mut ivpu_device) {
    let platform = ivpu_hw_btrs_platform_read(vdev);
    ivpu_dbg(vdev, MISC, b"Platform type: %s (%d)\n\0".as_ptr() as _, platform_to_str(platform), platform);
    match platform {
        IVPU_PLATFORM_SILICON | IVPU_PLATFORM_SIMICS | IVPU_PLATFORM_FPGA | IVPU_PLATFORM_HSLE => (*vdev).platform = platform,
        _ => ivpu_err(vdev, b"Invalid platform type: %d\n\0".as_ptr() as _, platform),
    }
}

unsafe fn wa_init(vdev: *mut ivpu_device) {
    (*vdev).wa.punit_disabled = false;
    (*vdev).wa.clear_runtime_mem = false;
    if ivpu_hw_btrs_gen(vdev) == IVPU_HW_BTRS_MTL { (*vdev).wa.interrupt_clear_with_0 = ivpu_hw_btrs_irqs_clear_with_0_mtl(vdev); }
    if (ivpu_device_id(vdev) == PCI_DEVICE_ID_LNL && ivpu_revision(vdev) < IVPU_HW_IP_REV_LNL_B0) ||
       (ivpu_device_id(vdev) == PCI_DEVICE_ID_NVL && ivpu_revision(vdev) == IVPU_HW_IP_REV_NVL_A0) { (*vdev).wa.disable_clock_relinquish = true; }
    if ivpu_test_mode & IVPU_TEST_MODE_CLK_RELINQ_ENABLE != 0 { (*vdev).wa.disable_clock_relinquish = false; }
    if ivpu_test_mode & IVPU_TEST_MODE_CLK_RELINQ_DISABLE != 0 { (*vdev).wa.disable_clock_relinquish = true; }
    if ivpu_hw_ip_gen(vdev) == IVPU_HW_IP_37XX { (*vdev).wa.wp0_during_power_up = true; }
    if ivpu_test_mode & IVPU_TEST_MODE_D0I2_DISABLE != 0 { (*vdev).wa.disable_d0i2 = true; }
    IVPU_PRINT_WA!(vdev, punit_disabled); IVPU_PRINT_WA!(vdev, clear_runtime_mem);
    IVPU_PRINT_WA!(vdev, interrupt_clear_with_0); IVPU_PRINT_WA!(vdev, disable_clock_relinquish);
    IVPU_PRINT_WA!(vdev, wp0_during_power_up); IVPU_PRINT_WA!(vdev, disable_d0i2);
}

unsafe fn timeouts_init(vdev: *mut ivpu_device) {
    if ivpu_test_mode & IVPU_TEST_MODE_DISABLE_TIMEOUTS != 0 {
        (*vdev).timeout.boot = -1; (*vdev).timeout.jsm = -1; (*vdev).timeout.tdr = -1;
        (*vdev).timeout.inference = -1; (*vdev).timeout.autosuspend = -1; (*vdev).timeout.d0i3_entry_msg = -1;
    } else if ivpu_is_fpga(vdev) {
        (*vdev).timeout.boot=50; (*vdev).timeout.jsm=15000; (*vdev).timeout.tdr=30000; (*vdev).timeout.inference=900000;
        (*vdev).timeout.autosuspend=-1; (*vdev).timeout.d0i3_entry_msg=500; (*vdev).timeout.state_dump_msg=10000;
    } else if ivpu_is_simics(vdev) {
        (*vdev).timeout.boot=50; (*vdev).timeout.jsm=500; (*vdev).timeout.tdr=10000; (*vdev).timeout.inference=300000;
        (*vdev).timeout.autosuspend=100; (*vdev).timeout.d0i3_entry_msg=100; (*vdev).timeout.state_dump_msg=10;
    } else {
        (*vdev).timeout.boot=1000; (*vdev).timeout.jsm=500; (*vdev).timeout.tdr=2000; (*vdev).timeout.inference=60000;
        (*vdev).timeout.autosuspend = if ivpu_hw_ip_gen(vdev)==IVPU_HW_IP_37XX {10} else {100};
        (*vdev).timeout.d0i3_entry_msg=5; (*vdev).timeout.state_dump_msg=100;
    }
}

unsafe fn priority_bands_init(vdev: *mut ivpu_device) {
    (*vdev).hw.hws.grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_IDLE]=0; (*vdev).hw.hws.process_grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_IDLE]=50000; (*vdev).hw.hws.process_quantum[VPU_JOB_SCHEDULING_PRIORITY_BAND_IDLE]=160000;
    (*vdev).hw.hws.grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_NORMAL]=50000; (*vdev).hw.hws.process_grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_NORMAL]=50000; (*vdev).hw.hws.process_quantum[VPU_JOB_SCHEDULING_PRIORITY_BAND_NORMAL]=300000;
    (*vdev).hw.hws.grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_FOCUS]=50000; (*vdev).hw.hws.process_grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_FOCUS]=50000; (*vdev).hw.hws.process_quantum[VPU_JOB_SCHEDULING_PRIORITY_BAND_FOCUS]=200000;
    (*vdev).hw.hws.grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_REALTIME]=0; (*vdev).hw.hws.process_grace_period[VPU_JOB_SCHEDULING_PRIORITY_BAND_REALTIME]=50000; (*vdev).hw.hws.process_quantum[VPU_JOB_SCHEDULING_PRIORITY_BAND_REALTIME]=200000;
}

pub unsafe fn ivpu_hw_range_init(vdev: *mut ivpu_device, range: *mut ivpu_addr_range, start: u64, size: u64) -> i32 {
    let end = match start.checked_add(size) { Some(v) => v, None => { ivpu_err(vdev,b"Invalid range: start 0x%llx size %llu\n\0".as_ptr() as _,start,size); return -EINVAL; } };
    if range.is_null() { ivpu_err(vdev,b"Invalid range: start 0x%llx size %llu\n\0".as_ptr() as _,start,size); return -EINVAL; }
    (*range).start=start; (*range).end=end; 0
}

unsafe fn memory_ranges_init(vdev: *mut ivpu_device) {
    if ivpu_hw_ip_gen(vdev)==IVPU_HW_IP_37XX {
        ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.runtime,0x84800000,SZ_64M); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.global,0x90000000,SZ_256M); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.user,0xa0000000,511*SZ_1M); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.shave,0x180000000,SZ_2G); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.dma,0x200000000,SZ_128G);
    } else { ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.runtime,0x80000000,SZ_64M); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.global,0x90000000,SZ_256M); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.shave,0x80000000,SZ_2G); ivpu_hw_range_init(vdev,&mut (*(*vdev).hw).ranges.user,0x100000000,SZ_256G); (*(*vdev).hw).ranges.dma=(*(*vdev).hw).ranges.user; }
    drm_WARN_ON!(&(*vdev).drm, (*(*vdev).hw).ranges.global.start % FW_SHARED_MEM_ALIGNMENT != 0);
}

unsafe fn wp_enable(vdev:*mut ivpu_device)->i32 { ivpu_hw_btrs_wp_drive(vdev,true) }
unsafe fn wp_disable(vdev:*mut ivpu_device)->i32 { ivpu_hw_btrs_wp_drive(vdev,false) }

pub unsafe fn ivpu_hw_power_up(vdev:*mut ivpu_device)->i32 {
    let mut ret;
    if IVPU_WA!(vdev,wp0_during_power_up) { ret=wp_disable(vdev); if ret!=0 {ivpu_warn(vdev,b"Failed to disable workpoint: %d\n\0".as_ptr() as _,ret);} }
    ret=ivpu_hw_btrs_d0i3_disable(vdev); if ret!=0 {ivpu_warn(vdev,b"Failed to disable D0I3: %d\n\0".as_ptr() as _,ret);}
    ret=wp_enable(vdev); if ret!=0 {ivpu_err(vdev,b"Failed to enable workpoint: %d\n\0".as_ptr() as _,ret);return ret;}
    if ivpu_hw_btrs_gen(vdev)>=IVPU_HW_BTRS_LNL { if IVPU_WA!(vdev,disable_clock_relinquish) {ivpu_hw_btrs_clock_relinquish_disable_lnl(vdev);} ivpu_hw_btrs_profiling_freq_reg_set_lnl(vdev); ivpu_hw_btrs_ats_print_lnl(vdev); }
    ret=ivpu_hw_ip_host_ss_configure(vdev); if ret!=0 {ivpu_err(vdev,b"Failed to configure host SS: %d\n\0".as_ptr() as _,ret);return ret;} ivpu_hw_ip_idle_gen_disable(vdev);
    ret=ivpu_hw_btrs_wait_for_clock_res_own_ack(vdev); if ret!=0 {ivpu_err(vdev,b"Timed out waiting for clock resource own ACK\n\0".as_ptr() as _);return ret;}
    ret=ivpu_hw_ip_pwr_domain_enable(vdev); if ret!=0 {ivpu_err(vdev,b"Failed to enable power domain: %d\n\0".as_ptr() as _,ret);return ret;}
    ret=ivpu_hw_ip_host_ss_axi_enable(vdev); if ret!=0 {ivpu_err(vdev,b"Failed to enable AXI: %d\n\0".as_ptr() as _,ret);return ret;}
    if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_LNL {ivpu_hw_btrs_set_port_arbitration_weights_lnl(vdev);} ret=ivpu_hw_ip_top_noc_enable(vdev); if ret!=0 {ivpu_err(vdev,b"Failed to enable TOP NOC: %d\n\0".as_ptr() as _,ret);} ret
}

unsafe fn save_d0i3_entry_timestamp(vdev:*mut ivpu_device){(*(*vdev).hw).d0i3_entry_host_ts=ktime_get_boottime();(*(*vdev).hw).d0i3_entry_vpu_ts=ivpu_hw_ip_read_perf_timer_counter(vdev);}
pub unsafe fn ivpu_hw_reset(vdev:*mut ivpu_device)->i32 {let mut ret=0;if ivpu_hw_btrs_ip_reset(vdev){ivpu_err(vdev,b"Failed to reset NPU IP\n\0".as_ptr() as _);ret=-EIO;}if wp_disable(vdev)!=0{ivpu_err(vdev,b"Failed to disable workpoint\n\0".as_ptr() as _);ret=-EIO;}ret}
pub unsafe fn ivpu_hw_power_down(vdev:*mut ivpu_device)->i32 {let mut ret=0;save_d0i3_entry_timestamp(vdev);if !ivpu_hw_is_idle(vdev){ivpu_warn(vdev,b"NPU not idle during power down\n\0".as_ptr() as _);}if ivpu_hw_reset(vdev)!=0{ivpu_err(vdev,b"Failed to reset NPU\n\0".as_ptr() as _);ret=-EIO;}if ivpu_hw_btrs_d0i3_enable(vdev)!=0{ivpu_err(vdev,b"Failed to enter D0I3\n\0".as_ptr() as _);ret=-EIO;}ret}

pub unsafe fn ivpu_hw_init(vdev:*mut ivpu_device)->i32{ivpu_hw_btrs_info_init(vdev);ivpu_hw_btrs_freq_ratios_init(vdev);priority_bands_init(vdev);memory_ranges_init(vdev);platform_init(vdev);wa_init(vdev);timeouts_init(vdev);atomic_set!(&mut (*(*vdev).hw).firewall_irq_counter,0);0}
pub unsafe fn ivpu_hw_boot_fw(vdev:*mut ivpu_device)->i32{ivpu_hw_ip_snoop_disable(vdev);ivpu_hw_ip_tbu_mmu_enable(vdev);let ret=ivpu_hw_ip_soc_cpu_boot(vdev);if ret!=0{ivpu_err(vdev,b"Failed to boot SOC CPU: %d\n\0".as_ptr() as _,ret);}ret}
pub unsafe fn ivpu_hw_profiling_freq_drive(vdev:*mut ivpu_device,enable:bool){if ivpu_hw_ip_gen(vdev)==IVPU_HW_IP_37XX{(*(*vdev).hw).pll.profiling_freq=PLL_PROFILING_FREQ_DEFAULT;return;}(*(*vdev).hw).pll.profiling_freq=if enable{PLL_PROFILING_FREQ_HIGH}else{PLL_PROFILING_FREQ_DEFAULT};}
pub unsafe fn ivpu_irq_handlers_init(vdev:*mut ivpu_device){(*(*vdev).hw).irq.ip_irq_handler=if ivpu_hw_ip_gen(vdev)==IVPU_HW_IP_37XX{ivpu_hw_ip_irq_handler_37xx}else{ivpu_hw_ip_irq_handler_40xx};(*(*vdev).hw).irq.btrs_irq_handler=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{ivpu_hw_btrs_irq_handler_mtl}else{ivpu_hw_btrs_irq_handler_lnl};}
pub unsafe fn ivpu_hw_irq_enable(vdev:*mut ivpu_device){ivpu_hw_ip_irq_enable(vdev);ivpu_hw_btrs_irq_enable(vdev)}
pub unsafe fn ivpu_hw_irq_disable(vdev:*mut ivpu_device){ivpu_hw_btrs_irq_disable(vdev);ivpu_hw_ip_irq_disable(vdev)}

pub unsafe fn ivpu_hw_irq_handler(irq:i32,ptr:*mut ::core::ffi::c_void)->irqreturn_t{let vdev=ptr as *mut ivpu_device;ivpu_hw_btrs_global_int_disable(vdev);let btrs_handled=ivpu_hw_btrs_irq_handler(vdev,irq);let ip_handled=if !ivpu_hw_is_idle(vdev)||!btrs_handled{ivpu_hw_ip_irq_handler(vdev,irq)}else{false};ivpu_hw_btrs_global_int_enable(vdev);if !ip_handled&&!btrs_handled{return IRQ_NONE;}pm_runtime_mark_last_busy((*vdev).drm.dev);if ip_handled{IRQ_WAKE_THREAD}else{IRQ_HANDLED}}

pub unsafe fn ivpu_hw_uses_ecc_mca_signal(vdev:*mut ivpu_device)->bool{if ivpu_hw_ip_gen(vdev)<IVPU_HW_IP_50XX{return false;}let mut msr_integrity_caps=0u64;let ret=rdmsrq_safe(MSR_INTEGRITY_CAPS,&mut msr_integrity_caps);if ret!=0{ivpu_warn(vdev,b"Error reading MSR_INTEGRITY_CAPS: %d\0".as_ptr() as _,ret);return false;}ivpu_dbg(vdev,MISC,b"MSR_INTEGRITY_CAPS: 0x%llx\n\0".as_ptr() as _,msr_integrity_caps);msr_integrity_caps&ECC_MCA_SIGNAL_ENABLE_MASK!=0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
