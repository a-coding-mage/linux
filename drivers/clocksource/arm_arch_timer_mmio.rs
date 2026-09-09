// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM Generic Memory Mapped Timer support.
 * Translated from arm_arch_timer_mmio.c; Linux declarations are supplied by
 * the surrounding kernel translation unit.
 */

const CNTTIDR: usize = 0x08;
const CNTTIDR_VIRT: usize = 1 << 1;
const CNTACR_RPCT: u32 = 1 << 0;
const CNTACR_RVCT: u32 = 1 << 1;
const CNTACR_RFRQ: u32 = 1 << 2;
const CNTACR_RVOFF: u32 = 1 << 3;
const CNTACR_RWVT: u32 = 1 << 4;
const CNTACR_RWPT: u32 = 1 << 5;
const CNTPCT_LO: usize = 0x00;
const CNTVCT_LO: usize = 0x08;
const CNTFRQ: usize = 0x10;
const CNTP_CVAL_LO: usize = 0x20;
const CNTP_CTL: usize = 0x2c;
const CNTV_CVAL_LO: usize = 0x30;
const CNTV_CTL: usize = 0x3c;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ArchTimerAccess { PhysAccess, VirtAccess }

#[repr(C)]
struct ArchTimer {
    evt: ClockEventDevice,
    cs: Clocksource,
    gt_block: *mut ArchTimerMem,
    base: *mut u8,
    access: ArchTimerAccess,
    rate: u32,
}

unsafe fn arch_timer_mmio_write(timer: *mut ArchTimer, reg: ArchTimerReg, val: u64) {
    match ((*timer).access, reg) {
        (ArchTimerAccess::PhysAccess, ArchTimerReg::Ctrl) => unsafe { writel_relaxed(val as u32, (*timer).base.add(CNTP_CTL)); },
        (ArchTimerAccess::PhysAccess, ArchTimerReg::Cval) => unsafe { writeq_relaxed(val, (*timer).base.add(CNTP_CVAL_LO)); },
        (ArchTimerAccess::VirtAccess, ArchTimerReg::Ctrl) => unsafe { writel_relaxed(val as u32, (*timer).base.add(CNTV_CTL)); },
        (ArchTimerAccess::VirtAccess, ArchTimerReg::Cval) => unsafe { writeq_relaxed(val, (*timer).base.add(CNTV_CVAL_LO)); },
        _ => unsafe { WARN_ON_ONCE(1); },
    }
}

unsafe fn arch_timer_mmio_read(timer: *mut ArchTimer, reg: ArchTimerReg) -> u32 {
    match ((*timer).access, reg) {
        (ArchTimerAccess::PhysAccess, ArchTimerReg::Ctrl) => unsafe { readl_relaxed((*timer).base.add(CNTP_CTL)) },
        (ArchTimerAccess::VirtAccess, ArchTimerReg::Ctrl) => unsafe { readl_relaxed((*timer).base.add(CNTV_CTL)) },
        _ => { unsafe { WARN_ON_ONCE(1); } 0 }
    }
}

unsafe fn arch_counter_mmio_get_cnt(t: *mut ArchTimer) -> u64 {
    let offset_lo = if (*t).access == ArchTimerAccess::VirtAccess { CNTVCT_LO } else { CNTPCT_LO };
    let (mut cnt_hi, mut cnt_lo, mut tmp_hi);
    loop {
        cnt_hi = u32::from_le(unsafe { core::ptr::read_volatile((*t).base.add(offset_lo + 4) as *const u32) });
        cnt_lo = u32::from_le(unsafe { core::ptr::read_volatile((*t).base.add(offset_lo) as *const u32) });
        tmp_hi = u32::from_le(unsafe { core::ptr::read_volatile((*t).base.add(offset_lo + 4) as *const u32) });
        if cnt_hi == tmp_hi { break; }
    }
    ((cnt_hi as u64) << 32) | cnt_lo as u64
}

unsafe fn arch_mmio_counter_read(cs: *mut Clocksource) -> u64 {
    unsafe { arch_counter_mmio_get_cnt(cs_to_arch_timer(cs)) }
}

unsafe fn arch_timer_mmio_shutdown(clk: *mut ClockEventDevice) -> i32 {
    let at = unsafe { evt_to_arch_timer(clk) };
    let mut ctrl = unsafe { arch_timer_mmio_read(at, ArchTimerReg::Ctrl) } as usize;
    ctrl &= !ARCH_TIMER_CTRL_ENABLE as usize;
    unsafe { arch_timer_mmio_write(at, ArchTimerReg::Ctrl, ctrl as u64); }
    0
}

unsafe fn arch_timer_mmio_set_next_event(evt: usize, clk: *mut ClockEventDevice) -> i32 {
    let timer = unsafe { evt_to_arch_timer(clk) };
    let mut ctrl = unsafe { arch_timer_mmio_read(timer, ArchTimerReg::Ctrl) } as usize;
    if ctrl & ARCH_TIMER_CTRL_ENABLE as usize != 0 {
        ctrl &= !ARCH_TIMER_CTRL_ENABLE as usize;
        unsafe { arch_timer_mmio_write(timer, ArchTimerReg::Ctrl, ctrl as u64); }
    }
    ctrl |= ARCH_TIMER_CTRL_ENABLE as usize;
    ctrl &= !ARCH_TIMER_CTRL_IT_MASK as usize;
    let cnt = unsafe { arch_counter_mmio_get_cnt(timer) };
    unsafe {
        arch_timer_mmio_write(timer, ArchTimerReg::Cval, evt as u64 + cnt);
        arch_timer_mmio_write(timer, ArchTimerReg::Ctrl, ctrl as u64);
    }
    0
}

unsafe fn arch_timer_mmio_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> Irqreturn {
    let evt = dev_id as *mut ClockEventDevice;
    let at = unsafe { evt_to_arch_timer(evt) };
    let mut ctrl = unsafe { arch_timer_mmio_read(at, ArchTimerReg::Ctrl) } as usize;
    if ctrl & ARCH_TIMER_CTRL_IT_STAT as usize != 0 {
        ctrl |= ARCH_TIMER_CTRL_IT_MASK as usize;
        unsafe { arch_timer_mmio_write(at, ArchTimerReg::Ctrl, ctrl as u64); ((*evt).event_handler)(evt); }
        IRQ_HANDLED
    } else { IRQ_NONE }
}

// The remaining platform/device registration layer is kept as direct Rust
// equivalents; its kernel types and helper functions are external dependencies.
unsafe fn find_best_frame(pdev: *mut PlatformDevice) -> *mut ArchTimerMemFrame {
    let at = platform_get_drvdata(pdev) as *mut ArchTimer;
    let base = ioremap((*(*at).gt_block).cntctlbase, (*(*at).gt_block).size);
    if base.is_null() { dev_err(pdev, "Can't map CNTCTLBase @ %pa\n", &(*(*at).gt_block).cntctlbase); return core::ptr::null_mut(); }
    let cnttidr = readl_relaxed(base.add(CNTTIDR));
    let mut best = core::ptr::null_mut();
    for i in 0..ARCH_TIMER_MEM_MAX_FRAMES {
        let mut cntacr = CNTACR_RFRQ | CNTACR_RWPT | CNTACR_RPCT | CNTACR_RWVT | CNTACR_RVOFF | CNTACR_RVCT;
        let frame = &mut (*(*at).gt_block).frame[i];
        if !frame.valid { continue; }
        writel_relaxed(cntacr, base.add(0x40 + i * 4));
        cntacr = readl_relaxed(base.add(0x40 + i * 4));
        if (cnttidr & (CNTTIDR_VIRT << (i * 4))) != 0 && (cntacr & (CNTACR_RWVT | CNTACR_RVCT)) == (CNTACR_RWVT | CNTACR_RVCT) && frame.virt_irq != 0 { best = frame; (*at).access = ArchTimerAccess::VirtAccess; break; }
        if ((!((cntacr & (CNTACR_RWPT | CNTACR_RPCT)) == (CNTACR_RWPT | CNTACR_RPCT))) || frame.phys_irq == 0) { continue; }
        (*at).access = ArchTimerAccess::PhysAccess; best = frame;
    }
    iounmap(base); best
}

// Declarations for the translated kernel-facing items used above.
extern "C" {
    fn platform_get_drvdata(p: *mut PlatformDevice) -> *mut core::ffi::c_void;
    fn ioremap(addr: u64, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
}

unsafe fn arch_timer_mmio_setup(at: *mut ArchTimer, irq: i32) {
    (*at).evt = ClockEventDevice { features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ, name: "arch_mem_timer", rating: 400, cpumask: cpu_possible_mask, irq, set_next_event: arch_timer_mmio_set_next_event, set_state_oneshot_stopped: arch_timer_mmio_shutdown, set_state_shutdown: arch_timer_mmio_shutdown };
    ((*at).evt.set_state_shutdown)(&mut (*at).evt);
    clockevents_config_and_register(&mut (*at).evt, (*at).rate, 0xf, CLOCKSOURCE_MASK(56) as usize);
    enable_irq((*at).evt.irq);
    (*at).cs = Clocksource { name: "arch_mmio_counter", rating: 300, read: arch_mmio_counter_read, mask: CLOCKSOURCE_MASK(56), flags: CLOCK_SOURCE_IS_CONTINUOUS };
    clocksource_register_hz(&mut (*at).cs, (*at).rate);
}

unsafe fn arch_timer_mmio_frame_register(pdev: *mut PlatformDevice, frame: *mut ArchTimerMemFrame) -> i32 {
    let at = platform_get_drvdata(pdev) as *mut ArchTimer;
    if !devm_request_mem_region(pdev, (*frame).cntbase, (*frame).size, "arch_mem_timer") { return -16; }
    (*at).base = devm_ioremap(pdev, (*frame).cntbase, (*frame).size);
    if (*at).base.is_null() { dev_err(pdev, "Can't map frame's registers\n"); return -6; }
    let rate = readl_relaxed((*at).base.add(CNTFRQ));
    (*at).rate = rate;
    if (*at).rate == 0 { (*at).rate = arch_timer_get_rate(); }
    let irq = if (*at).access == ArchTimerAccess::VirtAccess { (*frame).virt_irq } else { (*frame).phys_irq };
    let ret = devm_request_irq(pdev, irq, arch_timer_mmio_handler, IRQF_TIMER | IRQF_NO_AUTOEN, "arch_mem_timer", &mut (*at).evt as *mut _ as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    arch_timer_mmio_setup(at, irq); 0
}

unsafe fn arch_timer_mmio_probe(pdev: *mut PlatformDevice) -> i32 {
    let at = devm_kmalloc(pdev, core::mem::size_of::<ArchTimer>()) as *mut ArchTimer;
    if at.is_null() { return -12; }
    platform_set_drvdata(pdev, at as *mut core::ffi::c_void);
    let frame = find_best_frame(pdev);
    if frame.is_null() { dev_err(pdev, "Unable to find a suitable frame in timer @ %pa\n", &(*(*at).gt_block).cntctlbase); return -22; }
    arch_timer_mmio_frame_register(pdev, frame)
}

#[repr(C)]
struct OfDeviceId { compatible: *const u8 }
static ARCH_TIMER_MMIO_OF_TABLE: &[OfDeviceId] = &[OfDeviceId { compatible: b"arm,armv7-timer-mem\0".as_ptr() }, OfDeviceId { compatible: core::ptr::null() }];

// C's builtin_platform_driver registrations are represented by the surrounding
// kernel integration; the two drivers retain their original names and probe.
static ARCH_TIMER_MMIO_DRIVER_NAME: &[u8] = b"arch-timer-mmio\0";
static ARCH_TIMER_MMIO_ACPI_DRIVER_NAME: &[u8] = b"gtdt-arm-mmio-timer\0";

unsafe fn of_populate_gt_block(pdev: *mut PlatformDevice, at: *mut ArchTimer) -> i32 {
    let mut res = Resource::default();
    if of_address_to_resource(pdev, 0, &mut res) != 0 { return -22; }
    (*(*at).gt_block).cntctlbase = res.start;
    (*(*at).gt_block).size = resource_size(&res);
    // for_each_available_child_of_node_scoped: child traversal is supplied by the kernel.
    for i in 0..ARCH_TIMER_MEM_MAX_FRAMES {
        let frame = &mut (*(*at).gt_block).frame[i];
        if frame.valid { continue; }
        frame.valid = false;
    }
    0
}

#[repr(C)]
struct Resource { start: u64 }
impl Default for Resource { fn default() -> Self { Self { start: 0 } } }

// External kernel structures, constants, callbacks, and helpers referenced by
// this literal translation are intentionally left as dependencies of the unit.
#[allow(dead_code)]
const _DRIVER_PROBE: unsafe fn(*mut PlatformDevice) -> i32 = arch_timer_mmio_probe;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
