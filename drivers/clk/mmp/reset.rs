// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux kernel and by reset.h are intentionally
// left as external Rust definitions.

#[repr(C)]
pub struct ResetControllerDev {
    pub of_reset_n_cells: u32,
    pub nr_resets: u32,
    pub ops: *const ResetControlOps,
    pub of_node: *mut DeviceNode,
    pub of_xlate: Option<unsafe extern "C" fn(
        *mut ResetControllerDev,
        *const OfPhandleArgs,
    ) -> i32>,
}

#[repr(C)]
pub struct ResetControlOps {
    pub assert: Option<unsafe extern "C" fn(*mut ResetControllerDev, usize) -> i32>,
    pub deassert: Option<unsafe extern "C" fn(*mut ResetControllerDev, usize) -> i32>,
}

#[repr(C)]
pub struct OfPhandleArgs {
    pub args_count: u32,
    pub args: *const u32,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Spinlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MmpClkResetCell {
    pub clk_id: u32,
    pub lock: *mut Spinlock,
    pub reg: *mut u32,
    pub bits: u32,
}

#[repr(C)]
pub struct MmpClkResetUnit {
    pub rcdev: ResetControllerDev,
    pub cells: *mut MmpClkResetCell,
}

unsafe extern "C" {
    fn readl(addr: *mut u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn reset_controller_register(rcdev: *mut ResetControllerDev) -> i32;
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn warn_on(condition: bool) -> bool;
}

unsafe fn rcdev_to_unit(rcdev: *mut ResetControllerDev) -> *mut MmpClkResetUnit {
    rcdev as *mut MmpClkResetUnit
}

unsafe extern "C" fn mmp_of_reset_xlate(
    rcdev: *mut ResetControllerDev,
    reset_spec: *const OfPhandleArgs,
) -> i32 {
    let unit = rcdev_to_unit(rcdev);
    let mut cell: *mut MmpClkResetCell;
    let mut i: u32;

    if warn_on((*reset_spec).args_count != (*rcdev).of_reset_n_cells) {
        return -22;
    }

    i = 0;
    while i < (*rcdev).nr_resets {
        cell = (*unit).cells.add(i as usize);
        if (*cell).clk_id == *(*reset_spec).args {
            break;
        }
        i += 1;
    }

    if i == (*rcdev).nr_resets {
        return -22;
    }

    i as i32
}

unsafe extern "C" fn mmp_clk_reset_assert(
    rcdev: *mut ResetControllerDev,
    id: usize,
) -> i32 {
    let unit = rcdev_to_unit(rcdev);
    let cell = (*unit).cells.add(id);
    let mut flags: usize = 0;
    let mut val: u32;

    if !(*cell).lock.is_null() {
        spin_lock_irqsave((*cell).lock, &mut flags);
    }

    val = readl((*cell).reg);
    val |= (*cell).bits;
    writel(val, (*cell).reg);

    if !(*cell).lock.is_null() {
        spin_unlock_irqrestore((*cell).lock, flags);
    }

    0
}

unsafe extern "C" fn mmp_clk_reset_deassert(
    rcdev: *mut ResetControllerDev,
    id: usize,
) -> i32 {
    let unit = rcdev_to_unit(rcdev);
    let cell = (*unit).cells.add(id);
    let mut flags: usize = 0;
    let mut val: u32;

    if !(*cell).lock.is_null() {
        spin_lock_irqsave((*cell).lock, &mut flags);
    }

    val = readl((*cell).reg);
    val &= !(*cell).bits;
    writel(val, (*cell).reg);

    if !(*cell).lock.is_null() {
        spin_unlock_irqrestore((*cell).lock, flags);
    }

    0
}

static MMP_CLK_RESET_OPS: ResetControlOps = ResetControlOps {
    assert: Some(mmp_clk_reset_assert),
    deassert: Some(mmp_clk_reset_deassert),
};

pub unsafe extern "C" fn mmp_clk_reset_register(
    np: *mut DeviceNode,
    cells: *mut MmpClkResetCell,
    nr_resets: i32,
) {
    let unit = kzalloc(core::mem::size_of::<MmpClkResetUnit>()) as *mut MmpClkResetUnit;
    if unit.is_null() {
        return;
    }

    core::ptr::write_bytes(unit, 0, 1);
    (*unit).cells = cells;
    (*unit).rcdev.of_reset_n_cells = 1;
    (*unit).rcdev.nr_resets = nr_resets as u32;
    (*unit).rcdev.ops = &MMP_CLK_RESET_OPS;
    (*unit).rcdev.of_node = np;
    (*unit).rcdev.of_xlate = Some(mmp_of_reset_xlate);

    reset_controller_register(&mut (*unit).rcdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
