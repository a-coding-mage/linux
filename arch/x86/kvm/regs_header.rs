/* SPDX-License-Identifier: GPL-2.0 */

// Translated from x86/kvm/regs.h.  Symbols supplied by the surrounding kernel
// are intentionally left as external dependencies.

pub const KVM_POSSIBLE_CR0_GUEST_BITS: _ = X86_CR0_TS | X86_CR0_WP;
pub const KVM_POSSIBLE_CR4_GUEST_BITS: _ = X86_CR4_PVI | X86_CR4_DE | X86_CR4_PCE | X86_CR4_OSFXSR
    | X86_CR4_OSXMMEXCPT | X86_CR4_PGE | X86_CR4_TSD | X86_CR4_FSGSBASE | X86_CR4_CET;
pub const X86_CR0_PDPTR_BITS: _ = X86_CR0_CD | X86_CR0_NW | X86_CR0_PG;
pub const X86_CR4_TLBFLUSH_BITS: _ = X86_CR4_PGE | X86_CR4_PCIDE | X86_CR4_PAE | X86_CR4_SMEP;
pub const X86_CR4_PDPTR_BITS: _ = X86_CR4_PGE | X86_CR4_PSE | X86_CR4_PAE | X86_CR4_SMEP;

pub const CR0_RESERVED_BITS: _ = !(X86_CR0_PE | X86_CR0_MP | X86_CR0_EM | X86_CR0_TS | X86_CR0_ET
    | X86_CR0_NE | X86_CR0_WP | X86_CR0_AM | X86_CR0_NW | X86_CR0_CD | X86_CR0_PG);
pub const CR4_RESERVED_BITS: _ = !(X86_CR4_VME | X86_CR4_PVI | X86_CR4_TSD | X86_CR4_DE | X86_CR4_PSE
    | X86_CR4_PAE | X86_CR4_MCE | X86_CR4_PGE | X86_CR4_PCE | X86_CR4_OSFXSR | X86_CR4_PCIDE
    | X86_CR4_OSXSAVE | X86_CR4_SMEP | X86_CR4_FSGSBASE | X86_CR4_OSXMMEXCPT | X86_CR4_LA57
    | X86_CR4_VMXE | X86_CR4_SMAP | X86_CR4_PKE | X86_CR4_UMIP | X86_CR4_LAM_SUP | X86_CR4_CET);
pub const CR8_RESERVED_BITS: _ = !X86_CR8_TPR;

pub const DR6_BUS_LOCK: u32 = 1 << 11;
pub const DR6_BD: u32 = 1 << 13;
pub const DR6_BS: u32 = 1 << 14;
pub const DR6_BT: u32 = 1 << 15;
pub const DR6_RTM: u32 = 1 << 16;
pub const DR6_ACTIVE_LOW: u32 = 0xffff0ff0;
pub const DR6_VOLATILE: u32 = 0x0001e80f;
pub const DR6_FIXED_1: u32 = DR6_ACTIVE_LOW & !DR6_VOLATILE;
pub const DR7_BP_EN_MASK: u32 = 0x000000ff;
pub const DR7_GE: u32 = 1 << 9;
pub const DR7_GD: u32 = 1 << 13;
pub const DR7_VOLATILE: u32 = 0xffff2bff;

extern "C" {
    pub fn kvm_post_set_cr0(vcpu: *mut kvm_vcpu, old_cr0: ulong, cr0: ulong);
    pub fn kvm_post_set_cr4(vcpu: *mut kvm_vcpu, old_cr4: ulong, cr4: ulong);
    pub fn kvm_set_cr0(vcpu: *mut kvm_vcpu, cr0: ulong) -> c_int;
    pub fn kvm_set_cr3(vcpu: *mut kvm_vcpu, cr3: ulong) -> c_int;
    pub fn kvm_set_cr4(vcpu: *mut kvm_vcpu, cr4: ulong) -> c_int;
    pub fn kvm_set_cr8(vcpu: *mut kvm_vcpu, cr8: ulong) -> c_int;
    pub fn kvm_set_dr(vcpu: *mut kvm_vcpu, dr: c_int, val: ulong) -> c_int;
    pub fn kvm_get_dr(vcpu: *mut kvm_vcpu, dr: c_int) -> ulong;
    pub fn kvm_get_cr8(vcpu: *mut kvm_vcpu) -> ulong;
    pub fn kvm_lmsw(vcpu: *mut kvm_vcpu, msw: ulong);
    pub fn load_pdptrs(vcpu: *mut kvm_vcpu, cr3: ulong) -> c_int;
}

#[inline]
pub unsafe fn is_long_mode(vcpu: *mut kvm_vcpu) -> bool {
    #[cfg(target_pointer_width = "64")]
    { ((*vcpu).arch.efer & EFER_LMA) != 0 }
    #[cfg(not(target_pointer_width = "64"))]
    { false }
}

#[inline]
pub unsafe fn is_64_bit_mode(vcpu: *mut kvm_vcpu) -> bool {
    let mut cs_db = 0;
    let mut cs_l = 0;
    WARN_ON_ONCE((*vcpu).arch.guest_state_protected);
    if !is_long_mode(vcpu) { return false; }
    kvm_x86_call(get_cs_db_l_bits)(vcpu, &mut cs_db, &mut cs_l);
    cs_l != 0
}

#[inline]
pub unsafe fn is_64_bit_hypercall(vcpu: *mut kvm_vcpu) -> bool {
    #[cfg(target_pointer_width = "64")]
    { (*vcpu).arch.guest_state_protected || is_64_bit_mode(vcpu) }
    #[cfg(not(target_pointer_width = "64"))]
    { false }
}

#[inline(always)]
pub unsafe fn kvm_reg_mode_mask(vcpu: *mut kvm_vcpu) -> ulong {
    #[cfg(target_pointer_width = "64")]
    { if is_64_bit_mode(vcpu) { GENMASK(63, 0) } else { GENMASK(31, 0) } }
    #[cfg(not(target_pointer_width = "64"))]
    { GENMASK(31, 0) }
}

macro_rules! __build_kvm_gpr_accessors {
    ($read:ident, $read_raw:ident, $write_raw:ident, $idx:ident) => {
        #[inline(always)] pub unsafe fn $read(vcpu: *mut kvm_vcpu) -> ulong { (*vcpu).arch.regs[$idx] & kvm_reg_mode_mask(vcpu) }
        #[inline(always)] pub unsafe fn $read_raw(vcpu: *mut kvm_vcpu) -> ulong { (*vcpu).arch.regs[$idx] }
        #[inline(always)] pub unsafe fn $write_raw(vcpu: *mut kvm_vcpu, val: ulong) { (*vcpu).arch.regs[$idx] = val; }
    };
}

macro_rules! build_kvm_gpr_accessors {
    ($eread:ident, $ewrite:ident, $read:ident, $read_raw:ident, $write_raw:ident, $idx:ident) => {
        #[inline(always)] pub unsafe fn $eread(vcpu: *mut kvm_vcpu) -> u32 { (*vcpu).arch.regs[$idx] as u32 }
        #[inline(always)] pub unsafe fn $ewrite(vcpu: *mut kvm_vcpu, val: u32) { (*vcpu).arch.regs[$idx] = val as ulong; }
        __build_kvm_gpr_accessors!($read, $read_raw, $write_raw, $idx);
    };
}
build_kvm_gpr_accessors!(kvm_eax_read, kvm_eax_write, kvm_rax_read, kvm_rax_read_raw, kvm_rax_write_raw, VCPU_REGS_RAX);
build_kvm_gpr_accessors!(kvm_ebx_read, kvm_ebx_write, kvm_rbx_read, kvm_rbx_read_raw, kvm_rbx_write_raw, VCPU_REGS_RBX);
build_kvm_gpr_accessors!(kvm_ecx_read, kvm_ecx_write, kvm_rcx_read, kvm_rcx_read_raw, kvm_rcx_write_raw, VCPU_REGS_RCX);
build_kvm_gpr_accessors!(kvm_edx_read, kvm_edx_write, kvm_rdx_read, kvm_rdx_read_raw, kvm_rdx_write_raw, VCPU_REGS_RDX);
build_kvm_gpr_accessors!(kvm_ebp_read, kvm_ebp_write, kvm_rbp_read, kvm_rbp_read_raw, kvm_rbp_write_raw, VCPU_REGS_RBP);
build_kvm_gpr_accessors!(kvm_esi_read, kvm_esi_write, kvm_rsi_read, kvm_rsi_read_raw, kvm_rsi_write_raw, VCPU_REGS_RSI);
build_kvm_gpr_accessors!(kvm_edi_read, kvm_edi_write, kvm_rdi_read, kvm_rdi_read_raw, kvm_rdi_write_raw, VCPU_REGS_RDI);

pub unsafe fn kvm_register_is_available(vcpu: *mut kvm_vcpu, reg: kvm_reg) -> bool {
    kvm_assert_register_caching_allowed!(vcpu); test_bit(reg, (*vcpu).arch.regs_avail)
}
pub unsafe fn kvm_register_is_dirty(vcpu: *mut kvm_vcpu, reg: kvm_reg) -> bool {
    kvm_assert_register_caching_allowed!(vcpu); test_bit(reg, (*vcpu).arch.regs_dirty)
}
pub unsafe fn kvm_register_mark_for_reload(vcpu: *mut kvm_vcpu, reg: kvm_reg) {
    kvm_assert_register_caching_allowed!(vcpu); __clear_bit(reg, (*vcpu).arch.regs_avail); __clear_bit(reg, (*vcpu).arch.regs_dirty);
}
pub unsafe fn kvm_register_mark_available(vcpu: *mut kvm_vcpu, reg: kvm_reg) { kvm_assert_register_caching_allowed!(vcpu); __set_bit(reg, (*vcpu).arch.regs_avail); }
pub unsafe fn kvm_register_mark_dirty(vcpu: *mut kvm_vcpu, reg: kvm_reg) { kvm_assert_register_caching_allowed!(vcpu); __set_bit(reg, (*vcpu).arch.regs_avail); __set_bit(reg, (*vcpu).arch.regs_dirty); }

pub unsafe fn kvm_register_test_and_mark_available(vcpu: *mut kvm_vcpu, reg: kvm_reg) -> bool { kvm_assert_register_caching_allowed!(vcpu); arch___test_and_set_bit(reg, (*vcpu).arch.regs_avail) }
pub unsafe fn kvm_clear_available_registers(vcpu: *mut kvm_vcpu, clear_mask: ulong) { (*vcpu).arch.regs_avail[0] &= !clear_mask; }
pub unsafe fn kvm_reset_dirty_registers(vcpu: *mut kvm_vcpu) { (*vcpu).arch.regs_dirty[0] = 0; }

pub unsafe fn kvm_register_read_raw(vcpu: *mut kvm_vcpu, reg: c_int) -> ulong {
    if WARN_ON_ONCE(reg as c_uint >= NR_VCPU_GENERAL_PURPOSE_REGS) { return 0; }
    if !kvm_register_is_available(vcpu, reg) { kvm_x86_call(cache_reg)(vcpu, reg); }
    (*vcpu).arch.regs[reg as usize]
}
pub unsafe fn kvm_register_read(vcpu: *mut kvm_vcpu, reg: c_int) -> ulong { kvm_register_read_raw(vcpu, reg) & kvm_reg_mode_mask(vcpu) }
pub unsafe fn kvm_register_write_raw(vcpu: *mut kvm_vcpu, reg: c_int, val: ulong) { if !WARN_ON_ONCE(reg as c_uint >= NR_VCPU_GENERAL_PURPOSE_REGS) { (*vcpu).arch.regs[reg as usize] = val; kvm_register_mark_dirty(vcpu, reg); } }
pub unsafe fn kvm_register_write(vcpu: *mut kvm_vcpu, reg: c_int, val: ulong) { kvm_register_write_raw(vcpu, reg, val & kvm_reg_mode_mask(vcpu)); }
pub unsafe fn kvm_rip_read(vcpu: *mut kvm_vcpu) -> ulong { if !kvm_register_is_available(vcpu, VCPU_REG_RIP) { kvm_x86_call(cache_reg)(vcpu, VCPU_REG_RIP); } (*vcpu).arch.rip }
pub unsafe fn kvm_rip_write(vcpu: *mut kvm_vcpu, val: ulong) { (*vcpu).arch.rip = val; kvm_register_mark_dirty(vcpu, VCPU_REG_RIP); }
pub unsafe fn kvm_rsp_read(vcpu: *mut kvm_vcpu) -> ulong { kvm_register_read_raw(vcpu, VCPU_REGS_RSP) }
pub unsafe fn kvm_rsp_write(vcpu: *mut kvm_vcpu, val: ulong) { kvm_register_write_raw(vcpu, VCPU_REGS_RSP, val); }
pub unsafe fn kvm_read_edx_eax(vcpu: *mut kvm_vcpu) -> u64 { kvm_eax_read(vcpu) as u64 | ((kvm_edx_read(vcpu) as u64) << 32) }
pub unsafe fn kvm_pdptr_read(vcpu: *mut kvm_vcpu, index: c_int) -> u64 { might_sleep!(); if !kvm_register_is_available(vcpu, VCPU_REG_PDPTR) { kvm_x86_call(cache_reg)(vcpu, VCPU_REG_PDPTR); } (*vcpu).arch.pdptrs[index as usize] }
pub unsafe fn kvm_pdptr_write(vcpu: *mut kvm_vcpu, index: c_int, value: u64) { (*vcpu).arch.pdptrs[index as usize] = value; }

pub unsafe fn kvm_read_cr0_bits(vcpu: *mut kvm_vcpu, mask: ulong) -> ulong { let tmask = mask & KVM_POSSIBLE_CR0_GUEST_BITS; if (tmask & (*vcpu).arch.cr0_guest_owned_bits) != 0 && !kvm_register_is_available(vcpu, VCPU_REG_CR0) { kvm_x86_call(cache_reg)(vcpu, VCPU_REG_CR0); } (*vcpu).arch.cr0 & mask }
pub unsafe fn kvm_is_cr0_bit_set(vcpu: *mut kvm_vcpu, bit: ulong) -> bool { kvm_read_cr0_bits(vcpu, bit) != 0 }
pub unsafe fn kvm_read_cr0(vcpu: *mut kvm_vcpu) -> ulong { kvm_read_cr0_bits(vcpu, !0) }
pub unsafe fn kvm_read_cr4_bits(vcpu: *mut kvm_vcpu, mask: ulong) -> ulong { let tmask = mask & KVM_POSSIBLE_CR4_GUEST_BITS; if (tmask & (*vcpu).arch.cr4_guest_owned_bits) != 0 && !kvm_register_is_available(vcpu, VCPU_REG_CR4) { kvm_x86_call(cache_reg)(vcpu, VCPU_REG_CR4); } (*vcpu).arch.cr4 & mask }
pub unsafe fn kvm_is_cr4_bit_set(vcpu: *mut kvm_vcpu, bit: ulong) -> bool { kvm_read_cr4_bits(vcpu, bit) != 0 }
pub unsafe fn kvm_read_cr3(vcpu: *mut kvm_vcpu) -> ulong { if !kvm_register_is_available(vcpu, VCPU_REG_CR3) { kvm_x86_call(cache_reg)(vcpu, VCPU_REG_CR3); } (*vcpu).arch.cr3 }
pub unsafe fn kvm_read_cr4(vcpu: *mut kvm_vcpu) -> ulong { kvm_read_cr4_bits(vcpu, !0) }
pub unsafe fn __kvm_is_valid_cr4(vcpu: *mut kvm_vcpu, cr4: ulong) -> bool { (cr4 & (*vcpu).arch.cr4_guest_rsvd_bits) == 0 }

pub unsafe fn is_protmode(vcpu: *mut kvm_vcpu) -> bool { kvm_is_cr0_bit_set(vcpu, X86_CR0_PE) }
pub unsafe fn is_pae(vcpu: *mut kvm_vcpu) -> bool { kvm_is_cr4_bit_set(vcpu, X86_CR4_PAE) }
pub unsafe fn is_pse(vcpu: *mut kvm_vcpu) -> bool { kvm_is_cr4_bit_set(vcpu, X86_CR4_PSE) }
pub unsafe fn is_paging(vcpu: *mut kvm_vcpu) -> bool { likely!(kvm_is_cr0_bit_set(vcpu, X86_CR0_PG)) }
pub unsafe fn is_pae_paging(vcpu: *mut kvm_vcpu) -> bool { !is_long_mode(vcpu) && is_pae(vcpu) && is_paging(vcpu) }
pub fn kvm_dr7_valid(data: u64) -> bool { (data >> 32) == 0 }
pub fn kvm_dr6_valid(data: u64) -> bool { (data >> 32) == 0 }
pub unsafe fn kvm_get_effective_dr7(vcpu: *mut kvm_vcpu) -> ulong { if (*vcpu).guest_debug & KVM_GUESTDBG_USE_HW_BP != 0 { (*vcpu).arch.guest_debug_dr7 } else { (*vcpu).arch.dr7 } }
pub unsafe fn enter_guest_mode(vcpu: *mut kvm_vcpu) { (*vcpu).arch.hflags |= HF_GUEST_MASK; (*vcpu).stat.guest_mode = 1; }
pub unsafe fn leave_guest_mode(vcpu: *mut kvm_vcpu) { (*vcpu).arch.hflags &= !HF_GUEST_MASK; if (*vcpu).arch.load_eoi_exitmap_pending { (*vcpu).arch.load_eoi_exitmap_pending = false; kvm_make_request(KVM_REQ_LOAD_EOI_EXITMAP, vcpu); } (*vcpu).stat.guest_mode = 0; }
pub unsafe fn is_guest_mode(vcpu: *mut kvm_vcpu) -> bool { (*vcpu).arch.hflags & HF_GUEST_MASK != 0 }
pub unsafe fn kvm_get_segment_base(vcpu: *mut kvm_vcpu, seg: c_int) -> ulong { kvm_x86_call(get_segment_base)(vcpu, seg) }
pub unsafe fn kvm_set_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: c_int) { kvm_x86_call(set_segment)(vcpu, var, seg); }
pub unsafe fn kvm_get_segment(vcpu: *mut kvm_vcpu, var: *mut kvm_segment, seg: c_int) { kvm_x86_call(get_segment)(vcpu, var, seg); }

extern "C" {
    pub fn kvm_get_linear_rip(vcpu: *mut kvm_vcpu) -> ulong;
    pub fn kvm_is_linear_rip(vcpu: *mut kvm_vcpu, linear_rip: ulong) -> bool;
    pub fn kvm_get_rflags(vcpu: *mut kvm_vcpu) -> ulong;
    pub fn __kvm_set_rflags(vcpu: *mut kvm_vcpu, rflags: ulong);
    pub fn kvm_set_rflags(vcpu: *mut kvm_vcpu, rflags: ulong);
    pub fn kvm_vcpu_ioctl_x86_get_sregs2(vcpu: *mut kvm_vcpu, sregs2: *mut kvm_sregs2);
    pub fn kvm_vcpu_ioctl_x86_set_sregs2(vcpu: *mut kvm_vcpu, sregs2: *mut kvm_sregs2) -> c_int;
    pub fn kvm_run_sync_regs_to_user(vcpu: *mut kvm_vcpu);
    pub fn kvm_run_sync_regs_from_user(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_update_dr0123(vcpu: *mut kvm_vcpu);
    pub fn kvm_update_dr7(vcpu: *mut kvm_vcpu);
    pub fn kvm_vcpu_ioctl_x86_get_debugregs(vcpu: *mut kvm_vcpu, dbgregs: *mut kvm_debugregs) -> c_int;
    pub fn kvm_vcpu_ioctl_x86_set_debugregs(vcpu: *mut kvm_vcpu, dbgregs: *mut kvm_debugregs) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
