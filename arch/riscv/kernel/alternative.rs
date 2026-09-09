// SPDX-License-Identifier: GPL-2.0-only
/*
 * alternative runtime patching
 * inspired by the ARM64 and x86 version
 *
 * Copyright (C) 2021 Sifive.
 */

// C headers supplied by the surrounding kernel translation unit are omitted.

#[repr(C)]
pub struct cpu_manufacturer_info_t {
    pub vendor_id: usize,
    pub arch_id: usize,
    pub imp_id: usize,
    pub patch_func: Option<unsafe extern "C" fn(*mut alt_entry, *mut alt_entry, usize, usize, u32)>,
}

#[repr(C)]
pub struct alt_entry {
    _private: [u8; 0],
}

extern "C" {
    fn csr_read(csr: usize) -> usize;
    fn sbi_get_mvendorid() -> usize;
    fn sbi_get_marchid() -> usize;
    fn sbi_get_mimpid() -> usize;
    fn andes_errata_patch_func(begin: *mut alt_entry, end: *mut alt_entry, archid: usize, impid: usize, stage: u32);
    fn mips_errata_patch_func(begin: *mut alt_entry, end: *mut alt_entry, archid: usize, impid: usize, stage: u32);
    fn sifive_errata_patch_func(begin: *mut alt_entry, end: *mut alt_entry, archid: usize, impid: usize, stage: u32);
    fn thead_errata_patch_func(begin: *mut alt_entry, end: *mut alt_entry, archid: usize, impid: usize, stage: u32);
    fn riscv_cpufeature_patch_func(begin: *mut alt_entry, end: *mut alt_entry, stage: u32);
    fn riscv_insn_extract_utype_itype_imm(auipc_insn: u32, jalr_insn: u32) -> i32;
    fn riscv_insn_insert_utype_itype_imm(auipc_insn: *mut u32, jalr_insn: *mut u32, imm: i32);
    fn riscv_insn_extract_jtype_imm(jal_insn: u32) -> i32;
    fn riscv_insn_insert_jtype_imm(jal_insn: *mut u32, imm: i32);
    fn riscv_insn_is_auipc(insn: u32) -> bool;
    fn riscv_insn_is_jalr(insn: u32) -> bool;
    fn riscv_insn_is_jal(insn: u32) -> bool;
    fn patch_text_nosync(addr: *mut core::ffi::c_void, insns: *const u32, len: usize);
}

unsafe fn riscv_fill_cpu_mfr_info(cpu_mfr_info: *mut cpu_manufacturer_info_t) {
    #[cfg(CONFIG_RISCV_M_MODE)]
    {
        (*cpu_mfr_info).vendor_id = csr_read(CSR_MVENDORID);
        (*cpu_mfr_info).arch_id = csr_read(CSR_MARCHID);
        (*cpu_mfr_info).imp_id = csr_read(CSR_MIMPID);
    }
    #[cfg(not(CONFIG_RISCV_M_MODE))]
    {
        (*cpu_mfr_info).vendor_id = sbi_get_mvendorid();
        (*cpu_mfr_info).arch_id = sbi_get_marchid();
        (*cpu_mfr_info).imp_id = sbi_get_mimpid();
    }

    (*cpu_mfr_info).patch_func = None;
    match (*cpu_mfr_info).vendor_id {
        #[cfg(CONFIG_ERRATA_ANDES)]
        ANDES_VENDOR_ID => (*cpu_mfr_info).patch_func = Some(andes_errata_patch_func),
        #[cfg(CONFIG_ERRATA_MIPS)]
        MIPS_VENDOR_ID => (*cpu_mfr_info).patch_func = Some(mips_errata_patch_func),
        #[cfg(CONFIG_ERRATA_SIFIVE)]
        SIFIVE_VENDOR_ID => (*cpu_mfr_info).patch_func = Some(sifive_errata_patch_func),
        #[cfg(CONFIG_ERRATA_THEAD)]
        THEAD_VENDOR_ID => (*cpu_mfr_info).patch_func = Some(thead_errata_patch_func),
        _ => {}
    }
}

unsafe fn riscv_instruction_at(p: *mut core::ffi::c_void) -> u32 {
    let parcel = p as *mut u16;
    (*parcel as u32) | ((*parcel.add(1) as u32) << 16)
}

unsafe fn riscv_alternative_fix_auipc_jalr(ptr: *mut core::ffi::c_void, auipc_insn: u32, jalr_insn: u32, patch_offset: i32) {
    let mut call = [auipc_insn, jalr_insn];
    let mut imm = riscv_insn_extract_utype_itype_imm(auipc_insn, jalr_insn);
    imm = imm.wrapping_sub(patch_offset);
    riscv_insn_insert_utype_itype_imm(call.as_mut_ptr(), call.as_mut_ptr().add(1), imm);
    patch_text_nosync(ptr, call.as_ptr(), core::mem::size_of::<u32>() * 2);
}

unsafe fn riscv_alternative_fix_jal(ptr: *mut core::ffi::c_void, mut jal_insn: u32, patch_offset: i32) {
    let mut imm = riscv_insn_extract_jtype_imm(jal_insn);
    imm = imm.wrapping_sub(patch_offset);
    riscv_insn_insert_jtype_imm(&mut jal_insn, imm);
    patch_text_nosync(ptr, &jal_insn, core::mem::size_of::<u32>());
}

pub unsafe fn riscv_alternative_fix_offsets(alt_ptr: *mut core::ffi::c_void, len: u32, patch_offset: i32) {
    let num_insn = (len as usize) / core::mem::size_of::<u32>();
    let mut i = 0usize;
    while i < num_insn {
        let insn = riscv_instruction_at((alt_ptr as *mut u8).add(i * core::mem::size_of::<u32>()) as *mut _);
        if riscv_insn_is_auipc(insn) && i < num_insn - 1 {
            let insn2 = riscv_instruction_at((alt_ptr as *mut u8).add((i + 1) * core::mem::size_of::<u32>()) as *mut _);
            if riscv_insn_is_jalr(insn2) && ((insn >> 7) & 0x1f) == 1 {
                riscv_alternative_fix_auipc_jalr((alt_ptr as *mut u8).add(i * 4) as *mut _, insn, insn2, patch_offset);
                i += 1;
            }
        }
        if riscv_insn_is_jal(insn) {
            let imm = riscv_insn_extract_jtype_imm(insn) as isize;
            let current = (alt_ptr as *mut u8).add(i * 4) as usize;
            let target = current.wrapping_add(imm as usize);
            let start = alt_ptr as usize;
            if target >= start && target < start.wrapping_add(len as usize) {
                i += 1;
                continue;
            }
            riscv_alternative_fix_jal(current as *mut _, insn, patch_offset);
        }
        i += 1;
    }
}

unsafe fn _apply_alternatives(begin: *mut alt_entry, end: *mut alt_entry, stage: u32) {
    let mut cpu_mfr_info = core::mem::MaybeUninit::<cpu_manufacturer_info_t>::uninit();
    riscv_fill_cpu_mfr_info(cpu_mfr_info.as_mut_ptr());
    let info = cpu_mfr_info.assume_init();
    riscv_cpufeature_patch_func(begin, end, stage);
    if let Some(patch_func) = info.patch_func {
        patch_func(begin, end, info.arch_id, info.imp_id, stage);
    }
}

unsafe fn apply_vdso_alternatives(base: *mut core::ffi::c_void, alternatives_begin: usize, alternatives_end: usize) {
    if alternatives_begin == alternatives_end { return; }
    _apply_alternatives(base.add(alternatives_begin) as *mut alt_entry, base.add(alternatives_end) as *mut alt_entry, RISCV_ALTERNATIVES_BOOT);
}

pub unsafe fn apply_boot_alternatives() {
    WARN_ON(smp_processor_id() != 0);
    _apply_alternatives(__alt_start as *mut alt_entry, __alt_end as *mut alt_entry, RISCV_ALTERNATIVES_BOOT);
    #[cfg(CONFIG_MMU)]
    apply_vdso_alternatives(vdso_start, __vdso_alternatives_start_offset, __vdso_alternatives_end_offset);
    #[cfg(CONFIG_RISCV_USER_CFI)]
    apply_vdso_alternatives(vdso_cfi_start, __vdso_alternatives_start_cfi_offset, __vdso_alternatives_end_cfi_offset);
    #[cfg(CONFIG_COMPAT)]
    apply_vdso_alternatives(compat_vdso_start, compat__vdso_alternatives_start_offset, compat__vdso_alternatives_end_offset);
}

pub unsafe fn apply_early_boot_alternatives() {
    #[cfg(CONFIG_RISCV_ALTERNATIVE_EARLY)]
    _apply_alternatives(__alt_start as *mut alt_entry, __alt_end as *mut alt_entry, RISCV_ALTERNATIVES_EARLY_BOOT);
}

#[cfg(CONFIG_MODULES)]
pub unsafe fn apply_module_alternatives(start: *mut core::ffi::c_void, length: usize) {
    _apply_alternatives(start as *mut alt_entry, start.add(length) as *mut alt_entry, RISCV_ALTERNATIVES_MODULE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
