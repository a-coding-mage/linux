/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the original header:
//   #include <linux/poll.h>
//   #include "tpm.h"

use core::ffi::c_char;

// External types supplied by the included headers.
pub type ssize_t = isize;
pub type size_t = usize;
pub type loff_t = i64;
pub type __poll_t = u32;
pub type u8 = core::primitive::u8;

// TPM_BUFSIZE is supplied by the included TPM header.

pub enum file {}
pub enum tpm_chip {}
pub enum tpm_space {}
pub enum mutex {}
pub enum timer_list {}
pub enum work_struct {}
pub enum wait_queue_head_t {}
pub enum poll_table {}

#[repr(C)]
pub struct file_priv {
    pub chip: *mut tpm_chip,
    pub space: *mut tpm_space,

    pub buffer_mutex: mutex,
    pub user_read_timer: timer_list, // user needs to claim result
    pub timeout_work: work_struct,
    pub async_work: work_struct,
    pub async_wait: wait_queue_head_t,
    pub response_length: ssize_t,
    pub response_read: bool,
    pub command_enqueued: bool,

    pub data_buffer: [u8; TPM_BUFSIZE],
}

extern "C" {
    pub fn tpm_common_open(
        file: *mut file,
        chip: *mut tpm_chip,
        priv_: *mut file_priv,
        space: *mut tpm_space,
    );

    pub fn tpm_common_read(
        file: *mut file,
        buf: *mut c_char,
        size: size_t,
        off: *mut loff_t,
    ) -> ssize_t;

    pub fn tpm_common_write(
        file: *mut file,
        buf: *const c_char,
        size: size_t,
        off: *mut loff_t,
    ) -> ssize_t;

    pub fn tpm_common_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;

    pub fn tpm_common_release(file: *mut file, priv_: *mut file_priv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
