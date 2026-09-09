// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2019 SUSE
 *
 * Author: Joerg Roedel <jroedel@suse.de>
 */

// C dependencies and the shared pre-decompression boot-stage implementation
// are supplied by the surrounding kernel translation unit.

pub unsafe fn early_set_pages_state(
    mut vaddr: c_ulong,
    mut paddr: c_ulong,
    npages: c_ulong,
    desc: *const psc_desc,
) {
    let paddr_end: c_ulong;

    vaddr &= PAGE_MASK;
    paddr &= PAGE_MASK;
    paddr_end = paddr.wrapping_add(npages.wrapping_shl(PAGE_SHIFT));

    while paddr < paddr_end {
        __page_state_change(vaddr, paddr, desc);

        vaddr = vaddr.wrapping_add(PAGE_SIZE);
        paddr = paddr.wrapping_add(PAGE_SIZE);
    }
}

pub unsafe fn early_snp_set_memory_private(
    vaddr: c_ulong,
    paddr: c_ulong,
    npages: c_ulong,
) {
    let d = psc_desc {
        page_state: SNP_PAGE_STATE_PRIVATE,
        caa: rip_rel_ptr(&boot_svsm_ca_page),
        caa_pa: boot_svsm_caa_pa,
    };

    /*
     * This can be invoked in early boot while running identity mapped, so
     * use an open coded check for SNP instead of using cc_platform_has().
     * This eliminates worries about jump tables or checking boot_cpu_data
     * in the cc_platform_has() function.
     */
    if (sev_status & MSR_AMD64_SEV_SNP_ENABLED) == 0 {
        return;
    }

    /* Ask the hypervisor to mark the memory pages as private in the RMP table. */
    early_set_pages_state(vaddr, paddr, npages, &d);
}

pub unsafe fn early_snp_set_memory_shared(
    vaddr: c_ulong,
    paddr: c_ulong,
    npages: c_ulong,
) {
    let d = psc_desc {
        page_state: SNP_PAGE_STATE_SHARED,
        caa: rip_rel_ptr(&boot_svsm_ca_page),
        caa_pa: boot_svsm_caa_pa,
    };

    /*
     * This can be invoked in early boot while running identity mapped, so
     * use an open coded check for SNP instead of using cc_platform_has().
     * This eliminates worries about jump tables or checking boot_cpu_data
     * in the cc_platform_has() function.
     */
    if (sev_status & MSR_AMD64_SEV_SNP_ENABLED) == 0 {
        return;
    }

    /* Ask hypervisor to mark the memory pages shared in the RMP table. */
    early_set_pages_state(vaddr, paddr, npages, &d);
}

unsafe fn find_cc_blob(bp: *mut boot_params) -> *mut cc_blob_sev_info {
    let mut cc_info: *mut cc_blob_sev_info;

    /* Boot kernel would have passed the CC blob via boot_params. */
    if (*bp).cc_blob_address != 0 {
        cc_info = (*bp).cc_blob_address as c_ulong as *mut cc_blob_sev_info;
        goto found_cc_info;
    }

    /*
     * If kernel was booted directly, without the use of the
     * boot/decompression kernel, the CC blob may have been passed via
     * setup_data instead.
     */
    cc_info = find_cc_blob_setup_data(bp);
    if cc_info.is_null() {
        return core::ptr::null_mut();
    }

    'found_cc_info: {
        if (*cc_info).magic != CC_BLOB_SEV_HDR_MAGIC {
            sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED);
        }
    }

    cc_info
}

unsafe fn svsm_setup(cc_info: *mut cc_blob_sev_info) {
    let secrets = (*cc_info).secrets_phys as *mut snp_secrets_page;
    let mut call: svsm_call = core::mem::zeroed();
    let pa: u64;

    /*
     * Record the SVSM Calling Area address (CAA) if the guest is not
     * running at VMPL0. The CA will be used to communicate with the
     * SVSM to perform the SVSM services.
     */
    if !svsm_setup_ca(cc_info, rip_rel_ptr(&boot_svsm_ca_page)) {
        return;
    }

    /*
     * It is very early in the boot and the kernel is running identity
     * mapped but without having adjusted the pagetables to where the
     * kernel was loaded (physbase), so the get the CA address using
     * RIP-relative addressing.
     */
    pa = rip_rel_ptr(&boot_svsm_ca_page) as u64;

    /*
     * Switch over to the boot SVSM CA while the current CA is still 1:1
     * mapped and thus addressable with VA == PA. There is no GHCB at this
     * point so use the MSR protocol.
     *
     * SVSM_CORE_REMAP_CA call:
     *   RAX = 0 (Protocol=0, CallID=0)
     *   RCX = New CA GPA
     */
    call.caa = (*secrets).svsm_caa as *mut svsm_ca;
    call.rax = SVSM_CORE_CALL(SVSM_CORE_REMAP_CA);
    call.rcx = pa;

    if svsm_call_msr_protocol(&mut call) != 0 {
        sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_SVSM_CA_REMAP_FAIL);
    }

    boot_svsm_caa_pa = pa;
}

pub unsafe fn snp_init(bp: *mut boot_params) -> bool {
    let cc_info: *mut cc_blob_sev_info;

    if bp.is_null() {
        return false;
    }

    cc_info = find_cc_blob(bp);
    if cc_info.is_null() {
        return false;
    }

    if (*cc_info).secrets_phys != 0 && (*cc_info).secrets_len == PAGE_SIZE {
        sev_secrets_pa = (*cc_info).secrets_phys;
    } else {
        return false;
    }

    setup_cpuid_table(cc_info);
    svsm_setup(cc_info);

    /*
     * The CC blob will be used later to access the secrets page. Cache
     * it here like the boot kernel does.
     */
    (*bp).cc_blob_address = cc_info as c_ulong as u32;

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
