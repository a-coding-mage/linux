// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Linaro Limited

// Dependencies from the original C includes:
// linux/err.h, linux/init.h, linux/clk-provider.h, linux/module.h,
// linux/device.h, linux/platform_device.h, linux/of.h, linux/slab.h,
// dt-bindings/sound/qcom,q6dsp-lpass-ports.h, q6dsp-lpass-clocks.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const Q6DSP_MAX_CLK_ID: usize = 105;
const Q6DSP_LPASS_CLK_ROOT_DEFAULT: c_int = 0;

const GFP_KERNEL: c_int = 0;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;

extern "C" {
    static LPASS_CLK_ATTRIBUTE_COUPLE_DIVISOR: u32;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn devm_of_clk_add_hw_provider(
        dev: *mut device,
        get: Option<unsafe extern "C" fn(*mut of_phandle_args, *mut c_void) -> *mut clk_hw>,
        data: *mut c_void,
    ) -> c_int;
    fn clk_hw_get_name(hw: *const clk_hw) -> *const c_char;
    fn ERR_PTR(error: isize) -> *mut c_void;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_rate_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
}

#[repr(C)]
pub struct of_phandle_args {
    pub args: [u32; 0],
}

#[repr(C)]
pub struct q6dsp_clk_init {
    pub clk_id: u32,
    pub q6dsp_clk_id: c_int,
    pub name: *const c_char,
    pub rate: c_int,
}

#[repr(C)]
pub struct q6dsp_clk_desc {
    pub clks: *const q6dsp_clk_init,
    pub num_clks: c_int,
    pub lpass_set_clk:
        unsafe extern "C" fn(*mut device, c_int, c_int, c_int, c_int) -> c_int,
    pub lpass_vote_clk:
        unsafe extern "C" fn(*mut device, c_int, *const c_char, *mut u32) -> c_int,
    pub lpass_unvote_clk: unsafe extern "C" fn(*mut device, c_int, u32),
}

#[repr(C)]
struct q6dsp_clk {
    dev: *mut device,
    q6dsp_clk_id: c_int,
    attributes: c_int,
    rate: c_int,
    handle: u32,
    hw: clk_hw,
}

unsafe fn to_q6dsp_clk(_hw: *mut clk_hw) -> *mut q6dsp_clk {
    (_hw as *mut u8).sub(core::mem::offset_of!(q6dsp_clk, hw)) as *mut q6dsp_clk
}

#[repr(C)]
struct q6dsp_cc {
    dev: *mut device,
    clks: [*mut q6dsp_clk; Q6DSP_MAX_CLK_ID],
    desc: *const q6dsp_clk_desc,
}

unsafe extern "C" fn clk_q6dsp_prepare(hw: *mut clk_hw) -> c_int {
    let clk = to_q6dsp_clk(hw);
    let cc = dev_get_drvdata((*clk).dev) as *mut q6dsp_cc;

    ((*(*cc).desc).lpass_set_clk)(
        (*clk).dev,
        (*clk).q6dsp_clk_id,
        (*clk).attributes,
        Q6DSP_LPASS_CLK_ROOT_DEFAULT,
        (*clk).rate,
    )
}

unsafe extern "C" fn clk_q6dsp_unprepare(hw: *mut clk_hw) {
    let clk = to_q6dsp_clk(hw);
    let cc = dev_get_drvdata((*clk).dev) as *mut q6dsp_cc;

    ((*(*cc).desc).lpass_set_clk)(
        (*clk).dev,
        (*clk).q6dsp_clk_id,
        (*clk).attributes,
        Q6DSP_LPASS_CLK_ROOT_DEFAULT,
        0,
    );
}

unsafe extern "C" fn clk_q6dsp_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    _parent_rate: c_ulong,
) -> c_int {
    let clk = to_q6dsp_clk(hw);

    (*clk).rate = rate as c_int;

    0
}

unsafe extern "C" fn clk_q6dsp_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let clk = to_q6dsp_clk(hw);

    (*clk).rate as c_ulong
}

unsafe extern "C" fn clk_q6dsp_determine_rate(
    _hw: *mut clk_hw,
    _req: *mut clk_rate_request,
) -> c_int {
    0
}

static clk_q6dsp_ops: clk_ops = clk_ops {
    prepare: Some(clk_q6dsp_prepare),
    unprepare: Some(clk_q6dsp_unprepare),
    set_rate: Some(clk_q6dsp_set_rate),
    determine_rate: Some(clk_q6dsp_determine_rate),
    recalc_rate: Some(clk_q6dsp_recalc_rate),
};

unsafe extern "C" fn clk_vote_q6dsp_block(hw: *mut clk_hw) -> c_int {
    let clk = to_q6dsp_clk(hw);
    let cc = dev_get_drvdata((*clk).dev) as *mut q6dsp_cc;

    ((*(*cc).desc).lpass_vote_clk)(
        (*clk).dev,
        (*clk).q6dsp_clk_id,
        clk_hw_get_name(&mut (*clk).hw),
        &mut (*clk).handle,
    )
}

unsafe extern "C" fn clk_unvote_q6dsp_block(hw: *mut clk_hw) {
    let clk = to_q6dsp_clk(hw);
    let cc = dev_get_drvdata((*clk).dev) as *mut q6dsp_cc;

    ((*(*cc).desc).lpass_unvote_clk)((*clk).dev, (*clk).q6dsp_clk_id, (*clk).handle);
}

static clk_vote_q6dsp_ops: clk_ops = clk_ops {
    prepare: Some(clk_vote_q6dsp_block),
    unprepare: Some(clk_unvote_q6dsp_block),
    set_rate: None,
    determine_rate: None,
    recalc_rate: None,
};

unsafe extern "C" fn q6dsp_of_clk_hw_get(
    clkspec: *mut of_phandle_args,
    data: *mut c_void,
) -> *mut clk_hw {
    let cc = data as *mut q6dsp_cc;
    let idx = (*clkspec).args.as_ptr().add(0).read() as u32;
    let attr = (*clkspec).args.as_ptr().add(1).read() as u32;

    if idx as usize >= Q6DSP_MAX_CLK_ID || attr > LPASS_CLK_ATTRIBUTE_COUPLE_DIVISOR {
        dev_err(
            (*cc).dev,
            b"Invalid clk specifier (%d, %d)\n\0".as_ptr() as *const c_char,
            idx,
            attr,
        );
        return ERR_PTR(-(EINVAL as isize)) as *mut clk_hw;
    }

    if !(*cc).clks[idx as usize].is_null() {
        (*(*cc).clks[idx as usize]).attributes = attr as c_int;
        return &mut (*(*cc).clks[idx as usize]).hw;
    }

    ERR_PTR(-(ENOENT as isize)) as *mut clk_hw
}

#[no_mangle]
pub unsafe extern "C" fn q6dsp_clock_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut cc: *mut q6dsp_cc;
    let dev = &mut (*pdev).dev as *mut device;
    let q6dsp_clks: *const q6dsp_clk_init;
    let desc: *const q6dsp_clk_desc;
    let mut i: c_int;
    let mut ret: c_int;

    cc = devm_kzalloc(dev, core::mem::size_of::<q6dsp_cc>(), GFP_KERNEL) as *mut q6dsp_cc;
    if cc.is_null() {
        return -ENOMEM;
    }

    desc = of_device_get_match_data(&mut (*pdev).dev) as *const q6dsp_clk_desc;
    if desc.is_null() {
        return -EINVAL;
    }

    (*cc).desc = desc;
    (*cc).dev = dev;
    q6dsp_clks = (*desc).clks;

    i = 0;
    while i < (*desc).num_clks {
        let id = (*q6dsp_clks.add(i as usize)).clk_id;
        let mut init = MaybeUninit::<clk_init_data>::zeroed().assume_init();
        let mut clk: *mut q6dsp_clk;

        init.name = (*q6dsp_clks.add(i as usize)).name;

        clk = devm_kzalloc(dev, core::mem::size_of::<q6dsp_clk>(), GFP_KERNEL) as *mut q6dsp_clk;
        if clk.is_null() {
            return -ENOMEM;
        }

        (*clk).dev = dev;
        (*clk).q6dsp_clk_id = (*q6dsp_clks.add(i as usize)).q6dsp_clk_id;
        (*clk).rate = (*q6dsp_clks.add(i as usize)).rate;
        (*clk).hw.init = &init;

        if (*clk).rate != 0 {
            init.ops = &clk_q6dsp_ops;
        } else {
            init.ops = &clk_vote_q6dsp_ops;
        }

        (*cc).clks[id as usize] = clk;

        ret = devm_clk_hw_register(dev, &mut (*clk).hw);
        if ret != 0 {
            return ret;
        }

        i += 1;
    }

    ret = devm_of_clk_add_hw_provider(dev, Some(q6dsp_of_clk_hw_get), cc as *mut c_void);
    if ret != 0 {
        return ret;
    }

    dev_set_drvdata(dev, cc as *mut c_void);

    0
}

// EXPORT_SYMBOL_GPL(q6dsp_clock_dev_probe);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
