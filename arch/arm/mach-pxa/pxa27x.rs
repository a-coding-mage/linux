// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-pxa/pxa27x.c
 *
 *  Code specific to PXA27x aka Bulverde.
 */

// Kernel headers and symbols referenced below are supplied by other translation units.

pub unsafe fn pxa27x_clear_otgph() {
    if cpu_is_pxa27x() && (PSSR & PSSR_OTGPH) != 0 {
        PSSR |= PSSR_OTGPH;
    }
}

static mut AC97_RESET_CONFIG: [c_ulong; 4] = [
    GPIO113_AC97_nRESET_GPIO_HIGH,
    GPIO113_AC97_nRESET,
    GPIO95_AC97_nRESET_GPIO_HIGH,
    GPIO95_AC97_nRESET,
];

pub unsafe fn pxa27x_configure_ac97reset(gpiod: *mut gpio_desc, to_gpio: bool) {
    if gpiod.is_null() { return; }
    let reset_gpio = desc_to_gpio(gpiod);
    /* Work around the PXA27x AC97 warm-reset configuration bug. */
    if reset_gpio == 113 {
        pxa2xx_mfp_config(if to_gpio { &AC97_RESET_CONFIG[0] } else { &AC97_RESET_CONFIG[1] }, 1);
    }
    if reset_gpio == 95 {
        pxa2xx_mfp_config(if to_gpio { &AC97_RESET_CONFIG[2] } else { &AC97_RESET_CONFIG[3] }, 1);
    }
}

// Preserved when CONFIG_PM is enabled.
#[cfg(CONFIG_PM)]
mod pm {
    use super::*;

    pub const PWRMODE: c_uint = PWRMODE_SLEEP;
    pub const SLEEP_SAVE_PSTR: usize = 0;
    pub const SLEEP_SAVE_MDREFR: usize = 1;
    pub const SLEEP_SAVE_PCFR: usize = 2;
    pub const SLEEP_SAVE_COUNT: usize = 3;

    unsafe fn pxa27x_cpu_pm_save(sleep_save: *mut c_ulong) {
        *sleep_save.add(SLEEP_SAVE_MDREFR) = __raw_readl(MDREFR);
        *sleep_save.add(SLEEP_SAVE_PCFR) = PCFR;
        *sleep_save.add(SLEEP_SAVE_PSTR) = PSTR;
    }

    unsafe fn pxa27x_cpu_pm_restore(sleep_save: *mut c_ulong) {
        __raw_writel(*sleep_save.add(SLEEP_SAVE_MDREFR), MDREFR);
        PCFR = *sleep_save.add(SLEEP_SAVE_PCFR);
        PSSR = PSSR_RDH | PSSR_PH;
        PSTR = *sleep_save.add(SLEEP_SAVE_PSTR);
    }

    unsafe fn pxa27x_cpu_pm_enter(state: suspend_state_t) {
        #[cfg(not(CONFIG_IWMMXT))]
        let mut acc0: u64 = 0;
        PCFR &= !PCFR_FVC;
        PEDR = 0xDF12FE1B;
        RCSR = RCSR_HWR | RCSR_WDR | RCSR_SMR | RCSR_GPR;
        match state {
            PM_SUSPEND_STANDBY => pxa_cpu_standby(),
            PM_SUSPEND_MEM => {
                cpu_suspend(PWRMODE, pxa27x_finish_suspend);
                #[cfg(not(CONFIG_IWMMXT))]
                { let _ = &mut acc0; }
            }
            _ => {}
        }
    }

    fn pxa27x_cpu_pm_valid(state: suspend_state_t) -> c_int {
        (state == PM_SUSPEND_MEM || state == PM_SUSPEND_STANDBY) as c_int
    }

    unsafe fn pxa27x_cpu_pm_prepare() -> c_int { PSPR = __pa_symbol(cpu_resume); 0 }
    unsafe fn pxa27x_cpu_pm_finish() { PSPR = 0; }

    static mut PXA27X_CPU_PM_FNS: pxa_cpu_pm_fns = pxa_cpu_pm_fns {
        save_count: SLEEP_SAVE_COUNT, save: Some(pxa27x_cpu_pm_save),
        restore: Some(pxa27x_cpu_pm_restore), valid: Some(pxa27x_cpu_pm_valid),
        enter: Some(pxa27x_cpu_pm_enter), prepare: Some(pxa27x_cpu_pm_prepare),
        finish: Some(pxa27x_cpu_pm_finish),
    };

    pub unsafe fn pxa27x_init_pm() { pxa_cpu_pm_fns = &mut PXA27X_CPU_PM_FNS; }
}

#[cfg(not(CONFIG_PM))]
unsafe fn pxa27x_init_pm() {}

unsafe fn pxa27x_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    let gpio = pxa_irq_to_gpio((*d).irq);
    if gpio >= 0 && gpio < 128 { return gpio_set_wake(gpio, on); }
    if (*d).irq == IRQ_KEYPAD { return keypad_set_wake(on); }
    let mask = match (*d).irq {
        IRQ_RTCAlrm => PWER_RTC,
        IRQ_USB => 1u32 << 26,
        _ => return -EINVAL,
    };
    if on != 0 { PWER |= mask; } else { PWER &= !mask; }
    0
}

pub unsafe fn pxa27x_init_irq() {
    pxa_init_irq(34, Some(pxa27x_set_wake));
    set_handle_irq(pxa27x_handle_irq);
}

unsafe fn pxa27x_dt_init_irq(node: *mut device_node, parent: *mut device_node) -> c_int {
    let _ = (node, parent);
    pxa_dt_irq_init(Some(pxa27x_set_wake));
    set_handle_irq(ichp_handle_irq);
    0
}

// IRQCHIP_DECLARE(pxa27x_intc, "marvell,pxa-intc", pxa27x_dt_init_irq);

static mut PXA27X_IO_DESC: [map_desc; 2] = [
    map_desc { virtual_: SMEMC_VIRT as c_ulong, pfn: __phys_to_pfn(PXA2XX_SMEMC_BASE), length: SMEMC_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: UNCACHED_PHYS_0, pfn: __phys_to_pfn(0), length: UNCACHED_PHYS_0_SIZE, type_: MT_DEVICE },
];

pub unsafe fn pxa27x_map_io() {
    pxa_map_io();
    iotable_init(PXA27X_IO_DESC.as_ptr(), PXA27X_IO_DESC.len());
    pxa27x_get_clk_frequency_khz(1);
}

pub unsafe fn pxa27x_set_i2c_power_info(info: *mut i2c_pxa_platform_data) {
    local_irq_disable(); PCFR |= PCFR_PI2CEN; local_irq_enable();
    pxa_register_device(&pxa27x_device_i2c_power, info);
}

static mut DEVICES: [*mut platform_device; 14] = [
    &mut pxa27x_device_gpio, &mut pxa27x_device_udc, &mut pxa_device_pmu,
    &mut pxa_device_i2s, &mut pxa_device_asoc_ssp1, &mut pxa_device_asoc_ssp2,
    &mut pxa_device_asoc_ssp3, &mut pxa_device_asoc_platform, &mut pxa_device_rtc,
    &mut pxa27x_device_ssp1, &mut pxa27x_device_ssp2, &mut pxa27x_device_ssp3,
    &mut pxa27x_device_pwm0, &mut pxa27x_device_pwm1,
];

// PXA25x/PXA27x/PXA3xx and PXA27x-specific DMA requestor mappings.
static PXA27X_SLAVE_MAP: [dma_slave_map; 20] = [
    dma_slave_map { dev_name: "pxa2xx-ac97", slave_name: "pcm_pcm_mic_mono", param: PDMA_FILTER_PARAM(LOWEST, 8) },
    dma_slave_map { dev_name: "pxa2xx-ac97", slave_name: "pcm_pcm_aux_mono_in", param: PDMA_FILTER_PARAM(LOWEST, 9) },
    dma_slave_map { dev_name: "pxa2xx-ac97", slave_name: "pcm_pcm_aux_mono_out", param: PDMA_FILTER_PARAM(LOWEST, 10) },
    dma_slave_map { dev_name: "pxa2xx-ac97", slave_name: "pcm_pcm_stereo_in", param: PDMA_FILTER_PARAM(LOWEST, 11) },
    dma_slave_map { dev_name: "pxa2xx-ac97", slave_name: "pcm_pcm_stereo_out", param: PDMA_FILTER_PARAM(LOWEST, 12) },
    dma_slave_map { dev_name: "pxa-ssp-dai.0", slave_name: "rx", param: PDMA_FILTER_PARAM(LOWEST, 13) },
    dma_slave_map { dev_name: "pxa-ssp-dai.0", slave_name: "tx", param: PDMA_FILTER_PARAM(LOWEST, 14) },
    dma_slave_map { dev_name: "pxa-ssp-dai.1", slave_name: "rx", param: PDMA_FILTER_PARAM(LOWEST, 15) },
    dma_slave_map { dev_name: "pxa-ssp-dai.1", slave_name: "tx", param: PDMA_FILTER_PARAM(LOWEST, 16) },
    dma_slave_map { dev_name: "pxa2xx-ir", slave_name: "rx", param: PDMA_FILTER_PARAM(LOWEST, 17) },
    dma_slave_map { dev_name: "pxa2xx-ir", slave_name: "tx", param: PDMA_FILTER_PARAM(LOWEST, 18) },
    dma_slave_map { dev_name: "pxa2xx-mci.0", slave_name: "rx", param: PDMA_FILTER_PARAM(LOWEST, 21) },
    dma_slave_map { dev_name: "pxa2xx-mci.0", slave_name: "tx", param: PDMA_FILTER_PARAM(LOWEST, 22) },
    dma_slave_map { dev_name: "pxa-ssp-dai.2", slave_name: "rx", param: PDMA_FILTER_PARAM(LOWEST, 66) },
    dma_slave_map { dev_name: "pxa-ssp-dai.2", slave_name: "tx", param: PDMA_FILTER_PARAM(LOWEST, 67) },
    dma_slave_map { dev_name: "pxa2xx-i2s", slave_name: "rx", param: PDMA_FILTER_PARAM(LOWEST, 2) },
    dma_slave_map { dev_name: "pxa2xx-i2s", slave_name: "tx", param: PDMA_FILTER_PARAM(LOWEST, 3) },
    dma_slave_map { dev_name: "pxa27x-camera.0", slave_name: "CI_Y", param: PDMA_FILTER_PARAM(HIGHEST, 68) },
    dma_slave_map { dev_name: "pxa27x-camera.0", slave_name: "CI_U", param: PDMA_FILTER_PARAM(HIGHEST, 69) },
    dma_slave_map { dev_name: "pxa27x-camera.0", slave_name: "CI_V", param: PDMA_FILTER_PARAM(HIGHEST, 70) },
];

static mut PXA27X_DMA_PDATA: mmp_dma_platdata = mmp_dma_platdata {
    dma_channels: 32, nb_requestors: 75, slave_map: PXA27X_SLAVE_MAP.as_ptr(), slave_map_cnt: 20,
};

unsafe fn pxa27x_init() -> c_int {
    let mut ret = 0;
    if cpu_is_pxa27x() {
        pxa_register_wdt(RCSR); pxa27x_init_pm();
        register_syscore(&pxa_irq_syscore); register_syscore(&pxa2xx_mfp_syscore);
        if !of_have_populated_dt() {
            software_node_register(&pxa2xx_gpiochip_node);
            pxa27x_device_gpio.dev.fwnode = software_node_fwnode(&pxa2xx_gpiochip_node);
            pxa2xx_set_dmac_info(&mut PXA27X_DMA_PDATA);
            ret = platform_add_devices(DEVICES.as_ptr(), DEVICES.len());
        }
    }
    ret
}

// postcore_initcall(pxa27x_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
