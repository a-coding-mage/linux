// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on sound/arm/pxa2xx-ac97.c and sound/soc/pxa/pxa2xx-ac97.c
 * which contain:
 *
 * Author:	Nicolas Pitre
 * Created:	Dec 02, 2004
 * Copyright:	MontaVista Software Inc.
 */

/* Dependencies from the original C includes:
 * linux/kernel.h, linux/platform_device.h, linux/interrupt.h, linux/clk.h,
 * linux/delay.h, linux/gpio/consumer.h, linux/module.h, linux/io.h,
 * linux/soc/pxa/cpu.h, sound/pxa2xx-lib.h,
 * linux/platform_data/asoc-pxa.h, pxa2xx-ac97-regs.h, pxa2xx-lib.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u32 = u32;
type bool_ = bool;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cpu_is_pxa25x() -> bool_;
    fn cpu_is_pxa27x() -> bool_;
    fn cpu_is_pxa3xx() -> bool_;
    fn pxa27x_configure_ac97reset(gpio: *mut gpio_desc, to_gpio: bool_);

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn wait_event_timeout(
        wq: *mut wait_queue_head_t,
        condition: c_ulong,
        timeout: c_ulong,
    ) -> c_ulong;

    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn udelay(usecs: c_ulong);
    fn mdelay(msecs: c_ulong);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn snd_BUG();

    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);

    fn devm_platform_ioremap_resource(dev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn platform_get_irq(dev: *mut platform_device, num: c_uint) -> c_int;
    fn request_irq(
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_uint, dev: *mut c_void);

    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

type c_long = isize;

unsafe extern "C" {
    static AC97_GPIO_STATUS: u16;
    static GSR_CDONE: u32;
    static GSR_SDONE: u32;
    static GSR_PCR: u32;
    static GSR_SCR: u32;
    static GCR_WARM_RST: u32;
    static GCR_COLD_RST: u32;
    static GCR_CLKBPB: u32;
    static GCR_PRIRDY_IEN: u32;
    static GCR_SECRDY_IEN: u32;
    static GCR_SDONE_IE: u32;
    static GCR_CDONE_IE: u32;
    static GCR_ACLINK_OFF: u32;
    static MISR_EOC: u32;
    static PISR_EOC: u32;
    static MCSR_EOC: u32;

    static SMC_REG_BASE: usize;
    static PMC_REG_BASE: usize;
    static SAC_REG_BASE: usize;
    static PAC_REG_BASE: usize;
    static GSR: usize;
    static GCR: usize;
    static MISR: usize;
    static PISR: usize;
    static MCSR: usize;
    static MODR: usize;

    static ENODEV: c_int;
    static ETIMEDOUT: c_int;
    static EIO: c_int;
    static IRQ_HANDLED: irqreturn_t;
    static IRQ_NONE: irqreturn_t;
    static GPIOD_OUT_HIGH: c_uint;
}

static mut car_mutex: mutex = mutex { _private: [] };
static mut gsr_wq: wait_queue_head_t = wait_queue_head_t { _private: [] };
static mut gsr_bits: c_long = 0;
static mut ac97_clk: *mut clk = core::ptr::null_mut();
static mut ac97conf_clk: *mut clk = core::ptr::null_mut();
pub static mut rst_gpio: *mut gpio_desc = core::ptr::null_mut();
static mut ac97_reg_base: *mut c_void = core::ptr::null_mut();

#[inline]
unsafe fn reg_offset(offset: usize) -> *mut c_void {
    unsafe { (ac97_reg_base as *mut u8).add(offset) as *mut c_void }
}

/*
 * Beware PXA27x bugs:
 *
 *   o Slot 12 read from modem space will hang controller.
 *   o CDONE, SDONE interrupt fails after any slot 12 IO.
 *
 * We therefore have an hybrid approach for waiting on SDONE (interrupt or
 * 1 jiffy timeout if interrupt never comes).
 */

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_read(slot: c_int, reg: u16) -> c_int {
    let mut val: c_int = unsafe { -ENODEV };
    let mut reg_addr: *mut u32;

    if slot > 0 {
        return unsafe { -ENODEV };
    }

    unsafe { mutex_lock(&raw mut car_mutex) };

    /* set up primary or secondary codec space */
    unsafe {
        if cpu_is_pxa25x() && reg == AC97_GPIO_STATUS {
            reg_addr = reg_offset(if slot != 0 { SMC_REG_BASE } else { PMC_REG_BASE }) as *mut u32;
        } else {
            reg_addr = reg_offset(if slot != 0 { SAC_REG_BASE } else { PAC_REG_BASE }) as *mut u32;
        }
        reg_addr = reg_addr.add((reg >> 1) as usize);

        /* start read access across the ac97 link */
        writel(GSR_CDONE | GSR_SDONE, reg_offset(GSR));
        gsr_bits = 0;
        val = (readl(reg_addr as *const c_void) & 0xffff) as c_int;
        if reg == AC97_GPIO_STATUS {
            mutex_unlock(&raw mut car_mutex);
            return val;
        }
        if wait_event_timeout(
            &raw mut gsr_wq,
            ((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong) & GSR_SDONE as c_ulong,
            1,
        ) <= 0
            && !(((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong)
                & GSR_SDONE as c_ulong
                != 0)
        {
            printk(
                c"%s: read error (ac97_reg=%d GSR=%#lx)\n".as_ptr(),
                c"pxa2xx_ac97_read".as_ptr(),
                reg as c_int,
                (readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong,
            );
            mutex_unlock(&raw mut car_mutex);
            return -ETIMEDOUT;
        }

        /* valid data now */
        writel(GSR_CDONE | GSR_SDONE, reg_offset(GSR));
        gsr_bits = 0;
        val = (readl(reg_addr as *const c_void) & 0xffff) as c_int;
        /* but we've just started another cycle... */
        wait_event_timeout(
            &raw mut gsr_wq,
            ((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong) & GSR_SDONE as c_ulong,
            1,
        );
        mutex_unlock(&raw mut car_mutex);
    }
    val
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_write(slot: c_int, reg: u16, val: u16) -> c_int {
    let mut reg_addr: *mut u32;
    let mut ret: c_int = 0;

    unsafe { mutex_lock(&raw mut car_mutex) };

    /* set up primary or secondary codec space */
    unsafe {
        if cpu_is_pxa25x() && reg == AC97_GPIO_STATUS {
            reg_addr = reg_offset(if slot != 0 { SMC_REG_BASE } else { PMC_REG_BASE }) as *mut u32;
        } else {
            reg_addr = reg_offset(if slot != 0 { SAC_REG_BASE } else { PAC_REG_BASE }) as *mut u32;
        }
        reg_addr = reg_addr.add((reg >> 1) as usize);

        writel(GSR_CDONE | GSR_SDONE, reg_offset(GSR));
        gsr_bits = 0;
        writel(val as u32, reg_addr as *mut c_void);
        if wait_event_timeout(
            &raw mut gsr_wq,
            ((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong) & GSR_CDONE as c_ulong,
            1,
        ) <= 0
            && !(((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong)
                & GSR_CDONE as c_ulong
                != 0)
        {
            printk(
                c"%s: write error (ac97_reg=%d GSR=%#lx)\n".as_ptr(),
                c"pxa2xx_ac97_write".as_ptr(),
                reg as c_int,
                (readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong,
            );
            ret = -EIO;
        }

        mutex_unlock(&raw mut car_mutex);
    }

    ret
}

/* CONFIG_PXA25x */
#[inline]
unsafe fn pxa_ac97_warm_pxa25x() {
    unsafe {
        gsr_bits = 0;

        writel(readl(reg_offset(GCR)) | GCR_WARM_RST, reg_offset(GCR));
    }
}

#[inline]
unsafe fn pxa_ac97_cold_pxa25x() {
    unsafe {
        writel(readl(reg_offset(GCR)) & GCR_COLD_RST, reg_offset(GCR)); /* clear everything but nCRST */
        writel(readl(reg_offset(GCR)) & !GCR_COLD_RST, reg_offset(GCR)); /* then assert nCRST */

        gsr_bits = 0;

        writel(GCR_COLD_RST, reg_offset(GCR));
    }
}

/* CONFIG_PXA27x */
#[inline]
unsafe fn pxa_ac97_warm_pxa27x() {
    unsafe {
        gsr_bits = 0;

        /* warm reset broken on Bulverde, so manually keep AC97 reset high */
        pxa27x_configure_ac97reset(rst_gpio, true);
        udelay(10);
        writel(readl(reg_offset(GCR)) | GCR_WARM_RST, reg_offset(GCR));
        pxa27x_configure_ac97reset(rst_gpio, false);
        udelay(500);
    }
}

#[inline]
unsafe fn pxa_ac97_cold_pxa27x() {
    unsafe {
        writel(readl(reg_offset(GCR)) & GCR_COLD_RST, reg_offset(GCR)); /* clear everything but nCRST */
        writel(readl(reg_offset(GCR)) & !GCR_COLD_RST, reg_offset(GCR)); /* then assert nCRST */

        gsr_bits = 0;

        /* PXA27x Developers Manual section 13.5.2.2.1 */
        clk_prepare_enable(ac97conf_clk);
        udelay(5);
        clk_disable_unprepare(ac97conf_clk);
        writel(GCR_COLD_RST | GCR_WARM_RST, reg_offset(GCR));
    }
}

/* CONFIG_PXA3xx */
#[inline]
unsafe fn pxa_ac97_warm_pxa3xx() {
    unsafe {
        gsr_bits = 0;

        /* Can't use interrupts */
        writel(readl(reg_offset(GCR)) | GCR_WARM_RST, reg_offset(GCR));
    }
}

#[inline]
unsafe fn pxa_ac97_cold_pxa3xx() {
    unsafe {
        /* Hold CLKBPB for 100us */
        writel(0, reg_offset(GCR));
        writel(GCR_CLKBPB, reg_offset(GCR));
        udelay(100);
        writel(0, reg_offset(GCR));

        writel(readl(reg_offset(GCR)) & GCR_COLD_RST, reg_offset(GCR)); /* clear everything but nCRST */
        writel(readl(reg_offset(GCR)) & !GCR_COLD_RST, reg_offset(GCR)); /* then assert nCRST */

        gsr_bits = 0;

        /* Can't use interrupts on PXA3xx */
        writel(
            readl(reg_offset(GCR)) & !(GCR_PRIRDY_IEN | GCR_SECRDY_IEN),
            reg_offset(GCR),
        );

        writel(GCR_WARM_RST | GCR_COLD_RST, reg_offset(GCR));
    }
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_try_warm_reset() -> bool_ {
    let mut gsr: c_ulong;
    let mut timeout: c_uint = 100;

    unsafe {
        /* CONFIG_PXA25x / CONFIG_PXA27x / CONFIG_PXA3xx conditional dispatch */
        if cpu_is_pxa25x() {
            pxa_ac97_warm_pxa25x();
        } else if cpu_is_pxa27x() {
            pxa_ac97_warm_pxa27x();
        } else if cpu_is_pxa3xx() {
            pxa_ac97_warm_pxa3xx();
        } else {
            snd_BUG();
        }

        while !(((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong)
            & (GSR_PCR | GSR_SCR) as c_ulong
            != 0)
            && timeout != 0
        {
            timeout = timeout.wrapping_sub(1);
            mdelay(1);
        }

        gsr = (readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong;
        if !(gsr & (GSR_PCR | GSR_SCR) as c_ulong != 0) {
            printk(
                c"%s: warm reset timeout (GSR=%#lx)\n".as_ptr(),
                c"pxa2xx_ac97_try_warm_reset".as_ptr(),
                gsr,
            );

            return false;
        }
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_try_cold_reset() -> bool_ {
    let mut gsr: c_ulong;
    let mut timeout: c_uint = 1000;

    unsafe {
        /* CONFIG_PXA25x / CONFIG_PXA27x / CONFIG_PXA3xx conditional dispatch */
        if cpu_is_pxa25x() {
            pxa_ac97_cold_pxa25x();
        } else if cpu_is_pxa27x() {
            pxa_ac97_cold_pxa27x();
        } else if cpu_is_pxa3xx() {
            pxa_ac97_cold_pxa3xx();
        } else {
            snd_BUG();
        }

        while !(((readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong)
            & (GSR_PCR | GSR_SCR) as c_ulong
            != 0)
            && timeout != 0
        {
            timeout = timeout.wrapping_sub(1);
            mdelay(1);
        }

        gsr = (readl(reg_offset(GSR)) as c_long | gsr_bits) as c_ulong;
        if !(gsr & (GSR_PCR | GSR_SCR) as c_ulong != 0) {
            printk(
                c"%s: cold reset timeout (GSR=%#lx)\n".as_ptr(),
                c"pxa2xx_ac97_try_cold_reset".as_ptr(),
                gsr,
            );

            return false;
        }
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_finish_reset() {
    unsafe {
        let mut gcr: u32 = readl(reg_offset(GCR));
        gcr &= !(GCR_PRIRDY_IEN | GCR_SECRDY_IEN);
        gcr |= GCR_SDONE_IE | GCR_CDONE_IE;
        writel(gcr, reg_offset(GCR));
    }
}

unsafe extern "C" fn pxa2xx_ac97_irq(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let status: c_long;

    unsafe {
        status = readl(reg_offset(GSR)) as c_long;
        if status != 0 {
            writel(status as u32, reg_offset(GSR));
            gsr_bits |= status;
            wake_up(&raw mut gsr_wq);

            /* Although we don't use those we still need to clear them
               since they tend to spuriously trigger when MMC is used
               (hardware bug? go figure)... */
            if cpu_is_pxa27x() {
                writel(MISR_EOC, reg_offset(MISR));
                writel(PISR_EOC, reg_offset(PISR));
                writel(MCSR_EOC, reg_offset(MCSR));
            }

            return IRQ_HANDLED;
        }

        IRQ_NONE
    }
}

/* CONFIG_PM */
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_suspend() -> c_int {
    unsafe {
        writel(readl(reg_offset(GCR)) | GCR_ACLINK_OFF, reg_offset(GCR));
        clk_disable_unprepare(ac97_clk);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_resume() -> c_int {
    unsafe {
        clk_prepare_enable(ac97_clk);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_probe(dev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let irq: c_int;

    unsafe {
        ac97_reg_base = devm_platform_ioremap_resource(dev, 0);
        if IS_ERR(ac97_reg_base) {
            return PTR_ERR(ac97_reg_base) as c_int;
        }

        if cpu_is_pxa27x() {
            /* Assert reset using GPIOD_OUT_HIGH, because reset is GPIO_ACTIVE_LOW */
            rst_gpio = devm_gpiod_get_optional(
                &raw mut (*dev).dev,
                c"reset".as_ptr(),
                GPIOD_OUT_HIGH,
            );
            if IS_ERR(rst_gpio as *const c_void) {
                return dev_err_probe(
                    &raw mut (*dev).dev,
                    PTR_ERR(rst_gpio as *const c_void),
                    c"reset gpio failed\n".as_ptr(),
                );
            }

            /*
             * This gpio is needed for a work-around to a bug in the ac97
             * controller during warm reset.  The direction and level is set
             * here so that it is an output driven high when switching from
             * AC97_nRESET alt function to generic gpio.
             */
            gpiod_set_consumer_name(rst_gpio, c"pxa27x ac97 reset".as_ptr());
            pxa27x_configure_ac97reset(rst_gpio, false);

            ac97conf_clk = clk_get(&raw mut (*dev).dev, c"AC97CONFCLK".as_ptr());
            if IS_ERR(ac97conf_clk as *const c_void) {
                ret = PTR_ERR(ac97conf_clk as *const c_void) as c_int;
                ac97conf_clk = core::ptr::null_mut();
                return ret;
            }
        }

        ac97_clk = clk_get(&raw mut (*dev).dev, c"AC97CLK".as_ptr());
        if IS_ERR(ac97_clk as *const c_void) {
            ret = PTR_ERR(ac97_clk as *const c_void) as c_int;
            ac97_clk = core::ptr::null_mut();
            if !ac97conf_clk.is_null() {
                clk_put(ac97conf_clk);
                ac97conf_clk = core::ptr::null_mut();
            }
            return ret;
        }

        ret = clk_prepare_enable(ac97_clk);
        if ret != 0 {
            clk_put(ac97_clk);
            ac97_clk = core::ptr::null_mut();
            if !ac97conf_clk.is_null() {
                clk_put(ac97conf_clk);
                ac97conf_clk = core::ptr::null_mut();
            }
            return ret;
        }

        irq = platform_get_irq(dev, 0);
        if irq < 0 {
            ret = irq;
            writel(readl(reg_offset(GCR)) | GCR_ACLINK_OFF, reg_offset(GCR));
            clk_put(ac97_clk);
            ac97_clk = core::ptr::null_mut();
            if !ac97conf_clk.is_null() {
                clk_put(ac97conf_clk);
                ac97conf_clk = core::ptr::null_mut();
            }
            return ret;
        }

        ret = request_irq(
            irq as c_uint,
            pxa2xx_ac97_irq,
            0,
            c"AC97".as_ptr(),
            core::ptr::null_mut(),
        );
        if ret < 0 {
            writel(readl(reg_offset(GCR)) | GCR_ACLINK_OFF, reg_offset(GCR));
            clk_put(ac97_clk);
            ac97_clk = core::ptr::null_mut();
            if !ac97conf_clk.is_null() {
                clk_put(ac97conf_clk);
                ac97conf_clk = core::ptr::null_mut();
            }
            return ret;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_hw_remove(dev: *mut platform_device) {
    unsafe {
        writel(readl(reg_offset(GCR)) | GCR_ACLINK_OFF, reg_offset(GCR));
        free_irq(platform_get_irq(dev, 0) as c_uint, core::ptr::null_mut());
        if !ac97conf_clk.is_null() {
            clk_put(ac97conf_clk);
            ac97conf_clk = core::ptr::null_mut();
        }
        clk_disable_unprepare(ac97_clk);
        clk_put(ac97_clk);
        ac97_clk = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_read_modr() -> u32 {
    unsafe {
        if ac97_reg_base.is_null() {
            return 0;
        }

        readl(reg_offset(MODR))
    }
}
/* EXPORT_SYMBOL_GPL(pxa2xx_ac97_read_modr); */

#[no_mangle]
pub unsafe extern "C" fn pxa2xx_ac97_read_misr() -> u32 {
    unsafe {
        if ac97_reg_base.is_null() {
            return 0;
        }

        readl(reg_offset(MISR))
    }
}
/* EXPORT_SYMBOL_GPL(pxa2xx_ac97_read_misr); */

/* MODULE_AUTHOR("Nicolas Pitre"); */
/* MODULE_DESCRIPTION("Intel/Marvell PXA sound library"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
