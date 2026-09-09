// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap1/serial.c
 *
 * OMAP1 serial support.
 */

// C includes are supplied by the surrounding kernel translation unit.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut uart1_ck: *mut clk;
    static mut uart2_ck: *mut clk;
    static mut uart3_ck: *mut clk;

    fn __raw_readb(addr: *mut c_void) -> u8;
    fn __raw_writeb(value: c_int, addr: *mut c_void);
    fn cpu_is_omap15xx() -> bool;
    fn cpu_is_omap16xx() -> bool;
    fn cpu_class_is_omap1() -> bool;
    fn ioremap(mapbase: u64, size: usize) -> *mut c_void;
    fn printk(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn clk_get(dev: *mut c_void, name: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: u64) -> c_int;
    fn omap_cfg_reg(reg: c_int);
    fn gpiod_get_index(dev: *mut c_void, con_id: *const c_char, index: c_int, flags: c_int) -> *mut gpio_desc;
    fn gpiod_to_irq(desc: *mut gpio_desc) -> c_int;
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                   flags: c_int, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn gpiod_put(desc: *mut gpio_desc);
    fn enable_irq_wake(irq: c_int) -> c_int;
    fn platform_device_register(dev: *mut platform_device) -> c_int;
}

#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct irqreturn_t(pub c_int);

#[repr(C)]
pub struct plat_serial8250_port {
    pub mapbase: u64,
    pub irq: c_int,
    pub flags: u32,
    pub iotype: u32,
    pub regshift: u32,
    pub uartclk: u32,
    pub membase: *mut c_void,
}

#[repr(C)]
pub struct device { pub platform_data: *mut c_void }
#[repr(C)]
pub struct platform_device {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: device,
}

#[inline]
unsafe fn omap_serial_in(up: *mut plat_serial8250_port, offset: c_int) -> u32 {
    let offset = offset.wrapping_shl((*up).regshift);
    __raw_readb((*up).membase.wrapping_add(offset as usize)) as u32
}

#[inline]
unsafe fn omap_serial_outp(p: *mut plat_serial8250_port, offset: c_int, value: c_int) {
    let offset = offset.wrapping_shl((*p).regshift);
    __raw_writeb(value, (*p).membase.wrapping_add(offset as usize));
}

/*
 * Internal UARTs need to be initialized for the 8250 autoconfig to work
 * properly. Note that the TX watermark initialization may not be needed
 * once the 8250.c watermark handling code is merged.
 */
unsafe fn omap_serial_reset(p: *mut plat_serial8250_port) {
    omap_serial_outp(p, UART_OMAP_MDR1, UART_OMAP_MDR1_DISABLE);
    omap_serial_outp(p, UART_OMAP_SCR, 0x08);
    omap_serial_outp(p, UART_OMAP_MDR1, UART_OMAP_MDR1_16X_MODE);

    if !cpu_is_omap15xx() {
        omap_serial_outp(p, UART_OMAP_SYSC, 0x01);
        while (omap_serial_in(p, UART_OMAP_SYSC) & 0x01 == 0) {}
    }
}

static mut serial_platform_data: [plat_serial8250_port; 4] = [
    plat_serial8250_port { mapbase: OMAP1_UART1_BASE, irq: INT_UART1, flags: UPF_BOOT_AUTOCONF, iotype: UPIO_MEM, regshift: 2, uartclk: OMAP16XX_BASE_BAUD * 16, membase: core::ptr::null_mut() },
    plat_serial8250_port { mapbase: OMAP1_UART2_BASE, irq: INT_UART2, flags: UPF_BOOT_AUTOCONF, iotype: UPIO_MEM, regshift: 2, uartclk: OMAP16XX_BASE_BAUD * 16, membase: core::ptr::null_mut() },
    plat_serial8250_port { mapbase: OMAP1_UART3_BASE, irq: INT_UART3, flags: UPF_BOOT_AUTOCONF, iotype: UPIO_MEM, regshift: 2, uartclk: OMAP16XX_BASE_BAUD * 16, membase: core::ptr::null_mut() },
    plat_serial8250_port { mapbase: 0, irq: 0, flags: 0, iotype: 0, regshift: 0, uartclk: 0, membase: core::ptr::null_mut() },
];

static mut serial_device: platform_device = platform_device {
    name: b"serial8250\0".as_ptr() as *const c_char,
    id: PLAT8250_DEV_PLATFORM,
    dev: device { platform_data: unsafe { &mut serial_platform_data as *mut _ as *mut c_void } },
};

/*
 * Note that on Innovator-1510 UART2 pins conflict with USB2.
 * By default UART2 does not work on Innovator-1510 if you have
 * USB OHCI enabled. To use UART2, you must disable USB2 first.
 */
pub unsafe fn omap_serial_init() {
    let mut i: usize;
    if cpu_is_omap15xx() {
        serial_platform_data[0].uartclk = OMAP1510_BASE_BAUD * 16;
        serial_platform_data[1].uartclk = OMAP1510_BASE_BAUD * 16;
        serial_platform_data[2].uartclk = OMAP1510_BASE_BAUD * 16;
    }
    i = 0;
    while i < serial_platform_data.len() - 1 {
        serial_platform_data[i].membase = ioremap(serial_platform_data[i].mapbase, SZ_2K);
        if serial_platform_data[i].membase.is_null() {
            printk(b"Could not ioremap uart%i\n\0".as_ptr() as *const c_char, i);
            i += 1;
            continue;
        }
        match i {
            0 => { uart1_ck = clk_get(core::ptr::null_mut(), b"uart1_ck\0".as_ptr() as *const c_char); if IS_ERR(uart1_ck) { printk(b"Could not get uart1_ck\n\0".as_ptr() as *const c_char); } else { clk_prepare_enable(uart1_ck); if cpu_is_omap15xx() { clk_set_rate(uart1_ck, 12000000); } } }
            1 => { uart2_ck = clk_get(core::ptr::null_mut(), b"uart2_ck\0".as_ptr() as *const c_char); if IS_ERR(uart2_ck) { printk(b"Could not get uart2_ck\n\0".as_ptr() as *const c_char); } else { clk_prepare_enable(uart2_ck); if cpu_is_omap15xx() { clk_set_rate(uart2_ck, 12000000); } else { clk_set_rate(uart2_ck, 48000000); } } }
            2 => { uart3_ck = clk_get(core::ptr::null_mut(), b"uart3_ck\0".as_ptr() as *const c_char); if IS_ERR(uart3_ck) { printk(b"Could not get uart3_ck\n\0".as_ptr() as *const c_char); } else { clk_prepare_enable(uart3_ck); if cpu_is_omap15xx() { clk_set_rate(uart3_ck, 12000000); } } }
            _ => {}
        }
        omap_serial_reset(&mut serial_platform_data[i]);
        i += 1;
    }
}

#[cfg(CONFIG_OMAP_SERIAL_WAKE)]
unsafe extern "C" fn omap_serial_wake_interrupt(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t { IRQ_HANDLED }

#[cfg(CONFIG_OMAP_SERIAL_WAKE)]
pub unsafe fn omap_serial_wake_trigger(enable: c_int) {
    if !cpu_is_omap16xx() { return; }
    if !uart1_ck.is_null() { omap_cfg_reg(if enable != 0 { V14_16XX_GPIO37 } else { V14_16XX_UART1_RX }); }
    if !uart2_ck.is_null() { omap_cfg_reg(if enable != 0 { R9_16XX_GPIO18 } else { R9_16XX_UART2_RX }); }
    if !uart3_ck.is_null() { omap_cfg_reg(if enable != 0 { L14_16XX_GPIO49 } else { L14_16XX_UART3_RX }); }
}

#[cfg(CONFIG_OMAP_SERIAL_WAKE)]
unsafe fn omap_serial_set_port_wakeup(idx: c_int) {
    let d = gpiod_get_index(core::ptr::null_mut(), b"wakeup\0".as_ptr() as *const c_char, idx, GPIOD_IN);
    if IS_ERR(d as *mut clk) { pr_err(b"Unable to get UART wakeup GPIO descriptor\n\0".as_ptr() as *const c_char); return; }
    let irq = gpiod_to_irq(d);
    let ret = request_irq(irq, omap_serial_wake_interrupt, IRQF_TRIGGER_RISING, b"serial wakeup\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if ret != 0 { gpiod_put(d); pr_err(b"No interrupt for UART%d wake GPIO\n\0".as_ptr() as *const c_char, idx + 1); return; }
    enable_irq_wake(irq);
}

#[cfg(CONFIG_OMAP_SERIAL_WAKE)]
pub unsafe fn omap_serial_wakeup_init() -> c_int {
    if !cpu_is_omap16xx() { return 0; }
    if !uart1_ck.is_null() { omap_serial_set_port_wakeup(0); }
    if !uart2_ck.is_null() { omap_serial_set_port_wakeup(1); }
    if !uart3_ck.is_null() { omap_serial_set_port_wakeup(2); }
    0
}

unsafe fn omap_init() -> c_int {
    if !cpu_class_is_omap1() { return -ENODEV; }
    platform_device_register(&mut serial_device)
}

// arch_initcall(omap_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
