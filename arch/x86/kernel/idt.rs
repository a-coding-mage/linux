// SPDX-License-Identifier: GPL-2.0-only
/* Interrupt descriptor table related code */

const DPL0: u8 = 0x0;
const DPL3: u8 = 0x3;
const DEFAULT_STACK: u8 = 0;

macro_rules! G {
    ($vector:expr, $addr:expr, $ist:expr, $type:expr, $dpl:expr, $segment:expr) => {
        idt_data { vector: $vector, bits: idt_bits { ist: $ist, type_: $type, dpl: $dpl, p: 1 }, addr: $addr, segment: $segment }
    };
}
macro_rules! INTG { ($vector:expr, $addr:expr) => { G!($vector, $addr, DEFAULT_STACK, GATE_INTERRUPT, DPL0, __KERNEL_CS) }; }
macro_rules! SYSG { ($vector:expr, $addr:expr) => { G!($vector, $addr, DEFAULT_STACK, GATE_INTERRUPT, DPL3, __KERNEL_CS) }; }
#[cfg(target_arch = "x86_64")]
macro_rules! ISTG { ($vector:expr, $addr:expr, $ist:expr) => { G!($vector, $addr, $ist + 1, GATE_INTERRUPT, DPL0, __KERNEL_CS) }; }
#[cfg(not(target_arch = "x86_64"))]
macro_rules! ISTG { ($vector:expr, $addr:expr, $ist:expr) => { INTG!($vector, $addr) }; }
macro_rules! TSKG { ($vector:expr, $gdt:expr) => { G!($vector, core::ptr::null(), DEFAULT_STACK, GATE_TASK, DPL0, $gdt << 3) }; }

const IDT_TABLE_SIZE: usize = IDT_ENTRIES * core::mem::size_of::<gate_desc>();
static mut idt_setup_done: bool = false;

static early_idts: &[idt_data] = &[
    INTG!(X86_TRAP_DB, asm_exc_debug), SYSG!(X86_TRAP_BP, asm_exc_int3),
    #[cfg(all(not(target_arch = "x86_64"), feature = "CONFIG_X86_32"))]
    INTG!(X86_TRAP_PF, asm_exc_page_fault),
    #[cfg(feature = "CONFIG_INTEL_TDX_GUEST")]
    INTG!(X86_TRAP_VE, asm_exc_virtualization_exception),
];

static def_idts: &[idt_data] = &[
    INTG!(X86_TRAP_DE, asm_exc_divide_error), ISTG!(X86_TRAP_NMI, asm_exc_nmi, IST_INDEX_NMI),
    INTG!(X86_TRAP_BR, asm_exc_bounds), INTG!(X86_TRAP_UD, asm_exc_invalid_op),
    INTG!(X86_TRAP_NM, asm_exc_device_not_available), INTG!(X86_TRAP_OLD_MF, asm_exc_coproc_segment_overrun),
    INTG!(X86_TRAP_TS, asm_exc_invalid_tss), INTG!(X86_TRAP_NP, asm_exc_segment_not_present),
    INTG!(X86_TRAP_SS, asm_exc_stack_segment), INTG!(X86_TRAP_GP, asm_exc_general_protection),
    INTG!(X86_TRAP_SPURIOUS, asm_exc_spurious_interrupt_bug), INTG!(X86_TRAP_MF, asm_exc_coprocessor_error),
    INTG!(X86_TRAP_AC, asm_exc_alignment_check), INTG!(X86_TRAP_XF, asm_exc_simd_coprocessor_error),
    #[cfg(feature = "CONFIG_X86_32")] TSKG!(X86_TRAP_DF, GDT_ENTRY_DOUBLEFAULT_TSS),
    #[cfg(not(feature = "CONFIG_X86_32"))] ISTG!(X86_TRAP_DF, asm_exc_double_fault, IST_INDEX_DF),
    ISTG!(X86_TRAP_DB, asm_exc_debug, IST_INDEX_DB),
    #[cfg(feature = "CONFIG_X86_MCE")] ISTG!(X86_TRAP_MC, asm_exc_machine_check, IST_INDEX_MCE),
    #[cfg(feature = "CONFIG_X86_CET")] INTG!(X86_TRAP_CP, asm_exc_control_protection),
    #[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")] ISTG!(X86_TRAP_VC, asm_exc_vmm_communication, IST_INDEX_VC),
    SYSG!(X86_TRAP_OF, asm_exc_overflow),
];

static ia32_idt: &[idt_data] = &[
    #[cfg(feature = "CONFIG_IA32_EMULATION")] SYSG!(IA32_SYSCALL_VECTOR, asm_int80_emulation),
    #[cfg(all(not(feature = "CONFIG_IA32_EMULATION"), feature = "CONFIG_X86_32"))] SYSG!(IA32_SYSCALL_VECTOR, entry_INT80_32),
];

static apic_idts: &[idt_data] = &[
    #[cfg(feature = "CONFIG_SMP")] INTG!(RESCHEDULE_VECTOR, asm_sysvec_reschedule_ipi),
    #[cfg(feature = "CONFIG_SMP")] INTG!(CALL_FUNCTION_VECTOR, asm_sysvec_call_function),
    #[cfg(feature = "CONFIG_SMP")] INTG!(CALL_FUNCTION_SINGLE_VECTOR, asm_sysvec_call_function_single),
    #[cfg(feature = "CONFIG_SMP")] INTG!(REBOOT_VECTOR, asm_sysvec_reboot),
    #[cfg(feature = "CONFIG_X86_THERMAL_VECTOR")] INTG!(THERMAL_APIC_VECTOR, asm_sysvec_thermal),
    #[cfg(feature = "CONFIG_X86_MCE_THRESHOLD")] INTG!(THRESHOLD_APIC_VECTOR, asm_sysvec_threshold),
    #[cfg(feature = "CONFIG_X86_MCE_AMD")] INTG!(DEFERRED_ERROR_VECTOR, asm_sysvec_deferred_error),
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")] INTG!(LOCAL_TIMER_VECTOR, asm_sysvec_apic_timer_interrupt),
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")] INTG!(X86_PLATFORM_IPI_VECTOR, asm_sysvec_x86_platform_ipi),
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_KVM"))] INTG!(POSTED_INTR_VECTOR, asm_sysvec_kvm_posted_intr_ipi),
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_KVM"))] INTG!(POSTED_INTR_WAKEUP_VECTOR, asm_sysvec_kvm_posted_intr_wakeup_ipi),
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_KVM"))] INTG!(POSTED_INTR_NESTED_VECTOR, asm_sysvec_kvm_posted_intr_nested_ipi),
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_GUEST_PERF_EVENTS"))] INTG!(PERF_GUEST_MEDIATED_PMI_VECTOR, asm_sysvec_perf_guest_mediated_pmi_handler),
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_IRQ_WORK"))] INTG!(IRQ_WORK_VECTOR, asm_sysvec_irq_work),
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")] INTG!(SPURIOUS_APIC_VECTOR, asm_sysvec_spurious_apic_interrupt),
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")] INTG!(ERROR_APIC_VECTOR, asm_sysvec_error_interrupt),
    #[cfg(all(feature = "CONFIG_X86_LOCAL_APIC", feature = "CONFIG_X86_POSTED_MSI"))] INTG!(POSTED_MSI_NOTIFICATION_VECTOR, asm_sysvec_posted_msi_notification),
];

static mut idt_table: [gate_desc; IDT_ENTRIES] = [gate_desc::ZERO; IDT_ENTRIES];
static mut idt_descr: desc_ptr = desc_ptr { size: (IDT_TABLE_SIZE - 1) as _, address: unsafe { &idt_table as *const _ as usize as _ } };

pub unsafe fn load_current_idt() { lockdep_assert_irqs_disabled(); load_idt(&idt_descr); }

#[cfg(feature = "CONFIG_X86_F00F_BUG")]
pub unsafe fn idt_is_f00f_address(address: usize) -> bool { ((address.wrapping_sub(idt_descr.address as usize)) >> 3) == 6 }

unsafe fn idt_setup_from_table(idt: *mut gate_desc, mut t: *const idt_data, mut size: i32, sys: bool) {
    let mut desc = core::mem::MaybeUninit::<gate_desc>::uninit();
    while size > 0 { idt_init_desc(desc.as_mut_ptr(), t); write_idt_entry(idt, (*t).vector, desc.as_ptr()); if sys { set_bit((*t).vector, system_vectors); } t = t.add(1); size -= 1; }
}
unsafe fn set_intr_gate(n: u32, addr: *const core::ffi::c_void) { let mut data = core::mem::MaybeUninit::<idt_data>::uninit(); init_idt_data(data.as_mut_ptr(), n, addr); idt_setup_from_table(idt_table.as_mut_ptr(), data.as_ptr(), 1, false); }

pub unsafe fn idt_setup_early_traps() { idt_setup_from_table(idt_table.as_mut_ptr(), early_idts.as_ptr(), early_idts.len() as _, true); load_idt(&idt_descr); }
pub unsafe fn idt_setup_traps() { idt_setup_from_table(idt_table.as_mut_ptr(), def_idts.as_ptr(), def_idts.len() as _, true); if ia32_enabled() { idt_setup_from_table(idt_table.as_mut_ptr(), ia32_idt.as_ptr(), ia32_idt.len() as _, true); } }

#[cfg(target_arch = "x86_64")]
static early_pf_idts: &[idt_data] = &[INTG!(X86_TRAP_PF, asm_exc_page_fault)];
#[cfg(target_arch = "x86_64")]
pub unsafe fn idt_setup_early_pf() { idt_setup_from_table(idt_table.as_mut_ptr(), early_pf_idts.as_ptr(), early_pf_idts.len() as _, true); }

#[cfg(all(feature = "CONFIG_KVM_INTEL", not(target_arch = "x86_64")))]
pub unsafe fn idt_entry_from_kvm(vector: u32) { idt_do_interrupt_irqoff(gate_offset(idt_table.as_mut_ptr().add(vector as usize))); }

unsafe fn idt_map_in_cea() { cea_set_pte(CPU_ENTRY_AREA_RO_IDT_VADDR, __pa_symbol(idt_table.as_ptr()), PAGE_KERNEL_RO); idt_descr.address = CPU_ENTRY_AREA_RO_IDT; }

pub unsafe fn idt_setup_apic_and_irq_gates() {
    let mut i = FIRST_EXTERNAL_VECTOR; idt_setup_from_table(idt_table.as_mut_ptr(), apic_idts.as_ptr(), apic_idts.len() as _, true);
    for_each_clear_bit_from(&mut i, system_vectors, FIRST_SYSTEM_VECTOR) { let entry = irq_entries_start.add((IDT_ALIGN * (i - FIRST_EXTERNAL_VECTOR)) as usize); set_intr_gate(i, entry as _); }
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    for_each_clear_bit_from(&mut i, system_vectors, NR_VECTORS) { let entry = spurious_entries_start.add((IDT_ALIGN * (i - FIRST_SYSTEM_VECTOR)) as usize); set_intr_gate(i, entry as _); }
    idt_map_in_cea(); load_idt(&idt_descr); set_memory_ro(&mut idt_table as *mut _ as usize, 1); idt_setup_done = true;
}

pub unsafe fn idt_setup_early_handler() { let mut i = 0; while i < NUM_EXCEPTION_VECTORS { set_intr_gate(i, early_idt_handler_array[i] as _); i += 1; } #[cfg(feature = "CONFIG_X86_32")] while i < NR_VECTORS { set_intr_gate(i, early_ignore_irq as _); i += 1; } load_idt(&idt_descr); }
pub unsafe fn idt_invalidate() { let idt = desc_ptr { address: 0, size: 0 }; load_idt(&idt); }
pub unsafe fn idt_install_sysvec(n: u32, function: *const core::ffi::c_void) { if WARN_ON(n < FIRST_SYSTEM_VECTOR) || WARN_ON(idt_setup_done) { return; } if !WARN_ON(test_and_set_bit(n, system_vectors)) { set_intr_gate(n, function); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
