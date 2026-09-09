/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2015  Intel Coropration

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

#[repr(C)]
pub struct mgmt_mesh_tx {
    pub list: list_head,
    pub index: i32,
    pub param_len: usize,
    pub sk: *mut sock,
    pub handle: u8,
    pub instance: u8,
    pub param: [u8; core::mem::size_of::<mgmt_cp_mesh_send>() + 31],
}

#[repr(C)]
pub struct mgmt_pending_cmd {
    pub list: list_head,
    pub opcode: u16,
    pub hdev: *mut hci_dev,
    pub param: *mut core::ffi::c_void,
    pub param_len: usize,
    pub sk: *mut sock,
    pub skb: *mut sk_buff,
    pub user_data: *mut core::ffi::c_void,
    pub cmd_complete: Option<unsafe extern "C" fn(cmd: *mut mgmt_pending_cmd, status: u8) -> i32>,
}

extern "C" {
    pub fn mgmt_alloc_skb(hdev: *mut hci_dev, opcode: u16, size: u32) -> *mut sk_buff;
    pub fn mgmt_send_event_skb(channel: u16, skb: *mut sk_buff, flag: i32,
                               skip_sk: *mut sock) -> i32;
    pub fn mgmt_send_event(event: u16, hdev: *mut hci_dev, channel: u16,
                           data: *mut core::ffi::c_void, data_len: u16,
                           flag: i32, skip_sk: *mut sock) -> i32;
    pub fn mgmt_cmd_status(sk: *mut sock, index: u16, cmd: u16, status: u8) -> i32;
    pub fn mgmt_cmd_complete(sk: *mut sock, index: u16, cmd: u16, status: u8,
                             rp: *mut core::ffi::c_void, rp_len: usize) -> i32;

    pub fn mgmt_pending_find(channel: u16, opcode: u16,
                             hdev: *mut hci_dev) -> *mut mgmt_pending_cmd;
    pub fn mgmt_pending_foreach(
        opcode: u16,
        hdev: *mut hci_dev,
        remove: bool,
        cb: Option<unsafe extern "C" fn(cmd: *mut mgmt_pending_cmd, data: *mut core::ffi::c_void)>,
        data: *mut core::ffi::c_void,
    );
    pub fn mgmt_pending_add(sk: *mut sock, opcode: u16, hdev: *mut hci_dev,
                            data: *mut core::ffi::c_void, len: u16) -> *mut mgmt_pending_cmd;
    pub fn mgmt_pending_new(sk: *mut sock, opcode: u16, hdev: *mut hci_dev,
                            data: *mut core::ffi::c_void, len: u16) -> *mut mgmt_pending_cmd;
    pub fn mgmt_pending_free(cmd: *mut mgmt_pending_cmd);
    pub fn mgmt_pending_remove(cmd: *mut mgmt_pending_cmd);
    pub fn __mgmt_pending_listed(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool;
    pub fn mgmt_pending_listed(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool;
    pub fn mgmt_pending_valid(hdev: *mut hci_dev, cmd: *mut mgmt_pending_cmd) -> bool;
    pub fn mgmt_mesh_foreach(
        hdev: *mut hci_dev,
        cb: Option<unsafe extern "C" fn(mesh_tx: *mut mgmt_mesh_tx, data: *mut core::ffi::c_void)>,
        data: *mut core::ffi::c_void,
        sk: *mut sock,
    );
    pub fn mgmt_mesh_find(hdev: *mut hci_dev, handle: u8) -> *mut mgmt_mesh_tx;
    pub fn mgmt_mesh_next(hdev: *mut hci_dev, sk: *mut sock) -> *mut mgmt_mesh_tx;
    pub fn mgmt_mesh_add(sk: *mut sock, hdev: *mut hci_dev,
                         data: *mut core::ffi::c_void, len: u16) -> *mut mgmt_mesh_tx;
    pub fn mgmt_mesh_remove(mesh_tx: *mut mgmt_mesh_tx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
