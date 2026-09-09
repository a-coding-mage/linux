// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 SUSE Linux Products GmbH. All rights reserved.
 * Copyright 2010-2011 Freescale Semiconductor, Inc.
 *
 * Authors:
 *     Alexander Graf <agraf@suse.de>
 */

// Kernel and architecture dependencies are supplied by other translation units.

const KVM_MAGIC_PAGE: isize = -4096;
const KVM_INST_LWZ: u32 = 0x80000000;
const KVM_INST_STW: u32 = 0x90000000;
const KVM_INST_LD: u32 = 0xe8000000;
const KVM_INST_STD: u32 = 0xf8000000;
const KVM_INST_NOP: u32 = 0x60000000;
const KVM_INST_B: u32 = 0x48000000;
const KVM_INST_B_MASK: u32 = 0x03ffffff;
const KVM_INST_B_MAX: isize = 0x01ffffff;
const KVM_INST_LI: u32 = 0x38000000;
const KVM_MASK_RT: u32 = 0x03e00000;
const KVM_RT_30: u32 = 0x03c00000;
const KVM_MASK_RB: u32 = 0x0000f800;
const KVM_INST_MFMSR: u32 = 0x7c0000a6;
const SPR_FROM: u32 = 0;
const SPR_TO: u32 = 0x100;
const KVM_INST_TLBSYNC: u32 = 0x7c00046c;
const KVM_INST_MTMSRD_L0: u32 = 0x7c000164;
const KVM_INST_MTMSRD_L1: u32 = 0x7c010164;
const KVM_INST_MTMSR: u32 = 0x7c000124;
const KVM_INST_WRTEE: u32 = 0x7c000106;
const KVM_INST_WRTEEI_0: u32 = 0x7c000146;
const KVM_INST_WRTEEI_1: u32 = 0x7c008146;
const KVM_INST_MTSRIN: u32 = 0x7c0001e4;

const fn kvm_inst_spr(sprn: u32, moveto: u32) -> u32 {
    0x7c0002a6 | ((sprn & 0x1f) << 16) | ((sprn & 0x3e0) << 6) | moveto
}
const fn kvm_inst_mfspr(sprn: u32) -> u32 { kvm_inst_spr(sprn, SPR_FROM) }
const fn kvm_inst_mtspr(sprn: u32) -> u32 { kvm_inst_spr(sprn, SPR_TO) }

static mut KVM_PATCHING_WORKED: bool = true;
extern "C" {
    static mut kvm_tmp: u8;
    static kvm_tmp_end: u8;
    static mut kvm_emulate_mtmsrd_branch_offs: u32;
    static mut kvm_emulate_mtmsrd_reg_offs: u32;
    static mut kvm_emulate_mtmsrd_orig_ins_offs: u32;
    static mut kvm_emulate_mtmsrd_len: u32;
    static mut kvm_emulate_mtmsrd: u32;
    static mut kvm_emulate_mtmsr_branch_offs: u32;
    static mut kvm_emulate_mtmsr_reg1_offs: u32;
    static mut kvm_emulate_mtmsr_reg2_offs: u32;
    static mut kvm_emulate_mtmsr_orig_ins_offs: u32;
    static mut kvm_emulate_mtmsr_len: u32;
    static mut kvm_emulate_mtmsr: u32;
    static kvm_template_start: u32;
    static kvm_template_end: u32;
}
static mut KVM_TMP_INDEX: usize = 0;

extern "C" {
    fn flush_icache_range(start: usize, end: usize);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn get_rt(rt: u32) -> u32;
    fn on_each_cpu(func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32);
    fn fault_in_readable(addr: *const core::ffi::c_void, size: usize) -> i32;
    fn local_irq_disable();
    fn local_irq_enable();
    fn kvm_map_magic_page(data: *mut core::ffi::c_void);
    fn kvm_para_available() -> bool;
    fn kvm_para_has_feature(feature: u32) -> bool;
    fn epapr_hypercall(input: *const usize, output: *mut usize, token: u32);
}
extern "C" { static mut epapr_paravirt_enabled: bool; static mut powersave_nap: u32; }

unsafe fn magic_var(offset: usize) -> isize { KVM_MAGIC_PAGE + offset as isize }
unsafe fn kvm_patch_ins(inst: *mut u32, new_inst: u32) { *inst = new_inst; flush_icache_range(inst as usize, inst as usize + 4); }
unsafe fn kvm_patch_ins_ll(inst: *mut u32, addr: isize, rt: u32) {
    #[cfg(target_pointer_width = "64")] { kvm_patch_ins(inst, KVM_INST_LD | rt | (addr as u32 & 0x0000fffc)); }
    #[cfg(not(target_pointer_width = "64"))] { kvm_patch_ins(inst, KVM_INST_LWZ | rt | (addr as u32 & 0x0000fffc)); }
}
unsafe fn kvm_patch_ins_ld(inst: *mut u32, addr: isize, rt: u32) {
    #[cfg(target_pointer_width = "64")] { kvm_patch_ins(inst, KVM_INST_LD | rt | (addr as u32 & 0x0000fffc)); }
    #[cfg(not(target_pointer_width = "64"))] { kvm_patch_ins(inst, KVM_INST_LWZ | rt | ((addr + 4) as u32 & 0x0000fffc)); }
}
unsafe fn kvm_patch_ins_lwz(inst: *mut u32, addr: isize, rt: u32) { kvm_patch_ins(inst, KVM_INST_LWZ | rt | (addr as u32 & 0xffff)); }
unsafe fn kvm_patch_ins_std(inst: *mut u32, addr: isize, rt: u32) {
    #[cfg(target_pointer_width = "64")] { kvm_patch_ins(inst, KVM_INST_STD | rt | (addr as u32 & 0x0000fffc)); }
    #[cfg(not(target_pointer_width = "64"))] { kvm_patch_ins(inst, KVM_INST_STW | rt | ((addr + 4) as u32 & 0x0000fffc)); }
}
unsafe fn kvm_patch_ins_stw(inst: *mut u32, addr: isize, rt: u32) { kvm_patch_ins(inst, KVM_INST_STW | rt | (addr as u32 & 0x0000fffc)); }
unsafe fn kvm_patch_ins_nop(inst: *mut u32) { kvm_patch_ins(inst, KVM_INST_NOP); }
unsafe fn kvm_patch_ins_b(inst: *mut u32, addr: isize) { kvm_patch_ins(inst, KVM_INST_B | (addr as u32 & KVM_INST_B_MASK)); }

unsafe fn kvm_alloc(len: usize) -> *mut u32 {
    if KVM_TMP_INDEX + len > (&kvm_tmp_end as *const u8 as usize - &kvm_tmp as *const u8 as usize) {
        KVM_PATCHING_WORKED = false; return core::ptr::null_mut();
    }
    let p = (&mut kvm_tmp as *mut u8).add(KVM_TMP_INDEX) as *mut u32;
    KVM_TMP_INDEX += len; p
}

unsafe fn patch_chunk(inst: *mut u32, len: u32, template: *mut u32, branch: u32, orig: Option<u32>, edits: impl Fn(*mut u32)) {
    let p = kvm_alloc((len * 4) as usize); if p.is_null() { return; }
    let start = p as isize - inst as isize; let end = inst as isize + 4 - p.add(branch as usize) as isize;
    if start > KVM_INST_B_MAX { KVM_PATCHING_WORKED = false; return; }
    memcpy(p as *mut _, template as *const _, (len * 4) as usize); *p.add(branch as usize) |= end as u32 & KVM_INST_B_MASK; edits(p);
    if let Some(v) = orig { *p.add(0) = v; }
    flush_icache_range(p as usize, p as usize + (len * 4) as usize); kvm_patch_ins_b(inst, start);
}

unsafe fn kvm_map_magic_page_impl(data: *mut core::ffi::c_void) {
    let features = data as *mut u32; let input = [KVM_MAGIC_PAGE as usize, (KVM_MAGIC_PAGE as usize) | 1, 0,0,0,0,0,0]; let mut output = [0usize; 8];
    epapr_hypercall(input.as_ptr(), output.as_mut_ptr(), 0); *features = output[0];
}

// The instruction decoder and architecture-specific patch cases are kept in source form.
// Build-time kernel symbols and SPR constants are intentionally unresolved here.
unsafe fn kvm_check_ins(inst: *mut u32, features: u32) {
    let raw = *inst; let no_rt = raw & !KVM_MASK_RT; let rt = raw & KVM_MASK_RT;
    match no_rt {
        KVM_INST_MFMSR => kvm_patch_ins_ld(inst, magic_var(0), rt),
        KVM_INST_TLBSYNC => kvm_patch_ins_nop(inst),
        KVM_INST_MTMSRD_L1 | KVM_INST_MTMSRD_L0 | KVM_INST_MTMSR => { kvm_patch_ins_b(inst, rt as isize); let _ = features; }
        _ => { let _ = (features, rt); }
    }
}

#[cfg(feature = "CONFIG_BOOKE")]
unsafe fn kvm_patch_ins_wrtee(inst: *mut u32, rt: u32, imm_one: i32) {
    let _ = (inst, rt, imm_one);
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_32")]
unsafe fn kvm_patch_ins_mtsrin(inst: *mut u32, rt: u32, rb: u32) {
    let _ = (inst, rt, rb);
}

// In the kernel these declarations are emitted by the architecture's generated
// assembly templates; retain them as external symbols for the translated unit.
extern "C" {
    static _stext: u32;
    static _etext: u32;
}

unsafe fn kvm_use_magic_page() {
    let mut features = 0u32; on_each_cpu(kvm_map_magic_page_impl, &mut features as *mut _ as *mut _, 1);
    if fault_in_readable(KVM_MAGIC_PAGE as *const _, 4) != 0 { KVM_PATCHING_WORKED = false; return; }
    local_irq_disable();
    let mut p = &kvm_template_start as *const u32 as usize;
    let end = &kvm_template_end as *const u32 as usize;
    while p < end { kvm_check_ins(p as *mut u32, features); p += 4; }
    local_irq_enable();
}

unsafe fn kvm_guest_init() -> i32 {
    if !kvm_para_available() || !epapr_paravirt_enabled { return 0; }
    if kvm_para_has_feature(1) { kvm_use_magic_page(); }
    #[cfg(target_arch = "powerpc64")] { powersave_nap = 1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
