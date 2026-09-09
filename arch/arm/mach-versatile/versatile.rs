// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Versatile board support using the device tree
 *
 *  Copyright (C) 2010 Secret Lab Technologies Ltd.
 *  Copyright (C) 2009 Jeremy Kerr <jeremy.kerr@canonical.com>
 *  Copyright (C) 2004 ARM Limited
 *  Copyright (C) 2000 Deep Blue Solutions Ltd
 */

// Linux kernel dependencies supplied by other translation units.

const VERSATILE_SYS_PCICTL_OFFSET: usize = 0x44;
const VERSATILE_SYS_MCI_OFFSET: usize = 0x48;

const VERSATILE_MMCI0_BASE: usize = 0x10005000;
const VERSATILE_MMCI1_BASE: usize = 0x1000B000;
const VERSATILE_SCTL_BASE: usize = 0x101E0000;

const VERSATILE_REFCLK: u32 = 0;
const VERSATILE_TIMCLK: u32 = 1;

const VERSATILE_TIMER1_EN_SEL: u32 = 15;
const VERSATILE_TIMER2_EN_SEL: u32 = 17;
const VERSATILE_TIMER3_EN_SEL: u32 = 19;
const VERSATILE_TIMER4_EN_SEL: u32 = 21;

const fn io_address(x: usize) -> usize {
    ((x & 0x0fffffff) + ((x >> 4) & 0x0f000000) + 0xf0000000)
}

static mut versatile_sys_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn mmc_status(dev: *mut device) -> u32 {
    let adev = container_of_device(dev);
    let mask: u32;

    if (*adev).res.start == VERSATILE_MMCI0_BASE {
        mask = 1;
    } else {
        mask = 2;
    }

    readl((versatile_sys_base as *mut u8).add(VERSATILE_SYS_MCI_OFFSET) as *const u32) & mask
}

static mut mmc0_plat_data: mmci_platform_data = mmci_platform_data {
    ocr_mask: MMC_VDD_32_33 | MMC_VDD_33_34,
    status: Some(mmc_status),
};

static mut mmc1_plat_data: mmci_platform_data = mmci_platform_data {
    ocr_mask: MMC_VDD_32_33 | MMC_VDD_33_34,
    status: Some(mmc_status),
};

/*
 * Lookup table for attaching a specific name and platform_data pointer to
 * devices as they get created by of_platform_populate(). Ideally this table
 * would not exist, but the current clock implementation depends on some
 * devices having a specific name.
 */
static mut versatile_auxdata_lookup: [of_dev_auxdata; 3] = [
    of_dev_auxdata {
        compatible: "arm,primecell".as_ptr() as *const i8,
        physical_address: VERSATILE_MMCI0_BASE,
        name: "fpga:05".as_ptr() as *const i8,
        platform_data: unsafe { &mut mmc0_plat_data },
    },
    of_dev_auxdata {
        compatible: "arm,primecell".as_ptr() as *const i8,
        physical_address: VERSATILE_MMCI1_BASE,
        name: "fpga:0b".as_ptr() as *const i8,
        platform_data: unsafe { &mut mmc1_plat_data },
    },
    of_dev_auxdata::default(),
];

static mut versatile_io_desc: [map_desc; 1] = [map_desc {
    virtual_address: io_address(VERSATILE_SCTL_BASE),
    pfn: __phys_to_pfn(VERSATILE_SCTL_BASE),
    length: SZ_4K * 9,
    type_: MT_DEVICE,
}];

unsafe fn versatile_map_io() {
    debug_ll_io_init();
    iotable_init(versatile_io_desc.as_ptr(), versatile_io_desc.len());
}

unsafe fn versatile_init_early() {
    let val: u32;

    val = readl(io_address(VERSATILE_SCTL_BASE) as *const u32);
    writel(
        (VERSATILE_TIMCLK << VERSATILE_TIMER1_EN_SEL)
            | (VERSATILE_TIMCLK << VERSATILE_TIMER2_EN_SEL)
            | (VERSATILE_TIMCLK << VERSATILE_TIMER3_EN_SEL)
            | (VERSATILE_TIMCLK << VERSATILE_TIMER4_EN_SEL)
            | val,
        io_address(VERSATILE_SCTL_BASE) as *mut u32,
    );
}

unsafe fn versatile_dt_pci_init() {
    let mut val: u32;
    let np: *mut device_node;
    let mut newprop: *mut property;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "arm,versatile-pci".as_ptr() as *const i8);
    if np.is_null() {
        return;
    }

    val = readl((versatile_sys_base as *mut u8).add(VERSATILE_SYS_PCICTL_OFFSET) as *const u32);
    if val & 1 != 0 {
        writel(1, (versatile_sys_base as *mut u8).add(VERSATILE_SYS_PCICTL_OFFSET) as *mut u32);
        of_node_put(np);
        return;
    }

    newprop = kzalloc_obj();
    if newprop.is_null() {
        of_node_put(np);
        return;
    }

    (*newprop).name = kstrdup("status".as_ptr() as *const i8, GFP_KERNEL);
    (*newprop).value = kstrdup("disabled".as_ptr() as *const i8, GFP_KERNEL);
    (*newprop).length = core::mem::size_of::<[u8; 9]>();
    of_update_property(np, newprop);

    pr_info!("Not plugged into PCI backplane!\n");
    of_node_put(np);
}

unsafe fn versatile_dt_init() {
    let np: *mut device_node;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "arm,core-module-versatile".as_ptr() as *const i8);
    if !np.is_null() {
        versatile_sys_base = of_iomap(np, 0);
    }
    WARN_ON(versatile_sys_base.is_null());

    versatile_dt_pci_init();
    of_platform_default_populate(core::ptr::null(), versatile_auxdata_lookup.as_ptr(), core::ptr::null_mut());
}

static versatile_dt_match: [*const i8; 3] = [
    "arm,versatile-ab".as_ptr() as *const i8,
    "arm,versatile-pb".as_ptr() as *const i8,
    core::ptr::null(),
];

// DT_MACHINE_START(VERSATILE_PB, "ARM-Versatile (Device Tree Support)")
// .map_io = versatile_map_io, .init_early = versatile_init_early,
// .init_machine = versatile_dt_init, .dt_compat = versatile_dt_match,
// MACHINE_END


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
