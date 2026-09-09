// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

// C dependencies: linux/delay.h, linux/module.h, linux/of_address.h,
// linux/slab.h, and clk-mtk.h provide the referenced types and functions.

const REF2USB_TX_EN: u32 = 1u32 << 0;
const REF2USB_TX_LPF_EN: u32 = 1u32 << 1;
const REF2USB_TX_OUT_EN: u32 = 1u32 << 2;
const REF2USB_EN_MASK: u32 = REF2USB_TX_EN | REF2USB_TX_LPF_EN | REF2USB_TX_OUT_EN;

#[repr(C)]
struct mtk_ref2usb_tx {
    hw: clk_hw,
    base_addr: *mut core::ffi::c_void,
}

#[inline]
unsafe fn to_mtk_ref2usb_tx(hw: *mut clk_hw) -> *mut mtk_ref2usb_tx {
    // Equivalent to container_of(hw, struct mtk_ref2usb_tx, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(mtk_ref2usb_tx, hw)) as *mut mtk_ref2usb_tx
}

unsafe fn mtk_ref2usb_tx_is_prepared(hw: *mut clk_hw) -> i32 {
    let tx = to_mtk_ref2usb_tx(hw);

    if (readl((*tx).base_addr) & REF2USB_EN_MASK) == REF2USB_EN_MASK {
        1
    } else {
        0
    }
}

unsafe fn mtk_ref2usb_tx_prepare(hw: *mut clk_hw) -> i32 {
    let tx = to_mtk_ref2usb_tx(hw);
    let mut val: u32;

    val = readl((*tx).base_addr);

    val |= REF2USB_TX_EN;
    writel(val, (*tx).base_addr);
    udelay(100);

    val |= REF2USB_TX_LPF_EN;
    writel(val, (*tx).base_addr);

    val |= REF2USB_TX_OUT_EN;
    writel(val, (*tx).base_addr);

    0
}

unsafe fn mtk_ref2usb_tx_unprepare(hw: *mut clk_hw) {
    let tx = to_mtk_ref2usb_tx(hw);
    let mut val: u32;

    val = readl((*tx).base_addr);
    val &= !REF2USB_EN_MASK;
    writel(val, (*tx).base_addr);
}

static mtk_ref2usb_tx_ops: clk_ops = clk_ops {
    is_prepared: Some(mtk_ref2usb_tx_is_prepared),
    prepare: Some(mtk_ref2usb_tx_prepare),
    unprepare: Some(mtk_ref2usb_tx_unprepare),
};

unsafe fn mtk_clk_register_ref2usb_tx(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    reg: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let tx = kzalloc_obj::<mtk_ref2usb_tx>();
    if tx.is_null() {
        return err_ptr(-12); // -ENOMEM
    }

    (*tx).base_addr = reg;
    let mut init: clk_init_data = core::mem::zeroed();
    (*tx).hw.init = &mut init;

    init.name = name;
    init.ops = &mtk_ref2usb_tx_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;

    let ret = clk_hw_register(core::ptr::null_mut(), &mut (*tx).hw);

    if ret != 0 {
        kfree(tx as *mut core::ffi::c_void);
        return err_ptr(ret);
    }

    &mut (*tx).hw
}

unsafe fn mtk_clk_unregister_ref2usb_tx(hw: *mut clk_hw) {
    let tx = to_mtk_ref2usb_tx(hw);

    clk_hw_unregister(hw);
    kfree(tx as *mut core::ffi::c_void);
}

// EXPORT_SYMBOL_GPL(mtk_clk_register_ref2usb_tx);
// EXPORT_SYMBOL_GPL(mtk_clk_unregister_ref2usb_tx);
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
