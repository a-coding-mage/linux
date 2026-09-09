// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * BRIEF MODULE DESCRIPTION
 *	MyCable XXS1500 board support
 *
 * Copyright 2003, 2008 MontaVista Software Inc.
 * Author: MontaVista Software, Inc. <source@mvista.com>
 */

// Linux and platform headers are supplied by the surrounding translation unit.

extern "C" {
    fn alchemy_uart_putchar(addr: usize, c: core::ffi::c_char);
    fn alchemy_gpio1_input_enable();
    fn alchemy_gpio2_enable();
    fn alchemy_rdsys(reg: u32) -> u32;
    fn alchemy_wrsys(value: u32, reg: u32);
    fn alchemy_uart_enable(addr: usize);
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn wmb();
    fn irq_set_irq_type(irq: u32, irq_type: u32) -> i32;
    fn platform_add_devices(devs: *mut *mut platform_device, count: usize) -> i32;
}

#[repr(C)]
pub struct resource {
    pub name: *const core::ffi::c_char,
    pub flags: u64,
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
}

extern "C" {
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>;
}

pub unsafe extern "C" fn get_system_type() -> *const core::ffi::c_char {
    b"XXS1500\0".as_ptr() as *const core::ffi::c_char
}

pub unsafe extern "C" fn prom_putchar(c: core::ffi::c_char) {
    alchemy_uart_putchar(AU1000_UART0_PHYS_ADDR, c);
}

unsafe extern "C" fn xxs1500_reset(_c: *mut core::ffi::c_char) {
    core::arch::asm!("jr {0}", in(reg) 0xbfc00000usize);
}

unsafe extern "C" fn xxs1500_power_off() {
    loop {
        core::arch::asm!(".set mips32", "wait", ".set mips0");
    }
}

pub unsafe extern "C" fn board_setup() {
    let mut pin_func: u32;

    pm_power_off = Some(xxs1500_power_off);
    _machine_halt = Some(xxs1500_power_off);
    _machine_restart = Some(xxs1500_reset);

    alchemy_gpio1_input_enable();
    alchemy_gpio2_enable();

    /* Set multiple use pins (UART3/GPIO) to UART (it's used as UART too) */
    pin_func = alchemy_rdsys(AU1000_SYS_PINFUNC) & !SYS_PF_UR3;
    pin_func |= SYS_PF_UR3;
    alchemy_wrsys(pin_func, AU1000_SYS_PINFUNC);

    /* Enable UART */
    alchemy_uart_enable(AU1000_UART3_PHYS_ADDR);
    /* Enable DTR (MCR bit 0) = USB power up */
    __raw_writel(1, KSEG1ADDR(AU1000_UART3_PHYS_ADDR + 0x18) as *mut core::ffi::c_void);
    wmb();
}

static mut xxs1500_pcmcia_res: [resource; 3] = [
    resource {
        name: b"pcmcia-io\0".as_ptr() as *const core::ffi::c_char,
        flags: IORESOURCE_MEM,
        start: AU1000_PCMCIA_IO_PHYS_ADDR,
        end: AU1000_PCMCIA_IO_PHYS_ADDR + 0x000400000 - 1,
    },
    resource {
        name: b"pcmcia-attr\0".as_ptr() as *const core::ffi::c_char,
        flags: IORESOURCE_MEM,
        start: AU1000_PCMCIA_ATTR_PHYS_ADDR,
        end: AU1000_PCMCIA_ATTR_PHYS_ADDR + 0x000400000 - 1,
    },
    resource {
        name: b"pcmcia-mem\0".as_ptr() as *const core::ffi::c_char,
        flags: IORESOURCE_MEM,
        start: AU1000_PCMCIA_MEM_PHYS_ADDR,
        end: AU1000_PCMCIA_MEM_PHYS_ADDR + 0x000400000 - 1,
    },
];

static mut xxs1500_pcmcia_dev: platform_device = platform_device {
    name: b"xxs1500_pcmcia\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
    num_resources: 3,
    resource: core::ptr::addr_of_mut!(xxs1500_pcmcia_res) as *mut resource,
};

static mut xxs1500_devs: [*mut platform_device; 1] = [
    core::ptr::addr_of_mut!(xxs1500_pcmcia_dev),
];

unsafe extern "C" fn xxs1500_dev_init() -> i32 {
    irq_set_irq_type(AU1500_GPIO204_INT, IRQ_TYPE_LEVEL_HIGH);
    irq_set_irq_type(AU1500_GPIO201_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO202_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO203_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO205_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO207_INT, IRQ_TYPE_LEVEL_LOW);

    irq_set_irq_type(AU1500_GPIO0_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO1_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO2_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO3_INT, IRQ_TYPE_LEVEL_LOW);
    irq_set_irq_type(AU1500_GPIO4_INT, IRQ_TYPE_LEVEL_LOW); /* CF irq */
    irq_set_irq_type(AU1500_GPIO5_INT, IRQ_TYPE_LEVEL_LOW);

    platform_add_devices(xxs1500_devs.as_mut_ptr(), xxs1500_devs.len())
}

// device_initcall(xxs1500_dev_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
