// SPDX-License-Identifier: GPL-2.0
/* SuperH Timer Support - TMU */

// Kernel headers and externally supplied symbols are intentionally referenced as dependencies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ShTmuModel { SH_TMU, SH_TMU_SH3 }

#[repr(C)] pub struct ShTmuDevice;

#[repr(C)]
pub struct ShTmuChannel {
    pub tmu: *mut ShTmuDevice,
    pub index: u32,
    pub base: *mut core::ffi::c_void,
    pub irq: i32,
    pub periodic: usize,
    pub ced: ClockEventDevice,
    pub cs: ClockSource,
    pub cs_enabled: bool,
    pub enable_count: u32,
}

#[repr(C)]
pub struct ShTmuDevice {
    pub pdev: *mut PlatformDevice,
    pub mapbase: *mut core::ffi::c_void,
    pub clk: *mut Clk,
    pub rate: usize,
    pub model: ShTmuModel,
    pub lock: RawSpinLock,
    pub channels: *mut ShTmuChannel,
    pub num_channels: u32,
    pub has_clockevent: bool,
    pub has_clocksource: bool,
}

pub const TSTR: i32 = -1;
pub const TCOR: i32 = 0;
pub const TCNT: i32 = 1;
pub const TCR: i32 = 2;
pub const TCR_UNF: u32 = 1 << 8;
pub const TCR_UNIE: u32 = 1 << 5;
pub const TCR_TPSC_CLK4: u32 = 0 << 0;
pub const TCR_TPSC_CLK16: u32 = 1 << 0;
pub const TCR_TPSC_CLK64: u32 = 2 << 0;
pub const TCR_TPSC_CLK256: u32 = 3 << 0;
pub const TCR_TPSC_CLK1024: u32 = 4 << 0;
pub const TCR_TPSC_MASK: u32 = 7 << 0;

unsafe fn sh_tmu_read(ch: *mut ShTmuChannel, reg_nr: i32) -> usize {
    let c = &*ch;
    if reg_nr == TSTR {
        return match (*c.tmu).model {
            ShTmuModel::SH_TMU_SH3 => ioread8((*c.tmu).mapbase.add(2)) as usize,
            ShTmuModel::SH_TMU => ioread8((*c.tmu).mapbase.add(4)) as usize,
        };
    }
    let offs = (reg_nr << 2) as usize;
    if reg_nr == TCR { ioread16(c.base.add(offs)) as usize } else { ioread32(c.base.add(offs)) as usize }
}

unsafe fn sh_tmu_write(ch: *mut ShTmuChannel, reg_nr: i32, value: usize) {
    let c = &*ch;
    if reg_nr == TSTR {
        match (*c.tmu).model {
            ShTmuModel::SH_TMU_SH3 => { iowrite8(value as u8, (*c.tmu).mapbase.add(2)); return; }
            ShTmuModel::SH_TMU => { iowrite8(value as u8, (*c.tmu).mapbase.add(4)); return; }
        }
    }
    let offs = (reg_nr << 2) as usize;
    if reg_nr == TCR { iowrite16(value as u16, c.base.add(offs)); } else { iowrite32(value as u32, c.base.add(offs)); }
}

unsafe fn sh_tmu_start_stop_ch(ch: *mut ShTmuChannel, start: i32) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*(*ch).tmu).lock, &mut flags);
    let mut value = sh_tmu_read(ch, TSTR);
    if start != 0 { value |= 1usize << (*ch).index; } else { value &= !(1usize << (*ch).index); }
    sh_tmu_write(ch, TSTR, value);
    raw_spin_unlock_irqrestore(&mut (*(*ch).tmu).lock, flags);
}

unsafe fn __sh_tmu_enable(ch: *mut ShTmuChannel) -> i32 {
    sh_tmu_start_stop_ch(ch, 0); sh_tmu_write(ch, TCOR, 0xffff_ffff); sh_tmu_write(ch, TCNT, 0xffff_ffff);
    sh_tmu_write(ch, TCR, TCR_TPSC_CLK4 as usize); sh_tmu_start_stop_ch(ch, 1); 0
}
unsafe fn sh_tmu_enable(ch: *mut ShTmuChannel) -> i32 {
    (*ch).enable_count += 1; if (*ch).enable_count > 1 { return 0; }
    dev_pm_syscore_device(&mut (*(*ch).tmu).pdev.as_mut().unwrap().dev, true); __sh_tmu_enable(ch)
}
unsafe fn __sh_tmu_disable(ch: *mut ShTmuChannel) { sh_tmu_start_stop_ch(ch, 0); sh_tmu_write(ch, TCR, TCR_TPSC_CLK4 as usize); }
unsafe fn sh_tmu_disable(ch: *mut ShTmuChannel) {
    if (*ch).enable_count == 0 { warn_on(true); return; } (*ch).enable_count -= 1;
    if (*ch).enable_count != 0 { return; } __sh_tmu_disable(ch);
    dev_pm_syscore_device(&mut (*(*ch).tmu).pdev.as_mut().unwrap().dev, false);
}

unsafe fn sh_tmu_set_next(ch: *mut ShTmuChannel, delta: usize, periodic: i32) {
    sh_tmu_start_stop_ch(ch, 0); sh_tmu_read(ch, TCR); sh_tmu_write(ch, TCR, (TCR_UNIE | TCR_TPSC_CLK4) as usize);
    if periodic != 0 { sh_tmu_write(ch, TCOR, delta); } else { sh_tmu_write(ch, TCOR, 0xffff_ffff); }
    sh_tmu_write(ch, TCNT, delta); sh_tmu_start_stop_ch(ch, 1);
}

// The remaining callback registration and platform-driver declarations retain the C interfaces.
// External kernel types/functions are supplied by the surrounding translation unit.
unsafe fn sh_tmu_clocksource_read(cs: *mut ClockSource) -> u64 { sh_tmu_read(cs_to_sh_tmu(cs), TCNT) as u64 ^ 0xffff_ffff }

extern "C" {
    type PlatformDevice; type Clk; type RawSpinLock; type ClockEventDevice; type ClockSource;
    fn ioread8(p: *mut core::ffi::c_void) -> u8; fn ioread16(p: *mut core::ffi::c_void) -> u16; fn ioread32(p: *mut core::ffi::c_void) -> u32;
    fn iowrite8(v: u8, p: *mut core::ffi::c_void); fn iowrite16(v: u16, p: *mut core::ffi::c_void); fn iowrite32(v: u32, p: *mut core::ffi::c_void);
    fn raw_spin_lock_irqsave(l: *mut RawSpinLock, f: *mut usize); fn raw_spin_unlock_irqrestore(l: *mut RawSpinLock, f: usize);
    fn dev_pm_syscore_device(d: *mut Device, e: bool); fn warn_on(v: bool);
    fn cs_to_sh_tmu(cs: *mut ClockSource) -> *mut ShTmuChannel;
}

#[repr(C)] pub struct Device { pub private: [u8; 0] }

unsafe fn sh_tmu_clocksource_enable(cs: *mut ClockSource) -> i32 { let ch = cs_to_sh_tmu(cs); if (*ch).cs_enabled { warn_on(true); return 0; } let r = sh_tmu_enable(ch); if r == 0 { (*ch).cs_enabled = true; } r }
unsafe fn sh_tmu_clocksource_disable(cs: *mut ClockSource) { let ch = cs_to_sh_tmu(cs); if (*ch).cs_enabled { sh_tmu_disable(ch); (*ch).cs_enabled = false; } }
unsafe fn sh_tmu_clocksource_suspend(cs: *mut ClockSource) { let ch = cs_to_sh_tmu(cs); if (*ch).cs_enabled { (*ch).enable_count -= 1; if (*ch).enable_count == 0 { __sh_tmu_disable(ch); } } }
unsafe fn sh_tmu_clocksource_resume(cs: *mut ClockSource) { let ch = cs_to_sh_tmu(cs); if (*ch).cs_enabled { if (*ch).enable_count == 0 { __sh_tmu_enable(ch); } (*ch).enable_count += 1; } }
unsafe fn sh_tmu_clock_event_start(ch: *mut ShTmuChannel, periodic: i32, rate: usize) { sh_tmu_enable(ch); if periodic != 0 { (*ch).periodic = (rate + HZ / 2) / HZ; sh_tmu_set_next(ch, (*ch).periodic, 1); } }
unsafe fn sh_tmu_clock_event_shutdown(ch: *mut ShTmuChannel) -> i32 { sh_tmu_disable(ch); 0 }
unsafe fn sh_tmu_clock_event_set_state(ch: *mut ShTmuChannel, periodic: i32, rate: usize) -> i32 { sh_tmu_disable(ch); sh_tmu_clock_event_start(ch, periodic, rate); 0 }
unsafe fn sh_tmu_clock_event_set_oneshot(ch: *mut ShTmuChannel, rate: usize) -> i32 { sh_tmu_clock_event_set_state(ch, 0, rate) }
unsafe fn sh_tmu_clock_event_set_periodic(ch: *mut ShTmuChannel, rate: usize) -> i32 { sh_tmu_clock_event_set_state(ch, 1, rate) }
unsafe fn sh_tmu_clock_event_next(ch: *mut ShTmuChannel, delta: usize) -> i32 { sh_tmu_set_next(ch, delta, 0); 0 }
pub const HZ: usize = 100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
