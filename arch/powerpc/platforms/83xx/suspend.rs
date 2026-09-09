// SPDX-License-Identifier: GPL-2.0-only
/* MPC83xx suspend support; translated from suspend.c. */

// Kernel and architecture dependencies are supplied by the surrounding tree.

const PMCCR1_NEXT_STATE: u32 = 0x0C;
const PMCCR1_NEXT_STATE_SHIFT: u32 = 2;
const PMCCR1_CURR_STATE: u32 = 0x03;
const IMMR_SYSCR_OFFSET: usize = 0x100;
const IMMR_RCW_OFFSET: usize = 0x900;
const RCW_PCI_HOST: u32 = 0x80000000;

const PMCCR_DLPEN: u32 = 2;
const PMCCR_SLPEN: u32 = 1;
const PMCER_GPIO: u32 = 0x100;
const PMCER_PCI: u32 = 0x080;
const PMCER_USB: u32 = 0x040;
const PMCER_ETSEC1: u32 = 0x020;
const PMCER_ETSEC2: u32 = 0x010;
const PMCER_TIMER: u32 = 0x008;
const PMCER_INT1: u32 = 0x004;
const PMCER_INT2: u32 = 0x002;
const PMCER_PMCI: u32 = 0x001;
const PMCER_ALL: u32 = 0x1FF;
const PMCCR1_USE_STATE: u32 = 0x80000000;
const PMCCR1_PME_EN: u32 = 0x00000080;
const PMCCR1_ASSERT_PME: u32 = 0x00000040;
const PMCCR1_POWER_OFF: u32 = 0x00000020;

extern "C" {
    fn mpc83xx_enter_deep_sleep(immrbase: usize);
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn enable_kernel_fp();
    fn mpc6xx_enter_standby();
    fn set_freezable();
    fn signal_pending(task: *mut core::ffi::c_void) -> bool;
    fn current() -> *mut core::ffi::c_void;
    fn pm_suspend(state: i32) -> i32;
    fn kthread_run(func: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
                   data: *mut core::ffi::c_void, name: *const u8) -> *mut core::ffi::c_void;
    fn get_immrbase() -> usize;
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn of_device_is_available(np: *mut device_node) -> bool;
    fn of_device_get_match_data(dev: *mut device) -> *const pmc_type;
    fn of_address_to_resource(np: *mut device_node, index: u32, res: *mut resource) -> i32;
    fn irq_of_parse_and_map(np: *mut device_node, index: u32) -> i32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                   flags: u32, name: *const u8, dev: *mut platform_device) -> i32;
    fn free_irq(irq: i32, dev: *mut platform_device);
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn wake_up(wq: *mut core::ffi::c_void);
}

#[repr(C)]
struct mpc83xx_pmc { config: u32, event: u32, mask: u32, config1: u32, config2: u32 }
#[repr(C)] struct mpc83xx_rcw { rcwlr: u32, rcwhr: u32 }
#[repr(C)] struct mpc83xx_clock { spmr: u32, occr: u32, sccr: u32 }
#[repr(C)] struct mpc83xx_syscr { sgprl: u32, sgprh: u32, spridr: u32, _pad: u32, spcr: u32, sicrl: u32, sicrh: u32 }
#[repr(C)] struct mpc83xx_saved { sicrl: u32, sicrh: u32, sccr: u32 }
#[repr(C)] struct pmc_type { has_deep_sleep: i32 }

#[repr(C)] struct device_node;
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct resource { start: usize }
#[repr(C)] struct platform_suspend_ops {
    valid: Option<unsafe extern "C" fn(i32) -> bool>,
    begin: Option<unsafe extern "C" fn(i32) -> i32>,
    enter: Option<unsafe extern "C" fn(i32) -> i32>,
    end: Option<unsafe extern "C" fn()>,
}

static mut has_deep_sleep: i32 = 0;
static mut deep_sleeping: i32 = 0;
static mut pmc_irq: i32 = 0;
static mut pmc_regs: *mut mpc83xx_pmc = core::ptr::null_mut();
static mut clock_regs: *mut mpc83xx_clock = core::ptr::null_mut();
static mut syscr_regs: *mut mpc83xx_syscr = core::ptr::null_mut();
static mut saved_regs: mpc83xx_saved = mpc83xx_saved { sicrl: 0, sicrh: 0, sccr: 0 };
static mut is_pci_agent: i32 = 0;
static mut wake_from_pci: i32 = 0;
static mut immrbase: usize = 0;
static mut pci_pm_state: i32 = 0;
static mut agent_wq: *mut core::ffi::c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn fsl_deep_sleep() -> i32 { deep_sleeping }

unsafe fn mpc83xx_change_state() -> i32 {
    let mut reg_cfg1 = in_be32(&(*pmc_regs).config1);
    if is_pci_agent != 0 {
        pci_pm_state = (reg_cfg1 & PMCCR1_NEXT_STATE) >> PMCCR1_NEXT_STATE_SHIFT;
        let curr_state = reg_cfg1 & PMCCR1_CURR_STATE;
        if curr_state != pci_pm_state as u32 {
            reg_cfg1 = (reg_cfg1 & !PMCCR1_CURR_STATE) | pci_pm_state as u32;
            out_be32(&mut (*pmc_regs).config1, reg_cfg1);
            wake_up(agent_wq);
            return 1;
        }
    }
    0
}

unsafe extern "C" fn pmc_irq_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    let event = in_be32(&(*pmc_regs).event);
    let mut ret = 0;
    if mpc83xx_change_state() != 0 { ret = 1; }
    if event != 0 { out_be32(&mut (*pmc_regs).event, event); ret = 1; }
    ret
}

unsafe fn mpc83xx_suspend_restore_regs() {
    out_be32(&mut (*syscr_regs).sicrl, saved_regs.sicrl);
    out_be32(&mut (*syscr_regs).sicrh, saved_regs.sicrh);
    out_be32(&mut (*clock_regs).sccr, saved_regs.sccr);
}
unsafe fn mpc83xx_suspend_save_regs() {
    saved_regs.sicrl = in_be32(&(*syscr_regs).sicrl);
    saved_regs.sicrh = in_be32(&(*syscr_regs).sicrh);
    saved_regs.sccr = in_be32(&(*clock_regs).sccr);
}

unsafe extern "C" fn mpc83xx_suspend_enter(_state: i32) -> i32 {
    let mut ret = -11;
    if wake_from_pci != 0 {
        if pci_pm_state != if deep_sleeping != 0 { 3 } else { 2 } { return ret; }
        out_be32(&mut (*pmc_regs).config1, in_be32(&(*pmc_regs).config1) | PMCCR1_PME_EN);
    }
    out_be32(&mut (*pmc_regs).config, PMCCR_SLPEN | PMCCR_DLPEN);
    if deep_sleeping != 0 {
        mpc83xx_suspend_save_regs();
        out_be32(&mut (*pmc_regs).mask, PMCER_ALL);
        out_be32(&mut (*pmc_regs).config1, in_be32(&(*pmc_regs).config1) | PMCCR1_POWER_OFF);
        enable_kernel_fp();
        mpc83xx_enter_deep_sleep(immrbase);
        out_be32(&mut (*pmc_regs).config1, in_be32(&(*pmc_regs).config1) & !PMCCR1_POWER_OFF);
        out_be32(&mut (*pmc_regs).mask, PMCER_PMCI);
        mpc83xx_suspend_restore_regs();
    } else {
        out_be32(&mut (*pmc_regs).mask, PMCER_PMCI);
        mpc6xx_enter_standby();
    }
    ret = 0;
    out_be32(&mut (*pmc_regs).config1, in_be32(&(*pmc_regs).config1) & !PMCCR1_PME_EN);
    ret
}
unsafe extern "C" fn mpc83xx_suspend_end() { deep_sleeping = 0; }
unsafe extern "C" fn mpc83xx_suspend_valid(state: i32) -> bool { state == 1 || state == 3 }
unsafe extern "C" fn mpc83xx_suspend_begin(state: i32) -> i32 {
    match state { 1 => { deep_sleeping = 0; 0 }, 3 => { if has_deep_sleep != 0 { deep_sleeping = 1; } 0 }, _ => -22 }
}

unsafe extern "C" fn agent_thread_fn(_data: *mut core::ffi::c_void) -> i32 {
    set_freezable();
    loop {
        while pci_pm_state < 2 { /* wait_event_freezable(agent_wq, pci_pm_state >= 2) */ }
        if signal_pending(current()) || pci_pm_state < 2 { continue; }
        wake_from_pci = 1;
        pm_suspend(if pci_pm_state == 3 { 3 } else { 1 });
        wake_from_pci = 0;
    }
}
unsafe fn mpc83xx_set_agent() {
    out_be32(&mut (*pmc_regs).config1, PMCCR1_USE_STATE);
    out_be32(&mut (*pmc_regs).mask, PMCER_PMCI);
    kthread_run(agent_thread_fn, core::ptr::null_mut(), b"PCI power mgt\0".as_ptr());
}

unsafe fn mpc83xx_is_pci_agent() -> i32 {
    let rcw_regs = ioremap(get_immrbase() + IMMR_RCW_OFFSET, core::mem::size_of::<mpc83xx_rcw>()) as *mut mpc83xx_rcw;
    if rcw_regs.is_null() { return -12; }
    let ret = if in_be32(&(*rcw_regs).rcwhr) & RCW_PCI_HOST == 0 { 1 } else { 0 };
    iounmap(rcw_regs as *mut core::ffi::c_void); ret
}

static mpc83xx_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(mpc83xx_suspend_valid), begin: Some(mpc83xx_suspend_begin),
    enter: Some(mpc83xx_suspend_enter), end: Some(mpc83xx_suspend_end),
};
static pmc_types: [pmc_type; 2] = [
    pmc_type { has_deep_sleep: 1 }, pmc_type { has_deep_sleep: 0 },
];

#[repr(C)] struct of_device_id { compatible: *const u8, data: *const pmc_type }
static pmc_match: [of_device_id; 3] = [
    of_device_id { compatible: b"fsl,mpc8313-pmc\0".as_ptr(), data: &pmc_types[0] },
    of_device_id { compatible: b"fsl,mpc8349-pmc\0".as_ptr(), data: &pmc_types[1] },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn pmc_probe(ofdev: *mut platform_device) -> i32 {
    let np = (*ofdev).dev.of_node;
    let mut res = resource { start: 0 };
    let type_ = of_device_get_match_data(&mut (*ofdev).dev);
    if type_.is_null() { return -22; }
    if !of_device_is_available(np) { return -19; }
    has_deep_sleep = (*type_).has_deep_sleep;
    immrbase = get_immrbase();
    is_pci_agent = mpc83xx_is_pci_agent();
    if is_pci_agent < 0 { return is_pci_agent; }
    if of_address_to_resource(np, 0, &mut res) != 0 { return -19; }
    pmc_irq = irq_of_parse_and_map(np, 0);
    if pmc_irq != 0 && request_irq(pmc_irq, pmc_irq_handler, 0x80, b"pmc\0".as_ptr(), ofdev) != 0 { return -16; }
    pmc_regs = ioremap(res.start, core::mem::size_of::<mpc83xx_pmc>()) as *mut mpc83xx_pmc;
    if pmc_regs.is_null() { return -12; }
    if of_address_to_resource(np, 1, &mut res) != 0 { iounmap(pmc_regs as *mut _); return -19; }
    clock_regs = ioremap(res.start, core::mem::size_of::<mpc83xx_clock>()) as *mut mpc83xx_clock;
    if clock_regs.is_null() { iounmap(pmc_regs as *mut _); return -12; }
    if has_deep_sleep != 0 {
        syscr_regs = ioremap(immrbase + IMMR_SYSCR_OFFSET, core::mem::size_of::<mpc83xx_syscr>()) as *mut mpc83xx_syscr;
        if syscr_regs.is_null() { iounmap(clock_regs as *mut _); iounmap(pmc_regs as *mut _); return -12; }
    }
    if is_pci_agent != 0 { mpc83xx_set_agent(); }
    suspend_set_ops(&mpc83xx_suspend_ops); 0
}

#[repr(C)] struct driver { name: *const u8, of_match_table: *const of_device_id, suppress_bind_attrs: bool }
#[repr(C)] struct platform_driver { driver: driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32> }
static mut pmc_driver: platform_driver = platform_driver {
    driver: driver { name: b"mpc83xx-pmc\0".as_ptr(), of_match_table: pmc_match.as_ptr(), suppress_bind_attrs: true },
    probe: Some(pmc_probe),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
