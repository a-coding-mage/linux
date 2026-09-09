// SPDX-License-Identifier: GPL-2.0

// External kernel declarations and configuration supplied by other files.

static mut protection_map: [pgprot_t; 16] = [
    PAGE_NONE,
    PAGE_READONLY,
    PAGE_COPY,
    PAGE_COPY,
    PAGE_READONLY_EXEC,
    PAGE_READONLY_EXEC,
    PAGE_COPY_EXEC,
    PAGE_COPY_EXEC,
    PAGE_NONE,
    PAGE_READONLY,
    PAGE_SHARED,
    PAGE_SHARED,
    PAGE_READONLY_EXEC,
    PAGE_READONLY_EXEC,
    PAGE_SHARED_EXEC,
    PAGE_SHARED_EXEC,
];

pub unsafe fn add_encrypt_protection_map() {
    let mut i: usize = 0;

    while i < core::mem::size_of_val(&protection_map) / core::mem::size_of::<pgprot_t>() {
        protection_map[i] = pgprot_encrypted(protection_map[i]);
        i += 1;
    }
}

pub unsafe fn vm_get_page_prot(vm_flags: vm_flags_t) -> pgprot_t {
    let mut val: c_ulong = pgprot_val(
        protection_map[(vm_flags & (VM_READ | VM_WRITE | VM_EXEC | VM_SHARED)) as usize],
    );

    // CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS
    //
    // Take the 4 protection key bits out of the vma->vm_flags value and
    // turn them in to the bits that we can put in to a pte.
    //
    // Only override these if Protection Keys are available (which is only
    // on 64-bit).
    #[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
    {
        if vm_flags & VM_PKEY_BIT0 != 0 {
            val |= _PAGE_PKEY_BIT0;
        }
        if vm_flags & VM_PKEY_BIT1 != 0 {
            val |= _PAGE_PKEY_BIT1;
        }
        if vm_flags & VM_PKEY_BIT2 != 0 {
            val |= _PAGE_PKEY_BIT2;
        }
        if vm_flags & VM_PKEY_BIT3 != 0 {
            val |= _PAGE_PKEY_BIT3;
        }
    }

    val = __sme_set(val);
    if val & _PAGE_PRESENT != 0 {
        val &= __supported_pte_mask;
    }
    __pgprot(val)
}

// EXPORT_SYMBOL(vm_get_page_prot);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
