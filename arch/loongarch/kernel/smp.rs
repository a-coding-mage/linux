// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 2000, 2001 Kanoj Sarcar
 * Copyright (C) 2000, 2001 Ralf Baechle
 * Copyright (C) 2000, 2001 Silicon Graphics, Inc.
 * Copyright (C) 2000, 2001, 2003 Broadcom Corporation
 */
#![allow(non_snake_case, non_camel_case_types, dead_code)]

// Linux headers and configuration-provided symbols are external dependencies.

pub static mut __cpu_number_map: [i32; NR_CPUS] = [0; NR_CPUS];
pub static mut __cpu_logical_map: [i32; NR_CPUS] = [0; NR_CPUS];
pub static mut cpu_sibling_map: [cpumask_t; NR_CPUS] = [cpumask_t::ZERO; NR_CPUS];
pub static mut cpu_llc_shared_map: [cpumask_t; NR_CPUS] = [cpumask_t::ZERO; NR_CPUS];
pub static mut cpu_core_map: [cpumask_t; NR_CPUS] = [cpumask_t::ZERO; NR_CPUS];
static mut cpu_foreign_map: [cpumask_t; NR_CPUS] = [cpumask_t::ZERO; NR_CPUS];
static mut cpu_sibling_setup_map: cpumask_t = cpumask_t::ZERO;
static mut cpu_llc_shared_setup_map: cpumask_t = cpumask_t::ZERO;
static mut cpu_core_setup_map: cpumask_t = cpumask_t::ZERO;

pub static mut cpuboot_data: secondary_data = secondary_data::ZERO;
static mut cpu_state: i32 = 0;
static cpu_starting: completion = completion::ZERO;
static cpu_running: completion = completion::ZERO;

static ipi_types: [&[u8]; NR_IPI] = [
    b"Rescheduling interrupts", b"Function call interrupts",
    b"IRQ work interrupts", b"Clear vector interrupts",
];

pub unsafe fn show_ipi_list(p: *mut seq_file, prec: i32) {
    for i in 0..NR_IPI {
        seq_printf(p, b"%*s%u:" as _, prec - 1, b"IPI" as _, i);
        for_each_online_cpu!(cpu, { seq_put_decimal_ull_width(p, b" " as _, per_cpu!(irq_stat, cpu).ipi_irqs[i], 10); });
        seq_printf(p, b" LoongArch  %d  %s\n" as _, i + 1, ipi_types[i].as_ptr());
    }
}

unsafe fn set_cpu_core_map(cpu: i32) {
    cpumask_set_cpu(cpu, &mut cpu_core_setup_map);
    for_each_cpu!(i, &cpu_core_setup_map, {
        if cpu_data[cpu as usize].package == cpu_data[i as usize].package {
            cpumask_set_cpu(i, &mut cpu_core_map[cpu as usize]);
            cpumask_set_cpu(cpu, &mut cpu_core_map[i as usize]);
        }
    });
}

unsafe fn set_cpu_llc_shared_map(cpu: i32) {
    cpumask_set_cpu(cpu, &mut cpu_llc_shared_setup_map);
    for_each_cpu!(i, &cpu_llc_shared_setup_map, {
        if cpu_to_node(cpu) == cpu_to_node(i) {
            cpumask_set_cpu(i, &mut cpu_llc_shared_map[cpu as usize]);
            cpumask_set_cpu(cpu, &mut cpu_llc_shared_map[i as usize]);
        }
    });
}

unsafe fn clear_cpu_llc_shared_map(cpu: i32) {
    for_each_cpu!(i, &cpu_llc_shared_setup_map, {
        if cpu_to_node(cpu) == cpu_to_node(i) {
            cpumask_clear_cpu(i, &mut cpu_llc_shared_map[cpu as usize]);
            cpumask_clear_cpu(cpu, &mut cpu_llc_shared_map[i as usize]);
        }
    });
    cpumask_clear_cpu(cpu, &mut cpu_llc_shared_setup_map);
}

unsafe fn set_cpu_sibling_map(cpu: i32) {
    cpumask_set_cpu(cpu, &mut cpu_sibling_setup_map);
    for_each_cpu!(i, &cpu_sibling_setup_map, {
        if cpus_are_siblings(cpu, i) {
            cpumask_set_cpu(i, &mut cpu_sibling_map[cpu as usize]);
            cpumask_set_cpu(cpu, &mut cpu_sibling_map[i as usize]);
        }
    });
}

unsafe fn clear_cpu_sibling_map(cpu: i32) {
    for_each_cpu!(i, &cpu_sibling_setup_map, {
        if cpus_are_siblings(cpu, i) {
            cpumask_clear_cpu(i, &mut cpu_sibling_map[cpu as usize]);
            cpumask_clear_cpu(cpu, &mut cpu_sibling_map[i as usize]);
        }
    });
    cpumask_clear_cpu(cpu, &mut cpu_sibling_setup_map);
}

pub unsafe fn calculate_cpu_foreign_map() {
    let mut temp_foreign_map = cpumask_t::ZERO;
    cpumask_clear(&mut temp_foreign_map);
    for_each_online_cpu!(i, {
        let mut core_present = 0;
        for_each_cpu!(k, &temp_foreign_map, { if cpus_are_siblings(i, k) { core_present = 1; } });
        if core_present == 0 { cpumask_set_cpu(i, &mut temp_foreign_map); }
    });
    for_each_online_cpu!(i, { cpumask_andnot(&mut cpu_foreign_map[i as usize], &temp_foreign_map, &cpu_sibling_map[i as usize]); });
}

unsafe fn csr_mail_send(data: u64, cpu: i32, mailbox: i32) {
    let mut val = IOCSR_MBUF_SEND_BLOCKING | (IOCSR_MBUF_SEND_BOX_HI(mailbox) << IOCSR_MBUF_SEND_BOX_SHIFT) | (cpu << IOCSR_MBUF_SEND_CPU_SHIFT) | (data & IOCSR_MBUF_SEND_H32_MASK);
    iocsr_write64(val, LOONGARCH_IOCSR_MBUF_SEND);
    val = IOCSR_MBUF_SEND_BLOCKING | (IOCSR_MBUF_SEND_BOX_LO(mailbox) << IOCSR_MBUF_SEND_BOX_SHIFT) | (cpu << IOCSR_MBUF_SEND_CPU_SHIFT) | (data << IOCSR_MBUF_SEND_BUF_SHIFT);
    iocsr_write64(val, LOONGARCH_IOCSR_MBUF_SEND);
}

unsafe fn ipi_read_clear(_cpu: i32) -> u32 {
    let action = iocsr_read32(LOONGARCH_IOCSR_IPI_STATUS);
    iocsr_write32(action, LOONGARCH_IOCSR_IPI_CLEAR);
    wbflush();
    action
}

unsafe fn ipi_write_action(cpu: i32, action: u32) {
    iocsr_write32(IOCSR_IPI_SEND_BLOCKING | action | (cpu << IOCSR_IPI_SEND_CPU_SHIFT), LOONGARCH_IOCSR_IPI_SEND);
}

unsafe fn loongson_send_ipi_single(cpu: i32, action: u32) { ipi_write_action(cpu_logical_map(cpu), action); }
unsafe fn loongson_send_ipi_mask(mask: *const cpumask_t, action: u32) {
    for_each_cpu!(i, mask, { ipi_write_action(cpu_logical_map(i), action); });
}

pub unsafe fn arch_smp_send_reschedule(cpu: i32) { mp_ops.send_ipi_single(cpu, ACTION_RESCHEDULE); }

unsafe fn loongson_ipi_interrupt(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu = smp_processor_id();
    let action = ipi_read_clear(cpu_logical_map(cpu));
    if action & SMP_RESCHEDULE != 0 { scheduler_ipi(); per_cpu!(irq_stat, cpu).ipi_irqs[IPI_RESCHEDULE] += 1; }
    if action & SMP_CALL_FUNCTION != 0 { generic_smp_call_function_interrupt(); per_cpu!(irq_stat, cpu).ipi_irqs[IPI_CALL_FUNCTION] += 1; }
    if action & SMP_IRQ_WORK != 0 { irq_work_run(); per_cpu!(irq_stat, cpu).ipi_irqs[IPI_IRQ_WORK] += 1; }
    if action & SMP_CLEAR_VECTOR != 0 { complete_irq_moving(); per_cpu!(irq_stat, cpu).ipi_irqs[IPI_CLEAR_VECTOR] += 1; }
    IRQ_HANDLED
}

unsafe fn loongson_init_ipi() {
    let ipi_irq = get_percpu_irq(INT_IPI);
    if ipi_irq < 0 { panic!("IPI IRQ mapping failed\n"); }
    irq_set_percpu_devid(ipi_irq);
    if request_percpu_irq(ipi_irq, loongson_ipi_interrupt, b"IPI\0".as_ptr(), &mut irq_stat) < 0 { panic!("IPI IRQ request failed\n"); }
}

pub static mut mp_ops: smp_ops = smp_ops { init_ipi: loongson_init_ipi, send_ipi_single: loongson_send_ipi_single, send_ipi_mask: loongson_send_ipi_mask };

// The remaining functions are direct translations; configuration-gated C sections retain their intent.
pub unsafe fn loongson_smp_setup() { fdt_smp_setup(); if loongson_sysconf.cores_per_package == 0 { loongson_sysconf.cores_per_package = num_processors; } cpu_data[0].core = cpu_logical_map(0) % loongson_sysconf.cores_per_package; cpu_data[0].package = cpu_logical_map(0) / loongson_sysconf.cores_per_package; pv_ipi_init(); iocsr_write32(0xffffffff, LOONGARCH_IOCSR_IPI_EN); pr_info!(b"Detected %i available CPU(s)\n", loongson_sysconf.nr_cpus); }

pub unsafe fn loongson_boot_secondary(cpu: i32, idle: *mut task_struct) { pr_info!(b"Booting CPU#%d...\n", cpu); let entry = __pa_symbol(&smpboot_entry as *const _ as u64); cpuboot_data.task = idle as u64; cpuboot_data.stack = task_pt_regs(idle) as u64; cpuboot_data.offset = per_cpu_offset(cpu); csr_mail_send(entry, cpu_logical_map(cpu), 0); loongson_send_ipi_single(cpu, ACTION_BOOT_CPU); }

pub unsafe fn loongson_smp_finish() { local_irq_enable(); iocsr_write64(0, LOONGARCH_IOCSR_MBUF0); pr_info!(b"CPU#%d finished\n", smp_processor_id()); }

pub unsafe fn smp_cpus_done(_max_cpus: u32) {}

unsafe fn flush_tlb_all_ipi(_info: *mut core::ffi::c_void) { local_flush_tlb_all(); }
pub unsafe fn flush_tlb_all() { on_each_cpu(flush_tlb_all_ipi, core::ptr::null_mut(), 1); }
unsafe fn flush_tlb_one_ipi(info: *mut core::ffi::c_void) { local_flush_tlb_one(info as u64); }
pub unsafe fn flush_tlb_one(vaddr: u64) { on_each_cpu(flush_tlb_one_ipi, vaddr as *mut _, 1); }

pub unsafe fn loongson_prepare_cpus(_max_cpus: u32) {
    parse_acpi_topology(); cpu_data[0].global_id = cpu_logical_map(0);
    let mut threads_per_core = if !pptt_enabled { 1 } else { 0 };
    if pptt_enabled { for_each_possible_cpu!(i, { if cpu_to_node(i) == 0 && cpus_are_siblings(0, i) { threads_per_core += 1; } }); }
    for i in 0..loongson_sysconf.nr_cpus { set_cpu_present(i, true); csr_mail_send(0, __cpu_logical_map[i as usize], 0); }
    per_cpu!(cpu_state, smp_processor_id()) = CPU_ONLINE; cpu_smt_set_num_threads(threads_per_core, threads_per_core);
}

pub unsafe fn loongson_init_secondary() { let cpu = smp_processor_id(); let imask = ECFGF_IP0|ECFGF_IP1|ECFGF_IP2|ECFGF_IPI|ECFGF_PMC|ECFGF_TIMER|ECFGF_SIP0; change_csr_ecfg(ECFG0_IM, imask); iocsr_write32(0xffffffff, LOONGARCH_IOCSR_IPI_EN); per_cpu!(cpu_state, cpu) = CPU_ONLINE; cpu_data[cpu as usize].global_id = cpu_logical_map(cpu); }

pub unsafe fn __cpu_up(cpu: u32, tidle: *mut task_struct) -> i32 { loongson_boot_secondary(cpu as i32, tidle); if wait_for_completion_timeout(&cpu_starting, msecs_to_jiffies(5000)) == 0 { pr_crit!(b"CPU%u: failed to start\n", cpu); return -EIO; } wait_for_completion(&cpu_running); 0 }

pub unsafe fn start_secondary() {
    sync_counter(); let cpu = raw_smp_processor_id(); set_my_cpu_offset(per_cpu_offset(cpu)); cpu_probe(); set_current(current); constant_clockevent_init(); loongson_init_secondary(); set_cpu_sibling_map(cpu); set_cpu_llc_shared_map(cpu); set_cpu_core_map(cpu); notify_cpu_starting(cpu); complete(&cpu_starting); set_cpu_online(cpu, true); calculate_cpu_foreign_map(); complete(&cpu_running); WARN_ON_ONCE(!irqs_disabled()); loongson_smp_finish(); cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

unsafe fn stop_this_cpu(_dummy: *mut core::ffi::c_void) { set_cpu_online(smp_processor_id(), false); calculate_cpu_foreign_map(); local_irq_disable(); rcutree_report_cpu_dead(); loop {} }
pub unsafe fn smp_send_stop() { smp_call_function(stop_this_cpu, core::ptr::null_mut(), 0); }

pub unsafe fn setup_profiling_timer(_multiplier: u32) -> i32 { 0 }

pub struct flush_tlb_data { pub vma: *mut vm_area_struct, pub addr1: u64, pub addr2: u64 }
unsafe fn flush_tlb_range_ipi(info: *mut core::ffi::c_void) { let fd = &*(info as *const flush_tlb_data); local_flush_tlb_range(fd.vma, fd.addr1, fd.addr2); }
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, start: u64, end: u64) { let fd = flush_tlb_data { vma, addr1:start, addr2:end }; on_each_cpu_mask(mm_cpumask((*vma).vm_mm), flush_tlb_range_ipi, &fd as *const _ as *mut _, 1); }
unsafe fn flush_tlb_kernel_range_ipi(info: *mut core::ffi::c_void) { let fd = &*(info as *const flush_tlb_data); local_flush_tlb_kernel_range(fd.addr1, fd.addr2); }
pub unsafe fn flush_tlb_kernel_range(start: u64, end: u64) { let fd = flush_tlb_data { vma: core::ptr::null_mut(), addr1:start, addr2:end }; on_each_cpu(flush_tlb_kernel_range_ipi, &fd as *const _ as *mut _, 1); }
unsafe fn flush_tlb_page_ipi(info: *mut core::ffi::c_void) { let fd = &*(info as *const flush_tlb_data); local_flush_tlb_page(fd.vma, fd.addr1); }
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, page: u64) { let fd = flush_tlb_data { vma, addr1:page, addr2:0 }; on_each_cpu_mask(mm_cpumask((*vma).vm_mm), flush_tlb_page_ipi, &fd as *const _ as *mut _, 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
