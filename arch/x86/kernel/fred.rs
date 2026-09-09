/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations supplied by the kernel's other Rust/C translation units. */
extern "C" {
    fn loadsegment_ss(value: usize);
    fn wrmsrq(msr: usize, value: usize);
    fn cr4_set_bits(bits: usize);
    fn idt_invalidate();
    fn setup_clear_cpu_cap(cap: usize);
    fn __this_cpu_read_fred_rsp0() -> usize;
    fn __this_cpu_ist_top_va(stack: usize) -> usize;
}

const FRED_DB_STACK_LEVEL: usize = 1;
const FRED_NMI_STACK_LEVEL: usize = 2;
const FRED_MC_STACK_LEVEL: usize = 2;
/*
 * #DF is the highest level because a #DF means "something went wrong
 * *while delivering an exception*." The number of cases for which that
 * can happen with FRED is drastically reduced and basically amounts to
 * "the stack you pointed me to is broken." Thus, always change stacks
 * on #DF, which means it should be at the highest level.
 */
const FRED_DF_STACK_LEVEL: usize = 3;

#[inline]
const fn fred_stklvl(vector: usize, level: usize) -> usize {
    level << (2 * vector)
}

/* DEFINE_PER_CPU(unsigned long, fred_rsp0); */
#[no_mangle]
pub static mut fred_rsp0: usize = 0;

pub unsafe fn cpu_init_fred_exceptions() {
    /*
     * If a kernel event is delivered before a CPU goes to user level for
     * the first time, its SS is NULL thus NULL is pushed into the SS field
     * of the FRED stack frame.  But before ERETS is executed, the CPU may
     * context switch to another task and go to user level.  Then when the
     * CPU comes back to kernel mode, SS is changed to __KERNEL_DS.  Later
     * when ERETS is executed to return from the kernel event handler, a #GP
     * fault is generated because SS doesn't match the SS saved in the FRED
     * stack frame.
     *
     * Initialize SS to __KERNEL_DS when enabling FRED to avoid such #GPs.
     */
    loadsegment_ss(__KERNEL_DS);

    wrmsrq(
        MSR_IA32_FRED_CONFIG,
        /* Reserve for CALL emulation */
        FRED_CONFIG_REDZONE |
            FRED_CONFIG_INT_STKLVL(0) |
            FRED_CONFIG_ENTRYPOINT(asm_fred_entrypoint_user),
    );

    wrmsrq(MSR_IA32_FRED_STKLVLS, 0);

    /*
     * Ater a CPU offline/online cycle, the FRED RSP0 MSR should be
     * resynchronized with its per-CPU cache.
     */
    wrmsrq(MSR_IA32_FRED_RSP0, __this_cpu_read_fred_rsp0());

    wrmsrq(MSR_IA32_FRED_RSP1, 0);
    wrmsrq(MSR_IA32_FRED_RSP2, 0);
    wrmsrq(MSR_IA32_FRED_RSP3, 0);

    /* Enable FRED */
    cr4_set_bits(X86_CR4_FRED);
    /* Any further IDT use is a bug */
    idt_invalidate();

    /* Use int $0x80 for 32-bit system calls in FRED mode */
    setup_clear_cpu_cap(X86_FEATURE_SYSFAST32);
    setup_clear_cpu_cap(X86_FEATURE_SYSCALL32);
}

/* Must be called after setup_cpu_entry_areas() */
pub unsafe fn cpu_init_fred_rsps() {
    /*
     * The purpose of separate stacks for NMI, #DB and #MC *in the kernel*
     * (remember that user space faults are always taken on stack level 0)
     * is to avoid overflowing the kernel stack.
     */
    wrmsrq(
        MSR_IA32_FRED_STKLVLS,
        fred_stklvl(X86_TRAP_DB, FRED_DB_STACK_LEVEL)
            | fred_stklvl(X86_TRAP_NMI, FRED_NMI_STACK_LEVEL)
            | fred_stklvl(X86_TRAP_MC, FRED_MC_STACK_LEVEL)
            | fred_stklvl(X86_TRAP_DF, FRED_DF_STACK_LEVEL),
    );

    /* The FRED equivalents to IST stacks... */
    wrmsrq(MSR_IA32_FRED_RSP1, __this_cpu_ist_top_va(DB));
    wrmsrq(MSR_IA32_FRED_RSP2, __this_cpu_ist_top_va(NMI));
    wrmsrq(MSR_IA32_FRED_RSP3, __this_cpu_ist_top_va(DF));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
