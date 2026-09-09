/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux and MicroBlaze declarations supplied by the surrounding kernel.

#[no_mangle]
pub static mut KSP: u32 = 0;
#[no_mangle]
pub static mut KM: u32 = 0;
#[no_mangle]
pub static mut ENTRY_SP: u32 = 0;
#[no_mangle]
pub static mut R11_SAVE: u32 = 0;
#[no_mangle]
pub static mut CURRENT_SAVE: u32 = 0;

/*
 * Placed cmd_line to .data section because can be initialized from
 * ASM code. Default position is BSS section which is cleared
 * in machine_early_init().
 */
#[link_section = ".data"]
#[no_mangle]
pub static mut cmd_line: [u8; COMMAND_LINE_SIZE as usize] = [0; COMMAND_LINE_SIZE as usize];

pub unsafe fn setup_arch(cmdline_p: *mut *mut u8) {
    *cmdline_p = boot_command_line;

    setup_memory();
    console_verbose();
    unflatten_device_tree();
    setup_cpuinfo();
    microblaze_cache_init();
    xilinx_pci_init();
}

#[cfg(feature = "CONFIG_MTD_UCLINUX")]
#[inline]
pub unsafe fn get_romfs_len(addr: *mut u32) -> u32 {
    #[cfg(feature = "CONFIG_ROMFS_FS")]
    {
        if core::slice::from_raw_parts(addr as *const u8, 8) == b"-rom1fs-" {
            return u32::from_be((*addr.add(2)).to_be());
        }
    }
    #[cfg(feature = "CONFIG_CRAMFS")]
    {
        if *addr == u32::from_le(0x28cd3d45) {
            return u32::from_le(*addr.add(1));
        }
    }
    0
}

#[no_mangle]
pub static mut kernel_tlb: usize = 0;

pub unsafe fn machine_early_init(
    cmdline: *const u8,
    ram: u32,
    fdt: u32,
    msr: u32,
    tlb0: u32,
    tlb1: u32,
) {
    let mut src: *mut usize;
    let mut dst: *mut usize;
    let mut offset: u32 = 0;

    #[cfg(feature = "CONFIG_MTD_UCLINUX")]
    {
        let mut romfs_size: i32;
        let mut romfs_base: u32;
        let old_klimit = klimit;

        romfs_base = if ram != 0 { ram } else { &__init_end as *const _ as u32 };
        romfs_size = PAGE_ALIGN(get_romfs_len(romfs_base as *mut u32)) as i32;
        if romfs_size == 0 {
            romfs_base = &__bss_start as *const _ as u32;
            romfs_size = PAGE_ALIGN(get_romfs_len(romfs_base as *mut u32)) as i32;
        }
        if romfs_size > 0 {
            core::ptr::copy(romfs_base as *const u8, &mut __bss_stop as *mut _ as *mut u8, romfs_size as usize);
            klimit = klimit.wrapping_add(romfs_size as usize);
        }
        let _ = old_klimit;
    }

    core::ptr::write_bytes(__bss_start, 0, __bss_stop as usize - __bss_start as usize);
    core::ptr::write_bytes(_ssbss, 0, _esbss as usize - _ssbss as usize);
    early_init_devtree(_fdt_start);

    kernel_tlb = (tlb0 as usize).wrapping_add(tlb1 as usize);
    pr_info!("Ramdisk addr 0x{:08x}, ", ram);
    if fdt != 0 { pr_info!("FDT at 0x{:08x}\n", fdt); }
    else { pr_info!("Compiled-in FDT at {:?}\n", _fdt_start); }

    #[cfg(feature = "CONFIG_MTD_UCLINUX")]
    {
        pr_info!("Found romfs @ 0x{:08x} (0x{:08x})\n", romfs_base, romfs_size);
        pr_info!("#### klimit {:?} ####\n", old_klimit);
        BUG_ON!(romfs_size < 0);
        pr_info!("Moved 0x{:08x} bytes from 0x{:08x} to 0x{:08x}\n", romfs_size, romfs_base, &__bss_stop);
        pr_info!("New klimit: 0x{:08x}\n", klimit);
    }

    #[cfg(feature = "CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR")]
    if msr != 0 { pr_info!("!!!Your kernel has setup MSR instruction but "); pr_cont!("CPU don't have it {:x}\n", msr); }
    #[cfg(not(feature = "CONFIG_XILINX_MICROBLAZE0_USE_MSR_INSTR"))]
    if msr == 0 { pr_info!("!!!Your kernel not setup MSR instruction but "); pr_cont!("CPU have it {:x}\n", msr); }

    #[cfg(not(feature = "CONFIG_MANUAL_RESET_VECTOR"))]
    { offset = 0x2; }
    dst = (offset as usize * core::mem::size_of::<u32>()) as *mut usize;
    src = __ivt_start.add(offset as usize);
    while src < __ivt_end { *dst = *src; src = src.add(1); dst = dst.add(1); }
    KM = 0x1;
    CURRENT_SAVE = current as usize as u32;
}

pub unsafe fn time_init() { of_clk_init(core::ptr::null()); setup_cpuinfo_clk(); timer_probe(); }

#[cfg(feature = "CONFIG_DEBUG_FS")]
pub static mut of_debugfs_root: *mut dentry = core::ptr::null_mut();

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn microblaze_debugfs_init() -> i32 { of_debugfs_root = debugfs_create_dir(b"microblaze\0".as_ptr(), core::ptr::null_mut()); 0 }

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn debugfs_tlb() -> i32 { debugfs_create_u32(b"tlb_skip\0".as_ptr(), S_IRUGO, of_debugfs_root, &mut tlb_skip); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
