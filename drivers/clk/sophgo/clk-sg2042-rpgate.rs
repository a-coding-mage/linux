// SPDX-License-Identifier: GPL-2.0
/*
 * Sophgo SG2042 RP clock Driver
 *
 * Copyright (C) 2024 Sophgo Technology Inc.
 * Copyright (C) 2024 Chen Wang <unicorn_wang@outlook.com>
 */

// Dependencies supplied by the Linux clock, platform, device-tree, and SG2042 headers.

const R_SYSGATE_BEGIN: u32 = 0x0368;
const R_RP_RXU_CLK_ENABLE: u32 = 0x0368 - R_SYSGATE_BEGIN;
const R_MP0_STATUS_REG: u32 = 0x0380 - R_SYSGATE_BEGIN;
const R_MP0_CONTROL_REG: u32 = 0x0384 - R_SYSGATE_BEGIN;
const R_MP1_STATUS_REG: u32 = 0x0388 - R_SYSGATE_BEGIN;
const R_MP1_CONTROL_REG: u32 = 0x038C - R_SYSGATE_BEGIN;
const R_MP2_STATUS_REG: u32 = 0x0390 - R_SYSGATE_BEGIN;
const R_MP2_CONTROL_REG: u32 = 0x0394 - R_SYSGATE_BEGIN;
const R_MP3_STATUS_REG: u32 = 0x0398 - R_SYSGATE_BEGIN;
const R_MP3_CONTROL_REG: u32 = 0x039C - R_SYSGATE_BEGIN;
const R_MP4_STATUS_REG: u32 = 0x03A0 - R_SYSGATE_BEGIN;
const R_MP4_CONTROL_REG: u32 = 0x03A4 - R_SYSGATE_BEGIN;
const R_MP5_STATUS_REG: u32 = 0x03A8 - R_SYSGATE_BEGIN;
const R_MP5_CONTROL_REG: u32 = 0x03AC - R_SYSGATE_BEGIN;
const R_MP6_STATUS_REG: u32 = 0x03B0 - R_SYSGATE_BEGIN;
const R_MP6_CONTROL_REG: u32 = 0x03B4 - R_SYSGATE_BEGIN;
const R_MP7_STATUS_REG: u32 = 0x03B8 - R_SYSGATE_BEGIN;
const R_MP7_CONTROL_REG: u32 = 0x03BC - R_SYSGATE_BEGIN;
const R_MP8_STATUS_REG: u32 = 0x03C0 - R_SYSGATE_BEGIN;
const R_MP8_CONTROL_REG: u32 = 0x03C4 - R_SYSGATE_BEGIN;
const R_MP9_STATUS_REG: u32 = 0x03C8 - R_SYSGATE_BEGIN;
const R_MP9_CONTROL_REG: u32 = 0x03CC - R_SYSGATE_BEGIN;
const R_MP10_STATUS_REG: u32 = 0x03D0 - R_SYSGATE_BEGIN;
const R_MP10_CONTROL_REG: u32 = 0x03D4 - R_SYSGATE_BEGIN;
const R_MP11_STATUS_REG: u32 = 0x03D8 - R_SYSGATE_BEGIN;
const R_MP11_CONTROL_REG: u32 = 0x03DC - R_SYSGATE_BEGIN;
const R_MP12_STATUS_REG: u32 = 0x03E0 - R_SYSGATE_BEGIN;
const R_MP12_CONTROL_REG: u32 = 0x03E4 - R_SYSGATE_BEGIN;
const R_MP13_STATUS_REG: u32 = 0x03E8 - R_SYSGATE_BEGIN;
const R_MP13_CONTROL_REG: u32 = 0x03EC - R_SYSGATE_BEGIN;
const R_MP14_STATUS_REG: u32 = 0x03F0 - R_SYSGATE_BEGIN;
const R_MP14_CONTROL_REG: u32 = 0x03F4 - R_SYSGATE_BEGIN;
const R_MP15_STATUS_REG: u32 = 0x03F8 - R_SYSGATE_BEGIN;
const R_MP15_CONTROL_REG: u32 = 0x03FC - R_SYSGATE_BEGIN;

/** Gate clock for RP (riscv processors) subsystem. */
#[repr(C)]
pub struct Sg2042RpgateClock {
    pub hw: ClkHw,
    pub id: u32,
    pub offset_enable: u32,
    pub bit_idx: u8,
}

macro_rules! sg2042_gate_fw {
    ($id:expr, $name:literal, $parent:literal, $flags:expr, $enable:expr, $bit:expr) => {
        Sg2042RpgateClock {
            hw: ClkHw { init: CLK_HW_INIT_FW_NAME!($name, $parent, $flags) },
            id: $id,
            offset_enable: $enable,
            bit_idx: $bit,
        }
    };
}

/* Gate clocks for RP subsystem (including the MP subsystem). */
static SG2042_GATE_RP: [Sg2042RpgateClock; 48] = [
    sg2042_gate_fw!(GATE_CLK_RXU0, "clk_gate_rxu0", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 0),
    sg2042_gate_fw!(GATE_CLK_RXU1, "clk_gate_rxu1", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 1),
    sg2042_gate_fw!(GATE_CLK_RXU2, "clk_gate_rxu2", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 2),
    sg2042_gate_fw!(GATE_CLK_RXU3, "clk_gate_rxu3", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 3),
    sg2042_gate_fw!(GATE_CLK_RXU4, "clk_gate_rxu4", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 4),
    sg2042_gate_fw!(GATE_CLK_RXU5, "clk_gate_rxu5", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 5),
    sg2042_gate_fw!(GATE_CLK_RXU6, "clk_gate_rxu6", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 6),
    sg2042_gate_fw!(GATE_CLK_RXU7, "clk_gate_rxu7", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 7),
    sg2042_gate_fw!(GATE_CLK_RXU8, "clk_gate_rxu8", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 8),
    sg2042_gate_fw!(GATE_CLK_RXU9, "clk_gate_rxu9", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 9),
    sg2042_gate_fw!(GATE_CLK_RXU10, "clk_gate_rxu10", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 10),
    sg2042_gate_fw!(GATE_CLK_RXU11, "clk_gate_rxu11", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 11),
    sg2042_gate_fw!(GATE_CLK_RXU12, "clk_gate_rxu12", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 12),
    sg2042_gate_fw!(GATE_CLK_RXU13, "clk_gate_rxu13", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 13),
    sg2042_gate_fw!(GATE_CLK_RXU14, "clk_gate_rxu14", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 14),
    sg2042_gate_fw!(GATE_CLK_RXU15, "clk_gate_rxu15", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 15),
    sg2042_gate_fw!(GATE_CLK_RXU16, "clk_gate_rxu16", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 16),
    sg2042_gate_fw!(GATE_CLK_RXU17, "clk_gate_rxu17", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 17),
    sg2042_gate_fw!(GATE_CLK_RXU18, "clk_gate_rxu18", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 18),
    sg2042_gate_fw!(GATE_CLK_RXU19, "clk_gate_rxu19", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 19),
    sg2042_gate_fw!(GATE_CLK_RXU20, "clk_gate_rxu20", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 20),
    sg2042_gate_fw!(GATE_CLK_RXU21, "clk_gate_rxu21", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 21),
    sg2042_gate_fw!(GATE_CLK_RXU22, "clk_gate_rxu22", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 22),
    sg2042_gate_fw!(GATE_CLK_RXU23, "clk_gate_rxu23", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 23),
    sg2042_gate_fw!(GATE_CLK_RXU24, "clk_gate_rxu24", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 24),
    sg2042_gate_fw!(GATE_CLK_RXU25, "clk_gate_rxu25", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 25),
    sg2042_gate_fw!(GATE_CLK_RXU26, "clk_gate_rxu26", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 26),
    sg2042_gate_fw!(GATE_CLK_RXU27, "clk_gate_rxu27", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 27),
    sg2042_gate_fw!(GATE_CLK_RXU28, "clk_gate_rxu28", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 28),
    sg2042_gate_fw!(GATE_CLK_RXU29, "clk_gate_rxu29", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 29),
    sg2042_gate_fw!(GATE_CLK_RXU30, "clk_gate_rxu30", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 30),
    sg2042_gate_fw!(GATE_CLK_RXU31, "clk_gate_rxu31", "rpgate", 0, R_RP_RXU_CLK_ENABLE, 31),
    sg2042_gate_fw!(GATE_CLK_MP0, "clk_gate_mp0", "rpgate", CLK_IS_CRITICAL, R_MP0_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP1, "clk_gate_mp1", "rpgate", CLK_IS_CRITICAL, R_MP1_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP2, "clk_gate_mp2", "rpgate", CLK_IS_CRITICAL, R_MP2_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP3, "clk_gate_mp3", "rpgate", CLK_IS_CRITICAL, R_MP3_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP4, "clk_gate_mp4", "rpgate", CLK_IS_CRITICAL, R_MP4_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP5, "clk_gate_mp5", "rpgate", CLK_IS_CRITICAL, R_MP5_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP6, "clk_gate_mp6", "rpgate", CLK_IS_CRITICAL, R_MP6_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP7, "clk_gate_mp7", "rpgate", CLK_IS_CRITICAL, R_MP7_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP8, "clk_gate_mp8", "rpgate", CLK_IS_CRITICAL, R_MP8_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP9, "clk_gate_mp9", "rpgate", CLK_IS_CRITICAL, R_MP9_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP10, "clk_gate_mp10", "rpgate", CLK_IS_CRITICAL, R_MP10_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP11, "clk_gate_mp11", "rpgate", CLK_IS_CRITICAL, R_MP11_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP12, "clk_gate_mp12", "rpgate", CLK_IS_CRITICAL, R_MP12_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP13, "clk_gate_mp13", "rpgate", CLK_IS_CRITICAL, R_MP13_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP14, "clk_gate_mp14", "rpgate", CLK_IS_CRITICAL, R_MP14_CONTROL_REG, 0),
    sg2042_gate_fw!(GATE_CLK_MP15, "clk_gate_mp15", "rpgate", CLK_IS_CRITICAL, R_MP15_CONTROL_REG, 0),
];

static mut SG2042_CLK_LOCK: Spinlock = DEFINE_SPINLOCK!();

unsafe fn sg2042_clk_register_rpgates(
    dev: *mut Device,
    clk_data: *mut Sg2042ClkData,
    gate_clks: *const Sg2042RpgateClock,
    num_gate_clks: i32,
) -> i32 {
    let mut ret = 0;
    for i in 0..num_gate_clks {
        let gate = &*gate_clks.add(i as usize);
        let hw = devm_clk_hw_register_gate_parent_data(
            dev, gate.hw.init.name, gate.hw.init.parent_data, gate.hw.init.flags,
            (*clk_data).iobase.add(gate.offset_enable as usize), gate.bit_idx, 0,
            &raw mut SG2042_CLK_LOCK,
        );
        if IS_ERR(hw) {
            pr_err!("failed to register clock %s\n", gate.hw.init.name);
            ret = PTR_ERR(hw);
            break;
        }
        (*clk_data).onecell_data.hws[gate.id as usize] = hw;
    }
    ret
}

unsafe fn sg2042_init_clkdata(
    pdev: *mut PlatformDevice, num_clks: i32, pp_clk_data: *mut *mut Sg2042ClkData,
) -> i32 {
    let clk_data = devm_kzalloc(&mut (*pdev).dev, struct_size!(Sg2042ClkData, onecell_data.hws, num_clks), GFP_KERNEL);
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).iobase = devm_platform_ioremap_resource(pdev, 0);
    if WARN_ON(IS_ERR((*clk_data).iobase)) { return PTR_ERR((*clk_data).iobase); }
    (*clk_data).onecell_data.num = num_clks;
    *pp_clk_data = clk_data;
    0
}

unsafe fn sg2042_rpgate_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut clk_data: *mut Sg2042ClkData = core::ptr::null_mut();
    let num_clks = SG2042_GATE_RP.len() as i32;
    let mut ret = sg2042_init_clkdata(pdev, num_clks, &mut clk_data);
    if ret != 0 { goto_error!(error_out); }
    ret = sg2042_clk_register_rpgates(&mut (*pdev).dev, clk_data, SG2042_GATE_RP.as_ptr(), num_clks);
    if ret != 0 { goto_error!(error_out); }
    return devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_onecell_get, &mut (*clk_data).onecell_data);
error_out:
    pr_err!("%s failed error number %d\n", "sg2042_rpgate_probe", ret);
    ret
}

static SG2042_RPGATE_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "sophgo,sg2042-rpgate" },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut SG2042_RPGATE_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(sg2042_rpgate_probe),
    driver: DeviceDriver {
        name: "clk-sophgo-sg2042-rpgate",
        of_match_table: SG2042_RPGATE_MATCH.as_ptr(),
        suppress_bind_attrs: true,
    },
};

module_platform_driver!(SG2042_RPGATE_DRIVER);
MODULE_DEVICE_TABLE!(of, SG2042_RPGATE_MATCH);
MODULE_AUTHOR!("Chen Wang");
MODULE_DESCRIPTION!("Sophgo SG2042 rp subsystem clock driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
