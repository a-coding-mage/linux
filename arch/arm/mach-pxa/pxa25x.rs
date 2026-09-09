// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-pxa/pxa25x.c
 *
 *  Author: Nicolas Pitre
 *  Created: Jun 15, 2001
 *  Copyright: MontaVista Software Inc.
 *
 * Code specific to PXA21x/25x/26x variants.
 *
 * Since this file should be linked before any other machine specific file,
 * the __initcall() here will be executed first. This serves as default
 * initialization stuff for PXA machines which can be overridden later if
 * need be.
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_PM")]
const SLEEP_SAVE_PSTR: usize = 0;
#[cfg(feature = "CONFIG_PM")]
const SLEEP_SAVE_COUNT: usize = 1;

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa25x_cpu_pm_save(sleep_save: *mut libc::c_ulong) {
    *sleep_save.add(SLEEP_SAVE_PSTR) = PSTR;
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa25x_cpu_pm_restore(sleep_save: *mut libc::c_ulong) {
    PSTR = *sleep_save.add(SLEEP_SAVE_PSTR);
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa25x_cpu_pm_enter(state: suspend_state_t) {
    // Clear reset status
    RCSR = RCSR_HWR | RCSR_WDR | RCSR_SMR | RCSR_GPR;

    match state {
        PM_SUSPEND_MEM => cpu_suspend(PWRMODE_SLEEP, pxa25x_finish_suspend),
        _ => (),
    }
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa25x_cpu_pm_prepare() -> libc::c_int {
    // set resume return address
    PSPR = __pa_symbol(cpu_resume);
    0
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa25x_cpu_pm_finish() {
    // ensure not to come back here if it wasn't intended
    PSPR = 0;
}

#[cfg(feature = "CONFIG_PM")]
static mut PXA25X_CPU_PM_FNS: pxa_cpu_pm_fns = pxa_cpu_pm_fns {
    save_count: SLEEP_SAVE_COUNT,
    valid: Some(suspend_valid_only_mem),
    save: Some(pxa25x_cpu_pm_save),
    restore: Some(pxa25x_cpu_pm_restore),
    enter: Some(pxa25x_cpu_pm_enter),
    prepare: Some(pxa25x_cpu_pm_prepare),
    finish: Some(pxa25x_cpu_pm_finish),
};

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa25x_init_pm() {
    pxa_cpu_pm_fns = &raw mut PXA25X_CPU_PM_FNS;
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
unsafe fn pxa25x_init_pm() {}

// PXA25x: supports wakeup from GPIO0..GPIO15 and RTC alarm
unsafe fn pxa25x_set_wake(d: *mut irq_data, on: libc::c_uint) -> libc::c_int {
    let gpio = pxa_irq_to_gpio((*d).irq);
    let mut mask: u32 = 0;

    if gpio >= 0 && gpio < 85 {
        return gpio_set_wake(gpio, on);
    }

    if (*d).irq == IRQ_RTCAlrm {
        mask = PWER_RTC;
    } else {
        return -EINVAL;
    }

    if on != 0 {
        PWER |= mask;
    } else {
        PWER &= !mask;
    }

    0
}

unsafe fn pxa25x_init_irq() {
    pxa_init_irq(32, pxa25x_set_wake);
    set_handle_irq(pxa25x_handle_irq);
}

unsafe fn pxa25x_dt_init_irq(
    node: *mut device_node,
    parent: *mut device_node,
) -> libc::c_int {
    let _ = (node, parent);
    pxa_dt_irq_init(pxa25x_set_wake);
    set_handle_irq(icip_handle_irq);
    0
}

IRQCHIP_DECLARE!(pxa25x_intc, "marvell,pxa-intc", pxa25x_dt_init_irq);

static mut PXA25X_IO_DESC: [map_desc; 2] = [
    map_desc {
        // Mem Ctl
        virtual_: SMEMC_VIRT as libc::c_ulong,
        pfn: __phys_to_pfn(PXA2XX_SMEMC_BASE),
        length: SMEMC_SIZE,
        type_: MT_DEVICE,
    },
    map_desc {
        // UNCACHED_PHYS_0
        virtual_: UNCACHED_PHYS_0,
        pfn: __phys_to_pfn(0x00000000),
        length: UNCACHED_PHYS_0_SIZE,
        type_: MT_DEVICE,
    },
];

unsafe fn pxa25x_map_io() {
    pxa_map_io();
    iotable_init(PXA25X_IO_DESC.as_mut_ptr(), PXA25X_IO_DESC.len());
    pxa25x_get_clk_frequency_khz(1);
}

static mut PXA25X_DEVICES: [*mut platform_device; 11] = [
    &raw mut pxa25x_device_gpio,
    &raw mut pxa25x_device_udc,
    &raw mut pxa_device_pmu,
    &raw mut pxa_device_i2s,
    &raw mut sa1100_device_rtc,
    &raw mut pxa25x_device_ssp,
    &raw mut pxa25x_device_nssp,
    &raw mut pxa25x_device_assp,
    &raw mut pxa25x_device_pwm0,
    &raw mut pxa25x_device_pwm1,
    &raw mut pxa_device_asoc_platform,
];

static PXA25X_SLAVE_MAP: [dma_slave_map; 18] = [
    dma_slave_map { slave: "pxa2xx-ac97", peripheral: "pcm_pcm_mic_mono", param: PDMA_FILTER_PARAM!(LOWEST, 8) },
    dma_slave_map { slave: "pxa2xx-ac97", peripheral: "pcm_pcm_aux_mono_in", param: PDMA_FILTER_PARAM!(LOWEST, 9) },
    dma_slave_map { slave: "pxa2xx-ac97", peripheral: "pcm_pcm_aux_mono_out", param: PDMA_FILTER_PARAM!(LOWEST, 10) },
    dma_slave_map { slave: "pxa2xx-ac97", peripheral: "pcm_pcm_stereo_in", param: PDMA_FILTER_PARAM!(LOWEST, 11) },
    dma_slave_map { slave: "pxa2xx-ac97", peripheral: "pcm_pcm_stereo_out", param: PDMA_FILTER_PARAM!(LOWEST, 12) },
    dma_slave_map { slave: "pxa-ssp-dai.1", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 13) },
    dma_slave_map { slave: "pxa-ssp-dai.1", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 14) },
    dma_slave_map { slave: "pxa-ssp-dai.2", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 15) },
    dma_slave_map { slave: "pxa-ssp-dai.2", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 16) },
    dma_slave_map { slave: "pxa2xx-ir", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 17) },
    dma_slave_map { slave: "pxa2xx-ir", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 18) },
    dma_slave_map { slave: "pxa2xx-mci.0", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 21) },
    dma_slave_map { slave: "pxa2xx-mci.0", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 22) },
    dma_slave_map { slave: "pxa25x-ssp.0", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 13) },
    dma_slave_map { slave: "pxa25x-ssp.0", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 14) },
    dma_slave_map { slave: "pxa25x-nssp.1", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 15) },
    dma_slave_map { slave: "pxa25x-nssp.1", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 16) },
    dma_slave_map { slave: "pxa25x-nssp.2", peripheral: "rx", param: PDMA_FILTER_PARAM!(LOWEST, 23) },
    dma_slave_map { slave: "pxa25x-nssp.2", peripheral: "tx", param: PDMA_FILTER_PARAM!(LOWEST, 24) },
];

static mut PXA25X_DMA_PDATA: mmp_dma_platdata = mmp_dma_platdata {
    dma_channels: 16,
    nb_requestors: 40,
    slave_map: PXA25X_SLAVE_MAP.as_ptr(),
    slave_map_cnt: PXA25X_SLAVE_MAP.len(),
};

unsafe fn pxa25x_init() -> libc::c_int {
    let mut ret: libc::c_int = 0;

    if cpu_is_pxa25x() {
        pxa_register_wdt(RCSR);
        pxa25x_init_pm();
        register_syscore(&raw mut pxa_irq_syscore);
        register_syscore(&raw mut pxa2xx_mfp_syscore);

        if !of_have_populated_dt() {
            software_node_register(&raw mut pxa2xx_gpiochip_node);
            pxa25x_device_gpio.dev.fwnode = software_node_fwnode(
                &raw mut pxa2xx_gpiochip_node,
            );
            pxa2xx_set_dmac_info(&raw mut PXA25X_DMA_PDATA);
            ret = platform_add_devices(PXA25X_DEVICES.as_mut_ptr(), PXA25X_DEVICES.len());
        }
    }

    ret
}

postcore_initcall!(pxa25x_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
