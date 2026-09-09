// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2013 Emilio López
 * Emilio López <emilio@elopez.com.ar>
 *
 * Copyright 2013 Chen-Yu Tsai
 * Chen-Yu Tsai <wens@csie.org>
 */

// Dependencies supplied by the Linux clock, device-tree, I/O, and allocator
// interfaces are intentionally left as external symbols.

static mut GMAC_LOCK: DEFINE_SPINLOCK_TYPE = DEFINE_SPINLOCK!(/* gmac_lock */);

const SUN7I_A20_GMAC_GPIT: u32 = 2;
const SUN7I_A20_GMAC_MASK: u32 = 0x3;
const SUN7I_A20_GMAC_PARENTS: usize = 2;

static mut SUN7I_A20_GMAC_MUX_TABLE: [u32; SUN7I_A20_GMAC_PARENTS] = [
    0x00, // Select mii_phy_tx_clk
    0x02, // Select gmac_int_tx_clk
];

/**
 * sun7i_a20_gmac_clk_setup - Setup function for A20/A31 GMAC clock module
 * @node: &struct device_node for the clock
 *
 * This clock looks something like this
 *                               ________________________
 *  MII TX clock from PHY >-----|___________    _________|----> to GMAC core
 *  GMAC Int. RGMII TX clk >----|___________\__/__gate---|----> to PHY
 *  Ext. 125MHz RGMII TX clk >--|__divider__/            |
 *                              |________________________|
 *
 * The external 125 MHz reference is optional, i.e. GMAC can use its
 * internal TX clock just fine. The A31 GMAC clock module does not have
 * the divider controls for the external reference.
 *
 * To keep it simple, let the GMAC use either the MII TX clock for MII mode,
 * and its internal TX clock for GMII and RGMII modes. The GMAC driver should
 * select the appropriate source and gate/ungate the output to the PHY.
 *
 * Only the GMAC should use this clock. Altering the clock so that it doesn't
 * match the GMAC's operation parameters will result in the GMAC not being
 * able to send traffic out. The GMAC driver should set the clock rate and
 * enable/disable this clock to configure the required state. The clock
 * driver then responds by auto-reparenting the clock.
 */
unsafe fn sun7i_a20_gmac_clk_setup(node: *mut device_node) {
    let mut clk: *mut clk;
    let mux: *mut clk_mux;
    let gate: *mut clk_gate;
    let mut clk_name: *const core::ffi::c_char = (*node).name;
    let mut parents: [*const core::ffi::c_char; SUN7I_A20_GMAC_PARENTS] =
        [core::ptr::null(); SUN7I_A20_GMAC_PARENTS];
    let mut reg: *mut core::ffi::c_void;

    if of_property_read_string(
        node,
        c"clock-output-names".as_ptr(),
        &mut clk_name,
    ) != 0 {
        return;
    }

    /* allocate mux and gate clock structs */
    mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() {
        return;
    }

    gate = kzalloc_obj::<clk_gate>();
    if gate.is_null() {
        goto_free_mux(mux);
        return;
    }

    /* gmac clock requires exactly 2 parents */
    if of_clk_parent_fill(node, parents.as_mut_ptr(), 2) != 2 {
        goto_free_gate(gate, mux);
        return;
    }

    reg = of_iomap(node, 0);
    if reg.is_null() {
        goto_free_gate(gate, mux);
        return;
    }

    /* set up gate and fixed rate properties */
    (*gate).reg = reg;
    (*gate).bit_idx = SUN7I_A20_GMAC_GPIT;
    (*gate).lock = &raw mut GMAC_LOCK;
    (*mux).reg = reg;
    (*mux).mask = SUN7I_A20_GMAC_MASK;
    (*mux).table = raw mut SUN7I_A20_GMAC_MUX_TABLE;
    (*mux).lock = &raw mut GMAC_LOCK;

    clk = clk_register_composite(
        core::ptr::null_mut(),
        clk_name,
        parents.as_ptr(),
        SUN7I_A20_GMAC_PARENTS,
        &mut (*mux).hw,
        &clk_mux_ops,
        core::ptr::null_mut(),
        core::ptr::null(),
        &mut (*gate).hw,
        &clk_gate_ops,
        0,
    );

    if is_err(clk) {
        iounmap(reg);
        goto_free_gate(gate, mux);
        return;
    }

    of_clk_add_provider(node, of_clk_src_simple_get, clk);
}

unsafe fn goto_free_mux(mux: *mut clk_mux) {
    kfree(mux);
}

unsafe fn goto_free_gate(gate: *mut clk_gate, mux: *mut clk_mux) {
    kfree(gate);
    goto_free_mux(mux);
}

// CLK_OF_DECLARE(sun7i_a20_gmac, "allwinner,sun7i-a20-gmac-clk",
//                sun7i_a20_gmac_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
