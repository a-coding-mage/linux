/* amdgpu_drm.h -- Public header for the amdgpu driver -*- linux-c -*-
 *
 * Copyright 2000 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Fremont, California.
 * Copyright 2002 Tungsten Graphics, Inc., Cedar Park, Texas.
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors:
 *    Kevin E. Martin <martin@valinux.com>
 *    Gareth Hughes <gareth@valinux.com>
 *    Keith Whitwell <keith@tungstengraphics.com>
 */

// conditional C linkage
#define __AMDGPU_DRM_H__

// dependency: drm.h

// conditional C linkage
extern "C" {


pub const DRM_AMDGPU_GEM_CREATE: u64 = 0x00;pub const DRM_AMDGPU_GEM_MMAP: u64 = 0x01;pub const DRM_AMDGPU_CTX: u64 = 0x02;pub const DRM_AMDGPU_BO_LIST: u64 = 0x03;pub const DRM_AMDGPU_CS: u64 = 0x04;pub const DRM_AMDGPU_INFO: u64 = 0x05;pub const DRM_AMDGPU_GEM_METADATA: u64 = 0x06;pub const DRM_AMDGPU_GEM_WAIT_IDLE: u64 = 0x07;pub const DRM_AMDGPU_GEM_VA: u64 = 0x08;pub const DRM_AMDGPU_WAIT_CS: u64 = 0x09;pub const DRM_AMDGPU_GEM_OP: u64 = 0x10;pub const DRM_AMDGPU_GEM_USERPTR: u64 = 0x11;pub const DRM_AMDGPU_WAIT_FENCES: u64 = 0x12;pub const DRM_AMDGPU_VM: u64 = 0x13;pub const DRM_AMDGPU_FENCE_TO_HANDLE: u64 = 0x14;pub const DRM_AMDGPU_SCHED: u64 = 0x15;pub const DRM_AMDGPU_USERQ: u64 = 0x16;pub const DRM_AMDGPU_USERQ_SIGNAL: u64 = 0x17;pub const DRM_AMDGPU_USERQ_WAIT: u64 = 0x18;pub const DRM_AMDGPU_GEM_LIST_HANDLES: u64 = 0x19;pub const DRM_AMDGPU_PROC_OPTIONS: u64 = 0x1A;
pub const DRM_IOCTL_AMDGPU_GEM_CREATE: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_CREATE, union drm_amdgpu_gem_create);pub const DRM_IOCTL_AMDGPU_GEM_MMAP: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_MMAP, union drm_amdgpu_gem_mmap);pub const DRM_IOCTL_AMDGPU_CTX: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_CTX, union drm_amdgpu_ctx);pub const DRM_IOCTL_AMDGPU_BO_LIST: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_BO_LIST, union drm_amdgpu_bo_list);pub const DRM_IOCTL_AMDGPU_CS: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_CS, union drm_amdgpu_cs);pub const DRM_IOCTL_AMDGPU_INFO: u64 = DRM_IOW(DRM_COMMAND_BASE + DRM_AMDGPU_INFO, struct drm_amdgpu_info);pub const DRM_IOCTL_AMDGPU_GEM_METADATA: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_METADATA, struct drm_amdgpu_gem_metadata);pub const DRM_IOCTL_AMDGPU_GEM_WAIT_IDLE: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_WAIT_IDLE, union drm_amdgpu_gem_wait_idle);pub const DRM_IOCTL_AMDGPU_GEM_VA: u64 = DRM_IOW(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_VA, struct drm_amdgpu_gem_va);pub const DRM_IOCTL_AMDGPU_WAIT_CS: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_WAIT_CS, union drm_amdgpu_wait_cs);pub const DRM_IOCTL_AMDGPU_GEM_OP: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_OP, struct drm_amdgpu_gem_op);pub const DRM_IOCTL_AMDGPU_GEM_USERPTR: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_USERPTR, struct drm_amdgpu_gem_userptr);pub const DRM_IOCTL_AMDGPU_WAIT_FENCES: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_WAIT_FENCES, union drm_amdgpu_wait_fences);pub const DRM_IOCTL_AMDGPU_VM: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_VM, union drm_amdgpu_vm);pub const DRM_IOCTL_AMDGPU_FENCE_TO_HANDLE: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_FENCE_TO_HANDLE, union drm_amdgpu_fence_to_handle);pub const DRM_IOCTL_AMDGPU_SCHED: u64 = DRM_IOW(DRM_COMMAND_BASE + DRM_AMDGPU_SCHED, union drm_amdgpu_sched);pub const DRM_IOCTL_AMDGPU_USERQ: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ, union drm_amdgpu_userq);pub const DRM_IOCTL_AMDGPU_USERQ_SIGNAL: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ_SIGNAL, struct drm_amdgpu_userq_signal);pub const DRM_IOCTL_AMDGPU_USERQ_WAIT: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_USERQ_WAIT, struct drm_amdgpu_userq_wait);pub const DRM_IOCTL_AMDGPU_GEM_LIST_HANDLES: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_GEM_LIST_HANDLES, struct drm_amdgpu_gem_list_handles);pub const DRM_IOCTL_AMDGPU_PROC_OPTIONS: u64 = DRM_IOWR(DRM_COMMAND_BASE + DRM_AMDGPU_PROC_OPTIONS, struct drm_amdgpu_proc_options);
/**
 * DOC: memory domains
 *
 * %AMDGPU_GEM_DOMAIN_CPU	System memory that is not GPU accessible.
 * Memory in this pool could be swapped out to disk if there is pressure.
 *
 * %AMDGPU_GEM_DOMAIN_GTT	GPU accessible system memory, mapped into the
 * GPU's virtual address space via gart. Gart memory linearizes non-contiguous
 * pages of system memory, allows GPU access system memory in a linearized
 * fashion.
 *
 * %AMDGPU_GEM_DOMAIN_VRAM	Local video memory. For APUs, it is memory
 * carved out by the BIOS.
 *
 * %AMDGPU_GEM_DOMAIN_GDS	Global on-chip data storage used to share data
 * across shader threads.
 *
 * %AMDGPU_GEM_DOMAIN_GWS	Global wave sync, used to synchronize the
 * execution of all the waves on a device.
 *
 * %AMDGPU_GEM_DOMAIN_OA	Ordered append, used by 3D or Compute engines
 * for appending data.
 *
 * %AMDGPU_GEM_DOMAIN_DOORBELL	Doorbell. It is an MMIO region for
 * signalling user mode queues.
 */
pub const AMDGPU_GEM_DOMAIN_CPU: u64 = 0x1;pub const AMDGPU_GEM_DOMAIN_GTT: u64 = 0x2;pub const AMDGPU_GEM_DOMAIN_VRAM: u64 = 0x4;pub const AMDGPU_GEM_DOMAIN_GDS: u64 = 0x8;pub const AMDGPU_GEM_DOMAIN_GWS: u64 = 0x10;pub const AMDGPU_GEM_DOMAIN_OA: u64 = 0x20;pub const AMDGPU_GEM_DOMAIN_DOORBELL: u64 = 0x40;pub const AMDGPU_GEM_DOMAIN_MASK: u64 = (AMDGPU_GEM_DOMAIN_CPU | \;					 AMDGPU_GEM_DOMAIN_GTT | \
					 AMDGPU_GEM_DOMAIN_VRAM | \
					 AMDGPU_GEM_DOMAIN_GDS | \
					 AMDGPU_GEM_DOMAIN_GWS | \
					 AMDGPU_GEM_DOMAIN_OA |	\
					 AMDGPU_GEM_DOMAIN_DOORBELL)

pub const AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED: u64 = (1 << 0);pub const AMDGPU_GEM_CREATE_NO_CPU_ACCESS: u64 = (1 << 1);pub const AMDGPU_GEM_CREATE_CPU_GTT_USWC: u64 = (1 << 2);pub const AMDGPU_GEM_CREATE_VRAM_CLEARED: u64 = (1 << 3);pub const AMDGPU_GEM_CREATE_VRAM_CONTIGUOUS: u64 = (1 << 5);pub const AMDGPU_GEM_CREATE_VM_ALWAYS_VALID: u64 = (1 << 6);pub const AMDGPU_GEM_CREATE_EXPLICIT_SYNC: u64 = (1 << 7);/* Flag that indicates allocating MQD gart on GFX9, where the mtype
 * for the second page onward should be set to NC. It should never
 * be used by user space applications.
 */
pub const AMDGPU_GEM_CREATE_CP_MQD_GFX9: u64 = (1 << 8);/* Flag that BO may contain sensitive data that must be wiped before
 * releasing the memory
 */
pub const AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE: u64 = (1 << 9);/* Flag that BO will be encrypted and that the TMZ bit should be
 * set in the PTEs when mapping this buffer via GPUVM or
 * accessing it with various hw blocks
 */
pub const AMDGPU_GEM_CREATE_ENCRYPTED: u64 = (1 << 10);/* Flag that BO will be used only in preemptible context, which does
 * not require GTT memory accounting
 */
pub const AMDGPU_GEM_CREATE_PREEMPTIBLE: u64 = (1 << 11);/* Flag that BO can be discarded under memory pressure without keeping the
 * content.
 */
pub const AMDGPU_GEM_CREATE_DISCARDABLE: u64 = (1 << 12);/* Flag that BO is shared coherently between multiple devices or CPU threads.
 * May depend on GPU instructions to flush caches to system scope explicitly.
 *
 * This influences the choice of MTYPE in the PTEs on GFXv9 and later GPUs and
 * may override the MTYPE selected in AMDGPU_VA_OP_MAP.
 */
pub const AMDGPU_GEM_CREATE_COHERENT: u64 = (1 << 13);/* Flag that BO should not be cached by GPU. Coherent without having to flush
 * GPU caches explicitly
 *
 * This influences the choice of MTYPE in the PTEs on GFXv9 and later GPUs and
 * may override the MTYPE selected in AMDGPU_VA_OP_MAP.
 */
pub const AMDGPU_GEM_CREATE_UNCACHED: u64 = (1 << 14);/* Flag that BO should be coherent across devices when using device-level
 * atomics. May depend on GPU instructions to flush caches to device scope
 * explicitly, promoting them to system scope automatically.
 *
 * This influences the choice of MTYPE in the PTEs on GFXv9 and later GPUs and
 * may override the MTYPE selected in AMDGPU_VA_OP_MAP.
 */
pub const AMDGPU_GEM_CREATE_EXT_COHERENT: u64 = (1 << 15);pub const AMDGPU_GEM_CREATE_GFX12_DCC: u64 = (1 << 16);
#[repr(C)]
pub struct drm_amdgpu_gem_create_in {
	u64 bo_size;	u64 alignment;	u64 domains;	u64 domain_flags;};
#[repr(C)]
pub struct drm_amdgpu_gem_create_out {
	u32 handle;	u32 _pad;};
#[repr(C)]
pub union drm_amdgpu_gem_create {
	struct drm_amdgpu_gem_create_in		in;	struct drm_amdgpu_gem_create_out	out;};
pub const AMDGPU_BO_LIST_OP_CREATE: u64 = 0;pub const AMDGPU_BO_LIST_OP_DESTROY: u64 = 1;pub const AMDGPU_BO_LIST_OP_UPDATE: u64 = 2;
#[repr(C)]
pub struct drm_amdgpu_bo_list_in {
	u32 operation;	u32 list_handle;	u32 bo_number;	u32 bo_info_size;	u64 bo_info_ptr;};
#[repr(C)]
pub struct drm_amdgpu_bo_list_entry {
	u32 bo_handle;	u32 bo_priority;};
#[repr(C)]
pub struct drm_amdgpu_bo_list_out {
	u32 list_handle;	u32 _pad;};
#[repr(C)]
pub union drm_amdgpu_bo_list {
	struct drm_amdgpu_bo_list_in in;	struct drm_amdgpu_bo_list_out out;};
pub const AMDGPU_CTX_OP_ALLOC_CTX: u64 = 1;pub const AMDGPU_CTX_OP_FREE_CTX: u64 = 2;pub const AMDGPU_CTX_OP_QUERY_STATE: u64 = 3;pub const AMDGPU_CTX_OP_QUERY_STATE2: u64 = 4;pub const AMDGPU_CTX_OP_GET_STABLE_PSTATE: u64 = 5;pub const AMDGPU_CTX_OP_SET_STABLE_PSTATE: u64 = 6;
pub const AMDGPU_CTX_NO_RESET: u64 = 0;pub const AMDGPU_CTX_GUILTY_RESET: u64 = 1;pub const AMDGPU_CTX_INNOCENT_RESET: u64 = 2;pub const AMDGPU_CTX_UNKNOWN_RESET: u64 = 3;
pub const AMDGPU_CTX_QUERY2_FLAGS_RESET: u64 = (1<<0);pub const AMDGPU_CTX_QUERY2_FLAGS_VRAMLOST: u64 = (1<<1);pub const AMDGPU_CTX_QUERY2_FLAGS_GUILTY: u64 = (1<<2);pub const AMDGPU_CTX_QUERY2_FLAGS_RAS_CE: u64 = (1<<3);pub const AMDGPU_CTX_QUERY2_FLAGS_RAS_UE: u64 = (1<<4);pub const AMDGPU_CTX_QUERY2_FLAGS_RESET_IN_PROGRESS: u64 = (1<<5);
pub const AMDGPU_CTX_PRIORITY_UNSET: u64 = -2048;pub const AMDGPU_CTX_PRIORITY_VERY_LOW: u64 = -1023;pub const AMDGPU_CTX_PRIORITY_LOW: u64 = -512;pub const AMDGPU_CTX_PRIORITY_NORMAL: u64 = 0;/*
 * When used in struct drm_amdgpu_ctx_in, a priority above NORMAL requires
 * CAP_SYS_NICE or DRM_MASTER
*/
pub const AMDGPU_CTX_PRIORITY_HIGH: u64 = 512;pub const AMDGPU_CTX_PRIORITY_VERY_HIGH: u64 = 1023;
pub const AMDGPU_CTX_STABLE_PSTATE_FLAGS_MASK: u64 = 0xf;pub const AMDGPU_CTX_STABLE_PSTATE_NONE: u64 = 0;pub const AMDGPU_CTX_STABLE_PSTATE_STANDARD: u64 = 1;pub const AMDGPU_CTX_STABLE_PSTATE_MIN_SCLK: u64 = 2;pub const AMDGPU_CTX_STABLE_PSTATE_MIN_MCLK: u64 = 3;pub const AMDGPU_CTX_STABLE_PSTATE_PEAK: u64 = 4;
#[repr(C)]
pub struct drm_amdgpu_ctx_in {
	u32	op;	u32	flags;	u32	ctx_id;	i32	priority;};
#[repr(C)]
pub union drm_amdgpu_ctx_out {
		struct {
			u32	ctx_id;			u32	_pad;		} alloc;
		struct {
			u64	flags;			u32	hangs;			u32	reset_status;		} state;
		struct {
			u32	flags;			u32	_pad;		} pstate;};
#[repr(C)]
pub union drm_amdgpu_ctx {
	struct drm_amdgpu_ctx_in in;	union drm_amdgpu_ctx_out out;};
pub const AMDGPU_USERQ_OP_CREATE: u64 = 1;pub const AMDGPU_USERQ_OP_FREE: u64 = 2;
pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_MASK: u64 = 0x3;pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_SHIFT: u64 = 0;pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_LOW: u64 = 0;pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_LOW: u64 = 1;pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_HIGH: u64 = 2;pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_HIGH: u64 = 3 /* admin only */;pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_SECURE: u64 = (1 << 2);
/*
 * This structure is a container to pass input configuration
 * info for all supported userqueue related operations.
 * For operation AMDGPU_USERQ_OP_CREATE: user is expected
 *  to set all fields, excep the parameter 'queue_id'.
 * For operation AMDGPU_USERQ_OP_FREE: the only input parameter expected
 *  to be set is 'queue_id', eveything else is ignored.
 */
#[repr(C)]
pub struct drm_amdgpu_userq_in {
	u32	op;	u32	queue_id;	u32   ip_type;	/**
	 * @doorbell_handle: the handle of doorbell GEM object
	 * associated with this userqueue client.
	 */
	u32   doorbell_handle;	/**
	 * @doorbell_offset: 32-bit offset of the doorbell in the doorbell bo.
	 * Kernel will generate absolute doorbell offset using doorbell_handle
	 * and doorbell_offset in the doorbell bo.
	 */
	u32   doorbell_offset;	/**
	 * @flags: flags used for queue parameters
	 */
	u32 flags;	/**
	 * @queue_va: Virtual address of the GPU memory which holds the queue
	 * object. The queue holds the workload packets.
	 */
	u64   queue_va;	/**
	 * @queue_size: Size of the queue in bytes, this needs to be 256-byte
	 * aligned.
	 */
	u64   queue_size;	/**
	 * @rptr_va : Virtual address of the GPU memory which holds the ring RPTR.
	 * This object must be at least 8 byte in size and aligned to 8-byte offset.
	 */
	u64   rptr_va;	/**
	 * @wptr_va : Virtual address of the GPU memory which holds the ring WPTR.
	 * This object must be at least 8 byte in size and aligned to 8-byte offset.
	 *
	 * Queue, RPTR and WPTR can come from the same object, as long as the size
	 * and alignment related requirements are met.
	 */
	u64   wptr_va;	/**
	 * @mqd: MQD (memory queue descriptor) is a set of parameters which allow
	 * the GPU to uniquely define and identify a usermode queue.
	 *
	 * MQD data can be of different size for different GPU IP/engine and
	 * their respective versions/revisions, so this points to a u64 *
	 * which holds IP specific MQD of this usermode queue.
	 */
	u64 mqd;	/**
	 * @size: size of MQD data in bytes, it must match the MQD structure
	 * size of the respective engine/revision defined in UAPI for ex, for
	 * gfx11 workloads, size = sizeof(drm_amdgpu_userq_mqd_gfx11).
	 */
	u64 mqd_size;};
#[repr(C)]
pub struct drm_amdgpu_userq_out {
	/**
	 * For operation AMDGPU_USERQ_OP_CREATE: This field contains a unique
	 * queue ID to represent the newly created userqueue in the system, otherwise
	 * it should be ignored.
	 */
	u32	queue_id;	u32 _pad;};
#[repr(C)]
pub union drm_amdgpu_userq {
	struct drm_amdgpu_userq_in in;	struct drm_amdgpu_userq_out out;};
#[repr(C)]
pub struct drm_amdgpu_userq_mqd_gfx11 {
	/**
	 * @shadow_va: Virtual address of the GPU memory to hold the shadow buffer.
	 * Use AMDGPU_INFO_IOCTL to find the exact size of the object.
	 */
	u64   shadow_va;	/**
	 * @csa_va: Virtual address of the GPU memory to hold the CSA buffer.
	 * Use AMDGPU_INFO_IOCTL to find the exact size of the object.
	 */
	u64   csa_va;};
#[repr(C)]
pub struct drm_amdgpu_userq_mqd_sdma_gfx11 {
	/**
	 * @csa_va: Virtual address of the GPU memory to hold the CSA buffer.
	 * This must be a from a separate GPU object, and use AMDGPU_INFO IOCTL
	 * to get the size.
	 */
	u64   csa_va;};
#[repr(C)]
pub struct drm_amdgpu_userq_mqd_compute_gfx11 {
	/**
	 * @eop_va: Virtual address of the GPU memory to hold the EOP buffer.
	 * This must be a from a separate GPU object, and use AMDGPU_INFO IOCTL
	 * to get the size.
	 */
	u64   eop_va;};
#[repr(C)]
pub struct drm_amdgpu_userq_signal {
	/**
	 * @queue_id: Queue handle used by the userq fence creation function
	 * to retrieve the WPTR.
	 */
	u32	queue_id;	u32	pad;	/**
	 * @syncobj_handles: The list of syncobj handles submitted by the user queue
	 * job to be signaled.
	 */
	u64	syncobj_handles;	/**
	 * @num_syncobj_handles: A count that represents the number of syncobj handles in
	 * @syncobj_handles.
	 */
	u16	num_syncobj_handles;	u16	pad0;	u32	pad1;	/**
	 * @bo_read_handles: The list of BO handles that the submitted user queue job
	 * is using for read only. This will update BO fences in the kernel.
	 */
	u64	bo_read_handles;	/**
	 * @bo_write_handles: The list of BO handles that the submitted user queue job
	 * is using for write only. This will update BO fences in the kernel.
	 */
	u64	bo_write_handles;	/**
	 * @num_bo_read_handles: A count that represents the number of read BO handles in
	 * @bo_read_handles.
	 */
	u32	num_bo_read_handles;	/**
	 * @num_bo_write_handles: A count that represents the number of write BO handles in
	 * @bo_write_handles.
	 */
	u32	num_bo_write_handles;};
#[repr(C)]
pub struct drm_amdgpu_userq_fence_info {
	/**
	 * @va: A gpu address allocated for each queue which stores the
	 * read pointer (RPTR) value.
	 */
	u64	va;	/**
	 * @value: A 64 bit value represents the write pointer (WPTR) of the
	 * queue commands which compared with the RPTR value to signal the
	 * fences.
	 */
	u64	value;};
#[repr(C)]
pub struct drm_amdgpu_userq_wait {
	/**
	 * @waitq_id: Queue handle used by the userq wait IOCTL to retrieve the
	 * wait queue and maintain the fence driver references in it.
	 */
	u32	waitq_id;	u32	pad;	/**
	 * @syncobj_handles: The list of syncobj handles submitted by the user queue
	 * job to get the va/value pairs.
	 */
	u64	syncobj_handles;	/**
	 * @syncobj_timeline_handles: The list of timeline syncobj handles submitted by
	 * the user queue job to get the va/value pairs at given @syncobj_timeline_points.
	 */
	u64	syncobj_timeline_handles;	/**
	 * @syncobj_timeline_points: The list of timeline syncobj points submitted by the
	 * user queue job for the corresponding @syncobj_timeline_handles.
	 */
	u64	syncobj_timeline_points;	/**
	 * @bo_read_handles: The list of read BO handles submitted by the user queue
	 * job to get the va/value pairs.
	 */
	u64	bo_read_handles;	/**
	 * @bo_write_handles: The list of write BO handles submitted by the user queue
	 * job to get the va/value pairs.
	 */
	u64	bo_write_handles;	/**
	 * @num_syncobj_timeline_handles: A count that represents the number of timeline
	 * syncobj handles in @syncobj_timeline_handles.
	 */
	u16	num_syncobj_timeline_handles;	/**
	 * @num_fences: This field can be used both as input and output. As input it defines
	 * the maximum number of fences that can be returned and as output it will specify
	 * how many fences were actually returned from the ioctl.
	 */
	u16	num_fences;	/**
	 * @num_syncobj_handles: A count that represents the number of syncobj handles in
	 * @syncobj_handles.
	 */
	u16	num_syncobj_handles;	u16	pad0;	/**
	 * @num_bo_read_handles: A count that represents the number of read BO handles in
	 * @bo_read_handles.
	 */
	u32	num_bo_read_handles;	/**
	 * @num_bo_write_handles: A count that represents the number of write BO handles in
	 * @bo_write_handles.
	 */
	u32	num_bo_write_handles;	/**
	 * @out_fences: The field is a return value from the ioctl containing the list of
	 * address/value pairs to wait for.
	 */
	u64	out_fences;};
pub const AMDGPU_VM_OP_RESERVE_VMID: u64 = 1;pub const AMDGPU_VM_OP_UNRESERVE_VMID: u64 = 2;
#[repr(C)]
pub struct drm_amdgpu_vm_in {
	u32	op;	u32	flags;};
#[repr(C)]
pub struct drm_amdgpu_vm_out {
	u64	flags;};
#[repr(C)]
pub union drm_amdgpu_vm {
	struct drm_amdgpu_vm_in in;	struct drm_amdgpu_vm_out out;};
pub const AMDGPU_SCHED_OP_PROCESS_PRIORITY_OVERRIDE: u64 = 1;pub const AMDGPU_SCHED_OP_CONTEXT_PRIORITY_OVERRIDE: u64 = 2;
#[repr(C)]
pub struct drm_amdgpu_sched_in {
	u32	op;	u32	fd;	i32	priority;	u32   ctx_id;};
#[repr(C)]
pub union drm_amdgpu_sched {
	struct drm_amdgpu_sched_in in;};
/*
 * This is not a reliable API and you should expect it to fail for any
 * number of reasons and have fallback path that do not use userptr to
 * perform any operation.
 */
pub const AMDGPU_GEM_USERPTR_READONLY: u64 = (1 << 0);pub const AMDGPU_GEM_USERPTR_ANONONLY: u64 = (1 << 1);pub const AMDGPU_GEM_USERPTR_VALIDATE: u64 = (1 << 2);pub const AMDGPU_GEM_USERPTR_REGISTER: u64 = (1 << 3);
#[repr(C)]
pub struct drm_amdgpu_gem_userptr {
	u64		addr;	u64		size;	u32		flags;	u32		handle;};
pub const AMDGPU_TILING_ARRAY_MODE_SHIFT: u64 = 0;pub const AMDGPU_TILING_ARRAY_MODE_MASK: u64 = 0xf;pub const AMDGPU_TILING_PIPE_CONFIG_SHIFT: u64 = 4;pub const AMDGPU_TILING_PIPE_CONFIG_MASK: u64 = 0x1f;pub const AMDGPU_TILING_TILE_SPLIT_SHIFT: u64 = 9;pub const AMDGPU_TILING_TILE_SPLIT_MASK: u64 = 0x7;pub const AMDGPU_TILING_MICRO_TILE_MODE_SHIFT: u64 = 12;pub const AMDGPU_TILING_MICRO_TILE_MODE_MASK: u64 = 0x7;pub const AMDGPU_TILING_BANK_WIDTH_SHIFT: u64 = 15;pub const AMDGPU_TILING_BANK_WIDTH_MASK: u64 = 0x3;pub const AMDGPU_TILING_BANK_HEIGHT_SHIFT: u64 = 17;pub const AMDGPU_TILING_BANK_HEIGHT_MASK: u64 = 0x3;pub const AMDGPU_TILING_MACRO_TILE_ASPECT_SHIFT: u64 = 19;pub const AMDGPU_TILING_MACRO_TILE_ASPECT_MASK: u64 = 0x3;pub const AMDGPU_TILING_NUM_BANKS_SHIFT: u64 = 21;pub const AMDGPU_TILING_NUM_BANKS_MASK: u64 = 0x3;
pub const AMDGPU_TILING_SWIZZLE_MODE_SHIFT: u64 = 0;pub const AMDGPU_TILING_SWIZZLE_MODE_MASK: u64 = 0x1f;pub const AMDGPU_TILING_DCC_OFFSET_256B_SHIFT: u64 = 5;pub const AMDGPU_TILING_DCC_OFFSET_256B_MASK: u64 = 0xFFFFFF;pub const AMDGPU_TILING_DCC_PITCH_MAX_SHIFT: u64 = 29;pub const AMDGPU_TILING_DCC_PITCH_MAX_MASK: u64 = 0x3FFF;pub const AMDGPU_TILING_DCC_INDEPENDENT_64B_SHIFT: u64 = 43;pub const AMDGPU_TILING_DCC_INDEPENDENT_64B_MASK: u64 = 0x1;pub const AMDGPU_TILING_DCC_INDEPENDENT_128B_SHIFT: u64 = 44;pub const AMDGPU_TILING_DCC_INDEPENDENT_128B_MASK: u64 = 0x1;pub const AMDGPU_TILING_SCANOUT_SHIFT: u64 = 63;pub const AMDGPU_TILING_SCANOUT_MASK: u64 = 0x1;
pub const AMDGPU_TILING_GFX12_SWIZZLE_MODE_SHIFT: u64 = 0;pub const AMDGPU_TILING_GFX12_SWIZZLE_MODE_MASK: u64 = 0x7;pub const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_SHIFT: u64 = 3;pub const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_MASK: u64 = 0x3 /* 0:64B, 1:128B, 2:256B */;pub const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_SHIFT: u64 = 5;pub const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_MASK: u64 = 0x7 /* CB_COLOR0_INFO.NUMBER_TYPE */;pub const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_SHIFT: u64 = 8;pub const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_MASK: u64 = 0x3f /* [0:4]:CB_COLOR0_INFO.FORMAT, [5]:MM */;/* When clearing the buffer or moving it from VRAM to GTT, don't compress and set DCC metadata
 * to uncompressed. Set when parts of an allocation bypass DCC and read raw data. */
pub const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_SHIFT: u64 = 14;pub const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_MASK: u64 = 0x1;pub const AMDGPU_TILING_GFX12_SCANOUT_SHIFT: u64 = 63;pub const AMDGPU_TILING_GFX12_SCANOUT_MASK: u64 = 0x1;
#define AMDGPU_TILING_SET(field, value) \
	(((u64)(value) & AMDGPU_TILING_##field##_MASK) << AMDGPU_TILING_##field##_SHIFT)
#define AMDGPU_TILING_GET(value, field) \
	(((u64)(value) >> AMDGPU_TILING_##field##_SHIFT) & AMDGPU_TILING_##field##_MASK)

pub const AMDGPU_GEM_METADATA_OP_SET_METADATA: u64 = 1;pub const AMDGPU_GEM_METADATA_OP_GET_METADATA: u64 = 2;
#[repr(C)]
pub struct drm_amdgpu_gem_metadata {
	u32	handle;	u32	op;	struct {
		u64	flags;		u64	tiling_info;		u32	data_size_bytes;		u32	data[64];	} data;};
#[repr(C)]
pub struct drm_amdgpu_gem_mmap_in {
	u32 handle;	u32 _pad;};
#[repr(C)]
pub struct drm_amdgpu_gem_mmap_out {
	u64 addr_ptr;};
#[repr(C)]
pub union drm_amdgpu_gem_mmap {
	struct drm_amdgpu_gem_mmap_in   in;	struct drm_amdgpu_gem_mmap_out out;};
#[repr(C)]
pub struct drm_amdgpu_gem_wait_idle_in {
	u32 handle;	u32 flags;	u64 timeout;};
#[repr(C)]
pub struct drm_amdgpu_gem_wait_idle_out {
	u32 status;	u32 domain;};
#[repr(C)]
pub union drm_amdgpu_gem_wait_idle {
	struct drm_amdgpu_gem_wait_idle_in  in;	struct drm_amdgpu_gem_wait_idle_out out;};
#[repr(C)]
pub struct drm_amdgpu_wait_cs_in {
	/* Command submission handle
         * handle equals 0 means none to wait for
         * handle equals ~0ull means wait for the latest sequence number
         */
	u64 handle;	u64 timeout;	u32 ip_type;	u32 ip_instance;	u32 ring;	u32 ctx_id;};
#[repr(C)]
pub struct drm_amdgpu_wait_cs_out {
	u64 status;};
#[repr(C)]
pub union drm_amdgpu_wait_cs {
	struct drm_amdgpu_wait_cs_in in;	struct drm_amdgpu_wait_cs_out out;};
#[repr(C)]
pub struct drm_amdgpu_fence {
	u32 ctx_id;	u32 ip_type;	u32 ip_instance;	u32 ring;	u64 seq_no;};
#[repr(C)]
pub struct drm_amdgpu_wait_fences_in {
	u64 fences;	u32 fence_count;	u32 wait_all;	u64 timeout_ns;};
#[repr(C)]
pub struct drm_amdgpu_wait_fences_out {
	u32 status;	u32 first_signaled;};
#[repr(C)]
pub union drm_amdgpu_wait_fences {
	struct drm_amdgpu_wait_fences_in in;	struct drm_amdgpu_wait_fences_out out;};
pub const AMDGPU_GEM_OP_GET_GEM_CREATE_INFO: u64 = 0;pub const AMDGPU_GEM_OP_SET_PLACEMENT: u64 = 1;pub const AMDGPU_GEM_OP_GET_MAPPING_INFO: u64 = 2;
#[repr(C)]
pub struct drm_amdgpu_gem_vm_entry {
	u64 addr;
	u64 size;
	u64 offset;
	u64 flags;};
#[repr(C)]
pub struct drm_amdgpu_gem_op {
	u32	handle;	u32	op;	u64	value;	u32	num_entries;
	u32	padding;};
pub const AMDGPU_GEM_LIST_HANDLES_FLAG_IS_IMPORT: u64 = (1 << 0);
#[repr(C)]
pub struct drm_amdgpu_gem_list_handles {
	u64   entries;
	u32   num_entries;
	u32 padding;};
#[repr(C)]
pub struct drm_amdgpu_gem_list_handles_entry {
	u32 gem_handle;
	u32 flags;
	u64 size;
	u64 preferred_domains;
	u64 alloc_flags;
	u64 alignment;};
pub const AMDGPU_VA_OP_MAP: u64 = 1;pub const AMDGPU_VA_OP_UNMAP: u64 = 2;pub const AMDGPU_VA_OP_CLEAR: u64 = 3;pub const AMDGPU_VA_OP_REPLACE: u64 = 4;
pub const AMDGPU_VM_DELAY_UPDATE: u64 = (1 << 0);
pub const AMDGPU_VM_PAGE_READABLE: u64 = (1 << 1);pub const AMDGPU_VM_PAGE_WRITEABLE: u64 = (1 << 2);pub const AMDGPU_VM_PAGE_EXECUTABLE: u64 = (1 << 3);pub const AMDGPU_VM_PAGE_PRT: u64 = (1 << 4);pub const AMDGPU_VM_MTYPE_MASK: u64 = (0xf << 5);pub const AMDGPU_VM_MTYPE_DEFAULT: u64 = (0 << 5);pub const AMDGPU_VM_MTYPE_NC: u64 = (1 << 5);pub const AMDGPU_VM_MTYPE_WC: u64 = (2 << 5);pub const AMDGPU_VM_MTYPE_CC: u64 = (3 << 5);pub const AMDGPU_VM_MTYPE_UC: u64 = (4 << 5);pub const AMDGPU_VM_MTYPE_RW: u64 = (5 << 5);pub const AMDGPU_VM_PAGE_NOALLOC: u64 = (1 << 9);
#[repr(C)]
pub struct drm_amdgpu_gem_va {
	u32 handle;	u32 _pad;	u32 operation;	u32 flags;	u64 va_address;	u64 offset_in_bo;	u64 map_size;	/**
	 * vm_timeline_point is a sequence number used to add new timeline point.
	 */
	u64 vm_timeline_point;	/**
	 * The vm page table update fence is installed in given vm_timeline_syncobj_out
	 * at vm_timeline_point.
	 */
	u32 vm_timeline_syncobj_out;	u32 num_syncobj_handles;	u64 input_fence_syncobj_handles;};
pub const AMDGPU_HW_IP_GFX: u64 = 0;pub const AMDGPU_HW_IP_COMPUTE: u64 = 1;pub const AMDGPU_HW_IP_DMA: u64 = 2;pub const AMDGPU_HW_IP_UVD: u64 = 3;pub const AMDGPU_HW_IP_VCE: u64 = 4;pub const AMDGPU_HW_IP_UVD_ENC: u64 = 5;pub const AMDGPU_HW_IP_VCN_DEC: u64 = 6;/*
 * From VCN4, AMDGPU_HW_IP_VCN_ENC is re-used to support
 * both encoding and decoding jobs.
 */
pub const AMDGPU_HW_IP_VCN_ENC: u64 = 7;pub const AMDGPU_HW_IP_VCN_JPEG: u64 = 8;pub const AMDGPU_HW_IP_VPE: u64 = 9;pub const AMDGPU_HW_IP_NUM: u64 = 10;
pub const AMDGPU_HW_IP_INSTANCE_MAX_COUNT: u64 = 1;
pub const AMDGPU_CHUNK_ID_IB: u64 = 0x01;pub const AMDGPU_CHUNK_ID_FENCE: u64 = 0x02;pub const AMDGPU_CHUNK_ID_DEPENDENCIES: u64 = 0x03;pub const AMDGPU_CHUNK_ID_SYNCOBJ_IN: u64 = 0x04;pub const AMDGPU_CHUNK_ID_SYNCOBJ_OUT: u64 = 0x05;pub const AMDGPU_CHUNK_ID_BO_HANDLES: u64 = 0x06;pub const AMDGPU_CHUNK_ID_SCHEDULED_DEPENDENCIES: u64 = 0x07;pub const AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT: u64 = 0x08;pub const AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL: u64 = 0x09;pub const AMDGPU_CHUNK_ID_CP_GFX_SHADOW: u64 = 0x0a;
#[repr(C)]
pub struct drm_amdgpu_cs_chunk {
	u32		chunk_id;	u32		length_dw;	u64		chunk_data;};
#[repr(C)]
pub struct drm_amdgpu_cs_in {
	u32		ctx_id;	u32		bo_list_handle;	u32		num_chunks;	u32		flags;	u64		chunks;};
#[repr(C)]
pub struct drm_amdgpu_cs_out {
	u64 handle;};
#[repr(C)]
pub union drm_amdgpu_cs {
	struct drm_amdgpu_cs_in in;	struct drm_amdgpu_cs_out out;};

pub const AMDGPU_IB_FLAG_CE: u64 = (1<<0);
pub const AMDGPU_IB_FLAG_PREAMBLE: u64 = (1<<1);
pub const AMDGPU_IB_FLAG_PREEMPT: u64 = (1<<2);
/* The IB fence should do the L2 writeback but not invalidate any shader
 * caches (L2/vL1/sL1/I$). */
pub const AMDGPU_IB_FLAG_TC_WB_NOT_INVALIDATE: u64 = (1 << 3);
/* Set GDS_COMPUTE_MAX_WAVE_ID = DEFAULT before PACKET3_INDIRECT_BUFFER.
 * This will reset wave ID counters for the IB.
 */
pub const AMDGPU_IB_FLAG_RESET_GDS_MAX_WAVE_ID: u64 = (1 << 4);
/* Flag the IB as secure (TMZ)
 */
pub const AMDGPU_IB_FLAGS_SECURE: u64 = (1 << 5);
/* Tell KMD to flush and invalidate caches
 */
pub const AMDGPU_IB_FLAG_EMIT_MEM_SYNC: u64 = (1 << 6);
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_ib {
	u32 _pad;	u32 flags;	u64 va_start;	u32 ib_bytes;	u32 ip_type;	u32 ip_instance;	u32 ring;};
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_dep {
	u32 ip_type;	u32 ip_instance;	u32 ring;	u32 ctx_id;	u64 handle;};
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_fence {
	u32 handle;	u32 offset;};
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_sem {
	u32 handle;};
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_syncobj {
       u32 handle;       u32 flags;       u64 point;};
pub const AMDGPU_FENCE_TO_HANDLE_GET_SYNCOBJ: u64 = 0;pub const AMDGPU_FENCE_TO_HANDLE_GET_SYNCOBJ_FD: u64 = 1;pub const AMDGPU_FENCE_TO_HANDLE_GET_SYNC_FILE_FD: u64 = 2;
#[repr(C)]
pub union drm_amdgpu_fence_to_handle {
	struct {
		struct drm_amdgpu_fence fence;		u32 what;		u32 pad;	} in;	struct {
		u32 handle;	} out;};
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_data {
	union {
		struct drm_amdgpu_cs_chunk_ib		ib_data;		struct drm_amdgpu_cs_chunk_fence	fence_data;	};};
pub const AMDGPU_CS_CHUNK_CP_GFX_SHADOW_FLAGS_INIT_SHADOW: u64 = 0x1;
#[repr(C)]
pub struct drm_amdgpu_cs_chunk_cp_gfx_shadow {
	u64 shadow_va;	u64 csa_va;	u64 gds_va;	u64 flags;};
/*
 *  Query h/w info: Flag that this is integrated (a.h.a. fusion) GPU
 *
 */
pub const AMDGPU_IDS_FLAGS_FUSION: u64 = 0x01;pub const AMDGPU_IDS_FLAGS_PREEMPTION: u64 = 0x02;pub const AMDGPU_IDS_FLAGS_TMZ: u64 = 0x04;pub const AMDGPU_IDS_FLAGS_CONFORMANT_TRUNC_COORD: u64 = 0x08;pub const AMDGPU_IDS_FLAGS_GANG_SUBMIT: u64 = 0x10;
/*
 *  Query h/w info: Flag identifying VF/PF/PT mode
 *
 */
pub const AMDGPU_IDS_FLAGS_MODE_MASK: u64 = 0x300;pub const AMDGPU_IDS_FLAGS_MODE_SHIFT: u64 = 0x8;pub const AMDGPU_IDS_FLAGS_MODE_PF: u64 = 0x0;pub const AMDGPU_IDS_FLAGS_MODE_VF: u64 = 0x1;pub const AMDGPU_IDS_FLAGS_MODE_PT: u64 = 0x2;
pub const AMDGPU_INFO_ACCEL_WORKING: u64 = 0x00;pub const AMDGPU_INFO_CRTC_FROM_ID: u64 = 0x01;pub const AMDGPU_INFO_HW_IP_INFO: u64 = 0x02;pub const AMDGPU_INFO_HW_IP_COUNT: u64 = 0x03;pub const AMDGPU_INFO_TIMESTAMP: u64 = 0x05;pub const AMDGPU_INFO_FW_VERSION: u64 = 0x0e;	#define AMDGPU_INFO_FW_VCE		0x1
	#define AMDGPU_INFO_FW_UVD		0x2
	#define AMDGPU_INFO_FW_GMC		0x03
	#define AMDGPU_INFO_FW_GFX_ME		0x04
	#define AMDGPU_INFO_FW_GFX_PFP		0x05
	#define AMDGPU_INFO_FW_GFX_CE		0x06
	#define AMDGPU_INFO_FW_GFX_RLC		0x07
	#define AMDGPU_INFO_FW_GFX_MEC		0x08
	#define AMDGPU_INFO_FW_SMC		0x0a
	#define AMDGPU_INFO_FW_SDMA		0x0b
	#define AMDGPU_INFO_FW_SOS		0x0c
	#define AMDGPU_INFO_FW_ASD		0x0d
	#define AMDGPU_INFO_FW_VCN		0x0e
	#define AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL 0x0f
	#define AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM 0x10
	#define AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM 0x11
	#define AMDGPU_INFO_FW_DMCU		0x12
	#define AMDGPU_INFO_FW_TA		0x13
	#define AMDGPU_INFO_FW_DMCUB		0x14
	#define AMDGPU_INFO_FW_TOC		0x15
	#define AMDGPU_INFO_FW_CAP		0x16
	#define AMDGPU_INFO_FW_GFX_RLCP		0x17
	#define AMDGPU_INFO_FW_GFX_RLCV		0x18
	#define AMDGPU_INFO_FW_MES_KIQ		0x19
	#define AMDGPU_INFO_FW_MES		0x1a
	#define AMDGPU_INFO_FW_IMU		0x1b
	#define AMDGPU_INFO_FW_VPE		0x1c

pub const AMDGPU_INFO_NUM_BYTES_MOVED: u64 = 0x0f;pub const AMDGPU_INFO_VRAM_USAGE: u64 = 0x10;pub const AMDGPU_INFO_GTT_USAGE: u64 = 0x11;pub const AMDGPU_INFO_GDS_CONFIG: u64 = 0x13;pub const AMDGPU_INFO_VRAM_GTT: u64 = 0x14;pub const AMDGPU_INFO_READ_MMR_REG: u64 = 0x15;pub const AMDGPU_INFO_DEV_INFO: u64 = 0x16;pub const AMDGPU_INFO_VIS_VRAM_USAGE: u64 = 0x17;pub const AMDGPU_INFO_NUM_EVICTIONS: u64 = 0x18;pub const AMDGPU_INFO_MEMORY: u64 = 0x19;pub const AMDGPU_INFO_VCE_CLOCK_TABLE: u64 = 0x1A;pub const AMDGPU_INFO_VBIOS: u64 = 0x1B;	#define AMDGPU_INFO_VBIOS_SIZE		0x1
	#define AMDGPU_INFO_VBIOS_IMAGE		0x2
	#define AMDGPU_INFO_VBIOS_INFO		0x3
pub const AMDGPU_INFO_NUM_HANDLES: u64 = 0x1C;pub const AMDGPU_INFO_SENSOR: u64 = 0x1D;	#define AMDGPU_INFO_SENSOR_GFX_SCLK		0x1
	#define AMDGPU_INFO_SENSOR_GFX_MCLK		0x2
	#define AMDGPU_INFO_SENSOR_GPU_TEMP		0x3
	#define AMDGPU_INFO_SENSOR_GPU_LOAD		0x4
	#define AMDGPU_INFO_SENSOR_GPU_AVG_POWER	0x5
	#define AMDGPU_INFO_SENSOR_VDDNB		0x6
	#define AMDGPU_INFO_SENSOR_VDDGFX		0x7
	#define AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_SCLK		0x8
	#define AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_MCLK		0x9
	#define AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_SCLK			0xa
	#define AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_MCLK			0xb
	#define AMDGPU_INFO_SENSOR_GPU_INPUT_POWER	0xc
pub const AMDGPU_INFO_NUM_VRAM_CPU_PAGE_FAULTS: u64 = 0x1E;pub const AMDGPU_INFO_VRAM_LOST_COUNTER: u64 = 0x1F;pub const AMDGPU_INFO_RAS_ENABLED_FEATURES: u64 = 0x20;pub const AMDGPU_INFO_RAS_ENABLED_UMC: u64 = (1 << 0);pub const AMDGPU_INFO_RAS_ENABLED_SDMA: u64 = (1 << 1);pub const AMDGPU_INFO_RAS_ENABLED_GFX: u64 = (1 << 2);pub const AMDGPU_INFO_RAS_ENABLED_MMHUB: u64 = (1 << 3);pub const AMDGPU_INFO_RAS_ENABLED_ATHUB: u64 = (1 << 4);pub const AMDGPU_INFO_RAS_ENABLED_PCIE: u64 = (1 << 5);pub const AMDGPU_INFO_RAS_ENABLED_HDP: u64 = (1 << 6);pub const AMDGPU_INFO_RAS_ENABLED_XGMI: u64 = (1 << 7);pub const AMDGPU_INFO_RAS_ENABLED_DF: u64 = (1 << 8);pub const AMDGPU_INFO_RAS_ENABLED_SMN: u64 = (1 << 9);pub const AMDGPU_INFO_RAS_ENABLED_SEM: u64 = (1 << 10);pub const AMDGPU_INFO_RAS_ENABLED_MP0: u64 = (1 << 11);pub const AMDGPU_INFO_RAS_ENABLED_MP1: u64 = (1 << 12);pub const AMDGPU_INFO_RAS_ENABLED_FUSE: u64 = (1 << 13);pub const AMDGPU_INFO_VIDEO_CAPS: u64 = 0x21;	#define AMDGPU_INFO_VIDEO_CAPS_DECODE		0
	#define AMDGPU_INFO_VIDEO_CAPS_ENCODE		1
pub const AMDGPU_INFO_MAX_IBS: u64 = 0x22;pub const AMDGPU_INFO_GPUVM_FAULT: u64 = 0x23;pub const AMDGPU_INFO_UQ_FW_AREAS: u64 = 0x24;
pub const AMDGPU_INFO_MMR_SE_INDEX_SHIFT: u64 = 0;pub const AMDGPU_INFO_MMR_SE_INDEX_MASK: u64 = 0xff;pub const AMDGPU_INFO_MMR_SH_INDEX_SHIFT: u64 = 8;pub const AMDGPU_INFO_MMR_SH_INDEX_MASK: u64 = 0xff;
#[repr(C)]
pub struct drm_amdgpu_query_fw {
	u32 fw_type;	/**
	 * Index of the IP if there are more IPs of
	 * the same type.
	 */
	u32 ip_instance;	/**
	 * Index of the engine. Whether this is used depends
	 * on the firmware type. (e.g. MEC, SDMA)
	 */
	u32 index;	u32 _pad;};
#[repr(C)]
pub struct drm_amdgpu_info {
	u64 return_pointer;	/* The size of the return value. Just like "size" in "snprintf",
	 * it limits how many bytes the kernel can write. */
	u32 return_size;	u32 query;
	union {
		struct {
			u32 id;			u32 _pad;		} mode_crtc;
		struct {
			u32 type;			/**
			 * Index of the IP if there are more IPs of the same
			 * type. Ignored by AMDGPU_INFO_HW_IP_COUNT.
			 */
			u32 ip_instance;		} query_hw_ip;
		struct {
			u32 dword_offset;			u32 count;			u32 instance;			u32 flags;		} read_mmr_reg;
		struct drm_amdgpu_query_fw query_fw;
		struct {
			u32 type;			u32 offset;		} vbios_info;
		struct {
			u32 type;		} sensor_info;
		struct {
			u32 type;		} video_cap;	};};
#[repr(C)]
pub struct drm_amdgpu_info_gds {
	u32 gds_gfx_partition_size;	u32 compute_partition_size;	u32 gds_total_size;	u32 gws_per_gfx_partition;	u32 gws_per_compute_partition;	u32 oa_per_gfx_partition;	u32 oa_per_compute_partition;	u32 _pad;};
#[repr(C)]
pub struct drm_amdgpu_info_vram_gtt {
	u64 vram_size;	u64 vram_cpu_accessible_size;	u64 gtt_size;};
#[repr(C)]
pub struct drm_amdgpu_heap_info {
	u64 total_heap_size;
	u64 usable_heap_size;
	/**
	 * Number of bytes allocated in the heap. This includes all processes
	 * and private allocations in the kernel. It changes when new buffers
	 * are allocated, freed, and moved. It cannot be larger than
	 * heap_size.
	 */
	u64 heap_usage;
	/**
	 * Theoretical possible max. size of buffer which
	 * could be allocated in the given heap
	 */
	u64 max_allocation;};
#[repr(C)]
pub struct drm_amdgpu_memory_info {
	struct drm_amdgpu_heap_info vram;	struct drm_amdgpu_heap_info cpu_accessible_vram;	struct drm_amdgpu_heap_info gtt;};
#[repr(C)]
pub struct drm_amdgpu_info_firmware {
	u32 ver;	u32 feature;};
#[repr(C)]
pub struct drm_amdgpu_info_vbios {
	u8 name[64];	u8 vbios_pn[64];	u32 version;	u32 pad;	u8 vbios_ver_str[32];	u8 date[32];};
pub const AMDGPU_VRAM_TYPE_UNKNOWN: u64 = 0;pub const AMDGPU_VRAM_TYPE_GDDR1: u64 = 1;pub const AMDGPU_VRAM_TYPE_DDR2: u64 = 2;pub const AMDGPU_VRAM_TYPE_GDDR3: u64 = 3;pub const AMDGPU_VRAM_TYPE_GDDR4: u64 = 4;pub const AMDGPU_VRAM_TYPE_GDDR5: u64 = 5;pub const AMDGPU_VRAM_TYPE_HBM: u64 = 6;pub const AMDGPU_VRAM_TYPE_DDR3: u64 = 7;pub const AMDGPU_VRAM_TYPE_DDR4: u64 = 8;pub const AMDGPU_VRAM_TYPE_GDDR6: u64 = 9;pub const AMDGPU_VRAM_TYPE_DDR5: u64 = 10;pub const AMDGPU_VRAM_TYPE_LPDDR4: u64 = 11;pub const AMDGPU_VRAM_TYPE_LPDDR5: u64 = 12;pub const AMDGPU_VRAM_TYPE_HBM3E: u64 = 13;pub const AMDGPU_VRAM_TYPE_HBM4: u64 = 14;
#[repr(C)]
pub struct drm_amdgpu_info_device {
	u32 device_id;	u32 chip_rev;	u32 external_rev;	u32 pci_rev;	u32 family;	u32 num_shader_engines;	u32 num_shader_arrays_per_engine;	u32 gpu_counter_freq;	u64 max_engine_clock;	u64 max_memory_clock;	u32 cu_active_number;	u32 cu_ao_mask;	u32 cu_bitmap[4][4];	u32 enabled_rb_pipes_mask;	u32 num_rb_pipes;	u32 num_hw_gfx_contexts;	u32 pcie_gen;	u64 ids_flags;	u64 virtual_address_offset;	u64 virtual_address_max;	u32 virtual_address_alignment;	u32 pte_fragment_size;	u32 gart_page_size;	u32 ce_ram_size;	u32 vram_type;	u32 vram_bit_width;	u32 vce_harvest_config;	u32 gc_double_offchip_lds_buf;	u64 prim_buf_gpu_addr;	u64 pos_buf_gpu_addr;	u64 cntl_sb_buf_gpu_addr;	u64 param_buf_gpu_addr;	u32 prim_buf_size;	u32 pos_buf_size;	u32 cntl_sb_buf_size;	u32 param_buf_size;	u32 wave_front_size;	u32 num_shader_visible_vgprs;	u32 num_cu_per_sh;	u32 num_tcc_blocks;	u32 gs_vgt_table_depth;	u32 gs_prim_buffer_depth;	u32 max_gs_waves_per_vgt;	u32 pcie_num_lanes;	u32 cu_ao_bitmap[4][4];	u64 high_va_offset;	u64 high_va_max;	u32 pa_sc_tile_steering_override;	u64 tcc_disabled_mask;	u64 min_engine_clock;	u64 min_memory_clock;	u32 tcp_cache_size;       /* AKA GL0, VMEM cache */
	u32 num_sqc_per_wgp;	u32 sqc_data_cache_size;  /* AKA SMEM cache */
	u32 sqc_inst_cache_size;	u32 gl1c_cache_size;	u32 gl2c_cache_size;	u64 mall_size;            /* AKA infinity cache */
	u32 enabled_rb_pipes_mask_hi;	u32 shadow_size;	u32 shadow_alignment;	u32 csa_size;	u32 csa_alignment;	u32 userq_ip_mask;	u32 pad;};
#[repr(C)]
pub struct drm_amdgpu_info_hw_ip {
	u32  hw_ip_version_major;	u32  hw_ip_version_minor;	u64  capabilities_flags;	u32  ib_start_alignment;	u32  ib_size_alignment;	u32  available_rings;	u32  ip_discovery_version;	u32  userq_num_slots;};
#[repr(C)]
pub struct drm_amdgpu_info_num_handles {
	u32  uvd_max_handles;	u32  uvd_used_handles;};
pub const AMDGPU_VCE_CLOCK_TABLE_ENTRIES: u64 = 6;
#[repr(C)]
pub struct drm_amdgpu_info_vce_clock_table_entry {
	u32 sclk;	u32 mclk;	u32 eclk;	u32 pad;};
#[repr(C)]
pub struct drm_amdgpu_info_vce_clock_table {
	struct drm_amdgpu_info_vce_clock_table_entry entries[AMDGPU_VCE_CLOCK_TABLE_ENTRIES];	u32 num_valid_entries;	u32 pad;};
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG2: u64 = 0;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4: u64 = 1;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VC1: u64 = 2;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC: u64 = 3;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC: u64 = 4;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG: u64 = 5;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9: u64 = 6;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1: u64 = 7;pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT: u64 = 8;
#[repr(C)]
pub struct drm_amdgpu_info_video_codec_info {
	u32 valid;	u32 max_width;	u32 max_height;	u32 max_pixels_per_frame;	u32 max_level;	u32 pad;};
#[repr(C)]
pub struct drm_amdgpu_info_video_caps {
	struct drm_amdgpu_info_video_codec_info codec_info[AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT];};
pub const AMDGPU_VMHUB_TYPE_MASK: u64 = 0xff;pub const AMDGPU_VMHUB_TYPE_SHIFT: u64 = 0;pub const AMDGPU_VMHUB_TYPE_GFX: u64 = 0;pub const AMDGPU_VMHUB_TYPE_MM0: u64 = 1;pub const AMDGPU_VMHUB_TYPE_MM1: u64 = 2;pub const AMDGPU_VMHUB_IDX_MASK: u64 = 0xff00;pub const AMDGPU_VMHUB_IDX_SHIFT: u64 = 8;
#[repr(C)]
pub struct drm_amdgpu_info_gpuvm_fault {
	u64 addr;	u32 status;	u32 vmhub;};
#[repr(C)]
pub struct drm_amdgpu_info_uq_metadata_gfx {
	u32 shadow_size;	u32 shadow_alignment;	u32 csa_size;	u32 csa_alignment;};
#[repr(C)]
pub struct drm_amdgpu_info_uq_metadata_compute {
	u32 eop_size;	u32 eop_alignment;};
#[repr(C)]
pub struct drm_amdgpu_info_uq_metadata_sdma {
	u32 csa_size;	u32 csa_alignment;};
#[repr(C)]
pub struct drm_amdgpu_info_uq_metadata {
	union {
		struct drm_amdgpu_info_uq_metadata_gfx gfx;		struct drm_amdgpu_info_uq_metadata_compute compute;		struct drm_amdgpu_info_uq_metadata_sdma sdma;	};};
/*
 * Supported GPU families
 */
pub const AMDGPU_FAMILY_UNKNOWN: u64 = 0;pub const AMDGPU_FAMILY_SI: u64 = 110 /* Hainan, Oland, Verde, Pitcairn, Tahiti */;pub const AMDGPU_FAMILY_CI: u64 = 120 /* Bonaire, Hawaii */;pub const AMDGPU_FAMILY_KV: u64 = 125 /* Kaveri, Kabini, Mullins */;pub const AMDGPU_FAMILY_VI: u64 = 130 /* Iceland, Tonga */;pub const AMDGPU_FAMILY_CZ: u64 = 135 /* Carrizo, Stoney */;pub const AMDGPU_FAMILY_AI: u64 = 141 /* Vega10 */;pub const AMDGPU_FAMILY_RV: u64 = 142 /* Raven */;pub const AMDGPU_FAMILY_NV: u64 = 143 /* Navi10 */;pub const AMDGPU_FAMILY_VGH: u64 = 144 /* Van Gogh */;pub const AMDGPU_FAMILY_GC_11_0_0: u64 = 145 /* GC 11.0.0 */;pub const AMDGPU_FAMILY_YC: u64 = 146 /* Yellow Carp */;pub const AMDGPU_FAMILY_GC_11_0_1: u64 = 148 /* GC 11.0.1 */;pub const AMDGPU_FAMILY_GC_10_3_6: u64 = 149 /* GC 10.3.6 */;pub const AMDGPU_FAMILY_GC_10_3_7: u64 = 151 /* GC 10.3.7 */;pub const AMDGPU_FAMILY_GC_11_5_0: u64 = 150 /* GC 11.5.0 */;pub const AMDGPU_FAMILY_GC_11_5_4: u64 = 154 /* GC 11.5.4 */;pub const AMDGPU_FAMILY_GC_12_0_0: u64 = 152 /* GC 12.0.0 */;
/*
 * Definition of user options
 *
 * option: AMDGPU_PROC_OPTIONS_OP_KFD_SIGBUS_DELAY
 *    0:          Disable sigbus delay - SIGBUS will be raised immediately
 *    0xFFFFFFFF: SIGBUS will not be raised
 *    other:      Set the sigbus delay in milliseconds
 */
pub const AMDGPU_PROC_OPTIONS_OP_KFD_SIGBUS_DELAY: u64 = 0;
pub const AMDGPU_PROC_OPTIONS_KFD_SIGBUS_DELAY_DISABLED: u64 = 0xFFFFFFFFu;
#[repr(C)]
pub struct drm_amdgpu_proc_options {
	u32 op;	struct {
		u32 value;	} kfd_sigbus_delay;};
// conditional C linkage
}




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
