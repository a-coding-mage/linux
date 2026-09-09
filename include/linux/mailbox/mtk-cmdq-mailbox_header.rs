/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 MediaTek Inc.
 *
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external here: platform_device.h, slab.h, and types.h.

pub const CMDQ_INST_SIZE: u32 = 8; // instruction is 64-bit
pub const CMDQ_SUBSYS_SHIFT: u32 = 16;
pub const CMDQ_OP_CODE_SHIFT: u32 = 24;
pub const CMDQ_JUMP_PASS: u32 = CMDQ_INST_SIZE;

pub const CMDQ_WFE_UPDATE: u32 = 1u32 << 31;
pub const CMDQ_WFE_UPDATE_VALUE: u32 = 1u32 << 16;
pub const CMDQ_WFE_WAIT: u32 = 1u32 << 15;
pub const CMDQ_WFE_WAIT_VALUE: u32 = 0x1;

/*
 * WFE arg_b
 * bit 0-11: wait value
 * bit 15: 1 - wait, 0 - no wait
 * bit 16-27: update value
 * bit 31: 1 - update, 0 - no update
 */
pub const CMDQ_WFE_OPTION: u32 = CMDQ_WFE_WAIT | CMDQ_WFE_WAIT_VALUE;

/** cmdq event maximum */
pub const CMDQ_MAX_EVENT: u32 = 0x3ff;

/*
 * CMDQ_CODE_MASK:
 *   set write mask
 *   format: op mask
 * CMDQ_CODE_WRITE:
 *   write value into target register
 *   format: op subsys address value
 * CMDQ_CODE_JUMP:
 *   jump by offset
 *   format: op offset
 * CMDQ_CODE_WFE:
 *   wait for event and clear
 *   it is just clear if no wait
 *   format: [wait]  op event update:1 to_wait:1 wait:1
 *           [clear] op event update:1 to_wait:0 wait:0
 * CMDQ_CODE_EOC:
 *   end of command
 *   format: op irq_flag
 */
#[repr(u32)]
pub enum CmdqCode {
    CMDQ_CODE_MASK = 0x02,
    CMDQ_CODE_WRITE = 0x04,
    CMDQ_CODE_POLL = 0x08,
    CMDQ_CODE_JUMP = 0x10,
    CMDQ_CODE_WFE = 0x20,
    CMDQ_CODE_EOC = 0x40,
    CMDQ_CODE_READ_S = 0x80,
    CMDQ_CODE_WRITE_S = 0x90,
    CMDQ_CODE_WRITE_S_MASK = 0x91,
    CMDQ_CODE_LOGIC = 0xa0,
}

#[repr(C)]
pub struct CmdqCbData {
    pub sta: i32,
    pub pkt: *mut CmdqPkt,
}

#[repr(C)]
pub struct CmdqMboxPriv {
    pub shift_pa: u8,
    pub mminfra_offset: dma_addr_t,
}

#[repr(C)]
pub struct CmdqPkt {
    pub va_base: *mut core::ffi::c_void,
    pub pa_base: dma_addr_t,
    pub cmd_buf_size: usize, // command occupied size
    pub buf_size: usize, // real buffer size
    pub priv_: CmdqMboxPriv, // for generating instruction
}

/**
 * cmdq_get_mbox_priv() - get the private data of mailbox channel
 * @chan: mailbox channel
 * @priv: pointer to store the private data of mailbox channel
 *
 * While generating the GCE instruction to command buffer, the private data
 * of GCE hardware may need to be referenced, such as the shift bits of
 * physical address.
 *
 * This function should be called before generating the GCE instruction.
 */
unsafe extern "C" {
    pub fn cmdq_get_mbox_priv(chan: *mut mbox_chan, priv_: *mut CmdqMboxPriv);

    /**
     * cmdq_get_shift_pa() - get the shift bits of physical address
     * @chan: mailbox channel
     *
     * GCE can only fetch the command buffer address from a 32-bit register.
     * Some SOCs support more than 32-bit command buffer address for GCE, which
     * requires some shift bits to make the address fit into the 32-bit register.
     *
     * Return: the shift bits of physical address
     */
    pub fn cmdq_get_shift_pa(chan: *mut mbox_chan) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
