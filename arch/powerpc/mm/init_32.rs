// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  PowerPC version
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *  PPC44x/36-bit changes by Matt Porter (mporter@mvista.com)
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

// The included Linux and PowerPC headers provide the types, constants,
// globals, and functions referenced below.

#[cfg(any(CONFIG_KERNEL_START_BOOL, CONFIG_LOWMEM_SIZE_BOOL))]
// The amount of lowmem must be within 0xF0000000 - KERNELBASE.
// Build-time configuration must enforce the corresponding C preprocessor check.
const _LOWMEM_CONFIGURATION_CHECK: () = ();

const MAX_LOW_MEM: usize = CONFIG_LOWMEM_SIZE;

pub static mut total_memory: phys_addr_t = 0;
pub static mut total_lowmem: phys_addr_t = 0;

#[cfg(CONFIG_RELOCATABLE)]
pub static mut virt_phys_offset: i64 = 0;

pub static mut lowmem_end_addr: phys_addr_t = 0;

pub static mut boot_mapsize: i32 = 0;

#[cfg(CONFIG_PPC_PMAC)]
pub static mut agp_special_page: usize = 0;

extern "C" {
    static mut ppc_md: MachDep;
    static mut memstart_addr: phys_addr_t;
    static mut ioremap_bot: usize;

    fn memblock_end_of_DRAM() -> phys_addr_t;
    fn adjust_total_lowmem();
    fn memblock_enforce_memory_limit(limit: phys_addr_t);
    fn MMU_init_hw();
    fn mapin_ram();
    fn btext_unmap();
    fn kasan_mmu_init();
    fn setup_kup();
    fn update_mmu_feature_fixups(feature: u32);
    fn memblock_set_current_limit(limit: phys_addr_t);
}

#[repr(C)]
pub struct MachDep {
    pub progress: Option<unsafe extern "C" fn(message: *const u8, value: u32)>,
}

pub unsafe fn MMU_init() {
    if let Some(progress) = ppc_md.progress {
        progress(b"MMU:enter\0".as_ptr(), 0x111);
    }

    total_lowmem = memblock_end_of_DRAM().wrapping_sub(memstart_addr);
    total_memory = total_lowmem;
    lowmem_end_addr = memstart_addr.wrapping_add(total_lowmem);

    #[cfg(CONFIG_PPC_85xx)]
    {
        /* Freescale Book-E parts expect lowmem to be mapped by fixed TLB
         * entries, so we need to adjust lowmem to match the amount we can map
         * in the fixed entries */
        adjust_total_lowmem();
    }

    if total_lowmem > MAX_LOW_MEM {
        total_lowmem = MAX_LOW_MEM;
        lowmem_end_addr = memstart_addr.wrapping_add(total_lowmem);
        #[cfg(not(CONFIG_HIGHMEM))]
        {
            total_memory = total_lowmem;
            memblock_enforce_memory_limit(total_lowmem);
        }
    }

    /* Initialize the MMU hardware */
    if let Some(progress) = ppc_md.progress {
        progress(b"MMU:hw init\0".as_ptr(), 0x300);
    }
    MMU_init_hw();

    /* Map in all of RAM starting at KERNELBASE */
    if let Some(progress) = ppc_md.progress {
        progress(b"MMU:mapin\0".as_ptr(), 0x301);
    }
    mapin_ram();

    /* Initialize early top-down ioremap allocator */
    ioremap_bot = IOREMAP_TOP;

    if let Some(progress) = ppc_md.progress {
        progress(b"MMU:exit\0".as_ptr(), 0x211);
    }

    /* From now on, btext is no longer BAT mapped if it was at all */
    #[cfg(CONFIG_BOOTX_TEXT)]
    btext_unmap();

    kasan_mmu_init();

    setup_kup();

    update_mmu_feature_fixups(MMU_FTR_KUAP);

    /* Shortly after that, the entire linear mapping will be available */
    memblock_set_current_limit(lowmem_end_addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
