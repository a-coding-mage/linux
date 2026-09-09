// SPDX-License-Identifier: GPL-2.0

// Kernel includes and configuration symbols are supplied by the surrounding
// translation unit.

#[cfg(CONFIG_X86_64)]
const PERCPU_FIRST_CHUNK_RESERVE: usize = PERCPU_MODULE_RESERVE;
#[cfg(not(CONFIG_X86_64))]
const PERCPU_FIRST_CHUNK_RESERVE: usize = 0;

// DEFINE_PER_CPU_CACHE_HOT(int, cpu_number);
// EXPORT_PER_CPU_SYMBOL(cpu_number);
// DEFINE_PER_CPU_CACHE_HOT(unsigned long, this_cpu_off);
// EXPORT_PER_CPU_SYMBOL(this_cpu_off);
#[no_mangle]
pub static mut __per_cpu_offset: [usize; NR_CPUS] = [0; NR_CPUS];
// EXPORT_SYMBOL(__per_cpu_offset);

#[cfg(CONFIG_X86_32)]
unsafe fn pcpu_need_numa() -> bool {
    #[cfg(CONFIG_NUMA)]
    {
        let mut last: *mut pg_data_t = core::ptr::null_mut();
        for_each_possible_cpu!(cpu, {
            let node = early_cpu_to_node(cpu);
            if node_online(node) && !NODE_DATA(node).is_null()
                && !last.is_null() && last != NODE_DATA(node)
            {
                return true;
            }
            last = NODE_DATA(node);
        });
    }
    false
}

unsafe fn pcpu_cpu_distance(from: u32, to: u32) -> i32 {
    #[cfg(CONFIG_NUMA)]
    {
        if early_cpu_to_node(from) == early_cpu_to_node(to) {
            return LOCAL_DISTANCE;
        } else {
            return REMOTE_DISTANCE;
        }
    }
    LOCAL_DISTANCE
}

unsafe fn pcpu_cpu_to_node(cpu: i32) -> i32 {
    early_cpu_to_node(cpu)
}

pub unsafe fn pcpu_populate_pte(addr: usize) {
    populate_extra_pte(addr);
}

unsafe fn setup_percpu_segment(cpu: i32) {
    #[cfg(CONFIG_X86_32)]
    {
        let mut d = GDT_ENTRY_INIT(DESC_DATA32, per_cpu_offset(cpu), 0xFFFFF);
        write_gdt_entry(get_cpu_gdt_rw(cpu), GDT_ENTRY_PERCPU, &mut d, DESCTYPE_S);
    }
}

pub unsafe fn setup_per_cpu_areas() {
    let mut cpu: u32;
    let mut delta: usize;
    let mut rc: i32;

    pr_info!(
        "NR_CPUS:%d nr_cpumask_bits:%d nr_cpu_ids:%u nr_node_ids:%u\n",
        NR_CPUS, nr_cpumask_bits, nr_cpu_ids, nr_node_ids
    );

    #[cfg(CONFIG_X86_32)]
    if pcpu_chosen_fc == PCPU_FC_AUTO && pcpu_need_numa() {
        pcpu_chosen_fc = PCPU_FC_PAGE;
    }

    rc = -EINVAL;
    if pcpu_chosen_fc != PCPU_FC_PAGE {
        let dyn_size: usize = PERCPU_MODULE_RESERVE + PERCPU_DYNAMIC_RESERVE
            - PERCPU_FIRST_CHUNK_RESERVE;
        let atom_size: usize;
        #[cfg(CONFIG_X86_64)]
        { atom_size = PMD_SIZE; }
        #[cfg(not(CONFIG_X86_64))]
        { atom_size = PAGE_SIZE; }

        rc = pcpu_embed_first_chunk(
            PERCPU_FIRST_CHUNK_RESERVE, dyn_size, atom_size,
            pcpu_cpu_distance, pcpu_cpu_to_node,
        );
        if rc < 0 {
            pr_warn!(
                "%s allocator failed (%d), falling back to page size\n",
                pcpu_fc_names[pcpu_chosen_fc], rc
            );
        }
    }
    if rc < 0 {
        rc = pcpu_page_first_chunk(PERCPU_FIRST_CHUNK_RESERVE, pcpu_cpu_to_node);
    }
    if rc < 0 {
        panic!("cannot initialize percpu area (err=%d)", rc);
    }

    // alrighty, percpu areas up and running
    delta = (pcpu_base_addr as usize).wrapping_sub(__per_cpu_start as usize);
    for_each_possible_cpu!(cpu, {
        per_cpu_offset!(cpu) = delta + pcpu_unit_offsets[cpu as usize];
        per_cpu!(this_cpu_off, cpu) = per_cpu_offset!(cpu);
        per_cpu!(cpu_number, cpu) = cpu;
        setup_percpu_segment(cpu as i32);

        #[cfg(CONFIG_X86_LOCAL_APIC)]
        {
            per_cpu!(x86_cpu_to_apicid, cpu) = early_per_cpu_map!(x86_cpu_to_apicid, cpu);
            per_cpu!(x86_cpu_to_acpiid, cpu) = early_per_cpu_map!(x86_cpu_to_acpiid, cpu);
        }
        #[cfg(CONFIG_NUMA)]
        {
            per_cpu!(x86_cpu_to_node_map, cpu) = early_per_cpu_map!(x86_cpu_to_node_map, cpu);
            set_cpu_numa_node(cpu, early_cpu_to_node(cpu));
        }
        if cpu == 0 {
            switch_gdt_and_percpu_base(cpu as i32);
        }
    });

    #[cfg(CONFIG_X86_LOCAL_APIC)]
    {
        early_per_cpu_ptr!(x86_cpu_to_apicid) = core::ptr::null_mut();
        early_per_cpu_ptr!(x86_cpu_to_acpiid) = core::ptr::null_mut();
    }
    #[cfg(CONFIG_NUMA)]
    { early_per_cpu_ptr!(x86_cpu_to_node_map) = core::ptr::null_mut(); }

    setup_node_to_cpumask_map();
    setup_cpu_local_masks();
    sync_initial_page_table();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
