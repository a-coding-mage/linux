/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Userspace interfaces for /dev/mshv* devices and derived fds. */

pub const MSHV_IOCTL: u32 = 0xB8;

pub const MSHV_PT_BIT_LAPIC: u32 = 0;
pub const MSHV_PT_BIT_X2APIC: u32 = 1;
pub const MSHV_PT_BIT_GPA_SUPER_PAGES: u32 = 2;
pub const MSHV_PT_BIT_CPU_AND_XSAVE_FEATURES: u32 = 3;
pub const MSHV_PT_BIT_NESTED_VIRTUALIZATION: u32 = 4;
pub const MSHV_PT_BIT_SMT_ENABLED_GUEST: u32 = 5;
pub const MSHV_PT_BIT_COUNT: u32 = 6;
pub const MSHV_PT_FLAGS_MASK: u32 = (1 << MSHV_PT_BIT_COUNT) - 1;

pub const MSHV_PT_ISOLATION_NONE: u32 = 0;
pub const MSHV_PT_ISOLATION_COUNT: u32 = 1;

#[repr(C)]
pub struct mshv_create_partition { pub pt_flags: u64, pub pt_isolation: u64 }

pub const MSHV_NUM_CPU_FEATURES_BANKS: usize = 2;

#[repr(C, packed)]
pub struct mshv_create_partition_v2 {
    pub pt_flags: u64,
    pub pt_isolation: u64,
    pub pt_num_cpu_fbanks: u16,
    pub pt_rsvd: [u8; 6],
    pub pt_cpu_fbanks: [u64; MSHV_NUM_CPU_FEATURES_BANKS],
    pub pt_rsvd1: [u64; 2],
    /* __x86_64__: pt_disabled_xsave; other architectures: pt_rsvd2. */
    pub pt_disabled_xsave_or_rsvd2: u64,
}

#[repr(C)]
pub struct mshv_create_vp { pub vp_index: u32 }

pub const MSHV_SET_MEM_BIT_WRITABLE: u32 = 0;
pub const MSHV_SET_MEM_BIT_EXECUTABLE: u32 = 1;
pub const MSHV_SET_MEM_BIT_UNMAP: u32 = 2;
pub const MSHV_SET_MEM_BIT_COUNT: u32 = 3;
pub const MSHV_SET_MEM_FLAGS_MASK: u32 = (1 << MSHV_SET_MEM_BIT_COUNT) - 1;
pub const MSHV_HV_PAGE_SIZE: u32 = 0x1000;

#[repr(C)]
pub struct mshv_user_mem_region {
    pub size: u64, pub guest_pfn: u64, pub userspace_addr: u64,
    pub flags: u8, pub rsvd: [u8; 7],
}

pub const MSHV_IRQFD_BIT_DEASSIGN: u32 = 0;
pub const MSHV_IRQFD_BIT_RESAMPLE: u32 = 1;
pub const MSHV_IRQFD_BIT_COUNT: u32 = 2;
pub const MSHV_IRQFD_FLAGS_MASK: u32 = (1 << MSHV_IRQFD_BIT_COUNT) - 1;

#[repr(C)]
pub struct mshv_user_irqfd { pub fd: i32, pub resamplefd: i32, pub gsi: u32, pub flags: u32 }

pub const MSHV_IOEVENTFD_BIT_DATAMATCH: u32 = 0;
pub const MSHV_IOEVENTFD_BIT_PIO: u32 = 1;
pub const MSHV_IOEVENTFD_BIT_DEASSIGN: u32 = 2;
pub const MSHV_IOEVENTFD_BIT_COUNT: u32 = 3;
pub const MSHV_IOEVENTFD_FLAGS_MASK: u32 = (1 << MSHV_IOEVENTFD_BIT_COUNT) - 1;

#[repr(C)]
pub struct mshv_user_ioeventfd {
    pub datamatch: u64, pub addr: u64, pub len: u32, pub fd: i32,
    pub flags: u32, pub rsvd: [u8; 4],
}

#[repr(C)]
pub struct mshv_user_irq_entry { pub gsi: u32, pub address_lo: u32, pub address_hi: u32, pub data: u32 }

#[repr(C)]
pub struct mshv_user_irq_table {
    pub nr: u32, pub rsvd: u32, pub entries: [mshv_user_irq_entry; 0],
}

pub const MSHV_GPAP_ACCESS_TYPE_ACCESSED: u32 = 0;
pub const MSHV_GPAP_ACCESS_TYPE_DIRTY: u32 = 1;
pub const MSHV_GPAP_ACCESS_TYPE_COUNT: u32 = 2;
pub const MSHV_GPAP_ACCESS_OP_NOOP: u32 = 0;
pub const MSHV_GPAP_ACCESS_OP_CLEAR: u32 = 1;
pub const MSHV_GPAP_ACCESS_OP_SET: u32 = 2;
pub const MSHV_GPAP_ACCESS_OP_COUNT: u32 = 3;

#[repr(C)]
pub struct mshv_gpap_access_bitmap {
    pub access_type: u8, pub access_op: u8, pub rsvd: [u8; 6],
    pub page_count: u64, pub gpap_base: u64, pub bitmap_ptr: u64,
}

#[repr(C)]
pub struct mshv_root_hvcall {
    pub code: u16, pub reps: u16, pub in_sz: u16, pub out_sz: u16,
    pub status: u16, pub rsvd: [u8; 6], pub in_ptr: u64, pub out_ptr: u64,
}

pub const MSHV_RUN_VP_BUF_SZ: usize = 256;
pub const MSHV_VP_MMAP_OFFSET_REGISTERS: u32 = 0;
pub const MSHV_VP_MMAP_OFFSET_INTERCEPT_MESSAGE: u32 = 1;
pub const MSHV_VP_MMAP_OFFSET_GHCB: u32 = 2;
pub const MSHV_VP_MMAP_OFFSET_COUNT: u32 = 3;

#[repr(C)]
pub struct mshv_run_vp { pub msg_buf: [u8; MSHV_RUN_VP_BUF_SZ] }

pub const MSHV_VP_STATE_LAPIC: u32 = 0;
pub const MSHV_VP_STATE_XSAVE: u32 = 1;
pub const MSHV_VP_STATE_SIMP: u32 = 2;
pub const MSHV_VP_STATE_SIEFP: u32 = 3;
pub const MSHV_VP_STATE_SYNTHETIC_TIMERS: u32 = 4;
pub const MSHV_VP_STATE_COUNT: u32 = 5;

#[repr(C)]
pub struct mshv_get_set_vp_state { pub type_: u8, pub rsvd: [u8; 3], pub buf_sz: u32, pub buf_ptr: u64 }

pub const MSHV_CAP_CORE_API_STABLE: u32 = 0x0;
pub const MSHV_CAP_REGISTER_PAGE: u32 = 0x1;
pub const MSHV_CAP_VTL_RETURN_ACTION: u32 = 0x2;
pub const MSHV_CAP_DR6_SHARED: u32 = 0x3;
pub const MSHV_MAX_RUN_MSG_SIZE: u32 = 256;

#[repr(C)]
pub struct mshv_vp_registers { pub count: u32, pub reserved: u32, pub regs_ptr: u64 }
#[repr(C)]
pub struct mshv_vtl_set_eventfd { pub fd: i32, pub flag: u32 }
#[repr(C)]
pub struct mshv_vtl_signal_event { pub connection_id: u32, pub flag: u32 }
#[repr(C)]
pub struct mshv_vtl_sint_post_msg { pub message_type: u64, pub connection_id: u32, pub payload_size: u32, pub payload_ptr: u64 }
#[repr(C)]
pub struct mshv_vtl_ram_disposition { pub start_pfn: u64, pub last_pfn: u64 }
#[repr(C)]
pub struct mshv_vtl_set_poll_file { pub cpu: u32, pub fd: u32 }
#[repr(C)]
pub struct mshv_vtl_hvcall_setup { pub bitmap_array_size: u64, pub allow_bitmap_ptr: u64 }
#[repr(C)]
pub struct mshv_vtl_hvcall { pub control: u64, pub input_size: u64, pub input_ptr: u64, pub status: u64, pub output_size: u64, pub output_ptr: u64 }
#[repr(C)]
pub struct mshv_sint_mask { pub mask: u8, pub reserved: [u8; 7] }

/* IOCTL definitions retain their source-level encoding and type intent. */
// MSHV_CREATE_PARTITION _IOW(MSHV_IOCTL, 0x00, struct mshv_create_partition)
// MSHV_INITIALIZE_PARTITION _IO(MSHV_IOCTL, 0x00)
// MSHV_CREATE_VP _IOW(MSHV_IOCTL, 0x01, struct mshv_create_vp)
// MSHV_SET_GUEST_MEMORY _IOW(MSHV_IOCTL, 0x02, struct mshv_user_mem_region)
// MSHV_IRQFD _IOW(MSHV_IOCTL, 0x03, struct mshv_user_irqfd)
// MSHV_IOEVENTFD _IOW(MSHV_IOCTL, 0x04, struct mshv_user_ioeventfd)
// MSHV_SET_MSI_ROUTING _IOW(MSHV_IOCTL, 0x05, struct mshv_user_irq_table)
// MSHV_GET_GPAP_ACCESS_BITMAP _IOWR(MSHV_IOCTL, 0x06, struct mshv_gpap_access_bitmap)
// MSHV_ROOT_HVCALL _IOWR(MSHV_IOCTL, 0x07, struct mshv_root_hvcall)
// MSHV_RUN_VP _IOR(MSHV_IOCTL, 0x00, struct mshv_run_vp)
// MSHV_GET_VP_STATE _IOWR(MSHV_IOCTL, 0x01, struct mshv_get_set_vp_state)
// MSHV_SET_VP_STATE _IOWR(MSHV_IOCTL, 0x02, struct mshv_get_set_vp_state)
// MSHV_CHECK_EXTENSION _IOW(MSHV_IOCTL, 0x00, __u32)
// MSHV_CREATE_VTL _IOR(MSHV_IOCTL, 0x1D, char)
// MSHV_ADD_VTL0_MEMORY _IOW(MSHV_IOCTL, 0x21, struct mshv_vtl_ram_disposition)
// MSHV_SET_POLL_FILE _IOW(MSHV_IOCTL, 0x25, struct mshv_vtl_set_poll_file)
// MSHV_RETURN_TO_LOWER_VTL _IO(MSHV_IOCTL, 0x27)
// MSHV_GET_VP_REGISTERS _IOWR(MSHV_IOCTL, 0x05, struct mshv_vp_registers)
// MSHV_SET_VP_REGISTERS _IOW(MSHV_IOCTL, 0x06, struct mshv_vp_registers)
// MSHV_SINT_SIGNAL_EVENT _IOW(MSHV_IOCTL, 0x22, struct mshv_vtl_signal_event)
// MSHV_SINT_POST_MESSAGE _IOW(MSHV_IOCTL, 0x23, struct mshv_vtl_sint_post_msg)
// MSHV_SINT_SET_EVENTFD _IOW(MSHV_IOCTL, 0x24, struct mshv_vtl_set_eventfd)
// MSHV_SINT_PAUSE_MESSAGE_STREAM _IOW(MSHV_IOCTL, 0x25, struct mshv_sint_mask)
// MSHV_HVCALL_SETUP _IOW(MSHV_IOCTL, 0x1E, struct mshv_vtl_hvcall_setup)
// MSHV_HVCALL _IOWR(MSHV_IOCTL, 0x1F, struct mshv_vtl_hvcall)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
