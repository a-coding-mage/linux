// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 interrupt routines.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

// Kernel and PS3 platform dependencies are supplied by the surrounding tree.

const PS3_BMP_MINALIGN: usize = 64;

#[repr(C)]
pub struct ps3_bmp {
    pub status: u64,
    pub unused_1: [u64; 3],
    pub mask: c_ulong,
    pub unused_2: [u64; 3],
}

#[repr(C, align(64))]
pub struct ps3_private {
    pub bmp: ps3_bmp,
    pub bmp_lock: spinlock_t,
    pub ppe_id: u64,
    pub thread_id: u64,
    pub ipi_debug_brk_mask: c_ulong,
    pub ipi_mask: c_ulong,
}

static mut ps3_private: PerCpu<ps3_private> = DEFINE_PER_CPU!();

unsafe fn ps3_chip_mask(d: *mut irq_data) {
    let pd = irq_data_get_irq_chip_data(d);
    let mut flags: c_ulong = 0;

    DBG!("%s:%d: thread_id %llu, virq %d\n", __func__, __LINE__, (*pd).thread_id, (*d).irq);
    local_irq_save(&mut flags);
    clear_bit(63 - (*d).irq, &mut (*pd).bmp.mask);
    lv1_did_update_interrupt_mask((*pd).ppe_id, (*pd).thread_id);
    local_irq_restore(flags);
}

unsafe fn ps3_chip_unmask(d: *mut irq_data) {
    let pd = irq_data_get_irq_chip_data(d);
    let mut flags: c_ulong = 0;

    DBG!("%s:%d: thread_id %llu, virq %d\n", __func__, __LINE__, (*pd).thread_id, (*d).irq);
    local_irq_save(&mut flags);
    set_bit(63 - (*d).irq, &mut (*pd).bmp.mask);
    lv1_did_update_interrupt_mask((*pd).ppe_id, (*pd).thread_id);
    local_irq_restore(flags);
}

unsafe fn ps3_chip_eoi(d: *mut irq_data) {
    let pd = irq_data_get_irq_chip_data(d) as *const ps3_private;
    if !test_bit(63 - (*d).irq, &(*pd).ipi_mask) {
        lv1_end_of_interrupt_ext((*pd).ppe_id, (*pd).thread_id, (*d).irq);
    }
}

static mut ps3_irq_chip: irq_chip = irq_chip {
    name: "ps3\0".as_ptr() as *const c_char,
    irq_mask: Some(ps3_chip_mask),
    irq_unmask: Some(ps3_chip_unmask),
    irq_eoi: Some(ps3_chip_eoi),
};

unsafe fn ps3_virq_setup(cpu: ps3_cpu_binding, outlet: c_ulong, virq: *mut c_uint) -> c_int {
    let mut result: c_int;
    let cpu = if cpu == PS3_BINDING_CPU_ANY { 0 } else { cpu };
    let pd = per_cpu_ptr(&mut ps3_private, cpu);

    *virq = irq_create_mapping(core::ptr::null_mut(), outlet);
    if *virq == 0 {
        FAIL!("%s:%d: irq_create_mapping failed: outlet %lu\n", __func__, __LINE__, outlet);
        return -ENOMEM;
    }
    DBG!("%s:%d: outlet %lu => cpu %u, virq %u\n", __func__, __LINE__, outlet, cpu, *virq);
    result = irq_set_chip_data(*virq, pd);
    if result != 0 {
        FAIL!("%s:%d: irq_set_chip_data failed\n", __func__, __LINE__);
        irq_dispose_mapping(*virq);
        return result;
    }
    ps3_chip_mask(irq_get_irq_data(*virq));
    result
}

unsafe fn ps3_virq_destroy(virq: c_uint) -> c_int {
    let pd = irq_get_chip_data(virq) as *const ps3_private;
    DBG!("%s:%d: ppe_id %llu, thread_id %llu, virq %u\n", __func__, __LINE__, (*pd).ppe_id, (*pd).thread_id, virq);
    irq_set_chip_data(virq, core::ptr::null_mut());
    irq_dispose_mapping(virq);
    DBG!("%s:%d <-\n", __func__, __LINE__);
    0
}

pub unsafe fn ps3_irq_plug_setup(cpu: ps3_cpu_binding, outlet: c_ulong, virq: *mut c_uint) -> c_int {
    let result = ps3_virq_setup(cpu, outlet, virq);
    if result != 0 { FAIL!("%s:%d: ps3_virq_setup failed\n", __func__, __LINE__); return result; }
    let pd = irq_get_chip_data(*virq) as *const ps3_private;
    let result = lv1_connect_irq_plug_ext((*pd).ppe_id, (*pd).thread_id, *virq, outlet, 0);
    if result != 0 {
        FAIL!("%s:%d: lv1_connect_irq_plug_ext failed: %s\n", __func__, __LINE__, ps3_result(result));
        ps3_virq_destroy(*virq);
        return -EPERM;
    }
    result
}

pub unsafe fn ps3_irq_plug_destroy(virq: c_uint) -> c_int {
    let pd = irq_get_chip_data(virq) as *const ps3_private;
    DBG!("%s:%d: ppe_id %llu, thread_id %llu, virq %u\n", __func__, __LINE__, (*pd).ppe_id, (*pd).thread_id, virq);
    ps3_chip_mask(irq_get_irq_data(virq));
    let result = lv1_disconnect_irq_plug_ext((*pd).ppe_id, (*pd).thread_id, virq);
    if result != 0 { FAIL!("%s:%d: lv1_disconnect_irq_plug_ext failed: %s\n", __func__, __LINE__, ps3_result(result)); }
    ps3_virq_destroy(virq);
    result
}

pub unsafe fn ps3_event_receive_port_setup(cpu: ps3_cpu_binding, virq: *mut c_uint) -> c_int {
    let mut outlet = 0u64;
    let result = lv1_construct_event_receive_port(&mut outlet);
    if result != 0 { FAIL!("%s:%d: lv1_construct_event_receive_port failed: %s\n", __func__, __LINE__, ps3_result(result)); *virq = 0; return result; }
    let result = ps3_irq_plug_setup(cpu, outlet as c_ulong, virq);
    BUG_ON(result);
    result
}

pub unsafe fn ps3_event_receive_port_destroy(virq: c_uint) -> c_int {
    ps3_chip_mask(irq_get_irq_data(virq));
    let result = lv1_destruct_event_receive_port(virq_to_hw(virq));
    if result != 0 { FAIL!("%s:%d: lv1_destruct_event_receive_port failed: %s\n", __func__, __LINE__, ps3_result(result)); }
    result
}

pub unsafe fn ps3_send_event_locally(virq: c_uint) -> c_int { lv1_send_event_locally(virq_to_hw(virq)) }

pub unsafe fn ps3_sb_event_receive_port_setup(dev: *mut ps3_system_bus_device, cpu: ps3_cpu_binding, virq: *mut c_uint) -> c_int {
    let result = ps3_event_receive_port_setup(cpu, virq);
    if result != 0 { return result; }
    let result = lv1_connect_interrupt_event_receive_port((*dev).bus_id, (*dev).dev_id, virq_to_hw(*virq), (*dev).interrupt_id);
    if result != 0 { ps3_event_receive_port_destroy(*virq); *virq = 0; return result; }
    result
}

pub unsafe fn ps3_sb_event_receive_port_destroy(dev: *mut ps3_system_bus_device, virq: c_uint) -> c_int {
    let mut result = lv1_disconnect_interrupt_event_receive_port((*dev).bus_id, (*dev).dev_id, virq_to_hw(virq), (*dev).interrupt_id);
    if result != 0 { FAIL!("%s:%d: lv1_disconnect_interrupt_event_receive_port failed: %s\n", __func__, __LINE__, ps3_result(result)); }
    result = ps3_event_receive_port_destroy(virq); BUG_ON(result);
    result = ps3_virq_destroy(virq); BUG_ON(result);
    result
}

pub unsafe fn ps3_io_irq_setup(cpu: ps3_cpu_binding, interrupt_id: c_uint, virq: *mut c_uint) -> c_int {
    let mut outlet = 0u64;
    let result = lv1_construct_io_irq_outlet(interrupt_id, &mut outlet);
    if result != 0 { FAIL!("%s:%d: lv1_construct_io_irq_outlet failed: %s\n", __func__, __LINE__, ps3_result(result)); return result; }
    let result = ps3_irq_plug_setup(cpu, outlet as c_ulong, virq); BUG_ON(result); result
}

pub unsafe fn ps3_io_irq_destroy(virq: c_uint) -> c_int {
    let outlet = virq_to_hw(virq);
    ps3_chip_mask(irq_get_irq_data(virq));
    let mut result = ps3_irq_plug_destroy(virq); BUG_ON(result);
    result = lv1_destruct_io_irq_outlet(outlet);
    if result != 0 { FAIL!("%s:%d: lv1_destruct_io_irq_outlet failed: %s\n", __func__, __LINE__, ps3_result(result)); }
    result
}

pub unsafe fn ps3_vuart_irq_setup(cpu: ps3_cpu_binding, virt_addr_bmp: *mut c_void, virq: *mut c_uint) -> c_int {
    BUG_ON(!is_kernel_addr(virt_addr_bmp as u64));
    let lpar_addr = ps3_mm_phys_to_lpar(__pa(virt_addr_bmp));
    let mut outlet = 0u64;
    let result = lv1_configure_virtual_uart_irq(lpar_addr, &mut outlet);
    if result != 0 { FAIL!("%s:%d: lv1_configure_virtual_uart_irq failed: %s\n", __func__, __LINE__, ps3_result(result)); return result; }
    let result = ps3_irq_plug_setup(cpu, outlet as c_ulong, virq); BUG_ON(result); result
}

pub unsafe fn ps3_vuart_irq_destroy(virq: c_uint) -> c_int {
    ps3_chip_mask(irq_get_irq_data(virq));
    let result = lv1_deconfigure_virtual_uart_irq();
    if result != 0 { return result; }
    let result = ps3_irq_plug_destroy(virq); BUG_ON(result); result
}

pub unsafe fn ps3_spe_irq_setup(cpu: ps3_cpu_binding, spe_id: c_ulong, class: c_uint, virq: *mut c_uint) -> c_int {
    BUG_ON(class > 2);
    let mut outlet = 0u64;
    let result = lv1_get_spe_irq_outlet(spe_id, class, &mut outlet);
    if result != 0 { FAIL!("%s:%d: lv1_get_spe_irq_outlet failed: %s\n", __func__, __LINE__, ps3_result(result)); return result; }
    let result = ps3_irq_plug_setup(cpu, outlet as c_ulong, virq); BUG_ON(result); result
}

pub unsafe fn ps3_spe_irq_destroy(virq: c_uint) -> c_int {
    ps3_chip_mask(irq_get_irq_data(virq));
    let result = ps3_irq_plug_destroy(virq); BUG_ON(result); result
}

const PS3_INVALID_OUTLET: irq_hw_number_t = -1i64 as irq_hw_number_t;
const PS3_PLUG_MAX: c_uint = 63;

unsafe fn ps3_host_map(_h: *mut irq_domain, virq: c_uint, _hwirq: irq_hw_number_t) -> c_int {
    irq_set_chip_and_handler(virq, &mut ps3_irq_chip, handle_fasteoi_irq);
    0
}

unsafe fn ps3_host_match(_h: *mut irq_domain, _np: *mut device_node, _bus_token: irq_domain_bus_token) -> c_int { 1 }

static ps3_host_ops: irq_domain_ops = irq_domain_ops { map: Some(ps3_host_map), match_: Some(ps3_host_match) };

pub unsafe fn ps3_register_ipi_debug_brk(cpu: c_uint, virq: c_uint) {
    let pd = per_cpu_ptr(&mut ps3_private, cpu);
    set_bit(63 - virq, &mut (*pd).ipi_debug_brk_mask);
    DBG!("%s:%d: cpu %u, virq %u, mask %lxh\n", __func__, __LINE__, cpu, virq, (*pd).ipi_debug_brk_mask);
}

pub unsafe fn ps3_register_ipi_irq(cpu: c_uint, virq: c_uint) {
    let pd = per_cpu_ptr(&mut ps3_private, cpu);
    set_bit(63 - virq, &mut (*pd).ipi_mask);
    DBG!("%s:%d: cpu %u, virq %u, ipi_mask %lxh\n", __func__, __LINE__, cpu, virq, (*pd).ipi_mask);
}

unsafe fn ps3_get_irq() -> c_uint {
    let pd = this_cpu_ptr(&mut ps3_private);
    let mut x = (*pd).bmp.status & (*pd).bmp.mask as u64;
    if x & (*pd).ipi_debug_brk_mask as u64 != 0 { x &= (*pd).ipi_debug_brk_mask as u64; }
    let mut plug = x.leading_zeros();
    plug &= 0x3f;
    if plug == 0 { dump_bmp(per_cpu_ptr(&mut ps3_private, 0)); dump_bmp(per_cpu_ptr(&mut ps3_private, 1)); return 0; }
    if test_bit(63 - plug, &(*pd).ipi_mask) { lv1_end_of_interrupt_ext((*pd).ppe_id, (*pd).thread_id, plug); }
    plug
}

pub unsafe fn ps3_init_IRQ() {
    let host = irq_domain_create_nomap(core::ptr::null_mut(), PS3_PLUG_MAX + 1, &ps3_host_ops, core::ptr::null_mut());
    irq_set_default_domain(host);
    for_each_possible_cpu!(cpu, {
        let pd = per_cpu_ptr(&mut ps3_private, cpu);
        lv1_get_logical_ppe_id(&mut (*pd).ppe_id);
        (*pd).thread_id = get_hard_smp_processor_id(cpu);
        spin_lock_init(&mut (*pd).bmp_lock);
        let result = lv1_configure_irq_state_bitmap((*pd).ppe_id, (*pd).thread_id, ps3_mm_phys_to_lpar(__pa(&mut (*pd).bmp)));
        if result != 0 { FAIL!("%s:%d: lv1_configure_irq_state_bitmap failed: %s\n", __func__, __LINE__, ps3_result(result)); }
    });
    ppc_md.get_irq = Some(ps3_get_irq);
}

pub unsafe fn ps3_shutdown_IRQ(cpu: c_int) {
    let mut ppe_id = 0u64;
    let thread_id = get_hard_smp_processor_id(cpu as c_uint);
    lv1_get_logical_ppe_id(&mut ppe_id);
    let result = lv1_configure_irq_state_bitmap(ppe_id, thread_id, 0);
    DBG!("%s:%d: lv1_configure_irq_state_bitmap (%llu:%llu/%d) %s\n", __func__, __LINE__, ppe_id, thread_id, cpu, ps3_result(result));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
