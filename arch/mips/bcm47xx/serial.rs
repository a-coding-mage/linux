/*
 * 8250 UART probe driver for the BCM47XX platforms
 * Author: Aurelien Jarno
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 Aurelien Jarno <aurelien@aurel32.net>
 */

// Linux headers supplied by the surrounding kernel translation.

static mut uart8250_data: [plat_serial8250_port; 5] = [plat_serial8250_port::default(); 5];

static mut uart8250_device: platform_device = platform_device {
    name: "serial8250",
    id: PLAT8250_DEV_PLATFORM,
    dev: device {
        platform_data: unsafe { uart8250_data.as_mut_ptr() as *mut _ },
    },
};

#[cfg(CONFIG_BCM47XX_SSB)]
unsafe fn uart8250_init_ssb() -> i32 {
    let mcore: *mut ssb_mipscore = &mut bcm47xx_bus.ssb.mipscore;

    core::ptr::write_bytes(
        uart8250_data.as_mut_ptr() as *mut u8,
        0,
        core::mem::size_of_val(&uart8250_data),
    );

    let mut i = 0usize;
    while i < (*mcore).nr_serial_ports as usize && i < uart8250_data.len() - 1 {
        let p: *mut plat_serial8250_port = &mut uart8250_data[i];
        let ssb_port: *mut ssb_serial_port = &mut (*mcore).serial_ports[i];

        (*p).mapbase = (*ssb_port).regs as u32;
        (*p).membase = (*ssb_port).regs as *mut core::ffi::c_void;
        (*p).irq = (*ssb_port).irq + 2;
        (*p).uartclk = (*ssb_port).baud_base;
        (*p).regshift = (*ssb_port).reg_shift;
        (*p).iotype = UPIO_MEM;
        (*p).flags = UPF_BOOT_AUTOCONF | UPF_SHARE_IRQ;
        i += 1;
    }
    platform_device_register(&mut uart8250_device)
}

#[cfg(CONFIG_BCM47XX_BCMA)]
unsafe fn uart8250_init_bcma() -> i32 {
    let cc: *mut bcma_drv_cc = &mut bcm47xx_bus.bcma.bus.drv_cc;

    core::ptr::write_bytes(
        uart8250_data.as_mut_ptr() as *mut u8,
        0,
        core::mem::size_of_val(&uart8250_data),
    );

    let mut i = 0usize;
    while i < (*cc).nr_serial_ports as usize && i < uart8250_data.len() - 1 {
        let p: *mut plat_serial8250_port = &mut uart8250_data[i];
        let bcma_port: *mut bcma_serial_port = &mut (*cc).serial_ports[i];

        (*p).mapbase = (*bcma_port).regs as u32;
        (*p).membase = (*bcma_port).regs as *mut core::ffi::c_void;
        (*p).irq = (*bcma_port).irq;
        (*p).uartclk = (*bcma_port).baud_base;
        (*p).regshift = (*bcma_port).reg_shift;
        (*p).iotype = UPIO_MEM;
        (*p).flags = UPF_BOOT_AUTOCONF | UPF_SHARE_IRQ;
        i += 1;
    }
    platform_device_register(&mut uart8250_device)
}

unsafe fn uart8250_init() -> i32 {
    match bcm47xx_bus_type {
        #[cfg(CONFIG_BCM47XX_SSB)]
        BCM47XX_BUS_TYPE_SSB => uart8250_init_ssb(),
        #[cfg(CONFIG_BCM47XX_BCMA)]
        BCM47XX_BUS_TYPE_BCMA => uart8250_init_bcma(),
        _ => -EINVAL,
    }
}

// Equivalent of device_initcall(uart8250_init).


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
