/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, as published by
 * the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this file; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 * or visit http://www.gnu.org/licenses/.
 *
 * This file may also be available under a different license from Cavium.
 * Contact Cavium Networks for more information
 ***********************license end**************************************/

/*
 * Small helper utilities.
 */

extern "C" {
    /** Convert an interface mode into a human readable string. */
    pub fn cvmx_helper_interface_mode_to_string(
        mode: cvmx_helper_interface_mode,
    ) -> *const ::core::ffi::c_char;
    pub fn cvmx_helper_setup_red(pass_thresh: i32, drop_thresh: i32) -> i32;
    pub fn cvmx_helper_get_version() -> *const ::core::ffi::c_char;
    pub fn __cvmx_helper_setup_gmx(interface: i32, num_ports: i32) -> i32;
    pub fn cvmx_helper_get_ipd_port(interface: i32, port: i32) -> i32;
    pub fn cvmx_helper_get_interface_num(ipd_port: i32) -> i32;
    pub fn cvmx_helper_get_interface_index_num(ipd_port: i32) -> i32;
    pub fn cvmx_helper_ports_on_interface(interface: i32) -> i32;
    pub fn cvmx_ptr_to_phys(ptr: *const ::core::ffi::c_void) -> u64;
    pub fn cvmx_phys_to_ptr(phys: u64) -> *mut ::core::ffi::c_void;
    pub fn cvmx_fpa_free(ptr: *mut ::core::ffi::c_void, pool: u64, back: i32);
}

#[inline]
pub unsafe fn cvmx_helper_get_first_ipd_port(interface: i32) -> i32 {
    unsafe { cvmx_helper_get_ipd_port(interface, 0) }
}

#[inline]
pub unsafe fn cvmx_helper_get_last_ipd_port(interface: i32) -> i32 {
    unsafe {
        cvmx_helper_get_first_ipd_port(interface)
            + cvmx_helper_ports_on_interface(interface)
            - 1
    }
}

#[inline]
pub unsafe fn cvmx_helper_free_packet_data(work: *mut cvmx_wqe) {
    let mut number_buffers: u64;
    let mut buffer_ptr: cvmx_buf_ptr;
    let mut next_buffer_ptr: cvmx_buf_ptr;
    let mut start_of_buffer: u64;

    unsafe {
        number_buffers = (*work).word2.s.bufs;
        if number_buffers == 0 {
            return;
        }
        buffer_ptr = (*work).packet_ptr;

        /*
         * Since the number of buffers is not zero, we know this is
         * not a dynamic short packet. We need to check if it is a
         * packet received with IPD_CTL_STATUS[NO_WPTR]. If this is
         * true, we need to free all buffers except for the first
         * one. The caller doesn't expect their WQE pointer to be
         * freed
         */
        start_of_buffer = ((buffer_ptr.s.addr >> 7) - buffer_ptr.s.back) << 7;
        if cvmx_ptr_to_phys(work as *const ::core::ffi::c_void) == start_of_buffer {
            next_buffer_ptr = *(cvmx_phys_to_ptr(buffer_ptr.s.addr - 8) as *const cvmx_buf_ptr);
            buffer_ptr = next_buffer_ptr;
            number_buffers -= 1;
        }

        while number_buffers != 0 {
            /* Remember the back pointer is in cache lines, not 64bit words */
            start_of_buffer = ((buffer_ptr.s.addr >> 7) - buffer_ptr.s.back) << 7;
            /* Read pointer to next buffer before we free the current buffer. */
            next_buffer_ptr = *(cvmx_phys_to_ptr(buffer_ptr.s.addr - 8) as *const cvmx_buf_ptr);
            cvmx_fpa_free(
                cvmx_phys_to_ptr(start_of_buffer),
                buffer_ptr.s.pool,
                0,
            );
            buffer_ptr = next_buffer_ptr;
            number_buffers -= 1;
        }
    }
}
    /** Setup Random Early Drop to automatically begin dropping packets. */
    /** Get the version of the CVMX libraries. */
    /** Setup the common GMX settings that determine the number of ports. */
    /** Returns the IPD/PKO port number for a port on the given interface. */
    /** Returns the interface number for an IPD/PKO port number. */
    /** Returns the interface index number for an IPD/PKO port number. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
