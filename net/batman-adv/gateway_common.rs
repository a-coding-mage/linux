// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner
 */

use core::ffi::c_void;
use core::mem::size_of;

// Declarations supplied by the surrounding batman-adv implementation.
extern "C" {
    fn batadv_tvlv_container_unregister(bat_priv: *mut batadv_priv, typ: u8, version: u8);
    fn batadv_tvlv_container_register(
        bat_priv: *mut batadv_priv,
        typ: u8,
        version: u8,
        value: *const c_void,
        value_len: usize,
    );
    fn batadv_gw_node_update(
        bat_priv: *mut batadv_priv,
        orig: *mut batadv_orig_node,
        gateway: *const batadv_tvlv_gateway_data,
    );
    fn batadv_gw_check_election(bat_priv: *mut batadv_priv, orig: *mut batadv_orig_node);
    fn batadv_tvlv_handler_register(
        bat_priv: *mut batadv_priv,
        handler: unsafe extern "C" fn(
            *mut batadv_priv,
            *mut batadv_orig_node,
            u8,
            *mut c_void,
            u16,
        ),
        _ogm_handler: *const c_void,
        _client_handler: *const c_void,
        typ: u8,
        version: u8,
        flags: u8,
    );
    fn batadv_tvlv_handler_unregister(bat_priv: *mut batadv_priv, typ: u8, version: u8);
}

// Types and constants are supplied by the included batman-adv headers.
type u32_ = u32;

#[repr(C)]
pub struct batadv_tvlv_gateway_data {
    pub bandwidth_down: u32,
    pub bandwidth_up: u32,
}

pub enum batadv_priv {}
pub enum batadv_orig_node {}

unsafe extern "C" fn batadv_gw_tvlv_ogm_handler_v1(
    bat_priv: *mut batadv_priv,
    orig: *mut batadv_orig_node,
    flags: u8,
    tvlv_value: *mut c_void,
    tvlv_value_len: u16,
) {
    let mut gateway: batadv_tvlv_gateway_data = batadv_tvlv_gateway_data {
        bandwidth_down: 0,
        bandwidth_up: 0,
    };

    /* only fetch the tvlv value if the handler wasn't called via the
     * CIFNOTFND flag and if there is data to fetch
     */
    if (flags & BATADV_TVLV_HANDLER_OGM_CIFNOTFND) != 0
        || (tvlv_value_len as usize) < size_of::<batadv_tvlv_gateway_data>()
    {
        gateway.bandwidth_down = 0;
        gateway.bandwidth_up = 0;
    } else {
        let gateway_ptr = tvlv_value as *const batadv_tvlv_gateway_data;
        gateway.bandwidth_down = (*gateway_ptr).bandwidth_down;
        gateway.bandwidth_up = (*gateway_ptr).bandwidth_up;
        if gateway.bandwidth_down == 0 || gateway.bandwidth_up == 0 {
            gateway.bandwidth_down = 0;
            gateway.bandwidth_up = 0;
        }
    }

    batadv_gw_node_update(bat_priv, orig, &gateway);

    /* restart gateway selection */
    if gateway.bandwidth_down != 0
        && core::ptr::read_volatile(&(*(bat_priv as *const batadv_priv)).gw.mode)
            == BATADV_GW_MODE_CLIENT
    {
        batadv_gw_check_election(bat_priv, orig);
    }
}

pub unsafe fn batadv_gw_tvlv_container_update(bat_priv: *mut batadv_priv) {
    let mut gw: batadv_tvlv_gateway_data = core::mem::zeroed();
    let gw_mode = core::ptr::read_volatile(&(*(bat_priv as *const batadv_priv)).gw.mode);

    match gw_mode {
        BATADV_GW_MODE_OFF | BATADV_GW_MODE_CLIENT => {
            batadv_tvlv_container_unregister(bat_priv, BATADV_TVLV_GW, 1);
        }
        BATADV_GW_MODE_SERVER => {
            let down = core::ptr::read_volatile(
                &(*(bat_priv as *const batadv_priv)).gw.bandwidth_down,
            );
            let up = core::ptr::read_volatile(
                &(*(bat_priv as *const batadv_priv)).gw.bandwidth_up,
            );
            gw.bandwidth_down = down.to_be();
            gw.bandwidth_up = up.to_be();
            batadv_tvlv_container_register(
                bat_priv,
                BATADV_TVLV_GW,
                1,
                &gw as *const _ as *const c_void,
                size_of::<batadv_tvlv_gateway_data>(),
            );
        }
        _ => {}
    }
}

pub unsafe fn batadv_gw_init(bat_priv: *mut batadv_priv) {
    if (*(bat_priv as *mut batadv_priv)).algo_ops.gw.init_sel_class.is_some() {
        ((*(bat_priv as *mut batadv_priv)).algo_ops.gw.init_sel_class.unwrap())(bat_priv);
    } else {
        core::ptr::write_volatile(&mut (*(bat_priv as *mut batadv_priv)).gw.sel_class, 1);
    }

    batadv_tvlv_handler_register(
        bat_priv,
        batadv_gw_tvlv_ogm_handler_v1,
        core::ptr::null(),
        core::ptr::null(),
        BATADV_TVLV_GW,
        1,
        BATADV_TVLV_HANDLER_OGM_CIFNOTFND,
    );
}

pub unsafe fn batadv_gw_free(bat_priv: *mut batadv_priv) {
    batadv_tvlv_container_unregister(bat_priv, BATADV_TVLV_GW, 1);
    batadv_tvlv_handler_unregister(bat_priv, BATADV_TVLV_GW, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
