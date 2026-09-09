// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common prep/pmac/chrp boot and setup code.
 */

// C includes provide the external kernel and architecture symbols used below.

// #define DBG(fmt...)

extern "C" {
    fn bootx_init(r4: libc::c_ulong, phys: libc::c_ulong);
}

pub static mut boot_cpuid_phys: libc::c_int = 0;
// EXPORT_SYMBOL_GPL(boot_cpuid_phys);

pub static mut smp_hw_index: [libc::c_int; NR_CPUS] = [0; NR_CPUS];
// EXPORT_SYMBOL(smp_hw_index);

pub static mut DMA_MODE_READ: libc::c_uint = 0;
pub static mut DMA_MODE_WRITE: libc::c_uint = 0;

// EXPORT_SYMBOL(DMA_MODE_READ);
// EXPORT_SYMBOL(DMA_MODE_WRITE);

/*
 * This is run before start_kernel(), the kernel has been relocated
 * and we are running with enough of the MMU enabled to have our
 * proper kernel virtual addresses
 *
 * We do the initial parsing of the flat device-tree and prepares
 * for the MMU to be fully initialized.
 */
pub unsafe fn machine_init(dt_ptr: u64) {
    let addr: *mut u32 = patch_site_addr(&patch__memset_nocache) as *mut u32;
    let mut insn: ppc_inst_t = core::mem::zeroed();

    /* Configure static keys first, now that we're relocated. */
    setup_feature_keys();

    early_ioremap_init();

    /* Enable early debugging if any specified (see udbg.h) */
    udbg_early_init();

    patch_instruction_site(&patch__memcpy_nocache, ppc_inst(PPC_RAW_NOP()));

    create_cond_branch(&mut insn, addr, branch_target(addr), 0x820000);
    patch_instruction(addr, insn); /* replace b by bne cr0 */

    /* Do some early initialization based on the flat device tree */
    early_init_devtree(__va(dt_ptr));

    early_init_mmu();

    setup_kdump_trampoline();
}

/* Checks "l2cr=xxxx" command-line option */
unsafe fn ppc_setup_l2cr(str_: *mut libc::c_char) -> libc::c_int {
    if cpu_has_feature(CPU_FTR_L2CR) {
        let val = simple_strtoul(str_, core::ptr::null_mut(), 0);
        printk(KERN_INFO, b"l2cr set to %lx\n\0".as_ptr(), val);
        _set_L2CR(0); /* force invalidate by disable cache */
        _set_L2CR(val); /* and enable it */
    }
    1
}
// __setup("l2cr=", ppc_setup_l2cr);

/* Checks "l3cr=xxxx" command-line option */
unsafe fn ppc_setup_l3cr(str_: *mut libc::c_char) -> libc::c_int {
    if cpu_has_feature(CPU_FTR_L3CR) {
        let val = simple_strtoul(str_, core::ptr::null_mut(), 0);
        printk(KERN_INFO, b"l3cr set to %lx\n\0".as_ptr(), val);
        _set_L3CR(val); /* and enable it */
    }
    1
}
// __setup("l3cr=", ppc_setup_l3cr);

unsafe fn ppc_init() -> libc::c_int {
    /* clear the progress line */
    if let Some(progress) = ppc_md.progress {
        progress(b"             \0".as_ptr(), 0xffff);
    }

    /* call platform init */
    if let Some(init) = ppc_md.init {
        init();
    }
    0
}
// arch_initcall(ppc_init);

unsafe fn alloc_stack() -> *mut libc::c_void {
    memblock_alloc_or_panic(THREAD_SIZE, THREAD_ALIGN)
}

pub unsafe fn irqstack_early_init() {
    if cfg!(CONFIG_VMAP_STACK) {
        return;
    }

    /* interrupt stacks must be in lowmem, we get that for free on ppc32
     * as the memblock is limited to lowmem by default */
    for_each_possible_cpu(|i: libc::c_uint| {
        softirq_ctx[i as usize] = alloc_stack();
        hardirq_ctx[i as usize] = alloc_stack();
    });
}

#[cfg(CONFIG_VMAP_STACK)]
pub static mut emergency_ctx: [*mut libc::c_void; NR_CPUS] = {
    let mut value = [core::ptr::null_mut(); NR_CPUS];
    value[0] = &raw mut init_stack as *mut _ as *mut libc::c_void;
    value
};

#[cfg(CONFIG_VMAP_STACK)]
pub unsafe fn emergency_stack_init() {
    for_each_possible_cpu(|i: libc::c_uint| {
        emergency_ctx[i as usize] = alloc_stack();
    });
}

#[cfg(CONFIG_BOOKE)]
pub unsafe fn exc_lvl_early_init() {
    for_each_possible_cpu(|i: libc::c_uint| {
        #[cfg(CONFIG_SMP)]
        let hw_cpu = get_hard_smp_processor_id(i);
        #[cfg(not(CONFIG_SMP))]
        let hw_cpu = 0;

        critirq_ctx[hw_cpu as usize] = alloc_stack();
        #[cfg(CONFIG_BOOKE)]
        {
            dbgirq_ctx[hw_cpu as usize] = alloc_stack();
            mcheckirq_ctx[hw_cpu as usize] = alloc_stack();
        }
    });
}

pub unsafe fn setup_power_save() {
    #[cfg(CONFIG_PPC_BOOK3S_32)]
    if cpu_has_feature(CPU_FTR_CAN_DOZE) || cpu_has_feature(CPU_FTR_CAN_NAP) {
        ppc_md.power_save = Some(ppc6xx_idle);
    }

    #[cfg(CONFIG_PPC_E500)]
    if cpu_has_feature(CPU_FTR_CAN_DOZE) || cpu_has_feature(CPU_FTR_CAN_NAP) {
        ppc_md.power_save = Some(e500_idle);
    }
}

pub unsafe fn initialize_cache_info() {
    /*
     * Set cache line size based on type of cpu as a default.
     * Systems with OF can look in the properties on the cpu node(s)
     * for a possibly more accurate value.
     */
    dcache_bsize = (*cur_cpu_spec).dcache_bsize;
    icache_bsize = (*cur_cpu_spec).icache_bsize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
