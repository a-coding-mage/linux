/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright		   : (C) 2002,2003 by Frank Mori Hess
 ***************************************************************************/

// C header guard: _GPIB_P_H

// C dependencies:
// #include <linux/types.h>
// #include "gpib_types.h"
// #include "gpib_proto.h"
// #include "gpib_cmd.h"
// #include <linux/gpib.h>
// #include <linux/gpib_ioctl.h>
// #include <linux/fs.h>
// #include <linux/interrupt.h>
// #include <linux/io.h>

unsafe extern "C" {
    pub fn gpib_register_driver(interface: *mut gpib_interface, mod_: *mut module) -> ::std::os::raw::c_int;
    pub fn gpib_unregister_driver(interface: *mut gpib_interface);
    pub fn gpib_pci_get_device(
        config: *const gpib_board_config,
        vendor_id: ::std::os::raw::c_uint,
        device_id: ::std::os::raw::c_uint,
        from: *mut pci_dev,
    ) -> *mut pci_dev;
    pub fn gpib_pci_get_subsys(
        config: *const gpib_board_config,
        vendor_id: ::std::os::raw::c_uint,
        device_id: ::std::os::raw::c_uint,
        ss_vendor: ::std::os::raw::c_uint,
        ss_device: ::std::os::raw::c_uint,
        from: *mut pci_dev,
    ) -> *mut pci_dev;
    pub fn num_gpib_events(queue: *const gpib_event_queue) -> ::std::os::raw::c_uint;
    pub fn push_gpib_event(board: *mut gpib_board, event_type: ::std::os::raw::c_short) -> ::std::os::raw::c_int;
    pub fn pop_gpib_event(
        board: *mut gpib_board,
        queue: *mut gpib_event_queue,
        event_type: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
    pub fn gpib_request_pseudo_irq(
        board: *mut gpib_board,
        handler: Option<unsafe extern "C" fn(::std::os::raw::c_int, *mut ::std::os::raw::c_void) -> irqreturn_t>,
    ) -> ::std::os::raw::c_int;
    pub fn gpib_free_pseudo_irq(board: *mut gpib_board);
    pub fn gpib_match_device_path(
        dev: *mut device,
        device_path_in: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub static mut board_array: [gpib_board; GPIB_MAX_NUM_BOARDS];
    pub static mut registered_drivers: list_head;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
