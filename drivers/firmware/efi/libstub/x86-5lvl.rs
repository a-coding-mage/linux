// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the surrounding kernel translation unit:
// linux::efi, asm::boot, asm::cpuid::api, asm::desc, asm::efi,
// efistub, and x86_stub.

pub static mut EFI_NO5LVL: bool = false;

static mut LA57_TOGGLE: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> = None;

static GDT: [desc_struct; 2] = [
    GDT_ENTRY_INIT(DESC_CODE32, 0, 0xfffff),
    GDT_ENTRY_INIT(DESC_CODE64, 0, 0xfffff),
];

/*
 * Enabling (or disabling) 5 level paging is tricky, because it can only be
 * done from 32-bit mode with paging disabled. This means not only that the
 * code itself must be running from 32-bit addressable physical memory, but
 * also that the root page table must be 32-bit addressable, as programming
 * a 64-bit value into CR3 when running in 32-bit mode is not supported.
 */
pub unsafe fn efi_setup_5level_paging() -> efi_status_t {
    let tmpl_size: u8 = (&trampoline_ljmp_imm_offset as *const _ as usize
        - &trampoline_32bit_src as *const _ as usize) as u8;
    let status: efi_status_t;
    let mut la57_code: *mut u8 = core::ptr::null_mut();

    if !efi_is_64bit() {
        return EFI_SUCCESS;
    }

    /* check for 5 level paging support */
    if native_cpuid_eax(0) < 7
        || (native_cpuid_ecx(7) & (1 << (X86_FEATURE_LA57 & 31))) == 0
    {
        return EFI_SUCCESS;
    }

    /* allocate some 32-bit addressable memory for code and a page table */
    status = efi_allocate_pages(
        2 * PAGE_SIZE,
        &mut la57_code as *mut _ as *mut c_ulong,
        U32_MAX,
    );
    if status != EFI_SUCCESS {
        return status;
    }

    LA57_TOGGLE = Some(core::mem::transmute(memcpy(
        la57_code as *mut c_void,
        trampoline_32bit_src as *const c_void,
        tmpl_size as usize,
    )));
    memset(
        la57_code.add(tmpl_size as usize) as *mut c_void,
        0x90,
        PAGE_SIZE - tmpl_size as usize,
    );

    /*
     * To avoid the need to allocate a 32-bit addressable stack, the
     * trampoline uses a LJMP instruction to switch back to long mode.
     * LJMP takes an absolute destination address, which needs to be
     * fixed up at runtime.
     */
    *(la57_code.add(trampoline_ljmp_imm_offset) as *mut u32) =
        (*(la57_code.add(trampoline_ljmp_imm_offset) as *mut u32))
            .wrapping_add(la57_code as usize as u32);

    efi_adjust_memory_range_protection(
        LA57_TOGGLE.unwrap() as usize as c_ulong,
        PAGE_SIZE,
    );

    EFI_SUCCESS
}

pub unsafe fn efi_5level_switch() {
    let want_la57 = !EFI_NO5LVL;
    let have_la57 = native_read_cr4() & X86_CR4_LA57 != 0;
    let need_toggle = want_la57 ^ have_la57;
    let toggle = match LA57_TOGGLE {
        Some(f) => f,
        None => return,
    };
    let pgt = (toggle as usize + PAGE_SIZE) as *mut u64;
    let cr3 = native_read_cr3_pa() as *mut pgd_t;
    let mut new_cr3: *mut u64;

    if !need_toggle {
        return;
    }

    if !have_la57 {
        /*
         * 5 level paging will be enabled, so a root level page needs
         * to be allocated from the 32-bit addressable physical region,
         * with its first entry referring to the existing hierarchy.
         */
        new_cr3 = memset(pgt as *mut c_void, 0, PAGE_SIZE) as *mut u64;
        *new_cr3 = cr3 as u64 | _PAGE_TABLE_NOENC;
    } else {
        /* take the new root table pointer from the current entry #0 */
        new_cr3 = (native_pgd_val(*cr3) & PTE_PFN_MASK) as *mut u64;

        /* copy the new root table if it is not 32-bit addressable */
        if new_cr3 as u64 > U32_MAX {
            new_cr3 = memcpy(pgt as *mut c_void, new_cr3 as *const c_void, PAGE_SIZE)
                as *mut u64;
        }
    }

    native_load_gdt(&mut desc_ptr {
        size: (core::mem::size_of_val(&GDT) - 1) as _,
        address: GDT.as_ptr() as u64,
    });

    toggle(new_cr3 as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
