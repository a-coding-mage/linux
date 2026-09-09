// SPDX-License-Identifier: GPL-2.0-only

// Translated from C. The VMCOREINFO_* entries below correspond to the
// declarations/macros provided by <linux/vmcore_info.h>.

unsafe extern "C" {
    fn vmcoreinfo_append_str(fmt: *const core::ffi::c_char, ...);
    fn early_radix_enabled() -> i32;
    fn kaslr_offset() -> core::ffi::c_ulong;
}

pub unsafe fn arch_crash_save_vmcoreinfo() {
    #[cfg(CONFIG_NUMA)]
    VMCOREINFO_SYMBOL!(node_data);
    #[cfg(CONFIG_NUMA)]
    VMCOREINFO_LENGTH!(node_data, MAX_NUMNODES);
    #[cfg(not(CONFIG_NUMA))]
    VMCOREINFO_SYMBOL!(contig_page_data);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_SYMBOL!(vmemmap_list);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_SYMBOL!(mmu_vmemmap_psize);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_SYMBOL!(mmu_psize_defs);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_STRUCT_SIZE!(vmemmap_backing);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_OFFSET!(vmemmap_backing, list);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_OFFSET!(vmemmap_backing, phys);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_OFFSET!(vmemmap_backing, virt_addr);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_STRUCT_SIZE!(mmu_psize_def);
    #[cfg(all(CONFIG_PPC64, CONFIG_SPARSEMEM_VMEMMAP))]
    VMCOREINFO_OFFSET!(mmu_psize_def, shift);
    VMCOREINFO_SYMBOL!(cur_cpu_spec);
    VMCOREINFO_OFFSET!(cpu_spec, cpu_features);
    VMCOREINFO_OFFSET!(cpu_spec, mmu_features);

    vmcoreinfo_append_str(
        c"NUMBER(RADIX_MMU)=%d\n".as_ptr(),
        early_radix_enabled(),
    );
    vmcoreinfo_append_str(
        c"KERNELOFFSET=%lx\n".as_ptr(),
        kaslr_offset(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
