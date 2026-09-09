// SPDX-License-Identifier: GPL-2.0

// Linux and architecture headers supplying the declarations used below.
// The CONFIG_CPU_HAS_FPU conditional dependency is preserved externally.

#[repr(C)]
#[derive(Copy, Clone)]
enum IpiMessageType {
    IpiEmpty,
    IpiReschedule,
    IpiCallFunc,
    IpiIrqWork,
    IpiMax,
}

#[repr(C)]
struct IpiDataStruct {
    bits: ::core::ffi::c_ulong,
    stats: [::core::ffi::c_ulong; IpiMessageType::IpiMax as usize],
}

// DEFINE_PER_CPU(struct ipi_data_struct, ipi_data);
extern "C" {
    static mut ipi_data: IpiDataStruct;
}

unsafe fn handle_ipi(_irq: ::core::ffi::c_int, _dev: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let stats = (*this_cpu_ptr(&raw mut ipi_data)).stats.as_mut_ptr();

    loop {
        let ops: ::core::ffi::c_ulong = xchg(&mut (*this_cpu_ptr(&raw mut ipi_data)).bits, 0);
        if ops == 0 {
            return IRQ_HANDLED;
        }

        if ops & (1 << IpiMessageType::IpiReschedule as usize) != 0 {
            *stats.add(IpiMessageType::IpiReschedule as usize) += 1;
            scheduler_ipi();
        }

        if ops & (1 << IpiMessageType::IpiCallFunc as usize) != 0 {
            *stats.add(IpiMessageType::IpiCallFunc as usize) += 1;
            generic_smp_call_function_interrupt();
        }

        if ops & (1 << IpiMessageType::IpiIrqWork as usize) != 0 {
            *stats.add(IpiMessageType::IpiIrqWork as usize) += 1;
            irq_work_run();
        }

        BUG_ON((ops >> IpiMessageType::IpiMax as usize) != 0);
    }
}

static mut send_arch_ipi: Option<unsafe extern "C" fn(*const Cpumask)> = None;
static mut ipi_irq: ::core::ffi::c_int = 0;

unsafe extern "C" fn set_send_ipi(func: unsafe extern "C" fn(*const Cpumask), irq: ::core::ffi::c_int) {
    if send_arch_ipi.is_some() {
        return;
    }
    send_arch_ipi = Some(func);
    ipi_irq = irq;
}

unsafe fn send_ipi_message(to_whom: *const Cpumask, operation: IpiMessageType) {
    let mut i: ::core::ffi::c_int = 0;
    for_each_cpu(&mut i, to_whom);
    while i >= 0 {
        set_bit(operation as usize, &mut (*per_cpu_ptr(&raw mut ipi_data, i as usize)).bits);
        i = next_cpu(i, to_whom);
    }

    smp_mb();
    if let Some(func) = send_arch_ipi {
        func(to_whom);
    }
}

static ipi_names: [&'static [u8]; IpiMessageType::IpiMax as usize] = [
    b"Empty interrupts\0",
    b"Rescheduling interrupts\0",
    b"Function call interrupts\0",
    b"Irq work interrupts\0",
];

unsafe extern "C" fn arch_show_interrupts(p: *mut SeqFile, prec: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i = 0;
    while i < IpiMessageType::IpiMax as usize {
        seq_printf(p, prec - 1, b"IPI\0".as_ptr(), i, if prec >= 4 { b" \0".as_ptr() } else { b"\0".as_ptr() });
        let mut cpu = 0;
        for_each_online_cpu(&mut cpu);
        while cpu >= 0 {
            seq_printf(p, 10, b"\0".as_ptr(), (*per_cpu_ptr(&raw mut ipi_data, cpu as usize)).stats[i]);
            cpu = next_online_cpu(cpu);
        }
        seq_printf(p, 0, b" %s\n\0".as_ptr(), ipi_names[i].as_ptr());
        i += 1;
    }
    0
}

unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *mut Cpumask) {
    send_ipi_message(mask, IpiMessageType::IpiCallFunc);
}

unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: ::core::ffi::c_int) {
    send_ipi_message(cpumask_of(cpu), IpiMessageType::IpiCallFunc);
}

unsafe extern "C" fn ipi_stop(_unused: *mut ::core::ffi::c_void) {
    loop {}
}

unsafe extern "C" fn smp_send_stop() {
    on_each_cpu(ipi_stop, core::ptr::null_mut(), 1);
}

unsafe extern "C" fn arch_smp_send_reschedule(cpu: ::core::ffi::c_int) {
    send_ipi_message(cpumask_of(cpu), IpiMessageType::IpiReschedule);
}

// #ifdef CONFIG_IRQ_WORK
unsafe extern "C" fn arch_irq_work_raise() {
    send_ipi_message(cpumask_of(smp_processor_id()), IpiMessageType::IpiIrqWork);
}

unsafe extern "C" fn smp_prepare_cpus(_max_cpus: ::core::ffi::c_uint) {}

static mut ipi_dummy_dev: ::core::ffi::c_int = 0;

unsafe extern "C" fn setup_smp_ipi() {
    if ipi_irq == 0 {
        return;
    }
    let rc = request_percpu_irq(ipi_irq, handle_ipi, b"IPI Interrupt\0".as_ptr(), &mut ipi_dummy_dev as *mut _ as *mut _);
    if rc != 0 {
        panic!("%s IRQ request failed\n", "setup_smp_ipi");
    }
    enable_percpu_irq(ipi_irq, 0);
}

unsafe extern "C" fn setup_smp() {
    let mut node: *mut DeviceNode = core::ptr::null_mut();
    let mut cpu: ::core::ffi::c_uint;
    for_each_of_cpu_node(&mut node);
    while !node.is_null() {
        if !of_device_is_available(node) {
            node = next_of_cpu_node(node);
            continue;
        }
        cpu = of_get_cpu_hwid(node, 0);
        if cpu < NR_CPUS {
            set_cpu_possible(cpu, true);
            set_cpu_present(cpu, true);
        }
        node = next_of_cpu_node(node);
    }
}

extern "C" {
    fn _start_smp_secondary();
    static mut secondary_hint: ::core::ffi::c_uint;
    static mut secondary_hint2: ::core::ffi::c_uint;
    static mut secondary_ccr: ::core::ffi::c_uint;
    static mut secondary_stack: ::core::ffi::c_uint;
    static mut secondary_msa1: ::core::ffi::c_uint;
    static mut secondary_pgd: ::core::ffi::c_uint;
}

unsafe extern "C" fn __cpu_up(cpu: ::core::ffi::c_uint, tidle: *mut TaskStruct) -> ::core::ffi::c_int {
    let mut mask: ::core::ffi::c_ulong = 1 << cpu;
    secondary_stack = task_stack_page(tidle) as ::core::ffi::c_uint + THREAD_SIZE - 8;
    secondary_hint = mfcr(b"cr31\0".as_ptr());
    secondary_hint2 = mfcr(b"cr<21, 1>\0".as_ptr());
    secondary_ccr = mfcr(b"cr18\0".as_ptr());
    secondary_msa1 = read_mmu_msa1();
    secondary_pgd = mfcr(b"cr<29, 15>\0".as_ptr());
    // Flush data from cache because the other CPUs are in reset status.
    mtcr(b"cr17\0".as_ptr(), 0x22);
    if mask & mfcr(b"cr<29, 0>\0".as_ptr()) != 0 {
        if let Some(func) = send_arch_ipi { func(cpumask_of(cpu as ::core::ffi::c_int)); }
    } else {
        mask |= mfcr(b"cr<29, 0>\0".as_ptr());
        mtcr(b"cr<29, 0>\0".as_ptr(), mask);
    }
    while !cpu_online(cpu) {}
    secondary_stack = 0;
    0
}

unsafe extern "C" fn smp_cpus_done(_max_cpus: ::core::ffi::c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
