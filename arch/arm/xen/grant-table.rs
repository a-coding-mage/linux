/******************************************************************************
 * grant_table.rs
 * ARM specific part
 *
 * Granting foreign access to our memory reservation.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation; or, when distributed
 * separately from the Linux kernel or incorporated into other
 * software packages, subject to the following license:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this source file (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// Dependencies supplied by Xen headers:
// xen_pfn_t, grant_status_t, and ENOSYS.

pub unsafe fn arch_gnttab_map_shared(
    frames: *mut xen_pfn_t,
    nr_gframes: c_ulong,
    max_nr_gframes: c_ulong,
    shared: *mut *mut c_void,
) -> c_int {
    let _ = (frames, nr_gframes, max_nr_gframes, shared);
    -ENOSYS
}

pub unsafe fn arch_gnttab_unmap(shared: *mut c_void, nr_gframes: c_ulong) {
    let _ = (shared, nr_gframes);
}

pub unsafe fn arch_gnttab_map_status(
    frames: *mut u64,
    nr_gframes: c_ulong,
    max_nr_gframes: c_ulong,
    shared: *mut *mut grant_status_t,
) -> c_int {
    let _ = (frames, nr_gframes, max_nr_gframes, shared);
    -ENOSYS
}

pub unsafe fn arch_gnttab_init(nr_shared: c_ulong, nr_status: c_ulong) -> c_int {
    let _ = (nr_shared, nr_status);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
