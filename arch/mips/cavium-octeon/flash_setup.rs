/*
 *   Octeon Bootbus flash setup
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007, 2008 Cavium Networks
 */

// Dependencies supplied by the surrounding kernel and architecture code.

static mut FLASH_MAP: map_info = unsafe { core::mem::zeroed() };
static mut MYMTD: *mut mtd_info = core::ptr::null_mut();
static PART_PROBE_TYPES: [*const core::ffi::c_char; 3] = [
    b"cmdlinepart\0".as_ptr() as *const core::ffi::c_char,
    // CONFIG_MTD_REDBOOT_PARTS conditionally adds: b"RedBoot\0".
    core::ptr::null(),
];

unsafe fn octeon_flash_map_read(map: *mut map_info, ofs: c_ulong) -> map_word {
    down(&mut octeon_bootbus_sem);
    let r = inline_map_read(map, ofs);
    up(&mut octeon_bootbus_sem);
    r
}

unsafe fn octeon_flash_map_write(
    map: *mut map_info,
    datum: map_word,
    ofs: c_ulong,
) {
    down(&mut octeon_bootbus_sem);
    inline_map_write(map, datum, ofs);
    up(&mut octeon_bootbus_sem);
}

unsafe fn octeon_flash_map_copy_from(
    map: *mut map_info,
    to: *mut core::ffi::c_void,
    from: c_ulong,
    len: isize,
) {
    down(&mut octeon_bootbus_sem);
    inline_map_copy_from(map, to, from, len);
    up(&mut octeon_bootbus_sem);
}

unsafe fn octeon_flash_map_copy_to(
    map: *mut map_info,
    to: c_ulong,
    from: *const core::ffi::c_void,
    len: isize,
) {
    down(&mut octeon_bootbus_sem);
    inline_map_copy_to(map, to, from, len);
    up(&mut octeon_bootbus_sem);
}

/*
 * Module/ driver initialization.
 *
 * Returns Zero on success
 */
unsafe fn octeon_flash_probe(pdev: *mut platform_device) -> i32 {
    let mut region_cfg: cvmx_mio_boot_reg_cfgx = core::mem::zeroed();
    let mut cs: u32 = 0;
    let np = (*pdev).dev.of_node;

    let mut r = of_property_read_u32(np, b"reg\0".as_ptr() as *const i8, &mut cs);
    if r != 0 {
        return r;
    }

    /*
     * Read the bootbus region 0 setup to determine the base
     * address of the flash.
     */
    region_cfg.u64 = cvmx_read_csr(CVMX_MIO_BOOT_REG_CFGX(cs));
    if region_cfg.s.en != 0 {
        /*
         * The bootloader always takes the flash and sets its
         * address so the entire flash fits below
         * 0x1fc00000. This way the flash aliases to
         * 0x1fc00000 for booting. Software can access the
         * full flash at the true address, while core boot can
         * access 4MB.
         */
        /* Use this name so old part lines work */
        FLASH_MAP.name = b"phys_mapped_flash\0".as_ptr() as *const i8;
        FLASH_MAP.phys = region_cfg.s.base << 16;
        FLASH_MAP.size = 0x1fc00000 - FLASH_MAP.phys;
        /* 8-bit bus (0 + 1) or 16-bit bus (1 + 1) */
        FLASH_MAP.bankwidth = region_cfg.s.width + 1;
        FLASH_MAP.virt = ioremap(FLASH_MAP.phys, FLASH_MAP.size);
        pr_notice!(
            "Bootbus flash: Setting flash for %luMB flash at 0x%08llx\n",
            FLASH_MAP.size >> 20,
            FLASH_MAP.phys,
        );
        WARN_ON(!map_bankwidth_supported(FLASH_MAP.bankwidth));
        FLASH_MAP.read = Some(octeon_flash_map_read);
        FLASH_MAP.write = Some(octeon_flash_map_write);
        FLASH_MAP.copy_from = Some(octeon_flash_map_copy_from);
        FLASH_MAP.copy_to = Some(octeon_flash_map_copy_to);
        MYMTD = do_map_probe(b"cfi_probe\0".as_ptr() as *const i8, &mut FLASH_MAP);
        if !MYMTD.is_null() {
            (*MYMTD).owner = THIS_MODULE;
            mtd_device_parse_register(
                MYMTD,
                PART_PROBE_TYPES.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                0,
            );
        } else {
            pr_err!("Failed to register MTD device for flash\n");
        }
    }
    0
}

static OF_FLASH_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"cfi-flash\0".as_ptr() as *const i8,
    },
    of_device_id { compatible: core::ptr::null() },
];

static mut OF_FLASH_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"octeon-of-flash\0".as_ptr() as *const i8,
        of_match_table: OF_FLASH_MATCH.as_ptr(),
    },
    probe: Some(octeon_flash_probe),
};

unsafe fn octeon_flash_init() -> i32 {
    platform_driver_register(&mut OF_FLASH_DRIVER)
}

// late_initcall(octeon_flash_init);
// MODULE_DEVICE_TABLE(of, of_flash_match);
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
