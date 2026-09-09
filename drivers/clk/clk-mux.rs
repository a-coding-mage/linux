// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2011 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
 * Copyright (C) 2011 Richard Zhao, Linaro <richard.zhao@linaro.org>
 * Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
 *
 * Simple multiplexer clock implementation
 */

// C dependencies: linux/clk-provider.h, linux/device.h, linux/module.h,
// linux/slab.h, linux/io.h, and linux/err.h.

/*
 * DOC: basic adjustable multiplexer clock that cannot gate
 *
 * Traits of this clock:
 * prepare - clk_prepare only ensures that parents are prepared
 * enable - clk_enable only ensures that parents are enabled
 * rate - rate is only affected by parent switching.  No clk_set_rate support
 * parent - parent is adjustable through clk_set_parent
 */

#[inline]
unsafe fn clk_mux_readl(mux: *mut clk_mux) -> u32 {
    if (*mux).flags & CLK_MUX_BIG_ENDIAN != 0 {
        return ioread32be((*mux).reg);
    }
    readl((*mux).reg)
}

#[inline]
unsafe fn clk_mux_writel(mux: *mut clk_mux, val: u32) {
    if (*mux).flags & CLK_MUX_BIG_ENDIAN != 0 {
        iowrite32be(val, (*mux).reg);
    } else {
        writel(val, (*mux).reg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn clk_mux_val_to_index(
    hw: *mut clk_hw,
    table: *const u32,
    flags: u32,
    mut val: u32,
) -> i32 {
    let num_parents = clk_hw_get_num_parents(hw);

    if !table.is_null() {
        let mut i = 0;
        while i < num_parents {
            if *table.add(i as usize) == val {
                return i as i32;
            }
            i += 1;
        }
        return -EINVAL;
    }

    if val != 0 && flags & CLK_MUX_INDEX_BIT != 0 {
        val = ffs(val) - 1;
    }
    if val != 0 && flags & CLK_MUX_INDEX_ONE != 0 {
        val -= 1;
    }
    if val >= num_parents {
        return -EINVAL;
    }
    val as i32
}

#[no_mangle]
pub unsafe extern "C" fn clk_mux_index_to_val(
    table: *const u32,
    flags: u32,
    index: u8,
) -> u32 {
    let mut val = index as u32;
    if !table.is_null() {
        val = *table.add(index as usize);
    } else {
        if flags & CLK_MUX_INDEX_BIT != 0 {
            val = 1u32 << index;
        }
        if flags & CLK_MUX_INDEX_ONE != 0 {
            val += 1;
        }
    }
    val
}

unsafe fn clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_clk_mux(hw);
    let mut val = clk_mux_readl(mux) >> (*mux).shift;
    val &= (*mux).mask;
    clk_mux_val_to_index(hw, (*mux).table, (*mux).flags, val) as u8
}

unsafe fn clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux = to_clk_mux(hw);
    let mut val = clk_mux_index_to_val((*mux).table, (*mux).flags, index);
    let mut flags: c_ulong = 0;
    let mut reg: u32;

    if !(*mux).lock.is_null() {
        spin_lock_irqsave((*mux).lock, &mut flags);
    } else {
        __acquire((*mux).lock);
    }

    if (*mux).flags & CLK_MUX_HIWORD_MASK != 0 {
        reg = (*mux).mask << ((*mux).shift + 16);
    } else {
        reg = clk_mux_readl(mux);
        reg &= !((*mux).mask << (*mux).shift);
    }
    val <<= (*mux).shift;
    reg |= val;
    clk_mux_writel(mux, reg);

    if !(*mux).lock.is_null() {
        spin_unlock_irqrestore((*mux).lock, flags);
    } else {
        __release((*mux).lock);
    }
    0
}

unsafe fn clk_mux_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mux = to_clk_mux(hw);
    clk_mux_determine_rate_flags(hw, req, (*mux).flags)
}

pub static clk_mux_ops: clk_ops = clk_ops {
    get_parent: Some(clk_mux_get_parent),
    set_parent: Some(clk_mux_set_parent),
    determine_rate: Some(clk_mux_determine_rate),
};

pub static clk_mux_ro_ops: clk_ops = clk_ops {
    get_parent: Some(clk_mux_get_parent),
    set_parent: None,
    determine_rate: None,
};

pub unsafe extern "C" fn __clk_hw_register_mux(
    dev: *mut device, np: *mut device_node, name: *const c_char,
    num_parents: u8, parent_names: *const *const c_char,
    parent_hws: *const *const clk_hw, parent_data: *const clk_parent_data,
    flags: c_ulong, reg: *mut c_void, shift: u8, mask: u32,
    clk_mux_flags: u8, table: *const u32, lock: *mut spinlock_t,
) -> *mut clk_hw {
    let mut mux: *mut clk_mux;
    let mut hw: *mut clk_hw;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut ret: i32 = -EINVAL;

    if clk_mux_flags & CLK_MUX_HIWORD_MASK != 0 {
        let width = fls(mask) - ffs(mask) + 1;
        if width + shift as i32 > 16 {
            pr_err!("mux value exceeds LOWORD field\n");
            return ERR_PTR(-EINVAL);
        }
    }
    mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() { return ERR_PTR(-ENOMEM); }

    init.name = name;
    init.ops = if clk_mux_flags & CLK_MUX_READ_ONLY != 0 {
        &clk_mux_ro_ops
    } else { &clk_mux_ops };
    init.flags = flags;
    init.parent_names = parent_names;
    init.parent_data = parent_data;
    init.parent_hws = parent_hws;
    init.num_parents = num_parents;

    (*mux).reg = reg;
    (*mux).shift = shift;
    (*mux).mask = mask;
    (*mux).flags = clk_mux_flags;
    (*mux).lock = lock;
    (*mux).table = table;
    (*mux).hw.init = &mut init;

    hw = &mut (*mux).hw;
    if !dev.is_null() || np.is_null() { ret = clk_hw_register(dev, hw); }
    else { ret = of_clk_hw_register(np, hw); }
    if ret != 0 { kfree(mux as *mut c_void); hw = ERR_PTR(ret); }
    hw
}

unsafe fn devm_clk_hw_release_mux(_dev: *mut device, res: *mut c_void) {
    clk_hw_unregister_mux(*(res as *mut *mut clk_hw));
}

pub unsafe extern "C" fn __devm_clk_hw_register_mux(
    dev: *mut device, np: *mut device_node, name: *const c_char,
    num_parents: u8, parent_names: *const *const c_char,
    parent_hws: *const *const clk_hw, parent_data: *const clk_parent_data,
    flags: c_ulong, reg: *mut c_void, shift: u8, mask: u32,
    clk_mux_flags: u8, table: *const u32, lock: *mut spinlock_t,
) -> *mut clk_hw {
    let ptr = devres_alloc(Some(devm_clk_hw_release_mux), core::mem::size_of::<*mut clk_hw>(), GFP_KERNEL);
    if ptr.is_null() { return ERR_PTR(-ENOMEM); }
    let hw = __clk_hw_register_mux(dev, np, name, num_parents, parent_names, parent_hws,
        parent_data, flags, reg, shift, mask, clk_mux_flags, table, lock);
    if !IS_ERR(hw) { *(ptr as *mut *mut clk_hw) = hw; devres_add(dev, ptr); }
    else { devres_free(ptr); }
    hw
}

pub unsafe extern "C" fn clk_register_mux_table(
    dev: *mut device, name: *const c_char, parent_names: *const *const c_char,
    num_parents: u8, flags: c_ulong, reg: *mut c_void, shift: u8, mask: u32,
    clk_mux_flags: u8, table: *const u32, lock: *mut spinlock_t,
) -> *mut clk {
    let hw = clk_hw_register_mux_table(dev, name, parent_names, num_parents, flags,
        reg, shift, mask, clk_mux_flags, table, lock);
    if IS_ERR(hw) { return ERR_CAST(hw); }
    (*hw).clk
}

pub unsafe extern "C" fn clk_unregister_mux(clk: *mut clk) {
    let hw = __clk_get_hw(clk);
    if hw.is_null() { return; }
    let mux = to_clk_mux(hw);
    clk_unregister(clk);
    kfree(mux as *mut c_void);
}

pub unsafe extern "C" fn clk_hw_unregister_mux(hw: *mut clk_hw) {
    let mux = to_clk_mux(hw);
    clk_hw_unregister(hw);
    kfree(mux as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
