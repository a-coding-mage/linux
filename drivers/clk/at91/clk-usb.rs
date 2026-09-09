// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Dependencies supplied by the Linux clock, PMC, device-tree, syscon, and regmap APIs.

const SAM9X5_USB_DIV_SHIFT: u32 = 8;
const SAM9X5_USB_MAX_DIV: u32 = 0xf;
const RM9200_USB_DIV_SHIFT: u32 = 28;
const RM9200_USB_DIV_TAB_SIZE: usize = 4;
const SAM9X5_USBS_MASK: u32 = 1;
const SAM9X60_USBS_MASK: u32 = 3;

#[repr(C)]
struct at91sam9x5_clk_usb {
    hw: clk_hw,
    regmap: *mut regmap,
    pms: at91_clk_pms,
    usbs_mask: u32,
    num_parents: u8,
}

#[repr(C)]
struct at91rm9200_clk_usb {
    hw: clk_hw,
    regmap: *mut regmap,
    divisors: [u32; 4],
}

unsafe fn to_at91sam9x5_clk_usb(hw: *mut clk_hw) -> *mut at91sam9x5_clk_usb {
    (hw as *mut u8).sub(core::mem::offset_of!(at91sam9x5_clk_usb, hw))
        as *mut at91sam9x5_clk_usb
}

unsafe fn to_at91rm9200_clk_usb(hw: *mut clk_hw) -> *mut at91rm9200_clk_usb {
    (hw as *mut u8).sub(core::mem::offset_of!(at91rm9200_clk_usb, hw))
        as *mut at91rm9200_clk_usb
}

unsafe extern "C" fn at91sam9x5_clk_usb_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let usb = to_at91sam9x5_clk_usb(hw);
    let mut usbr: c_uint = 0;
    regmap_read((*usb).regmap, AT91_PMC_USB, &mut usbr);
    let usbdiv = (usbr & AT91_PMC_OHCIUSBDIV) >> SAM9X5_USB_DIV_SHIFT;
    (parent_rate + ((usbdiv + 1) / 2)) / (usbdiv as c_ulong + 1)
}

unsafe extern "C" fn at91sam9x5_clk_usb_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let mut best_rate: c_long = -EINVAL as c_long;
    let mut best_diff: c_int = -1;
    let mut i = 0;
    while i < clk_hw_get_num_parents(hw) {
        let parent = clk_hw_get_parent_by_index(hw, i);
        if !parent.is_null() {
            let mut div = 1;
            while div < SAM9X5_USB_MAX_DIV + 2 {
                let mut tmp_parent_rate = (*req).rate * div as c_ulong;
                tmp_parent_rate = clk_hw_round_rate(parent, tmp_parent_rate);
                if tmp_parent_rate != 0 {
                    let tmp_rate = (tmp_parent_rate + div as c_ulong / 2) / div as c_ulong;
                    let tmp_diff = if tmp_rate < (*req).rate { (*req).rate - tmp_rate } else { tmp_rate - (*req).rate } as c_int;
                    if best_diff < 0 || best_diff > tmp_diff {
                        best_rate = tmp_rate as c_long;
                        best_diff = tmp_diff;
                        (*req).best_parent_rate = tmp_parent_rate;
                        (*req).best_parent_hw = parent;
                    }
                    if best_diff == 0 || tmp_rate < (*req).rate { break; }
                }
                div += 1;
            }
            if best_diff == 0 { break; }
        }
        i += 1;
    }
    if best_rate < 0 { return best_rate as c_int; }
    (*req).rate = best_rate as c_ulong;
    0
}

unsafe extern "C" fn at91sam9x5_clk_usb_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let usb = to_at91sam9x5_clk_usb(hw);
    if index >= (*usb).num_parents { return -EINVAL; }
    regmap_update_bits((*usb).regmap, AT91_PMC_USB, (*usb).usbs_mask, index as u32);
    0
}

unsafe extern "C" fn at91sam9x5_clk_usb_get_parent(hw: *mut clk_hw) -> u8 {
    let usb = to_at91sam9x5_clk_usb(hw);
    let mut usbr = 0;
    regmap_read((*usb).regmap, AT91_PMC_USB, &mut usbr);
    (usbr & (*usb).usbs_mask) as u8
}

unsafe extern "C" fn at91sam9x5_clk_usb_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let usb = to_at91sam9x5_clk_usb(hw);
    if rate == 0 { return -EINVAL; }
    let div = (parent_rate + rate / 2) / rate;
    if div > SAM9X5_USB_MAX_DIV as c_ulong + 1 || div == 0 { return -EINVAL; }
    regmap_update_bits((*usb).regmap, AT91_PMC_USB, AT91_PMC_OHCIUSBDIV, ((div - 1) as u32) << SAM9X5_USB_DIV_SHIFT);
    0
}

unsafe extern "C" fn at91sam9x5_usb_save_context(hw: *mut clk_hw) -> c_int {
    let usb = to_at91sam9x5_clk_usb(hw);
    let parent_hw = clk_hw_get_parent(hw);
    (*usb).pms.parent = at91sam9x5_clk_usb_get_parent(hw);
    (*usb).pms.parent_rate = clk_hw_get_rate(parent_hw);
    (*usb).pms.rate = at91sam9x5_clk_usb_recalc_rate(hw, (*usb).pms.parent_rate);
    0
}

unsafe extern "C" fn at91sam9x5_usb_restore_context(hw: *mut clk_hw) {
    let usb = to_at91sam9x5_clk_usb(hw);
    if at91sam9x5_clk_usb_set_parent(hw, (*usb).pms.parent) != 0 { return; }
    at91sam9x5_clk_usb_set_rate(hw, (*usb).pms.rate, (*usb).pms.parent_rate);
}

unsafe extern "C" fn at91sam9n12_clk_usb_enable(hw: *mut clk_hw) -> c_int { let usb = to_at91sam9x5_clk_usb(hw); regmap_update_bits((*usb).regmap, AT91_PMC_USB, AT91_PMC_USBS, AT91_PMC_USBS); 0 }
unsafe extern "C" fn at91sam9n12_clk_usb_disable(hw: *mut clk_hw) { let usb = to_at91sam9x5_clk_usb(hw); regmap_update_bits((*usb).regmap, AT91_PMC_USB, AT91_PMC_USBS, 0); }
unsafe extern "C" fn at91sam9n12_clk_usb_is_enabled(hw: *mut clk_hw) -> c_int { let usb = to_at91sam9x5_clk_usb(hw); let mut usbr=0; regmap_read((*usb).regmap, AT91_PMC_USB, &mut usbr); (usbr & AT91_PMC_USBS) as c_int }

unsafe extern "C" fn _at91sam9x5_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: u8, usbs_mask: u32) -> *mut clk_hw {
    let usb = kzalloc_obj::<at91sam9x5_clk_usb>(); if usb.is_null() { return ERR_PTR(-ENOMEM); }
    (*usb).regmap=regmap; (*usb).usbs_mask=usbs_mask; (*usb).num_parents=num_parents;
    let init = clk_init_data { name, ops: &at91sam9x5_usb_ops, parent_names, num_parents, flags: CLK_SET_RATE_GATE|CLK_SET_PARENT_GATE|CLK_SET_RATE_PARENT };
    (*usb).hw.init=&init; let hw=&mut (*usb).hw; let ret=clk_hw_register(core::ptr::null_mut(),hw); if ret!=0 { kfree(usb as *mut c_void); return ERR_PTR(ret); } hw
}

#[no_mangle]
pub unsafe extern "C" fn at91sam9x5_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: u8) -> *mut clk_hw {
    _at91sam9x5_clk_register_usb(regmap, name, parent_names, num_parents, SAM9X5_USBS_MASK)
}

#[no_mangle]
pub unsafe extern "C" fn sam9x60_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: u8) -> *mut clk_hw {
    _at91sam9x5_clk_register_usb(regmap, name, parent_names, num_parents, SAM9X60_USBS_MASK)
}

#[no_mangle]
pub unsafe extern "C" fn at91sam9n12_clk_register_usb(regmap:*mut regmap,name:*const c_char,parent_name:*const c_char)->*mut clk_hw { let usb=kzalloc_obj::<at91sam9x5_clk_usb>(); if usb.is_null(){return ERR_PTR(-ENOMEM)} (*usb).regmap=regmap; let init=clk_init_data{name,ops:&at91sam9n12_usb_ops,parent_names:&parent_name,num_parents:1,flags:CLK_SET_RATE_GATE|CLK_SET_RATE_PARENT}; (*usb).hw.init=&init; let hw=&mut (*usb).hw; let ret=clk_hw_register(core::ptr::null_mut(),hw); if ret!=0{kfree(usb as *mut c_void);return ERR_PTR(ret)} hw }

unsafe extern "C" fn at91rm9200_clk_usb_recalc_rate(hw:*mut clk_hw,parent_rate:c_ulong)->c_ulong { let usb=to_at91rm9200_clk_usb(hw); let mut pllbr=0; regmap_read((*usb).regmap,AT91_CKGR_PLLBR,&mut pllbr); let i=((pllbr&AT91_PMC_USBDIV)>>RM9200_USB_DIV_SHIFT) as usize; if (*usb).divisors[i]!=0 { parent_rate/(*usb).divisors[i] } else { 0 } }
unsafe extern "C" fn at91rm9200_clk_usb_set_rate(hw:*mut clk_hw,rate:c_ulong,parent_rate:c_ulong)->c_int { let usb=to_at91rm9200_clk_usb(hw); if rate==0{return -EINVAL}; let div=(parent_rate+rate/2)/rate; for i in 0..4 { if (*usb).divisors[i]==div { regmap_update_bits((*usb).regmap,AT91_CKGR_PLLBR,AT91_PMC_USBDIV,(i as u32)<<RM9200_USB_DIV_SHIFT); return 0; }} -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn at91rm9200_clk_register_usb(regmap:*mut regmap,name:*const c_char,parent_name:*const c_char,divisors:*const u32)->*mut clk_hw { let usb=kzalloc_obj::<at91rm9200_clk_usb>(); if usb.is_null(){return ERR_PTR(-ENOMEM)} (*usb).regmap=regmap; core::ptr::copy_nonoverlapping(divisors,(*usb).divisors.as_mut_ptr(),4); let init=clk_init_data{name,ops:&at91rm9200_usb_ops,parent_names:&parent_name,num_parents:1,flags:CLK_SET_RATE_PARENT}; (*usb).hw.init=&init; let hw=&mut (*usb).hw; let ret=clk_hw_register(core::ptr::null_mut(),hw); if ret!=0{kfree(usb as *mut c_void);return ERR_PTR(ret)} hw }

// External kernel types, constants, and functions are supplied by the translated dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
