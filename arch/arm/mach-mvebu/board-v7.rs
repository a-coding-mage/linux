// SPDX-License-Identifier: GPL-2.0-only
/*
 * Device Tree support for Armada 370 and XP platforms.
 *
 * Copyright (C) 2012 Marvell
 *
 * Lior Amsalem <alior@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// Linux and local header dependencies are supplied by other translation units.

static mut scu_base: *mut core::ffi::c_void = core::ptr::null_mut();

/*
 * Enables the SCU when available. Obviously, this is only useful on
 * Cortex-A based SOCs, not on PJ4B based ones.
 */
unsafe fn mvebu_scu_enable() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
                                     c"arm,cortex-a9-scu".as_ptr());
    if !np.is_null() {
        scu_base = of_iomap(np, 0);
        scu_enable(scu_base);
        of_node_put(np);
    }
}

pub unsafe fn mvebu_get_scu_base() -> *mut core::ffi::c_void {
    scu_base
}

/*
 * When returning from suspend, the platform goes through the
 * bootloader, which executes its DDR3 training code. This code has the
 * unfortunate idea of using the first 10 KB of each DRAM bank to
 * exercise the RAM and calculate the optimal timings. Therefore, this
 * area of RAM is overwritten, and shouldn't be used by the kernel if
 * suspend/resume is supported.
 */

#[cfg(CONFIG_SUSPEND)]
const MVEBU_DDR_TRAINING_AREA_SZ: usize = 10 * SZ_1K;

#[cfg(CONFIG_SUSPEND)]
unsafe fn mvebu_scan_mem(node: usize, _uname: *const core::ffi::c_char,
                         _depth: i32, _data: *mut core::ffi::c_void) -> i32 {
    let type_ = of_get_flat_dt_prop(node, c"device_type".as_ptr(), core::ptr::null_mut());
    if type_.is_null() || strcmp(type_, c"memory".as_ptr()) != 0 { return 0; }

    let mut l: i32 = 0;
    let mut reg = of_get_flat_dt_prop(node, c"linux,usable-memory".as_ptr(), &mut l);
    if reg.is_null() { reg = of_get_flat_dt_prop(node, c"reg".as_ptr(), &mut l); }
    if reg.is_null() { return 0; }

    let endp = reg.add((l as usize) / core::mem::size_of::<u32>());
    while endp.offset_from(reg) >= (dt_root_addr_cells + dt_root_size_cells) as isize {
        let base = dt_mem_next_cell(dt_root_addr_cells, &mut reg);
        let size = dt_mem_next_cell(dt_root_size_cells, &mut reg);
        if size < MVEBU_DDR_TRAINING_AREA_SZ as u64 {
            pr_warn(c"Too little memory to reserve for DDR training\n".as_ptr());
        }
        memblock_reserve(base, MVEBU_DDR_TRAINING_AREA_SZ as u64);
    }
    0
}

#[cfg(CONFIG_SUSPEND)]
unsafe fn mvebu_memblock_reserve() { of_scan_flat_dt(mvebu_scan_mem, core::ptr::null_mut()); }

#[cfg(not(CONFIG_SUSPEND))]
unsafe fn mvebu_memblock_reserve() {}

unsafe fn mvebu_init_irq() {
    irqchip_init();
    mvebu_scu_enable();
    coherency_init();
    BUG_ON(mvebu_mbus_dt_init(coherency_available()));
}

unsafe fn i2c_quirk() {
    let mut dev: u32 = 0;
    let mut rev: u32 = 0;
    if mvebu_get_soc_id(&mut dev, &mut rev) == 0 && rev > MV78XX0_A0_REV { return; }

    let mut np = core::ptr::null_mut();
    while { np = for_each_compatible_node(np, core::ptr::null_mut(), c"marvell,mv78230-i2c".as_ptr()); !np.is_null() } {
        let new_compat = kzalloc_obj::<property>();
        (*new_compat).name = kstrdup(c"compatible".as_ptr(), GFP_KERNEL);
        (*new_compat).length = core::mem::size_of::<[u8; 28]>();
        (*new_compat).value = kstrdup(c"marvell,mv78230-a0-i2c".as_ptr(), GFP_KERNEL);
        of_update_property(np, new_compat);
    }
}

unsafe fn mvebu_dt_init() {
    if of_machine_is_compatible(c"marvell,armadaxp".as_ptr()) { i2c_quirk(); }
}

unsafe fn armada_370_xp_dt_fixup() {
    #[cfg(CONFIG_SMP)]
    smp_set_ops(smp_ops(armada_xp_smp_ops));
}

static armada_370_xp_dt_compat: [*const core::ffi::c_char; 2] = [
    c"marvell,armada-370-xp".as_ptr(), core::ptr::null(),
];

static armada_375_dt_compat: [*const core::ffi::c_char; 2] = [
    c"marvell,armada375".as_ptr(), core::ptr::null(),
];

static armada_38x_dt_compat: [*const core::ffi::c_char; 3] = [
    c"marvell,armada380".as_ptr(), c"marvell,armada385".as_ptr(), core::ptr::null(),
];

static armada_39x_dt_compat: [*const core::ffi::c_char; 3] = [
    c"marvell,armada390".as_ptr(), c"marvell,armada398".as_ptr(), core::ptr::null(),
];

// DT_MACHINE_START/ MACHINE_END are represented by the platform's Rust
// registration mechanism; preserve each original machine descriptor here.
static ARMADA_370_XP_DT: DtMachine = DtMachine {
    name: c"Marvell Armada 370/XP (Device Tree)".as_ptr(), l2c_aux_val: 0,
    l2c_aux_mask: !0, init_machine: Some(mvebu_dt_init), init_irq: Some(mvebu_init_irq),
    restart: Some(mvebu_restart), reserve: Some(mvebu_memblock_reserve),
    dt_compat: &armada_370_xp_dt_compat, dt_fixup: Some(armada_370_xp_dt_fixup),
};
static ARMADA_375_DT: DtMachine = DtMachine {
    name: c"Marvell Armada 375 (Device Tree)".as_ptr(), l2c_aux_val: 0,
    l2c_aux_mask: !0, init_irq: Some(mvebu_init_irq), init_machine: Some(mvebu_dt_init),
    restart: Some(mvebu_restart), dt_compat: &armada_375_dt_compat,
};
static ARMADA_38X_DT: DtMachine = DtMachine {
    name: c"Marvell Armada 380/385 (Device Tree)".as_ptr(), l2c_aux_val: 0,
    l2c_aux_mask: !0, init_irq: Some(mvebu_init_irq), restart: Some(mvebu_restart),
    dt_compat: &armada_38x_dt_compat,
};
static ARMADA_39X_DT: DtMachine = DtMachine {
    name: c"Marvell Armada 39x (Device Tree)".as_ptr(), l2c_aux_val: 0,
    l2c_aux_mask: !0, init_irq: Some(mvebu_init_irq), restart: Some(mvebu_restart),
    dt_compat: &armada_39x_dt_compat,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
