/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct Bcm63xxPcmciaPlatformData {
    pub ready_gpio: u32,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut Bcm63xxPcmciaPlatformData,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut Resource,
    pub dev: Device,
}

extern "C" {
    fn bcm63xx_set_cs_status(cs: u32, enable: u32) -> i32;
    fn bcm63xx_set_cs_base(cs: u32, base: u32, size: u32) -> i32;
    fn bcm63xx_get_cpu_id() -> u32;
    fn bcm63xx_regset_address(regset: u32) -> usize;
    fn bcm63xx_get_irq_number(irq: u32) -> usize;
    fn platform_device_register(device: *mut PlatformDevice) -> i32;
    fn pr_err(message: *const u8, ...);
    fn BCMCPU_IS_6348() -> bool;
    fn BCMCPU_IS_6358() -> bool;
}

// Constants and macros below are provided by the corresponding BCM63xx headers.
extern "C" {
    static BCM_PCMCIA_COMMON_BASE_PA: usize;
    static BCM_PCMCIA_COMMON_END_PA: usize;
    static BCM_PCMCIA_ATTR_BASE_PA: usize;
    static BCM_PCMCIA_ATTR_END_PA: usize;
    static BCM_PCMCIA_IO_BASE_PA: usize;
    static BCM_PCMCIA_IO_END_PA: usize;
    static BCM_PCMCIA_COMMON_SIZE: u32;
    static BCM_PCMCIA_ATTR_SIZE: u32;
    static BCM_PCMCIA_IO_SIZE: u32;
    static MPI_CS_PCMCIA_COMMON: u32;
    static MPI_CS_PCMCIA_ATTR: u32;
    static MPI_CS_PCMCIA_IO: u32;
    static BCM6348_CPU_ID: u32;
    static BCM6358_CPU_ID: u32;
    static RSET_PCMCIA: u32;
    static RSET_PCMCIA_SIZE: usize;
    static IRQ_PCMCIA: u32;
    static IORESOURCE_MEM: usize;
    static IORESOURCE_IRQ: usize;
    static IORESOURCE_IO: usize;
    static ENODEV: i32;
}

static mut pcmcia_resources: [Resource; 6] = [
    Resource { start: 0, end: 0, flags: unsafe { IORESOURCE_MEM } },
    Resource { start: unsafe { BCM_PCMCIA_COMMON_BASE_PA }, end: unsafe { BCM_PCMCIA_COMMON_END_PA }, flags: unsafe { IORESOURCE_MEM } },
    Resource { start: unsafe { BCM_PCMCIA_ATTR_BASE_PA }, end: unsafe { BCM_PCMCIA_ATTR_END_PA }, flags: unsafe { IORESOURCE_MEM } },
    Resource { start: unsafe { BCM_PCMCIA_IO_BASE_PA }, end: unsafe { BCM_PCMCIA_IO_END_PA }, flags: unsafe { IORESOURCE_MEM } },
    Resource { start: 0, end: 0, flags: unsafe { IORESOURCE_IRQ } },
    Resource { start: unsafe { BCM_PCMCIA_IO_BASE_PA }, end: unsafe { BCM_PCMCIA_IO_END_PA }, flags: unsafe { IORESOURCE_IO } },
];

static mut pd: Bcm63xxPcmciaPlatformData = Bcm63xxPcmciaPlatformData { ready_gpio: 0 };

static mut bcm63xx_pcmcia_device: PlatformDevice = PlatformDevice {
    name: b"bcm63xx_pcmcia\0".as_ptr(),
    id: 0,
    num_resources: 6,
    resource: core::ptr::null_mut(),
    dev: Device { platform_data: core::ptr::null_mut() },
};

unsafe fn config_pcmcia_cs(cs: u32, base: u32, size: u32) -> i32 {
    let mut ret = bcm63xx_set_cs_status(cs, 0);
    if ret == 0 {
        ret = bcm63xx_set_cs_base(cs, base, size);
    }
    if ret == 0 {
        ret = bcm63xx_set_cs_status(cs, 1);
    }
    ret
}

#[repr(C)]
struct PcmciaCs {
    cs: u32,
    base: u32,
    size: u32,
}

static pcmcia_cs: [PcmciaCs; 3] = [
    PcmciaCs { cs: unsafe { MPI_CS_PCMCIA_COMMON }, base: unsafe { BCM_PCMCIA_COMMON_BASE_PA as u32 }, size: unsafe { BCM_PCMCIA_COMMON_SIZE } },
    PcmciaCs { cs: unsafe { MPI_CS_PCMCIA_ATTR }, base: unsafe { BCM_PCMCIA_ATTR_BASE_PA as u32 }, size: unsafe { BCM_PCMCIA_ATTR_SIZE } },
    PcmciaCs { cs: unsafe { MPI_CS_PCMCIA_IO }, base: unsafe { BCM_PCMCIA_IO_BASE_PA as u32 }, size: unsafe { BCM_PCMCIA_IO_SIZE } },
];

pub unsafe fn bcm63xx_pcmcia_register() -> i32 {
    let mut ret: i32;
    let mut i: usize;

    if !BCMCPU_IS_6348() && !BCMCPU_IS_6358() {
        return 0;
    }

    match bcm63xx_get_cpu_id() {
        id if id == BCM6348_CPU_ID => pd.ready_gpio = 22,
        id if id == BCM6358_CPU_ID => pd.ready_gpio = 18,
        _ => return -ENODEV,
    }

    pcmcia_resources[0].start = bcm63xx_regset_address(RSET_PCMCIA);
    pcmcia_resources[0].end = pcmcia_resources[0].start + RSET_PCMCIA_SIZE - 1;
    pcmcia_resources[4].start = bcm63xx_get_irq_number(IRQ_PCMCIA);

    i = 0;
    while i < 3 {
        ret = config_pcmcia_cs(pcmcia_cs[i].cs, pcmcia_cs[i].base, pcmcia_cs[i].size);
        if ret != 0 {
            pr_err(b"unable to set pcmcia chip select\n\0".as_ptr());
            return ret;
        }
        i += 1;
    }

    bcm63xx_pcmcia_device.resource = pcmcia_resources.as_mut_ptr();
    bcm63xx_pcmcia_device.dev.platform_data = &mut pd;
    platform_device_register(&mut bcm63xx_pcmcia_device)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
