// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for J2 processor
 *
 * Copyright (C) 2015-2016 Smart Energy Instruments, Inc.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[no_mangle]
pub static mut j2_ipi_messages: [u32; NR_CPUS] = [0; NR_CPUS];

extern "C" {
    static mut sh2_cpuid_addr: *mut u32;
}

static mut j2_ipi_trigger: *mut u32 = core::ptr::null_mut();
static mut j2_ipi_irq: i32 = 0;

unsafe extern "C" fn j2_ipi_interrupt_handler(_irq: i32, _arg: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu: u32 = hard_smp_processor_id();
    let pmsg: *mut u32 = per_cpu_ptr(&raw mut j2_ipi_messages, cpu);
    let (messages, _) : (u32, u32);
    let mut current: u32;

    loop {
        current = core::ptr::read_volatile(pmsg);
        if cmpxchg(pmsg, current, 0) == current {
            break;
        }
    }
    messages = current;

    if messages == 0 {
        return IRQ_NONE;
    }

    let mut i = 0;
    while i < SMP_MSG_NR {
        if messages & (1u32 << i) != 0 {
            smp_message_recv(i);
        }
        i += 1;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn j2_smp_setup() {}

unsafe extern "C" fn j2_prepare_cpus(max_cpus: u32) {
    let mut np: *mut device_node;
    let mut i: u32;
    let mut max: u32 = 1;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"jcore,ipi-controller\0".as_ptr() as *const i8);
    if np.is_null() {
        goto_out(max);
        return;
    }

    j2_ipi_irq = irq_of_parse_and_map(np, 0);
    j2_ipi_trigger = of_iomap(np, 0);
    if j2_ipi_irq == 0 || j2_ipi_trigger.is_null() {
        goto_out(max);
        return;
    }

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"jcore,cpuid-mmio\0".as_ptr() as *const i8);
    if np.is_null() {
        goto_out(max);
        return;
    }

    sh2_cpuid_addr = of_iomap(np, 0);
    if sh2_cpuid_addr.is_null() {
        goto_out(max);
        return;
    }

    if request_irq(j2_ipi_irq, Some(j2_ipi_interrupt_handler), IRQF_PERCPU,
                   b"ipi\0".as_ptr() as *const i8,
                   j2_ipi_interrupt_handler as *mut core::ffi::c_void) != 0 {
        goto_out(max);
        return;
    }

    max = max_cpus;
    goto_out(max);
}

unsafe fn goto_out(max: u32) {
    let mut i = max;
    while i < NR_CPUS {
        set_cpu_possible(i, false);
        set_cpu_present(i, false);
        i += 1;
    }
}

unsafe extern "C" fn j2_start_cpu(cpu: u32, entry_point: u64) {
    let np: *mut device_node;
    let mut regs = [0u32; 2];
    let release: *mut core::ffi::c_void;
    let initpc: *mut core::ffi::c_void;

    if cpu == 0 { return; }

    np = of_get_cpu_node(cpu, core::ptr::null_mut());
    if np.is_null() { return; }

    if of_property_read_u32_array(np, b"cpu-release-addr\0".as_ptr() as *const i8,
                                  regs.as_mut_ptr(), 2) != 0 { return; }
    release = ioremap(regs[0] as u64, core::mem::size_of::<u32>());
    initpc = ioremap(regs[1] as u64, core::mem::size_of::<u32>());

    __raw_writel(entry_point as u32, initpc);
    __raw_writel(1, release);

    iounmap(initpc);
    iounmap(release);

    pr_info!("J2 SMP: requested start of cpu {}\n", cpu);
}

unsafe extern "C" fn j2_smp_processor_id() -> u32 {
    __raw_readl(sh2_cpuid_addr)
}

unsafe extern "C" fn j2_send_ipi(cpu: u32, message: u32) {
    let pmsg: *mut u32;
    let mut old: u32;
    let val: u32;

    /* There is only one IPI interrupt shared by all messages, so
     * we keep a separate interrupt flag per message type in sw. */
    pmsg = per_cpu_ptr(&raw mut j2_ipi_messages, cpu);
    loop {
        old = core::ptr::read_volatile(pmsg);
        if cmpxchg(pmsg, old, old | (1u32 << message)) == old { break; }
    }

    /* Generate the actual interrupt by writing to CCRn bit 28. */
    val = __raw_readl(j2_ipi_trigger.add(cpu as usize));
    __raw_writel(val | (1u32 << 28), j2_ipi_trigger.add(cpu as usize));
}

#[repr(C)]
struct plat_smp_ops {
    smp_setup: Option<unsafe extern "C" fn()>,
    prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    start_cpu: Option<unsafe extern "C" fn(u32, u64)>,
    smp_processor_id: Option<unsafe extern "C" fn() -> u32>,
    send_ipi: Option<unsafe extern "C" fn(u32, u32)>,
    cpu_die: Option<unsafe extern "C" fn()>,
    cpu_disable: Option<unsafe extern "C" fn() -> i32>,
    play_dead: Option<unsafe extern "C" fn()>,
}

static mut j2_smp_ops: plat_smp_ops = plat_smp_ops {
    smp_setup: Some(j2_smp_setup),
    prepare_cpus: Some(j2_prepare_cpus),
    start_cpu: Some(j2_start_cpu),
    smp_processor_id: Some(j2_smp_processor_id),
    send_ipi: Some(j2_send_ipi),
    cpu_die: Some(native_cpu_die),
    cpu_disable: Some(native_cpu_disable),
    play_dead: Some(native_play_dead),
};

// CPU_METHOD_OF_DECLARE(j2_cpu_method, "jcore,spin-table", &j2_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
