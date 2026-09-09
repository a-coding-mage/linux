// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/mach-sa1100/clock.c
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static mut GAFR: u32;
    static mut GPDR: u32;
    static mut TUCR: u32;

    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn sa11x0_getspeed(cpu: i32) -> usize;

    fn clk_hw_register_fixed_rate(
        dev: *mut core::ffi::c_void,
        name: *const i8,
        parent_name: *const i8,
        flags: u32,
        rate: u32,
    ) -> *mut clk_hw;
    fn clk_hw_register_clkdev(
        hw: *mut clk_hw,
        con_id: *const i8,
        dev_id: *const i8,
    );
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn clk_hw_register_mux(
        dev: *mut core::ffi::c_void,
        name: *const i8,
        parent_names: *const *const i8,
        num_parents: usize,
        flags: u32,
        reg: *mut u32,
        shift: u8,
        width: u8,
        clk_flags: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk_hw;
    fn clk_set_rate(clk: *mut clk, rate: usize) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const i8,
    pub ops: *const clk_ops,
    pub parent_names: *const *const i8,
    pub num_parents: usize,
    pub flags: u32,
}

const GPIO_32_768KHZ: u32 = 1 << 27;
const CLK_GET_RATE_NOCACHE: u32 = 1 << 0;
const CLK_IS_CRITICAL: u32 = 1 << 1;

static CLK_TUCR_PARENTS: [*const i8; 2] = [b"clk32768\0".as_ptr() as *const i8, b"clk3686400\0".as_ptr() as *const i8];

// DEFINE_SPINLOCK(tucr_lock)
static mut TUCR_LOCK: spinlock_t = spinlock_t { _private: [] };

unsafe extern "C" fn clk_gpio27_enable(_hw: *mut clk_hw) -> i32 {
    let mut flags: usize = 0;

    /*
     * First, set up the 3.6864MHz clock on GPIO 27 for the SA-1111:
     * (SA-1110 Developer's Manual, section 9.1.2.1)
     */
    local_irq_save(&mut flags);
    GAFR |= GPIO_32_768KHZ;
    GPDR |= GPIO_32_768KHZ;
    local_irq_restore(flags);

    0
}

unsafe extern "C" fn clk_gpio27_disable(_hw: *mut clk_hw) {
    let mut flags: usize = 0;

    local_irq_save(&mut flags);
    GPDR &= !GPIO_32_768KHZ;
    GAFR &= !GPIO_32_768KHZ;
    local_irq_restore(flags);
}

static CLK_GPIO27_OPS: clk_ops = clk_ops {
    enable: Some(clk_gpio27_enable),
    disable: Some(clk_gpio27_disable),
    recalc_rate: None,
};

static CLK_GPIO27_PARENTS: [*const i8; 1] = [b"tucr-mux\0".as_ptr() as *const i8];

static CLK_GPIO27_INIT_DATA: clk_init_data = clk_init_data {
    name: b"gpio27\0".as_ptr() as *const i8,
    ops: &CLK_GPIO27_OPS,
    parent_names: CLK_GPIO27_PARENTS.as_ptr(),
    num_parents: CLK_GPIO27_PARENTS.len(),
    flags: 0,
};

/*
 * Derived from the table 8-1 in the SA1110 manual, the MPLL appears to
 * multiply its input rate by 4 x (4 + PPCR).  This calculation gives
 * the exact rate.  The figures given in the table are the rates rounded
 * to 100kHz.  Stick with sa11x0_getspeed() for the time being.
 */
unsafe extern "C" fn clk_mpll_recalc_rate(_hw: *mut clk_hw, _prate: usize) -> usize {
    sa11x0_getspeed(0) * 1000
}

static CLK_MPLL_OPS: clk_ops = clk_ops {
    enable: None,
    disable: None,
    recalc_rate: Some(clk_mpll_recalc_rate),
};

static CLK_MPLL_PARENTS: [*const i8; 1] = [b"clk3686400\0".as_ptr() as *const i8];

static CLK_MPLL_INIT_DATA: clk_init_data = clk_init_data {
    name: b"mpll\0".as_ptr() as *const i8,
    ops: &CLK_MPLL_OPS,
    parent_names: CLK_MPLL_PARENTS.as_ptr(),
    num_parents: CLK_MPLL_PARENTS.len(),
    flags: CLK_GET_RATE_NOCACHE | CLK_IS_CRITICAL,
};

#[no_mangle]
pub unsafe extern "C" fn sa11xx_clk_init() -> i32 {
    let mut hw: *mut clk_hw;
    let mut ret: i32;

    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"clk32768\0".as_ptr() as *const i8, core::ptr::null(), 0, 32768);
    if hw.is_null() {
        return -(12);
    }

    clk_hw_register_clkdev(hw, core::ptr::null(), b"sa1100-rtc\0".as_ptr() as *const i8);

    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"clk3686400\0".as_ptr() as *const i8, core::ptr::null(), 0, 3686400);
    if hw.is_null() {
        return -(12);
    }

    clk_hw_register_clkdev(hw, b"OSTIMER0\0".as_ptr() as *const i8, core::ptr::null());

    hw = kzalloc(core::mem::size_of::<clk_hw>(), 0) as *mut clk_hw;
    if hw.is_null() {
        return -12;
    }
    (*hw).init = &CLK_MPLL_INIT_DATA;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(hw as *mut core::ffi::c_void);
        return ret;
    }

    clk_hw_register_clkdev(hw, core::ptr::null(), b"sa11x0-fb\0".as_ptr() as *const i8);
    clk_hw_register_clkdev(hw, core::ptr::null(), b"sa11x0-pcmcia\0".as_ptr() as *const i8);
    clk_hw_register_clkdev(hw, core::ptr::null(), b"sa11x0-pcmcia.0\0".as_ptr() as *const i8);
    clk_hw_register_clkdev(hw, core::ptr::null(), b"sa11x0-pcmcia.1\0".as_ptr() as *const i8);
    clk_hw_register_clkdev(hw, core::ptr::null(), b"1800\0".as_ptr() as *const i8);

    hw = clk_hw_register_mux(
        core::ptr::null_mut(),
        b"tucr-mux\0".as_ptr() as *const i8,
        CLK_TUCR_PARENTS.as_ptr(),
        CLK_TUCR_PARENTS.len(),
        0,
        &mut TUCR,
        0,
        0,
        0,
        &mut TUCR_LOCK,
    );
    clk_set_rate((*hw).clk, 3686400);

    hw = kzalloc(core::mem::size_of::<clk_hw>(), 0) as *mut clk_hw;
    if hw.is_null() {
        return -12;
    }
    (*hw).init = &CLK_GPIO27_INIT_DATA;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(hw as *mut core::ffi::c_void);
        return ret;
    }

    clk_hw_register_clkdev(hw, core::ptr::null(), b"sa1111.0\0".as_ptr() as *const i8);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
