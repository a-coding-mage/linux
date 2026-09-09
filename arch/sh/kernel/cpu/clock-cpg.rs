// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

const CLK_ENABLE_ON_INIT: u32 = 1 << 0;
const CONFIG_SH_PCLK_FREQ: u64 = 0;

#[repr(C)]
pub struct clk {
    pub parent: *mut clk,
    pub flags: u32,
    pub rate: u64,
    pub ops: *mut c_void,
}

#[repr(C)]
pub struct clk_lookup {
    pub con_id: *const c_char,
    pub clk: *mut clk,
}

extern "C" {
    fn arch_init_clk_ops(ops: *mut *mut c_void, index: c_int);
    fn clk_register(clk: *mut clk) -> c_int;
    fn clkdev_add_table(lookups: *mut clk_lookup, num: usize);
    fn clk_add_alias(
        alias: *const c_char,
        dev_id: *const c_char,
        con_id: *const c_char,
        clk: *mut clk,
    ) -> c_int;
}

static mut master_clk: clk = clk {
    parent: core::ptr::null_mut(),
    flags: CLK_ENABLE_ON_INIT,
    rate: CONFIG_SH_PCLK_FREQ,
    ops: core::ptr::null_mut(),
};

static mut peripheral_clk: clk = clk {
    parent: core::ptr::addr_of_mut!(master_clk),
    flags: CLK_ENABLE_ON_INIT,
    rate: 0,
    ops: core::ptr::null_mut(),
};

static mut bus_clk: clk = clk {
    parent: core::ptr::addr_of_mut!(master_clk),
    flags: CLK_ENABLE_ON_INIT,
    rate: 0,
    ops: core::ptr::null_mut(),
};

static mut cpu_clk: clk = clk {
    parent: core::ptr::addr_of_mut!(master_clk),
    flags: CLK_ENABLE_ON_INIT,
    rate: 0,
    ops: core::ptr::null_mut(),
};

/*
 * The ordering of these clocks matters, do not change it.
 */
static mut onchip_clocks: [*mut clk; 4] = [
    core::ptr::addr_of_mut!(master_clk),
    core::ptr::addr_of_mut!(peripheral_clk),
    core::ptr::addr_of_mut!(bus_clk),
    core::ptr::addr_of_mut!(cpu_clk),
];

static mut lookups: [clk_lookup; 4] = [
    /* main clocks */
    clk_lookup { con_id: b"master_clk\0".as_ptr() as *const c_char, clk: core::ptr::addr_of_mut!(master_clk) },
    clk_lookup { con_id: b"peripheral_clk\0".as_ptr() as *const c_char, clk: core::ptr::addr_of_mut!(peripheral_clk) },
    clk_lookup { con_id: b"bus_clk\0".as_ptr() as *const c_char, clk: core::ptr::addr_of_mut!(bus_clk) },
    clk_lookup { con_id: b"cpu_clk\0".as_ptr() as *const c_char, clk: core::ptr::addr_of_mut!(cpu_clk) },
];

pub unsafe extern "C" fn cpg_clk_init() -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;

    i = 0;
    while i < onchip_clocks.len() as c_int {
        let clk: *mut clk = onchip_clocks[i as usize];
        arch_init_clk_ops(&mut (*clk).ops, i);
        if !(*clk).ops.is_null() {
            ret |= clk_register(clk);
        }
        i += 1;
    }

    clkdev_add_table(lookups.as_mut_ptr(), lookups.len());

    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-tmu-sh3.0\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());
    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-tmu.0\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());
    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-tmu.1\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());
    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-tmu.2\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());
    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-mtu2\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());
    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-cmt-16.0\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());
    clk_add_alias(b"fck\0".as_ptr() as *const c_char, b"sh-cmt-32.0\0".as_ptr() as *const c_char, b"peripheral_clk\0".as_ptr() as *const c_char, core::ptr::null_mut());

    ret
}

/*
 * Placeholder for compatibility, until the lazy CPUs do this
 * on their own.
 */
pub unsafe extern "C" fn arch_clk_init() -> c_int {
    cpg_clk_init()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
