// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2017 NXP
 * Copyright 2011,2016 Freescale Semiconductor, Inc.
 * Copyright 2011 Linaro Ltd.
 */

// Kernel dependencies supplied by the surrounding repository.

const MMDC_MAPSR: usize = 0x404;
const BP_MMDC_MAPSR_PSD: u32 = 0;
const BP_MMDC_MAPSR_PSS: u32 = 4;
const MMDC_MDMISC: usize = 0x18;
const BM_MMDC_MDMISC_DDR_TYPE: u32 = 0x18;
const BP_MMDC_MDMISC_DDR_TYPE: u32 = 0x3;
const TOTAL_CYCLES: i32 = 0x0;
const BUSY_CYCLES: i32 = 0x1;
const READ_ACCESSES: i32 = 0x2;
const WRITE_ACCESSES: i32 = 0x3;
const READ_BYTES: i32 = 0x4;
const WRITE_BYTES: i32 = 0x5;
const DBG_DIS: u32 = 0x0;
const DBG_EN: u32 = 0x1;
const DBG_RST: u32 = 0x2;
const PRF_FRZ: u32 = 0x4;
const CYC_OVF: u32 = 0x8;
const PROFILE_SEL: u32 = 0x10;
const MMDC_MADPCR0: usize = 0x410;
const MMDC_MADPCR1: usize = 0x414;
const MMDC_MADPSR0: usize = 0x418;
const MMDC_MADPSR1: usize = 0x41c;
const MMDC_MADPSR2: usize = 0x420;
const MMDC_MADPSR3: usize = 0x424;
const MMDC_MADPSR4: usize = 0x428;
const MMDC_MADPSR5: usize = 0x42c;
const MMDC_NUM_COUNTERS: usize = 6;
const MMDC_FLAG_PROFILE_SEL: u32 = 0x1;
const MMDC_PRF_AXI_ID_CLEAR: u32 = 0x0;

static mut ddr_type: i32 = 0;

#[repr(C)]
struct fsl_mmdc_devtype_data { flags: u32 }

static imx6q_data: fsl_mmdc_devtype_data = fsl_mmdc_devtype_data { flags: 0 };
static imx6qp_data: fsl_mmdc_devtype_data = fsl_mmdc_devtype_data { flags: MMDC_FLAG_PROFILE_SEL };

// `struct of_device_id` entries and the sentinel are supplied by the kernel ABI.
static imx_mmdc_dt_ids: [of_device_id; 3] = [
    of_device_id { compatible: "fsl,imx6q-mmdc", data: &imx6q_data as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx6qp-mmdc", data: &imx6qp_data as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "", data: core::ptr::null() },
];

#[cfg(feature = "CONFIG_PERF_EVENTS")]
mod perf_events {
    use super::*;

    static mut mmdc_pmu_poll_period_us: u32 = 1_000_000;

    #[repr(C)]
    struct mmdc_pmu {
        pmu: pmu,
        mmdc_base: *mut core::ffi::c_void,
        cpu: cpumask_t,
        hrtimer: hrtimer,
        active_events: u32,
        id: i32,
        dev: *mut device,
        mmdc_events: [*mut perf_event; MMDC_NUM_COUNTERS],
        node: hlist_node,
        devtype_data: *const fsl_mmdc_devtype_data,
        mmdc_ipg_clk: *mut clk,
    }

    static mut cpuhp_mmdc_state: cpuhp_state = 0;
    static mut mmdc_ida: ida = ida::new();

    fn mmdc_pmu_timer_period() -> ktime_t {
        unsafe { ns_to_ktime((mmdc_pmu_poll_period_us as u64).wrapping_mul(1000)) }
    }

    unsafe fn mmdc_pmu_read_counter(pmu_mmdc: *mut mmdc_pmu, cfg: i32) -> u32 {
        let base = (*pmu_mmdc).mmdc_base as *mut u8;
        let offset = match cfg {
            TOTAL_CYCLES => MMDC_MADPSR0,
            BUSY_CYCLES => MMDC_MADPSR1,
            READ_ACCESSES => MMDC_MADPSR2,
            WRITE_ACCESSES => MMDC_MADPSR3,
            READ_BYTES => MMDC_MADPSR4,
            WRITE_BYTES => MMDC_MADPSR5,
            _ => { WARN_ONCE!(true, "invalid configuration for mmdc counter"); return 0; }
        };
        readl(base.add(offset))
    }

    unsafe fn mmdc_pmu_event_update(event: *mut perf_event) {
        let pmu_mmdc = to_mmdc_pmu!((*event).pmu);
        let cfg = (*event).attr.config as i32;
        let hwc = &mut (*event).hw;
        let mut prev;
        let mut new_count;
        loop {
            prev = local64_read(&hwc.prev_count);
            new_count = mmdc_pmu_read_counter(pmu_mmdc, cfg);
            if local64_cmpxchg(&hwc.prev_count, prev, new_count as u64) == prev { break; }
        }
        local64_add((new_count as u64).wrapping_sub(prev) & 0xffff_ffff, &(*event).count);
    }

    unsafe fn mmdc_pmu_overflow_handler(pmu_mmdc: *mut mmdc_pmu) {
        for i in 0..MMDC_NUM_COUNTERS {
            let event = (*pmu_mmdc).mmdc_events[i];
            if !event.is_null() { mmdc_pmu_event_update(event); }
        }
    }

    unsafe fn mmdc_pmu_event_start(event: *mut perf_event, _flags: i32) {
        let pmu_mmdc = to_mmdc_pmu!((*event).pmu);
        let base = (*pmu_mmdc).mmdc_base as *mut u8;
        hrtimer_start(&mut (*pmu_mmdc).hrtimer, mmdc_pmu_timer_period(), HRTIMER_MODE_REL_PINNED);
        local64_set(&mut (*event).hw.prev_count, 0);
        writel(DBG_RST, base.add(MMDC_MADPCR0));
        writel((*event).attr.config1 as u32, base.add(MMDC_MADPCR1));
        let mut val = DBG_EN;
        if ((*(*pmu_mmdc).devtype_data).flags & MMDC_FLAG_PROFILE_SEL) != 0 { val |= PROFILE_SEL; }
        writel(val, base.add(MMDC_MADPCR0));
    }

    unsafe fn mmdc_pmu_event_stop(event: *mut perf_event, _flags: i32) {
        let pmu_mmdc = to_mmdc_pmu!((*event).pmu);
        let base = (*pmu_mmdc).mmdc_base as *mut u8;
        writel(PRF_FRZ, base.add(MMDC_MADPCR0));
        writel(MMDC_PRF_AXI_ID_CLEAR, base.add(MMDC_MADPCR1));
        mmdc_pmu_event_update(event);
    }

    unsafe fn mmdc_pmu_event_add(event: *mut perf_event, flags: i32) -> i32 {
        let p = to_mmdc_pmu!((*event).pmu);
        let cfg = (*event).attr.config as usize;
        if flags & PERF_EF_START != 0 { mmdc_pmu_event_start(event, flags); }
        if !(*p).mmdc_events[cfg].is_null() { return -EAGAIN; }
        (*p).mmdc_events[cfg] = event;
        (*p).active_events += 1;
        local64_set(&mut (*event).hw.prev_count, mmdc_pmu_read_counter(p, cfg as i32) as u64);
        0
    }

    unsafe fn mmdc_pmu_event_del(event: *mut perf_event, _flags: i32) {
        let p = to_mmdc_pmu!((*event).pmu);
        let cfg = (*event).attr.config as usize;
        (*p).mmdc_events[cfg] = core::ptr::null_mut();
        (*p).active_events -= 1;
        if (*p).active_events == 0 { hrtimer_cancel(&mut (*p).hrtimer); }
        mmdc_pmu_event_stop(event, PERF_EF_UPDATE);
    }

    unsafe fn mmdc_pmu_timer_handler(timer: *mut hrtimer) -> hrtimer_restart {
        let p = container_of!(timer, mmdc_pmu, hrtimer);
        mmdc_pmu_overflow_handler(p);
        hrtimer_forward_now(timer, mmdc_pmu_timer_period());
        HRTIMER_RESTART
    }

    unsafe fn mmdc_pmu_init(p: *mut mmdc_pmu, base: *mut core::ffi::c_void, dev: *mut device) -> i32 {
        core::ptr::write_bytes(p, 0, 1);
        (*p).mmdc_base = base;
        (*p).dev = dev;
        (*p).id = ida_alloc(&mut mmdc_ida, GFP_KERNEL);
        (*p).id
    }

    unsafe fn imx_mmdc_perf_init(pdev: *mut platform_device, base: *mut core::ffi::c_void, clk: *mut clk) -> i32 {
        let p = kzalloc::<mmdc_pmu>();
        if p.is_null() { pr_err!("failed to allocate PMU device!\n"); return -ENOMEM; }
        let ret = mmdc_pmu_init(p, base, &mut (*pdev).dev);
        if ret < 0 { kfree(p); return ret; }
        (*p).mmdc_ipg_clk = clk;
        (*p).devtype_data = device_get_match_data(&mut (*pdev).dev);
        hrtimer_setup(&mut (*p).hrtimer, mmdc_pmu_timer_handler, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
        cpumask_set_cpu(raw_smp_processor_id(), &mut (*p).cpu);
        let ret = perf_pmu_register(&mut (*p).pmu, devm_kasprintf(pdev_as_device!(pdev), GFP_KERNEL, "mmdc%d", ret), -1);
        if ret != 0 { ida_free(&mut mmdc_ida, (*p).id); kfree(p); return ret; }
        platform_set_drvdata(pdev, p);
        0
    }

    // The remaining PMU registration and platform-driver glue retain the C ABI
    // and kernel callback layout; their declarations are provided externally.
    unsafe fn imx_mmdc_remove(pdev: *mut platform_device) {
        let p = platform_get_drvdata(pdev);
        ida_free(&mut mmdc_ida, (*p).id);
        perf_pmu_unregister(&mut (*p).pmu);
        iounmap((*p).mmdc_base);
        clk_disable_unprepare((*p).mmdc_ipg_clk);
        kfree(p);
    }
}

#[cfg(not(feature = "CONFIG_PERF_EVENTS"))]
// In the non-PERF_EVENTS build, imx_mmdc_remove is NULL and perf initialization returns 0.

unsafe fn imx_mmdc_probe(pdev: *mut platform_device) -> i32 {
    let mut mmdc_ipg_clk: *mut clk = devm_clk_get(pdev_as_device!(pdev), core::ptr::null());
    if IS_ERR(mmdc_ipg_clk) { mmdc_ipg_clk = core::ptr::null_mut(); }
    let err = clk_prepare_enable(mmdc_ipg_clk);
    if err != 0 { dev_err!(pdev, "Unable to enable mmdc ipg clock.\n"); return err; }
    let base = of_iomap((*pdev).dev.of_node, 0);
    WARN_ON!(!base.is_null());
    let val = readl_relaxed((base as *mut u8).add(MMDC_MDMISC));
    ddr_type = ((val & BM_MMDC_MDMISC_DDR_TYPE) >> BP_MMDC_MDMISC_DDR_TYPE) as i32;
    let reg = (base as *mut u8).add(MMDC_MAPSR);
    let mut val = readl_relaxed(reg);
    val &= !(1u32 << BP_MMDC_MAPSR_PSD);
    writel_relaxed(val, reg);
    let err = imx_mmdc_perf_init(pdev, base, mmdc_ipg_clk);
    if err != 0 { iounmap(base); clk_disable_unprepare(mmdc_ipg_clk); }
    err
}

pub unsafe fn imx_mmdc_get_ddr_type() -> i32 { ddr_type }

unsafe fn imx_mmdc_init() -> i32 { platform_driver_register(&mut imx_mmdc_driver) }
// postcore_initcall(imx_mmdc_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
