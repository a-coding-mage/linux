/*
 * Platform device support for Au1x00 SoCs.
 *
 * Copyright 2004, Matt Porter <mporter@kernel.crashing.org>
 *
 * (C) Copyright Embedded Alley Solutions, Inc 2005
 * Author: Pantelis Antoniou <pantelis@embeddedalley.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// C headers and build-time configuration are supplied by the surrounding kernel bindings.

unsafe fn alchemy_8250_pm(port: *mut uart_port, state: c_uint, old_state: c_uint) {
    #[cfg(CONFIG_SERIAL_8250)]
    {
        match state {
            0 => {
                alchemy_uart_enable(CPHYSADDR((*port).membase));
                serial8250_do_pm(port, state, old_state);
            }
            3 => {
                serial8250_do_pm(port, state, old_state);
                alchemy_uart_disable(CPHYSADDR((*port).membase));
            }
            _ => serial8250_do_pm(port, state, old_state),
        }
    }
}

macro_rules! PORT {
    ($base:expr, $irq:expr) => { plat_serial8250_port {
        mapbase: $base, mapsize: 0x1000, irq: $irq, regshift: 2,
        flags: UPF_SKIP_TEST | UPF_IOREMAP | UPF_FIXED_TYPE,
        type_: PORT_16550A, pm: Some(alchemy_8250_pm), ..unsafe { core::mem::zeroed() }
    }};
}

static mut au1x00_uart_data: [[plat_serial8250_port; 4]; 6] = unsafe { core::mem::zeroed() };

static mut au1xx0_uart_device: platform_device = platform_device { name: "serial8250", id: PLAT8250_DEV_AU1X00, ..unsafe { core::mem::zeroed() } };

unsafe fn alchemy_setup_uarts(ctype: c_int) {
    let mut uartclk: c_long;
    let mut s = core::mem::size_of::<plat_serial8250_port>();
    let c = alchemy_get_uarts(ctype);
    let clk = clk_get(core::ptr::null_mut(), ALCHEMY_PERIPH_CLK);
    if IS_ERR(clk) { return; }
    if clk_prepare_enable(clk) != 0 { clk_put(clk); return; }
    uartclk = clk_get_rate(clk);
    clk_put(clk);
    let ports = kcalloc(s, (c + 1) as usize, GFP_KERNEL) as *mut plat_serial8250_port;
    if ports.is_null() { printk(KERN_INFO, "Alchemy: no memory for UART data\0"); return; }
    core::ptr::copy_nonoverlapping(au1x00_uart_data[ctype as usize].as_ptr(), ports, c as usize);
    au1xx0_uart_device.dev.platform_data = ports as *mut _;
    for s in 0..c {
        (*ports.add(s as usize)).uartclk = uartclk;
        if au_platform_setup(ports.add(s as usize)) < 0 {
            kfree(ports as *mut _); printk(KERN_INFO, "Alchemy: missing support for UARTs\0"); return;
        }
    }
    if platform_device_register(&mut au1xx0_uart_device) != 0 { printk(KERN_INFO, "Alchemy: failed to register UARTs\0"); }
}

static mut alchemy_all_dmamask: u64 = DMA_BIT_MASK(32);

unsafe fn alchemy_ehci_power_on(_pdev: *mut platform_device) -> c_int { alchemy_usb_control(ALCHEMY_USB_EHCI0, 1) }
unsafe fn alchemy_ehci_power_off(_pdev: *mut platform_device) { alchemy_usb_control(ALCHEMY_USB_EHCI0, 0); }
static mut alchemy_ehci_pdata: usb_ehci_pdata = usb_ehci_pdata { no_io_watchdog: 1, power_on: Some(alchemy_ehci_power_on), power_off: Some(alchemy_ehci_power_off), power_suspend: Some(alchemy_ehci_power_off) };

unsafe fn alchemy_ohci_power_on(pdev: *mut platform_device) -> c_int {
    let unit = if (*pdev).id == 1 { ALCHEMY_USB_OHCI1 } else { ALCHEMY_USB_OHCI0 }; alchemy_usb_control(unit, 1)
}
unsafe fn alchemy_ohci_power_off(pdev: *mut platform_device) {
    let unit = if (*pdev).id == 1 { ALCHEMY_USB_OHCI1 } else { ALCHEMY_USB_OHCI0 }; alchemy_usb_control(unit, 0);
}
static mut alchemy_ohci_pdata: usb_ohci_pdata = usb_ohci_pdata { power_on: Some(alchemy_ohci_power_on), power_off: Some(alchemy_ohci_power_off), power_suspend: Some(alchemy_ohci_power_off) };

static mut alchemy_ohci_data: [[c_ulong; 2]; 6] = unsafe { core::mem::zeroed() };
static mut alchemy_ehci_data: [[c_ulong; 2]; 6] = unsafe { core::mem::zeroed() };

unsafe fn _new_usbres(r: *mut *mut resource, d: *mut *mut platform_device) -> c_int {
    *r = kzalloc_objs::<resource>(2); if (*r).is_null() { return -ENOMEM; }
    *d = kzalloc_obj::<platform_device>(); if (*d).is_null() { kfree(*r as *mut _); return -ENOMEM; }
    (**d).dev.coherent_dma_mask = DMA_BIT_MASK(32); (**d).num_resources = 2; (**d).resource = *r; 0
}

unsafe fn alchemy_setup_usb(ctype: c_int) {
    let (mut res, mut pdev): (*mut resource, *mut platform_device) = (core::ptr::null_mut(), core::ptr::null_mut());
    if _new_usbres(&mut res, &mut pdev) != 0 { return; }
    (*res.add(0)).start = alchemy_ohci_data[ctype as usize][0]; (*res.add(0)).end = (*res).start + 0x100 - 1; (*res.add(0)).flags = IORESOURCE_MEM;
    (*res.add(1)).start = alchemy_ohci_data[ctype as usize][1]; (*res.add(1)).end = (*res.add(1)).start; (*res.add(1)).flags = IORESOURCE_IRQ;
    (*pdev).name = "ohci-platform"; (*pdev).id = 0; (*pdev).dev.dma_mask = &mut alchemy_all_dmamask; (*pdev).dev.platform_data = &mut alchemy_ohci_pdata as *mut _;
    if platform_device_register(pdev) != 0 { printk(KERN_INFO, "Alchemy USB: cannot add OHCI0\0"); }
    if ctype == ALCHEMY_CPU_AU1200 || ctype == ALCHEMY_CPU_AU1300 {
        if _new_usbres(&mut res, &mut pdev) != 0 { return; }
        (*res).start = alchemy_ehci_data[ctype as usize][0]; (*res).end = (*res).start + 0x100 - 1; (*res).flags = IORESOURCE_MEM;
        (*res.add(1)).start = alchemy_ehci_data[ctype as usize][1]; (*res.add(1)).end = (*res.add(1)).start; (*res.add(1)).flags = IORESOURCE_IRQ;
        (*pdev).name = "ehci-platform"; (*pdev).id = 0; (*pdev).dev.dma_mask = &mut alchemy_all_dmamask; (*pdev).dev.platform_data = &mut alchemy_ehci_pdata as *mut _;
        if platform_device_register(pdev) != 0 { printk(KERN_INFO, "Alchemy USB: cannot add EHCI0\0"); }
    }
    if ctype == ALCHEMY_CPU_AU1300 {
        if _new_usbres(&mut res, &mut pdev) != 0 { return; }
        (*res).start = AU1300_USB_OHCI1_PHYS_ADDR; (*res).end = (*res).start + 0x100 - 1; (*res).flags = IORESOURCE_MEM;
        (*res.add(1)).start = AU1300_USB_INT; (*res.add(1)).end = (*res.add(1)).start; (*res.add(1)).flags = IORESOURCE_IRQ;
        (*pdev).name = "ohci-platform"; (*pdev).id = 1; (*pdev).dev.dma_mask = &mut alchemy_all_dmamask; (*pdev).dev.platform_data = &mut alchemy_ohci_pdata as *mut _;
        if platform_device_register(pdev) != 0 { printk(KERN_INFO, "Alchemy USB: cannot add OHCI1\0"); }
    }
}

// Ethernet resource tables and platform data retain the C layout and constants.
const MAC_RES_COUNT: usize = 4;
static mut au1xxx_eth0_resources: [[resource; MAC_RES_COUNT]; 4] = unsafe { core::mem::zeroed() };
static mut au1xxx_eth1_resources: [[resource; MAC_RES_COUNT]; 4] = unsafe { core::mem::zeroed() };
static mut au1xxx_eth0_platform_data: au1000_eth_platform_data = unsafe { core::mem::zeroed() };
static mut au1xxx_eth1_platform_data: au1000_eth_platform_data = unsafe { core::mem::zeroed() };
static mut au1xxx_eth0_device: platform_device = unsafe { core::mem::zeroed() };
static mut au1xxx_eth1_device: platform_device = unsafe { core::mem::zeroed() };

unsafe fn au1xxx_override_eth_cfg(port: c_uint, eth_data: *const au1000_eth_platform_data) {
    if eth_data.is_null() || port > 1 { return; }
    if port == 0 { core::ptr::copy_nonoverlapping(eth_data, &mut au1xxx_eth0_platform_data, 1); }
    else { core::ptr::copy_nonoverlapping(eth_data, &mut au1xxx_eth1_platform_data, 1); }
}

unsafe fn alchemy_setup_macs(ctype: c_int) {
    if alchemy_get_macs(ctype) < 1 { return; }
    let macres = kmemdup_array(au1xxx_eth0_resources[ctype as usize].as_ptr(), MAC_RES_COUNT, core::mem::size_of::<resource>(), GFP_KERNEL) as *mut resource;
    if macres.is_null() { printk(KERN_INFO, "Alchemy: no memory for MAC0 resources\0"); return; }
    au1xxx_eth0_device.resource = macres;
    let mut ethaddr = [0u8; 6]; let i = prom_get_ethernet_addr(ethaddr.as_mut_ptr());
    if i == 0 && !is_valid_ether_addr(au1xxx_eth0_platform_data.mac.as_ptr()) { core::ptr::copy_nonoverlapping(ethaddr.as_ptr(), au1xxx_eth0_platform_data.mac.as_mut_ptr(), 6); }
    if platform_device_register(&mut au1xxx_eth0_device) != 0 { printk(KERN_INFO, "Alchemy: failed to register MAC0\0"); }
    if alchemy_get_macs(ctype) < 2 { return; }
    let macres = kmemdup_array(au1xxx_eth1_resources[ctype as usize].as_ptr(), MAC_RES_COUNT, core::mem::size_of::<resource>(), GFP_KERNEL) as *mut resource;
    if macres.is_null() { printk(KERN_INFO, "Alchemy: no memory for MAC1 resources\0"); return; }
    au1xxx_eth1_device.resource = macres; ethaddr[5] = ethaddr[5].wrapping_add(1);
    if i == 0 && !is_valid_ether_addr(au1xxx_eth1_platform_data.mac.as_ptr()) { core::ptr::copy_nonoverlapping(ethaddr.as_ptr(), au1xxx_eth1_platform_data.mac.as_mut_ptr(), 6); }
    if (alchemy_rdsys(AU1000_SYS_PINFUNC) & SYS_PF_NI2) == 0 && platform_device_register(&mut au1xxx_eth1_device) != 0 { printk(KERN_INFO, "Alchemy: failed to register MAC1\0"); }
}

unsafe fn au1xxx_platform_init() -> c_int {
    let ctype = alchemy_get_cputype(); alchemy_setup_uarts(ctype); alchemy_setup_macs(ctype); alchemy_setup_usb(ctype); 0
}

arch_initcall!(au1xxx_platform_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
