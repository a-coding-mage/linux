/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016-2017, The Linux Foundation. All rights reserved.
 */

// Dependency intent: <asm/byteorder.h> supplies cpu_to_le32.

/*
 * This data type corresponds to the native Command Element
 * supported by BAM DMA Engine.
 *
 * @cmd_and_addr - upper 8 bits command and lower 24 bits register address.
 * @data - For write command: content to be written into peripheral register.
 *         For read command: lower 32 bits of destination address.
 * @mask - For write command: register write mask.
 *         For read command on BAM v1.6.0+: upper 4 bits of destination address.
 *         For read command on BAM < v1.6.0: ignored by hardware.
 *         Setting to 0 ensures 32-bit addressing compatibility.
 * @reserved - for future usage.
 */
#[repr(C)]
pub struct bam_cmd_element {
    pub cmd_and_addr: u32,
    pub data: u32,
    pub mask: u32,
    pub reserved: u32,
}

/*
 * This enum indicates the command type in a command element
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bam_command_type {
    BAM_WRITE_COMMAND = 0,
    BAM_READ_COMMAND,
}

/*
 * prep_bam_ce_le32 - Wrapper function to prepare a single BAM command
 * element with the data already in le32 format.
 *
 * @bam_ce: bam command element
 * @addr: target address
 * @cmd: BAM command
 * @data: actual data for write and dest addr for read in le32
 *
 * For BAM v1.6.0+, the mask field behavior depends on command type:
 * - Write commands: mask = write mask (typically 0xffffffff)
 * - Read commands: mask = upper 4 bits of destination address (0 for 32-bit)
 */
pub unsafe fn bam_prep_ce_le32(
    bam_ce: *mut bam_cmd_element,
    addr: u32,
    cmd: bam_command_type,
    data: u32,
) {
    (*bam_ce).cmd_and_addr = cpu_to_le32(
        (addr & 0x00ff_ffff) | (((cmd as u32) & 0xff) << 24),
    );
    (*bam_ce).data = data;
    if cmd == bam_command_type::BAM_READ_COMMAND {
        (*bam_ce).mask = cpu_to_le32(0x0); /* 32-bit addressing */
    } else {
        (*bam_ce).mask = cpu_to_le32(0xffff_ffff); /* Write mask */
    }
    (*bam_ce).reserved = 0;
}

/*
 * bam_prep_ce - Wrapper function to prepare a single BAM command element
 * with the data.
 *
 * @bam_ce: BAM command element
 * @addr: target address
 * @cmd: BAM command
 * @data: actual data for write and destination address for read
 */
pub unsafe fn bam_prep_ce(
    bam_ce: *mut bam_cmd_element,
    addr: u32,
    cmd: bam_command_type,
    data: u32,
) {
    bam_prep_ce_le32(bam_ce, addr, cmd, cpu_to_le32(data));
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
