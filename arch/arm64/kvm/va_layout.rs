// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies are supplied by the surrounding kernel translation.

/* The LSB of the HYP VA tag */
static mut tag_lsb: u8 = 0;
/* The HYP VA tag value with the region bit */
static mut tag_val: u64 = 0;
static mut va_mask: u64 = 0;

/* Compute HYP VA by using the same computation as kern_hyp_va(). */
unsafe fn __early_kern_hyp_va(mut addr: u64) -> u64 {
    addr &= va_mask;
    addr |= tag_val << tag_lsb;
    addr
}

/* Store a hyp VA <-> PA offset into a EL2-owned variable. */
unsafe fn init_hyp_physvirt_offset() {
    let kern_va: u64 = lm_alias(__hyp_text_start as _ ) as u64;
    let hyp_va = __early_kern_hyp_va(kern_va);
    hyp_physvirt_offset = __pa(kern_va) as i64 - hyp_va as i64;
}

/* Calculate the actual VA size used by the hypervisor */
unsafe fn kvm_hyp_va_bits() -> u32 {
    // The ID map and regular kernel stage 1 both need to fit in the range.
    core::cmp::max(IDMAP_VA_BITS, vabits_actual)
}

/* Compute the hypervisor virtual-address layout. */
unsafe fn kvm_compute_layout() {
    let idmap_addr: phys_addr_t = __pa_symbol(__hyp_idmap_text_start);
    let hyp_va_bits = kvm_hyp_va_bits();
    let mut hyp_va_msb = idmap_addr & BIT(hyp_va_bits - 1);
    hyp_va_msb ^= BIT(hyp_va_bits - 1);

    tag_lsb = fls64((phys_to_virt(memblock_start_of_DRAM()) as u64)
        ^ (high_memory as u64 - 1)) as u8;
    va_mask = GENMASK_ULL(tag_lsb as u32 - 1, 0);
    tag_val = hyp_va_msb;

    if IS_ENABLED(CONFIG_RANDOMIZE_BASE) && tag_lsb != (hyp_va_bits - 1) as u8 {
        tag_val |= get_random_long() & GENMASK_ULL(hyp_va_bits - 2, tag_lsb as u32);
    }
    tag_val >>= tag_lsb;
    init_hyp_physvirt_offset();
}

/* Convert kimg VAs in the .hyp.reloc ELF section to hyp VAs. */
unsafe fn kvm_apply_hyp_relocations() {
    let mut rel = __hyp_reloc_begin as *mut i32;
    let end = __hyp_reloc_end as *mut i32;
    while rel < end {
        let ptr = lm_alias((rel as *mut u8).offset(*rel as isize)) as *mut usize;
        let kimg_va = *ptr;
        *ptr = __early_kern_hyp_va(lm_alias(kimg_va as _) as usize as u64) as usize;
        rel = rel.add(1);
    }
}

unsafe fn compute_instruction(n: i32, rd: u32, rn: u32) -> u32 {
    let mut insn = AARCH64_BREAK_FAULT;
    match n {
        0 => insn = aarch64_insn_gen_logical_immediate(AARCH64_INSN_LOGIC_AND, AARCH64_INSN_VARIANT_64BIT, rn, rd, va_mask),
        1 => insn = aarch64_insn_gen_extr(AARCH64_INSN_VARIANT_64BIT, rn, rn, rd, tag_lsb),
        2 => insn = aarch64_insn_gen_add_sub_imm(rd, rn, tag_val & GENMASK(11, 0), AARCH64_INSN_VARIANT_64BIT, AARCH64_INSN_ADSB_ADD),
        3 => insn = aarch64_insn_gen_add_sub_imm(rd, rn, tag_val & GENMASK(23, 12), AARCH64_INSN_VARIANT_64BIT, AARCH64_INSN_ADSB_ADD),
        4 => insn = aarch64_insn_gen_extr(AARCH64_INSN_VARIANT_64BIT, rn, rn, rd, 64 - tag_lsb),
        _ => {}
    }
    insn
}

unsafe fn kvm_update_va_mask(_alt: *mut alt_instr, origptr: *const __le32, updptr: *mut __le32, nr_inst: i32) {
    BUG_ON(nr_inst != 5);
    for i in 0..nr_inst {
        if cpus_have_cap(ARM64_HAS_VIRT_HOST_EXTN) || (tag_val == 0 && i > 0) {
            *updptr.add(i as usize) = cpu_to_le32(aarch64_insn_gen_nop());
            continue;
        }
        let oinsn = le32_to_cpu(*origptr.add(i as usize));
        let rd = aarch64_insn_decode_register(AARCH64_INSN_REGTYPE_RD, oinsn);
        let rn = aarch64_insn_decode_register(AARCH64_INSN_REGTYPE_RN, oinsn);
        let insn = compute_instruction(i, rd, rn);
        BUG_ON(insn == AARCH64_BREAK_FAULT);
        *updptr.add(i as usize) = cpu_to_le32(insn);
    }
}

unsafe fn kvm_patch_vector_branch(_alt: *mut alt_instr, origptr: *const __le32, mut updptr: *mut __le32, nr_inst: i32) {
    BUG_ON(nr_inst != 4);
    if !cpus_have_cap(ARM64_SPECTRE_V3A) || WARN_ON_ONCE(cpus_have_cap(ARM64_HAS_VIRT_HOST_EXTN)) { return; }
    let mut addr = __early_kern_hyp_va(kvm_ksym_ref(__kvm_hyp_vector) as u64);
    addr |= (origptr as u64) & GENMASK_ULL(10, 7);
    addr += KVM_VECTOR_PREAMBLE;
    *updptr = cpu_to_le32(aarch64_insn_gen_movewide(AARCH64_INSN_REG_0, addr as u16, 0, AARCH64_INSN_VARIANT_64BIT, AARCH64_INSN_MOVEWIDE_ZERO)); updptr = updptr.add(1);
    *updptr = cpu_to_le32(aarch64_insn_gen_movewide(AARCH64_INSN_REG_0, (addr >> 16) as u16, 16, AARCH64_INSN_VARIANT_64BIT, AARCH64_INSN_MOVEWIDE_KEEP)); updptr = updptr.add(1);
    *updptr = cpu_to_le32(aarch64_insn_gen_movewide(AARCH64_INSN_REG_0, (addr >> 32) as u16, 32, AARCH64_INSN_VARIANT_64BIT, AARCH64_INSN_MOVEWIDE_KEEP)); updptr = updptr.add(1);
    *updptr = cpu_to_le32(aarch64_insn_gen_branch_reg(AARCH64_INSN_REG_0, AARCH64_INSN_BRANCH_NOLINK));
}

unsafe fn generate_mov_q(val: u64, origptr: *const __le32, mut updptr: *mut __le32, nr_inst: i32) {
    BUG_ON(nr_inst != 4);
    let rd = aarch64_insn_decode_register(AARCH64_INSN_REGTYPE_RD, le32_to_cpu(*origptr));
    for (shift, mode) in [(0, AARCH64_INSN_MOVEWIDE_ZERO), (16, AARCH64_INSN_MOVEWIDE_KEEP), (32, AARCH64_INSN_MOVEWIDE_KEEP), (48, AARCH64_INSN_MOVEWIDE_KEEP)] {
        *updptr = cpu_to_le32(aarch64_insn_gen_movewide(rd, (val >> shift) as u16, shift, AARCH64_INSN_VARIANT_64BIT, mode));
        updptr = updptr.add(1);
    }
}

unsafe fn kvm_get_kimage_voffset(_alt: *mut alt_instr, origptr: *const __le32, updptr: *mut __le32, nr_inst: i32) { generate_mov_q(kimage_voffset, origptr, updptr, nr_inst); }
unsafe fn kvm_compute_final_ctr_el0(_alt: *mut alt_instr, origptr: *const __le32, updptr: *mut __le32, nr_inst: i32) { generate_mov_q(read_sanitised_ftr_reg(SYS_CTR_EL0), origptr, updptr, nr_inst); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
