/*
 * Setup pointers to hardware-dependent routines.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 1998, 2001, 07, 08 by Ralf Baechle
 * Copyright (C) 2001 MIPS Technologies, Inc.
 * Copyright (C) 2007 by Thomas Bogendoerfer
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn jazz_machine_restart(command: *mut core::ffi::c_char);
    fn add_wired_entry(entrylo0: u64, entrylo1: u64, entryhi: u64, pagemask: u64);
    fn set_io_port_base(base: u64);
    fn request_resource(parent: *mut resource, child: *mut resource) -> i32;
    fn add_preferred_console(name: *const core::ffi::c_char, idx: i32, options: *const core::ffi::c_char) -> i32;
    fn platform_device_register(device: *mut platform_device) -> i32;
}

extern "C" {
    static mut ioport_resource: resource;
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>;
}

#[repr(C)]
struct resource {
    start: u64,
    end: u64,
    name: *const core::ffi::c_char,
    flags: u64,
}

#[repr(C)]
struct plat_serial8250_port {
    mapbase: u64,
    membase: *mut core::ffi::c_void,
    irq: u32,
    uartclk: u32,
    iotype: u32,
    flags: u32,
}

#[repr(C)]
struct device {
    platform_data: *mut core::ffi::c_void,
    dma_mask: *mut u64,
    coherent_dma_mask: u64,
}

#[repr(C)]
struct platform_device {
    name: *const core::ffi::c_char,
    id: i32,
    num_resources: u32,
    resource: *mut resource,
    dev: device,
}

static mut jazz_io_resources: [resource; 4] = [
    resource { start: 0x00, end: 0x1f, name: b"dma1\0".as_ptr() as *const _, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    resource { start: 0x40, end: 0x5f, name: b"timer\0".as_ptr() as *const _, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    resource { start: 0x80, end: 0x8f, name: b"dma page reg\0".as_ptr() as *const _, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    resource { start: 0xc0, end: 0xdf, name: b"dma2\0".as_ptr() as *const _, flags: IORESOURCE_IO | IORESOURCE_BUSY },
];

pub unsafe extern "C" fn plat_mem_setup() {
    let mut i: usize;

    /* Map 0xe0000000 -> 0x0:800005C0, 0xe0010000 -> 0x1:30000580 */
    add_wired_entry(0x02000017, 0x03c00017, 0xe0000000, PM_64K);
    /* Map 0xe2000000 -> 0x0:900005C0, 0xe3010000 -> 0x0:910005C0 */
    add_wired_entry(0x02400017, 0x02440017, 0xe2000000, PM_16M);
    /* Map 0xe4000000 -> 0x0:600005C0, 0xe4100000 -> 400005C0 */
    add_wired_entry(0x01800017, 0x01000017, 0xe4000000, PM_4M);

    set_io_port_base(JAZZ_PORT_BASE);
    #[cfg(CONFIG_EISA)]
    {
        EISA_bus = 1;
    }

    /* request I/O space for devices used on all i[345]86 PCs */
    i = 0;
    while i < jazz_io_resources.len() {
        request_resource(&mut ioport_resource, &mut jazz_io_resources[i]);
        i += 1;
    }

    /* The RTC is outside the port address space */

    _machine_restart = Some(jazz_machine_restart);

    add_preferred_console(b"ttyS\0".as_ptr() as *const _, 0, b"9600\0".as_ptr() as *const _);
}

#[cfg(CONFIG_OLIVETTI_M700)]
const UART_CLK: u32 = 1843200;
#[cfg(not(CONFIG_OLIVETTI_M700))]
const UART_CLK: u32 = 8000000 / 16;

static mut jazz_serial_data: [plat_serial8250_port; 3] = [
    plat_serial8250_port { mapbase: JAZZ_SERIAL1_BASE, membase: JAZZ_SERIAL1_BASE as *mut _, irq: JAZZ_SERIAL1_IRQ, uartclk: UART_CLK, iotype: UPIO_MEM, flags: UPF_BOOT_AUTOCONF },
    plat_serial8250_port { mapbase: JAZZ_SERIAL2_BASE, membase: JAZZ_SERIAL2_BASE as *mut _, irq: JAZZ_SERIAL2_IRQ, uartclk: UART_CLK, iotype: UPIO_MEM, flags: UPF_BOOT_AUTOCONF },
    plat_serial8250_port { mapbase: 0, membase: core::ptr::null_mut(), irq: 0, uartclk: 0, iotype: 0, flags: 0 },
];

static mut jazz_serial8250_device: platform_device = platform_device {
    name: b"serial8250\0".as_ptr() as *const _, id: PLAT8250_DEV_PLATFORM, num_resources: 0, resource: core::ptr::null_mut(),
    dev: device { platform_data: core::ptr::addr_of_mut!(jazz_serial_data) as *mut _, dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 },
};

static mut jazz_esp_rsrc: [resource; 3] = [
    resource { start: JAZZ_SCSI_BASE, end: JAZZ_SCSI_BASE + 31, name: core::ptr::null(), flags: IORESOURCE_MEM },
    resource { start: JAZZ_SCSI_DMA, end: JAZZ_SCSI_DMA, name: core::ptr::null(), flags: IORESOURCE_MEM },
    resource { start: JAZZ_SCSI_IRQ, end: JAZZ_SCSI_IRQ, name: core::ptr::null(), flags: IORESOURCE_IRQ },
];
static mut jazz_esp_dma_mask: u64 = DMA_BIT_MASK(32);
static mut jazz_esp_pdev: platform_device = platform_device {
    name: b"jazz_esp\0".as_ptr() as *const _, id: 0, num_resources: 3, resource: core::ptr::addr_of_mut!(jazz_esp_rsrc),
    dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::addr_of_mut!(jazz_esp_dma_mask), coherent_dma_mask: DMA_BIT_MASK(32) },
};

static mut jazz_sonic_rsrc: [resource; 2] = [
    resource { start: JAZZ_ETHERNET_BASE, end: JAZZ_ETHERNET_BASE + 0xff, name: core::ptr::null(), flags: IORESOURCE_MEM },
    resource { start: JAZZ_ETHERNET_IRQ, end: JAZZ_ETHERNET_IRQ, name: core::ptr::null(), flags: IORESOURCE_IRQ },
];
static mut jazz_sonic_dma_mask: u64 = DMA_BIT_MASK(32);
static mut jazz_sonic_pdev: platform_device = platform_device {
    name: b"jazzsonic\0".as_ptr() as *const _, id: 0, num_resources: 2, resource: core::ptr::addr_of_mut!(jazz_sonic_rsrc),
    dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::addr_of_mut!(jazz_sonic_dma_mask), coherent_dma_mask: DMA_BIT_MASK(32) },
};

static mut jazz_cmos_rsrc: [resource; 2] = [
    resource { start: 0x70, end: 0x71, name: core::ptr::null(), flags: IORESOURCE_IO },
    resource { start: 8, end: 8, name: core::ptr::null(), flags: IORESOURCE_IRQ },
];
static mut jazz_cmos_pdev: platform_device = platform_device {
    name: b"rtc_cmos\0".as_ptr() as *const _, id: 0, num_resources: 2, resource: core::ptr::addr_of_mut!(jazz_cmos_rsrc),
    dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 },
};

static mut pcspeaker_pdev: platform_device = platform_device {
    name: b"pcspkr\0".as_ptr() as *const _, id: -1, num_resources: 0, resource: core::ptr::null_mut(),
    dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 },
};

unsafe extern "C" fn jazz_setup_devinit() -> i32 {
    platform_device_register(&mut jazz_serial8250_device);
    platform_device_register(&mut jazz_esp_pdev);
    platform_device_register(&mut jazz_sonic_pdev);
    platform_device_register(&mut jazz_cmos_pdev);
    platform_device_register(&mut pcspeaker_pdev);
    0
}

// device_initcall(jazz_setup_devinit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
