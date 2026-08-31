/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Userspace interface for in-kernel virtio accelerators. */

/* vhost is used to reduce the number of system calls involved in virtio.
 *
 * Existing virtio net code is used in the guest without modification.
 *
 * This header includes interface used by userspace hypervisor for
 * device configuration.
 */

/* Depends on Linux uapi definitions from:
 * <linux/vhost_types.h>, <linux/types.h>, and <linux/ioctl.h>.
 */

pub const VHOST_FILE_UNBIND: i32 = -1;

/* ioctls */

pub const VHOST_VIRTIO: u32 = 0xAF;

/* Features bitmask for forward compatibility.  Transport bits are used for
 * vhost specific features. */
pub const VHOST_GET_FEATURES: u64 = _IOR!(VHOST_VIRTIO, 0x00, __u64);
pub const VHOST_SET_FEATURES: u64 = _IOW!(VHOST_VIRTIO, 0x00, __u64);

/* Set current process as the (exclusive) owner of this file descriptor.  This
 * must be called before any other vhost command.  Further calls to
 * VHOST_SET_OWNER fail until VHOST_RESET_OWNER is called. */
pub const VHOST_SET_OWNER: u64 = _IO!(VHOST_VIRTIO, 0x01);
/* Give up ownership, and reset the device to default values.
 * Allows subsequent call to VHOST_SET_OWNER to succeed. */
pub const VHOST_RESET_OWNER: u64 = _IO!(VHOST_VIRTIO, 0x02);

/* Set up/modify memory layout */
pub const VHOST_SET_MEM_TABLE: u64 = _IOW!(VHOST_VIRTIO, 0x03, vhost_memory);

/* Write logging setup. */
/* Memory writes can optionally be logged by setting bit at an offset
 * (calculated from the physical address) from specified log base.
 * The bit is set using an atomic 32 bit operation. */
/* Set base address for logging. */
pub const VHOST_SET_LOG_BASE: u64 = _IOW!(VHOST_VIRTIO, 0x04, __u64);
/* Specify an eventfd file descriptor to signal on log write. */
pub const VHOST_SET_LOG_FD: u64 = _IOW!(VHOST_VIRTIO, 0x07, i32);
/* By default, a device gets one vhost_worker that its virtqueues share. This
 * command allows the owner of the device to create an additional vhost_worker
 * for the device. It can later be bound to 1 or more of its virtqueues using
 * the VHOST_ATTACH_VRING_WORKER command.
 *
 * This must be called after VHOST_SET_OWNER and the caller must be the owner
 * of the device. The new thread will inherit caller's cgroups and namespaces,
 * and will share the caller's memory space. The new thread will also be
 * counted against the caller's RLIMIT_NPROC value.
 *
 * The worker's ID used in other commands will be returned in
 * vhost_worker_state.
 */
pub const VHOST_NEW_WORKER: u64 = _IOR!(VHOST_VIRTIO, 0x8, vhost_worker_state);
/* Free a worker created with VHOST_NEW_WORKER if it's not attached to any
 * virtqueue. If userspace is not able to call this for workers its created,
 * the kernel will free all the device's workers when the device is closed.
 */
pub const VHOST_FREE_WORKER: u64 = _IOW!(VHOST_VIRTIO, 0x9, vhost_worker_state);

/* Ring setup. */
/* Set number of descriptors in ring. This parameter can not
 * be modified while ring is running (bound to a device). */
pub const VHOST_SET_VRING_NUM: u64 = _IOW!(VHOST_VIRTIO, 0x10, vhost_vring_state);
/* Set addresses for the ring. */
pub const VHOST_SET_VRING_ADDR: u64 = _IOW!(VHOST_VIRTIO, 0x11, vhost_vring_addr);
/* Base value where queue looks for available descriptors */
pub const VHOST_SET_VRING_BASE: u64 = _IOW!(VHOST_VIRTIO, 0x12, vhost_vring_state);
/* Get accessor: reads index, writes value in num */
pub const VHOST_GET_VRING_BASE: u64 = _IOWR!(VHOST_VIRTIO, 0x12, vhost_vring_state);

/* Set the vring byte order in num. Valid values are VHOST_VRING_LITTLE_ENDIAN
 * or VHOST_VRING_BIG_ENDIAN (other values return -EINVAL).
 * The byte order cannot be changed while the device is active: trying to do so
 * returns -EBUSY.
 * This is a legacy only API that is simply ignored when VIRTIO_F_VERSION_1 is
 * set.
 * Not all kernel configurations support this ioctl, but all configurations that
 * support SET also support GET.
 */
pub const VHOST_VRING_LITTLE_ENDIAN: u32 = 0;
pub const VHOST_VRING_BIG_ENDIAN: u32 = 1;
pub const VHOST_SET_VRING_ENDIAN: u64 = _IOW!(VHOST_VIRTIO, 0x13, vhost_vring_state);
pub const VHOST_GET_VRING_ENDIAN: u64 = _IOW!(VHOST_VIRTIO, 0x14, vhost_vring_state);
/* Attach a vhost_worker created with VHOST_NEW_WORKER to one of the device's
 * virtqueues.
 *
 * This will replace the virtqueue's existing worker. If the replaced worker
 * is no longer attached to any virtqueues, it can be freed with
 * VHOST_FREE_WORKER.
 */
pub const VHOST_ATTACH_VRING_WORKER: u64 = _IOW!(VHOST_VIRTIO, 0x15, vhost_vring_worker);
/* Return the vring worker's ID */
pub const VHOST_GET_VRING_WORKER: u64 = _IOWR!(VHOST_VIRTIO, 0x16, vhost_vring_worker);

/* The following ioctls use eventfd file descriptors to signal and poll
 * for events. */

/* Set eventfd to poll for added buffers */
pub const VHOST_SET_VRING_KICK: u64 = _IOW!(VHOST_VIRTIO, 0x20, vhost_vring_file);
/* Set eventfd to signal when buffers have beed used */
pub const VHOST_SET_VRING_CALL: u64 = _IOW!(VHOST_VIRTIO, 0x21, vhost_vring_file);
/* Set eventfd to signal an error */
pub const VHOST_SET_VRING_ERR: u64 = _IOW!(VHOST_VIRTIO, 0x22, vhost_vring_file);
/* Set busy loop timeout (in us) */
pub const VHOST_SET_VRING_BUSYLOOP_TIMEOUT: u64 =
    _IOW!(VHOST_VIRTIO, 0x23, vhost_vring_state);
/* Get busy loop timeout (in us) */
pub const VHOST_GET_VRING_BUSYLOOP_TIMEOUT: u64 =
    _IOW!(VHOST_VIRTIO, 0x24, vhost_vring_state);

/* Set or get vhost backend capability */

pub const VHOST_SET_BACKEND_FEATURES: u64 = _IOW!(VHOST_VIRTIO, 0x25, __u64);
pub const VHOST_GET_BACKEND_FEATURES: u64 = _IOR!(VHOST_VIRTIO, 0x26, __u64);

/* VHOST_NET specific defines */

/* Attach virtio net ring to a raw socket, or tap device.
 * The socket must be already bound to an ethernet device, this device will be
 * used for transmit.  Pass fd -1 to unbind from the socket and the transmit
 * device.  This can be used to stop the ring (e.g. for migration). */
pub const VHOST_NET_SET_BACKEND: u64 = _IOW!(VHOST_VIRTIO, 0x30, vhost_vring_file);

/* VHOST_SCSI specific defines */

pub const VHOST_SCSI_SET_ENDPOINT: u64 = _IOW!(VHOST_VIRTIO, 0x40, vhost_scsi_target);
pub const VHOST_SCSI_CLEAR_ENDPOINT: u64 = _IOW!(VHOST_VIRTIO, 0x41, vhost_scsi_target);
/* Changing this breaks userspace. */
pub const VHOST_SCSI_GET_ABI_VERSION: u64 = _IOW!(VHOST_VIRTIO, 0x42, i32);
/* Set and get the events missed flag */
pub const VHOST_SCSI_SET_EVENTS_MISSED: u64 = _IOW!(VHOST_VIRTIO, 0x43, __u32);
pub const VHOST_SCSI_GET_EVENTS_MISSED: u64 = _IOW!(VHOST_VIRTIO, 0x44, __u32);

/* VHOST_VSOCK specific defines */

pub const VHOST_VSOCK_SET_GUEST_CID: u64 = _IOW!(VHOST_VIRTIO, 0x60, __u64);
pub const VHOST_VSOCK_SET_RUNNING: u64 = _IOW!(VHOST_VIRTIO, 0x61, i32);

/* VHOST_VDPA specific defines */

/* Get the device id. The device ids follow the same definition of
 * the device id defined in virtio-spec.
 */
pub const VHOST_VDPA_GET_DEVICE_ID: u64 = _IOR!(VHOST_VIRTIO, 0x70, __u32);
/* Get and set the status. The status bits follow the same definition
 * of the device status defined in virtio-spec.
 */
pub const VHOST_VDPA_GET_STATUS: u64 = _IOR!(VHOST_VIRTIO, 0x71, __u8);
pub const VHOST_VDPA_SET_STATUS: u64 = _IOW!(VHOST_VIRTIO, 0x72, __u8);
/* Get and set the device config. The device config follows the same
 * definition of the device config defined in virtio-spec.
 */
pub const VHOST_VDPA_GET_CONFIG: u64 = _IOR!(VHOST_VIRTIO, 0x73, vhost_vdpa_config);
pub const VHOST_VDPA_SET_CONFIG: u64 = _IOW!(VHOST_VIRTIO, 0x74, vhost_vdpa_config);
/* Enable/disable the ring. */
pub const VHOST_VDPA_SET_VRING_ENABLE: u64 = _IOW!(VHOST_VIRTIO, 0x75, vhost_vring_state);
/* Get the max ring size. */
pub const VHOST_VDPA_GET_VRING_NUM: u64 = _IOR!(VHOST_VIRTIO, 0x76, __u16);

/* Set event fd for config interrupt*/
pub const VHOST_VDPA_SET_CONFIG_CALL: u64 = _IOW!(VHOST_VIRTIO, 0x77, i32);

/* Get the valid iova range */
pub const VHOST_VDPA_GET_IOVA_RANGE: u64 = _IOR!(VHOST_VIRTIO, 0x78, vhost_vdpa_iova_range);
/* Get the config size */
pub const VHOST_VDPA_GET_CONFIG_SIZE: u64 = _IOR!(VHOST_VIRTIO, 0x79, __u32);

/* Get the number of address spaces. */
pub const VHOST_VDPA_GET_AS_NUM: u64 = _IOR!(VHOST_VIRTIO, 0x7A, u32);

/* Get the group for a virtqueue: read index, write group in num,
 * The virtqueue index is stored in the index field of
 * vhost_vring_state. The group for this specific virtqueue is
 * returned via num field of vhost_vring_state.
 */
pub const VHOST_VDPA_GET_VRING_GROUP: u64 = _IOWR!(VHOST_VIRTIO, 0x7B, vhost_vring_state);
/* Set the ASID for a virtqueue group. The group index is stored in
 * the index field of vhost_vring_state, the ASID associated with this
 * group is stored at num field of vhost_vring_state.
 */
pub const VHOST_VDPA_SET_GROUP_ASID: u64 = _IOW!(VHOST_VIRTIO, 0x7C, vhost_vring_state);

/* Suspend a device so it does not process virtqueue requests anymore
 *
 * After the return of ioctl the device must preserve all the necessary state
 * (the virtqueue vring base plus the possible device specific states) that is
 * required for restoring in the future. The device must not change its
 * configuration after that point.
 */
pub const VHOST_VDPA_SUSPEND: u64 = _IO!(VHOST_VIRTIO, 0x7D);

/* Resume a device so it can resume processing virtqueue requests
 *
 * After the return of this ioctl the device will have restored all the
 * necessary states and it is fully operational to continue processing the
 * virtqueue descriptors.
 */
pub const VHOST_VDPA_RESUME: u64 = _IO!(VHOST_VIRTIO, 0x7E);

/* Get the group for the descriptor table including driver & device areas
 * of a virtqueue: read index, write group in num.
 * The virtqueue index is stored in the index field of vhost_vring_state.
 * The group ID of the descriptor table for this specific virtqueue
 * is returned via num field of vhost_vring_state.
 */
pub const VHOST_VDPA_GET_VRING_DESC_GROUP: u64 =
    _IOWR!(VHOST_VIRTIO, 0x7F, vhost_vring_state);

/* Get the count of all virtqueues */
pub const VHOST_VDPA_GET_VQS_COUNT: u64 = _IOR!(VHOST_VIRTIO, 0x80, __u32);

/* Get the number of virtqueue groups. */
pub const VHOST_VDPA_GET_GROUP_NUM: u64 = _IOR!(VHOST_VIRTIO, 0x81, __u32);

/* Get the queue size of a specific virtqueue.
 * userspace set the vring index in vhost_vring_state.index
 * kernel set the queue size in vhost_vring_state.num
 */
pub const VHOST_VDPA_GET_VRING_SIZE: u64 = _IOWR!(VHOST_VIRTIO, 0x82, vhost_vring_state);

/* Extended features manipulation */
pub const VHOST_GET_FEATURES_ARRAY: u64 = _IOR!(VHOST_VIRTIO, 0x83, vhost_features_array);
pub const VHOST_SET_FEATURES_ARRAY: u64 = _IOW!(VHOST_VIRTIO, 0x83, vhost_features_array);

/* fork_owner values for vhost */
pub const VHOST_FORK_OWNER_KTHREAD: u32 = 0;
pub const VHOST_FORK_OWNER_TASK: u32 = 1;

/**
 * VHOST_SET_FORK_FROM_OWNER - Set the fork_owner flag for the vhost device,
 * This ioctl must called before VHOST_SET_OWNER.
 * Only available when CONFIG_VHOST_ENABLE_FORK_OWNER_CONTROL=y
 *
 * @param fork_owner: An 8-bit value that determines the vhost thread mode
 *
 * When fork_owner is set to VHOST_FORK_OWNER_TASK(default value):
 *   - Vhost will create vhost worker as tasks forked from the owner,
 *     inheriting all of the owner's attributes.
 *
 * When fork_owner is set to VHOST_FORK_OWNER_KTHREAD:
 *   - Vhost will create vhost workers as kernel threads.
 */
pub const VHOST_SET_FORK_FROM_OWNER: u64 = _IOW!(VHOST_VIRTIO, 0x84, __u8);

/**
 * VHOST_GET_FORK_OWNER - Get the current fork_owner flag for the vhost device.
 * Only available when CONFIG_VHOST_ENABLE_FORK_OWNER_CONTROL=y
 *
 * @return: An 8-bit value indicating the current thread mode.
 */
pub const VHOST_GET_FORK_FROM_OWNER: u64 = _IOR!(VHOST_VIRTIO, 0x85, __u8);
