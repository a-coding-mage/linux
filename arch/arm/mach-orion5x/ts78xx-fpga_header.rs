/* SPDX-License-Identifier: GPL-2.0 */

pub const TS7800_FPGA_MAGIC: u32 = 0x00b480;

#[inline]
pub const fn fpgaid(magic: u32, rev: u32) -> u32 {
    magic.wrapping_shl(8).wrapping_add(rev)
}

/*
 * get yer id's from http://ts78xx.digriz.org.uk/
 * do *not* make up your own or 'borrow' any!
 */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FpgaIds {
    /* Technologic Systems */
    TS7800_REV_1 = fpgaid(TS7800_FPGA_MAGIC, 0x01),
    TS7800_REV_2 = fpgaid(TS7800_FPGA_MAGIC, 0x02),
    TS7800_REV_3 = fpgaid(TS7800_FPGA_MAGIC, 0x03),
    TS7800_REV_4 = fpgaid(TS7800_FPGA_MAGIC, 0x04),
    TS7800_REV_5 = fpgaid(TS7800_FPGA_MAGIC, 0x05),
    TS7800_REV_6 = fpgaid(TS7800_FPGA_MAGIC, 0x06),
    TS7800_REV_7 = fpgaid(TS7800_FPGA_MAGIC, 0x07),
    TS7800_REV_8 = fpgaid(TS7800_FPGA_MAGIC, 0x08),
    TS7800_REV_9 = fpgaid(TS7800_FPGA_MAGIC, 0x09),

    /* Unaffordable & Expensive */
    UAE_DUMMY = fpgaid(0xffffff, 0x01),
}

#[repr(C)]
pub struct fpga_device {
    // C bit-fields: present:1, init:1. The low two bits hold these values.
    pub present: u32,
    pub init: u32,
}

#[repr(C)]
pub struct fpga_devices {
    /* Technologic Systems */
    pub ts_rtc: fpga_device,
    pub ts_nand: fpga_device,
    pub ts_rng: fpga_device,
}

#[repr(C)]
pub struct ts78xx_fpga_data {
    pub id: u32,
    pub state: i32,
    pub supports: fpga_devices,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
