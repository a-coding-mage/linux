/*
 * Broadcom specific AMBA
 * ChipCommon NAND flash interface
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Dependencies supplied by the surrounding kernel/bcma translation.

/* Alternate NAND controller driver name in order to allow both bcm47xxnflash
 * and bcma_brcmnand to be built into the same kernel image.
 */
static mut BCMA_NFLASH_ALT_NAME: *const core::ffi::c_char =
    b"bcma_brcmnand\0".as_ptr() as *const core::ffi::c_char;

pub static mut bcma_nflash_dev: platform_device = platform_device {
    name: b"bcma_nflash\0".as_ptr() as *const core::ffi::c_char,
    num_resources: 0,
    ..unsafe { core::mem::zeroed() }
};

static mut probes: [*const core::ffi::c_char; 2] = [
    b"bcm47xxpart\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

/* Initialize NAND flash access */
pub unsafe fn bcma_nflash_init(cc: *mut bcma_drv_cc) -> i32 {
    let bus = (*(*cc).core).bus;
    let mut reg: u32;

    if (*bus).chipinfo.id != BCMA_CHIP_ID_BCM4706
        && (*(*cc).core).id.rev != 38
    {
        bcma_err(bus, b"NAND flash on unsupported board!\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOTSUPP;
    }

    if ((*cc).capabilities & BCMA_CC_CAP_NFLASH) == 0 {
        bcma_err(
            bus,
            b"NAND flash not present according to ChipCommon\n\0".as_ptr()
                as *const core::ffi::c_char,
        );
        return -ENODEV;
    }

    (*cc).nflash.present = true;
    if (*(*cc).core).id.rev == 38
        && ((*cc).status & BCMA_CC_CHIPST_5357_NAND_BOOT) != 0
    {
        (*cc).nflash.boot = true;
        /* Determine the chip select that is being used */
        reg = bcma_cc_read32(cc, BCMA_CC_NAND_CS_NAND_SELECT) & 0xff;
        (*cc).nflash.brcmnand_info.chip_select = ffs(reg) - 1;
        (*cc).nflash.brcmnand_info.part_probe_types = probes.as_ptr();
        (*cc).nflash.brcmnand_info.ecc_stepsize = 512;
        (*cc).nflash.brcmnand_info.ecc_strength = 1;
        bcma_nflash_dev.name = BCMA_NFLASH_ALT_NAME;
    }

    /* Prepare platform device, but don't register it yet. It's too early,
     * malloc (required by device_private_init) is not available yet. */
    bcma_nflash_dev.dev.platform_data = &mut (*cc).nflash as *mut _ as *mut core::ffi::c_void;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
