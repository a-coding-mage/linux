// SPDX-License-Identifier: GPL-2.0
/*
 * Zynq UltraScale+ MPSoC clock controller
 *
 *  Copyright (C) 2016-2018 Xilinx
 *
 * Gated clock implementation
 */

// Dependencies supplied by the Linux clock framework and clk-zynqmp.h.

/**
 * struct zynqmp_clk_gate - gating clock
 * @hw:          handle between common and hardware-specific interfaces
 * @flags:       hardware-specific flags
 * @clk_id:      Id of clock
 */
#[repr(C)]
struct zynqmp_clk_gate {
    hw: clk_hw,
    flags: u8,
    clk_id: u32,
}

#[inline]
unsafe fn to_zynqmp_clk_gate(hw: *mut clk_hw) -> *mut zynqmp_clk_gate {
    (hw as *mut u8).sub(core::mem::offset_of!(zynqmp_clk_gate, hw)) as *mut zynqmp_clk_gate
}

/**
 * zynqmp_clk_gate_enable() - Enable clock
 * @hw:         handle between common and hardware-specific interfaces
 *
 * Return: 0 on success else error code
 */
unsafe extern "C" fn zynqmp_clk_gate_enable(hw: *mut clk_hw) -> i32 {
    let gate = &*to_zynqmp_clk_gate(hw);
    let clk_name: *const core::ffi::c_char = clk_hw_get_name(hw);
    let clk_id: u32 = gate.clk_id;
    let ret: i32 = zynqmp_pm_clock_enable(clk_id);

    if ret != 0 {
        pr_debug(
            "%s() clock enable failed for %s (id %d), ret = %d\n",
            "zynqmp_clk_gate_enable",
            clk_name,
            clk_id,
            ret,
        );
    }

    ret
}

/*
 * zynqmp_clk_gate_disable() - Disable clock
 * @hw:         handle between common and hardware-specific interfaces
 */
unsafe extern "C" fn zynqmp_clk_gate_disable(hw: *mut clk_hw) {
    let gate = &*to_zynqmp_clk_gate(hw);
    let clk_name: *const core::ffi::c_char = clk_hw_get_name(hw);
    let clk_id: u32 = gate.clk_id;
    let ret: i32 = zynqmp_pm_clock_disable(clk_id);

    if ret != 0 {
        pr_debug(
            "%s() clock disable failed for %s (id %d), ret = %d\n",
            "zynqmp_clk_gate_disable",
            clk_name,
            clk_id,
            ret,
        );
    }
}

/**
 * zynqmp_clk_gate_is_enabled() - Check clock state
 * @hw:         handle between common and hardware-specific interfaces
 *
 * Return: 1 if enabled, 0 if disabled else error code
 */
unsafe extern "C" fn zynqmp_clk_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = &*to_zynqmp_clk_gate(hw);
    let clk_name: *const core::ffi::c_char = clk_hw_get_name(hw);
    let clk_id: u32 = gate.clk_id;
    let mut state: i32 = 0;
    let ret: i32 = zynqmp_pm_clock_getstate(clk_id, &mut state);

    if ret != 0 {
        pr_debug(
            "%s() clock get state failed for %s, ret = %d\n",
            "zynqmp_clk_gate_is_enabled",
            clk_name,
            ret,
        );
        return -EIO;
    }

    if state != 0 { 1 } else { 0 }
}

static zynqmp_clk_gate_ops: clk_ops = clk_ops {
    enable: Some(zynqmp_clk_gate_enable),
    disable: Some(zynqmp_clk_gate_disable),
    is_enabled: Some(zynqmp_clk_gate_is_enabled),
};

/**
 * zynqmp_clk_register_gate() - Register a gate clock with the clock framework
 * @name:          Name of this clock
 * @clk_id:        Id of this clock
 * @parents:       Name of this clock's parents
 * @num_parents:   Number of parents
 * @nodes:         Clock topology node
 *
 * Return: clock hardware of the registered clock gate
 */
unsafe extern "C" fn zynqmp_clk_register_gate(
    name: *const core::ffi::c_char,
    clk_id: u32,
    parents: *const *const core::ffi::c_char,
    num_parents: u8,
    nodes: *const clock_topology,
) -> *mut clk_hw {
    let mut gate: *mut zynqmp_clk_gate = kzalloc_obj::<zynqmp_clk_gate>();
    if gate.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &zynqmp_clk_gate_ops;
    init.flags = zynqmp_clk_map_common_ccf_flags((*nodes).flag);
    init.parent_names = parents;
    init.num_parents = 1;

    (*gate).flags = (*nodes).type_flag;
    (*gate).hw.init = &init;
    (*gate).clk_id = clk_id;

    let mut hw: *mut clk_hw = &mut (*gate).hw;
    let ret: i32 = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(gate as *mut core::ffi::c_void);
        hw = ERR_PTR(ret);
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
