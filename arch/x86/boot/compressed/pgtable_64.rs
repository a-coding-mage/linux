// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit.

const BIOS_START_MIN: u32 = 0x20000; // 128K, less than this is insane
const BIOS_START_MAX: u32 = 0x9f000; // 640K, absolute maximum

/* __pgtable_l5_enabled needs to be in .data to avoid being cleared along with .bss */
#[link_section = ".data"]
pub static mut __PGTABLE_L5_ENABLED: u32 = 0;
#[link_section = ".data"]
pub static mut pgdir_shift: u32 = 39;
#[link_section = ".data"]
pub static mut ptrs_per_p4d: u32 = 1;

/* Buffer to preserve trampoline memory */
static mut trampoline_save: [core::ffi::c_char; TRAMPOLINE_32BIT_SIZE] =
    [0; TRAMPOLINE_32BIT_SIZE];

/*
 * Trampoline address will be printed by extract_kernel() for debugging
 * purposes.
 *
 * Avoid putting the pointer into .bss as it will be cleared between
 * configure_5level_paging() and extract_kernel().
 */
#[link_section = ".data"]
pub static mut trampoline_32bit: *mut u64 = core::ptr::null_mut();

extern "C" {
    fn cmdline_find_option_bool(option: *const core::ffi::c_char) -> core::ffi::c_int;
}

unsafe fn find_trampoline_placement() -> usize {
    let mut bios_start: usize = 0;
    let mut ebda_start: usize = 0;
    let mut entry: *mut boot_e820_entry;
    let mut signature: *mut core::ffi::c_char;
    let mut i: i32;

    /*
     * Find a suitable spot for the trampoline.
     * This code is based on reserve_bios_regions().
     */

    /*
     * EFI systems may not provide legacy ROM. The memory may not be mapped
     * at all.
     *
     * Only look for values in the legacy ROM for non-EFI system.
     */
    signature = &mut (*boot_params_ptr).efi_info.efi_loader_signature as *mut _ as *mut _;
    if strncmp(signature, EFI32_LOADER_SIGNATURE, 4) != 0
        && strncmp(signature, EFI64_LOADER_SIGNATURE, 4) != 0
    {
        ebda_start = (*(0x40e as *const u16) as usize) << 4;
        bios_start = (*(0x413 as *const u16) as usize) << 10;
    }

    if bios_start < BIOS_START_MIN as usize || bios_start > BIOS_START_MAX as usize {
        bios_start = BIOS_START_MAX as usize;
    }

    if ebda_start > BIOS_START_MIN as usize && ebda_start < bios_start {
        bios_start = ebda_start;
    }

    bios_start = round_down(bios_start, PAGE_SIZE);

    /* Find the first usable memory region under bios_start. */
    i = (*boot_params_ptr).e820_entries as i32 - 1;
    while i >= 0 {
        let mut new = bios_start;

        entry = &mut (*boot_params_ptr).e820_table[i as usize];

        /* Skip all entries above bios_start. */
        if bios_start <= (*entry).addr as usize {
            i -= 1;
            continue;
        }

        /* Skip non-RAM entries. */
        if (*entry).type_ != E820_TYPE_RAM {
            i -= 1;
            continue;
        }

        /* Adjust bios_start to the end of the entry if needed. */
        if bios_start > ((*entry).addr + (*entry).size) as usize {
            new = ((*entry).addr + (*entry).size) as usize;
        }

        /* Keep bios_start page-aligned. */
        new = round_down(new, PAGE_SIZE);

        /* Skip the entry if it's too small. */
        if new - TRAMPOLINE_32BIT_SIZE < (*entry).addr as usize {
            i -= 1;
            continue;
        }

        /* Protect against underflow. */
        if new - TRAMPOLINE_32BIT_SIZE > bios_start {
            break;
        }

        bios_start = new;
        break;
    }

    /* Place the trampoline just below the end of low memory */
    bios_start - TRAMPOLINE_32BIT_SIZE
}

pub unsafe extern "C" fn configure_5level_paging(bp: *mut boot_params, pgtable: *mut core::ffi::c_void) {
    let mut toggle_la57: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> = None;
    let mut l5_required = false;

    /* Initialize boot_params. Required for cmdline_find_option_bool(). */
    sanitize_boot_params(bp);
    boot_params_ptr = bp;

    /*
     * Check if LA57 is desired and supported.
     *
     * There are several parts to the check:
     *   - if user asked to disable 5-level paging: no5lvl in cmdline
     *   - if the machine supports 5-level paging:
     *     + CPUID leaf 7 is supported
     *     + the leaf has the feature bit set
     */
    if cmdline_find_option_bool(b"no5lvl\0".as_ptr() as *const _) == 0
        && native_cpuid_eax(0) >= 7
        && (native_cpuid_ecx(7) & BIT(16)) != 0
    {
        l5_required = true;

        /* Initialize variables for 5-level paging */
        __PGTABLE_L5_ENABLED = 1;
        pgdir_shift = 48;
        ptrs_per_p4d = 512;
    }

    /*
     * The trampoline will not be used if the paging mode is already set to
     * the desired one.
     */
    if l5_required == ((native_read_cr4() & X86_CR4_LA57) != 0) {
        return;
    }

    trampoline_32bit = find_trampoline_placement() as *mut u64;

    /* Preserve trampoline memory */
    memcpy(trampoline_save.as_mut_ptr() as *mut _, trampoline_32bit, TRAMPOLINE_32BIT_SIZE);

    /* Clear trampoline memory first */
    memset(trampoline_32bit as *mut _, 0, TRAMPOLINE_32BIT_SIZE);

    /* Copy trampoline code in place */
    toggle_la57 = Some(core::mem::transmute(memcpy(
        trampoline_32bit.add(TRAMPOLINE_32BIT_CODE_OFFSET / core::mem::size_of::<u64>()),
        &trampoline_32bit_src as *const _,
        TRAMPOLINE_32BIT_CODE_SIZE,
    )));

    /*
     * Avoid the need for a stack in the 32-bit trampoline code, by using
     * LJMP rather than LRET to return back to long mode. LJMP takes an
     * immediate absolute address, which needs to be adjusted based on the
     * placement of the trampoline.
     */
    let toggle_la57_ptr = toggle_la57.unwrap();
    *((toggle_la57_ptr as *mut u8).add(trampoline_ljmp_imm_offset) as *mut u32) +=
        toggle_la57_ptr as usize as u32;

    /*
     * The code below prepares page table in trampoline memory.
     *
     * The new page table will be used by trampoline code for switching
     * from 4- to 5-level paging or vice versa.
     */

    if l5_required {
        /*
         * For 4- to 5-level paging transition, set up current CR3 as
         * the first and the only entry in a new top-level page table.
         */
        *trampoline_32bit = native_read_cr3_pa() | _PAGE_TABLE_NOENC;
    } else {
        let new_cr3: *mut u64;
        let pgdp: *mut pgd_t;

        /*
         * For 5- to 4-level paging transition, copy page table pointed
         * by first entry in the current top-level page table as our
         * new top-level page table.
         *
         * We cannot just point to the page table from trampoline as it
         * may be above 4G.
         */
        pgdp = native_read_cr3_pa() as *mut pgd_t;
        new_cr3 = (native_pgd_val(*pgdp) & PTE_PFN_MASK) as *mut u64;
        memcpy(trampoline_32bit, new_cr3, PAGE_SIZE);
    }

    toggle_la57(trampoline_32bit as *mut _);

    /*
     * Move the top level page table out of trampoline memory.
     */
    memcpy(pgtable, trampoline_32bit, PAGE_SIZE);
    native_write_cr3(pgtable as usize);

    /* Restore trampoline memory */
    memcpy(trampoline_32bit, trampoline_save.as_ptr(), TRAMPOLINE_32BIT_SIZE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
