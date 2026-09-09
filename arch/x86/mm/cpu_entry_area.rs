// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation.

static DEFINE_PER_CPU_PAGE_ALIGNED!(struct entry_stack_page, entry_stack_storage);

#[cfg(target_arch = "x86_64")]
static DEFINE_PER_CPU_PAGE_ALIGNED!(struct exception_stacks, exception_stacks);
#[cfg(target_arch = "x86_64")]
DEFINE_PER_CPU!(struct cea_exception_stacks *, cea_exception_stacks);

#[cfg(target_arch = "x86_64")]
static DEFINE_PER_CPU_READ_MOSTLY!(unsigned long, _cea_offset);

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn cea_offset(cpu: c_uint) -> c_uint {
    per_cpu!(_cea_offset, cpu)
}

#[cfg(target_arch = "x86_64")]
#[init]
unsafe fn init_cea_offsets() {
    let max_cea: c_uint;
    let mut i: c_uint;
    let mut j: c_uint;

    if !kaslr_enabled() {
        for_each_possible_cpu!(i) {
            per_cpu!(_cea_offset, i) = i;
        }
        return;
    }

    max_cea = (CPU_ENTRY_AREA_MAP_SIZE - PAGE_SIZE) / CPU_ENTRY_AREA_SIZE;

    /* O(sodding terrible) */
    for_each_possible_cpu!(i) {
        let mut cea: c_uint;

        loop {
            cea = get_random_u32_below(max_cea);

            for_each_possible_cpu!(j) {
                if cea_offset(j) == cea {
                    continue;
                }
                if i == j {
                    break;
                }
            }

            // C's goto `again` retries the random selection when a collision
            // is found; the surrounding kernel macro supplies that state.
            per_cpu!(_cea_offset, i) = cea;
            break;
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
DECLARE_PER_CPU_PAGE_ALIGNED!(struct doublefault_stack, doublefault_stack);

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
unsafe fn cea_offset(cpu: c_uint) -> c_uint {
    cpu
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
unsafe fn init_cea_offsets() {}

/* Is called from entry code, so must be noinstr */
#[noinstr]
unsafe fn get_cpu_entry_area(cpu: c_int) -> *mut struct cpu_entry_area {
    let va: c_ulong = CPU_ENTRY_AREA_PER_CPU + cea_offset(cpu as c_uint) * CPU_ENTRY_AREA_SIZE;
    BUILD_BUG_ON!(core::mem::size_of::<struct cpu_entry_area>() % PAGE_SIZE != 0);

    va as *mut struct cpu_entry_area
}
EXPORT_SYMBOL!(get_cpu_entry_area);

unsafe fn cea_set_pte(cea_vaddr: *mut c_void, pa: phys_addr_t, flags: pgprot_t) {
    let va = cea_vaddr as c_ulong;
    let mut pte = pfn_pte(pa >> PAGE_SHIFT, flags);

    /*
     * The cpu_entry_area is shared between the user and kernel
     * page tables.  All of its ptes can safely be global.
     * _PAGE_GLOBAL gets reused to help indicate PROT_NONE for
     * non-present PTEs, so be careful not to set it in that
     * case to avoid confusion.
     */
    if boot_cpu_has(X86_FEATURE_PGE) && (pgprot_val(flags) & _PAGE_PRESENT) != 0 {
        pte = pte_set_flags(pte, _PAGE_GLOBAL);
    }

    set_pte_vaddr(va, pte);
}

#[init]
unsafe fn cea_map_percpu_pages(mut cea_vaddr: *mut c_void, mut ptr: *mut c_void,
                               mut pages: c_int, prot: pgprot_t) {
    while pages != 0 {
        cea_set_pte(cea_vaddr, per_cpu_ptr_to_phys(ptr), prot);
        pages -= 1;
        cea_vaddr = (cea_vaddr as usize).wrapping_add(PAGE_SIZE as usize) as *mut c_void;
        ptr = (ptr as usize).wrapping_add(PAGE_SIZE as usize) as *mut c_void;
    }
}

#[init]
unsafe fn percpu_setup_debug_store(cpu: c_uint) {
    #[cfg(CONFIG_CPU_SUP_INTEL)]
    {
        let mut npages: c_uint;
        let mut cea: *mut c_void;

        if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL {
            return;
        }

        cea = core::ptr::addr_of_mut!((*get_cpu_entry_area(cpu as c_int)).cpu_debug_store) as *mut c_void;
        npages = core::mem::size_of::<struct debug_store>() as c_uint / PAGE_SIZE;
        BUILD_BUG_ON!(core::mem::size_of::<struct debug_store>() % PAGE_SIZE != 0);
        cea_map_percpu_pages(cea, core::ptr::addr_of_mut!(per_cpu!(cpu_debug_store, cpu)) as *mut c_void,
                             npages as c_int, PAGE_KERNEL);

        cea = core::ptr::addr_of_mut!((*get_cpu_entry_area(cpu as c_int)).cpu_debug_buffers) as *mut c_void;
        /*
         * Force the population of PMDs for not yet allocated per cpu
         * memory like debug store buffers.
         */
        npages = core::mem::size_of::<struct debug_store_buffers>() as c_uint / PAGE_SIZE;
        while npages != 0 {
            cea_set_pte(cea, 0, PAGE_NONE);
            npages -= 1;
            cea = (cea as usize).wrapping_add(PAGE_SIZE as usize) as *mut c_void;
        }
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn percpu_setup_exception_stacks(cpu: c_uint) {
    let estacks = per_cpu_ptr(&exception_stacks, cpu);
    let cea = get_cpu_entry_area(cpu as c_int);
    let mut npages: c_uint;

    BUILD_BUG_ON!(core::mem::size_of::<exception_stacks>() % PAGE_SIZE != 0);
    per_cpu!(cea_exception_stacks, cpu) = core::ptr::addr_of_mut!((*cea).estacks);

    /*
     * The exceptions stack mappings in the per cpu area are protected
     * by guard pages so each stack must be mapped separately. DB2 is
     * not mapped; it just exists to catch triple nesting of #DB.
     */
    macro_rules! cea_map_stack { ($name:ident) => {
        npages = core::mem::size_of::<struct $name##_stack>() as c_uint / PAGE_SIZE;
        cea_map_percpu_pages(core::ptr::addr_of_mut!((*cea).estacks.$name##_stack) as *mut c_void,
                             core::ptr::addr_of_mut!((*estacks).$name##_stack) as *mut c_void,
                             npages as c_int, PAGE_KERNEL);
    } }
    cea_map_stack!(DF);
    cea_map_stack!(NMI);
    cea_map_stack!(DB);
    cea_map_stack!(MCE);

    if IS_ENABLED!(CONFIG_AMD_MEM_ENCRYPT) && cc_platform_has(CC_ATTR_GUEST_STATE_ENCRYPT) {
        cea_map_stack!(VC);
        cea_map_stack!(VC2);
    }
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn percpu_setup_exception_stacks(cpu: c_uint) {
    let cea = get_cpu_entry_area(cpu as c_int);
    cea_map_percpu_pages(core::ptr::addr_of_mut!((*cea).doublefault_stack) as *mut c_void,
                         core::ptr::addr_of_mut!(per_cpu!(doublefault_stack, cpu)) as *mut c_void,
                         1, PAGE_KERNEL);
}

/* Setup the fixmap mappings only once per-processor */
#[init]
unsafe fn setup_cpu_entry_area(cpu: c_uint) {
    let cea = get_cpu_entry_area(cpu as c_int);
    #[cfg(target_arch = "x86_64")]
    let (gdt_prot, tss_prot) = (PAGE_KERNEL_RO, PAGE_KERNEL_RO);
    #[cfg(not(target_arch = "x86_64"))]
    let (gdt_prot, tss_prot) = (PAGE_KERNEL, PAGE_KERNEL);

    kasan_populate_shadow_for_vaddr(cea, CPU_ENTRY_AREA_SIZE, early_cpu_to_node(cpu));
    cea_set_pte(core::ptr::addr_of_mut!((*cea).gdt) as *mut c_void, get_cpu_gdt_paddr(cpu), gdt_prot);
    cea_map_percpu_pages(core::ptr::addr_of_mut!((*cea).entry_stack_page) as *mut c_void,
                         per_cpu_ptr(&entry_stack_storage, cpu), 1, PAGE_KERNEL);

    /*
     * The Intel SDM says (Volume 3, 7.2.1):
     *
     *  Avoid placing a page boundary in the part of the TSS that the
     *  processor reads during a task switch (the first 104 bytes). The
     *  processor may not correctly perform address translations if a
     *  boundary occurs in this area. During a task switch, the processor
     *  reads and writes into the first 104 bytes of each TSS (using
     *  contiguous physical addresses beginning with the physical address
     *  of the first byte of the TSS). So, after TSS access begins, if
     *  part of the 104 bytes is not physically contiguous, the processor
     *  will access incorrect information without generating a page-fault
     *  exception.
     *
     * There are also a lot of errata involving the TSS spanning a page
     * boundary.  Assert that we're not doing that.
     */
    BUILD_BUG_ON!((offset_of!(struct tss_struct, x86_tss) ^ offsetofend!(struct tss_struct, x86_tss)) & PAGE_MASK);
    BUILD_BUG_ON!(core::mem::size_of::<struct tss_struct>() % PAGE_SIZE != 0);
    /*
     * VMX changes the host TR limit to 0x67 after a VM exit. This is
     * okay, since 0x67 covers the size of struct x86_hw_tss. Make sure
     * that this is correct.
     */
    BUILD_BUG_ON!(offset_of!(struct tss_struct, x86_tss) != 0);
    BUILD_BUG_ON!(core::mem::size_of::<struct x86_hw_tss>() != 0x68);

    cea_map_percpu_pages(core::ptr::addr_of_mut!((*cea).tss) as *mut c_void,
                         per_cpu_ptr(&cpu_tss_rw, cpu),
                         (core::mem::size_of::<struct tss_struct>() / PAGE_SIZE) as c_int,
                         tss_prot);

    #[cfg(not(target_arch = "x86_64"))]
    { per_cpu!(cpu_entry_area, cpu) = cea; }
    percpu_setup_exception_stacks(cpu);
    percpu_setup_debug_store(cpu);
}

#[init]
unsafe fn setup_cpu_entry_area_ptes() {
    #[cfg(not(target_arch = "x86_64"))]
    {
        let mut start: c_ulong;
        let end: c_ulong;

        /* The +1 is for the readonly IDT: */
        BUILD_BUG_ON!((CPU_ENTRY_AREA_PAGES + 1) * PAGE_SIZE != CPU_ENTRY_AREA_MAP_SIZE);
        BUG_ON!(CPU_ENTRY_AREA_BASE & !PMD_MASK);

        start = CPU_ENTRY_AREA_BASE;
        end = start + CPU_ENTRY_AREA_MAP_SIZE;

        /* Careful here: start + PMD_SIZE might wrap around */
        while start < end && start >= CPU_ENTRY_AREA_BASE {
            populate_extra_pte(start);
            start = start.wrapping_add(PMD_SIZE);
        }
    }
}

#[init]
unsafe fn setup_cpu_entry_areas() {
    let mut cpu: c_uint;

    init_cea_offsets();
    setup_cpu_entry_area_ptes();
    for_each_possible_cpu!(cpu) {
        setup_cpu_entry_area(cpu);
    }

    /*
     * This is the last essential update to swapper_pgdir which needs
     * to be synchronized to initial_page_table on 32bit.
     */
    sync_initial_page_table();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
