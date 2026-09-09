// SPDX-License-Identifier: GPL-2.0-only
/*
 * PPC64 code to handle Linux booting another kernel.
 *
 * Copyright (C) 2004-2005, IBM Corp.
 *
 * Created by: Milton D Miller II
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

pub unsafe fn machine_kexec_prepare(image: *mut kimage) -> c_int {
    let mut i: c_int;
    let (mut begin, mut end): (c_ulong, c_ulong);
    let (mut low, mut high): (c_ulong, c_ulong);
    let mut node: *mut device_node;
    let mut basep: *const c_ulong;
    let mut sizep: *const c_uint;

    /*
     * Since we use the kernel fault handlers and paging code to
     * handle the virtual mode, we must make sure no destination
     * overlaps kernel static data or bss.
     */
    i = 0;
    while i < (*image).nr_segments {
        if (*image).segment[i as usize].mem < __pa(_end) {
            return -ETXTBSY;
        }
        i += 1;
    }

    /* We also should not overwrite the tce tables */
    for_each_node_by_type!(node, "pci") {
        basep = of_get_property(node, "linux,tce-base", core::ptr::null_mut());
        sizep = of_get_property(node, "linux,tce-size", core::ptr::null_mut());
        if basep.is_null() || sizep.is_null() {
            continue;
        }
        low = *basep;
        high = low + (*sizep as c_ulong);
        i = 0;
        while i < (*image).nr_segments {
            begin = (*image).segment[i as usize].mem;
            end = begin + (*image).segment[i as usize].memsz;
            if begin < high && end > low {
                of_node_put(node);
                return -ETXTBSY;
            }
            i += 1;
        }
    }
    0
}

/* Called during kexec sequence with MMU off */
unsafe fn copy_segments(ind: c_ulong) {
    let mut entry: c_ulong;
    let mut ptr: *mut c_ulong = core::ptr::null_mut();
    let mut dest: *mut c_void = core::ptr::null_mut();
    let mut addr: *mut c_void;

    /*
     * We rely on kexec_load to create a lists that properly
     * initializes these pointers before they are used.
     * We will still crash if the list is wrong, but at least
     * the compiler will be quiet.
     */
    entry = ind;
    while (entry & IND_DONE) == 0 {
        addr = __va(entry & PAGE_MASK);
        match entry & IND_FLAGS {
            IND_DESTINATION => dest = addr,
            IND_INDIRECTION => ptr = addr as *mut c_ulong,
            IND_SOURCE => {
                copy_page(dest, addr);
                dest = (dest as *mut u8).add(PAGE_SIZE) as *mut c_void;
            }
            _ => {}
        }
        entry = *ptr;
        ptr = ptr.add(1);
    }
}

/* Called during kexec sequence with MMU off */
pub unsafe fn kexec_copy_flush(image: *mut kimage) {
    let nr_segments = (*image).nr_segments as isize;
    let mut ranges: [kexec_segment; KEXEC_SEGMENT_MAX] = core::mem::zeroed();

    /* save the ranges on the stack to efficiently flush the icache */
    memcpy(ranges.as_mut_ptr() as *mut c_void, (*image).segment as *const c_void,
           core::mem::size_of_val(&ranges));

    /*
     * After this call we may not use anything allocated in dynamic
     * memory, including *image.
     *
     * Only globals and the stack are allowed.
     */
    copy_segments((*image).head);

    /*
     * we need to clear the icache for all dest pages sometime,
     * including ones that were in place on the original copy
     */
    let mut i = 0isize;
    while i < nr_segments {
        flush_icache_range(__va(ranges[i as usize].mem) as c_ulong,
                           __va(ranges[i as usize].mem + ranges[i as usize].memsz) as c_ulong);
        i += 1;
    }
}

#[cfg(CONFIG_SMP)]
static mut kexec_all_irq_disabled: c_int = 0;

#[cfg(CONFIG_SMP)]
unsafe fn kexec_smp_down(_arg: *mut c_void) {
    local_irq_disable();
    hard_irq_disable();
    mb(); /* make sure our irqs are disabled before we say they are */
    (*get_paca()).kexec_state = KEXEC_STATE_IRQS_OFF;
    while kexec_all_irq_disabled == 0 { cpu_relax(); }
    mb(); /* make sure all irqs are disabled before this */
    hw_breakpoint_disable();
    /*
     * Now every CPU has IRQs off, we can clear out any pending
     * IPIs and be sure that no more will come in after this.
     */
    if !ppc_md.kexec_cpu_down.is_null() { ppc_md.kexec_cpu_down(0, 1); }
    reset_sprs();
    kexec_smp_wait();
    /* NOTREACHED */
}

#[cfg(CONFIG_SMP)]
unsafe fn kexec_prepare_cpus_wait(wait_state: c_int) {
    let my_cpu = raw_smp_processor_id();
    let mut notified = -1;
    hw_breakpoint_disable();
    for_each_online_cpu!(i) {
        if i == my_cpu { continue; }
        while (*paca_ptrs[i as usize]).kexec_state < wait_state {
            barrier();
            if i != notified {
                printk!(KERN_INFO "kexec: waiting for cpu %d (physical %d) to enter %i state\n",
                        i, (*paca_ptrs[i as usize]).hw_cpu_id, wait_state);
                notified = i;
            }
        }
    }
    mb();
}

#[cfg(all(CONFIG_SMP, CONFIG_HOTPLUG_SMT))]
unsafe fn kexec_smt_reenable() {
    lock_device_hotplug();
    cpu_smt_num_threads = threads_per_core;
    cpu_smt_control = CPU_SMT_ENABLED;
    unlock_device_hotplug();
}
#[cfg(not(all(CONFIG_SMP, CONFIG_HOTPLUG_SMT)))]
unsafe fn kexec_smt_reenable() {}

#[cfg(CONFIG_SMP)]
unsafe fn wake_offline_cpus() {
    let mut cpu = 0;
    kexec_smt_reenable();
    for_each_present_cpu!(cpu) {
        if !cpu_online(cpu) {
            printk!(KERN_INFO "kexec: Waking offline cpu %d.\n", cpu);
            WARN_ON(add_cpu(cpu));
        }
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn kexec_prepare_cpus() {
    wake_offline_cpus();
    smp_call_function(kexec_smp_down, core::ptr::null_mut(), 0);
    local_irq_disable();
    hard_irq_disable();
    mb(); /* make sure IRQs are disabled before we say they are */
    (*get_paca()).kexec_state = KEXEC_STATE_IRQS_OFF;
    kexec_prepare_cpus_wait(KEXEC_STATE_IRQS_OFF);
    kexec_all_irq_disabled = 1;
    /*
     * Before removing MMU mappings make sure all CPUs have entered real
     * mode:
     */
    kexec_prepare_cpus_wait(KEXEC_STATE_REAL_MODE);
    /* after we tell the others to go down */
    if !ppc_md.kexec_cpu_down.is_null() { ppc_md.kexec_cpu_down(0, 0); }
}

#[cfg(not(CONFIG_SMP))]
unsafe fn kexec_prepare_cpus() {
    /*
     * move the secondarys to us so that we can copy
     * the new kernel 0-0x100 safely
     *
     * do this if kexec in setup.c ?
     *
     * We need to release the cpus if we are ever going from an
     * UP to an SMP kernel.
     */
    smp_release_cpus();
    if !ppc_md.kexec_cpu_down.is_null() { ppc_md.kexec_cpu_down(0, 0); }
    local_irq_disable();
    hard_irq_disable();
}

/*
 * kexec thread structure and stack.
 *
 * We need to make sure that this is 16384-byte aligned due to the
 * way process stacks are handled.  It also must be statically allocated
 * or allocated as part of the kimage, because everything else may be
 * overwritten when we copy the kexec image.  We piggyback on the
 * "init_task" linker section here to statically allocate a stack.
 *
 * We could use a smaller stack if we don't care about anything using
 * current, but that audit has not been performed.
 */
static mut kexec_stack: thread_union = thread_union { _bindgen_opaque_blob: [] };

/* For similar reasons to the stack above, the kexecing CPU needs to be on a
 * static PACA; we switch to kexec_paca.
 */
static mut kexec_paca: paca_struct = paca_struct::zeroed();

extern "C" {
    fn kexec_sequence(newstack: *mut c_void, start: c_ulong, image: *mut kimage,
                      control: *mut c_void, clear_all: unsafe extern "C" fn(),
                      copy_with_mmu_off: bool) -> !;
}

/* too late to fail here */
pub unsafe fn default_machine_kexec(image: *mut kimage) {
    let copy_with_mmu_off: bool;
    /* prepare control code if any */

    /*
     * If the kexec boot is the normal one, need to shutdown other cpus
     * into our wait loop and quiesce interrupts.
     * Otherwise, in the case of crashed mode (crashing_cpu >= 0),
     * stopping other CPUs and collecting their pt_regs is done before
     * using debugger IPI.
     */
    if !kdump_in_progress() { kexec_prepare_cpus(); }

    #[cfg(CONFIG_PPC_PSERIES)]
    {
        /*
         * This must be done after other CPUs have shut down, otherwise they
         * could execute the 'scv' instruction, which is not supported with
         * reloc disabled (see configure_exceptions()).
         */
        if firmware_has_feature(FW_FEATURE_SET_MODE) { pseries_disable_reloc_on_exc(); }
    }

    printk!("kexec: Starting switchover sequence.\n");
    /* switch to a staticly allocated stack.  Based on irq stack code.
     * We setup preempt_count to avoid using VMX in memcpy.
     * XXX: the task struct will likely be invalid once we do the copy!
     */
    (*current_thread_info()).flags = 0;
    (*current_thread_info()).preempt_count = HARDIRQ_OFFSET;

    /* We need a static PACA, too; copy this CPU's PACA over and switch to
     * it. Also poison per_cpu_offset and NULL lppaca to catch anyone using
     * non-static data.
     */
    memcpy(&mut kexec_paca as *mut _ as *mut c_void, get_paca() as *mut _ as *const c_void,
           core::mem::size_of::<paca_struct>());
    kexec_paca.data_offset = 0xedeaddeadeeeeeee;
    #[cfg(CONFIG_PPC_PSERIES)] { kexec_paca.lppaca_ptr = core::ptr::null_mut(); }

    if is_secure_guest() && !((*image).preserve_context || (*image).type_ == KEXEC_TYPE_CRASH) {
        uv_unshare_all_pages();
        printk!("kexec: Unshared all shared pages.\n");
    }
    paca_ptrs[kexec_paca.paca_index as usize] = &mut kexec_paca;
    setup_paca(&mut kexec_paca);

    /*
     * On Book3S, the copy must happen with the MMU off if we are either
     * using Radix page tables or we are not in an LPAR since we can
     * overwrite the page tables while copying.
     *
     * In an LPAR, we keep the MMU on otherwise we can't access beyond
     * the RMA. On BookE there is no real MMU off mode, so we have to
     * keep it enabled as well (but then we have bolted TLB entries).
     */
    #[cfg(CONFIG_PPC_BOOK3E_64)]
    { copy_with_mmu_off = false; }
    #[cfg(not(CONFIG_PPC_BOOK3E_64))]
    { copy_with_mmu_off = radix_enabled() || !(firmware_has_feature(FW_FEATURE_LPAR) || firmware_has_feature(FW_FEATURE_PS3_LV1)); }

    /* Some things are best done in assembly.  Finding globals with
     * a toc is easier in C, so pass in what we can.
     */
    kexec_sequence(&mut kexec_stack as *mut _ as *mut c_void, (*image).start, image,
                   page_address((*image).control_code_page), mmu_cleanup_all,
                   copy_with_mmu_off);
}

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
static mut htab_base: be64 = 0;
#[cfg(CONFIG_PPC_64S_HASH_MMU)]
static mut htab_size: be64 = 0;

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
static mut htab_base_prop: property = property {
    name: "linux,htab-base",
    length: core::mem::size_of::<c_ulong>() as c_int,
    value: unsafe { &mut htab_base as *mut be64 as *mut c_void },
};

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
static mut htab_size_prop: property = property {
    name: "linux,htab-size",
    length: core::mem::size_of::<c_ulong>() as c_int,
    value: unsafe { &mut htab_size as *mut be64 as *mut c_void },
};

#[cfg(CONFIG_PPC_64S_HASH_MMU)]
unsafe fn export_htab_values() -> c_int {
    /* On machines with no htab htab_address is NULL */
    if htab_address.is_null() { return -ENODEV; }
    let node = of_find_node_by_path("/chosen");
    if node.is_null() { return -ENODEV; }
    /* remove any stale properties so ours can be found */
    of_remove_property(node, of_find_property(node, "linux,htab-base", core::ptr::null_mut()));
    of_remove_property(node, of_find_property(node, "linux,htab-size", core::ptr::null_mut()));
    htab_base = cpu_to_be64(__pa(htab_address));
    of_add_property(node, &mut htab_base_prop);
    htab_size = cpu_to_be64(htab_size_bytes);
    of_add_property(node, &mut htab_size_prop);
    of_node_put(node);
    0
}

#[cfg(any(CONFIG_KEXEC_FILE, CONFIG_CRASH_DUMP))]
unsafe fn add_node_props(fdt: *mut c_void, node_offset: c_int, dn: *const device_node) -> c_int {
    if dn.is_null() { return -EINVAL; }
    let mut ret = 0;
    for_each_property_of_node!(dn, pp) {
        ret = fdt_setprop(fdt, node_offset, (*pp).name, (*pp).value, (*pp).length);
        if ret < 0 {
            pr_err!("Unable to add %s property: %s\n", (*pp).name, fdt_strerror(ret));
            return ret;
        }
    }
    ret
}

#[cfg(any(CONFIG_KEXEC_FILE, CONFIG_CRASH_DUMP))]
pub unsafe fn update_cpus_node(fdt: *mut c_void) -> c_int {
    let cpus_offset = fdt_path_offset(fdt, "/cpus");
    if cpus_offset < 0 && cpus_offset != -FDT_ERR_NOTFOUND {
        pr_err!("Malformed device tree: error reading /cpus node: %s\n", fdt_strerror(cpus_offset));
        return cpus_offset;
    }
    let mut prev_node_offset = cpus_offset;
    let mut cpus_subnode_offset = fdt_first_subnode(fdt, cpus_offset);
    while cpus_subnode_offset >= 0 {
        let prop = fdt_get_property(fdt, cpus_subnode_offset, "device_type", core::ptr::null_mut());
        if prop.is_null() || strcmp((*prop).data, "cpu") != 0 {
            prev_node_offset = cpus_subnode_offset;
        } else {
            let ret = fdt_del_node(fdt, cpus_subnode_offset);
            if ret < 0 { pr_err!("Failed to delete a cpus sub-node: %s\n", fdt_strerror(ret)); return ret; }
        }
        cpus_subnode_offset = if prev_node_offset == cpus_offset {
            fdt_first_subnode(fdt, cpus_offset)
        } else { fdt_next_subnode(fdt, prev_node_offset) };
    }
    let cpus_node = of_find_node_by_path("/cpus");
    if cpus_node.is_null() { pr_err!("No /cpus node found\n"); return -EINVAL; }
    let mut ret = 0;
    for_each_child_of_node!(cpus_node, dn) {
        let device_type = of_get_property(dn, "device_type", core::ptr::null_mut());
        if device_type.is_null() || strcmp(device_type, "cpu") != 0 { continue; }
        cpus_subnode_offset = fdt_add_subnode(fdt, cpus_offset, (*dn).full_name);
        if cpus_subnode_offset < 0 {
            pr_err!("Unable to add %s subnode: %s\n", (*dn).full_name, fdt_strerror(cpus_subnode_offset));
            ret = cpus_subnode_offset;
            break;
        }
        ret = add_node_props(fdt, cpus_subnode_offset, dn);
        if ret < 0 { break; }
    }
    of_node_put(cpus_node);
    of_node_put(dn);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
