// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation:
// linux/linkage.h, linux/types.h, asm/desc.h, asm/init.h, asm/setup.h,
// asm/sev.h, and asm/trapnr.h.

/*
 * Data structures and code used for IDT setup in head_64.S. The bringup-IDT is
 * used until the idt_table takes over. On the boot CPU this happens in
 * x86_64_start_kernel(), on secondary CPUs in start_secondary(). In both cases
 * this happens in the functions called from head_64.S.
 *
 * The idt_table can't be used that early because all the code modifying it is
 * in idt.c and can be instrumented by tracing or KASAN, which both don't work
 * during early CPU bringup. Also the idt_table has the runtime vectors
 * configured which require certain CPU state to be setup already (like TSS),
 * which also hasn't happened yet in early CPU bringup.
 */
#[repr(C)]
static mut bringup_idt_table: [gate_desc; NUM_EXCEPTION_VECTORS] =
    [gate_desc::default(); NUM_EXCEPTION_VECTORS]; // __page_aligned_data

/* This may run while still in the direct mapping */
pub unsafe fn startup_64_load_idt(vc_handler: *mut core::ffi::c_void) {
    let desc = desc_ptr {
        address: rip_rel_ptr(bringup_idt_table.as_mut_ptr()) as usize,
        size: core::mem::size_of::<[gate_desc; NUM_EXCEPTION_VECTORS]>() - 1,
    };
    let mut data: idt_data = core::mem::zeroed();
    let mut idt_desc: gate_desc = core::mem::zeroed();

    /* @vc_handler is set only for a VMM Communication Exception */
    if !vc_handler.is_null() {
        init_idt_data(&mut data, X86_TRAP_VC, vc_handler);
        idt_init_desc(&mut idt_desc, &data);
        native_write_idt_entry(desc.address as *mut gate_desc, X86_TRAP_VC, &idt_desc);
    }

    native_load_idt(&desc);
}

/*
 * Setup boot CPU state needed before kernel switches to virtual addresses.
 */
pub unsafe fn startup_64_setup_gdt_idt() {
    let gp: *mut gdt_page = rip_rel_ptr((&raw mut gdt_page) as *mut gdt_page);
    let mut handler: *mut core::ffi::c_void = core::ptr::null_mut();

    let startup_gdt_descr = desc_ptr {
        address: (*gp).gdt.as_mut_ptr() as usize,
        size: GDT_SIZE - 1,
    };

    /* Load GDT */
    native_load_gdt(&startup_gdt_descr);

    /* New GDT is live - reload data segment registers */
    core::arch::asm!(
        "movl %eax, %ds",
        "movl %eax, %ss",
        "movl %eax, %es",
        in("eax") __KERNEL_DS,
        options(nostack, preserves_flags)
    );

    if cfg!(CONFIG_AMD_MEM_ENCRYPT) {
        handler = rip_rel_ptr((&raw mut vc_no_ghcb) as *mut _);
    }

    startup_64_load_idt(handler);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
