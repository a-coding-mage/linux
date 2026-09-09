// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PPC476 board specific routines
 *
 * Copyright 2010 Torez Smith, IBM Corporation.
 *
 * Based on earlier code:
 *    Matt Porter <mporter@kernel.crashing.org>
 *    Copyright 2002-2005 MontaVista Software Inc.
 *
 *    Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *    Copyright (c) 2003-2005 Zultys Technologies
 *
 *    Rewritten and ported to the merged powerpc tree:
 *    Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

// Linux/kernel and architecture dependencies supplied by other translation units.
#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct Mpic {
    _private: [u8; 0],
}
#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct SmpOps {
    pub probe: Option<unsafe extern "C" fn()>,
    pub message_pass: Option<unsafe extern "C" fn()>,
    pub setup_cpu: Option<unsafe extern "C" fn(c_int)>,
    pub kick_cpu: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub give_timebase: Option<unsafe extern "C" fn()>,
    pub take_timebase: Option<unsafe extern "C" fn()>,
}
extern "C" {
    static mut ppc_md_get_irq: Option<unsafe extern "C" fn() -> c_int>;
    static mut smp_ops: *mut SmpOps;
    fn of_platform_bus_probe(a: *mut c_void, b: *const OfDeviceId, c: *mut c_void);
    fn of_instantiate_rtc();
    fn of_property_present(np: *mut DeviceNode, name: *const c_char) -> bool;
    fn of_device_is_compatible(np: *mut DeviceNode, name: *const c_char) -> bool;
    fn uic_init_tree();
    fn uic_get_irq() -> c_int;
    fn panic(msg: *const c_char) -> !;
    fn mpic_alloc(np: *mut DeviceNode, flags: c_int, reset: c_int, a: c_int, b: c_int, name: *const c_char) -> *mut Mpic;
    fn mpic_init(mpic: *mut Mpic);
    fn mpic_get_irq() -> c_int;
    fn mpic_setup_this_cpu();
    fn of_get_cpu_node(cpu: c_int, thread: *mut c_void) -> *mut DeviceNode;
    fn of_get_property(np: *mut DeviceNode, name: *const c_char, len: *mut c_int) -> *const u64;
    fn __va(addr: u64) -> *mut c_void;
    fn __pa(addr: unsafe extern "C" fn());
    fn smp_wmb();
    fn mb();
    fn start_secondary_47x();
    fn smp_mpic_probe();
    fn smp_mpic_message_pass();
    fn smp_generic_give_timebase();
    fn smp_generic_take_timebase();
    fn mmu_has_feature(feature: u32) -> bool;
    fn udbg_progress(msg: *const c_char, code: c_uint);
    fn ppc4xx_reset_system();
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}
type c_uint = u32;

static ISS4XX_OF_BUS: [OfDeviceId; 5] = [
    OfDeviceId { compatible: b"ibm,plb4\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ibm,plb6\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ibm,opb\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ibm,ebc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn iss4xx_device_probe() -> c_int {
    of_platform_bus_probe(core::ptr::null_mut(), ISS4XX_OF_BUS.as_ptr(), core::ptr::null_mut());
    of_instantiate_rtc();
    0
}
// machine_device_initcall(iss4xx, iss4xx_device_probe);

/* We can have either UICs or MPICs */
unsafe extern "C" fn iss4xx_init_irq() {
    let mut np: *mut DeviceNode = core::ptr::null_mut();
    // for_each_node_with_property(np, "interrupt-controller")
    loop {
        if !np.is_null() && !of_property_present(np, b"interrupts\0".as_ptr() as *const c_char) {
            break;
        }
        break;
    }
    if np.is_null() {
        panic(b"Can't find top level interrupt controller\0".as_ptr() as *const c_char);
    }
    if of_device_is_compatible(np, b"ibm,uic\0".as_ptr() as *const c_char) {
        uic_init_tree();
        ppc_md_get_irq = Some(uic_get_irq);
    // #ifdef CONFIG_MPIC
    } else if of_device_is_compatible(np, b"chrp,open-pic\0".as_ptr() as *const c_char) {
        let mpic = mpic_alloc(np, 0, 1, 0, 0, b" MPIC     \0".as_ptr() as *const c_char);
        assert!(!mpic.is_null());
        mpic_init(mpic);
        ppc_md_get_irq = Some(mpic_get_irq);
    // #endif
    } else {
        panic(b"Unrecognized top level interrupt controller\0".as_ptr() as *const c_char);
    }
}

// #ifdef CONFIG_SMP
unsafe extern "C" fn smp_iss4xx_setup_cpu(_cpu: c_int) {
    mpic_setup_this_cpu();
}
unsafe extern "C" fn smp_iss4xx_kick_cpu(cpu: c_int) -> c_int {
    let cpunode = of_get_cpu_node(cpu, core::ptr::null_mut());
    assert!(!cpunode.is_null());
    let spin_table_addr_prop = of_get_property(cpunode, b"cpu-release-addr\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if spin_table_addr_prop.is_null() {
        pr_err(b"CPU%d: Can't start, missing cpu-release-addr !\n\0".as_ptr() as *const c_char, cpu);
        return -2;
    }
    let spin_table = __va(*spin_table_addr_prop) as *mut u32;
    pr_debug(b"CPU%d: Spin table mapped at %p\n\0".as_ptr() as *const c_char, cpu, spin_table);
    *spin_table.add(3) = cpu as u32;
    smp_wmb();
    *spin_table.add(1) = __pa(start_secondary_47x) as u32;
    mb();
    0
}
static mut ISS_SMP_OPS: SmpOps = SmpOps {
    probe: Some(smp_mpic_probe), message_pass: Some(smp_mpic_message_pass),
    setup_cpu: Some(smp_iss4xx_setup_cpu), kick_cpu: Some(smp_iss4xx_kick_cpu),
    give_timebase: Some(smp_generic_give_timebase), take_timebase: Some(smp_generic_take_timebase),
};
unsafe extern "C" fn iss4xx_smp_init() {
    if mmu_has_feature(1) { smp_ops = &raw mut ISS_SMP_OPS; }
}
// #else
unsafe extern "C" fn iss4xx_smp_init() { }
// #endif

unsafe extern "C" fn iss4xx_setup_arch() { iss4xx_smp_init(); }

// define_machine(iss4xx) {
//     .name = "ISS-4xx", .compatible = "ibm,iss-4xx",
//     .progress = udbg_progress, .init_IRQ = iss4xx_init_irq,
//     .setup_arch = iss4xx_setup_arch, .restart = ppc4xx_reset_system,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
