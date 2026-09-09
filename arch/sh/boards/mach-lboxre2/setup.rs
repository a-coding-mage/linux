// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/lbox/setup.c
 *
 * Copyright (C) 2007 Nobuhiro Iwamatsu
 *
 * NTT COMWARE L-BOX RE2 Support
 */

// Dependencies supplied by the surrounding kernel translation.

static mut CF_IDE_RESOURCES: [struct_resource; 3] = [
    struct_resource {
        start: 0x1f0,
        end: 0x1f0 + 8,
        flags: IORESOURCE_IO,
        ..struct_resource::default()
    },
    struct_resource {
        start: 0x1f0 + 0x206,
        end: 0x1f0 + 8 + 0x206 + 8,
        flags: IORESOURCE_IO,
        ..struct_resource::default()
    },
    struct_resource {
        start: IRQ_CF0,
        flags: IORESOURCE_IRQ,
        ..struct_resource::default()
    },
];

static mut CF_IDE_DEVICE: platform_device = platform_device {
    name: "pata_platform",
    id: -1,
    num_resources: 3,
    resource: unsafe { CF_IDE_RESOURCES.as_mut_ptr() },
    ..platform_device::default()
};

static mut LBOXRE2_DEVICES: [*mut platform_device; 1] = [
    unsafe { &raw mut CF_IDE_DEVICE },
];

unsafe extern "C" fn lboxre2_devices_setup() -> i32 {
    let mut cf0_io_base: u32; /* Boot CF base address */
    let prot: pgprot_t;
    let paddrbase: c_ulong;
    let psize: c_ulong;

    /* open I/O area window */
    paddrbase = virt_to_phys(PA_AREA5_IO as *mut c_void);
    psize = PAGE_SIZE;
    prot = PAGE_KERNEL_PCC(1, _PAGE_PCC_IO16);
    cf0_io_base = ioremap_prot(paddrbase, psize, prot) as u32;
    if cf0_io_base == 0 {
        printk!(KERN_ERR, "%s : can't open CF I/O window!\n", "lboxre2_devices_setup");
        return -ENOMEM;
    }

    CF_IDE_RESOURCES[0].start = CF_IDE_RESOURCES[0].start.wrapping_add(cf0_io_base as _);
    CF_IDE_RESOURCES[0].end = CF_IDE_RESOURCES[0].end.wrapping_add(cf0_io_base as _);
    CF_IDE_RESOURCES[1].start = CF_IDE_RESOURCES[1].start.wrapping_add(cf0_io_base as _);
    CF_IDE_RESOURCES[1].end = CF_IDE_RESOURCES[1].end.wrapping_add(cf0_io_base as _);

    platform_add_devices(
        LBOXRE2_DEVICES.as_mut_ptr(),
        LBOXRE2_DEVICES.len(),
    )
}

device_initcall!(lboxre2_devices_setup);

/*
 * The Machine Vector
 */
static mut MV_LBOXRE2: sh_machine_vector = sh_machine_vector {
    mv_name: "L-BOX RE2",
    mv_init_irq: Some(init_lboxre2_IRQ),
    ..sh_machine_vector::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
