// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) Linaro.
 * Copyright (C) Huawei Futurewei Technologies.
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
fn get_tcr_el1_t1sz() -> u64 {
    (read_sysreg(tcr_el1) & TCR_EL1_T1SZ_MASK) >> TCR_EL1_T1SZ_SHIFT
}

pub unsafe fn arch_crash_save_vmcoreinfo() {
    VMCOREINFO_NUMBER!(VA_BITS);
    /* Please note VMCOREINFO_NUMBER() uses "%d", not "%x" */
    vmcoreinfo_append_str!("NUMBER(MODULES_VADDR)=0x%lx\n", MODULES_VADDR);
    vmcoreinfo_append_str!("NUMBER(MODULES_END)=0x%lx\n", MODULES_END);
    vmcoreinfo_append_str!("NUMBER(VMALLOC_END)=0x%lx\n", VMALLOC_END);
    vmcoreinfo_append_str!("NUMBER(VMEMMAP_START)=0x%lx\n", VMEMMAP_START);
    vmcoreinfo_append_str!("NUMBER(VMEMMAP_END)=0x%lx\n", VMEMMAP_END);
    vmcoreinfo_append_str!("NUMBER(kimage_voffset)=0x%llx\n", kimage_voffset);
    vmcoreinfo_append_str!("NUMBER(PHYS_OFFSET)=0x%llx\n", PHYS_OFFSET);
    vmcoreinfo_append_str!(
        "NUMBER(TCR_EL1_T1SZ)=0x%llx\n",
        get_tcr_el1_t1sz(),
    );
    vmcoreinfo_append_str!("KERNELOFFSET=%lx\n", kaslr_offset());
    vmcoreinfo_append_str!(
        "NUMBER(KERNELPACMASK)=0x%llx\n",
        if system_supports_address_auth() {
            ptrauth_kernel_pac_mask()
        } else {
            0
        },
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
