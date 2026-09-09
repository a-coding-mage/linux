// SPDX-License-Identifier: GPL-2.0

const CCM_CCDR: usize = 0x4;
const CCDR_MMDC_CH0_MASK: u32 = 1u32 << 17;
const CCDR_MMDC_CH1_MASK: u32 = 1u32 << 16;

// DEFINE_SPINLOCK(imx_ccm_lock);
pub static mut imx_ccm_lock: spinlock_t = spinlock_t::new();

pub static mut mcore_booted: bool = false;

pub unsafe fn imx_unregister_hw_clocks(hws: *mut *mut clk_hw, count: u32) {
    let mut i: u32 = 0;
    while i < count {
        clk_hw_unregister(*hws.add(i as usize));
        i += 1;
    }
}

pub unsafe fn imx_mmdc_mask_handshake(ccm_base: *mut core::ffi::c_void, chn: u32) {
    let reg = readl_relaxed((ccm_base as *mut u8).add(CCM_CCDR) as *const u32);
    let reg = reg | if chn == 0 { CCDR_MMDC_CH0_MASK } else { CCDR_MMDC_CH1_MASK };
    writel_relaxed(reg, (ccm_base as *mut u8).add(CCM_CCDR) as *mut u32);
}

pub unsafe fn imx_check_clocks(clks: *mut *mut clk, count: u32) {
    let mut i: u32 = 0;
    while i < count {
        if is_err(*clks.add(i as usize)) {
            pr_err!("i.MX clk {}: register failed with {}\n", i, ptr_err(*clks.add(i as usize)));
        }
        i += 1;
    }
}

pub unsafe fn imx_check_clk_hws(clks: *mut *mut clk_hw, count: u32) {
    let mut i: u32 = 0;
    while i < count {
        if is_err(*clks.add(i as usize)) {
            pr_err!("i.MX clk {}: register failed with {}\n", i, ptr_err(*clks.add(i as usize)));
        }
        i += 1;
    }
}

unsafe fn imx_obtain_fixed_clock_from_dt(name: *const core::ffi::c_char) -> *mut clk {
    let mut phandle: of_phandle_args = core::mem::zeroed();
    let mut clk: *mut clk = err_ptr(-ENODEV);
    let path = kasprintf(GFP_KERNEL, c"/clocks/%s".as_ptr(), name);
    if path.is_null() {
        return err_ptr(-ENOMEM);
    }

    phandle.np = of_find_node_by_path(path);
    kfree(path);

    if !phandle.np.is_null() {
        clk = of_clk_get_from_provider(&mut phandle);
        of_node_put(phandle.np);
    }
    clk
}

pub unsafe fn imx_obtain_fixed_clock(name: *const core::ffi::c_char, rate: u64) -> *mut clk {
    let mut clk = imx_obtain_fixed_clock_from_dt(name);
    if is_err(clk) {
        clk = imx_clk_fixed(name, rate);
    }
    clk
}

pub unsafe fn imx_obtain_fixed_clock_hw(name: *const core::ffi::c_char, rate: u64) -> *mut clk_hw {
    let mut clk = imx_obtain_fixed_clock_from_dt(name);
    if is_err(clk) {
        clk = imx_clk_fixed(name, rate);
    }
    __clk_get_hw(clk)
}

pub unsafe fn imx_obtain_fixed_of_clock(np: *mut device_node, name: *const core::ffi::c_char, rate: u64) -> *mut clk_hw {
    let clk = of_clk_get_by_name(np, name);
    if is_err(clk) {
        imx_obtain_fixed_clock_hw(name, rate)
    } else {
        __clk_get_hw(clk)
    }
}

pub unsafe fn imx_get_clk_hw_by_name(np: *mut device_node, name: *const core::ffi::c_char) -> *mut clk_hw {
    let clk = of_clk_get_by_name(np, name);
    if is_err(clk) {
        return err_ptr(-ENOENT);
    }
    __clk_get_hw(clk)
}

// This fixups the register CCM_CSCMR1 write value.  The write/read/divider
// values of the aclk_podf field have the relationship described by the table
// in the original C implementation.  The xor operation performs the fixup.
const CSCMR1_FIXUP: u32 = 0x00600000;

pub unsafe fn imx_cscmr1_fixup(val: *mut u32) {
    *val ^= CSCMR1_FIXUP;
}

// The following section is excluded when building as a module in C.
static mut imx_keep_uart_clocks: bool = false;
static mut imx_enabled_uart_clocks: i32 = 0;
static mut imx_uart_clocks: *mut *mut clk = core::ptr::null_mut();

unsafe fn imx_keep_uart_clocks_param(_str: *mut core::ffi::c_char) -> i32 {
    imx_keep_uart_clocks = true;
    0
}

pub unsafe fn imx_register_uart_clocks() {
    let mut num: u32 = 0;
    imx_enabled_uart_clocks = 0;

    // CONFIG_OF is a build-time condition; when disabled this function does nothing.
    #[cfg(CONFIG_OF)]
    {
        if imx_keep_uart_clocks {
            num = of_clk_get_parent_count(of_stdout);
            if num == 0 || of_stdout.is_null() {
                return;
            }

            imx_uart_clocks = kzalloc_objs::<*mut clk>(num);
            if imx_uart_clocks.is_null() {
                return;
            }

            let mut i: u32 = 0;
            while i < num {
                *imx_uart_clocks.add(imx_enabled_uart_clocks as usize) = of_clk_get(of_stdout, i);
                if is_err(*imx_uart_clocks.add(imx_enabled_uart_clocks as usize)) {
                    return;
                }
                if !(*imx_uart_clocks.add(imx_enabled_uart_clocks as usize)).is_null() {
                    clk_prepare_enable(*imx_uart_clocks.add(imx_enabled_uart_clocks as usize));
                    imx_enabled_uart_clocks += 1;
                }
                i += 1;
            }
        }
    }
}

unsafe fn imx_clk_disable_uart() -> i32 {
    if imx_keep_uart_clocks && imx_enabled_uart_clocks != 0 {
        let mut i: i32 = 0;
        while i < imx_enabled_uart_clocks {
            clk_disable_unprepare(*imx_uart_clocks.add(i as usize));
            clk_put(*imx_uart_clocks.add(i as usize));
            i += 1;
        }
    }
    kfree(imx_uart_clocks as *mut core::ffi::c_void);
    0
}

// late_initcall_sync(imx_clk_disable_uart);

// MODULE_DESCRIPTION("Common clock support for NXP i.MX SoC family");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
