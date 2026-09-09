// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2024 Ventana Micro Systems Inc.
 */

// Dependencies supplied by the surrounding kernel and architecture code.

pub static mut kvm_riscv_nacl_available: bool = false;
pub static mut kvm_riscv_nacl_sync_csr_available: bool = false;
pub static mut kvm_riscv_nacl_sync_hfence_available: bool = false;
pub static mut kvm_riscv_nacl_sync_sret_available: bool = false;
pub static mut kvm_riscv_nacl_autoswap_csr_available: bool = false;
pub static mut kvm_riscv_nacl: kvm_riscv_nacl = unsafe { core::mem::zeroed() };

pub unsafe fn __kvm_riscv_nacl_hfence(
    shmem: *mut u8,
    control: c_ulong,
    page_num: c_ulong,
    page_count: c_ulong,
) {
    let mut i: c_int;
    let mut ent: c_int = -1;
    let mut try_count: c_int = 5;
    let mut entp: *mut __lelong;

    'again: loop {
        i = 0;
        while i < SBI_NACL_SHMEM_HFENCE_ENTRY_MAX {
            entp = shmem.add(SBI_NACL_SHMEM_HFENCE_ENTRY_CONFIG(i) as usize)
                as *mut __lelong;
            if lelong_to_cpu(*entp) & SBI_NACL_SHMEM_HFENCE_CONFIG_PEND != 0 {
                i += 1;
                continue;
            }

            ent = i;
            break;
        }

        if ent < 0 {
            if try_count != 0 {
                try_count -= 1;
                nacl_sync_hfence(!0 as c_ulong);
                continue 'again;
            } else {
                pr_warn("KVM: No free entry in NACL shared memory\n");
                return;
            }
        }

        entp = shmem.add(SBI_NACL_SHMEM_HFENCE_ENTRY_CONFIG(i) as usize)
            as *mut __lelong;
        *entp = cpu_to_lelong(control);
        entp = shmem.add(SBI_NACL_SHMEM_HFENCE_ENTRY_PNUM(i) as usize)
            as *mut __lelong;
        *entp = cpu_to_lelong(page_num);
        entp = shmem.add(SBI_NACL_SHMEM_HFENCE_ENTRY_PCOUNT(i) as usize)
            as *mut __lelong;
        *entp = cpu_to_lelong(page_count);
        return;
    }
}

pub unsafe fn kvm_riscv_nacl_enable() -> c_int {
    let rc: c_int;
    let ret: sbiret;
    let nacl: *mut kvm_riscv_nacl;

    if !kvm_riscv_nacl_available() {
        return 0;
    }
    nacl = this_cpu_ptr(&raw mut kvm_riscv_nacl);

    ret = sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_SET_SHMEM,
                    (*nacl).shmem_phys, 0, 0, 0, 0, 0);
    rc = sbi_err_map_linux_errno(ret.error);
    if rc != 0 {
        return rc;
    }

    0
}

pub unsafe fn kvm_riscv_nacl_disable() {
    if !kvm_riscv_nacl_available() {
        return;
    }

    sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_SET_SHMEM,
              SBI_SHMEM_DISABLE, SBI_SHMEM_DISABLE, 0, 0, 0, 0);
}

pub unsafe fn kvm_riscv_nacl_exit() {
    let mut cpu: c_int;
    let nacl: *mut kvm_riscv_nacl;

    if !kvm_riscv_nacl_available() {
        return;
    }

    /* Allocate per-CPU shared memory */
    for_each_possible_cpu!(cpu) {
        nacl = per_cpu_ptr(&raw mut kvm_riscv_nacl, cpu);
        if (*nacl).shmem.is_null() {
            continue;
        }

        free_pages((*nacl).shmem as c_ulong,
                   get_order(SBI_NACL_SHMEM_SIZE));
        (*nacl).shmem = core::ptr::null_mut();
        (*nacl).shmem_phys = 0;
    }
}

unsafe fn nacl_probe_feature(feature_id: c_long) -> c_long {
    let ret: sbiret;

    if !kvm_riscv_nacl_available() {
        return 0;
    }

    ret = sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_PROBE_FEATURE,
                    feature_id, 0, 0, 0, 0, 0);
    ret.value
}

pub unsafe fn kvm_riscv_nacl_init() -> c_int {
    let mut cpu: c_int;
    let mut shmem_page: *mut page;
    let nacl: *mut kvm_riscv_nacl;

    if sbi_spec_version < sbi_mk_version(1, 0)
        || sbi_probe_extension(SBI_EXT_NACL) <= 0
    {
        return -ENODEV;
    }

    /* Enable NACL support */
    static_branch_enable(&raw mut kvm_riscv_nacl_available);

    /* Probe NACL features */
    if nacl_probe_feature(SBI_NACL_FEAT_SYNC_CSR) != 0 {
        static_branch_enable(&raw mut kvm_riscv_nacl_sync_csr_available);
    }
    if nacl_probe_feature(SBI_NACL_FEAT_SYNC_HFENCE) != 0 {
        static_branch_enable(&raw mut kvm_riscv_nacl_sync_hfence_available);
    }
    if nacl_probe_feature(SBI_NACL_FEAT_SYNC_SRET) != 0 {
        static_branch_enable(&raw mut kvm_riscv_nacl_sync_sret_available);
    }
    if nacl_probe_feature(SBI_NACL_FEAT_AUTOSWAP_CSR) != 0 {
        static_branch_enable(&raw mut kvm_riscv_nacl_autoswap_csr_available);
    }

    /* Allocate per-CPU shared memory */
    for_each_possible_cpu!(cpu) {
        nacl = per_cpu_ptr(&raw mut kvm_riscv_nacl, cpu);

        shmem_page = alloc_pages(GFP_KERNEL | __GFP_ZERO,
                                  get_order(SBI_NACL_SHMEM_SIZE));
        if shmem_page.is_null() {
            kvm_riscv_nacl_exit();
            return -ENOMEM;
        }
        (*nacl).shmem = page_to_virt(shmem_page);
        (*nacl).shmem_phys = page_to_phys(shmem_page);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
