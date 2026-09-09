// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 SMP routines.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

// Linux kernel and architecture dependencies are supplied by other files.

#[cfg(debug_assertions)]
macro_rules! DBG {
    ($($arg:tt)*) => { unsafe { udbg_printf(format_args!($($arg)*)); } };
}

#[cfg(not(debug_assertions))]
macro_rules! DBG {
    ($($arg:tt)*) => { unsafe { pr_debug(format_args!($($arg)*)); } };
}

const MSG_COUNT: usize = 4;

// Equivalent to DEFINE_PER_CPU(unsigned int [MSG_COUNT], ps3_ipi_virqs).
static mut ps3_ipi_virqs: [[u32; MSG_COUNT]; 2] = [[0; MSG_COUNT]; 2];

unsafe extern "C" {
    fn ps3_send_event_locally(virq: u32) -> i32;
    fn ps3_event_receive_port_setup(cpu: i32, virq: *mut u32) -> i32;
    fn smp_request_message_ipi(virq: u32, msg: i32) -> i32;
    fn ps3_register_ipi_irq(cpu: i32, virq: u32);
    fn ps3_register_ipi_debug_brk(cpu: i32, virq: u32);
    fn ps3_event_receive_port_destroy(virq: u32);
    fn smp_generic_kick_cpu(cpu: i32);
    fn udbg_printf(args: core::fmt::Arguments<'_>);
    fn pr_debug(args: core::fmt::Arguments<'_>);
}

// These constants are provided by asm/smp.h.
const PPC_MSG_CALL_FUNCTION: i32 = 0;
const PPC_MSG_RESCHEDULE: i32 = 1;
const PPC_MSG_TICK_BROADCAST: i32 = 2;
const PPC_MSG_NMI_IPI: usize = 3;

#[repr(C)]
struct smp_ops_t {
    probe: Option<unsafe extern "C" fn()>,
    message_pass: Option<unsafe extern "C" fn(i32, i32)>,
    kick_cpu: Option<unsafe extern "C" fn(i32)>,
}

unsafe extern "C" {
    static mut smp_ops: *mut smp_ops_t;
}

unsafe extern "C" fn ps3_smp_message_pass(cpu: i32, msg: i32) {
    let result: i32;
    let virq: u32;

    if msg >= MSG_COUNT as i32 {
        DBG!("{}:{}: bad msg: {}\n", "ps3_smp_message_pass", line!(), msg);
        return;
    }

    virq = ps3_ipi_virqs[cpu as usize][msg as usize];
    result = ps3_send_event_locally(virq);

    if result != 0 {
        DBG!("{}:{}: ps3_send_event_locally({}, {}) failed ({})\n",
            "ps3_smp_message_pass", line!(), cpu, msg, result);
    }
}

unsafe extern "C" fn ps3_smp_probe() {
    let mut cpu: i32 = 0;

    while cpu < 2 {
        let mut result: i32;
        let virqs: *mut u32 = ps3_ipi_virqs[cpu as usize].as_mut_ptr();
        let mut i: i32 = 0;

        DBG!(" -> {}:{}: ({})\n", "ps3_smp_probe", line!(), cpu);

        // Check assumptions on ps3_ipi_virqs[] indexing.
        const _: () = assert!(PPC_MSG_CALL_FUNCTION == 0);
        const _: () = assert!(PPC_MSG_RESCHEDULE == 1);
        const _: () = assert!(PPC_MSG_TICK_BROADCAST == 2);
        const _: () = assert!(PPC_MSG_NMI_IPI == 3);

        while i < MSG_COUNT as i32 {
            result = ps3_event_receive_port_setup(cpu, virqs.add(i as usize));

            if result != 0 {
                i += 1;
                continue;
            }

            DBG!("{}:{}: ({}, {}) => virq {}\n",
                "ps3_smp_probe", line!(), cpu, i, *virqs.add(i as usize));

            result = smp_request_message_ipi(*virqs.add(i as usize), i);

            if result != 0 {
                *virqs.add(i as usize) = 0;
            } else {
                ps3_register_ipi_irq(cpu, *virqs.add(i as usize));
            }
            i += 1;
        }

        ps3_register_ipi_debug_brk(cpu, *virqs.add(PPC_MSG_NMI_IPI));
        DBG!(" <- {}:{}: ({})\n", "ps3_smp_probe", line!(), cpu);
        cpu += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ps3_smp_cleanup_cpu(cpu: i32) {
    let virqs: *mut u32 = ps3_ipi_virqs[cpu as usize].as_mut_ptr();
    let mut i: i32 = 0;

    DBG!(" -> {}:{}: ({})\n", "ps3_smp_cleanup_cpu", line!(), cpu);

    while i < MSG_COUNT as i32 {
        // Can't call free_irq from interrupt context.
        ps3_event_receive_port_destroy(*virqs.add(i as usize));
        *virqs.add(i as usize) = 0;
        i += 1;
    }

    DBG!(" <- {}:{}: ({})\n", "ps3_smp_cleanup_cpu", line!(), cpu);
}

static mut ps3_smp_ops: smp_ops_t = smp_ops_t {
    probe: Some(ps3_smp_probe),
    message_pass: Some(ps3_smp_message_pass),
    kick_cpu: Some(smp_generic_kick_cpu),
};

#[no_mangle]
pub unsafe extern "C" fn smp_init_ps3() {
    DBG!(" -> {}\n", "smp_init_ps3");
    smp_ops = &raw mut ps3_smp_ops;
    DBG!(" <- {}\n", "smp_init_ps3");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
