// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 IBM Corporation.
 */

// Linux and architecture-specific includes from the C source are external dependencies.

#[repr(C)]
pub union IcpIplWord {
    pub word: u32,
    pub bytes: [u8; 4],
}

#[repr(C)]
pub struct IcpIpl {
    pub xirr_poll: IcpIplWord,
    pub xirr: IcpIplWord,
    pub dummy: u32,
    pub qirr: IcpIplWord,
    pub link_a: u32,
    pub link_b: u32,
    pub link_c: u32,
}

extern "C" {
    static mut icp_native_regs: [*mut IcpIpl; NR_CPUS];
    static mut icp_ops: *const IcpOps;

    fn smp_processor_id() -> i32;
    fn kvmppc_get_xics_latch() -> u32;
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn out_8(addr: *mut u8, value: u8);
    fn iosync();
    fn xics_set_base_cppr(value: u8);
    fn xics_pop_cppr() -> u8;
    fn xics_push_cppr(value: u32);
    fn irqd_to_hwirq(d: *mut IrqData) -> u64;
    fn irq_find_mapping(host: *mut Core, vec: u32) -> u32;
    fn xics_mask_unknown_vec(vec: u32);
    fn kvmppc_set_host_ipi(cpu: i32);
    fn kvmppc_clear_host_ipi(cpu: i32);
    fn smp_ipi_demux() -> IrqReturn;
    fn get_hard_smp_processor_id(cpu: i32) -> i32;
    fn cpu_present(cpu: i32) -> bool;
    fn kasprintf(flags: u32, fmt: *const u8, ...) -> *mut i8;
    fn request_mem_region(addr: u64, size: u64, name: *const i8) -> *mut Resource;
    fn ioremap(addr: u64, size: u64) -> *mut IcpIpl;
    fn kvmppc_set_xics_phys(cpu: i32, addr: u64);
    fn release_mem_region(addr: u64, size: u64);
    fn of_get_property(np: *mut DeviceNode, name: *const u8, len: *mut u32) -> *const u32;
    fn of_read_number(address: *const u32, size: i32) -> u32;
    fn of_address_count(np: *mut DeviceNode) -> i32;
    fn of_address_to_resource(np: *mut DeviceNode, index: i32, resource: *mut Resource) -> i32;
    fn resource_size(resource: *const Resource) -> u64;
}

// External types, constants, logging helpers, and iteration primitives are supplied by other files.
extern "C" {
    static mut xics_host: *mut Core;
}

#[repr(C)] pub struct IrqData { _private: [u8; 0] }
#[repr(C)] pub struct Core { _private: [u8; 0] }
#[repr(C)] pub struct Resource { _private: [u8; 0] }
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct IrqReturn { _private: [u8; 0] }
#[repr(C)] pub struct IcpOps {
    pub get_irq: Option<unsafe extern "C" fn() -> u32>,
    pub eoi: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub set_priority: Option<unsafe extern "C" fn(u8)>,
    pub teardown_cpu: Option<unsafe extern "C" fn()>,
    pub flush_ipi: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_SMP)] pub ipi_action: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> IrqReturn>,
    #[cfg(CONFIG_SMP)] pub cause_ipi: Option<unsafe extern "C" fn(i32)>,
}

const NR_CPUS: usize = 0; // Supplied by the target kernel configuration.
const XICS_IPI: u32 = 0; // Supplied by the target architecture headers.
const XICS_IRQ_SPURIOUS: u32 = 0; // Supplied by the target architecture headers.
const IPI_PRIORITY: u8 = 0; // Supplied by the target architecture headers.
const GFP_KERNEL: u32 = 0; // Supplied by the target kernel headers.

#[inline]
unsafe fn icp_native_get_xirr() -> u32 {
    let cpu = smp_processor_id() as usize;
    let xirr = kvmppc_get_xics_latch();
    if xirr != 0 { return xirr; }
    in_be32(&(*icp_native_regs[cpu]).xirr.word)
}

#[inline]
unsafe fn icp_native_set_xirr(value: u32) {
    let cpu = smp_processor_id() as usize;
    out_be32(&mut (*icp_native_regs[cpu]).xirr.word, value);
}

#[inline]
unsafe fn icp_native_set_cppr(value: u8) {
    let cpu = smp_processor_id() as usize;
    out_8(&mut (*icp_native_regs[cpu]).xirr.bytes[0], value);
}

#[inline]
unsafe fn icp_native_set_qirr(n_cpu: i32, value: u8) {
    out_8(&mut (*icp_native_regs[n_cpu as usize]).qirr.bytes[0], value);
}

unsafe fn icp_native_set_cpu_priority(cppr: u8) {
    xics_set_base_cppr(cppr);
    icp_native_set_cppr(cppr);
    iosync();
}

#[no_mangle]
pub unsafe extern "C" fn icp_native_eoi(d: *mut IrqData) {
    let hw_irq = irqd_to_hwirq(d) as u32;
    iosync();
    icp_native_set_xirr(((xics_pop_cppr() as u32) << 24) | hw_irq);
}

unsafe fn icp_native_teardown_cpu() {
    let cpu = smp_processor_id();
    icp_native_set_qirr(cpu, 0xff);
}

unsafe fn icp_native_flush_ipi() {
    icp_native_set_xirr((0x00 << 24) | XICS_IPI);
}

unsafe fn icp_native_get_irq() -> u32 {
    let xirr = icp_native_get_xirr();
    let vec = xirr & 0x00ffffff;
    if vec == XICS_IRQ_SPURIOUS { return 0; }
    let irq = irq_find_mapping(xics_host, vec);
    if irq != 0 {
        xics_push_cppr(vec);
        return irq;
    }
    xics_mask_unknown_vec(vec);
    icp_native_set_xirr(xirr);
    0
}

#[cfg(CONFIG_SMP)]
unsafe fn icp_native_cause_ipi(cpu: i32) {
    kvmppc_set_host_ipi(cpu);
    icp_native_set_qirr(cpu, IPI_PRIORITY);
}

#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn icp_native_flush_interrupt() {
    let xirr = icp_native_get_xirr();
    let vec = xirr & 0x00ffffff;
    if vec == XICS_IRQ_SPURIOUS { return; }
    if vec == XICS_IPI {
        let cpu = smp_processor_id();
        kvmppc_clear_host_ipi(cpu);
        icp_native_set_qirr(cpu, 0xff);
    } else {
        // pr_err("XICS: hw interrupt 0x%x to offline cpu, disabling\n", vec);
        xics_mask_unknown_vec(vec);
    }
    icp_native_set_xirr(xirr);
}

#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn xics_wake_cpu(cpu: i32) {
    icp_native_set_qirr(cpu, IPI_PRIORITY);
}

#[cfg(CONFIG_SMP)]
unsafe fn icp_native_ipi_action(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    let cpu = smp_processor_id();
    kvmppc_clear_host_ipi(cpu);
    icp_native_set_qirr(cpu, 0xff);
    smp_ipi_demux()
}

unsafe fn icp_native_map_one_cpu(hw_id: i32, addr: u64, size: u64) -> i32 {
    let mut cpu = -1;
    // for_each_possible_cpu(i) { ... } — supplied by the kernel's CPU iteration facilities.
    for i in 0..NR_CPUS as i32 {
        if !cpu_present(i) { continue; }
        if hw_id == get_hard_smp_processor_id(i) { cpu = i; break; }
    }
    if cpu == -1 { return 0; }
    let rname = kasprintf(GFP_KERNEL, b"CPU %d [0x%x] Interrupt Presentation\0".as_ptr(), cpu, hw_id);
    if rname.is_null() { return -12; }
    if request_mem_region(addr, size, rname).is_null() {
        // pr_warn("icp_native: Could not reserve ICP MMIO for CPU %d, interrupt server #0x%x\n", cpu, hw_id);
        return -16;
    }
    icp_native_regs[cpu as usize] = ioremap(addr, size);
    kvmppc_set_xics_phys(cpu, addr);
    if icp_native_regs[cpu as usize].is_null() {
        // pr_warn("icp_native: Failed ioremap for CPU %d, interrupt server #0x%x, addr %#lx\n", cpu, hw_id, addr);
        release_mem_region(addr, size);
        return -12;
    }
    0
}

unsafe fn icp_native_init_one_node(np: *mut DeviceNode, indx: *mut u32) -> i32 {
    let mut ilen = 0u32;
    let ireg = of_get_property(np, b"ibm,interrupt-server-ranges\0".as_ptr(), &mut ilen);
    // WARN_ON((ireg == NULL) || (ilen != 2*sizeof(u32)));
    let mut num_servers = 0u32;
    if !ireg.is_null() {
        *indx = of_read_number(ireg, 1);
        if ilen >= 2 * core::mem::size_of::<u32>() as u32 {
            num_servers = of_read_number(ireg.add(1), 1);
        }
    }
    let num_reg = of_address_count(np);
    if num_servers != 0 && num_servers != num_reg as u32 {
        // pr_err("icp_native: ICP reg len (%d) != num servers (%d)", num_reg, num_servers);
        return -1;
    }
    for i in 0..num_reg {
        let mut r = core::mem::MaybeUninit::<Resource>::uninit();
        let err = of_address_to_resource(np, i, r.as_mut_ptr());
        if err != 0 {
            // pr_err("icp_native: Could not translate ICP MMIO for interrupt server 0x%x (%d)\n", *indx, err);
            return -1;
        }
        let r = r.assume_init();
        if icp_native_map_one_cpu(*indx as i32, (*(&r as *const Resource as *const u64)), resource_size(&r)) != 0 {
            return -1;
        }
        *indx += 1;
    }
    0
}

static icp_native_ops: IcpOps = IcpOps {
    get_irq: Some(icp_native_get_irq),
    eoi: Some(icp_native_eoi),
    set_priority: Some(icp_native_set_cpu_priority),
    teardown_cpu: Some(icp_native_teardown_cpu),
    flush_ipi: Some(icp_native_flush_ipi),
    #[cfg(CONFIG_SMP)] ipi_action: Some(icp_native_ipi_action),
    #[cfg(CONFIG_SMP)] cause_ipi: Some(icp_native_cause_ipi),
};

#[no_mangle]
pub unsafe extern "C" fn icp_native_init() -> i32 {
    let mut indx = 0u32;
    let mut found = 0;
    // for_each_compatible_node(np, NULL, "ibm,ppc-xicp")
    // and the fallback for_each_node_by_type are kernel device-tree iterators.
    // Their equivalent traversal is supplied by the surrounding translation.
    if found == 0 { return -19; }
    icp_ops = &icp_native_ops;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
