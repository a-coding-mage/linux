// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hi6220 stub clock driver
 *
 * Copyright (c) 2015 Hisilicon Limited.
 * Copyright (c) 2015 Linaro Limited.
 *
 * Author: Leo Yan <leo.yan@linaro.org>
 */

// Dependencies are supplied by the surrounding kernel bindings.

const HI6220_STUB_ACPU0: u32 = 0;
const HI6220_STUB_ACPU1: u32 = 1;
const HI6220_STUB_GPU: u32 = 2;
const HI6220_STUB_DDR: u32 = 5;

const HI6220_MBOX_MSG_LEN: usize = 8;
const HI6220_MBOX_FREQ: u8 = 0xA;
const HI6220_MBOX_CMD_SET: u8 = 0x3;
const HI6220_MBOX_OBJ_AP: u8 = 0x0;

const ACPU_DFS_FREQ_MAX: u32 = 0x1724;
const ACPU_DFS_CUR_FREQ: u32 = 0x17CC;
const ACPU_DFS_FLAG: u32 = 0x1B30;
const ACPU_DFS_FREQ_REQ: u32 = 0x1B34;
const ACPU_DFS_FREQ_LMT: u32 = 0x1B38;
const ACPU_DFS_LOCK_FLAG: u32 = 0xAEAEAEAE;

#[repr(C)]
struct hi6220_stub_clk {
    id: u32,
    dev: *mut device,
    hw: clk_hw,
    dfs_map: *mut regmap,
    cl: mbox_client,
    mbox: *mut mbox_chan,
}

#[repr(C)]
struct hi6220_mbox_msg {
    type_: u8,
    cmd: u8,
    obj: u8,
    src: u8,
    para: [u8; 4],
}

#[repr(C)]
union hi6220_mbox_data {
    data: [u32; HI6220_MBOX_MSG_LEN],
    msg: hi6220_mbox_msg,
}

unsafe fn hi6220_acpu_get_freq(stub_clk: *mut hi6220_stub_clk) -> u32 {
    let mut freq: u32 = 0;
    regmap_read((*stub_clk).dfs_map, ACPU_DFS_CUR_FREQ, &mut freq);
    freq
}

unsafe fn hi6220_acpu_set_freq(stub_clk: *mut hi6220_stub_clk, freq: u32) -> i32 {
    let mut data: hi6220_mbox_data = core::mem::zeroed();
    regmap_write((*stub_clk).dfs_map, ACPU_DFS_FREQ_REQ, freq);

    data.msg.type_ = HI6220_MBOX_FREQ;
    data.msg.cmd = HI6220_MBOX_CMD_SET;
    data.msg.obj = HI6220_MBOX_OBJ_AP;
    data.msg.src = HI6220_MBOX_OBJ_AP;

    mbox_send_message((*stub_clk).mbox, &mut data);
    0
}

unsafe fn hi6220_acpu_round_freq(stub_clk: *mut hi6220_stub_clk, freq: u32) -> u32 {
    let mut limit_flag: u32 = 0;
    let mut limit_freq: u32 = u32::MAX;
    let mut max_freq: u32 = 0;

    regmap_read((*stub_clk).dfs_map, ACPU_DFS_FLAG, &mut limit_flag);
    if limit_flag == ACPU_DFS_LOCK_FLAG {
        regmap_read((*stub_clk).dfs_map, ACPU_DFS_FREQ_LMT, &mut limit_freq);
    }
    regmap_read((*stub_clk).dfs_map, ACPU_DFS_FREQ_MAX, &mut max_freq);
    max_freq = core::cmp::min(max_freq, limit_freq);
    if freq > max_freq {
        // Equivalent to WARN_ON(freq > max_freq): warning machinery is external.
        freq = max_freq;
    }
    freq
}

unsafe fn hi6220_stub_clk_recalc_rate(hw: *mut clk_hw, _parent_rate: usize) -> usize {
    let stub_clk = container_of_stub_clk(hw);
    let mut rate: u32 = 0;
    match (*stub_clk).id {
        HI6220_STUB_ACPU0 => {
            rate = hi6220_acpu_get_freq(stub_clk);
            rate = rate.wrapping_mul(1000);
        }
        _ => dev_err((*stub_clk).dev, "hi6220_stub_clk_recalc_rate: un-supported clock id %d\n", (*stub_clk).id),
    }
    rate as usize
}

unsafe fn hi6220_stub_clk_set_rate(hw: *mut clk_hw, rate: usize, _parent_rate: usize) -> i32 {
    let stub_clk = container_of_stub_clk(hw);
    let new_rate = rate / 1000;
    let mut ret = 0;
    match (*stub_clk).id {
        HI6220_STUB_ACPU0 => {
            ret = hi6220_acpu_set_freq(stub_clk, new_rate as u32);
            if ret < 0 { return ret; }
        }
        _ => dev_err((*stub_clk).dev, "hi6220_stub_clk_set_rate: un-supported clock id %d\n", (*stub_clk).id),
    }
    pr_debug!("hi6220_stub_clk_set_rate: set rate={}kHz\n", new_rate);
    ret
}

unsafe fn hi6220_stub_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let stub_clk = container_of_stub_clk(hw);
    let mut new_rate = (*req).rate / 1000;
    match (*stub_clk).id {
        HI6220_STUB_ACPU0 => new_rate = (hi6220_acpu_round_freq(stub_clk, new_rate as u32) as usize) * 1000,
        _ => dev_err((*stub_clk).dev, "hi6220_stub_clk_determine_rate: un-supported clock id %d\n", (*stub_clk).id),
    }
    (*req).rate = new_rate;
    0
}

static hi6220_stub_clk_ops: clk_ops = clk_ops {
    recalc_rate: Some(hi6220_stub_clk_recalc_rate),
    determine_rate: Some(hi6220_stub_clk_determine_rate),
    set_rate: Some(hi6220_stub_clk_set_rate),
};

unsafe fn hi6220_stub_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*pdev).dev.of_node;
    let stub_clk = devm_kzalloc(dev, core::mem::size_of::<hi6220_stub_clk>(), GFP_KERNEL) as *mut hi6220_stub_clk;
    if stub_clk.is_null() { return -ENOMEM; }

    (*stub_clk).dfs_map = syscon_regmap_lookup_by_phandle(np, "hisilicon,hi6220-clk-sram");
    if IS_ERR((*stub_clk).dfs_map) {
        dev_err(dev, "failed to get sram regmap\n");
        return PTR_ERR((*stub_clk).dfs_map);
    }
    (*stub_clk).hw.init = core::ptr::null_mut();
    (*stub_clk).dev = dev;
    (*stub_clk).id = HI6220_STUB_ACPU0;
    (*stub_clk).cl.dev = dev;
    (*stub_clk).cl.tx_done = None;
    (*stub_clk).cl.tx_block = true;
    (*stub_clk).cl.tx_tout = 500;
    (*stub_clk).cl.knows_txdone = false;
    (*stub_clk).mbox = mbox_request_channel(&mut (*stub_clk).cl, 0);
    if IS_ERR((*stub_clk).mbox) {
        dev_err(dev, "failed get mailbox channel\n");
        return PTR_ERR((*stub_clk).mbox);
    }
    let mut init = clk_init_data { name: "acpu0", ops: &hi6220_stub_clk_ops, num_parents: 0, flags: 0 };
    (*stub_clk).hw.init = &mut init;
    let clk = devm_clk_register(dev, &mut (*stub_clk).hw);
    if IS_ERR(clk) { return PTR_ERR(clk); }
    let ret = of_clk_add_provider(np, of_clk_src_simple_get, clk);
    if ret != 0 {
        dev_err(dev, "failed to register OF clock provider\n");
        return ret;
    }
    regmap_write((*stub_clk).dfs_map, ACPU_DFS_FLAG, 0);
    regmap_write((*stub_clk).dfs_map, ACPU_DFS_FREQ_REQ, 0);
    regmap_write((*stub_clk).dfs_map, ACPU_DFS_FREQ_LMT, 0);
    dev_dbg(dev, "Registered clock '%s'\n", init.name);
    0
}

static hi6220_stub_clk_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "hisilicon,hi6220-stub-clk" },
    of_device_id { compatible: "" },
];

static mut hi6220_stub_clk_driver: platform_driver = platform_driver {
    driver: device_driver { name: "hi6220-stub-clk", of_match_table: &hi6220_stub_clk_of_match },
    probe: Some(hi6220_stub_clk_probe),
};

unsafe fn hi6220_stub_clk_init() -> i32 {
    platform_driver_register(&mut hi6220_stub_clk_driver)
}

// subsys_initcall(hi6220_stub_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
