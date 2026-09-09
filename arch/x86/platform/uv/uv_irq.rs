/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SGI UV IRQ functions
 *
 * Copyright (C) 2008 Silicon Graphics, Inc. All rights reserved.
 */

// Linux and architecture dependencies are supplied by other translated files.

/* MMR offset and pnode of hub sourcing interrupts for a given irq */
#[repr(C)]
struct uv_irq_2_mmr_pnode {
    offset: c_ulong,
    pnode: c_int,
}

unsafe fn uv_program_mmr(cfg: *mut irq_cfg, info: *mut uv_irq_2_mmr_pnode) {
    let mut mmr_value: c_ulong = 0;
    let entry = &mut *(&mut mmr_value as *mut c_ulong as *mut uv_IO_APIC_route_entry);

    entry.vector = (*cfg).vector;
    entry.delivery_mode = APIC_DELIVERY_MODE_FIXED;
    entry.dest_mode = (*apic).dest_mode_logical;
    entry.polarity = 0;
    entry.trigger = 0;
    entry.mask = 0;
    entry.dest = (*cfg).dest_apicid;

    uv_write_global_mmr64((*info).pnode, (*info).offset, mmr_value);
}

unsafe extern "C" fn uv_noop(_data: *mut irq_data) {}

unsafe extern "C" fn uv_set_irq_affinity(
    data: *mut irq_data,
    mask: *const cpumask,
    force: bool,
) -> c_int {
    let parent = (*data).parent_data;
    let cfg = irqd_cfg(data);
    let ret = ((*(*parent).chip).irq_set_affinity)(parent, mask, force);

    if ret >= 0 {
        uv_program_mmr(cfg, (*data).chip_data as *mut uv_irq_2_mmr_pnode);
        vector_schedule_cleanup(cfg);
    }

    ret
}

static mut uv_irq_chip: irq_chip = irq_chip {
    name: "UV-CORE\0".as_ptr() as *const c_char,
    irq_mask: Some(uv_noop),
    irq_unmask: Some(uv_noop),
    irq_eoi: Some(apic_ack_irq),
    irq_set_affinity: Some(uv_set_irq_affinity),
};

unsafe extern "C" fn uv_domain_alloc(
    domain: *mut irq_domain,
    virq: c_uint,
    nr_irqs: c_uint,
    arg: *mut c_void,
) -> c_int {
    let info = arg as *mut irq_alloc_info;
    let irq_data = irq_domain_get_irq_data(domain, virq);
    let mut chip_data: *mut uv_irq_2_mmr_pnode;
    let ret: c_int;

    if nr_irqs > 1 || info.is_null() || (*info).type_ != X86_IRQ_ALLOC_TYPE_UV {
        return -EINVAL;
    }

    chip_data = kmalloc_node(
        core::mem::size_of::<uv_irq_2_mmr_pnode>(),
        GFP_KERNEL,
        irq_data_get_node(irq_data),
    ) as *mut uv_irq_2_mmr_pnode;
    if chip_data.is_null() {
        return -ENOMEM;
    }

    ret = irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, arg);
    if ret >= 0 {
        if (*info).uv.limit == UV_AFFINITY_CPU {
            irq_set_status_flags(virq, IRQ_NO_BALANCING);
        }

        (*chip_data).pnode = uv_blade_to_pnode((*info).uv.blade);
        (*chip_data).offset = (*info).uv.offset;
        irq_domain_set_info(
            domain,
            virq,
            virq,
            &raw mut uv_irq_chip,
            chip_data as *mut c_void,
            Some(handle_percpu_irq),
            core::ptr::null_mut(),
            (*info).uv.name,
        );
    } else {
        kfree(chip_data as *mut c_void);
    }

    ret
}

unsafe extern "C" fn uv_domain_free(
    domain: *mut irq_domain,
    virq: c_uint,
    nr_irqs: c_uint,
) {
    let irq_data = irq_domain_get_irq_data(domain, virq);

    BUG_ON(nr_irqs != 1);
    kfree((*irq_data).chip_data);
    irq_clear_status_flags(virq, IRQ_NO_BALANCING);
    irq_domain_free_irqs_top(domain, virq, nr_irqs);
}

unsafe extern "C" fn uv_domain_activate(
    _domain: *mut irq_domain,
    irq_data: *mut irq_data,
    _reserve: bool,
) -> c_int {
    uv_program_mmr(irqd_cfg(irq_data), (*irq_data).chip_data as *mut uv_irq_2_mmr_pnode);
    0
}

unsafe extern "C" fn uv_domain_deactivate(
    _domain: *mut irq_domain,
    irq_data: *mut irq_data,
) {
    let mut mmr_value: c_ulong = 0;
    let entry = &mut *(&mut mmr_value as *mut c_ulong as *mut uv_IO_APIC_route_entry);

    entry.mask = 1;
    uv_program_mmr(irqd_cfg(irq_data), (*irq_data).chip_data as *mut uv_irq_2_mmr_pnode);
}

static uv_domain_ops: irq_domain_ops = irq_domain_ops {
    alloc: Some(uv_domain_alloc),
    free: Some(uv_domain_free),
    activate: Some(uv_domain_activate),
    deactivate: Some(uv_domain_deactivate),
};

unsafe fn uv_get_irq_domain() -> *mut irq_domain {
    static mut uv_domain: *mut irq_domain = core::ptr::null_mut();
    static mut uv_lock: mutex = DEFINE_MUTEX!();
    let fn_: *mut fwnode_handle;

    mutex_lock(&raw mut uv_lock);
    if !uv_domain.is_null() {
        mutex_unlock(&raw mut uv_lock);
        return uv_domain;
    }

    fn_ = irq_domain_alloc_named_fwnode("UV-CORE\0".as_ptr() as *const c_char);
    if fn_.is_null() {
        mutex_unlock(&raw mut uv_lock);
        return uv_domain;
    }

    uv_domain = irq_domain_create_hierarchy(
        x86_vector_domain,
        0,
        0,
        fn_,
        &uv_domain_ops,
        core::ptr::null_mut(),
    );
    if uv_domain.is_null() {
        irq_domain_free_fwnode(fn_);
    }
    mutex_unlock(&raw mut uv_lock);

    uv_domain
}

/*
 * Set up a mapping of an available irq and vector, and enable the specified
 * MMR that defines the MSI that is to be sent to the specified CPU when an
 * interrupt is raised.
 */
#[no_mangle]
pub unsafe extern "C" fn uv_setup_irq(
    irq_name: *mut c_char,
    cpu: c_int,
    mmr_blade: c_int,
    mmr_offset: c_ulong,
    limit: c_int,
) -> c_int {
    let mut info: irq_alloc_info = core::mem::zeroed();
    let domain = uv_get_irq_domain();

    if domain.is_null() {
        return -ENOMEM;
    }

    init_irq_alloc_info(&mut info, cpumask_of(cpu));
    info.type_ = X86_IRQ_ALLOC_TYPE_UV;
    info.uv.limit = limit;
    info.uv.blade = mmr_blade;
    info.uv.offset = mmr_offset;
    info.uv.name = irq_name;

    irq_domain_alloc_irqs(domain, 1, uv_blade_to_memory_nid(mmr_blade), &mut info)
}

#[no_mangle]
pub unsafe extern "C" fn uv_teardown_irq(irq: c_uint) {
    irq_domain_free_irqs(irq, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
