// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ICS backend for OPAL managed interrupts.
 *
 * Copyright 2011 IBM Corp.
 */

// External Linux/PowerPC headers and build-time definitions are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct ics_native {
    pub ics: ics,
    pub node: *mut device_node,
    pub base: *mut core::ffi::c_void,
    pub ibase: u32,
    pub icount: u32,
}

#[inline]
unsafe fn to_ics_native(ics_ptr: *mut ics) -> *mut ics_native {
    ics_ptr as *mut ics_native
}

unsafe fn ics_native_xive(in_ptr: *mut ics_native, vec: u32) -> *mut u32 {
    (in_ptr as *mut u8)
        .add(0x800 + (((vec.wrapping_sub((*in_ptr).ibase)) as usize) << 2)) as *mut u32
}

unsafe fn ics_native_unmask_irq(d: *mut irq_data) {
    let vec = irqd_to_hwirq(d) as u32;
    let ics_ptr = irq_data_get_irq_chip_data(d);
    let in_ptr = to_ics_native(ics_ptr);
    let server: u32;

    pr_devel!("ics-native: unmask virq %d [hw 0x%x]\\n", (*d).irq, vec);

    if vec < (*in_ptr).ibase || vec >= (*in_ptr).ibase.wrapping_add((*in_ptr).icount) {
        return;
    }

    server = xics_get_irq_server((*d).irq, irq_data_get_affinity_mask(d), 0) as u32;
    out_be32(ics_native_xive(in_ptr, vec), (server << 8) | DEFAULT_PRIORITY);
}

unsafe fn ics_native_startup(d: *mut irq_data) -> u32 {
    // CONFIG_PCI_MSI: the generic MSI code leaves the card interrupt
    // disabled; firmware does not unmask it at that level, so do it here.
    // if !irq_data_get_msi_desc(d).is_null() { pci_msi_unmask_irq(d); }

    ics_native_unmask_irq(d);
    0
}

unsafe fn ics_native_do_mask(in_ptr: *mut ics_native, vec: u32) {
    out_be32(ics_native_xive(in_ptr, vec), 0xff);
}

unsafe fn ics_native_mask_irq(d: *mut irq_data) {
    let vec = irqd_to_hwirq(d) as u32;
    let ics_ptr = irq_data_get_irq_chip_data(d);
    let in_ptr = to_ics_native(ics_ptr);

    pr_devel!("ics-native: mask virq %d [hw 0x%x]\\n", (*d).irq, vec);

    if vec < (*in_ptr).ibase || vec >= (*in_ptr).ibase.wrapping_add((*in_ptr).icount) {
        return;
    }
    ics_native_do_mask(in_ptr, vec);
}

unsafe fn ics_native_set_affinity(
    d: *mut irq_data,
    cpumask: *const cpumask,
    _force: bool,
) -> i32 {
    let vec = irqd_to_hwirq(d) as u32;
    let ics_ptr = irq_data_get_irq_chip_data(d);
    let in_ptr = to_ics_native(ics_ptr);
    let server: i32;
    let mut xive: u32;

    if vec < (*in_ptr).ibase || vec >= (*in_ptr).ibase.wrapping_add((*in_ptr).icount) {
        return -EINVAL;
    }

    server = xics_get_irq_server((*d).irq, cpumask, 1);
    if server == -1 {
        pr_warn!("%s: No online cpus in the mask %*pb for irq %d\\n", __func__, cpumask_pr_args(cpumask), (*d).irq);
        return -1;
    }

    xive = in_be32(ics_native_xive(in_ptr, vec));
    xive = (xive & 0xff) | ((server as u32) << 8);
    out_be32(ics_native_xive(in_ptr, vec), xive);

    IRQ_SET_MASK_OK
}

static mut ics_native_irq_chip: irq_chip = irq_chip {
    name: "ICS",
    irq_startup: Some(ics_native_startup),
    irq_mask: Some(ics_native_mask_irq),
    irq_unmask: Some(ics_native_unmask_irq),
    irq_eoi: None, // Patched at init time
    irq_set_affinity: Some(ics_native_set_affinity),
    irq_set_type: Some(xics_set_irq_type),
    irq_retrigger: Some(xics_retrigger),
};

unsafe fn ics_native_check(ics_ptr: *mut ics, hw_irq: u32) -> i32 {
    let in_ptr = to_ics_native(ics_ptr);

    pr_devel!("%s: hw_irq=0x%x\\n", __func__, hw_irq);

    if hw_irq < (*in_ptr).ibase || hw_irq >= (*in_ptr).ibase.wrapping_add((*in_ptr).icount) {
        return -EINVAL;
    }
    0
}

unsafe fn ics_native_mask_unknown(ics_ptr: *mut ics, vec: u64) {
    let in_ptr = to_ics_native(ics_ptr);

    if vec < (*in_ptr).ibase as u64 || vec >= (*in_ptr).ibase.wrapping_add((*in_ptr).icount) as u64 {
        return;
    }
    ics_native_do_mask(in_ptr, vec as u32);
}

unsafe fn ics_native_get_server(ics_ptr: *mut ics, vec: u64) -> i64 {
    let in_ptr = to_ics_native(ics_ptr);
    let xive: u32;

    if vec < (*in_ptr).ibase as u64 || vec >= (*in_ptr).ibase.wrapping_add((*in_ptr).icount) as u64 {
        return -EINVAL as i64;
    }

    xive = in_be32(ics_native_xive(in_ptr, vec as u32));
    ((xive >> 8) & 0xfff) as i64
}

unsafe fn ics_native_host_match(ics_ptr: *mut ics, node: *mut device_node) -> bool {
    let in_ptr = to_ics_native(ics_ptr);
    (*in_ptr).node == node
}

static mut ics_native_template: ics = ics {
    check: Some(ics_native_check),
    mask_unknown: Some(ics_native_mask_unknown),
    get_server: Some(ics_native_get_server),
    host_match: Some(ics_native_host_match),
    chip: unsafe { &mut ics_native_irq_chip },
};

unsafe fn ics_native_add_one(np: *mut device_node) -> i32 {
    let mut ics_ptr: *mut ics_native;
    let mut ranges = [0u32; 2];
    let mut rc: i32;
    let count: i32;

    ics_ptr = kzalloc_obj::<ics_native>();
    if ics_ptr.is_null() {
        return -ENOMEM;
    }
    (*ics_ptr).node = of_node_get(np);
    core::ptr::copy_nonoverlapping(&ics_native_template, &mut (*ics_ptr).ics, 1);

    (*ics_ptr).base = of_iomap(np, 0);
    if (*ics_ptr).base.is_null() {
        pr_err!("Failed to map %pOFP\\n", np);
        rc = -ENOMEM;
        goto_fail(ics_ptr, rc);
    }

    count = of_property_count_u32_elems(np, "interrupt-ranges");
    if count < 2 || (count & 1) != 0 {
        pr_err!("Failed to read interrupt-ranges of %pOFP\\n", np);
        rc = -EINVAL;
        goto_fail(ics_ptr, rc);
    }
    if count > 2 {
        pr_warn!("ICS %pOFP has %d ranges, only one supported\\n", np, count >> 1);
    }
    rc = of_property_read_u32_array(np, "interrupt-ranges", ranges.as_mut_ptr(), 2);
    if rc != 0 {
        pr_err!("Failed to read interrupt-ranges of %pOFP\\n", np);
        goto_fail(ics_ptr, rc);
    }
    (*ics_ptr).ibase = ranges[0];
    (*ics_ptr).icount = ranges[1];

    pr_info!("ICS native initialized for sources %d..%d\\n", (*ics_ptr).ibase, (*ics_ptr).ibase + (*ics_ptr).icount - 1);
    xics_register_ics(&mut (*ics_ptr).ics);
    return 0;

    fn goto_fail(ics_ptr: *mut ics_native, rc: i32) -> i32 {
        unsafe {
            of_node_put((*ics_ptr).node);
            kfree(ics_ptr);
        }
        rc
    }
}

pub unsafe fn ics_native_init() -> i32 {
    let mut ics_ptr: *mut device_node;
    let mut found_one = false;

    ics_native_irq_chip.irq_eoi = (*icp_ops).eoi;

    for_each_compatible_node!(ics_ptr, core::ptr::null_mut(), "openpower,xics-sources") {
        if ics_native_add_one(ics_ptr) == 0 {
            found_one = true;
        }
    }

    if found_one {
        pr_info!("ICS native backend registered\\n");
    }

    if found_one { 0 } else { -ENODEV }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
