// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au1300 media block power gating (VSS)
 *
 * This is a stop-gap solution until I have the clock framework integration
 * ready. This stuff here really must be handled transparently when clocks
 * for various media blocks are enabled/disabled.
 */

const VSS_GATE: usize = 0x00; // gate wait timers
const VSS_CLKRST: usize = 0x04; // clock/block control
const VSS_FTR: usize = 0x08; // footers

extern "C" {
    fn alchemy_get_cputype() -> i32;
    fn __raw_writel(value: u32, addr: *mut core::ffi::c_void);
    fn wmb();
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
    fn KSEG1ADDR(addr: usize) -> usize;
    static AU1300_VSS_PHYS_ADDR: usize;
    static ALCHEMY_CPU_AU1300: i32;
}

#[repr(C)]
struct Spinlock {
    _opaque: [u8; 0],
}

static mut au1300_vss_lock: Spinlock = Spinlock { _opaque: [] };

#[inline]
unsafe fn vss_addr(block: i32) -> *mut u8 {
    (KSEG1ADDR(AU1300_VSS_PHYS_ADDR) + (block as usize).wrapping_mul(0x0c)) as *mut u8
}

/* enable a block as outlined in the databook */
#[inline]
unsafe fn __enable_block(block: i32) {
    let base = vss_addr(block);

    __raw_writel(3, base.add(VSS_CLKRST) as *mut core::ffi::c_void); // enable clock, assert reset
    wmb();

    __raw_writel(0x01fffffe, base.add(VSS_GATE) as *mut core::ffi::c_void); // maximum setup time
    wmb();

    /* enable footers in sequence */
    __raw_writel(0x01, base.add(VSS_FTR) as *mut core::ffi::c_void);
    wmb();
    __raw_writel(0x03, base.add(VSS_FTR) as *mut core::ffi::c_void);
    wmb();
    __raw_writel(0x07, base.add(VSS_FTR) as *mut core::ffi::c_void);
    wmb();
    __raw_writel(0x0f, base.add(VSS_FTR) as *mut core::ffi::c_void);
    wmb();

    __raw_writel(0x01ffffff, base.add(VSS_GATE) as *mut core::ffi::c_void); // start FSM too
    wmb();

    __raw_writel(2, base.add(VSS_CLKRST) as *mut core::ffi::c_void); // deassert reset
    wmb();

    __raw_writel(0x1f, base.add(VSS_FTR) as *mut core::ffi::c_void); // enable isolation cells
    wmb();
}

/* disable a block as outlined in the databook */
#[inline]
unsafe fn __disable_block(block: i32) {
    let base = vss_addr(block);

    __raw_writel(0x0f, base.add(VSS_FTR) as *mut core::ffi::c_void); // disable isolation cells
    wmb();
    __raw_writel(0, base.add(VSS_GATE) as *mut core::ffi::c_void); // disable FSM
    wmb();
    __raw_writel(3, base.add(VSS_CLKRST) as *mut core::ffi::c_void); // assert reset
    wmb();
    __raw_writel(1, base.add(VSS_CLKRST) as *mut core::ffi::c_void); // disable clock
    wmb();
    __raw_writel(0, base.add(VSS_FTR) as *mut core::ffi::c_void); // disable all footers
    wmb();
}

pub unsafe fn au1300_vss_block_control(block: i32, enable: i32) {
    let mut flags: usize = 0;

    if alchemy_get_cputype() != ALCHEMY_CPU_AU1300 {
        return;
    }

    /* only one block at a time */
    spin_lock_irqsave(
        &mut au1300_vss_lock as *mut Spinlock as *mut core::ffi::c_void,
        &mut flags,
    );
    if enable != 0 {
        __enable_block(block);
    } else {
        __disable_block(block);
    }
    spin_unlock_irqrestore(
        &mut au1300_vss_lock as *mut Spinlock as *mut core::ffi::c_void,
        flags,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
