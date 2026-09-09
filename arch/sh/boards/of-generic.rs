// SPDX-License-Identifier: GPL-2.0
/*
 * SH generic board support, using device tree
 *
 * Copyright (C) 2015-2016 Smart Energy Instruments, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_SMP)]
unsafe extern "C" {
    static __cpu_method_of_table: [OfCpuMethod; 0];
    fn native_cpu_die();
    fn native_cpu_disable() -> i32;
    fn native_play_dead();
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn cpumask_of(cpu: u32) -> *mut core::ffi::c_void;
    fn init_cpu_possible(mask: *mut core::ffi::c_void);
    fn set_cpu_possible(cpu: u64, possible: bool);
    fn set_cpu_present(cpu: u64, present: bool);
    static mut __cpu_number_map: [u32; 0];
    static mut __cpu_logical_map: [u32; 0];
    fn of_get_cpu_hwid(np: *mut DeviceNode, index: u32) -> u64;
    fn of_property_read_string(
        np: *mut DeviceNode,
        property: *const core::ffi::c_char,
        value: *mut *const core::ffi::c_char,
    ) -> i32;
    fn of_find_node_by_name(
        from: *mut DeviceNode,
        name: *const core::ffi::c_char,
    ) -> *mut DeviceNode;
    fn of_node_put(np: *mut DeviceNode);
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn register_smp_ops(ops: *const PlatSmpOps);
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfCpuMethod {
    pub method: *const core::ffi::c_char,
    pub ops: *const PlatSmpOps,
}

#[repr(C)]
pub struct PlatSmpOps {
    pub smp_setup: Option<unsafe extern "C" fn()>,
    pub prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub start_cpu: Option<unsafe extern "C" fn(u32, u64)>,
    pub smp_processor_id: Option<unsafe extern "C" fn() -> u32>,
    pub send_ipi: Option<unsafe extern "C" fn(u32, u32)>,
    pub cpu_die: Option<unsafe extern "C" fn()>,
    pub cpu_disable: Option<unsafe extern "C" fn() -> i32>,
    pub play_dead: Option<unsafe extern "C" fn()>,
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn dummy_smp_setup() {}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn dummy_prepare_cpus(_max_cpus: u32) {}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn dummy_start_cpu(_cpu: u32, _entry_point: u64) {}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn dummy_smp_processor_id() -> u32 {
    0
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn dummy_send_ipi(_cpu: u32, _message: u32) {}

#[cfg(CONFIG_SMP)]
static mut dummy_smp_ops: PlatSmpOps = PlatSmpOps {
    smp_setup: Some(dummy_smp_setup),
    prepare_cpus: Some(dummy_prepare_cpus),
    start_cpu: Some(dummy_start_cpu),
    smp_processor_id: Some(dummy_smp_processor_id),
    send_ipi: Some(dummy_send_ipi),
    cpu_die: Some(native_cpu_die),
    cpu_disable: Some(native_cpu_disable),
    play_dead: Some(native_play_dead),
};

#[cfg(CONFIG_SMP)]
#[no_mangle]
pub static __cpu_method_of_table_sentinel: OfCpuMethod = OfCpuMethod {
    method: core::ptr::null(),
    ops: core::ptr::null(),
};

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn sh_of_smp_probe() {
    let mut np: *mut DeviceNode;
    let mut method: *const core::ffi::c_char = core::ptr::null();
    let mut m: *const OfCpuMethod = __cpu_method_of_table.as_ptr();

    pr_info(b"SH generic board support: scanning for cpus\0".as_ptr() as *const _);

    init_cpu_possible(cpumask_of(0));

    // for_each_of_cpu_node(np)
    // The device-tree iterator is supplied by the surrounding translation.
    np = core::ptr::null_mut();
    while !np.is_null() {
        let id = of_get_cpu_hwid(np, 0);
        if id < NR_CPUS as u64 {
            if method.is_null() {
                of_property_read_string(
                    np,
                    b"enable-method\0".as_ptr() as *const _,
                    &mut method,
                );
            }
            set_cpu_possible(id, true);
            set_cpu_present(id, true);
            __cpu_number_map[id as usize] = id as u32;
            __cpu_logical_map[id as usize] = id as u32;
        }
        break;
    }
    if method.is_null() {
        np = of_find_node_by_name(core::ptr::null_mut(), b"cpus\0".as_ptr() as *const _);
        of_property_read_string(np, b"enable-method\0".as_ptr() as *const _, &mut method);
        of_node_put(np);
    }

    pr_info(b"CPU enable method: %s\n\0".as_ptr() as *const _, method);
    if !method.is_null() {
        while !(*m).method.is_null() {
            if strcmp((*m).method, method) == 0 {
                register_smp_ops((*m).ops);
                return;
            }
            m = m.add(1);
        }
    }
    register_smp_ops(&dummy_smp_ops);
}

#[cfg(not(CONFIG_SMP))]
unsafe extern "C" fn sh_of_smp_probe() {}

unsafe extern "C" fn noop() {}

unsafe extern "C" fn noopi() -> i32 {
    0
}

unsafe extern "C" fn sh_of_mem_reserve() {
    early_init_fdt_reserve_self();
    early_init_fdt_scan_reserved_mem();
}

unsafe extern "C" fn sh_of_setup(cmdline_p: *mut *mut core::ffi::c_char) {
    let mut root: *mut DeviceNode;

    sh_mv.mv_name = b"Unknown SH model\0".as_ptr() as *const _;
    root = of_find_node_by_path(b"/\0".as_ptr() as *const _);
    if !root.is_null() {
        of_property_read_string(root, b"model\0".as_ptr() as *const _, &mut sh_mv.mv_name);
        of_node_put(root);
    }

    sh_of_smp_probe();
}

unsafe extern "C" fn sh_of_irq_demux(irq: i32) -> i32 {
    /* FIXME: eventually this should not be used at all;
     * the interrupt controller should set_handle_irq(). */
    irq
}

unsafe extern "C" fn sh_of_init_irq() {
    pr_info(b"SH generic board support: scanning for interrupt controllers\n\0".as_ptr() as *const _);
    irqchip_init();
}

unsafe extern "C" fn sh_of_clk_init() -> i32 {
    #[cfg(CONFIG_COMMON_CLK)]
    {
        /* Disabled pending move to COMMON_CLK framework. */
        pr_info(b"SH generic board support: scanning for clk providers\n\0".as_ptr() as *const _);
        of_clk_init(core::ptr::null());
    }
    0
}

#[repr(C)]
pub struct ShMachineVector {
    pub mv_setup: Option<unsafe extern "C" fn(*mut *mut core::ffi::c_char)>,
    pub mv_name: *const core::ffi::c_char,
    pub mv_irq_demux: Option<unsafe extern "C" fn(i32) -> i32>,
    pub mv_init_irq: Option<unsafe extern "C" fn()>,
    pub mv_clk_init: Option<unsafe extern "C" fn() -> i32>,
    pub mv_mode_pins: Option<unsafe extern "C" fn() -> i32>,
    pub mv_mem_init: Option<unsafe extern "C" fn()>,
    pub mv_mem_reserve: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mut sh_of_generic_mv: ShMachineVector = ShMachineVector {
    mv_setup: Some(sh_of_setup),
    mv_name: b"devicetree\0".as_ptr() as *const _,
    mv_irq_demux: Some(sh_of_irq_demux),
    mv_init_irq: Some(sh_of_init_irq),
    mv_clk_init: Some(sh_of_clk_init),
    mv_mode_pins: Some(noopi),
    mv_mem_init: Some(noop),
    mv_mem_reserve: Some(sh_of_mem_reserve),
};

#[repr(C)]
pub struct ShClkOps {
    _private: [u8; 0],
}

pub unsafe extern "C" fn arch_init_clk_ops(_ops: *mut *mut ShClkOps, _idx: i32) {}

pub unsafe extern "C" fn plat_irq_setup() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
