// SPDX-License-Identifier: GPL-2.0-only
/*
 * Debug helper used to dump the stage-2 pagetables of the system and their
 * associated permissions.
 *
 * Copyright (C) Google, 2024
 * Author: Sebastian Ene <sebastianene@google.com>
 */

const MARKERS_LEN: usize = 2;
const KVM_PGTABLE_MAX_LEVELS: usize = KVM_PGTABLE_LAST_LEVEL as usize + 1;
const S2FNAMESZ: usize = core::mem::size_of::<[u8; 53]>();

#[repr(C)]
struct KvmPtdumpGuestState {
    mmu: *mut kvm_s2_mmu,
    parser_state: ptdump_pg_state,
    ipa_marker: [addr_marker; MARKERS_LEN],
    level: [ptdump_pg_level; KVM_PGTABLE_MAX_LEVELS],
}

static STAGE2_PTE_BITS: [ptdump_prot_bits; 9] = [
    ptdump_prot_bits { mask: PTE_VALID, val: PTE_VALID, set: " ", clear: "F" },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_LO_S2_S2AP_R, val: KVM_PTE_LEAF_ATTR_LO_S2_S2AP_R, set: "R", clear: " " },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_LO_S2_S2AP_W, val: KVM_PTE_LEAF_ATTR_LO_S2_S2AP_W, set: "W", clear: " " },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_HI_S2_XN, val: 0b00u64 << __bf_shf(KVM_PTE_LEAF_ATTR_HI_S2_XN), set: "px ux ", clear: "" },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_HI_S2_XN, val: 0b01u64 << __bf_shf(KVM_PTE_LEAF_ATTR_HI_S2_XN), set: "PXNux ", clear: "" },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_HI_S2_XN, val: 0b10u64 << __bf_shf(KVM_PTE_LEAF_ATTR_HI_S2_XN), set: "PXNUXN", clear: "" },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_HI_S2_XN, val: 0b11u64 << __bf_shf(KVM_PTE_LEAF_ATTR_HI_S2_XN), set: "px UXN", clear: "" },
    ptdump_prot_bits { mask: KVM_PTE_LEAF_ATTR_LO_S2_AF, val: KVM_PTE_LEAF_ATTR_LO_S2_AF, set: "AF", clear: "  " },
    ptdump_prot_bits { mask: PMD_TYPE_MASK, val: PMD_TYPE_SECT, set: "BLK", clear: "   " },
];

unsafe fn kvm_ptdump_visitor(ctx: *const kvm_pgtable_visit_ctx, _visit: kvm_pgtable_walk_flags) -> i32 {
    let st = (*ctx).arg as *mut ptdump_pg_state;
    note_page(&mut (*st).ptdump, (*ctx).addr, (*ctx).level, (*ctx).old);
    0
}

unsafe fn kvm_ptdump_build_levels(level: *mut ptdump_pg_level, start_lvl: u32) -> i32 {
    if start_lvl >= KVM_PGTABLE_LAST_LEVEL { return -EINVAL; }
    let mut mask: u64 = 0;
    for bit in STAGE2_PTE_BITS.iter() { mask |= bit.mask; }
    for i in start_lvl as usize..KVM_PGTABLE_MAX_LEVELS {
        // snprintf(level[i].name, sizeof(level[i].name), "%u", i);
        (*level.add(i)).num = STAGE2_PTE_BITS.len();
        (*level.add(i)).bits = STAGE2_PTE_BITS.as_ptr();
        (*level.add(i)).mask = mask;
    }
    0
}

unsafe fn kvm_ptdump_parser_create(mmu: *mut kvm_s2_mmu) -> *mut KvmPtdumpGuestState {
    let st = kzalloc_obj::<KvmPtdumpGuestState>(GFP_KERNEL_ACCOUNT);
    if st.is_null() { return ERR_PTR(-ENOMEM); }
    let ret = kvm_ptdump_build_levels((*st).level.as_mut_ptr(), (*(*mmu).pgt).start_level);
    if ret != 0 { kfree(st); return ERR_PTR(ret); }
    (*st).ipa_marker[0].name = "Guest IPA";
    (*st).ipa_marker[1].start_address = ULONG_MAX;
    (*st).mmu = mmu;
    st
}

unsafe fn kvm_ptdump_guest_show(m: *mut seq_file, _unused: *mut core::ffi::c_void) -> i32 {
    let st = (*m).private as *mut KvmPtdumpGuestState;
    let mmu = (*st).mmu;
    let kvm = kvm_s2_mmu_to_kvm(mmu);
    let walker = kvm_pgtable_walker { cb: Some(kvm_ptdump_visitor), arg: &mut (*st).parser_state as *mut _ as *mut _, flags: KVM_PGTABLE_WALK_LEAF };
    guard_write_lock(&mut (*kvm).mmu_lock);
    (*st).parser_state = ptdump_pg_state { marker: (*st).ipa_marker.as_mut_ptr(), end_address: 1u64 << (*(*mmu).pgt).ia_bits, level: -1, pg_level: (*st).level.as_mut_ptr(), seq: m };
    let ret = kvm_pgtable_walk((*mmu).pgt, 0, 1u64 << (*(*mmu).pgt).ia_bits, &walker);
    if ret != 0 { return ret; }
    note_page_flush(&mut (*st).parser_state.ptdump);
    0
}

unsafe fn kvm_ptdump_guest_open(m: *mut inode, file: *mut file) -> i32 {
    let mmu = (*m).i_private as *mut kvm_s2_mmu;
    let kvm = kvm_s2_mmu_to_kvm(mmu);
    if !kvm_get_kvm_safe(kvm) { return -ENOENT; }
    let st = kvm_ptdump_parser_create(mmu);
    if IS_ERR(st) { let ret = PTR_ERR(st); kvm_put_kvm(kvm); return ret; }
    let ret = single_open(file, Some(kvm_ptdump_guest_show), st as *mut _);
    if ret == 0 { return 0; }
    kfree(st); kvm_put_kvm(kvm); ret
}

unsafe fn kvm_ptdump_guest_close(m: *mut inode, file: *mut file) -> i32 {
    let kvm = kvm_s2_mmu_to_kvm((*m).i_private as *mut kvm_s2_mmu);
    let st = (*(file).private_data as *mut seq_file).as_ref().unwrap().private as *mut _;
    kfree(st); kvm_put_kvm(kvm); single_release(m, file)
}

static KVM_PTDUMP_GUEST_FOPS: file_operations = file_operations { open: Some(kvm_ptdump_guest_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(kvm_ptdump_guest_close) };

unsafe fn kvm_pgtable_range_show(m: *mut seq_file, _unused: *mut core::ffi::c_void) -> i32 { seq_printf(m, "%2u\n", (*((*m).private as *mut kvm_pgtable)).ia_bits); 0 }
unsafe fn kvm_pgtable_levels_show(m: *mut seq_file, _unused: *mut core::ffi::c_void) -> i32 { seq_printf(m, "%1d\n", KVM_PGTABLE_MAX_LEVELS as i32 - (*((*m).private as *mut kvm_pgtable)).start_level as i32); 0 }

// The remaining debugfs entry points retain the C kernel ABI and are declared
// with the corresponding external kernel types and helpers.
unsafe fn kvm_pgtable_debugfs_open(m: *mut inode, file: *mut file, show: unsafe fn(*mut seq_file, *mut core::ffi::c_void) -> i32) -> i32 { let mmu = (*m).i_private as *mut kvm_s2_mmu; let kvm = kvm_s2_mmu_to_kvm(mmu); if !kvm_get_kvm_safe(kvm) { return -ENOENT; } let ret = single_open(file, Some(show), (*mmu).pgt as *mut _); if ret < 0 { kvm_put_kvm(kvm); } ret }
unsafe fn kvm_pgtable_range_open(m: *mut inode, f: *mut file) -> i32 { kvm_pgtable_debugfs_open(m, f, kvm_pgtable_range_show) }
unsafe fn kvm_pgtable_levels_open(m: *mut inode, f: *mut file) -> i32 { kvm_pgtable_debugfs_open(m, f, kvm_pgtable_levels_show) }
unsafe fn kvm_pgtable_debugfs_close(m: *mut inode, f: *mut file) -> i32 { kvm_put_kvm(kvm_s2_mmu_to_kvm((*m).i_private as *mut kvm_s2_mmu)); single_release(m, f) }

static KVM_PGTABLE_RANGE_FOPS: file_operations = file_operations { open: Some(kvm_pgtable_range_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(kvm_pgtable_debugfs_close) };
static KVM_PGTABLE_LEVELS_FOPS: file_operations = file_operations { open: Some(kvm_pgtable_levels_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(kvm_pgtable_debugfs_close) };

pub unsafe fn kvm_nested_s2_ptdump_create_debugfs(mmu: *mut kvm_s2_mmu) { let mut file_name = [0u8; S2FNAMESZ]; snprintf(file_name.as_mut_ptr(), S2FNAMESZ, "0x%016llx-0x%016llx-s2-%sabled", (*mmu).tlb_vttbr, (*mmu).tlb_vtcr, if (*mmu).nested_stage2_enabled { "en" } else { "dis" }); (*mmu).shadow_pt_debugfs_dentry = debugfs_create_file(file_name.as_ptr(), 0o400, (*(*mmu).arch).debugfs_nv_dentry, mmu as *mut _, &KVM_PTDUMP_GUEST_FOPS); }
pub unsafe fn kvm_nested_s2_ptdump_remove_debugfs(mmu: *mut kvm_s2_mmu) { debugfs_remove((*mmu).shadow_pt_debugfs_dentry); }
pub unsafe fn kvm_s2_ptdump_create_debugfs(kvm: *mut kvm) { debugfs_create_file("stage2_page_tables", 0o400, (*kvm).debugfs_dentry, &mut (*kvm).arch.mmu as *mut _, &KVM_PTDUMP_GUEST_FOPS); debugfs_create_file("ipa_range", 0o400, (*kvm).debugfs_dentry, &mut (*kvm).arch.mmu as *mut _, &KVM_PGTABLE_RANGE_FOPS); debugfs_create_file("stage2_levels", 0o400, (*kvm).debugfs_dentry, &mut (*kvm).arch.mmu as *mut _, &KVM_PGTABLE_LEVELS_FOPS); if cpus_have_final_cap(ARM64_HAS_NESTED_VIRT) { (*kvm).arch.debugfs_nv_dentry = debugfs_create_dir("nested", (*kvm).debugfs_dentry); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
