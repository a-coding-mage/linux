// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation.  Linux headers and architecture
// definitions are supplied by the surrounding kernel translation.

const PAGE_OFFSET: usize = 0;
const MAXMEM: usize = 0;
const IO_BITMAP_OFFSET_INVALID: u32 = 0;
const X86_EFLAGS_FIXED: usize = 0;
const __USER_DS: u16 = 0;
const __KERNEL_CS: u16 = 0;
const __KERNEL_DS: u16 = 0;
const __KERNEL_PERCPU: u16 = 0;
const GDT_ENTRY_DOUBLEFAULT_TSS: u32 = 0;

#[repr(C)]
pub struct pt_regs {
    pub ss: usize, pub __ssh: usize, pub sp: usize, pub flags: usize,
    pub cs: usize, pub __csh: usize, pub ip: usize, pub orig_ax: usize,
    pub gs: usize, pub __gsh: usize, pub fs: usize, pub __fsh: usize,
    pub es: usize, pub __esh: usize, pub ds: usize, pub __dsh: usize,
    pub ax: usize, pub bp: usize, pub di: usize, pub si: usize,
    pub dx: usize, pub cx: usize, pub bx: usize,
}

#[repr(C)]
pub struct doublefault_tss {
    pub ldt: u32,
    pub io_bitmap_base: u32,
    pub ip: usize, pub flags: usize,
    pub es: u16, pub cs: u16, pub ss: u16, pub ds: u16,
    pub fs: u16, pub gs: u16,
    pub __cr3: usize,
    pub sp: usize,
}

#[repr(C)]
pub struct doublefault_stack {
    pub tss: doublefault_tss,
    pub stack: [u8; 4096],
}

extern "C" {
    fn native_read_cr2() -> usize;
    fn force_reload_TR();
    fn smp_processor_id() -> u32;
    fn trace_hardirqs_off();
    fn exc_double_fault(regs: *mut pt_regs, error_code: usize, cr2: usize);
    fn panic(message: *const u8) -> !;
    fn __set_tss_desc(cpu: u32, entry: u32, tss: *mut doublefault_tss);
    fn get_cpu_entry_area(cpu: u32) -> *mut cpu_entry_area;
    fn asm_exc_double_fault();
    fn __pa_nodebug(value: usize) -> usize;
    static mut swapper_pg_dir: usize;
}

#[repr(C)]
pub struct cpu_entry_area {
    pub doublefault_stack: doublefault_stack,
}

#[macro_export]
macro_rules! ptr_ok {
    ($x:expr) => { (($x) > PAGE_OFFSET && ($x) < PAGE_OFFSET + MAXMEM) };
}

// The C TSS(x) macro expands to this_cpu_read(cpu_tss_rw.x86_tss.x).
macro_rules! TSS {
    ($x:ident) => {{ unsafe { this_cpu_read($x) } }};
}

extern "C" {
    fn this_cpu_read(field: usize) -> usize;
    fn this_cpu_write(field: usize, value: usize);
}

unsafe fn set_df_gdt_entry(cpu: u32) {
    // Set up doublefault TSS pointer in the GDT
    __set_tss_desc(
        cpu,
        GDT_ENTRY_DOUBLEFAULT_TSS,
        &mut (*get_cpu_entry_area(cpu)).doublefault_stack.tss,
    );
}

// Called by double_fault with CR0.TS and EFLAGS.NT cleared.  The CPU thinks
// we're running the doublefault task.  Cannot return.
pub unsafe extern "C" fn doublefault_shim() -> ! {
    let cr2: usize;
    let mut regs: pt_regs = core::mem::zeroed();

    // BUILD_BUG_ON(sizeof(struct doublefault_stack) != PAGE_SIZE);
    cr2 = native_read_cr2();

    /* Reset back to the normal kernel task. */
    force_reload_TR();
    set_df_gdt_entry(smp_processor_id());
    trace_hardirqs_off();

    // Fill in pt_regs.  The unwinder does not see this frame, so a nested
    // stack dump cannot unwind to the source of the double fault.
    regs.ss = TSS!(ss);
    regs.__ssh = 0;
    regs.sp = TSS!(sp);
    regs.flags = TSS!(flags);
    regs.cs = TSS!(cs);
    regs.__csh = 0;
    regs.ip = TSS!(ip);
    regs.orig_ax = 0;
    regs.gs = TSS!(gs);
    regs.__gsh = 0;
    regs.fs = TSS!(fs);
    regs.__fsh = 0;
    regs.es = TSS!(es);
    regs.__esh = 0;
    regs.ds = TSS!(ds);
    regs.__dsh = 0;
    regs.ax = TSS!(ax);
    regs.bp = TSS!(bp);
    regs.di = TSS!(di);
    regs.si = TSS!(si);
    regs.dx = TSS!(dx);
    regs.cx = TSS!(cx);
    regs.bx = TSS!(bx);

    exc_double_fault(&mut regs, 0, cr2);

    // x86_32 does not save the original CR3 anywhere on a task switch.
    panic(b"cannot return from double fault\n\0".as_ptr())
}

#[no_mangle]
pub static mut doublefault_stack_percpu: doublefault_stack = doublefault_stack {
    tss: doublefault_tss {
        // No sp0 or ss0 -- we never run CPL != 0 with this TSS active.
        ldt: 0,
        io_bitmap_base: IO_BITMAP_OFFSET_INVALID,
        ip: asm_exc_double_fault as usize,
        flags: X86_EFLAGS_FIXED,
        es: __USER_DS, cs: __KERNEL_CS, ss: __KERNEL_DS,
        ds: __USER_DS, fs: __KERNEL_PERCPU, gs: 0,
        __cr3: 0,
        sp: 0,
    },
    stack: [0; 4096],
};

pub unsafe extern "C" fn doublefault_init_cpu_tss() {
    let cpu = smp_processor_id();
    let cea = get_cpu_entry_area(cpu);

    // The linker cannot initialize percpu variables that point elsewhere in
    // percpu space.
    this_cpu_write(
        0,
        (&mut (*cea).doublefault_stack.stack as *mut [u8; 4096] as usize)
            .wrapping_add(1),
    );

    set_df_gdt_entry(cpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
