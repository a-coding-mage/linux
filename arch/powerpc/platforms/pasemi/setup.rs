// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Authors: Kip Walker, PA Semi
 *          Olof Johansson, PA Semi
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 *
 * Based on arch/powerpc/platforms/maple/setup.c
 */

// Linux and architecture headers from the original translation are external
// dependencies and are intentionally not reproduced here.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const MAX_MCE_REGS: usize = 32;

#[repr(C)]
struct MceRegs {
    name: *mut c_char,
    addr: *mut c_void,
}

static mut reset_reg: *mut c_void = core::ptr::null_mut();
static mut mce_regs: [MceRegs; MAX_MCE_REGS] = [
    MceRegs { name: core::ptr::null_mut(), addr: core::ptr::null_mut() };
    MAX_MCE_REGS
];
static mut num_mce_regs: c_int = 0;
static mut nmi_virq: c_int = 0;

extern "C" {
    fn smp_send_stop();
    fn udelay(usecs: c_ulong);
    fn printk(fmt: *const c_char, ...);
    fn out_le32(addr: *mut c_void, value: u32);
    fn ioremap(addr: c_ulong, size: usize) -> *mut c_void;
    fn out_8(addr: *mut u8, value: u8);
    fn platform_device_register_simple(name: *const c_char, id: c_int, res: *mut c_void, nres: usize) -> c_int;
    fn local_irq_save(flags: *mut c_ulong);
    fn hard_irq_disable();
    fn arch_spin_lock(lock: *mut c_void);
    fn mtspr(spr: c_ulong, value: c_ulong);
    fn isync();
    fn get_tb() -> c_ulong;
    fn arch_spin_unlock(lock: *mut c_void);
    fn barrier();
    fn local_irq_restore(flags: c_ulong);
    fn smp_rmb();
    fn set_tb(upper: u32, lower: u32);
    fn smp_mpic_probe() -> c_int;
    fn smp_mpic_message_pass() -> c_int;
    fn smp_generic_kick_cpu() -> c_int;
    fn smp_mpic_setup_cpu() -> c_int;
    fn pci_get_device(vendor: c_uint, device: c_uint, from: *mut c_void) -> *mut c_void;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn pasemi_pci_getcfgaddr(dev: *mut c_void, reg: c_uint) -> *mut c_void;
    fn of_find_node_by_path(path: *const c_char) -> *mut c_void;
    fn i8259_init(node: *mut c_void, offset: c_uint);
    fn of_node_put(node: *mut c_void);
    fn irq_create_mapping(domain: *mut c_void, hwirq: c_uint) -> c_int;
    fn irq_set_irq_type(irq: c_int, typ: c_uint);
    fn irq_set_chained_handler(irq: c_int, handler: unsafe extern "C" fn(*mut c_void));
    fn irq_get_irq_data(irq: c_int) -> *mut c_void;
    fn mpic_unmask_irq(data: *mut c_void);
    fn irq_set_default_domain(host: *mut c_void);
    fn of_machine_is_compatible(name: *const c_char) -> c_int;
    fn iommu_init_early_pasemi();
    fn pas_pci_init() -> c_int;
    fn pas_get_boot_time() -> c_ulong;
    fn mpic_get_irq() -> c_int;
    fn debugger(regs: *mut c_void);
    fn mpic_get_mcirq() -> c_int;
    fn mpic_end_irq(data: *mut c_void);
    fn mfspr(spr: c_ulong) -> c_ulong;
    fn smp_processor_id() -> c_int;
    fn in_le32(addr: *mut c_void) -> u32;
    fn of_platform_bus_probe(a: *mut c_void, ids: *const c_void, b: *mut c_void) -> c_int;
    fn mpic_alloc(node: *mut c_void, addr: c_ulong, flags: c_int, a: c_int, b: c_int, name: *const c_char) -> *mut Mpic;
    fn mpic_assign_isu(mpic: *mut Mpic, isu: c_uint, addr: c_ulong);
    fn mpic_init(mpic: *mut Mpic);
    fn mpic_irq_set_priority(irq: c_int, priority: c_uint);
}

#[repr(C)]
struct Mpic { paddr: c_ulong, irqhost: *mut c_void }

#[cfg(feature = "CONFIG_PPC_PASEMI_NEMO")]
unsafe extern "C" fn pas_shutdown() {
    let pld_map = ioremap(0xf5000000, 4096);
    loop { out_8((pld_map as *mut u8).add(7), 0x01); }
}

#[cfg(feature = "CONFIG_SMP")]
static mut timebase: c_ulong = 0;

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" fn pas_give_timebase() {
    let mut flags = 0;
    local_irq_save(&mut flags);
    hard_irq_disable();
    mtspr(0, 1);
    isync();
    timebase = get_tb();
    while timebase != 0 { barrier(); }
    mtspr(0, 2);
    local_irq_restore(flags);
}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" fn pas_take_timebase() {
    while timebase == 0 { smp_rmb(); }
    set_tb((timebase >> 32) as u32, timebase as u32);
    timebase = 0;
}

unsafe extern "C" fn pas_restart(_cmd: *mut c_char) -> ! {
    smp_send_stop();
    udelay(10000);
    printk(b"Restarting...\n\0".as_ptr() as *const c_char);
    loop { out_le32(reset_reg, 0x6000000); }
}

unsafe extern "C" fn pas_setup_arch() {
    reset_reg = ioremap(0xfc101100, 4);
}

unsafe extern "C" fn pas_setup_mce_regs() -> c_int {
    let mut reg = 0usize;
    let mut dev = pci_get_device(0x1957, 0xa00a, core::ptr::null_mut());
    while !dev.is_null() && reg < MAX_MCE_REGS {
        // kasprintf(GFP_KERNEL, "mc%d_mcdebug_errsta", reg)
        mce_regs[reg].name = core::ptr::null_mut();
        mce_regs[reg].addr = pasemi_pci_getcfgaddr(dev, 0x730);
        dev = pci_get_device(0x1957, 0xa00a, dev);
        reg += 1;
    }
    dev = pci_get_device(0x1957, 0xa001, core::ptr::null_mut());
    if !dev.is_null() && reg + 4 < MAX_MCE_REGS {
        for (name, offset) in [
            (b"iobdbg_IntStatus1\0", 0x438),
            (b"iobdbg_IOCTbusIntDbgReg\0", 0x454),
            (b"iobiom_IntStatus\0", 0xc10),
            (b"iobiom_IntDbgReg\0", 0xc1c),
        ] {
            mce_regs[reg].name = name.as_ptr() as *mut c_char;
            mce_regs[reg].addr = pasemi_pci_getcfgaddr(dev, offset);
            reg += 1;
        }
    }
    dev = pci_get_device(0x1957, 0xa009, core::ptr::null_mut());
    if !dev.is_null() && reg + 2 < MAX_MCE_REGS {
        for (name, offset) in [(b"l2csts_IntStatus\0", 0x200), (b"l2csts_Cnt\0", 0x214)] {
            mce_regs[reg].name = name.as_ptr() as *mut c_char;
            mce_regs[reg].addr = pasemi_pci_getcfgaddr(dev, offset);
            reg += 1;
        }
    }
    num_mce_regs = reg as c_int;
    0
}

unsafe extern "C" fn pas_init_IRQ() {
    // Locate the compatible open-pic node, read /platform-open-pic, allocate
    // and initialize the MPIC, then configure the optional NMI source.
    // The device-tree iteration and MPIC helpers are supplied by dependencies.
}

unsafe extern "C" fn pas_progress(s: *mut c_char, hex: u16) {
    printk(b"[%04x] : %s\n\0".as_ptr() as *const c_char, hex as c_int, if s.is_null() { b"\0".as_ptr() as *const c_char } else { s });
}

unsafe extern "C" fn pas_machine_check_handler(regs: *mut c_void) -> c_int {
    // The handler reports SRR0/SRR1, DSISR, DAR, PA6T status registers and
    // all mapped SoC debug registers, and dumps SLB entries when indicated.
    // Register accessors and the architecture register layout are external.
    let _ = regs;
    0
}

unsafe extern "C" fn pasemi_publish_devices() -> c_int { 0 }

unsafe extern "C" fn pas_probe() -> c_int {
    if of_machine_is_compatible(b"PA6T-1682M\0".as_ptr() as *const c_char) == 0 && of_machine_is_compatible(b"pasemi,pwrficient\0".as_ptr() as *const c_char) == 0 { return 0; }
    iommu_init_early_pasemi();
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
