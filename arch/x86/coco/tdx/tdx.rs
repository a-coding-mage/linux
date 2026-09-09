// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021-2022 Intel Corporation */

/* C dependencies and build-time configuration are supplied by the kernel. */

const EPT_READ: u64 = 0;
const EPT_WRITE: u64 = 1;
const PORT_READ: u64 = 0;
const PORT_WRITE: u64 = 1;

#[inline]
const fn ve_is_io_in(e: u32) -> bool { e & (1 << 3) != 0 }
#[inline]
const fn ve_get_io_size(e: u32) -> u32 { (e & 7) + 1 }
#[inline]
const fn ve_get_port_num(e: u32) -> u32 { e >> 16 }
#[inline]
const fn ve_is_io_string(e: u32) -> bool { e & (1 << 4) != 0 }

const TDCALL_INVALID_OPERAND: u64 = 0xc0000100;
const TDCALL_OPERAND_BUSY: u64 = 0x80000200;
const TDREPORT_SUBTYPE_0: u64 = 0;

static mut NR_SHARED: atomic_long_t = atomic_long_t::new(0);

#[no_mangle]
pub unsafe extern "C" fn __tdx_hypercall_failed() -> ! {
    instrumentation_begin();
    panic!("TDVMCALL failed. TDX module bug?");
}

#[cfg(CONFIG_KVM_GUEST)]
pub unsafe extern "C" fn tdx_kvm_hypercall(nr: u32, p1: usize, p2: usize, p3: usize, p4: usize) -> isize {
    let mut args = tdx_module_args { r10: nr as u64, r11: p1 as u64, r12: p2 as u64, r13: p3 as u64, r14: p4 as u64, ..core::mem::zeroed() };
    __tdx_hypercall(&mut args)
}

#[inline]
unsafe fn tdcall(fn_: u64, args: *mut tdx_module_args) {
    if __tdcall_ret(fn_, args) != 0 { panic!("TDCALL failed (Buggy TDX module!)"); }
}

#[inline]
unsafe fn tdg_vm_rd(field: u64, value: *mut u64) -> u64 {
    let mut args: tdx_module_args = core::mem::zeroed(); args.rdx = field;
    let ret = __tdcall_ret(TDG_VM_RD, &mut args); *value = args.r8; ret
}

#[inline]
unsafe fn tdg_vm_wr(field: u64, value: u64, mask: u64) -> u64 {
    let mut args: tdx_module_args = core::mem::zeroed(); args.rdx = field; args.r8 = value; args.r9 = mask;
    __tdcall(TDG_VM_WR, &mut args)
}

pub unsafe extern "C" fn tdx_mcall_get_report0(reportdata: *mut u8, tdreport: *mut u8) -> i32 {
    let mut args: tdx_module_args = core::mem::zeroed();
    args.rcx = virt_to_phys(tdreport); args.rdx = virt_to_phys(reportdata); args.r8 = TDREPORT_SUBTYPE_0;
    let ret = __tdcall(TDG_MR_REPORT, &mut args);
    if ret != 0 { return if ret >> 32 == TDCALL_INVALID_OPERAND { -ENXIO } else if ret >> 32 == TDCALL_OPERAND_BUSY { -EBUSY } else { -EIO }; }
    0
}

pub unsafe extern "C" fn tdx_mcall_extend_rtmr(index: u8, data: *mut u8) -> i32 {
    let mut args: tdx_module_args = core::mem::zeroed(); args.rcx = virt_to_phys(data); args.rdx = index as u64;
    let ret = __tdcall(TDG_MR_RTMR_EXTEND, &mut args);
    if ret != 0 { return if ret >> 32 == TDCALL_INVALID_OPERAND { -ENXIO } else if ret >> 32 == TDCALL_OPERAND_BUSY { -EBUSY } else { -EIO }; }
    0
}

pub unsafe extern "C" fn tdx_hcall_get_quote(buf: *mut u8, size: usize) -> u64 {
    _tdx_hypercall(TDVMCALL_GET_QUOTE, cc_mkdec(virt_to_phys(buf)), size as u64, 0, 0)
}

unsafe fn disable_sept_ve(td_attr: u64) {
    let mut config = 0; let mut controls = 0;
    tdg_vm_rd(TDCS_CONFIG_FLAGS, &mut config);
    if config & TDCS_CONFIG_FLEXIBLE_PENDING_VE == 0 {
        if td_attr & TDX_TD_ATTR_SEPT_VE_DISABLE != 0 { return; }
        if td_attr & TDX_TD_ATTR_DEBUG != 0 { pr_warn("TD misconfiguration: SEPT #VE has to be disabled"); } else { tdx_panic("TD misconfiguration: SEPT #VE has to be disabled"); }
        return;
    }
    tdg_vm_rd(TDCS_TD_CTLS, &mut controls);
    if controls & TD_CTLS_PENDING_VE_DISABLE != 0 || td_attr & TDX_TD_ATTR_DEBUG != 0 { return; }
    tdg_vm_wr(TDCS_TD_CTLS, TD_CTLS_PENDING_VE_DISABLE, TD_CTLS_PENDING_VE_DISABLE);
}

unsafe fn enable_cpu_topology_enumeration() {
    let mut configured = 0; tdg_vm_rd(TDCS_TOPOLOGY_ENUM_CONFIGURED, &mut configured);
    if configured == 0 { pr_err("VMM did not configure X2APIC_IDs properly"); return; }
    tdg_vm_wr(TDCS_TD_CTLS, TD_CTLS_ENUM_TOPOLOGY, TD_CTLS_ENUM_TOPOLOGY);
}
unsafe fn reduce_unnecessary_ve() { if tdg_vm_wr(TDCS_TD_CTLS, TD_CTLS_REDUCE_VE, TD_CTLS_REDUCE_VE) != TDX_SUCCESS { enable_cpu_topology_enumeration(); } }

unsafe fn tdx_setup(cc_mask: *mut u64) {
    let mut args: tdx_module_args = core::mem::zeroed(); tdcall(TDG_VP_INFO, &mut args);
    let gpa_width = args.rcx & 0x3f; *cc_mask = 1u64 << (gpa_width - 1);
    tdg_vm_wr(TDCS_NOTIFY_ENABLES, 0, u64::MAX); disable_sept_ve(args.rdx); reduce_unnecessary_ve();
}

unsafe fn ve_instr_len(ve: *const ve_info) -> i32 {
    match (*ve).exit_reason { EXIT_REASON_HLT | EXIT_REASON_MSR_READ | EXIT_REASON_MSR_WRITE | EXIT_REASON_CPUID | EXIT_REASON_IO_INSTRUCTION => (*ve).instr_len as i32, EXIT_REASON_EPT_VIOLATION => { WARN_ONCE(true, "ve->instr_len is not defined for EPT violations"); 0 }, _ => { WARN_ONCE(true, "Unexpected #VE-type"); (*ve).instr_len as i32 } }
}

unsafe fn __halt(irq_disabled: bool) -> u64 { let mut a: tdx_module_args = core::mem::zeroed(); a.r10=TDX_HYPERCALL_STANDARD; a.r11=hcall_func(EXIT_REASON_HLT); a.r12=irq_disabled as u64; __tdx_hypercall(&mut a) }
unsafe fn handle_halt(ve: *mut ve_info) -> i32 { let disabled=irqs_disabled(); if WARN_ONCE(!disabled, "HLT emulation with IRQs enabled") { return -EIO; } if __halt(disabled)!=0 { return -EIO; } ve_instr_len(ve) }
pub unsafe extern "C" fn tdx_halt() { if __halt(false)!=0 { WARN_ONCE(true, "HLT instruction emulation failed"); } }
unsafe fn tdx_safe_halt() { tdx_halt(); raw_local_irq_enable(); }

unsafe fn read_msr(regs: *mut pt_regs, ve: *mut ve_info) -> i32 { let mut a: tdx_module_args=core::mem::zeroed(); a.r10=TDX_HYPERCALL_STANDARD; a.r11=hcall_func(EXIT_REASON_MSR_READ); a.r12=(*regs).cx; if __tdx_hypercall(&mut a)!=0{return -EIO;} (*regs).ax=lower_32_bits(a.r11); (*regs).dx=upper_32_bits(a.r11); ve_instr_len(ve) }
unsafe fn write_msr(regs: *mut pt_regs, ve: *mut ve_info) -> i32 { let mut a: tdx_module_args=core::mem::zeroed(); a.r10=TDX_HYPERCALL_STANDARD; a.r11=hcall_func(EXIT_REASON_MSR_WRITE); a.r12=(*regs).cx; a.r13=((*regs).dx<<32)|(*regs).ax; if __tdx_hypercall(&mut a)!=0{-EIO}else{ve_instr_len(ve)} }
unsafe fn handle_cpuid(regs: *mut pt_regs, ve: *mut ve_info) -> i32 { if (*regs).ax<0x40000000||(*regs).ax>0x4fffffff {(*regs).ax=0;(*regs).bx=0;(*regs).cx=0;(*regs).dx=0;return ve_instr_len(ve);} let mut a:tdx_module_args=core::mem::zeroed();a.r10=TDX_HYPERCALL_STANDARD;a.r11=hcall_func(EXIT_REASON_CPUID);a.r12=(*regs).ax;a.r13=(*regs).cx;if __tdx_hypercall(&mut a)!=0{-EIO}else{(*regs).ax=a.r12;(*regs).bx=a.r13;(*regs).cx=a.r14;(*regs).dx=a.r15;ve_instr_len(ve)} }

unsafe fn handle_io(regs: *mut pt_regs, ve: *mut ve_info) -> i32 { let q=(*ve).exit_qual as u32; if ve_is_io_string(q){return -EIO;} let size=ve_get_io_size(q) as i32; let port=ve_get_port_num(q) as i32; let mut a:tdx_module_args=core::mem::zeroed();a.r10=TDX_HYPERCALL_STANDARD;a.r11=hcall_func(EXIT_REASON_IO_INSTRUCTION);a.r12=size as u64;a.r13=if ve_is_io_in(q){PORT_READ}else{PORT_WRITE};a.r14=port as u64;if __tdx_hypercall(&mut a)!=0{-EIO}else{if ve_is_io_in(q){(*regs).ax=a.r11;}ve_instr_len(ve)} }

pub unsafe extern "C" fn tdx_get_ve_info(ve:*mut ve_info){let mut a:tdx_module_args=core::mem::zeroed();tdcall(TDG_VP_VEINFO_GET,&mut a);(*ve).exit_reason=a.rcx;(*ve).exit_qual=a.rdx;(*ve).gla=a.r8;(*ve).gpa=a.r9;(*ve).instr_len=lower_32_bits(a.r10);(*ve).instr_info=upper_32_bits(a.r10);}
pub unsafe extern "C" fn tdx_early_handle_ve(regs:*mut pt_regs)->bool{let mut ve:ve_info=core::mem::zeroed();tdx_get_ve_info(&mut ve);if ve.exit_reason!=EXIT_REASON_IO_INSTRUCTION{return false;}let n=handle_io(regs,&mut ve);if n<0{return false;}(*regs).ip+=n as u64;true}
pub unsafe extern "C" fn tdx_handle_virt_exception(regs:*mut pt_regs,ve:*mut ve_info)->bool{let n=match (*ve).exit_reason{EXIT_REASON_HLT=>handle_halt(ve),EXIT_REASON_MSR_READ=>read_msr(regs,ve),EXIT_REASON_MSR_WRITE=>write_msr(regs,ve),EXIT_REASON_CPUID=>handle_cpuid(regs,ve),EXIT_REASON_IO_INSTRUCTION=>handle_io(regs,ve),_=>-EIO};if n<0{return false;}(*regs).ip+=n as u64;true}

unsafe fn tdx_tlb_flush_required(private: bool)->bool{!private}
unsafe fn tdx_cache_flush_required()->bool{true}
unsafe fn tdx_enc_status_change_prepare(vaddr:usize,numpages:i32,enc:bool)->i32{if enc&&!tdx_enc_status_changed(vaddr,numpages,enc){-EIO}else{0}}
unsafe fn tdx_enc_status_change_finish(_vaddr:usize,numpages:i32,_enc:bool)->i32{atomic_long_add(numpages as isize,&mut NR_SHARED);0}
unsafe fn tdx_enc_status_changed(_vaddr:usize,_numpages:i32,_enc:bool)->bool{true}
unsafe fn tdx_kexec_begin(){if !set_memory_enc_stop_conversion(){pr_warn("Failed to stop shared<->private conversions");}}
unsafe fn tdx_kexec_finish(){}

pub unsafe extern "C" fn tdx_early_init(){let mut cc_mask=0;let mut eax=0;let mut sig=[0u32;3];cpuid_count(TDX_CPUID_LEAF_ID,0,&mut eax,&mut sig[0],&mut sig[2],&mut sig[1]);if memcmp(TDX_IDENT.as_ptr(),sig.as_ptr(),sig.len()*4)!=0{return;}setup_force_cpu_cap(X86_FEATURE_TDX_GUEST);setup_force_cpu_cap(X86_FEATURE_TSC_RELIABLE);cc_vendor=CC_VENDOR_INTEL;tdx_setup(&mut cc_mask);cc_set_mask(cc_mask);physical_mask&=cc_mask-1;tdx_announce();}
unsafe fn tdx_announce(){let mut a:tdx_module_args=core::mem::zeroed();let mut c=0;pr_info("Guest detected");tdcall(TDG_VP_INFO,&mut a);tdx_dump_attributes(a.rdx);tdg_vm_rd(TDCS_TD_CTLS,&mut c);tdx_dump_td_ctls(c);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
