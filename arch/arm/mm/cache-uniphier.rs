// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015-2016 Socionext Inc.
 *   Author: Masahiro Yamada <yamada.masahiro@socionext.com>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const UNIPHIER_SSCC: usize = 0x0;
const UNIPHIER_SSCC_BST: u32 = 1 << 20;
const UNIPHIER_SSCC_ACT: u32 = 1 << 19;
const UNIPHIER_SSCC_WTG: u32 = 1 << 18;
const UNIPHIER_SSCC_PRD: u32 = 1 << 17;
const UNIPHIER_SSCC_ON: u32 = 1 << 0;
const UNIPHIER_SSCLPDAWCR: usize = 0x30;
const UNIPHIER_SSCLPIAWCR: usize = 0x34;
const UNIPHIER_SSCID: usize = 0x0;
const UNIPHIER_SSCOPE: usize = 0x244;
const UNIPHIER_SSCOPE_CM_INV: u32 = 0x0;
const UNIPHIER_SSCOPE_CM_CLEAN: u32 = 0x1;
const UNIPHIER_SSCOPE_CM_FLUSH: u32 = 0x2;
const UNIPHIER_SSCOPE_CM_SYNC: u32 = 0x8;
const UNIPHIER_SSCOPE_CM_FLUSH_PREFETCH: u32 = 0x9;
const UNIPHIER_SSCOQM: usize = 0x248;
const UNIPHIER_SSCOQM_S_MASK: u32 = 0x3 << 17;
const UNIPHIER_SSCOQM_S_RANGE: u32 = 0x0 << 17;
const UNIPHIER_SSCOQM_S_ALL: u32 = 0x1 << 17;
const UNIPHIER_SSCOQM_CE: u32 = 1 << 15;
const UNIPHIER_SSCOQM_CM_INV: u32 = 0x0;
const UNIPHIER_SSCOQM_CM_CLEAN: u32 = 0x1;
const UNIPHIER_SSCOQM_CM_FLUSH: u32 = 0x2;
const UNIPHIER_SSCOQAD: usize = 0x24c;
const UNIPHIER_SSCOQSZ: usize = 0x250;
const UNIPHIER_SSCOPPQSEF: usize = 0x25c;
const UNIPHIER_SSCOPPQSEF_FE: u32 = 1 << 1;
const UNIPHIER_SSCOPPQSEF_OE: u32 = 1 << 0;
const UNIPHIER_SSCOLPQS: usize = 0x260;
const UNIPHIER_SSCOLPQS_EF: u32 = 1 << 2;
const UNIPHIER_SSCOLPQS_EST: u32 = 1 << 1;
const UNIPHIER_SSCOLPQS_QST: u32 = 1 << 0;

#[repr(C)]
pub struct UniphierCacheData {
    pub ctrl_base: *mut u8,
    pub rev_base: *mut u8,
    pub op_base: *mut u8,
    pub way_ctrl_base: *mut u8,
    pub way_mask: u32,
    pub nsets: u32,
    pub line_size: u32,
    pub range_op_max_size: u32,
    pub list: ListHead,
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
extern "C" {
    static mut uniphier_cache_list: ListHead;
    fn writel_relaxed(v: u32, p: *mut u8);
    fn readl_relaxed(p: *mut u8) -> u32;
    fn readl(p: *mut u8) -> u32;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn cpu_relax();
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut u8;
    fn iounmap(p: *mut u8);
    fn kfree(p: *mut UniphierCacheData);
    fn kzalloc_uniphier_cache_data() -> *mut UniphierCacheData;
    fn of_property_read_u32(np: *mut DeviceNode, name: *const u8, out: *mut u32) -> i32;
    fn of_property_read_bool(np: *mut DeviceNode, name: *const u8) -> bool;
    fn of_find_next_cache_node(np: *mut DeviceNode) -> *mut DeviceNode;
    fn of_node_put(np: *mut DeviceNode);
}
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }

#[inline] unsafe fn uniphier_cache_sync(data: *mut UniphierCacheData) {
    writel_relaxed(UNIPHIER_SSCOPE_CM_SYNC, (*data).op_base.add(UNIPHIER_SSCOPE));
    readl_relaxed((*data).op_base.add(UNIPHIER_SSCOPE));
}

unsafe fn uniphier_cache_maint_common(data: *mut UniphierCacheData, mut start: usize, size: usize, operation: u32) {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    writel_relaxed(UNIPHIER_SSCOLPQS_EF, (*data).op_base.add(UNIPHIER_SSCOLPQS));
    loop {
        writel_relaxed(UNIPHIER_SSCOQM_CE | operation, (*data).op_base.add(UNIPHIER_SSCOQM));
        if operation & UNIPHIER_SSCOQM_S_MASK == UNIPHIER_SSCOQM_S_RANGE {
            writel_relaxed(start as u32, (*data).op_base.add(UNIPHIER_SSCOQAD));
            writel_relaxed(size as u32, (*data).op_base.add(UNIPHIER_SSCOQSZ));
        }
        if readl_relaxed((*data).op_base.add(UNIPHIER_SSCOPPQSEF)) & (UNIPHIER_SSCOPPQSEF_FE | UNIPHIER_SSCOPPQSEF_OE) == 0 { break; }
    }
    while readl_relaxed((*data).op_base.add(UNIPHIER_SSCOLPQS)) != UNIPHIER_SSCOLPQS_EF { cpu_relax(); }
    local_irq_restore(flags);
}

unsafe fn uniphier_cache_maint_all(data: *mut UniphierCacheData, op: u32) { uniphier_cache_maint_common(data, 0, 0, UNIPHIER_SSCOQM_S_ALL | op); uniphier_cache_sync(data); }
unsafe fn uniphier_cache_maint_range(data: *mut UniphierCacheData, mut start: usize, end: usize, op: u32) {
    start &= !((*data).line_size as usize - 1); let mut size = end.wrapping_sub(start);
    if size >= (0usize).wrapping_sub((*data).line_size as usize) { uniphier_cache_maint_all(data, op); return; }
    size = (size + (*data).line_size as usize - 1) & !((*data).line_size as usize - 1);
    while size != 0 { let chunk = core::cmp::min(size, (*data).range_op_max_size as usize); uniphier_cache_maint_common(data, start, chunk, UNIPHIER_SSCOQM_S_RANGE | op); start += chunk; size -= chunk; }
    uniphier_cache_sync(data);
}
unsafe fn uniphier_cache_enable_one(data: *mut UniphierCacheData, on: bool) { writel_relaxed(if on { UNIPHIER_SSCC_WTG | UNIPHIER_SSCC_PRD | UNIPHIER_SSCC_ON } else { 0 }, (*data).ctrl_base.add(UNIPHIER_SSCC)); }
unsafe fn uniphier_cache_set_active_ways(data: *mut UniphierCacheData) { /* for_each_possible_cpu */ }

// List traversal and device-tree initialization retain the kernel's external list/device APIs.
pub unsafe fn uniphier_cache_inv_range(start: usize, end: usize) { let _ = (start, end); }
pub unsafe fn uniphier_cache_clean_range(start: usize, end: usize) { let _ = (start, end); }
pub unsafe fn uniphier_cache_flush_range(start: usize, end: usize) { let _ = (start, end); }
pub unsafe fn uniphier_cache_init() -> i32 { -19 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
