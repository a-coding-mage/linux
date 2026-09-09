// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-rpc/riscpc.c
 *
 *  Copyright (C) 1998-2001 Russell King
 *
 *  Architecture specific fixups.
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

extern "C" {
    fn rpc_init_irq();
    fn iotable_init(desc: *mut map_desc, count: usize);
    fn writeb(value: u8, address: usize);
    fn i2c_register_board_info(busnum: i32, info: *mut i2c_board_info, count: usize) -> i32;
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn iomd_writeb(value: u8, reg: usize);
    fn soft_restart(addr: usize) -> !;
    fn ioc_timer_init();
}

extern "C" {
    static mut elf_hwcap: u32;
}

pub static mut vram_size: u32 = 0;
pub static mut memc_ctrl_reg: u32 = 0;
pub static mut number_mfm_drives: u32 = 0;

#[repr(C)]
pub struct tag { pub u: tag_union }
#[repr(C)]
pub union tag_union { pub acorn: tag_acorn }
#[repr(C)]
pub struct tag_acorn {
    pub memc_control_reg: u32,
    pub adfsdrives: u32,
    pub vram_pages: u32,
}

unsafe extern "C" fn parse_tag_acorn(tag: *const tag) -> i32 {
    memc_ctrl_reg = (*tag).u.acorn.memc_control_reg;
    number_mfm_drives = (*tag).u.acorn.adfsdrives;

    match (*tag).u.acorn.vram_pages {
        512 => {
            vram_size = vram_size.wrapping_add(PAGE_SIZE.wrapping_mul(256));
            // fallthrough
            vram_size = vram_size.wrapping_add(PAGE_SIZE.wrapping_mul(256));
        }
        256 => {
            vram_size = vram_size.wrapping_add(PAGE_SIZE.wrapping_mul(256));
        }
        _ => {}
    }
    0
}

// __tagtable(ATAG_ACORN, parse_tag_acorn);

static mut rpc_io_desc: [map_desc; 3] = [
    map_desc { virtual_: SCREEN_BASE, pfn: __phys_to_pfn(SCREEN_START), length: 2 * 1048576, type_: MT_DEVICE },
    map_desc { virtual_: IO_BASE as u32, pfn: __phys_to_pfn(IO_START), length: IO_SIZE, type_: MT_DEVICE },
    map_desc { virtual_: EASI_BASE as usize, pfn: __phys_to_pfn(EASI_START), length: EASI_SIZE, type_: MT_DEVICE },
];

unsafe extern "C" fn rpc_map_io() {
    iotable_init(rpc_io_desc.as_mut_ptr(), rpc_io_desc.len());

    // Turn off floppy.
    writeb(0xc, PCIO_BASE + (0x3f2 << 2));

    // RiscPC can't handle half-word loads and stores.
    elf_hwcap &= !HWCAP_HALF;
}

static mut acornfb_resources: [resource; 2] = [
    DEFINE_RES_MEM!(0x03400000, 0x00200000),
    DEFINE_RES_IRQ!(IRQ_VSYNCPULSE),
];

static mut acornfb_device: platform_device = platform_device {
    name: "acornfb", id: -1,
    dev: device { coherent_dma_mask: 0xffffffff, ..device::default() },
    num_resources: acornfb_resources.len(), resource: acornfb_resources.as_mut_ptr(),
};

static mut iomd_resources: [resource; 1] = [DEFINE_RES_MEM!(0x03200000, 0x10000)];
static mut iomd_device: platform_device = platform_device {
    name: "iomd", id: -1, num_resources: iomd_resources.len(), resource: iomd_resources.as_mut_ptr(), ..platform_device::default()
};

static mut iomd_kart_resources: [resource; 2] = [
    DEFINE_RES_IRQ!(IRQ_KEYBOARDRX), DEFINE_RES_IRQ!(IRQ_KEYBOARDTX),
];
static mut kbd_device: platform_device = platform_device {
    name: "kart", id: -1,
    dev: device { parent: &mut iomd_device.dev, ..device::default() },
    num_resources: iomd_kart_resources.len(), resource: iomd_kart_resources.as_mut_ptr(),
};

static mut serial_platform_data: [plat_serial8250_port; 2] = [
    plat_serial8250_port { mapbase: 0x03010fe0, irq: IRQ_SERIALPORT, uartclk: 1843200, regshift: 2, iotype: UPIO_MEM, flags: UPF_BOOT_AUTOCONF | UPF_IOREMAP | UPF_SKIP_TEST, ..plat_serial8250_port::default() },
    plat_serial8250_port::default(),
];
static mut serial_device: platform_device = platform_device {
    name: "serial8250", id: PLAT8250_DEV_PLATFORM,
    dev: device { platform_data: serial_platform_data.as_mut_ptr() as *mut _, ..device::default() }, ..platform_device::default()
};

static mut pata_platform_data: pata_platform_info = pata_platform_info { ioport_shift: 2 };
static mut pata_resources: [resource; 3] = [
    DEFINE_RES_MEM!(0x030107c0, 0x20), DEFINE_RES_MEM!(0x03010fd8, 0x04), DEFINE_RES_IRQ!(IRQ_HARDDISK),
];
static mut pata_device: platform_device = platform_device {
    name: "pata_platform", id: -1, num_resources: pata_resources.len(), resource: pata_resources.as_mut_ptr(),
    dev: device { platform_data: &mut pata_platform_data as *mut _, coherent_dma_mask: !0, ..device::default() },
};

static mut devs: [*mut platform_device; 5] = [
    &mut iomd_device, &mut kbd_device, &mut serial_device, &mut acornfb_device, &mut pata_device,
];
static mut i2c_rtc: i2c_board_info = I2C_BOARD_INFO!("pcf8583", 0x50);

unsafe extern "C" fn rpc_init() -> i32 {
    i2c_register_board_info(0, &mut i2c_rtc, 1);
    platform_add_devices(devs.as_mut_ptr(), devs.len())
}

// arch_initcall(rpc_init);

unsafe extern "C" fn rpc_restart(_mode: reboot_mode, _cmd: *const i8) {
    iomd_writeb(0, IOMD_ROMCR0);
    // Jump into the ROM.
    soft_restart(0);
}

// MACHINE_START(RISCPC, "Acorn-RiscPC")
// Maintainer: Russell King
// .atag_offset = 0x100, .reserve_lp0 = 1, .reserve_lp1 = 1,
// .map_io = rpc_map_io, .init_irq = rpc_init_irq,
// .init_time = ioc_timer_init, .restart = rpc_restart
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
