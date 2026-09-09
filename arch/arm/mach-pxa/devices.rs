// SPDX-License-Identifier: GPL-2.0
// C dependencies and build-time configuration are supplied by the surrounding kernel translation.

pub unsafe fn pxa_register_device(dev: *mut platform_device, data: *mut core::ffi::c_void) {
    (*dev).dev.platform_data = data;
    let ret = platform_device_register(dev);
    if ret != 0 { dev_err(&mut (*dev).dev, c"unable to register device: %d\n", ret); }
}

static mut PXA_RESOURCE_PMU: resource = resource { start: IRQ_PMU, end: IRQ_PMU, flags: IORESOURCE_IRQ, ..resource::ZERO };
pub static mut pxa_device_pmu: platform_device = platform_device { name: c"xscale-pmu", id: -1, resource: unsafe { &raw mut PXA_RESOURCE_PMU }, num_resources: 1, ..platform_device::ZERO };

static mut PXAMCI_RESOURCES: [resource; 2] = [
    resource { start: 0x41100000, end: 0x41100fff, flags: IORESOURCE_MEM, ..resource::ZERO },
    resource { start: IRQ_MMC, end: IRQ_MMC, flags: IORESOURCE_IRQ, ..resource::ZERO },
];

pub unsafe fn pxa_set_mci_info(info: *const pxamci_platform_data, props: *const property_entry) {
    let mci_info = platform_device_info { name: c"pxa2xx-mci", id: 0,
        res: PXAMCI_RESOURCES.as_ptr(), num_res: PXAMCI_RESOURCES.len(), data: info as *const _,
        size_data: core::mem::size_of::<pxamci_platform_data>(), dma_mask: 0xffffffff,
        properties: props, ..platform_device_info::ZERO };
    let mci_dev = platform_device_register_full(&mci_info);
    let err = PTR_ERR_OR_ZERO(mci_dev);
    if err != 0 { pr_err(c"Unable to create mci device: %d\n", err); }
}

static mut PXA2XX_UDC_RESOURCES: [resource; 2] = [
    resource { start: 0x40600000, end: 0x4060ffff, flags: IORESOURCE_MEM, ..resource::ZERO },
    resource { start: IRQ_USB, end: IRQ_USB, flags: IORESOURCE_IRQ, ..resource::ZERO },
];
static mut UDC_DMA_MASK: u64 = !0u32 as u64;
pub static mut pxa25x_device_udc: platform_device = platform_device { name: c"pxa25x-udc", id: -1, resource: unsafe { PXA2XX_UDC_RESOURCES.as_mut_ptr() }, num_resources: 2, dev: device { dma_mask: unsafe { &raw mut UDC_DMA_MASK }, ..device::ZERO }, ..platform_device::ZERO };
pub static mut pxa27x_device_udc: platform_device = platform_device { name: c"pxa27x-udc", id: -1, resource: unsafe { PXA2XX_UDC_RESOURCES.as_mut_ptr() }, num_resources: 2, dev: device { dma_mask: unsafe { &raw mut UDC_DMA_MASK }, ..device::ZERO }, ..platform_device::ZERO };

static mut PXAFB_RESOURCES: [resource; 2] = [
    resource { start: 0x44000000, end: 0x4400ffff, flags: IORESOURCE_MEM, ..resource::ZERO },
    resource { start: IRQ_LCD, end: IRQ_LCD, flags: IORESOURCE_IRQ, ..resource::ZERO },
];
static mut FB_DMA_MASK: u64 = !0;
pub static mut pxa_device_fb: platform_device = platform_device { name: c"pxa2xx-fb", id: -1,
    dev: device { dma_mask: unsafe { &raw mut FB_DMA_MASK }, coherent_dma_mask: 0xffffffff, ..device::ZERO },
    num_resources: 2, resource: unsafe { PXAFB_RESOURCES.as_mut_ptr() }, ..platform_device::ZERO };
pub unsafe fn pxa_set_fb_info(parent: *mut device, info: *mut pxafb_mach_info) { pxa_device_fb.dev.parent = parent; pxa_register_device(&raw mut pxa_device_fb, info.cast()); }

macro_rules! uart_device { ($res:ident, $base:expr, $end:expr, $irq:ident, $dev:ident, $id:expr) => {
    static mut $res: [resource; 2] = [resource { start: $base, end: $end, flags: IORESOURCE_MEM, ..resource::ZERO }, resource { start: $irq, end: $irq, flags: IORESOURCE_IRQ, ..resource::ZERO }];
    pub static mut $dev: platform_device = platform_device { name: c"pxa2xx-uart", id: $id, resource: unsafe { $res.as_mut_ptr() }, num_resources: 2, ..platform_device::ZERO };
} }
uart_device!(PXA_RESOURCE_FFUART, 0x40100000, 0x40100023, IRQ_FFUART, pxa_device_ffuart, 0);
uart_device!(PXA_RESOURCE_BTUART, 0x40200000, 0x40200023, IRQ_BTUART, pxa_device_btuart, 1);
uart_device!(PXA_RESOURCE_STUART, 0x40700000, 0x40700023, IRQ_STUART, pxa_device_stuart, 2);
uart_device!(PXA_RESOURCE_HWUART, 0x41600000, 0x4160002f, IRQ_HWUART, pxa_device_hwuart, 3);
pub unsafe fn pxa_set_ffuart_info(info: *mut core::ffi::c_void) { pxa_register_device(&raw mut pxa_device_ffuart, info); }
pub unsafe fn pxa_set_btuart_info(info: *mut core::ffi::c_void) { pxa_register_device(&raw mut pxa_device_btuart, info); }
pub unsafe fn pxa_set_stuart_info(info: *mut core::ffi::c_void) { pxa_register_device(&raw mut pxa_device_stuart, info); }
pub unsafe fn pxa_set_hwuart_info(info: *mut core::ffi::c_void) { if cpu_is_pxa255() { pxa_register_device(&raw mut pxa_device_hwuart, info); } else { pr_info(c"UART: Ignoring attempt to register HWUART on non-PXA255 hardware"); } }

static mut PXAI2C_RESOURCES: [resource; 2] = [resource { start: 0x40301680, end: 0x403016a3, flags: IORESOURCE_MEM, ..resource::ZERO }, resource { start: IRQ_I2C, end: IRQ_I2C, flags: IORESOURCE_IRQ, ..resource::ZERO }];
pub static mut pxa_device_i2c: platform_device = platform_device { name: c"pxa2xx-i2c", id: 0, resource: unsafe { PXAI2C_RESOURCES.as_mut_ptr() }, num_resources: 2, ..platform_device::ZERO };
pub unsafe fn pxa_set_i2c_info(info: *mut i2c_pxa_platform_data) { pxa_register_device(&raw mut pxa_device_i2c, info.cast()); }

#[cfg(feature = "CONFIG_PXA27x")]
pub static mut pxa27x_device_i2c_power: platform_device = platform_device { name: c"pxa2xx-i2c", id: 1, resource: core::ptr::null_mut(), num_resources: 0, ..platform_device::ZERO };

static mut PXAI2S_RESOURCES: [resource; 2] = [resource { start: 0x40400000, end: 0x40400083, flags: IORESOURCE_MEM, ..resource::ZERO }, resource { start: IRQ_I2S, end: IRQ_I2S, flags: IORESOURCE_IRQ, ..resource::ZERO }];
pub static mut pxa_device_i2s: platform_device = platform_device { name: c"pxa2xx-i2s", id: -1, resource: unsafe { PXAI2S_RESOURCES.as_mut_ptr() }, num_resources: 2, ..platform_device::ZERO };
pub static mut pxa_device_asoc_ssp1: platform_device = platform_device { name: c"pxa-ssp-dai", id: 0, ..platform_device::ZERO };
pub static mut pxa_device_asoc_ssp2: platform_device = platform_device { name: c"pxa-ssp-dai", id: 1, ..platform_device::ZERO };
pub static mut pxa_device_asoc_ssp3: platform_device = platform_device { name: c"pxa-ssp-dai", id: 2, ..platform_device::ZERO };
pub static mut pxa_device_asoc_ssp4: platform_device = platform_device { name: c"pxa-ssp-dai", id: 3, ..platform_device::ZERO };
pub static mut pxa_device_asoc_platform: platform_device = platform_device { name: c"pxa-pcm-audio", id: -1, ..platform_device::ZERO };

static mut PXA_RTC_RESOURCES: [resource; 3] = [resource { start: 0x40900000, end: 0x40900000 + 0x3b, flags: IORESOURCE_MEM, ..resource::ZERO }, resource { start: IRQ_RTC1Hz, end: IRQ_RTC1Hz, name: c"rtc 1Hz", flags: IORESOURCE_IRQ, ..resource::ZERO }, resource { start: IRQ_RTCAlrm, end: IRQ_RTCAlrm, name: c"rtc alarm", flags: IORESOURCE_IRQ, ..resource::ZERO }];
pub static mut pxa_device_rtc: platform_device = platform_device { name: c"pxa-rtc", id: -1, num_resources: 3, resource: unsafe { PXA_RTC_RESOURCES.as_mut_ptr() }, ..platform_device::ZERO };
pub static mut sa1100_device_rtc: platform_device = platform_device { name: c"sa1100-rtc", id: -1, num_resources: 3, resource: unsafe { PXA_RTC_RESOURCES.as_mut_ptr() }, ..platform_device::ZERO };

#[cfg(feature = "CONFIG_PXA25x")]
macro_rules! simple_mem_device { ($res:ident, $dev:ident, $name:expr, $id:expr, $base:expr, $last:expr) => {
    static mut $res: [resource; 1] = [resource { start: $base, end: $last, flags: IORESOURCE_MEM, ..resource::ZERO }];
    pub static mut $dev: platform_device = platform_device { name: $name, id: $id, resource: unsafe { $res.as_mut_ptr() }, num_resources: 1, ..platform_device::ZERO };
} }
#[cfg(feature = "CONFIG_PXA25x")]
simple_mem_device!(PXA25X_RESOURCE_PWM0, pxa25x_device_pwm0, c"pxa25x-pwm", 0, 0x40b00000, 0x40b0000f);
#[cfg(feature = "CONFIG_PXA25x")]
simple_mem_device!(PXA25X_RESOURCE_PWM1, pxa25x_device_pwm1, c"pxa25x-pwm", 1, 0x40c00000, 0x40c0000f);

macro_rules! ssp_device { ($res:ident, $mask:ident, $dev:ident, $name:expr, $id:expr, $base:expr, $last:expr, $irq:ident) => {
    static mut $mask: u64 = 0xffffffff;
    static mut $res: [resource; 2] = [resource { start: $base, end: $last, flags: IORESOURCE_MEM, ..resource::ZERO }, resource { start: $irq, end: $irq, flags: IORESOURCE_IRQ, ..resource::ZERO }];
    pub static mut $dev: platform_device = platform_device { name: $name, id: $id, dev: device { dma_mask: unsafe { &raw mut $mask }, coherent_dma_mask: 0xffffffff, ..device::ZERO }, resource: unsafe { $res.as_mut_ptr() }, num_resources: 2, ..platform_device::ZERO };
} }
#[cfg(feature = "CONFIG_PXA25x")]
ssp_device!(PXA25X_RESOURCE_SSP, PXA25X_SSP_DMA_MASK, pxa25x_device_ssp, c"pxa25x-ssp", 0, 0x41000000, 0x4100001f, IRQ_SSP);
#[cfg(feature = "CONFIG_PXA25x")]
ssp_device!(PXA25X_RESOURCE_NSSP, PXA25X_NSSP_DMA_MASK, pxa25x_device_nssp, c"pxa25x-nssp", 1, 0x41400000, 0x4140002f, IRQ_NSSP);
#[cfg(feature = "CONFIG_PXA25x")]
ssp_device!(PXA25X_RESOURCE_ASSP, PXA25X_ASSP_DMA_MASK, pxa25x_device_assp, c"pxa25x-nssp", 2, 0x41500000, 0x4150002f, IRQ_ASSP);

#[cfg(any(feature = "CONFIG_PXA27x", feature = "CONFIG_PXA3xx"))]
ssp_device!(PXA27X_RESOURCE_SSP1, PXA27X_SSP1_DMA_MASK, pxa27x_device_ssp1, c"pxa27x-ssp", 0, 0x41000000, 0x4100003f, IRQ_SSP);
#[cfg(any(feature = "CONFIG_PXA27x", feature = "CONFIG_PXA3xx"))]
ssp_device!(PXA27X_RESOURCE_SSP2, PXA27X_SSP2_DMA_MASK, pxa27x_device_ssp2, c"pxa27x-ssp", 1, 0x41700000, 0x4170003f, IRQ_SSP2);
#[cfg(any(feature = "CONFIG_PXA27x", feature = "CONFIG_PXA3xx"))]
ssp_device!(PXA27X_RESOURCE_SSP3, PXA27X_SSP3_DMA_MASK, pxa27x_device_ssp3, c"pxa27x-ssp", 2, 0x41900000, 0x4190003f, IRQ_SSP3);
#[cfg(any(feature = "CONFIG_PXA27x", feature = "CONFIG_PXA3xx"))]
simple_mem_device!(PXA27X_RESOURCE_PWM0, pxa27x_device_pwm0, c"pxa27x-pwm", 0, 0x40b00000, 0x40b0001f);
#[cfg(any(feature = "CONFIG_PXA27x", feature = "CONFIG_PXA3xx"))]
simple_mem_device!(PXA27X_RESOURCE_PWM1, pxa27x_device_pwm1, c"pxa27x-pwm", 1, 0x40c00000, 0x40c0001f);

#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
pub static pxa2xx_gpiochip_node: software_node = software_node { name: c"gpio-pxa", ..software_node::ZERO };
#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
pub static mut pxa_resource_gpio: [resource; 4] = [
    resource { start: 0x40e00000, end: 0x40e0ffff, flags: IORESOURCE_MEM, ..resource::ZERO },
    resource { start: IRQ_GPIO0, end: IRQ_GPIO0, name: c"gpio0", flags: IORESOURCE_IRQ, ..resource::ZERO },
    resource { start: IRQ_GPIO1, end: IRQ_GPIO1, name: c"gpio1", flags: IORESOURCE_IRQ, ..resource::ZERO },
    resource { start: IRQ_GPIO_2_x, end: IRQ_GPIO_2_x, name: c"gpio_mux", flags: IORESOURCE_IRQ, ..resource::ZERO },
];
#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
static mut PXA2XX_GPIO_INFO: pxa_gpio_platform_data = pxa_gpio_platform_data { irq_base: PXA_GPIO_TO_IRQ(0), gpio_set_wake: Some(gpio_set_wake), ..pxa_gpio_platform_data::ZERO };
#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
pub static mut pxa25x_device_gpio: platform_device = platform_device { name: c"pxa25x-gpio", id: -1, num_resources: 4, resource: unsafe { pxa_resource_gpio.as_mut_ptr() }, dev: device { platform_data: unsafe { &raw mut PXA2XX_GPIO_INFO as *mut _ as *mut core::ffi::c_void }, ..device::ZERO }, ..platform_device::ZERO };
#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
pub static mut pxa27x_device_gpio: platform_device = platform_device { name: c"pxa27x-gpio", id: -1, num_resources: 4, resource: unsafe { pxa_resource_gpio.as_mut_ptr() }, dev: device { platform_data: unsafe { &raw mut PXA2XX_GPIO_INFO as *mut _ as *mut core::ffi::c_void }, ..device::ZERO }, ..platform_device::ZERO };

static mut PXA_DMA_RESOURCE: [resource; 2] = [resource { start: 0x40000000, end: 0x4000ffff, flags: IORESOURCE_MEM, ..resource::ZERO }, resource { start: IRQ_DMA, end: IRQ_DMA, flags: IORESOURCE_IRQ, ..resource::ZERO }];
static mut PXADMA_DMAMASK: u64 = 0xffffffff;
static mut PXA2XX_PXA_DMA: platform_device = platform_device { name: c"pxa-dma", id: 0, dev: device { dma_mask: unsafe { &raw mut PXADMA_DMAMASK }, coherent_dma_mask: 0xffffffff, ..device::ZERO }, num_resources: 2, resource: unsafe { PXA_DMA_RESOURCE.as_mut_ptr() }, ..platform_device::ZERO };
pub unsafe fn pxa2xx_set_dmac_info(dma_pdata: *mut mmp_dma_platdata) { pxa_register_device(&raw mut PXA2XX_PXA_DMA, dma_pdata.cast()); }
pub unsafe fn pxa_register_wdt(mut reset_status: u32) { let res = resource { start: OST_PHYS, end: OST_PHYS + OST_LEN - 1, flags: IORESOURCE_MEM, ..resource::ZERO }; reset_status &= RESET_STATUS_WATCHDOG; platform_device_register_resndata(core::ptr::null_mut(), c"sa1100_wdt", -1, &res, 1, &reset_status as *const _ as *const core::ffi::c_void, core::mem::size_of::<u32>()); }

extern "C" {
    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn PTR_ERR_OR_ZERO(ptr: *mut platform_device) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn cpu_is_pxa255() -> bool;
    fn platform_device_register_resndata(parent: *mut device, name: *const core::ffi::c_char, id: i32, res: *const resource, num: usize, data: *const core::ffi::c_void, size: usize);
    fn gpio_set_wake(_: u32, _: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
