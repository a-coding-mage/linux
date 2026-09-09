// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 * Copyright (C) 2018 Christoph Hellwig
 */

use core::ffi::c_void;

// Declarations supplied by the surrounding kernel translation unit.
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn to_of_node(fwnode: *mut fwnode_handle) -> *mut device_node;
    fn of_property_present(np: *mut device_node, name: *const i8) -> bool;
    fn of_property_read_u32_index(
        np: *mut device_node,
        name: *const i8,
        index: u32,
        out: *mut u32,
    ) -> i32;
    fn scs_is_enabled() -> bool;
    fn scs_alloc(node: i32) -> *mut c_void;
    fn cpu_to_node(cpu: i32) -> i32;
    fn panic(message: *const i8) -> !;
    fn __do_softirq();
    fn on_thread_stack() -> bool;
    fn call_on_irq_stack(arg: *mut c_void, function: unsafe extern "C" fn(*mut pt_regs));
    fn show_ipi_stats(p: *mut seq_file, prec: i32);
    fn irqchip_init();
    fn sbi_ipi_init();
    fn handle_arch_irq_present() -> bool;
}

static mut __get_intc_node: Option<unsafe extern "C" fn() -> *mut fwnode_handle> = None;

#[no_mangle]
pub unsafe extern "C" fn riscv_set_intc_hwnode_fn(
    function: Option<unsafe extern "C" fn() -> *mut fwnode_handle>,
) {
    __get_intc_node = function;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_get_intc_hwnode() -> *mut fwnode_handle {
    if let Some(function) = __get_intc_node {
        return function();
    }

    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn riscv_get_hart_index(
    fwnode: *mut fwnode_handle,
    logical_index: u32,
    hart_index: *mut u32,
) -> i32 {
    let prop_hart_index = b"riscv,hart-indexes\0";
    let np = to_of_node(fwnode);

    if np.is_null() || !of_property_present(np, prop_hart_index.as_ptr() as *const i8) {
        *hart_index = logical_index;
        return 0;
    }

    of_property_read_u32_index(
        np,
        prop_hart_index.as_ptr() as *const i8,
        logical_index,
        hart_index,
    )
}

#[cfg(feature = "CONFIG_IRQ_STACKS")]
mod irq_stacks {
    use super::*;

    // DECLARE_PER_CPU(ulong *, irq_shadow_call_stack_ptr)
    #[cfg(feature = "CONFIG_SHADOW_CALL_STACK")]
    #[no_mangle]
    pub static mut irq_shadow_call_stack_ptr: *mut c_void = core::ptr::null_mut();

    unsafe fn init_irq_scs() {
        if !scs_is_enabled() {
            return;
        }

        for_each_possible_cpu!(cpu, {
            let s = scs_alloc(cpu_to_node(cpu));
            if s.is_null() {
                panic(b"Failed to allocate IRQ shadow call stack resources\n\0".as_ptr() as *const i8);
            }
            irq_shadow_call_stack_ptr = s;
        });
    }

    #[no_mangle]
    pub static mut irq_stack_ptr: *mut c_void = core::ptr::null_mut();

    #[cfg(feature = "CONFIG_VMAP_STACK")]
    unsafe fn init_irq_stacks() {
        for_each_possible_cpu!(cpu, {
            let p = arch_alloc_vmap_stack(IRQ_STACK_SIZE, cpu_to_node(cpu));
            if p.is_null() {
                panic(b"Failed to allocate IRQ stack resources\n\0".as_ptr() as *const i8);
            }
            irq_stack_ptr = p;
        });
    }

    #[cfg(not(feature = "CONFIG_VMAP_STACK"))]
    unsafe fn init_irq_stacks() {
        // irq stack only needs to be 16 byte aligned - not IRQ_STACK_SIZE aligned.
        for_each_possible_cpu!(cpu, {
            irq_stack_ptr = core::ptr::null_mut();
        });
    }

    #[cfg(feature = "CONFIG_SOFTIRQ_ON_OWN_STACK")]
    unsafe extern "C" fn ___do_softirq(_regs: *mut pt_regs) {
        __do_softirq();
    }

    #[cfg(feature = "CONFIG_SOFTIRQ_ON_OWN_STACK")]
    #[no_mangle]
    pub unsafe extern "C" fn do_softirq_own_stack() {
        if on_thread_stack() {
            call_on_irq_stack(core::ptr::null_mut(), ___do_softirq);
        } else {
            __do_softirq();
        }
    }

    extern "C" {
        fn arch_alloc_vmap_stack(size: usize, node: i32) -> *mut c_void;
    }

    extern "C" {
        static IRQ_STACK_SIZE: usize;
    }
}

#[cfg(not(feature = "CONFIG_IRQ_STACKS"))]
unsafe fn init_irq_scs() {}
#[cfg(not(feature = "CONFIG_IRQ_STACKS"))]
unsafe fn init_irq_stacks() {}

#[no_mangle]
pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: i32) -> i32 {
    show_ipi_stats(p, prec);
    0
}

#[no_mangle]
pub unsafe extern "C" fn init_IRQ() {
    init_irq_scs();
    init_irq_stacks();
    irqchip_init();
    if !handle_arch_irq_present() {
        panic(b"No interrupt controller found.\0".as_ptr() as *const i8);
    }
    sbi_ipi_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
