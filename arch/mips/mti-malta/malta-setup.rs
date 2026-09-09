// SPDX-License-Identifier: GPL-2.0-only
/*
 * Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000 MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2008 Dmitri Vorobiev
 */

// Linux and MIPS platform dependencies supplied by the surrounding kernel.

const ROCIT_CONFIG_GEN0: u32 = 0x1f403000;
const ROCIT_CONFIG_GEN0_PCI_IOCU: u32 = 1 << 7;

static mut STANDARD_IO_RESOURCES: [Resource; 5] = [
    Resource { name: c"dma1".as_ptr(), start: 0x00, end: 0x1f, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    Resource { name: c"timer".as_ptr(), start: 0x40, end: 0x5f, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    Resource { name: c"keyboard".as_ptr(), start: 0x60, end: 0x6f, flags: IORESOURCE_IO },
    Resource { name: c"dma page reg".as_ptr(), start: 0x80, end: 0x8f, flags: IORESOURCE_IO | IORESOURCE_BUSY },
    Resource { name: c"dma2".as_ptr(), start: 0xc0, end: 0xdf, flags: IORESOURCE_IO | IORESOURCE_BUSY },
];

#[no_mangle]
pub unsafe extern "C" fn get_system_type() -> *const core::ffi::c_char {
    c"MIPS Malta".as_ptr()
}

#[cfg(feature = "CONFIG_BLK_DEV_FD")]
unsafe fn fd_activate() {
    /*
     * Activate Floppy Controller in the SMSC FDC37M817 Super I/O
     * Controller.
     * Done by YAMON 2.00 onwards
     */
    // Entering config state.
    SMSC_WRITE(SMSC_CONFIG_ENTER, SMSC_CONFIG_REG);

    // Activate floppy controller.
    SMSC_WRITE(SMSC_CONFIG_DEVNUM, SMSC_CONFIG_REG);
    SMSC_WRITE(SMSC_CONFIG_DEVNUM_FLOPPY, SMSC_DATA_REG);
    SMSC_WRITE(SMSC_CONFIG_ACTIVATE, SMSC_CONFIG_REG);
    SMSC_WRITE(SMSC_CONFIG_ACTIVATE_ENABLE, SMSC_DATA_REG);

    // Exit config state.
    SMSC_WRITE(SMSC_CONFIG_EXIT, SMSC_CONFIG_REG);
}

unsafe fn plat_setup_iocoherency() {
    let mut cfg: u32;

    if mips_revision_sconid == MIPS_REVISION_SCON_BONITO {
        if BONITO_PCICACHECTRL & BONITO_PCICACHECTRL_CPUCOH_PRES != 0 {
            BONITO_PCICACHECTRL |= BONITO_PCICACHECTRL_CPUCOH_EN;
            pr_info!("Enabled Bonito CPU coherency\n");
            dma_default_coherent = true;
        }
        if strstr(fw_getcmdline(), c"iobcuncached".as_ptr()).is_some() {
            BONITO_PCICACHECTRL &= !BONITO_PCICACHECTRL_IOBCCOH_EN;
            BONITO_PCIMEMBASECFG &= !(BONITO_PCIMEMBASECFG_MEMBASE0_CACHED |
                BONITO_PCIMEMBASECFG_MEMBASE1_CACHED);
            pr_info!("Disabled Bonito IOBC coherency\n");
        } else {
            BONITO_PCICACHECTRL |= BONITO_PCICACHECTRL_IOBCCOH_EN;
            BONITO_PCIMEMBASECFG |= BONITO_PCIMEMBASECFG_MEMBASE0_CACHED |
                BONITO_PCIMEMBASECFG_MEMBASE1_CACHED;
            pr_info!("Enabled Bonito IOBC coherency\n");
        }
    } else if mips_cps_numiocu(0) != 0 {
        // Nothing special needs to be done to enable coherency
        pr_info!("CMP IOCU detected\n");
        cfg = __raw_readl(CKSEG1ADDR(ROCIT_CONFIG_GEN0) as *const u32);
        if cfg & ROCIT_CONFIG_GEN0_PCI_IOCU != 0 {
            dma_default_coherent = true;
        } else {
            pr_crit!("IOCU OPERATION DISABLED BY SWITCH - DEFAULTING TO SW IO COHERENCY\n");
        }
    }

    if dma_default_coherent {
        pr_info!("Hardware DMA cache coherency enabled\n");
    } else {
        pr_info!("Software DMA cache coherency enabled\n");
    }
}

unsafe fn pci_clock_check() {
    let jmpr_p = ioremap(MALTA_JMPRS_REG, core::mem::size_of::<u32>()) as *mut u32;
    let jmpr = (__raw_readl(jmpr_p) >> 2) & 0x07;
    static PCICLOCKS: [i32; 8] = [33, 20, 25, 30, 12, 16, 37, 10];
    let pciclock = PCICLOCKS[jmpr as usize];
    let mut argptr = fw_getcmdline();

    /* If user passed a pci_clock= option, don't tack on another one. */
    let optptr = strstr(argptr, c"pci_clock=".as_ptr());
    if !optptr.is_null() && (optptr == argptr || *optptr.offset(-1) == b'\0' as i8 + b' ') {
        return;
    }

    if pciclock != 33 {
        pr_warn!("WARNING: PCI clock is %dMHz, setting pci_clock\n", pciclock);
        argptr = argptr.add(strlen(argptr));
        sprintf(argptr, c" pci_clock=%d".as_ptr(), pciclock);
        if pciclock < 20 || pciclock > 66 {
            pr_warn!("WARNING: IDE timing calculations will be incorrect\n");
        }
    }
}

#[cfg(all(feature = "CONFIG_VT", feature = "CONFIG_VGA_CONSOLE"))]
unsafe fn screen_info_setup() {
    static mut SI: ScreenInfo = ScreenInfo {
        orig_x: 0, orig_y: 25, ext_mem_k: 0, orig_video_page: 0,
        orig_video_mode: 0, orig_video_cols: 80, unused2: 0,
        orig_video_ega_bx: 0, unused3: 0, orig_video_lines: 25,
        orig_video_isVGA: VIDEO_TYPE_VGAC, orig_video_points: 16,
    };
    vgacon_register_screen(&raw mut SI);
}

unsafe fn bonito_quirks_setup() {
    let argptr = fw_getcmdline();
    if !strstr(argptr, c"debug".as_ptr()).is_null() {
        BONITO_BONGENCFG |= BONITO_BONGENCFG_DEBUGMODE;
        pr_info!("Enabled Bonito debug mode\n");
    } else {
        BONITO_BONGENCFG &= !BONITO_BONGENCFG_DEBUGMODE;
    }
}

pub unsafe extern "C" fn plat_get_fdt() -> *mut core::ffi::c_void {
    __dtb_start as *mut core::ffi::c_void
}

pub unsafe extern "C" fn plat_mem_setup() {
    let fdt = malta_dt_shim(plat_get_fdt());
    __dt_setup_arch(fdt);

    if cfg!(feature = "CONFIG_EVA") {
        // EVA has already been configured in mach-malta/kernel-init.h
        pr_info!("Enhanced Virtual Addressing (EVA) activated\n");
    }

    mips_pcibios_init();

    // Request I/O space for devices used on the Malta board.
    for resource in STANDARD_IO_RESOURCES.iter_mut() {
        insert_resource(&raw mut ioport_resource, resource);
    }

    // Enable DMA channel 4 (cascade channel) in the PIIX4 south bridge.
    enable_dma(4);

    if mips_revision_sconid == MIPS_REVISION_SCON_BONITO {
        bonito_quirks_setup();
    }

    plat_setup_iocoherency();
    pci_clock_check();

    #[cfg(feature = "CONFIG_BLK_DEV_FD")]
    fd_activate();

    #[cfg(all(feature = "CONFIG_VT", feature = "CONFIG_VGA_CONSOLE"))]
    screen_info_setup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
