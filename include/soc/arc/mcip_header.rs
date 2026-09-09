/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ARConnect IP Support (Multi core enabler: Cross core IPI, RTC ...)
 *
 * Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
 */

/* Dependency: <soc/arc/arc_aux.h> */

pub const ARC_REG_MCIP_BCR: u32 = 0x0d0;
pub const ARC_REG_MCIP_IDU_BCR: u32 = 0x0D5;
pub const ARC_REG_GFRC_BUILD: u32 = 0x0D6;
pub const ARC_REG_MCIP_CMD: u32 = 0x600;
pub const ARC_REG_MCIP_WDATA: u32 = 0x601;
pub const ARC_REG_MCIP_READBACK: u32 = 0x602;

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
pub struct McipCmd(pub u32);

impl McipCmd {
    pub const fn new(cmd: u32, param: u32) -> Self {
        Self((cmd & 0xff) | ((param & 0xffff) << 8))
    }

    pub const fn cmd(self) -> u32 { self.0 & 0xff }
    pub const fn param(self) -> u32 { (self.0 >> 8) & 0xffff }
    pub const fn pad(self) -> u32 { (self.0 >> 24) & 0xff }
}

pub const CMD_INTRPT_GENERATE_IRQ: u32 = 0x01;
pub const CMD_INTRPT_GENERATE_ACK: u32 = 0x02;
pub const CMD_INTRPT_READ_STATUS: u32 = 0x03;
pub const CMD_INTRPT_CHECK_SOURCE: u32 = 0x04;

/* Semaphore Commands */
pub const CMD_SEMA_CLAIM_AND_READ: u32 = 0x11;
pub const CMD_SEMA_RELEASE: u32 = 0x12;

pub const CMD_DEBUG_SET_MASK: u32 = 0x34;
pub const CMD_DEBUG_READ_MASK: u32 = 0x35;
pub const CMD_DEBUG_SET_SELECT: u32 = 0x36;
pub const CMD_DEBUG_READ_SELECT: u32 = 0x37;

pub const CMD_GFRC_READ_LO: u32 = 0x42;
pub const CMD_GFRC_READ_HI: u32 = 0x43;
pub const CMD_GFRC_SET_CORE: u32 = 0x47;
pub const CMD_GFRC_READ_CORE: u32 = 0x48;

pub const CMD_IDU_ENABLE: u32 = 0x71;
pub const CMD_IDU_DISABLE: u32 = 0x72;
pub const CMD_IDU_SET_MODE: u32 = 0x74;
pub const CMD_IDU_READ_MODE: u32 = 0x75;
pub const CMD_IDU_SET_DEST: u32 = 0x76;
pub const CMD_IDU_ACK_CIRQ: u32 = 0x79;
pub const CMD_IDU_SET_MASK: u32 = 0x7C;

pub const IDU_M_TRIG_LEVEL: u32 = 0x0;
pub const IDU_M_TRIG_EDGE: u32 = 0x1;
pub const IDU_M_DISTRI_RR: u32 = 0x0;
pub const IDU_M_DISTRI_DEST: u32 = 0x2;

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
pub struct McipBcr(pub u32);

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
pub struct McipIduBcr(pub u32);

impl McipIduBcr {
    pub const fn cirqnum(self) -> u32 { (self.0 >> 8) & 0x7 }
}

/* The IDU build register stores an exponent; multiply by 4 for the IRQ count. */
pub const fn mcip_idu_bcr_to_nr_irqs(bcr: McipIduBcr) -> u32 {
    4 * (1u32 << bcr.cirqnum())
}

extern "C" {
    pub fn write_aux_reg(reg: u32, value: u32);
    pub fn read_aux_reg(reg: u32) -> u32;
}

pub unsafe fn __mcip_cmd(cmd: u32, param: u32) {
    let buf = McipCmd::new(cmd, param);
    write_aux_reg(ARC_REG_MCIP_CMD, buf.0);
}

pub unsafe fn __mcip_cmd_data(cmd: u32, param: u32, data: u32) {
    write_aux_reg(ARC_REG_MCIP_WDATA, data);
    __mcip_cmd(cmd, param);
}

pub unsafe fn __mcip_cmd_read(cmd: u32, param: u32) -> u32 {
    __mcip_cmd(cmd, param);
    read_aux_reg(ARC_REG_MCIP_READBACK)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
