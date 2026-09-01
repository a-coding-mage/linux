// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// Generic IPC layer that can work over MMIO and SPI/I2C. PHY layer provided
// by platform driver code.
//

use core::ffi::{c_int, c_void};

pub type size_t = usize;
pub type u32 = u32;

const ENODEV: c_int = 19;
const ENOBUFS: c_int = 105;
const GFP_KERNEL: c_int = 0;
const SOF_FW_BOOT_COMPLETE: c_int = 0;
const SOF_IPC_TYPE_3: c_int = 3;
const SOF_IPC_TYPE_4: c_int = 4;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
	_private: [u8; 0],
}

#[repr(C)]
pub struct spinlock {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_pdata {
	pub ipc_type: c_int,
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
	pub msg_data: *mut c_void,
	pub msg_size: size_t,
	pub reply_size: size_t,
	pub reply_error: c_int,
	pub ipc_complete: bool,
	pub waitq: wait_queue_head,
}

#[repr(C)]
pub struct snd_sof_ipc {
	pub disable_ipc_tx: bool,
	pub tx_mutex: mutex,
	pub sdev: *mut snd_sof_dev,
	pub msg: snd_sof_ipc_msg,
	pub max_payload_size: size_t,
	pub ops: *const sof_ipc_ops,
}

#[repr(C)]
pub struct snd_sof_dev {
	pub ipc: *mut snd_sof_ipc,
	pub fw_state: c_int,
	pub ipc_lock: spinlock,
	pub msg: *mut snd_sof_ipc_msg,
	pub dev: *mut device,
	pub pdata: *mut snd_sof_pdata,
}

#[repr(C)]
pub struct sof_ipc_fw_loader_ops {
	pub validate: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub parse_ext_manifest: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_pcm_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_tplg_widget_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
	pub widget: *const sof_ipc_tplg_widget_ops,
	pub control: *const sof_ipc_tplg_control_ops,
}

#[repr(C)]
pub struct sof_ipc_fw_tracing_ops {
	pub init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub suspend: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_ops {
	pub tx_msg: Option<
		unsafe extern "C" fn(
			*mut snd_sof_dev,
			*mut c_void,
			size_t,
			*mut c_void,
			size_t,
			bool,
		) -> c_int,
	>,
	pub rx_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub set_get_data:
		Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut c_void, size_t, bool) -> c_int>,
	pub get_reply: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub fw_loader: *const sof_ipc_fw_loader_ops,
	pub pcm: *const sof_ipc_pcm_ops,
	pub tplg: *const sof_ipc_tplg_ops,
	pub fw_tracing: *const sof_ipc_fw_tracing_ops,
	pub init: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
	pub exit: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
}

unsafe extern "C" {
	static ipc3_ops: sof_ipc_ops;
	static ipc4_ops: sof_ipc_ops;

	fn snd_sof_dsp_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
	fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
	fn mutex_init(lock: *mut mutex);
	fn mutex_lock(lock: *mut mutex);
	fn mutex_unlock(lock: *mut mutex);
	fn spin_lock_irq(lock: *mut spinlock);
	fn spin_unlock_irq(lock: *mut spinlock);
	fn init_waitqueue_head(wq_head: *mut wait_queue_head);
	fn wake_up(wq_head: *mut wait_queue_head);
	fn dev_warn(dev: *mut device, fmt: *const u8, ...);
	fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
	fn dev_err(dev: *mut device, fmt: *const u8, ...);
}

/**
 * sof_ipc_send_msg - generic function to prepare and send one IPC message
 * @sdev:		pointer to SOF core device struct
 * @msg_data:		pointer to a message to send
 * @msg_bytes:		number of bytes in the message
 * @reply_bytes:	number of bytes available for the reply.
 *			The buffer for the reply data is not passed to this
 *			function, the available size is an information for the
 *			reply handling functions.
 *
 * On success the function returns 0, otherwise negative error number.
 *
 * Note: higher level sdev->ipc->tx_mutex must be held to make sure that
 *	 transfers are synchronized.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc_send_msg(
	sdev: *mut snd_sof_dev,
	msg_data: *mut c_void,
	msg_bytes: size_t,
	reply_bytes: size_t,
) -> c_int {
	let ipc = (*sdev).ipc;
	let msg: *mut snd_sof_ipc_msg;
	let ret: c_int;

	if (*ipc).disable_ipc_tx || (*sdev).fw_state != SOF_FW_BOOT_COMPLETE {
		return -ENODEV;
	}

	/*
	 * The spin-lock is needed to protect message objects against other
	 * atomic contexts.
	 */
	spin_lock_irq(&mut (*sdev).ipc_lock);

	/* initialise the message */
	msg = &mut (*ipc).msg;

	/* attach message data */
	(*msg).msg_data = msg_data;
	(*msg).msg_size = msg_bytes;

	(*msg).reply_size = reply_bytes;
	(*msg).reply_error = 0;

	(*sdev).msg = msg;

	ret = snd_sof_dsp_send_msg(sdev, msg);
	/* Next reply that we receive will be related to this message */
	if ret == 0 {
		(*msg).ipc_complete = false;
	}

	spin_unlock_irq(&mut (*sdev).ipc_lock);

	ret
}

/* send IPC message from host to DSP */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc_tx_message(
	ipc: *mut snd_sof_ipc,
	msg_data: *mut c_void,
	msg_bytes: size_t,
	reply_data: *mut c_void,
	reply_bytes: size_t,
) -> c_int {
	if msg_bytes > (*ipc).max_payload_size || reply_bytes > (*ipc).max_payload_size {
		return -ENOBUFS;
	}

	((*(*ipc).ops).tx_msg.unwrap())(
		(*ipc).sdev,
		msg_data,
		msg_bytes,
		reply_data,
		reply_bytes,
		false,
	)
}

/* IPC set or get data from host to DSP */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc_set_get_data(
	ipc: *mut snd_sof_ipc,
	msg_data: *mut c_void,
	msg_bytes: size_t,
	set: bool,
) -> c_int {
	((*(*ipc).ops).set_get_data.unwrap())((*ipc).sdev, msg_data, msg_bytes, set)
}

/*
 * send IPC message from host to DSP without modifying the DSP state.
 * This will be used for IPC's that can be handled by the DSP
 * even in a low-power D0 substate.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc_tx_message_no_pm(
	ipc: *mut snd_sof_ipc,
	msg_data: *mut c_void,
	msg_bytes: size_t,
	reply_data: *mut c_void,
	reply_bytes: size_t,
) -> c_int {
	if msg_bytes > (*ipc).max_payload_size || reply_bytes > (*ipc).max_payload_size {
		return -ENOBUFS;
	}

	((*(*ipc).ops).tx_msg.unwrap())(
		(*ipc).sdev,
		msg_data,
		msg_bytes,
		reply_data,
		reply_bytes,
		true,
	)
}

/* Generic helper function to retrieve the reply */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev) {
	/*
	 * Sometimes, there is unexpected reply ipc arriving. The reply
	 * ipc belongs to none of the ipcs sent from driver.
	 * In this case, the driver must ignore the ipc.
	 */
	if (*sdev).msg.is_null() {
		dev_warn((*sdev).dev, c"unexpected ipc interrupt raised!\n".as_ptr() as *const u8);
		return;
	}

	(*(*sdev).msg).reply_error = ((*(*(*sdev).ipc).ops).get_reply.unwrap())(sdev);
}

/* handle reply message from DSP */
#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, msg_id: u32) {
	let msg = &mut (*(*sdev).ipc).msg as *mut snd_sof_ipc_msg;

	if (*msg).ipc_complete {
		dev_dbg(
			(*sdev).dev,
			c"no reply expected, received 0x%x, will be ignored".as_ptr() as *const u8,
			msg_id,
		);
		return;
	}

	/* wake up and return the error if we have waiters on this message ? */
	(*msg).ipc_complete = true;
	wake_up(&mut (*msg).waitq);
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_init(sdev: *mut snd_sof_dev) -> *mut snd_sof_ipc {
	let ipc: *mut snd_sof_ipc;
	let msg: *mut snd_sof_ipc_msg;
	let ops: *const sof_ipc_ops;

	ipc = devm_kzalloc(
		(*sdev).dev,
		core::mem::size_of::<snd_sof_ipc>(),
		GFP_KERNEL,
	) as *mut snd_sof_ipc;
	if ipc.is_null() {
		return core::ptr::null_mut();
	}

	mutex_init(&mut (*ipc).tx_mutex);
	(*ipc).sdev = sdev;
	msg = &mut (*ipc).msg;

	/* indicate that we aren't sending a message ATM */
	(*msg).ipc_complete = true;

	init_waitqueue_head(&mut (*msg).waitq);

	match (*(*sdev).pdata).ipc_type {
		/* CONFIG_SND_SOC_SOF_IPC3 */
		SOF_IPC_TYPE_3 => {
			ops = &ipc3_ops;
		}
		/* CONFIG_SND_SOC_SOF_IPC4 */
		SOF_IPC_TYPE_4 => {
			ops = &ipc4_ops;
		}
		_ => {
			dev_err(
				(*sdev).dev,
				c"Not supported IPC version: %d\n".as_ptr() as *const u8,
				(*(*sdev).pdata).ipc_type,
			);
			return core::ptr::null_mut();
		}
	}

	/* check for mandatory ops */
	if (*ops).tx_msg.is_none()
		|| (*ops).rx_msg.is_none()
		|| (*ops).set_get_data.is_none()
		|| (*ops).get_reply.is_none()
	{
		dev_err((*sdev).dev, c"Missing IPC message handling ops\n".as_ptr() as *const u8);
		return core::ptr::null_mut();
	}

	if (*ops).fw_loader.is_null()
		|| (*(*ops).fw_loader).validate.is_none()
		|| (*(*ops).fw_loader).parse_ext_manifest.is_none()
	{
		dev_err((*sdev).dev, c"Missing IPC firmware loading ops\n".as_ptr() as *const u8);
		return core::ptr::null_mut();
	}

	if (*ops).pcm.is_null() {
		dev_err((*sdev).dev, c"Missing IPC PCM ops\n".as_ptr() as *const u8);
		return core::ptr::null_mut();
	}

	if (*ops).tplg.is_null() || (*(*ops).tplg).widget.is_null() || (*(*ops).tplg).control.is_null()
	{
		dev_err((*sdev).dev, c"Missing IPC topology ops\n".as_ptr() as *const u8);
		return core::ptr::null_mut();
	}

	if !(*ops).fw_tracing.is_null()
		&& ((*(*ops).fw_tracing).init.is_none()
			|| (*(*ops).fw_tracing).suspend.is_none()
			|| (*(*ops).fw_tracing).resume.is_none())
	{
		dev_err((*sdev).dev, c"Missing firmware tracing ops\n".as_ptr() as *const u8);
		return core::ptr::null_mut();
	}

	if let Some(init) = (*ops).init {
		if init(sdev) != 0 {
			return core::ptr::null_mut();
		}
	}

	(*ipc).ops = ops;

	ipc
}

#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_free(sdev: *mut snd_sof_dev) {
	let ipc = (*sdev).ipc;

	if ipc.is_null() {
		return;
	}

	/* disable sending of ipc's */
	mutex_lock(&mut (*ipc).tx_mutex);
	(*ipc).disable_ipc_tx = true;
	mutex_unlock(&mut (*ipc).tx_mutex);

	if let Some(exit) = (*(*ipc).ops).exit {
		exit(sdev);
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
