/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2016 Mellanox Technologies, LTD. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, or the OpenIB.org BSD license.
 */

// Dependencies supplied by the corresponding RDMA headers:
// <rdma/ib_user_mad.h>, <rdma/hfi/hfi1_ioctl.h>, and
// <rdma/rdma_user_ioctl_cmds.h>.

/// Legacy name, for user space applications which already use it.
pub const IB_IOCTL_MAGIC: u8 = RDMA_IOCTL_MAGIC;

/*
 * General blocks assignments.
 * It is closed on purpose - do not expose it to user space.
 * #define MAD_CMD_BASE 0x00
 * #define HFI1_CMD_BAS 0xE0
 */

/* MAD specific section */
pub const IB_USER_MAD_REGISTER_AGENT: _ = _IOWR!(RDMA_IOCTL_MAGIC, 0x01, ib_user_mad_reg_req);
pub const IB_USER_MAD_UNREGISTER_AGENT: _ = _IOW!(RDMA_IOCTL_MAGIC, 0x02, u32);
pub const IB_USER_MAD_ENABLE_PKEY: _ = _IO!(RDMA_IOCTL_MAGIC, 0x03);
pub const IB_USER_MAD_REGISTER_AGENT2: _ = _IOWR!(RDMA_IOCTL_MAGIC, 0x04, ib_user_mad_reg_req2);

/* HFI specific section */
/* allocate HFI and context */
pub const HFI1_IOCTL_ASSIGN_CTXT: _ = _IOWR!(RDMA_IOCTL_MAGIC, 0xE1, hfi1_user_info);
/* find out what resources we got */
pub const HFI1_IOCTL_CTXT_INFO: _ = _IOW!(RDMA_IOCTL_MAGIC, 0xE2, hfi1_ctxt_info);
/* set up userspace */
pub const HFI1_IOCTL_USER_INFO: _ = _IOW!(RDMA_IOCTL_MAGIC, 0xE3, hfi1_base_info);
/* update expected TID entries */
pub const HFI1_IOCTL_TID_UPDATE: _ = _IOWR!(RDMA_IOCTL_MAGIC, 0xE4, hfi1_tid_info);
/* free expected TID entries */
pub const HFI1_IOCTL_TID_FREE: _ = _IOWR!(RDMA_IOCTL_MAGIC, 0xE5, hfi1_tid_info);
/* force an update of PIO credit */
pub const HFI1_IOCTL_CREDIT_UPD: _ = _IO!(RDMA_IOCTL_MAGIC, 0xE6);
/* control receipt of packets */
pub const HFI1_IOCTL_RECV_CTRL: _ = _IOW!(RDMA_IOCTL_MAGIC, 0xE8, i32);
/* set the kind of polling we want */
pub const HFI1_IOCTL_POLL_TYPE: _ = _IOW!(RDMA_IOCTL_MAGIC, 0xE9, i32);
/* ack & clear user status bits */
pub const HFI1_IOCTL_ACK_EVENT: _ = _IOW!(RDMA_IOCTL_MAGIC, 0xEA, libc::c_ulong);
/* set context's pkey */
pub const HFI1_IOCTL_SET_PKEY: _ = _IOW!(RDMA_IOCTL_MAGIC, 0xEB, u16);
/* reset context's HW send context */
pub const HFI1_IOCTL_CTXT_RESET: _ = _IO!(RDMA_IOCTL_MAGIC, 0xEC);
/* read TID cache invalidations */
pub const HFI1_IOCTL_TID_INVAL_READ: _ = _IOWR!(RDMA_IOCTL_MAGIC, 0xED, hfi1_tid_info);
/* get the version of the user cdev */
pub const HFI1_IOCTL_GET_VERS: _ = _IOR!(RDMA_IOCTL_MAGIC, 0xEE, i32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
