/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2014 Intel Corporation

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, ANY SPECIAL DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR
   IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

// Conditional equivalent of IS_ENABLED(CONFIG_BT_DEBUGFS).
#[cfg(CONFIG_BT_DEBUGFS)]
extern "C" {
    pub fn hci_debugfs_create_common(hdev: *mut hci_dev);
    pub fn hci_debugfs_create_bredr(hdev: *mut hci_dev);
    pub fn hci_debugfs_create_le(hdev: *mut hci_dev);
    pub fn hci_debugfs_create_conn(conn: *mut hci_conn);
    pub fn hci_debugfs_create_basic(hdev: *mut hci_dev);
}

#[cfg(not(CONFIG_BT_DEBUGFS))]
#[inline]
pub unsafe fn hci_debugfs_create_common(_hdev: *mut hci_dev) {}

#[cfg(not(CONFIG_BT_DEBUGFS))]
#[inline]
pub unsafe fn hci_debugfs_create_bredr(_hdev: *mut hci_dev) {}

#[cfg(not(CONFIG_BT_DEBUGFS))]
#[inline]
pub unsafe fn hci_debugfs_create_le(_hdev: *mut hci_dev) {}

#[cfg(not(CONFIG_BT_DEBUGFS))]
#[inline]
pub unsafe fn hci_debugfs_create_conn(_conn: *mut hci_conn) {}

#[cfg(not(CONFIG_BT_DEBUGFS))]
#[inline]
pub unsafe fn hci_debugfs_create_basic(_hdev: *mut hci_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
