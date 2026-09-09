// SPDX-License-Identifier: GPL-2.0
/*
 * Hibernation support for x86
 *
 * Copyright (c) 2007 Rafael J. Wysocki <rjw@sisk.pl>
 * Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
 * Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
 */

// Linux and x86 header dependencies are supplied by the surrounding tree.

/*
 * Address to jump to in the last phase of restore in order to get to the image
 * kernel's text (this value is passed in the image header).
 */
pub static mut restore_jump_address: usize = 0;
pub static mut jump_address_phys: usize = 0;

/*
 * Value of the cr3 register from before the hibernation (this value is passed
 * in the image header).
 */
pub static mut restore_cr3: usize = 0;
pub static mut temp_pgt: usize = 0;
pub static mut relocated_restore_code: usize = 0;

/**
 * pfn_is_nosave - check if given pfn is in the 'nosave' section
 * @pfn: the page frame number to check.
 */
pub unsafe fn pfn_is_nosave(pfn: usize) -> i32 {
    let nosave_begin_pfn: usize;
    let nosave_end_pfn: usize;

    nosave_begin_pfn = __pa_symbol(&__nosave_begin as *const _) >> PAGE_SHIFT;
    nosave_end_pfn = PAGE_ALIGN(__pa_symbol(&__nosave_end as *const _)) >> PAGE_SHIFT;

    (pfn >= nosave_begin_pfn && pfn < nosave_end_pfn) as i32
}

#[repr(C)]
pub struct restore_data_record {
    pub jump_address: usize,
    pub jump_address_phys: usize,
    pub cr3: usize,
    pub magic: usize,
    pub e820_checksum: u32,
}

/**
 * compute_e820_crc32 - calculate crc32 of a given e820 table
 *
 * @table: the e820 table to be calculated
 *
 * Return: the resulting checksum
 */
unsafe fn compute_e820_crc32(table: *mut e820_table) -> u32 {
    let size = core::mem::offset_of!(e820_table, entries)
        + core::mem::size_of::<e820_entry>() * (*table).nr_entries as usize;

    !crc32_le(!0u32, table as *const u8, size)
}

#[cfg(target_arch = "x86_64")]
const RESTORE_MAGIC: usize = 0x23456789ABCDEF02usize;
#[cfg(not(target_arch = "x86_64"))]
const RESTORE_MAGIC: usize = 0x12345679usize;

/**
 * arch_hibernation_header_save - populate the architecture specific part
 *     of a hibernation image header
 * @addr: address where architecture specific header data will be saved.
 * @max_size: maximum size of architecture specific data in hibernation header.
 *
 * Return: 0 on success, -EOVERFLOW if max_size is insufficient.
 */
pub unsafe fn arch_hibernation_header_save(addr: *mut core::ffi::c_void, max_size: u32) -> i32 {
    let rdr = addr as *mut restore_data_record;

    if (max_size as usize) < core::mem::size_of::<restore_data_record>() {
        return -EOVERFLOW;
    }
    (*rdr).magic = RESTORE_MAGIC;
    (*rdr).jump_address = restore_registers as usize;
    (*rdr).jump_address_phys = __pa_symbol(restore_registers as *const _);

    /*
     * The restore code fixes up CR3 and CR4 in the following sequence:
     *
     * [in hibernation asm]
     * 1. CR3 <= temporary page tables
     * 2. CR4 <= mmu_cr4_features (from the kernel that restores us)
     * 3. CR3 <= rdr->cr3
     * 4. CR4 <= mmu_cr4_features (from us, i.e. the image kernel)
     * [in restore_processor_state()]
     * 5. CR4 <= saved CR4
     * 6. CR3 <= saved CR3
     *
     * Our mmu_cr4_features has CR4.PCIDE=0, and toggling
     * CR4.PCIDE while CR3's PCID bits are nonzero is illegal, so
     * rdr->cr3 needs to point to valid page tables but must not
     * have any of the PCID bits set.
     */
    (*rdr).cr3 = restore_cr3 & !CR3_PCID_MASK;

    (*rdr).e820_checksum = compute_e820_crc32(e820_table_firmware);
    0
}

/**
 * arch_hibernation_header_restore - read the architecture specific data
 *     from the hibernation image header
 * @addr: address to read the data from
 */
pub unsafe fn arch_hibernation_header_restore(addr: *mut core::ffi::c_void) -> i32 {
    let rdr = addr as *mut restore_data_record;

    if (*rdr).magic != RESTORE_MAGIC {
        pr_crit!("Unrecognized hibernate image header format!\n");
        return -EINVAL;
    }

    restore_jump_address = (*rdr).jump_address;
    jump_address_phys = (*rdr).jump_address_phys;
    restore_cr3 = (*rdr).cr3;

    if (*rdr).e820_checksum != compute_e820_crc32(e820_table_firmware) {
        pr_crit!("Hibernate inconsistent memory map detected!\n");
        return -ENODEV;
    }

    0
}

pub unsafe fn relocate_restore_code() -> i32 {
    let pgd: *mut pgd_t;
    let p4d: *mut p4d_t;
    let pud: *mut pud_t;
    let pmd: *mut pmd_t;
    let pte: *mut pte_t;

    relocated_restore_code = get_safe_page(GFP_ATOMIC);
    if relocated_restore_code == 0 {
        return -ENOMEM;
    }

    __memcpy(relocated_restore_code as *mut core::ffi::c_void, core_restore_code, PAGE_SIZE);

    /* Make the page containing the relocated code executable */
    pgd = (__va(read_cr3_pa()) as *mut pgd_t).add(pgd_index(relocated_restore_code));
    p4d = p4d_offset(pgd, relocated_restore_code);
    if p4d_leaf(*p4d) {
        set_p4d(p4d, __p4d(p4d_val(*p4d) & !_PAGE_NX));
    } else {
        pud = pud_offset(p4d, relocated_restore_code);
        if pud_leaf(*pud) {
            set_pud(pud, __pud(pud_val(*pud) & !_PAGE_NX));
        } else {
            pmd = pmd_offset(pud, relocated_restore_code);
            if pmd_leaf(*pmd) {
                set_pmd(pmd, __pmd(pmd_val(*pmd) & !_PAGE_NX));
            } else {
                pte = pte_offset_kernel(pmd, relocated_restore_code);
                set_pte(pte, __pte(pte_val(*pte) & !_PAGE_NX));
            }
        }
    }
    __flush_tlb_all();
    0
}

pub unsafe fn arch_resume_nosmt() -> i32 {
    let ret: i32;

    /*
     * We reached this while coming out of hibernation. This means
     * that SMT siblings are sleeping in hlt, as mwait is not safe
     * against control transition during resume (see comment in
     * hibernate_resume_nonboot_cpu_disable()).
     *
     * If the resumed kernel has SMT disabled, we have to take all
     * the SMT siblings out of hlt, and offline them again so that
     * they end up in mwait proper.
     *
     * Called with hotplug disabled.
     */
    cpu_hotplug_enable();
    ret = arch_cpu_rescan_dead_smt_siblings();
    cpu_hotplug_disable();
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
