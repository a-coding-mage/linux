// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const PROG_ID_MAX: u8 = 7;

#[inline]
fn prog_status_mask(id: u32) -> u32 {
    1u32 << (id + 8)
}

#[inline]
unsafe fn prog_pres(layout: *const clk_programmable_layout, pckr: u32) -> u32 {
    (pckr >> (*layout).pres_shift) & (*layout).pres_mask
}

const PROG_MAX_RM9200_CSS: u8 = 3;

#[repr(C)]
pub struct clk_programmable {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub mux_table: *mut u32,
    pub id: u8,
    pub layout: *const clk_programmable_layout,
    pub pms: at91_clk_pms,
}

#[inline]
unsafe fn to_clk_programmable(hw: *mut clk_hw) -> *mut clk_programmable {
    (hw as *mut u8).sub(offset_of!(clk_programmable, hw)) as *mut clk_programmable
}

unsafe fn clk_programmable_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let prog = to_clk_programmable(hw);
    let layout = (*prog).layout;
    let mut pckr: c_uint = 0;
    let rate: c_ulong;

    regmap_read((*prog).regmap, at91_pmc_pckr((*prog).id), &mut pckr);

    if (*layout).is_pres_direct {
        rate = parent_rate / (prog_pres(layout, pckr) as c_ulong + 1);
    } else {
        rate = parent_rate >> prog_pres(layout, pckr);
    }

    rate
}

unsafe fn clk_programmable_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let prog = to_clk_programmable(hw);
    let layout = (*prog).layout;
    let mut parent: *mut clk_hw;
    let mut best_rate: c_long = -EINVAL as c_long;
    let mut parent_rate: c_ulong;
    let mut tmp_rate: c_ulong = 0;
    let mut shift: c_int;
    let mut i: c_int = 0;

    while i < clk_hw_get_num_parents(hw) as c_int {
        parent = clk_hw_get_parent_by_index(hw, i as u8);
        if parent.is_null() {
            i += 1;
            continue;
        }

        parent_rate = clk_hw_get_rate(parent);
        if (*layout).is_pres_direct {
            shift = 0;
            while shift <= (*layout).pres_mask as c_int {
                tmp_rate = parent_rate / (shift as c_ulong + 1);
                if tmp_rate <= (*req).rate {
                    break;
                }
                shift += 1;
            }
        } else {
            shift = 0;
            while shift < (*layout).pres_mask as c_int {
                tmp_rate = parent_rate >> shift;
                if tmp_rate <= (*req).rate {
                    break;
                }
                shift += 1;
            }
        }

        if tmp_rate > (*req).rate {
            i += 1;
            continue;
        }

        if best_rate < 0
            || ((*req).rate - tmp_rate) < ((*req).rate - best_rate as c_ulong)
        {
            best_rate = tmp_rate as c_long;
            (*req).best_parent_rate = parent_rate;
            (*req).best_parent_hw = parent;
        }

        if best_rate == 0 {
            break;
        }
        i += 1;
    }

    if best_rate < 0 {
        return best_rate as c_int;
    }

    (*req).rate = best_rate as c_ulong;
    0
}

unsafe fn clk_programmable_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let prog = to_clk_programmable(hw);
    let layout = (*prog).layout;
    let mut mask = (*layout).css_mask;
    let mut pckr = index as c_uint;

    if (*layout).have_slck_mck {
        mask |= AT91_PMC_CSSMCK_MCK;
    }
    if !(*prog).mux_table.is_null() {
        pckr = clk_mux_index_to_val((*prog).mux_table, 0, index);
    }
    if index as c_uint > (*layout).css_mask {
        if index > PROG_MAX_RM9200_CSS && !(*layout).have_slck_mck {
            return -EINVAL;
        }
        pckr |= AT91_PMC_CSSMCK_MCK;
    }
    regmap_update_bits((*prog).regmap, at91_pmc_pckr((*prog).id), mask, pckr);
    0
}

unsafe fn clk_programmable_get_parent(hw: *mut clk_hw) -> u8 {
    let prog = to_clk_programmable(hw);
    let layout = (*prog).layout;
    let mut pckr: c_uint = 0;
    regmap_read((*prog).regmap, at91_pmc_pckr((*prog).id), &mut pckr);
    let mut ret = (pckr & (*layout).css_mask) as u8;
    if (*layout).have_slck_mck && (pckr & AT91_PMC_CSSMCK_MCK) != 0 && ret == 0 {
        ret = PROG_MAX_RM9200_CSS + 1;
    }
    if !(*prog).mux_table.is_null() {
        ret = clk_mux_val_to_index(&mut (*prog).hw, (*prog).mux_table, 0, ret);
    }
    ret
}

unsafe fn clk_programmable_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let prog = to_clk_programmable(hw);
    let layout = (*prog).layout;
    let div = parent_rate / rate;
    let shift: c_int;
    if div == 0 { return -EINVAL; }
    if (*layout).is_pres_direct {
        shift = div as c_int - 1;
        if shift > (*layout).pres_mask as c_int { return -EINVAL; }
    } else {
        shift = fls(div as c_int) - 1;
        if div != (1u64 << shift) as c_ulong || shift >= (*layout).pres_mask as c_int { return -EINVAL; }
    }
    regmap_update_bits((*prog).regmap, at91_pmc_pckr((*prog).id),
        (*layout).pres_mask << (*layout).pres_shift,
        (shift as c_uint) << (*layout).pres_shift);
    0
}

unsafe fn clk_programmable_save_context(hw: *mut clk_hw) -> c_int {
    let prog = to_clk_programmable(hw);
    let parent_hw = clk_hw_get_parent(hw);
    (*prog).pms.parent = clk_programmable_get_parent(hw);
    (*prog).pms.parent_rate = clk_hw_get_rate(parent_hw);
    (*prog).pms.rate = clk_programmable_recalc_rate(hw, (*prog).pms.parent_rate);
    0
}

unsafe fn clk_programmable_restore_context(hw: *mut clk_hw) {
    let prog = to_clk_programmable(hw);
    let ret = clk_programmable_set_parent(hw, (*prog).pms.parent);
    if ret != 0 { return; }
    clk_programmable_set_rate(hw, (*prog).pms.rate, (*prog).pms.parent_rate);
}

pub unsafe fn at91_clk_register_programmable(
    regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char,
    parent_hws: *mut *mut clk_hw, num_parents: u8, id: u8,
    layout: *const clk_programmable_layout, mux_table: *mut u32,
) -> *mut clk_hw {
    if id > PROG_ID_MAX || (parent_names.is_null() && parent_hws.is_null()) { return ERR_PTR(-EINVAL); }
    let prog = kzalloc_obj::<clk_programmable>();
    if prog.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &programmable_ops;
    if !parent_hws.is_null() { init.parent_hws = parent_hws as *const *const clk_hw; }
    else { init.parent_names = parent_names; }
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE;
    (*prog).id = id; (*prog).layout = layout; (*prog).hw.init = &init;
    (*prog).regmap = regmap; (*prog).mux_table = mux_table;
    let hw = &mut (*prog).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree(prog as *mut core::ffi::c_void); return ERR_PTR(ret); }
    hw
}

pub static at91rm9200_programmable_layout: clk_programmable_layout = clk_programmable_layout { pres_mask: 0x7, pres_shift: 2, css_mask: 0x3, have_slck_mck: false, is_pres_direct: false };
pub static at91sam9g45_programmable_layout: clk_programmable_layout = clk_programmable_layout { pres_mask: 0x7, pres_shift: 2, css_mask: 0x3, have_slck_mck: true, is_pres_direct: false };
pub static at91sam9x5_programmable_layout: clk_programmable_layout = clk_programmable_layout { pres_mask: 0x7, pres_shift: 4, css_mask: 0x7, have_slck_mck: false, is_pres_direct: false };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
