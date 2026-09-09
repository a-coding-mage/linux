// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hisilicon clock driver
 *
 * Copyright (c) 2013-2017 Hisilicon Limited.
 * Copyright (c) 2017 Linaro Limited.
 *
 * Author: Kai Zhao <zhaokai1@hisilicon.com>
 *         Tao Wang <kevin.wangtao@hisilicon.com>
 *         Leo Yan <leo.yan@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

const HI3660_STUB_CLOCK_DATA: usize = 0x70;
const MHZ: u32 = 1000 * 1000;

#[repr(C)]
struct hi3660_stub_clk_chan {
    cl: mailbox_client,
    mbox: *mut mailbox_channel,
}

#[repr(C)]
struct hi3660_stub_clk {
    id: u32,
    hw: clk_hw,
    cmd: u32,
    msg: [u32; 8],
    rate: u32,
}

extern "C" {
    static mut freq_reg: *mut core::ffi::c_void;
    static mut stub_clk_chan: hi3660_stub_clk_chan;

    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn mbox_send_message(chan: *mut mailbox_channel, data: *mut core::ffi::c_void) -> i32;
    fn mbox_client_txdone(chan: *mut mailbox_channel, r: i32);
    fn clk_determine_rate_noop(hw: *mut clk_hw, req: *mut core::ffi::c_void) -> i32;
    fn mbox_request_channel(cl: *mut mailbox_client, index: u32) -> *mut mailbox_channel;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn devm_of_clk_add_hw_provider(
        dev: *mut device,
        get: unsafe extern "C" fn(*mut of_phandle_args, *mut core::ffi::c_void) -> *mut clk_hw,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
}

#[repr(C)]
struct mailbox_client {
    dev: *mut device,
    tx_done: *mut core::ffi::c_void,
    tx_block: bool,
    knows_txdone: bool,
}

#[repr(C)] struct mailbox_channel { _private: [u8; 0] }
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct platform_device { dev: device, _private: [u8; 0] }
#[repr(C)] struct clk_hw { _private: [u8; 0] }
#[repr(C)] struct of_phandle_args { args: [u32; 16] }
#[repr(C)] struct platform_driver { _private: [u8; 0] }

const HI3660_CLK_STUB_NUM: usize = 4;
const HI3660_CLK_STUB_CLUSTER0: u32 = 0;
const HI3660_CLK_STUB_CLUSTER1: u32 = 1;
const HI3660_CLK_STUB_GPU: u32 = 2;
const HI3660_CLK_STUB_DDR: u32 = 3;

unsafe fn hi3660_stub_clk_recalc_rate(hw: *mut clk_hw, _parent_rate: u64) -> u64 {
    let stub_clk = hw as *mut hi3660_stub_clk;
    (*stub_clk).rate = readl((freq_reg as *mut u8).add(((*stub_clk).id << 2) as usize))
        .wrapping_mul(MHZ);
    (*stub_clk).rate as u64
}

unsafe fn hi3660_stub_clk_set_rate(hw: *mut clk_hw, rate: u64, _parent_rate: u64) -> i32 {
    let stub_clk = hw as *mut hi3660_stub_clk;
    (*stub_clk).msg[0] = (*stub_clk).cmd;
    (*stub_clk).msg[1] = (rate / MHZ as u64) as u32;
    dev_dbg((*stub_clk_chan).cl.dev, core::ptr::null(), (*stub_clk).msg[0], (*stub_clk).msg[1]);
    mbox_send_message(stub_clk_chan.mbox, (*stub_clk).msg.as_mut_ptr() as *mut core::ffi::c_void);
    mbox_client_txdone(stub_clk_chan.mbox, 0);
    (*stub_clk).rate = rate as u32;
    0
}

static mut hi3660_stub_clks: [hi3660_stub_clk; HI3660_CLK_STUB_NUM] = [
    hi3660_stub_clk { id: HI3660_CLK_STUB_CLUSTER0, hw: clk_hw { _private: [] }, cmd: 0x0001030A, msg: [0; 8], rate: 0 },
    hi3660_stub_clk { id: HI3660_CLK_STUB_CLUSTER1, hw: clk_hw { _private: [] }, cmd: 0x0002030A, msg: [0; 8], rate: 0 },
    hi3660_stub_clk { id: HI3660_CLK_STUB_GPU, hw: clk_hw { _private: [] }, cmd: 0x0003030A, msg: [0; 8], rate: 0 },
    hi3660_stub_clk { id: HI3660_CLK_STUB_DDR, hw: clk_hw { _private: [] }, cmd: 0x00040309, msg: [0; 8], rate: 0 },
];

unsafe extern "C" fn hi3660_stub_clk_hw_get(clkspec: *mut of_phandle_args, _data: *mut core::ffi::c_void) -> *mut clk_hw {
    let idx = (*clkspec).args[0] as usize;
    if idx >= HI3660_CLK_STUB_NUM { return core::ptr::null_mut(); }
    &mut hi3660_stub_clks[idx].hw
}

unsafe fn hi3660_stub_clk_probe(pdev: *mut platform_device) -> i32 {
    stub_clk_chan.cl.dev = &mut (*pdev).dev;
    stub_clk_chan.cl.tx_done = core::ptr::null_mut();
    stub_clk_chan.cl.tx_block = false;
    stub_clk_chan.cl.knows_txdone = false;
    stub_clk_chan.mbox = mbox_request_channel(&mut stub_clk_chan.cl, 0);
    freq_reg = devm_platform_ioremap_resource(pdev, 0);
    freq_reg = (freq_reg as *mut u8).add(HI3660_STUB_CLOCK_DATA) as *mut core::ffi::c_void;
    for i in 0..HI3660_CLK_STUB_NUM {
        let ret = devm_clk_hw_register(&mut (*pdev).dev, &mut hi3660_stub_clks[i].hw);
        if ret != 0 { return ret; }
    }
    devm_of_clk_add_hw_provider(&mut (*pdev).dev, hi3660_stub_clk_hw_get, hi3660_stub_clks.as_mut_ptr() as *mut core::ffi::c_void)
}

unsafe fn hi3660_stub_clk_init() -> i32 { platform_driver_register(core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
