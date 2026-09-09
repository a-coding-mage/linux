// SPDX-License-Identifier: GPL-2.0-only

// Translated from the C implementation. The referenced kernel symbols and
// macros are supplied by other files.

#[inline]
unsafe fn get_satp_value() -> u64 {
    csr_read(CSR_SATP)
}

pub unsafe fn arch_crash_save_vmcoreinfo() {
    VMCOREINFO_NUMBER!(phys_ram_base);

    vmcoreinfo_append_str!("NUMBER(PAGE_OFFSET)=0x%lx\n", PAGE_OFFSET);
    vmcoreinfo_append_str!("NUMBER(VMALLOC_END)=0x%lx\n", VMALLOC_END);
    // #ifdef CONFIG_MMU
    VMCOREINFO_NUMBER!(VA_BITS);
    vmcoreinfo_append_str!("NUMBER(VMEMMAP_START)=0x%lx\n", VMEMMAP_START);
    vmcoreinfo_append_str!("NUMBER(VMEMMAP_END)=0x%lx\n", VMEMMAP_END);
    // #ifdef CONFIG_64BIT
    vmcoreinfo_append_str!("NUMBER(MODULES_VADDR)=0x%lx\n", MODULES_VADDR);
    vmcoreinfo_append_str!("NUMBER(MODULES_END)=0x%lx\n", MODULES_END);
    // #endif
    // #endif
    vmcoreinfo_append_str!("NUMBER(KERNEL_LINK_ADDR)=0x%lx\n", KERNEL_LINK_ADDR);
    vmcoreinfo_append_str!(
        "NUMBER(va_kernel_pa_offset)=0x%lx\n",
        kernel_map.va_kernel_pa_offset,
    );
    vmcoreinfo_append_str!("KERNELOFFSET=%lx\n", kaslr_offset());
    vmcoreinfo_append_str!("NUMBER(satp)=0x%llx\n", get_satp_value());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
