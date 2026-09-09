// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
//		http://www.samsung.com
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// Common Codes for S3C64XX machines

// NOTE: Code in this file is not used when booting with Device Tree support.

/* External clock frequency */
static mut xtal_f: ::core::ffi::c_ulong = 12000000;
static mut xusbxti_f: ::core::ffi::c_ulong = 48000000;

pub unsafe fn s3c64xx_set_xtal_freq(freq: ::core::ffi::c_ulong) { xtal_f = freq; }
pub unsafe fn s3c64xx_set_xusbxti_freq(freq: ::core::ffi::c_ulong) { xusbxti_f = freq; }

/* uart registration process */
unsafe fn s3c64xx_init_uarts(cfg: *mut s3c2410_uartcfg, no: ::core::ffi::c_int) {
    s3c24xx_init_uartdevs(b"s3c6400-uart\0".as_ptr() as *const _, s3c64xx_uart_resources, cfg, no);
}

/* table of supported CPUs */
static name_s3c6410: &[u8] = b"S3C6410\0";
static mut cpu_ids: [cpu_table; 1] = [cpu_table {
    idcode: S3C6410_CPU_ID,
    idmask: S3C64XX_CPU_MASK,
    map_io: Some(s3c6410_map_io),
    init_uarts: Some(s3c64xx_init_uarts),
    init: Some(s3c6410_init),
    name: name_s3c6410.as_ptr(),
}];

/* minimal IO mapping */
// UART_OFFS (S3C_PA_UART & 0xfffff)
const UART_OFFS: usize = S3C_PA_UART & 0xfffff;

static mut s3c_iodesc: [map_desc; 10] = [
    map_desc { virtual_: S3C_VA_SYS as _, pfn: __phys_to_pfn(S3C64XX_PA_SYSCON), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: S3C_VA_MEM as _, pfn: __phys_to_pfn(S3C64XX_PA_SROM), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: (S3C_VA_UART + UART_OFFS) as _, pfn: __phys_to_pfn(S3C_PA_UART), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: VA_VIC0 as _, pfn: __phys_to_pfn(S3C64XX_PA_VIC0), length: SZ_16K, type_: MT_DEVICE },
    map_desc { virtual_: VA_VIC1 as _, pfn: __phys_to_pfn(S3C64XX_PA_VIC1), length: SZ_16K, type_: MT_DEVICE },
    map_desc { virtual_: S3C_VA_TIMER as _, pfn: __phys_to_pfn(S3C_PA_TIMER), length: SZ_16K, type_: MT_DEVICE },
    map_desc { virtual_: S3C64XX_VA_GPIO as _, pfn: __phys_to_pfn(S3C64XX_PA_GPIO), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: S3C64XX_VA_MODEM as _, pfn: __phys_to_pfn(S3C64XX_PA_MODEM), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: S3C_VA_WATCHDOG as _, pfn: __phys_to_pfn(S3C64XX_PA_WATCHDOG), length: SZ_4K, type_: MT_DEVICE },
    map_desc { virtual_: S3C_VA_USB_HSPHY as _, pfn: __phys_to_pfn(S3C64XX_PA_USB_HSPHY), length: SZ_1K, type_: MT_DEVICE },
];

static s3c64xx_subsys: bus_type = bus_type { name: b"s3c64xx-core\0".as_ptr(), dev_name: b"s3c64xx-core\0".as_ptr() };
static mut s3c64xx_dev: device = device { bus: &raw const s3c64xx_subsys };
static mut s3c64xx_pwm_variant: samsung_pwm_variant = samsung_pwm_variant { bits: 32, div_base: 0, has_tint_cstat: true, tclk_mask: (1 << 7) | (1 << 6) | (1 << 5), ..samsung_pwm_variant::ZERO };

pub unsafe fn s3c64xx_set_timer_source(event: s3c64xx_timer_mode, source: s3c64xx_timer_mode) {
    s3c64xx_pwm_variant.output_mask = (1 << SAMSUNG_PWM_NUM) - 1;
    s3c64xx_pwm_variant.output_mask &= !((1 << event as u32) | (1 << source as u32));
}

pub unsafe fn s3c64xx_timer_init() {
    let timer_irqs: [u32; SAMSUNG_PWM_NUM as usize] = [IRQ_TIMER0_VIC, IRQ_TIMER1_VIC, IRQ_TIMER2_VIC, IRQ_TIMER3_VIC, IRQ_TIMER4_VIC];
    samsung_pwm_clocksource_init(S3C_VA_TIMER, timer_irqs.as_ptr(), &raw const s3c64xx_pwm_variant);
}

pub unsafe fn s3c64xx_init_io(mach_desc: *mut map_desc, size: ::core::ffi::c_int) {
    iotable_init(s3c_iodesc.as_ptr(), s3c_iodesc.len());
    iotable_init(mach_desc, size as usize);
    s3c64xx_init_cpu();
    s3c_init_cpu(samsung_cpu_id, cpu_ids.as_ptr(), cpu_ids.len());
    samsung_pwm_set_platdata(&raw const s3c64xx_pwm_variant);
}

unsafe fn s3c64xx_dev_init() -> ::core::ffi::c_int {
    if of_have_populated_dt() || !soc_is_s3c64xx() { return 0; }
    subsys_system_register(&raw const s3c64xx_subsys, core::ptr::null());
    device_register(&raw mut s3c64xx_dev)
}

const IRQ_VIC0_RESUME: u32 = 1 << (IRQ_RTC_TIC - IRQ_VIC0_BASE);
const IRQ_VIC1_RESUME: u32 = (1 << (IRQ_RTC_ALARM - IRQ_VIC1_BASE)) | (1 << (IRQ_PENDN - IRQ_VIC1_BASE)) | (1 << (IRQ_HSMMC0 - IRQ_VIC1_BASE)) | (1 << (IRQ_HSMMC1 - IRQ_VIC1_BASE)) | (1 << (IRQ_HSMMC2 - IRQ_VIC1_BASE));

pub unsafe fn s3c64xx_init_irq(vic0_valid: u32, vic1_valid: u32) {
    s3c64xx_clk_init(core::ptr::null(), xtal_f, xusbxti_f, soc_is_s3c6400(), S3C_VA_SYS);
    printk(KERN_DEBUG, b"%s: initialising interrupts\n\0".as_ptr(), b"s3c64xx_init_irq\0".as_ptr());
    vic_init(VA_VIC0, IRQ_VIC0_BASE, vic0_valid, IRQ_VIC0_RESUME);
    vic_init(VA_VIC1, IRQ_VIC1_BASE, vic1_valid, IRQ_VIC1_RESUME);
}

#[inline]
fn eint_offset(irq: u32) -> u32 { irq - IRQ_EINT(0) }
#[inline]
fn eint_irq_to_bit(irq: u32) -> u32 { 1u32 << eint_offset(irq) }

unsafe fn s3c_irq_eint_mask(data: *mut irq_data) { let mask = __raw_readl(S3C64XX_EINT0MASK) | (*data).chip_data as u32; __raw_writel(mask, S3C64XX_EINT0MASK); }
unsafe fn s3c_irq_eint_unmask(data: *mut irq_data) { let mask = __raw_readl(S3C64XX_EINT0MASK) & !((*data).chip_data as u32); __raw_writel(mask, S3C64XX_EINT0MASK); }
unsafe fn s3c_irq_eint_ack(data: *mut irq_data) { __raw_writel((*data).chip_data as u32, S3C64XX_EINT0PEND); }
unsafe fn s3c_irq_eint_maskack(data: *mut irq_data) { s3c_irq_eint_mask(data); s3c_irq_eint_ack(data); }

unsafe fn s3c_irq_eint_set_type(data: *mut irq_data, irq_type: u32) -> i32 {
    let offs = eint_offset((*data).irq); if offs > 27 { return -EINVAL; }
    let reg = if offs <= 15 { S3C64XX_EINT0CON0 } else { S3C64XX_EINT0CON1 };
    let newvalue = match irq_type { IRQ_TYPE_NONE => { printk(KERN_WARNING, b"No edge setting!\n\0".as_ptr()); 0 }, IRQ_TYPE_EDGE_RISING => S3C2410_EXTINT_RISEEDGE, IRQ_TYPE_EDGE_FALLING => S3C2410_EXTINT_FALLEDGE, IRQ_TYPE_EDGE_BOTH => S3C2410_EXTINT_BOTHEDGE, IRQ_TYPE_LEVEL_LOW => S3C2410_EXTINT_LOWLEV, IRQ_TYPE_LEVEL_HIGH => S3C2410_EXTINT_HILEV, _ => { printk(KERN_ERR, b"No such irq type %d\0".as_ptr(), irq_type); return -1; } };
    let shift = if offs <= 15 { (offs / 2) * 4 } else { ((offs - 16) / 2) * 4 };
    let mask = 0x7 << shift; let mut ctrl = __raw_readl(reg); ctrl &= !mask; ctrl |= newvalue << shift; __raw_writel(ctrl, reg);
    let (pin, pin_val) = if offs < 16 { (S3C64XX_GPN(offs), S3C_GPIO_SFN(2)) } else if offs < 23 { (S3C64XX_GPL(offs + 8 - 16), S3C_GPIO_SFN(3)) } else { (S3C64XX_GPM(offs - 23), S3C_GPIO_SFN(3)) };
    s3c_gpio_cfgpin(pin, pin_val); 0
}

unsafe fn s3c_irq_demux_eint(start: u32, end: u32) {
    let mut status = __raw_readl(S3C64XX_EINT0PEND) & !__raw_readl(S3C64XX_EINT0MASK); status >>= start; status &= (1 << (end - start + 1)) - 1;
    let mut irq = IRQ_EINT(start); while irq <= IRQ_EINT(end) { if status & 1 != 0 { generic_handle_irq(irq); } status >>= 1; irq += 1; }
}
unsafe fn s3c_irq_demux_eint0_3(_: *mut irq_desc) { s3c_irq_demux_eint(0, 3); }
unsafe fn s3c_irq_demux_eint4_11(_: *mut irq_desc) { s3c_irq_demux_eint(4, 11); }
unsafe fn s3c_irq_demux_eint12_19(_: *mut irq_desc) { s3c_irq_demux_eint(12, 19); }
unsafe fn s3c_irq_demux_eint20_27(_: *mut irq_desc) { s3c_irq_demux_eint(20, 27); }

unsafe fn s3c64xx_init_irq_eint() -> i32 {
    if of_have_populated_dt() || !soc_is_s3c64xx() { return -ENODEV; }
    let mut irq = IRQ_EINT(0); while irq <= IRQ_EINT(27) { irq_set_chip_and_handler(irq, &raw const s3c_irq_eint, handle_level_irq); irq_set_chip_data(irq, eint_irq_to_bit(irq) as *mut _); irq_clear_status_flags(irq, IRQ_NOREQUEST); irq += 1; }
    irq_set_chained_handler(IRQ_EINT0_3, s3c_irq_demux_eint0_3); irq_set_chained_handler(IRQ_EINT4_11, s3c_irq_demux_eint4_11); irq_set_chained_handler(IRQ_EINT12_19, s3c_irq_demux_eint12_19); irq_set_chained_handler(IRQ_EINT20_27, s3c_irq_demux_eint20_27); 0
}

// The platform is deprecated and scheduled for removal when CONFIG_COMPILE_TEST is disabled.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
