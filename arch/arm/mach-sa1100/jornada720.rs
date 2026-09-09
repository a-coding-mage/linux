// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-sa1100/jornada720.c
 *
 * HP Jornada720 init code
 *
 * Copyright (C) 2007 Kristoffer Ericson <Kristoffer.Ericson@gmail.com>
 * Copyright (C) 2006 Filip Zyzniewski <filip.zyzniewski@tefnet.pl>
 *  Copyright (C) 2005 Michael Gernoth <michael@gernoth.net>
 */

// Linux kernel dependencies supplied by other translation units.

const TUCR_VAL: u32 = 0x20000400;
const SA1111REGSTART: usize = 0x40000000;
const SA1111REGLEN: usize = 0x00002000;
const EPSONREGSTART: usize = 0x48000000;
const EPSONREGLEN: usize = 0x00100000;
const EPSONFBSTART: usize = 0x48200000;
const EPSONFBLEN: usize = 512 * 1024;

static mut s1d13xxxfb_initregs: [s1d13xxxfb_regval; 104] = [
    s1d13xxxfb_regval { reg: 0x0001, val: 0x00 }, s1d13xxxfb_regval { reg: 0x01FC, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0004, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0005, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0008, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0009, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0010, val: 0x01 }, s1d13xxxfb_regval { reg: 0x0014, val: 0x11 },
    s1d13xxxfb_regval { reg: 0x0018, val: 0x01 }, s1d13xxxfb_regval { reg: 0x001C, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x001E, val: 0x01 }, s1d13xxxfb_regval { reg: 0x0020, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0021, val: 0x45 }, s1d13xxxfb_regval { reg: 0x002A, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x002B, val: 0x03 }, s1d13xxxfb_regval { reg: 0x0030, val: 0x1c },
    s1d13xxxfb_regval { reg: 0x0031, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0032, val: 0x4F },
    s1d13xxxfb_regval { reg: 0x0034, val: 0x07 }, s1d13xxxfb_regval { reg: 0x0035, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x0036, val: 0x0B }, s1d13xxxfb_regval { reg: 0x0038, val: 0xEF },
    s1d13xxxfb_regval { reg: 0x0039, val: 0x00 }, s1d13xxxfb_regval { reg: 0x003A, val: 0x13 },
    s1d13xxxfb_regval { reg: 0x003B, val: 0x0B }, s1d13xxxfb_regval { reg: 0x003C, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x0040, val: 0x05 }, s1d13xxxfb_regval { reg: 0x0041, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0042, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0043, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0044, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0046, val: 0x80 },
    s1d13xxxfb_regval { reg: 0x0047, val: 0x02 }, s1d13xxxfb_regval { reg: 0x0048, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x004A, val: 0x00 }, s1d13xxxfb_regval { reg: 0x004B, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0050, val: 0x4F }, s1d13xxxfb_regval { reg: 0x0052, val: 0x13 },
    s1d13xxxfb_regval { reg: 0x0053, val: 0x01 }, s1d13xxxfb_regval { reg: 0x0054, val: 0x0B },
    s1d13xxxfb_regval { reg: 0x0056, val: 0xDF }, s1d13xxxfb_regval { reg: 0x0057, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x0058, val: 0x2B }, s1d13xxxfb_regval { reg: 0x0059, val: 0x09 },
    s1d13xxxfb_regval { reg: 0x005A, val: 0x01 }, s1d13xxxfb_regval { reg: 0x005B, val: 0x10 },
    s1d13xxxfb_regval { reg: 0x0060, val: 0x03 }, s1d13xxxfb_regval { reg: 0x0062, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0063, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0064, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0066, val: 0x40 }, s1d13xxxfb_regval { reg: 0x0067, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x0068, val: 0x00 }, s1d13xxxfb_regval { reg: 0x006A, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x006B, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0070, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0071, val: 0x01 }, s1d13xxxfb_regval { reg: 0x0072, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0073, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0074, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0075, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0076, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0077, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0078, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x007A, val: 0x1F }, s1d13xxxfb_regval { reg: 0x007B, val: 0x3F },
    s1d13xxxfb_regval { reg: 0x007C, val: 0x1F }, s1d13xxxfb_regval { reg: 0x007E, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0080, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0081, val: 0x01 },
    s1d13xxxfb_regval { reg: 0x0082, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0083, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0084, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0085, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0086, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0087, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0088, val: 0x00 }, s1d13xxxfb_regval { reg: 0x008A, val: 0x1F },
    s1d13xxxfb_regval { reg: 0x008B, val: 0x3F }, s1d13xxxfb_regval { reg: 0x008C, val: 0x1F },
    s1d13xxxfb_regval { reg: 0x008E, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0100, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0101, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0102, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0103, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0104, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0105, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0106, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0108, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0109, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x010A, val: 0x00 }, s1d13xxxfb_regval { reg: 0x010C, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x010D, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0110, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0111, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0112, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0113, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0114, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0115, val: 0x00 }, s1d13xxxfb_regval { reg: 0x0118, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x0119, val: 0x00 }, s1d13xxxfb_regval { reg: 0x01E0, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x01E2, val: 0x00 }, s1d13xxxfb_regval { reg: 0x01E4, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x01F0, val: 0x10 }, s1d13xxxfb_regval { reg: 0x01F1, val: 0x00 },
    s1d13xxxfb_regval { reg: 0x01F4, val: 0x00 }, s1d13xxxfb_regval { reg: 0x01FC, val: 0x01 },
];

static mut s1d13xxxfb_data: s1d13xxxfb_pdata = s1d13xxxfb_pdata {
    initregs: unsafe { s1d13xxxfb_initregs.as_mut_ptr() },
    initregssize: 104,
    platform_init_video: None,
};

static mut s1d13xxxfb_resources: [resource; 2] = [
    DEFINE_RES_MEM(EPSONFBSTART, EPSONFBLEN),
    DEFINE_RES_MEM(EPSONREGSTART, EPSONREGLEN),
];

static mut s1d13xxxfb_device: platform_device = platform_device {
    name: S1D_DEVICENAME, id: 0,
    dev: device { platform_data: unsafe { &mut s1d13xxxfb_data as *mut _ }, ..device::default() },
    num_resources: 2, resource: unsafe { s1d13xxxfb_resources.as_mut_ptr() },
};

static mut jornada_pcmcia_gpiod_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "1800", table: [
        GPIO_LOOKUP("sa1111", 0, "s0-power", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP("sa1111", 1, "s1-power", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP("sa1111", 2, "s0-3v", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP("sa1111", 3, "s1-3v", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_SENTINEL,
    ],
};

static mut sa1111_resources: [resource; 2] = [DEFINE_RES_MEM(SA1111REGSTART, SA1111REGLEN), DEFINE_RES_IRQ(IRQ_GPIO1)];
static mut sa1111_info: sa1111_platform_data = sa1111_platform_data { disable_devs: SA1111_DEVID_PS2_MSE };
static mut sa1111_dmamask: u64 = 0xffffffff;
static mut sa1111_device: platform_device = platform_device {
    name: "sa1111", id: 0,
    dev: device { dma_mask: unsafe { &mut sa1111_dmamask }, coherent_dma_mask: 0xffffffff, platform_data: unsafe { &mut sa1111_info as *mut _ }, ..device::default() },
    num_resources: 2, resource: unsafe { sa1111_resources.as_mut_ptr() },
};
static mut jornada_ssp_device: platform_device = platform_device { name: "jornada_ssp", id: -1, ..platform_device::default() };
static mut jornada_kbd_resources: [resource; 1] = [DEFINE_RES_IRQ(IRQ_GPIO0)];
static mut jornada_kbd_device: platform_device = platform_device { name: "jornada720_kbd", id: -1, num_resources: 1, resource: unsafe { jornada_kbd_resources.as_mut_ptr() }, ..platform_device::default() };
static mut jornada_ts_gpiod_table: gpiod_lookup_table = gpiod_lookup_table { dev_id: "jornada_ts", table: [GPIO_LOOKUP("gpio", 9, "penup", GPIO_ACTIVE_HIGH)] };
static mut jornada_ts_device: platform_device = platform_device { name: "jornada_ts", id: -1, ..platform_device::default() };
static mut devices: [*mut platform_device; 5] = unsafe { [&mut sa1111_device, &mut jornada_ssp_device, &mut s1d13xxxfb_device, &mut jornada_kbd_device, &mut jornada_ts_device] };

unsafe fn jornada720_init() -> i32 {
    let mut ret: i32 = -ENODEV;
    if machine_is_jornada720() {
        GPDR |= GPIO_GPIO20;
        TUCR = TUCR_VAL;
        GPSR = GPIO_GPIO20;
        udelay(1);
        GPCR = GPIO_GPIO20;
        udelay(1);
        GPSR = GPIO_GPIO20;
        udelay(20);
        gpiod_add_lookup_table(&mut jornada_ts_gpiod_table);
        gpiod_add_lookup_table(&mut jornada_pcmcia_gpiod_table);
        ret = platform_add_devices(devices.as_mut_ptr(), devices.len());
    }
    ret
}

arch_initcall!(jornada720_init);

static mut jornada720_io_desc: [map_desc; 2] = [
    map_desc { virtual_: 0xf0000000, pfn: __phys_to_pfn(EPSONREGSTART), length: EPSONREGLEN, type_: MT_DEVICE },
    map_desc { virtual_: 0xf1000000, pfn: __phys_to_pfn(EPSONFBSTART), length: EPSONFBLEN, type_: MT_DEVICE },
];

unsafe fn jornada720_map_io() {
    sa1100_map_io();
    iotable_init(jornada720_io_desc.as_mut_ptr(), 2);
    sa1100_register_uart(0, 3);
    sa1100_register_uart(1, 1);
}

static mut jornada720_partitions: [mtd_partition; 7] = [
    mtd_partition { name: "JORNADA720 boot firmware", size: 0x00040000, offset: 0, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "JORNADA720 kernel", size: 0x000c0000, offset: 0x00040000, ..mtd_partition::default() },
    mtd_partition { name: "JORNADA720 params", size: 0x00040000, offset: 0x00100000, ..mtd_partition::default() },
    mtd_partition { name: "JORNADA720 initrd", size: 0x00100000, offset: 0x00140000, ..mtd_partition::default() },
    mtd_partition { name: "JORNADA720 root cramfs", size: 0x00300000, offset: 0x00240000, ..mtd_partition::default() },
    mtd_partition { name: "JORNADA720 usr cramfs", size: 0x00800000, offset: 0x00540000, ..mtd_partition::default() },
    mtd_partition { name: "JORNADA720 usr local", size: 0, offset: 0x00d00000, ..mtd_partition::default() },
];

unsafe fn jornada720_set_vpp(vpp: i32) {
    if vpp != 0 { PPSR |= PPC_LDD7; } else { PPSR &= !PPC_LDD7; }
    PPDR |= PPC_LDD7;
}

static mut jornada720_flash_data: flash_platform_data = flash_platform_data {
    map_name: "cfi_probe", set_vpp: Some(jornada720_set_vpp), parts: unsafe { jornada720_partitions.as_mut_ptr() }, nr_parts: 7,
};
static mut jornada720_flash_resource: resource = DEFINE_RES_MEM(SA1100_CS0_PHYS, SZ_32M);

unsafe fn jornada720_mach_init() {
    sa11x0_register_mtd(&mut jornada720_flash_data, &mut jornada720_flash_resource, 1);
}

MACHINE_START!(JORNADA720, "HP Jornada 720", {
    atag_offset: 0x100,
    map_io: jornada720_map_io,
    nr_irqs: SA1100_NR_IRQS,
    init_irq: sa1100_init_irq,
    init_time: sa1100_timer_init,
    init_machine: jornada720_mach_init,
    init_late: sa11x0_init_late,
    dma_zone_size: SZ_1M,
    restart: sa11x0_restart,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
