// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013-2015 Emilio López
 *
 * Emilio López <emilio@elopez.com.ar>
 */

/* Translated from clk-usb.c; referenced kernel types and functions are external. */

#[repr(C)]
pub struct usb_reset_data {
    pub reg: *mut core::ffi::c_void,
    pub lock: *mut spinlock_t,
    pub clk: *mut clk,
    pub rcdev: reset_controller_dev,
}

unsafe fn sunxi_usb_reset_assert(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    let data = container_of!(rcdev, usb_reset_data, rcdev);
    let mut flags: c_ulong = 0;
    let reg: u32;

    clk_prepare_enable((*data).clk);
    spin_lock_irqsave((*data).lock, &mut flags);

    reg = readl((*data).reg);
    writel(reg & !BIT(id), (*data).reg);

    spin_unlock_irqrestore((*data).lock, flags);
    clk_disable_unprepare((*data).clk);

    0
}

unsafe fn sunxi_usb_reset_deassert(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    let data = container_of!(rcdev, usb_reset_data, rcdev);
    let mut flags: c_ulong = 0;
    let reg: u32;

    clk_prepare_enable((*data).clk);
    spin_lock_irqsave((*data).lock, &mut flags);

    reg = readl((*data).reg);
    writel(reg | BIT(id), (*data).reg);

    spin_unlock_irqrestore((*data).lock, flags);
    clk_disable_unprepare((*data).clk);

    0
}

static sunxi_usb_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(sunxi_usb_reset_assert),
    deassert: Some(sunxi_usb_reset_deassert),
};

const SUNXI_USB_MAX_SIZE: usize = 32;

#[repr(C)]
pub struct usb_clk_data {
    pub clk_mask: u32,
    pub reset_mask: u32,
    pub reset_needs_clk: bool,
}

unsafe fn sunxi_usb_clk_setup(node: *mut device_node, data: *const usb_clk_data,
                              lock: *mut spinlock_t) {
    let clk_data: *mut clk_onecell_data;
    let reset_data: *mut usb_reset_data;
    let clk_parent: *const c_char;
    let mut clk_name: *const c_char = core::ptr::null();
    let reg: *mut core::ffi::c_void;
    let qty: i32;
    let mut i: usize = 0;
    let mut j: usize = 0;

    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if IS_ERR(reg) { return; }

    clk_parent = of_clk_get_parent_name(node, 0);
    if clk_parent.is_null() { return; }

    qty = find_last_bit(&(*data).clk_mask as *const u32 as *const c_ulong,
                        SUNXI_USB_MAX_SIZE) as i32;

    clk_data = kmalloc_obj::<clk_onecell_data>();
    if clk_data.is_null() { return; }

    (*clk_data).clks = kzalloc_objs::<*mut clk>((qty + 1) as usize);
    if (*clk_data).clks.is_null() {
        kfree(clk_data as *mut core::ffi::c_void);
        return;
    }

    for_each_set_bit!(i, &(*data).clk_mask as *const u32 as *const c_ulong,
                      SUNXI_USB_MAX_SIZE) {
        of_property_read_string_index(node, "clock-output-names\0".as_ptr() as *const c_char,
                                      j, &mut clk_name);
        *(*clk_data).clks.add(i) = clk_register_gate(core::ptr::null_mut(), clk_name,
                                                     clk_parent, 0, reg, i as u8, 0, lock);
        WARN_ON(IS_ERR(*(*clk_data).clks.add(i)));
        j += 1;
    }

    (*clk_data).clk_num = i;
    of_clk_add_provider(node, of_clk_src_onecell_get, clk_data);

    if (*data).reset_mask == 0 { return; }
    reset_data = kzalloc_obj::<usb_reset_data>();
    if reset_data.is_null() { return; }

    if (*data).reset_needs_clk {
        (*reset_data).clk = of_clk_get(node, 0);
        if IS_ERR((*reset_data).clk) {
            pr_err!("Could not get clock for reset controls\n");
            kfree(reset_data as *mut core::ffi::c_void);
            return;
        }
    }

    (*reset_data).reg = reg;
    (*reset_data).lock = lock;
    (*reset_data).rcdev.nr_resets = (__fls((*data).reset_mask) + 1) as u32;
    (*reset_data).rcdev.ops = &sunxi_usb_reset_ops;
    (*reset_data).rcdev.of_node = node;
    reset_controller_register(&mut (*reset_data).rcdev);
}

const sun4i_a10_usb_clk_data: usb_clk_data = usb_clk_data { clk_mask: BIT(8)|BIT(7)|BIT(6), reset_mask: BIT(2)|BIT(1)|BIT(0), reset_needs_clk: false };
static mut sun4i_a10_usb_lock: spinlock_t = DEFINE_SPINLOCK!();
unsafe fn sun4i_a10_usb_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun4i_a10_usb_clk_data, &mut sun4i_a10_usb_lock); }
CLK_OF_DECLARE!(sun4i_a10_usb, "allwinner,sun4i-a10-usb-clk", sun4i_a10_usb_setup);

const sun5i_a13_usb_clk_data: usb_clk_data = usb_clk_data { clk_mask: BIT(8)|BIT(6), reset_mask: BIT(1)|BIT(0), reset_needs_clk: false };
unsafe fn sun5i_a13_usb_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun5i_a13_usb_clk_data, &mut sun4i_a10_usb_lock); }
CLK_OF_DECLARE!(sun5i_a13_usb, "allwinner,sun5i-a13-usb-clk", sun5i_a13_usb_setup);

const sun6i_a31_usb_clk_data: usb_clk_data = usb_clk_data { clk_mask: BIT(18)|BIT(17)|BIT(16)|BIT(10)|BIT(9)|BIT(8), reset_mask: BIT(2)|BIT(1)|BIT(0), reset_needs_clk: false };
unsafe fn sun6i_a31_usb_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun6i_a31_usb_clk_data, &mut sun4i_a10_usb_lock); }
CLK_OF_DECLARE!(sun6i_a31_usb, "allwinner,sun6i-a31-usb-clk", sun6i_a31_usb_setup);

const sun8i_a23_usb_clk_data: usb_clk_data = usb_clk_data { clk_mask: BIT(16)|BIT(11)|BIT(10)|BIT(9)|BIT(8), reset_mask: BIT(2)|BIT(1)|BIT(0), reset_needs_clk: false };
unsafe fn sun8i_a23_usb_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun8i_a23_usb_clk_data, &mut sun4i_a10_usb_lock); }
CLK_OF_DECLARE!(sun8i_a23_usb, "allwinner,sun8i-a23-usb-clk", sun8i_a23_usb_setup);

const sun8i_h3_usb_clk_data: usb_clk_data = usb_clk_data { clk_mask: BIT(19)|BIT(18)|BIT(17)|BIT(16)|BIT(11)|BIT(10)|BIT(9)|BIT(8), reset_mask: BIT(3)|BIT(2)|BIT(1)|BIT(0), reset_needs_clk: false };
unsafe fn sun8i_h3_usb_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun8i_h3_usb_clk_data, &mut sun4i_a10_usb_lock); }
CLK_OF_DECLARE!(sun8i_h3_usb, "allwinner,sun8i-h3-usb-clk", sun8i_h3_usb_setup);

const sun9i_a80_usb_mod_data: usb_clk_data = usb_clk_data { clk_mask: BIT(6)|BIT(5)|BIT(4)|BIT(3)|BIT(2)|BIT(1), reset_mask: BIT(19)|BIT(18)|BIT(17), reset_needs_clk: true };
static mut a80_usb_mod_lock: spinlock_t = DEFINE_SPINLOCK!();
unsafe fn sun9i_a80_usb_mod_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun9i_a80_usb_mod_data, &mut a80_usb_mod_lock); }
CLK_OF_DECLARE!(sun9i_a80_usb_mod, "allwinner,sun9i-a80-usb-mod-clk", sun9i_a80_usb_mod_setup);

const sun9i_a80_usb_phy_data: usb_clk_data = usb_clk_data { clk_mask: BIT(10)|BIT(5)|BIT(4)|BIT(3)|BIT(2)|BIT(1), reset_mask: BIT(21)|BIT(20)|BIT(19)|BIT(18)|BIT(17), reset_needs_clk: true };
static mut a80_usb_phy_lock: spinlock_t = DEFINE_SPINLOCK!();
unsafe fn sun9i_a80_usb_phy_setup(node: *mut device_node) { sunxi_usb_clk_setup(node, &sun9i_a80_usb_phy_data, &mut a80_usb_phy_lock); }
CLK_OF_DECLARE!(sun9i_a80_usb_phy, "allwinner,sun9i-a80-usb-phy-clk", sun9i_a80_usb_phy_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
