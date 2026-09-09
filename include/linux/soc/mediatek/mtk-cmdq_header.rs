/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 MediaTek Inc.
 */

// Translated from mtk-cmdq.h. Linux-provided types and symbols are external dependencies.

pub const CMDQ_ADDR_HIGH: fn(u64) -> u32 = |addr| ((addr >> 16) & u32::MAX as u64) as u32;
pub const CMDQ_ADDR_LOW: fn(u64) -> u16 = |addr| (addr as u16) | (1u16 << 1);

pub const CMDQ_THR_SPR_IDX0: u32 = 0;
pub const CMDQ_THR_SPR_IDX1: u32 = 1;
pub const CMDQ_THR_SPR_IDX2: u32 = 2;
pub const CMDQ_THR_SPR_IDX3: u32 = 3;
pub const CMDQ_SUBSYS_INVALID: u8 = u8::MAX;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum cmdq_logic_op {
    CMDQ_LOGIC_ASSIGN = 0,
    CMDQ_LOGIC_ADD = 1,
    CMDQ_LOGIC_SUBTRACT = 2,
    CMDQ_LOGIC_MULTIPLY = 3,
    CMDQ_LOGIC_XOR = 8,
    CMDQ_LOGIC_NOT = 9,
    CMDQ_LOGIC_OR = 10,
    CMDQ_LOGIC_AND = 11,
    CMDQ_LOGIC_LEFT_SHIFT = 12,
    CMDQ_LOGIC_RIGHT_SHIFT = 13,
    CMDQ_LOGIC_MAX,
}

#[repr(C)]
pub union cmdq_operand_value { pub idx: u16, pub value: u16 }

#[repr(C)]
pub struct cmdq_operand { pub reg: bool, pub value: cmdq_operand_value }

#[repr(C)]
pub struct cmdq_client_reg {
    pub subsys: u8,
    pub pa_base: phys_addr_t,
    pub offset: u16,
    pub size: u16,
    pub pkt_write: Option<unsafe extern "C" fn(*mut cmdq_pkt, u8, u32, u16, u32) -> i32>,
    pub pkt_write_mask: Option<unsafe extern "C" fn(*mut cmdq_pkt, u8, u32, u16, u32, u32) -> i32>,
}

#[repr(C)]
pub struct cmdq_client { pub client: mbox_client, pub chan: *mut mbox_chan }

#[repr(C)]
pub struct cmdq_pkt { _private: [u8; 0] }

#[cfg(feature = "CONFIG_MTK_CMDQ")]
extern "C" {
    pub fn cmdq_dev_get_client_reg(dev: *mut device, client_reg: *mut cmdq_client_reg, idx: i32) -> i32;
    pub fn cmdq_mbox_create(dev: *mut device, index: i32) -> *mut cmdq_client;
    pub fn cmdq_mbox_destroy(client: *mut cmdq_client);
    pub fn cmdq_pkt_create(client: *mut cmdq_client, pkt: *mut cmdq_pkt, size: usize) -> i32;
    pub fn cmdq_pkt_destroy(client: *mut cmdq_client, pkt: *mut cmdq_pkt);
    pub fn cmdq_pkt_write(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32) -> i32;
    pub fn cmdq_pkt_write_pa(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32) -> i32;
    pub fn cmdq_pkt_write_subsys(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32) -> i32;
    pub fn cmdq_pkt_write_mask(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32, mask: u32) -> i32;
    pub fn cmdq_pkt_write_mask_pa(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32, mask: u32) -> i32;
    pub fn cmdq_pkt_write_mask_subsys(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32, mask: u32) -> i32;
    pub fn cmdq_pkt_read_s(pkt: *mut cmdq_pkt, high_addr_reg_idx: u16, addr_low: u16, reg_idx: u16) -> i32;
    pub fn cmdq_pkt_write_s(pkt: *mut cmdq_pkt, high_addr_reg_idx: u16, addr_low: u16, src_reg_idx: u16) -> i32;
    pub fn cmdq_pkt_write_s_mask(pkt: *mut cmdq_pkt, high_addr_reg_idx: u16, addr_low: u16, src_reg_idx: u16, mask: u32) -> i32;
    pub fn cmdq_pkt_write_s_value(pkt: *mut cmdq_pkt, high_addr_reg_idx: u8, addr_low: u16, value: u32) -> i32;
    pub fn cmdq_pkt_write_s_mask_value(pkt: *mut cmdq_pkt, high_addr_reg_idx: u8, addr_low: u16, value: u32, mask: u32) -> i32;
    pub fn cmdq_pkt_mem_move(pkt: *mut cmdq_pkt, src_addr: dma_addr_t, dst_addr: dma_addr_t) -> i32;
    pub fn cmdq_pkt_wfe(pkt: *mut cmdq_pkt, event: u16, clear: bool) -> i32;
    pub fn cmdq_pkt_acquire_event(pkt: *mut cmdq_pkt, event: u16) -> i32;
    pub fn cmdq_pkt_clear_event(pkt: *mut cmdq_pkt, event: u16) -> i32;
    pub fn cmdq_pkt_set_event(pkt: *mut cmdq_pkt, event: u16) -> i32;
    pub fn cmdq_pkt_poll(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32) -> i32;
    pub fn cmdq_pkt_poll_mask(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32, mask: u32) -> i32;
    pub fn cmdq_pkt_logic_command(pkt: *mut cmdq_pkt, result_reg_idx: u16, left_operand: *mut cmdq_operand, s_op: cmdq_logic_op, right_operand: *mut cmdq_operand) -> i32;
    pub fn cmdq_pkt_assign(pkt: *mut cmdq_pkt, reg_idx: u16, value: u32) -> i32;
    pub fn cmdq_pkt_poll_addr(pkt: *mut cmdq_pkt, addr: dma_addr_t, value: u32, mask: u32) -> i32;
    pub fn cmdq_pkt_jump_abs(pkt: *mut cmdq_pkt, addr: dma_addr_t, shift_pa: u8) -> i32;
    pub fn cmdq_pkt_jump_rel(pkt: *mut cmdq_pkt, offset: i32, shift_pa: u8) -> i32;
    pub fn cmdq_pkt_eoc(pkt: *mut cmdq_pkt) -> i32;
}

#[cfg(feature = "CONFIG_MTK_CMDQ")]
#[inline]
pub unsafe fn cmdq_pkt_jump(pkt: *mut cmdq_pkt, addr: dma_addr_t, shift_pa: u8) -> i32 { cmdq_pkt_jump_abs(pkt, addr, shift_pa) }

#[cfg(feature = "CONFIG_MTK_CMDQ")]
#[inline]
pub unsafe fn cmdq_pkt_jump_rel_temp(pkt: *mut cmdq_pkt, offset: i32, shift_pa: u8) -> i32 { cmdq_pkt_jump_rel(pkt, offset, shift_pa) }

// When CONFIG_MTK_CMDQ is disabled, the C header supplies inline stubs returning -ENODEV,
// -EINVAL, or -ENOENT. Their exact errno values are provided by the dependent Linux bindings.
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline]
pub unsafe fn cmdq_dev_get_client_reg(_: *mut device, _: *mut cmdq_client_reg, _: i32) -> i32 { -19 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline]
pub unsafe fn cmdq_mbox_create(_: *mut device, _: i32) -> *mut cmdq_client { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline]
pub unsafe fn cmdq_mbox_destroy(_: *mut cmdq_client) {}
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline]
pub unsafe fn cmdq_pkt_create(_: *mut cmdq_client, _: *mut cmdq_pkt, _: usize) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline]
pub unsafe fn cmdq_pkt_destroy(_: *mut cmdq_client, _: *mut cmdq_pkt) {}

#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
macro_rules! cmdq_noent { ($($name:ident ($($arg:ident : $ty:ty),*)),* $(,)?) => { $(
    #[inline] pub unsafe fn $name($($arg: $ty),*) -> i32 { -2 }
)* } }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
cmdq_noent! {
    cmdq_pkt_write(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32),
    cmdq_pkt_write_pa(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32),
    cmdq_pkt_write_subsys(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32),
    cmdq_pkt_write_mask(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32, mask: u32),
    cmdq_pkt_write_mask_pa(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32, mask: u32),
    cmdq_pkt_write_mask_subsys(pkt: *mut cmdq_pkt, subsys: u8, pa_base: u32, offset: u16, value: u32, mask: u32),
    cmdq_pkt_read_s(pkt: *mut cmdq_pkt, high_addr_reg_idx: u16, addr_low: u16, reg_idx: u16),
    cmdq_pkt_write_s(pkt: *mut cmdq_pkt, high_addr_reg_idx: u16, addr_low: u16, src_reg_idx: u16),
    cmdq_pkt_write_s_mask(pkt: *mut cmdq_pkt, high_addr_reg_idx: u16, addr_low: u16, src_reg_idx: u16, mask: u32),
    cmdq_pkt_write_s_value(pkt: *mut cmdq_pkt, high_addr_reg_idx: u8, addr_low: u16, value: u32),
    cmdq_pkt_write_s_mask_value(pkt: *mut cmdq_pkt, high_addr_reg_idx: u8, addr_low: u16, value: u32, mask: u32),
}

#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_wfe(_: *mut cmdq_pkt, _: u16, _: bool) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_clear_event(_: *mut cmdq_pkt, _: u16) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_set_event(_: *mut cmdq_pkt, _: u16) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_poll(_: *mut cmdq_pkt, _: u8, _: u16, _: u32) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_poll_mask(_: *mut cmdq_pkt, _: u8, _: u16, _: u32, _: u32) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_assign(_: *mut cmdq_pkt, _: u16, _: u32) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_poll_addr(_: *mut cmdq_pkt, _: dma_addr_t, _: u32, _: u32) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_jump_abs(_: *mut cmdq_pkt, _: dma_addr_t, _: u8) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_jump(_: *mut cmdq_pkt, _: dma_addr_t, _: u8) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_jump_rel(_: *mut cmdq_pkt, _: i32, _: u8) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_jump_rel_temp(_: *mut cmdq_pkt, _: i32, _: u8) -> i32 { -22 }
#[cfg(not(feature = "CONFIG_MTK_CMDQ"))]
#[inline] pub unsafe fn cmdq_pkt_eoc(_: *mut cmdq_pkt) -> i32 { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
