// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/devtree.c
 *
 *  Copyright (C) 2009 Canonical Ltd. <jeremy.kerr@canonical.com>
 */

// Linux and architecture dependencies are supplied by the surrounding tree.

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    static mut __cpu_method_of_table: of_cpu_method;
}

#[cfg(feature = "CONFIG_SMP")]
static __cpu_method_of_table_sentinel: of_cpu_method = of_cpu_method {
    method: core::ptr::null(),
    ops: core::ptr::null(),
};

#[cfg(feature = "CONFIG_SMP")]
unsafe fn set_smp_ops_by_method(node: *mut device_node) -> i32 {
    let mut method: *const core::ffi::c_char = core::ptr::null();
    let mut m: *mut of_cpu_method = core::ptr::addr_of_mut!(__cpu_method_of_table);

    if of_property_read_string(node, b"enable-method\0".as_ptr() as *const _, &mut method) != 0 {
        return 0;
    }

    while !(*m).method.is_null() {
        if strcmp((*m).method, method) == 0 {
            smp_set_ops((*m).ops);
            return 1;
        }
        m = m.add(1);
    }

    0
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe fn set_smp_ops_by_method(_node: *mut device_node) -> i32 {
    1
}

/*
 * arm_dt_init_cpu_maps - Function retrieves cpu nodes from the device tree
 * and builds the cpu logical map array containing MPIDR values related to
 * logical cpus
 *
 * Updates the cpu possible mask with the number of parsed cpu nodes
 */
pub unsafe fn arm_dt_init_cpu_maps() {
    let mut cpu: *mut device_node;
    let cpus: *mut device_node;
    let mut found_method = 0;
    let mut i: u32;
    let mut j: u32;
    let mut cpuidx: u32 = 1;
    let mpidr: u32 = if is_smp() { read_cpuid_mpidr() & MPIDR_HWID_BITMASK } else { 0 };

    let mut tmp_map = [MPIDR_INVALID; NR_CPUS];
    let mut bootcpu_valid = false;
    cpus = of_find_node_by_path(b"/cpus\0".as_ptr() as *const _);

    if cpus.is_null() {
        return;
    }

    // for_each_of_cpu_node(cpu)
    let mut node_index = 0;
    while {
        cpu = for_each_of_cpu_node_next(node_index);
        node_index += 1;
        !cpu.is_null()
    } {
        let hwid: u32 = of_get_cpu_hwid(cpu, 0);

        pr_debug!(" * %pOF...\n", cpu);

        if hwid & !MPIDR_HWID_BITMASK != 0 {
            of_node_put(cpu);
            return;
        }

        j = 0;
        while j < cpuidx {
            if WARN!(tmp_map[j as usize] == hwid,
                     "Duplicate /cpu reg properties in the DT\n") {
                of_node_put(cpu);
                return;
            }
            j += 1;
        }

        if hwid == mpidr {
            i = 0;
            bootcpu_valid = true;
        } else {
            i = cpuidx;
            cpuidx += 1;
        }

        if WARN!(cpuidx > nr_cpu_ids,
                 "DT /cpu %u nodes greater than max cores %u, capping them\n",
                 cpuidx, nr_cpu_ids) {
            cpuidx = nr_cpu_ids;
            of_node_put(cpu);
            break;
        }

        tmp_map[i as usize] = hwid;

        if found_method == 0 {
            found_method = set_smp_ops_by_method(cpu);
        }
    }

    if found_method == 0 {
        set_smp_ops_by_method(cpus);
    }

    if !bootcpu_valid {
        pr_warn!("DT missing boot CPU MPIDR[23:0], fall back to default cpu_logical_map\n");
        return;
    }

    i = 0;
    while i < cpuidx {
        set_cpu_possible(i, true);
        cpu_logical_map(i) = tmp_map[i as usize];
        pr_debug!("cpu logical map 0x%x\n", cpu_logical_map(i));
        i += 1;
    }
}

pub unsafe fn arch_match_cpu_phys_id(cpu: i32, phys_id: u64) -> bool {
    phys_id == cpu_logical_map(cpu)
}

unsafe fn arch_get_next_mach(match_: *mut *const *const core::ffi::c_char) -> *const core::ffi::c_void {
    static mut mdesc: *const machine_desc = core::ptr::addr_of!(__arch_info_begin);
    let m = mdesc;

    if m >= core::ptr::addr_of!(__arch_info_end) {
        return core::ptr::null();
    }

    mdesc = mdesc.add(1);
    *match_ = (*m).dt_compat;
    m as *const core::ffi::c_void
}

static __mach_desc_GENERIC_DT: machine_desc = machine_desc {
    l2c_aux_val: 0x0,
    l2c_aux_mask: !0x0,
};

pub unsafe fn setup_machine_fdt(dt_virt: *mut core::ffi::c_void) -> *const machine_desc {
    let mut mdesc: *const machine_desc;
    let mut mdesc_best: *const machine_desc = core::ptr::null();

    mdesc_best = &__mach_desc_GENERIC_DT;

    if dt_virt.is_null() || !early_init_dt_verify(dt_virt, __pa(dt_virt)) {
        return core::ptr::null();
    }

    mdesc = of_flat_dt_match_machine(mdesc_best, arch_get_next_mach);

    if mdesc.is_null() {
        let mut prop: *const core::ffi::c_char;
        let mut size: i32;
        let dt_root: core::ffi::c_ulong;

        early_print(b"\nError: unrecognized/unsupported device tree compatible list:\n[ \0".as_ptr() as *const _);

        dt_root = of_get_flat_dt_root();
        prop = of_get_flat_dt_prop(dt_root, b"compatible\0".as_ptr() as *const _, &mut size);
        while size > 0 {
            early_print(b"'%s' \0".as_ptr() as *const _, prop);
            size -= strlen(prop) as i32 + 1;
            prop = prop.add(strlen(prop) + 1);
        }
        early_print(b"]\n\n\0".as_ptr() as *const _);

        dump_machine_table();
    }

    if !(*mdesc).dt_fixup.is_none() {
        ((*mdesc).dt_fixup.unwrap())();
    }

    early_init_dt_scan_nodes();
    __machine_arch_type = (*mdesc).nr;
    mdesc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
