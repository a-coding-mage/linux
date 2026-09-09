/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2014 Intel Corporation

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF THE
   USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

/* Equivalent of: IS_ENABLED(CONFIG_BT_SELFTEST) && IS_MODULE(CONFIG_BT). */
#[cfg(all(CONFIG_BT_SELFTEST, CONFIG_BT_MODULE))]
extern "C" {
    pub fn bt_selftest() -> core::ffi::c_int;
}

/* When CONFIG_BT_SELFTEST=y and CONFIG_BT=m, self testing is run at module
 * loading time.
 */

/* When CONFIG_BT_SELFTEST=y and CONFIG_BT=y, self testing is run via
 * late_initcall() to make sure that subsys_initcall() of the Bluetooth
 * subsystem and device_initcall() of the Crypto subsystem do not clash.
 *
 * When CONFIG_BT_SELFTEST=n, this turns into an empty call that has no impact.
 */
#[cfg(not(all(CONFIG_BT_SELFTEST, CONFIG_BT_MODULE)))]
#[inline]
pub fn bt_selftest() -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
