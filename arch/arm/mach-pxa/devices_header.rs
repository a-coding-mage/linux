/* SPDX-License-Identifier: GPL-2.0 */

// C macro equivalent. `prio` is supplied as the corresponding PXAD priority
// constant by the caller; the original C token-pasting form is preserved in
// the macro's call shape.
#[macro_export]
macro_rules! PDMA_FILTER_PARAM {
    ($prio:expr, $requestor:expr) => {
        &mut pxad_param {
            prio: $prio,
            drcmr: $requestor,
        }
    };
}

pub struct platform_device;
pub struct mmp_dma_platdata;
pub struct pxad_param {
    pub prio: i32,
    pub drcmr: i32,
}
pub struct i2c_pxa_platform_data;
pub struct software_node;

extern "C" {
    pub static mut pxa_device_pmu: platform_device;
    pub static mut pxa3xx_device_mci2: platform_device;
    pub static mut pxa3xx_device_mci3: platform_device;
    pub static mut pxa25x_device_udc: platform_device;
    pub static mut pxa27x_device_udc: platform_device;
    pub static mut pxa_device_fb: platform_device;
    pub static mut pxa_device_ffuart: platform_device;
    pub static mut pxa_device_btuart: platform_device;
    pub static mut pxa_device_stuart: platform_device;
    pub static mut pxa_device_hwuart: platform_device;
    pub static mut pxa_device_i2c: platform_device;
    pub static mut pxa_device_i2s: platform_device;
    pub static mut sa1100_device_rtc: platform_device;
    pub static mut pxa_device_rtc: platform_device;
    pub static mut pxa_device_ac97: platform_device;

    pub static mut pxa27x_device_i2c_power: platform_device;
    pub static mut pxa27x_device_ohci: platform_device;
    pub static mut pxa27x_device_keypad: platform_device;

    pub static mut pxa25x_device_ssp: platform_device;
    pub static mut pxa25x_device_nssp: platform_device;
    pub static mut pxa25x_device_assp: platform_device;
    pub static mut pxa27x_device_ssp1: platform_device;
    pub static mut pxa27x_device_ssp2: platform_device;
    pub static mut pxa27x_device_ssp3: platform_device;
    pub static mut pxa3xx_device_ssp1: platform_device;
    pub static mut pxa3xx_device_ssp2: platform_device;
    pub static mut pxa3xx_device_ssp3: platform_device;
    pub static mut pxa3xx_device_ssp4: platform_device;

    pub static mut pxa25x_device_pwm0: platform_device;
    pub static mut pxa25x_device_pwm1: platform_device;
    pub static mut pxa27x_device_pwm0: platform_device;
    pub static mut pxa27x_device_pwm1: platform_device;

    pub static mut pxa3xx_device_nand: platform_device;
    pub static mut pxa3xx_device_i2c_power: platform_device;
    pub static mut pxa3xx_device_gcu: platform_device;

    pub static mut pxa_device_asoc_platform: platform_device;
    pub static mut pxa_device_asoc_ssp1: platform_device;
    pub static mut pxa_device_asoc_ssp2: platform_device;
    pub static mut pxa_device_asoc_ssp3: platform_device;
    pub static mut pxa_device_asoc_ssp4: platform_device;

    pub static mut pxa25x_device_gpio: platform_device;
    pub static mut pxa27x_device_gpio: platform_device;

    pub static pxa2xx_gpiochip_node: software_node;

    pub fn pxa_register_device(dev: *mut platform_device, data: *mut core::ffi::c_void);
    pub fn pxa2xx_set_dmac_info(dma_pdata: *mut mmp_dma_platdata);

    pub fn pxa_set_i2c_info(info: *mut i2c_pxa_platform_data);

    // Preserved build-time condition: CONFIG_PXA27x.
    #[cfg(CONFIG_PXA27x)]
    pub fn pxa27x_set_i2c_power_info(info: *mut i2c_pxa_platform_data);

    // Preserved build-time condition: CONFIG_PXA3xx.
    #[cfg(CONFIG_PXA3xx)]
    pub fn pxa3xx_set_i2c_power_info(info: *mut i2c_pxa_platform_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
