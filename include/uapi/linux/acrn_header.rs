/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Userspace interface for /dev/acrn_hsm - ACRN Hypervisor Service Module */

// `__u*` and `__le*` correspond to the Linux types supplied by the included
// kernel headers.
pub const ACRN_IO_REQUEST_MAX: usize = 16;
pub const ACRN_IOREQ_STATE_PENDING: u32 = 0;
pub const ACRN_IOREQ_STATE_COMPLETE: u32 = 1;
pub const ACRN_IOREQ_STATE_PROCESSING: u32 = 2;
pub const ACRN_IOREQ_STATE_FREE: u32 = 3;
pub const ACRN_IOREQ_TYPE_PORTIO: u32 = 0;
pub const ACRN_IOREQ_TYPE_MMIO: u32 = 1;
pub const ACRN_IOREQ_TYPE_PCICFG: u32 = 2;
pub const ACRN_IOREQ_DIR_READ: u32 = 0;
pub const ACRN_IOREQ_DIR_WRITE: u32 = 1;

#[repr(C)]
pub struct acrn_mmio_request { pub direction: u32, pub reserved: u32, pub address: u64, pub size: u64, pub value: u64 }
#[repr(C)]
pub struct acrn_pio_request { pub direction: u32, pub reserved: u32, pub address: u64, pub size: u64, pub value: u32 }
#[repr(C)]
pub struct acrn_pci_request { pub direction: u32, pub reserved: [u32; 3], pub size: u64, pub value: u32, pub bus: u32, pub dev: u32, pub func: u32, pub reg: u32 }

#[repr(C)]
pub union acrn_io_request_reqs { pub pio_request: acrn_pio_request, pub pci_request: acrn_pci_request, pub mmio_request: acrn_mmio_request, pub data: [u64; 8] }
#[repr(C, align(256))]
pub struct acrn_io_request {
    pub type_: u32, pub completion_polling: u32, pub reserved0: [u32; 14],
    pub reqs: acrn_io_request_reqs, pub reserved1: u32, pub kernel_handled: u32, pub processed: u32,
}
#[repr(C)]
pub union acrn_io_request_buffer_union { pub req_slot: [acrn_io_request; ACRN_IO_REQUEST_MAX], pub reserved: [u8; 4096] }
#[repr(C)]
pub struct acrn_io_request_buffer { pub _anon: acrn_io_request_buffer_union }

#[repr(C)] pub struct acrn_ioreq_notify { pub vmid: u16, pub reserved: u16, pub vcpu: u32 }
#[repr(C)] pub struct acrn_vm_creation { pub vmid: u16, pub reserved0: u16, pub vcpu_num: u16, pub reserved1: u16, pub uuid: [u8; 16], pub vm_flag: u64, pub ioreq_buf: u64, pub cpu_affinity: u64 }
#[repr(C)] pub struct acrn_gp_regs { pub rax:u64,pub rcx:u64,pub rdx:u64,pub rbx:u64,pub rsp:u64,pub rbp:u64,pub rsi:u64,pub rdi:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64 }
#[repr(C, packed)] pub struct acrn_descriptor_ptr { pub limit:u16, pub base:u64, pub reserved:[u16;3] }
#[repr(C)] pub struct acrn_regs { pub gprs:acrn_gp_regs,pub gdt:acrn_descriptor_ptr,pub idt:acrn_descriptor_ptr,pub rip:u64,pub cs_base:u64,pub cr0:u64,pub cr4:u64,pub cr3:u64,pub ia32_efer:u64,pub rflags:u64,pub reserved_64:[u64;4],pub cs_ar:u32,pub cs_limit:u32,pub reserved_32:[u32;3],pub cs_sel:u16,pub ss_sel:u16,pub ds_sel:u16,pub es_sel:u16,pub fs_sel:u16,pub gs_sel:u16,pub ldt_sel:u16,pub tr_sel:u16 }
#[repr(C)] pub struct acrn_vcpu_regs { pub vcpu_id:u16,pub reserved:[u16;3],pub vcpu_regs:acrn_regs }

pub const ACRN_MEM_ACCESS_RIGHT_MASK:u32=0x00000007; pub const ACRN_MEM_ACCESS_READ:u32=1; pub const ACRN_MEM_ACCESS_WRITE:u32=2; pub const ACRN_MEM_ACCESS_EXEC:u32=4; pub const ACRN_MEM_ACCESS_RWX:u32=ACRN_MEM_ACCESS_READ|ACRN_MEM_ACCESS_WRITE|ACRN_MEM_ACCESS_EXEC;
pub const ACRN_MEM_TYPE_MASK:u32=0x000007C0; pub const ACRN_MEM_TYPE_WB:u32=0x40; pub const ACRN_MEM_TYPE_WT:u32=0x80; pub const ACRN_MEM_TYPE_UC:u32=0x100; pub const ACRN_MEM_TYPE_WC:u32=0x200; pub const ACRN_MEM_TYPE_WP:u32=0x400;
pub const ACRN_MEMMAP_RAM:u32=0; pub const ACRN_MEMMAP_MMIO:u32=1;
#[repr(C)] pub union acrn_vm_memmap_addr { pub service_vm_pa:u64, pub vma_base:u64 }
#[repr(C)] pub struct acrn_vm_memmap { pub type_:u32,pub attr:u32,pub user_vm_pa:u64,pub _anon:acrn_vm_memmap_addr,pub len:u64 }
pub const ACRN_PTDEV_IRQ_INTX:u32=0; pub const ACRN_PTDEV_IRQ_MSI:u32=1; pub const ACRN_PTDEV_IRQ_MSIX:u32=2;
#[repr(C)] pub struct acrn_ptdev_irq_intx { pub virt_pin:u32,pub phys_pin:u32,pub is_pic_pin:u32 }
#[repr(C)] pub struct acrn_ptdev_irq { pub type_:u32,pub virt_bdf:u16,pub phys_bdf:u16,pub intx:acrn_ptdev_irq_intx }
pub const ACRN_PTDEV_QUIRK_ASSIGN:u32=1<<0; pub const ACRN_MMIODEV_RES_NUM:usize=3; pub const ACRN_PCI_NUM_BARS:usize=6;
#[repr(C)] pub struct acrn_pcidev { pub type_:u32,pub virt_bdf:u16,pub phys_bdf:u16,pub intr_line:u8,pub intr_pin:u8,pub bar:[u32;ACRN_PCI_NUM_BARS] }
#[repr(C)] pub struct acrn_mmio_dev_res { pub user_vm_pa:u64,pub service_vm_pa:u64,pub size:u64,pub mem_type:u64 }
#[repr(C)] pub struct acrn_mmiodev { pub name:[u8;8],pub res:[acrn_mmio_dev_res;ACRN_MMIODEV_RES_NUM] }
#[repr(C)] pub union acrn_vdev_id { pub value:u64, pub fields:acrn_vdev_id_fields }
#[repr(C)] pub struct acrn_vdev_id_fields { pub vendor:u16,pub device:u16,pub legacy_id:u32 }
#[repr(C)] pub struct acrn_vdev { pub id:acrn_vdev_id,pub slot:u64,pub io_addr:[u32;ACRN_PCI_NUM_BARS],pub io_size:[u32;ACRN_PCI_NUM_BARS],pub args:[u8;128] }
#[repr(C)] pub struct acrn_msi_entry { pub msi_addr:u64,pub msi_data:u64 }
#[repr(C, packed)] pub struct acrn_acpi_generic_address { pub space_id:u8,pub bit_width:u8,pub bit_offset:u8,pub access_size:u8,pub address:u64 }
#[repr(C)] pub struct acrn_cstate_data { pub cx_reg:acrn_acpi_generic_address,pub type_:u8,pub latency:u32,pub power:u64 }
#[repr(C)] pub struct acrn_pstate_data { pub core_frequency:u64,pub power:u64,pub transition_latency:u64,pub bus_master_latency:u64,pub control:u64,pub status:u64 }
pub const PMCMD_TYPE_MASK:u32=0x000000ff;
#[repr(u32)] pub enum acrn_pm_cmd_type { ACRN_PMCMD_GET_PX_CNT, ACRN_PMCMD_GET_PX_DATA, ACRN_PMCMD_GET_CX_CNT, ACRN_PMCMD_GET_CX_DATA }
pub const ACRN_IOEVENTFD_FLAG_PIO:u32=1; pub const ACRN_IOEVENTFD_FLAG_DATAMATCH:u32=2; pub const ACRN_IOEVENTFD_FLAG_DEASSIGN:u32=4;
#[repr(C)] pub struct acrn_ioeventfd { pub fd:u32,pub flags:u32,pub addr:u64,pub len:u32,pub reserved:u32,pub data:u64 }
pub const ACRN_IRQFD_FLAG_DEASSIGN:u32=1;
#[repr(C)] pub struct acrn_irqfd { pub fd:i32,pub flags:u32,pub msi:acrn_msi_entry }

pub const ACRN_IOCTL_TYPE:u32=0xA2;
// The ioctl constants use the Linux `_IO`, `_IOW`, and `_IOWR` encoding macros.
// They are kept as declarations because those architecture-dependent macros are
// supplied by the Linux ioctl headers.
pub const ACRN_IOCTL_CREATE_VM: u32 = _IOWR!(ACRN_IOCTL_TYPE, 0x10, acrn_vm_creation);
pub const ACRN_IOCTL_DESTROY_VM: u32 = _IO!(ACRN_IOCTL_TYPE, 0x11);
pub const ACRN_IOCTL_START_VM: u32 = _IO!(ACRN_IOCTL_TYPE, 0x12);
pub const ACRN_IOCTL_PAUSE_VM: u32 = _IO!(ACRN_IOCTL_TYPE, 0x13);
pub const ACRN_IOCTL_RESET_VM: u32 = _IO!(ACRN_IOCTL_TYPE, 0x15);
pub const ACRN_IOCTL_SET_VCPU_REGS: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x16, acrn_vcpu_regs);
pub const ACRN_IOCTL_INJECT_MSI: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x23, acrn_msi_entry);
pub const ACRN_IOCTL_VM_INTR_MONITOR: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x24, usize);
pub const ACRN_IOCTL_SET_IRQLINE: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x25, u64);
pub const ACRN_IOCTL_NOTIFY_REQUEST_FINISH: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x31, acrn_ioreq_notify);
pub const ACRN_IOCTL_CREATE_IOREQ_CLIENT: u32 = _IO!(ACRN_IOCTL_TYPE, 0x32);
pub const ACRN_IOCTL_ATTACH_IOREQ_CLIENT: u32 = _IO!(ACRN_IOCTL_TYPE, 0x33);
pub const ACRN_IOCTL_DESTROY_IOREQ_CLIENT: u32 = _IO!(ACRN_IOCTL_TYPE, 0x34);
pub const ACRN_IOCTL_CLEAR_VM_IOREQ: u32 = _IO!(ACRN_IOCTL_TYPE, 0x35);
pub const ACRN_IOCTL_SET_MEMSEG: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x41, acrn_vm_memmap);
pub const ACRN_IOCTL_UNSET_MEMSEG: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x42, acrn_vm_memmap);
pub const ACRN_IOCTL_SET_PTDEV_INTR: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x53, acrn_ptdev_irq);
pub const ACRN_IOCTL_RESET_PTDEV_INTR: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x54, acrn_ptdev_irq);
pub const ACRN_IOCTL_ASSIGN_PCIDEV: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x55, acrn_pcidev);
pub const ACRN_IOCTL_DEASSIGN_PCIDEV: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x56, acrn_pcidev);
pub const ACRN_IOCTL_ASSIGN_MMIODEV: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x57, acrn_mmiodev);
pub const ACRN_IOCTL_DEASSIGN_MMIODEV: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x58, acrn_mmiodev);
pub const ACRN_IOCTL_CREATE_VDEV: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x59, acrn_vdev);
pub const ACRN_IOCTL_DESTROY_VDEV: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x5A, acrn_vdev);
pub const ACRN_IOCTL_PM_GET_CPU_STATE: u32 = _IOWR!(ACRN_IOCTL_TYPE, 0x60, u64);
pub const ACRN_IOCTL_IOEVENTFD: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x70, acrn_ioeventfd);
pub const ACRN_IOCTL_IRQFD: u32 = _IOW!(ACRN_IOCTL_TYPE, 0x71, acrn_irqfd);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
