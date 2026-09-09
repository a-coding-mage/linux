// SPDX-License-Identifier: GPL-2.0+
/*
 * PowerPC Memory Protection Keys management
 *
 * Copyright 2017, Ram Pai, IBM Corporation.
 */

pub static mut num_pkey: i32 = 0;
pub static mut reserved_allocation_mask: u32 = 0;
static mut initial_allocation_mask: u32 = 0;
pub static mut default_amr: u64 = !0u64;
pub static mut default_iamr: u64 = 0x5555_5555_5555_5555u64;
pub static mut default_uamor: u64 = 0;
static mut execute_only_key: i32 = 2;
static mut pkey_execute_disable_supported: bool = false;

const AMR_BITS_PER_PKEY: i32 = 2;
const AMR_RD_BIT: u64 = 0x1;
const AMR_WR_BIT: u64 = 0x2;
const IAMR_EX_BIT: u64 = 0x1;
const PKEY_REG_BITS: i32 = (core::mem::size_of::<u64>() * 8) as i32;

#[inline]
unsafe fn pkeyshift(pkey: i32) -> i32 {
    PKEY_REG_BITS - ((pkey + 1) * AMR_BITS_PER_PKEY)
}

// External kernel declarations and configuration-provided symbols are intentionally unresolved here.

#[cfg_attr(not(CONFIG_PPC_MEM_KEYS), allow(dead_code))]
unsafe fn dt_scan_storage_keys(node: u64, _uname: *const i8, _depth: i32, data: *mut core::ffi::c_void) -> i32 {
    let type_ = of_get_flat_dt_prop(node, b"device_type\0".as_ptr() as *const i8, core::ptr::null_mut());
    let pkeys_total = data as *mut i32;
    if type_.is_null() || strcmp(type_, b"cpu\0".as_ptr() as *const i8) != 0 { return 0; }
    let prop = of_get_flat_dt_prop(node, b"ibm,processor-storage-keys\0".as_ptr() as *const i8, core::ptr::null_mut());
    if prop.is_null() { return 0; }
    *pkeys_total = be32_to_cpu(*prop);
    1
}

unsafe fn scan_pkey_feature() -> i32 {
    let mut pkeys_total = 0;
    if early_radix_enabled() { return 0; }
    let ret = of_scan_flat_dt(dt_scan_storage_keys, &mut pkeys_total as *mut i32 as *mut core::ffi::c_void);
    if ret == 0 && !firmware_has_feature(FW_FEATURE_LPAR) {
        let pvr = mfspr(SPRN_PVR);
        let ver = PVR_VER(pvr);
        if ver == PVR_POWER8 || ver == PVR_POWER8E || ver == PVR_POWER8NVL || ver == PVR_POWER9 || ver == PVR_HX_C2000 { pkeys_total = 32; }
    }
    // CONFIG_PPC_MEM_KEYS: the architecture-neutral limit is applied here.
    #[cfg(CONFIG_PPC_MEM_KEYS)]
    { pkeys_total = core::cmp::min(pkeys_total, ((ARCH_VM_PKEY_FLAGS >> VM_PKEY_SHIFT) + 1) as i32); }
    pkeys_total
}

pub unsafe fn pkey_early_init_devtree() {
    // CONFIG_PPC_MEM_KEYS contains the corresponding BUILD_BUG_ON invariants.
    if !early_cpu_has_feature(CPU_FTR_ARCH_206) { return; }
    let pkeys_total = scan_pkey_feature();
    if pkeys_total == 0 { mtspr(SPRN_UAMOR, default_uamor); return; }
    default_uamor = !0u64;
    (*cur_cpu_spec).mmu_features |= MMU_FTR_PKEY;
    pkey_execute_disable_supported = !(pvr_version_is(PVR_POWER7) || pvr_version_is(PVR_POWER7p));
    #[cfg(CONFIG_PPC_4K_PAGES)]
    { num_pkey = core::cmp::min(8, pkeys_total); }
    #[cfg(not(CONFIG_PPC_4K_PAGES))]
    { num_pkey = pkeys_total; }
    if num_pkey <= execute_only_key || !pkey_execute_disable_supported {
        execute_only_key = -1;
    } else {
        reserved_allocation_mask |= 1u32 << execute_only_key;
        default_amr |= 0x3u64 << pkeyshift(execute_only_key);
        default_iamr &= !(1u64 << pkeyshift(execute_only_key));
        default_uamor &= !(0x3u64 << pkeyshift(execute_only_key));
    }
    if num_pkey <= 3 {
        disable_kuep = true; disable_kuap = true;
        WARN(1, "Disabling kernel user protection due to low (%d) max supported keys\n", num_pkey);
    } else {
        reserved_allocation_mask |= 1u32 << 3;
        default_amr &= !(0x3u64 << pkeyshift(3));
        default_iamr &= !(1u64 << pkeyshift(3));
        default_uamor &= !(0x3u64 << pkeyshift(3));
    }
    default_amr &= !(0x3u64 << pkeyshift(0));
    default_iamr &= !(1u64 << pkeyshift(0));
    default_uamor &= !(0x3u64 << pkeyshift(0));
    initial_allocation_mask |= 1;
    reserved_allocation_mask |= 1u32 << 1;
    default_uamor &= !(0x3u64 << pkeyshift(1));
    for i in num_pkey..32 { reserved_allocation_mask |= 1u32 << i; default_uamor &= !(0x3u64 << pkeyshift(i)); }
    initial_allocation_mask |= reserved_allocation_mask;
    pr_info("Enabling pkeys with max key count %d\n", num_pkey);
    mtspr(SPRN_UAMOR, default_uamor);
}

#[cfg(CONFIG_PPC_KUEP)]
pub unsafe fn setup_kuep(disabled: bool) {
    if disabled || (!early_radix_enabled() && !early_mmu_has_feature(MMU_FTR_PKEY)) { return; }
    if smp_processor_id() == boot_cpuid { pr_info("Activating Kernel Userspace Execution Prevention\n"); (*cur_cpu_spec).mmu_features |= MMU_FTR_BOOK3S_KUEP; }
    mtspr(SPRN_IAMR, AMR_KUEP_BLOCKED); isync();
}

#[cfg(CONFIG_PPC_KUAP)]
pub unsafe fn setup_kuap(disabled: bool) {
    if disabled || (!early_radix_enabled() && !early_mmu_has_feature(MMU_FTR_PKEY)) { return; }
    if smp_processor_id() == boot_cpuid { pr_info("Activating Kernel Userspace Access Prevention\n"); (*cur_cpu_spec).mmu_features |= MMU_FTR_KUAP; }
    mtspr(SPRN_AMR, AMR_KUAP_BLOCKED); isync();
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn pkey_mm_init(mm: *mut mm_struct) {
    if !mmu_has_feature(MMU_FTR_PKEY) { return; }
    mm_pkey_allocation_map(mm) = initial_allocation_mask;
    (*mm).context.execute_only_pkey = execute_only_key;
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
unsafe fn init_amr(pkey: i32, init_bits: u8) {
    let new_amr_bits = ((init_bits as u64 & 0x3) << pkeyshift(pkey));
    let old_amr = current_thread_amr() & !(0x3u64 << pkeyshift(pkey));
    (*(*current).thread.regs).amr = old_amr | new_amr_bits;
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
unsafe fn init_iamr(pkey: i32, init_bits: u8) {
    let new_iamr_bits = ((init_bits as u64 & 0x1) << pkeyshift(pkey));
    let old_iamr = current_thread_iamr() & !(0x1u64 << pkeyshift(pkey));
    if !pkey_execute_disable_supported { return; }
    (*(*current).thread.regs).iamr = old_iamr | new_iamr_bits;
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn __arch_set_user_pkey_access(pkey: i32, init_val: u64) -> i32 {
    let mut new_amr_bits = 0u64; let mut new_iamr_bits = 0u64;
    let pkey_bits = 0x3u64 << pkeyshift(pkey);
    if (default_uamor & pkey_bits) != pkey_bits { return -22; }
    if init_val & PKEY_DISABLE_EXECUTE != 0 { if !pkey_execute_disable_supported { return -22; } new_iamr_bits |= IAMR_EX_BIT; }
    init_iamr(pkey, new_iamr_bits as u8);
    if init_val & PKEY_DISABLE_ACCESS != 0 { new_amr_bits |= AMR_RD_BIT | AMR_WR_BIT; } else if init_val & PKEY_DISABLE_WRITE != 0 { new_amr_bits |= AMR_WR_BIT; }
    init_amr(pkey, new_amr_bits as u8); 0
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn execute_only_pkey(mm: *mut mm_struct) -> i32 { (*mm).context.execute_only_pkey }

#[cfg(CONFIG_PPC_MEM_KEYS)]
unsafe fn vma_is_pkey_exec_only(vma: *mut vm_area_struct) -> bool {
    if ((*vma).vm_flags & VM_ACCESS_FLAGS) != VM_EXEC { return false; }
    vma_pkey(vma) == (*(*vma).vm_mm).context.execute_only_pkey
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn __arch_override_mprotect_pkey(vma: *mut vm_area_struct, prot: i32, mut pkey: i32) -> i32 {
    if vma_is_pkey_exec_only(vma) && prot != PROT_EXEC { return 0; }
    if prot == PROT_EXEC { pkey = execute_only_pkey((*vma).vm_mm); if pkey > 0 { return pkey; } }
    vma_pkey(vma)
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
unsafe fn pkey_access_permitted(pkey: i32, write: bool, execute: bool) -> bool {
    let shift = pkeyshift(pkey);
    if execute { return (current_thread_iamr() & (IAMR_EX_BIT << shift)) == 0; }
    let amr = current_thread_amr();
    if write { return (amr & (AMR_WR_BIT << shift)) == 0; }
    (amr & (AMR_RD_BIT << shift)) == 0
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn arch_pte_access_permitted(pte: u64, write: bool, execute: bool) -> bool {
    if !mmu_has_feature(MMU_FTR_PKEY) { return true; }
    pkey_access_permitted(pte_to_pkey_bits(pte), write, execute)
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn arch_vma_access_permitted(vma: *mut vm_area_struct, write: bool, execute: bool, foreign: bool) -> bool {
    if !mmu_has_feature(MMU_FTR_PKEY) || foreign || vma_is_foreign(vma) { return true; }
    pkey_access_permitted(vma_pkey(vma), write, execute)
}

#[cfg(CONFIG_PPC_MEM_KEYS)]
pub unsafe fn arch_dup_pkeys(oldmm: *mut mm_struct, mm: *mut mm_struct) {
    if !mmu_has_feature(MMU_FTR_PKEY) { return; }
    mm_pkey_allocation_map(mm) = mm_pkey_allocation_map(oldmm);
    (*mm).context.execute_only_pkey = (*oldmm).context.execute_only_pkey;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
