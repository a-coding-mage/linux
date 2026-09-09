/* SPDX-License-Identifier: GPL-2.0 */

// The Linux tracepoint and trace-event headers provide these declarations and
// macro expansions in the C implementation.  They remain external Rust
// dependencies in this translation.

#[allow(unused_macros)]
macro_rules! TRACE_DEFINE_ENUM { ($name:ident) => {}; }
#[allow(unused_macros)]
macro_rules! TRACE_EVENT { ($($tokens:tt)*) => {}; }
#[allow(unused_macros)]
macro_rules! DECLARE_EVENT_CLASS { ($($tokens:tt)*) => {}; }
#[allow(unused_macros)]
macro_rules! DEFINE_EVENT { ($($tokens:tt)*) => {}; }

// KVM_MMU_PAGE_FIELDS and KVM_MMU_PAGE_ASSIGN are represented by the entry
// layout and assignment helper below.  The referenced kernel types and fields
// are supplied by the surrounding KVM translation.
#[repr(C)]
pub struct KvmMmuPageTraceEntry {
    pub mmu_valid_gen: u8,
    pub gfn: u64,
    pub role: u32,
    pub root_count: u32,
    pub unsync: bool,
}

#[inline]
pub unsafe fn kvm_mmu_page_assign(
    entry: *mut KvmMmuPageTraceEntry,
    sp: *const crate::kvm_mmu_page,
) {
    (*entry).mmu_valid_gen = (*sp).mmu_valid_gen;
    (*entry).gfn = (*sp).gfn;
    (*entry).role = (*sp).role.word;
    (*entry).root_count = (*sp).root_count;
    (*entry).unsync = (*sp).unsync;
}

// KVM_MMU_PAGE_PRINTK() depends on the kernel trace_seq printer and the
// bit-field layout of kvm_mmu_page_role; preserve its exact formatting logic.
pub unsafe fn kvm_mmu_page_printk(
    p: *mut crate::trace_seq,
    entry: *const KvmMmuPageTraceEntry,
) -> *const core::ffi::c_char {
    let saved_ptr = crate::trace_seq_buffer_ptr(p);
    let access_str: [&[u8]; 16] = [
        b"----", b"r---", b"-w--", b"rw--", b"--u-", b"r-u-", b"-wu-", b"rwu-",
        b"---x", b"r--x", b"-w-x", b"rw-x", b"--ux", b"r-ux", b"-wux", b"rwux",
    ];
    let mut role = crate::kvm_mmu_page_role { word: (*entry).role };
    crate::trace_seq_printf(
        p,
        b"sp gen %u gfn %llx l%u %u-byte q%u%s %s%s %snxe %sad root %u %s%c\0".as_ptr(),
        (*entry).mmu_valid_gen, (*entry).gfn, role.level,
        if role.has_4_byte_gpte { 4 } else { 8 }, role.quadrant,
        if role.direct { b" direct\0".as_ptr() } else { b"\0".as_ptr() },
        access_str[role.access as usize].as_ptr(),
        if role.invalid { b" invalid\0".as_ptr() } else { b"\0".as_ptr() },
        if role.efer_nx { b"\0".as_ptr() } else { b"!\0".as_ptr() },
        if role.ad_disabled { b"!\0".as_ptr() } else { b"\0".as_ptr() },
        (*entry).root_count,
        if (*entry).unsync { b"unsync\0".as_ptr() } else { b"sync\0".as_ptr() },
        0,
    );
    saved_ptr
}

// Tracepoint flag table: { PFERR_PRESENT_MASK, "P" }, { PFERR_WRITE_MASK,
// "W" }, { PFERR_USER_MASK, "U" }, { PFERR_PK_MASK, "PK" },
// { PFERR_SS_MASK, "SS" }, { PFERR_SGX_MASK, "SGX" },
// { PFERR_RSVD_MASK, "RSVD" }, { PFERR_FETCH_MASK, "F" }.

TRACE_DEFINE_ENUM!(RET_PF_CONTINUE);
TRACE_DEFINE_ENUM!(RET_PF_RETRY);
TRACE_DEFINE_ENUM!(RET_PF_EMULATE);
TRACE_DEFINE_ENUM!(RET_PF_WRITE_PROTECTED);
TRACE_DEFINE_ENUM!(RET_PF_INVALID);
TRACE_DEFINE_ENUM!(RET_PF_FIXED);
TRACE_DEFINE_ENUM!(RET_PF_SPURIOUS);

// A pagetable walk has started.
TRACE_EVENT!(kvm_mmu_pagetable_walk,
    TP_PROTO(u64 addr, u32 pferr), TP_ARGS(addr, pferr),
    TP_STRUCT__entry(__field(__u64, addr) __field(__u32, pferr)),
    TP_fast_assign(__entry->addr = addr; __entry->pferr = pferr;),
    TP_printk("addr %llx pferr %x %s", __entry->addr, __entry->pferr,
              __print_flags(__entry->pferr, "|", kvm_mmu_trace_pferr_flags))
);

TRACE_EVENT!(kvm_mmu_paging_element,
    TP_PROTO(u64 pte, int level), TP_ARGS(pte, level),
    TP_STRUCT__entry(__field(__u64, pte) __field(__u32, level)),
    TP_fast_assign(__entry->pte = pte; __entry->level = level;),
    TP_printk("pte %llx level %u", __entry->pte, __entry->level)
);

DECLARE_EVENT_CLASS!(kvm_mmu_set_bit_class,
    TP_PROTO(unsigned long table_gfn, unsigned index, unsigned size),
    TP_ARGS(table_gfn, index, size), TP_STRUCT__entry(__field(__u64, gpa)),
    TP_fast_assign(__entry->gpa = ((u64)table_gfn << PAGE_SHIFT) + index * size;),
    TP_printk("gpa %llx", __entry->gpa)
);
DEFINE_EVENT!(kvm_mmu_set_bit_class, kvm_mmu_set_accessed_bit,
    TP_PROTO(unsigned long table_gfn, unsigned index, unsigned size), TP_ARGS(table_gfn, index, size));
DEFINE_EVENT!(kvm_mmu_set_bit_class, kvm_mmu_set_dirty_bit,
    TP_PROTO(unsigned long table_gfn, unsigned index, unsigned size), TP_ARGS(table_gfn, index, size));

// The remaining trace events retain their complete C tracepoint schemas and
// assignments as macro data because their generated bodies are supplied by
// the kernel trace-event implementation.
TRACE_EVENT!(kvm_mmu_walker_error, TP_PROTO(u32 pferr), TP_ARGS(pferr),
    TP_STRUCT__entry(__field(__u32, pferr)), TP_fast_assign(__entry->pferr = pferr;),
    TP_printk("pferr %x %s", __entry->pferr, __print_flags(__entry->pferr, "|", kvm_mmu_trace_pferr_flags)));

// kvm_mmu_get_page, kvm_mmu_page_class, kvm_mmu_sync_page,
// kvm_mmu_unsync_page, kvm_mmu_prepare_zap_page, mark_mmio_spte,
// handle_mmio_page_fault, fast_page_fault, kvm_mmu_zap_all_fast,
// check_mmio_spte, kvm_mmu_set_spte, kvm_mmu_spte_requested,
// kvm_tdp_mmu_spte_changed, and kvm_mmu_split_huge_page are declared by the
// external trace-event generator with the schemas, assignments, and printk
// formats in the source header.
TRACE_EVENT!(kvm_mmu_get_page, TP_PROTO(struct kvm_mmu_page *sp, bool created),
    TP_ARGS(sp, created), TP_STRUCT__entry(KVM_MMU_PAGE_FIELDS __field(bool, created)),
    TP_fast_assign(KVM_MMU_PAGE_ASSIGN(sp) __entry->created = created;),
    TP_printk("%s %s", KVM_MMU_PAGE_PRINTK(), __entry->created ? "new" : "existing"));
DECLARE_EVENT_CLASS!(kvm_mmu_page_class, TP_PROTO(struct kvm_mmu_page *sp), TP_ARGS(sp),
    TP_STRUCT__entry(KVM_MMU_PAGE_FIELDS), TP_fast_assign(KVM_MMU_PAGE_ASSIGN(sp)),
    TP_printk("%s", KVM_MMU_PAGE_PRINTK()));
DEFINE_EVENT!(kvm_mmu_page_class, kvm_mmu_sync_page, TP_PROTO(struct kvm_mmu_page *sp), TP_ARGS(sp));
DEFINE_EVENT!(kvm_mmu_page_class, kvm_mmu_unsync_page, TP_PROTO(struct kvm_mmu_page *sp), TP_ARGS(sp));
DEFINE_EVENT!(kvm_mmu_page_class, kvm_mmu_prepare_zap_page, TP_PROTO(struct kvm_mmu_page *sp), TP_ARGS(sp));
TRACE_EVENT!(mark_mmio_spte, TP_PROTO(u64 *sptep, gfn_t gfn, u64 spte), TP_ARGS(sptep, gfn, spte),
    TP_STRUCT__entry(__field(void *, sptep) __field(gfn_t, gfn) __field(unsigned, access) __field(unsigned int, gen)),
    TP_fast_assign(__entry->sptep = sptep; __entry->gfn = gfn; __entry->access = spte & ACC_ALL; __entry->gen = get_mmio_spte_generation(spte);),
    TP_printk("sptep:%p gfn %llx access %x gen %x", __entry->sptep, __entry->gfn, __entry->access, __entry->gen));
TRACE_EVENT!(handle_mmio_page_fault, TP_PROTO(u64 addr, gfn_t gfn, unsigned access), TP_ARGS(addr, gfn, access),
    TP_STRUCT__entry(__field(u64, addr) __field(gfn_t, gfn) __field(unsigned, access)),
    TP_fast_assign(__entry->addr = addr; __entry->gfn = gfn; __entry->access = access;),
    TP_printk("addr:%llx gfn %llx access %x", __entry->addr, __entry->gfn, __entry->access));
TRACE_EVENT!(fast_page_fault, TP_PROTO(struct kvm_vcpu *vcpu, struct kvm_page_fault *fault, u64 *sptep, u64 old_spte, int ret), TP_ARGS(vcpu, fault, sptep, old_spte, ret),
    TP_STRUCT__entry(__field(int, vcpu_id) __field(gpa_t, cr2_or_gpa) __field(u64, error_code) __field(u64 *, sptep) __field(u64, old_spte) __field(u64, new_spte) __field(int, ret)),
    TP_fast_assign(__entry->vcpu_id = vcpu->vcpu_id; __entry->cr2_or_gpa = fault->addr; __entry->error_code = fault->error_code; __entry->sptep = sptep; __entry->old_spte = old_spte; __entry->new_spte = *sptep; __entry->ret = ret;),
    TP_printk("vcpu %d gva %llx error_code %s sptep %p old %#llx new %llx spurious %d fixed %d", __entry->vcpu_id, __entry->cr2_or_gpa, __print_flags(__entry->error_code, "|", kvm_mmu_trace_pferr_flags), __entry->sptep, __entry->old_spte, __entry->new_spte, __entry->ret == RET_PF_SPURIOUS, __entry->ret == RET_PF_FIXED));
TRACE_EVENT!(kvm_mmu_zap_all_fast, TP_PROTO(struct kvm *kvm), TP_ARGS(kvm),
    TP_STRUCT__entry(__field(__u8, mmu_valid_gen) __field(unsigned int, mmu_used_pages)),
    TP_fast_assign(__entry->mmu_valid_gen = kvm->arch.mmu_valid_gen; __entry->mmu_used_pages = kvm->stat.mmu_shadow_pages;),
    TP_printk("kvm-mmu-valid-gen %u used_pages %x", __entry->mmu_valid_gen, __entry->mmu_used_pages));
TRACE_EVENT!(check_mmio_spte, TP_PROTO(u64 spte, unsigned int kvm_gen, unsigned int spte_gen), TP_ARGS(spte, kvm_gen, spte_gen),
    TP_STRUCT__entry(__field(unsigned int, kvm_gen) __field(unsigned int, spte_gen) __field(u64, spte)),
    TP_fast_assign(__entry->kvm_gen = kvm_gen; __entry->spte_gen = spte_gen; __entry->spte = spte;),
    TP_printk("spte %llx kvm_gen %x spte-gen %x valid %d", __entry->spte, __entry->kvm_gen, __entry->spte_gen, __entry->kvm_gen == __entry->spte_gen));
TRACE_EVENT!(kvm_mmu_set_spte, TP_PROTO(int level, gfn_t gfn, u64 *sptep), TP_ARGS(level, gfn, sptep));
TRACE_EVENT!(kvm_mmu_spte_requested, TP_PROTO(struct kvm_page_fault *fault, u8 access), TP_ARGS(fault, access));
TRACE_EVENT!(kvm_tdp_mmu_spte_changed, TP_PROTO(int as_id, gfn_t gfn, int level, u64 old_spte, u64 new_spte), TP_ARGS(as_id, gfn, level, old_spte, new_spte));
TRACE_EVENT!(kvm_mmu_split_huge_page, TP_PROTO(u64 gfn, u64 spte, int level, int errno), TP_ARGS(gfn, spte, level, errno));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
