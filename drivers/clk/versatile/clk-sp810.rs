// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2013 ARM Limited
 */

// Dependencies are supplied by the surrounding kernel Rust bindings.

#[repr(C)]
pub struct clk_sp810 {
    pub node: *mut device_node,
    pub base: *mut core::ffi::c_void,
    pub lock: spinlock_t,
    pub timerclken: [clk_sp810_timerclken; 4],
}

#[repr(C)]
pub struct clk_sp810_timerclken {
    pub hw: clk_hw,
    pub clk: *mut clk,
    pub sp810: *mut clk_sp810,
    pub channel: core::ffi::c_int,
}

unsafe fn to_clk_sp810_timerclken(hw: *mut clk_hw) -> *mut clk_sp810_timerclken {
    // Equivalent to container_of(_hw, struct clk_sp810_timerclken, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(clk_sp810_timerclken, hw))
        as *mut clk_sp810_timerclken
}

unsafe fn clk_sp810_timerclken_get_parent(hw: *mut clk_hw) -> u8 {
    let timerclken = to_clk_sp810_timerclken(hw);
    let val = readl((*(*timerclken).sp810).base.add(SCCTRL as usize));

    ((val & (1u32 << SCCTRL_TIMERENnSEL_SHIFT((*timerclken).channel))) != 0) as u8
}

unsafe fn clk_sp810_timerclken_set_parent(hw: *mut clk_hw, index: u8) -> core::ffi::c_int {
    let timerclken = to_clk_sp810_timerclken(hw);
    let sp810 = (*timerclken).sp810;
    let shift = SCCTRL_TIMERENnSEL_SHIFT((*timerclken).channel);
    let mut flags: core::ffi::c_ulong = 0;

    if WARN_ON(index > 1) {
        return -EINVAL;
    }

    spin_lock_irqsave(&mut (*sp810).lock, &mut flags);

    let mut val = readl((*sp810).base.add(SCCTRL as usize));
    val &= !(1u32 << shift);
    val |= (index as u32) << shift;
    writel(val, (*sp810).base.add(SCCTRL as usize));

    spin_unlock_irqrestore(&mut (*sp810).lock, flags);

    0
}

#[repr(C)]
pub struct clk_ops {
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> core::ffi::c_int>,
    pub get_parent: Option<unsafe fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe fn(*mut clk_hw, u8) -> core::ffi::c_int>,
}

static CLK_SP810_TIMERCLKEN_OPS: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    get_parent: Some(clk_sp810_timerclken_get_parent),
    set_parent: Some(clk_sp810_timerclken_set_parent),
};

unsafe fn clk_sp810_timerclken_of_get(
    clkspec: *mut of_phandle_args,
    data: *mut core::ffi::c_void,
) -> *mut clk {
    let sp810 = data as *mut clk_sp810;

    if WARN_ON((*clkspec).args_count != 1 ||
        (*clkspec).args[0] >= core::mem::size_of_val(&(*sp810).timerclken) / core::mem::size_of::<clk_sp810_timerclken>()) {
        return core::ptr::null_mut();
    }

    (*sp810).timerclken[(*clkspec).args[0] as usize].clk
}

unsafe extern "C" fn clk_sp810_of_setup(node: *mut device_node) {
    let sp810 = kzalloc_obj::<clk_sp810>();
    let mut parent_names: [*const core::ffi::c_char; 2] = [core::ptr::null(); 2];
    let num = parent_names.len() as core::ffi::c_int;
    let mut name = [0i8; 12];
    let mut init: clk_init_data = core::mem::zeroed();
    static mut INSTANCE: core::ffi::c_int = 0;
    let mut deprecated: bool;

    if sp810.is_null() {
        return;
    }

    if of_clk_parent_fill(node, parent_names.as_mut_ptr(), num) != num {
        pr_warn!("Failed to obtain parent clocks for SP810!\n");
        kfree(sp810 as *mut core::ffi::c_void);
        return;
    }

    (*sp810).node = node;
    (*sp810).base = of_iomap(node, 0);
    spin_lock_init(&mut (*sp810).lock);

    init.name = name.as_ptr();
    init.ops = &CLK_SP810_TIMERCLKEN_OPS;
    init.flags = 0;
    init.parent_names = parent_names.as_ptr();
    init.num_parents = num as u8;

    deprecated = !of_property_present(node, c"assigned-clock-parents".as_ptr());

    for i in 0..(*sp810).timerclken.len() {
        snprintf!(name.as_mut_ptr(), name.len(), "sp810_%d_%d", INSTANCE, i);

        (*sp810).timerclken[i].sp810 = sp810;
        (*sp810).timerclken[i].channel = i as core::ffi::c_int;
        (*sp810).timerclken[i].hw.init = &init;

        /*
         * If DT isn't setting the parent, force it to be
         * the 1 MHz clock without going through the framework.
         * We do this before clk_register() so that it can determine
         * the parent and setup the tree properly.
         */
        if deprecated {
            (CLK_SP810_TIMERCLKEN_OPS.set_parent.unwrap())(&mut (*sp810).timerclken[i].hw, 1);
        }

        (*sp810).timerclken[i].clk = clk_register(core::ptr::null_mut(), &mut (*sp810).timerclken[i].hw);
        WARN_ON(IS_ERR((*sp810).timerclken[i].clk));
    }

    of_clk_add_provider(node, Some(clk_sp810_timerclken_of_get), sp810 as *mut core::ffi::c_void);
    INSTANCE += 1;
}

// CLK_OF_DECLARE(sp810, "arm,sp810", clk_sp810_of_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
