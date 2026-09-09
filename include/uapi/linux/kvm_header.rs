/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// cfg not: __LINUX_KVM_H
pub const __LINUX_KVM_H: u64 = /*;
 * Userspace interface for /dev/kvm - kernel based virtual machine
 *
 * Note: you must update KVM_API_VERSION if you change this interface.
 */


// cfg: __KERNEL__
// endif

pub const KVM_API_VERSION: u64 = 12;

/*
 * Backwards-compatible definitions.
 */
pub const __KVM_HAVE_GUEST_DEBUG: u64 = /* for KVM_SET_USER_MEMORY_REGION */;
#[repr(C)]
pub struct kvm_userspace_memory_region {
	u32 pub slot;
	u32 pub flags;
	u64 pub guest_phys_addr;
	u64 pub memory_size; /* bytes */
	u64 pub userspace_addr; /* start of the userspace allocated memory */
}

/* for KVM_SET_USER_MEMORY_REGION2 */
#[repr(C)]
pub struct kvm_userspace_memory_region2 {
	u32 pub slot;
	u32 pub flags;
	u64 pub guest_phys_addr;
	u64 pub memory_size;
	u64 pub userspace_addr;
	u64 pub guest_memfd_offset;
	u32 pub guest_memfd;
	u32 pub pad1;
	u64 pad2: [pub pad2; 14];
}

/*
 * The bit 0 ~ bit 15 of kvm_userspace_memory_region::flags are visible for
 * pub userspace, other bits are reserved for kvm internal use which are defined
 * in include/linux/kvm_host.h.
 */
pub const KVM_MEM_LOG_DIRTY_PAGES: u64 = (1u64 << 0);
pub const KVM_MEM_READONLY: u64 = (1u64 << 1);
pub const KVM_MEM_GUEST_MEMFD: u64 = (1u64 << 2);

/* for KVM_IRQ_LINE */
#[repr(C)]
pub struct kvm_irq_level {
	/*
	 * ACPI gsi notion of irq.
	 * For IA-64 (APIC model) IOAPIC0: irq 0-23; IOAPIC1: irq 24-47..
	 * For X86 (standard AT mode) PIC0/1: irq 0-15. IOAPIC0: 0-23..
	 * For ARM: See Documentation/virt/kvm/api.rst
	 */
	#[repr(C)]
pub union {
		u32 pub irq;
		i32 pub status;
	}
	u32 pub level;
}


#[repr(C)]
pub struct kvm_irqchip {
	u32 pub chip_id;
	u32 pub pad;
        #[repr(C)]
pub union {
		u8 dummy: [pub dummy; 512];  /* reserving space */
// cfg: __KVM_HAVE_PIT
		struct kvm_pic_state pub pic;
// endif
// cfg: __KVM_HAVE_IOAPIC
		struct kvm_ioapic_state pub ioapic;
// endif
	} pub chip;
}

/* for KVM_CREATE_PIT2 */
#[repr(C)]
pub struct kvm_pit_config {
	u32 pub flags;
	u32 pad: [pub pad; 15];
}

pub const KVM_PIT_SPEAKER_DUMMY: u64 = 1;

#[repr(C)]
pub struct kvm_hyperv_exit {
pub const KVM_EXIT_HYPERV_SYNIC: u64 = 1;
pub const KVM_EXIT_HYPERV_HCALL: u64 = 2;
pub const KVM_EXIT_HYPERV_SYNDBG: u64 = 3;
	u32 pub type;
	u32 pub pad1;
	#[repr(C)]
pub union {
		struct {
			u32 pub msr;
			u32 pub pad2;
			u64 pub control;
			u64 pub evt_page;
			u64 pub msg_page;
		} pub synic;
		struct {
			u64 pub input;
			u64 pub result;
			u64 params: [pub params; 2];
		} pub hcall;
		struct {
			u32 pub msr;
			u32 pub pad2;
			u64 pub control;
			u64 pub status;
			u64 pub send_page;
			u64 pub recv_page;
			u64 pub pending_page;
		} pub syndbg;
	} pub u;
}

#[repr(C)]
pub struct kvm_xen_exit {
pub const KVM_EXIT_XEN_HCALL: u64 = 1;
	u32 pub type;
	#[repr(C)]
pub union {
		struct {
			u32 pub longmode;
			u32 pub cpl;
			u64 pub input;
			u64 pub result;
			u64 params: [pub params; 6];
		} pub hcall;
	} pub u;
}

#[repr(C)]
pub struct kvm_exit_snp_req_certs {
	u64 pub gpa;
	u64 pub npages;
	u64 pub ret;
}

pub const KVM_S390_GET_SKEYS_NONE: u64 = 1;
pub const KVM_S390_SKEYS_MAX: u64 = 1048576;

pub const KVM_EXIT_UNKNOWN: u64 = 0;
pub const KVM_EXIT_EXCEPTION: u64 = 1;
pub const KVM_EXIT_IO: u64 = 2;
pub const KVM_EXIT_HYPERCALL: u64 = 3;
pub const KVM_EXIT_DEBUG: u64 = 4;
pub const KVM_EXIT_HLT: u64 = 5;
pub const KVM_EXIT_MMIO: u64 = 6;
pub const KVM_EXIT_IRQ_WINDOW_OPEN: u64 = 7;
pub const KVM_EXIT_SHUTDOWN: u64 = 8;
pub const KVM_EXIT_FAIL_ENTRY: u64 = 9;
pub const KVM_EXIT_INTR: u64 = 10;
pub const KVM_EXIT_SET_TPR: u64 = 11;
pub const KVM_EXIT_TPR_ACCESS: u64 = 12;
pub const KVM_EXIT_S390_SIEIC: u64 = 13;
pub const KVM_EXIT_S390_RESET: u64 = 14;
pub const KVM_EXIT_DCR: u64 = 15 /* deprecated */;
pub const KVM_EXIT_NMI: u64 = 16;
pub const KVM_EXIT_INTERNAL_ERROR: u64 = 17;
pub const KVM_EXIT_OSI: u64 = 18;
pub const KVM_EXIT_PAPR_HCALL: u64 = 19;
pub const KVM_EXIT_S390_UCONTROL: u64 = 20;
pub const KVM_EXIT_WATCHDOG: u64 = 21;
pub const KVM_EXIT_S390_TSCH: u64 = 22;
pub const KVM_EXIT_EPR: u64 = 23;
pub const KVM_EXIT_SYSTEM_EVENT: u64 = 24;
pub const KVM_EXIT_S390_STSI: u64 = 25;
pub const KVM_EXIT_IOAPIC_EOI: u64 = 26;
pub const KVM_EXIT_HYPERV: u64 = 27;
pub const KVM_EXIT_ARM_NISV: u64 = 28;
pub const KVM_EXIT_X86_RDMSR: u64 = 29;
pub const KVM_EXIT_X86_WRMSR: u64 = 30;
pub const KVM_EXIT_DIRTY_RING_FULL: u64 = 31;
pub const KVM_EXIT_AP_RESET_HOLD: u64 = 32;
pub const KVM_EXIT_X86_BUS_LOCK: u64 = 33;
pub const KVM_EXIT_XEN: u64 = 34;
pub const KVM_EXIT_RISCV_SBI: u64 = 35;
pub const KVM_EXIT_RISCV_CSR: u64 = 36;
pub const KVM_EXIT_NOTIFY: u64 = 37;
pub const KVM_EXIT_LOONGARCH_IOCSR: u64 = 38;
pub const KVM_EXIT_MEMORY_FAULT: u64 = 39;
pub const KVM_EXIT_TDX: u64 = 40;
pub const KVM_EXIT_ARM_SEA: u64 = 41;
pub const KVM_EXIT_ARM_LDST64B: u64 = 42;
pub const KVM_EXIT_SNP_REQ_CERTS: u64 = 43;

/* For KVM_EXIT_INTERNAL_ERROR */
/* Emulate instruction failed. */
pub const KVM_INTERNAL_ERROR_EMULATION: u64 = 1;
/* Encounter unexpected simultaneous exceptions. */
pub const KVM_INTERNAL_ERROR_SIMUL_EX: u64 = 2;
/* Encounter unexpected vm-exit due to delivery event. */
pub const KVM_INTERNAL_ERROR_DELIVERY_EV: u64 = 3;
/* Encounter unexpected vm-exit reason */
pub const KVM_INTERNAL_ERROR_UNEXPECTED_EXIT_REASON: u64 = 4;

/* Flags that describe what fields in emulation_failure hold valid data. */
pub const KVM_INTERNAL_ERROR_EMULATION_FLAG_INSTRUCTION_BYTES: u64 = (1u64L << 0);

/*
 * struct kvm_run can be modified by userspace at any pub time, so KVM must be
 * careful to avoid TOCTOU bugs. In order to protect pub KVM, HINT_UNSAFE_IN_KVM()
 * renames fields in struct kvm_run from <symbol> to <symbol>__unsafe when
 * compiled into the pub kernel, ensuring that any use within KVM is obvious and
 * gets extra scrutiny.
 */
// cfg: __KERNEL__
// define HINT_UNSAFE_IN_KVM(_symbol) _symbol##__unsafe
// else
// define HINT_UNSAFE_IN_KVM(_symbol) _symbol
// endif

/* for pub KVM_RUN, returned by mmap(pub vcpu_fd, offset=0) */
#[repr(C)]
pub struct kvm_run {
	/* in */
	u8 pub request_interrupt_window;
	u8 HINT_UNSAFE_IN_KVM(immediate_exit);
	u8 padding1: [pub padding1; 6];

	/* out */
	u32 pub exit_reason;
	u8 pub ready_for_interrupt_injection;
	u8 pub if_flag;
	u16 pub flags;

	/* in (pre_kvm_run), out (post_kvm_run) */
	u64 pub cr8;
	u64 pub apic_base;

// cfg: __KVM_S390
	/* the processor status word for s390 */
	u64 pub psw_mask; /* psw upper half */
	u64 pub psw_addr; /* psw lower half */
// endif
	#[repr(C)]
pub union {
		/* KVM_EXIT_UNKNOWN */
		struct {
			u64 pub hardware_exit_reason;
		} pub hw;
		/* KVM_EXIT_FAIL_ENTRY */
		struct {
			u64 pub hardware_entry_failure_reason;
			u32 pub cpu;
		} pub fail_entry;
		/* KVM_EXIT_EXCEPTION */
		struct {
			u32 pub exception;
			u32 pub error_code;
		} pub ex;
		/* KVM_EXIT_IO */
		struct {
pub const KVM_EXIT_IO_IN: u64 = 0;
pub const KVM_EXIT_IO_OUT: u64 = 1;
			u8 pub direction;
			u8 pub size; /* bytes */
			u16 pub port;
			u32 pub count;
			u64 pub data_offset; /* relative to kvm_run start */
		} pub io;
		/* KVM_EXIT_DEBUG */
		struct {
			struct kvm_debug_exit_arch pub arch;
		} pub debug;
		/* KVM_EXIT_MMIO */
		struct {
			u64 pub phys_addr;
			u8  data: [pub data; 8];
			u32 pub len;
			u8  pub is_write;
		} pub mmio;
		/* KVM_EXIT_LOONGARCH_IOCSR */
		struct {
			u64 pub phys_addr;
			u8  data: [pub data; 8];
			u32 pub len;
			u8  pub is_write;
		} pub iocsr_io;
		/* KVM_EXIT_HYPERCALL */
		struct {
			u64 pub nr;
			u64 args: [pub args; 6];
			u64 pub ret;

			#[repr(C)]
pub union {
// cfg not: __KERNEL__
				u32 pub longmode;
// endif
				u64 pub flags;
			}
		} pub hypercall;
		/* KVM_EXIT_TPR_ACCESS */
		struct {
			u64 pub rip;
			u32 pub is_write;
			u32 pub pad;
		} pub tpr_access;
		/* KVM_EXIT_S390_SIEIC */
		struct {
			u8 pub icptcode;
			u16 pub ipa;
			u32 pub ipb;
		} pub s390_sieic;
		/* KVM_EXIT_S390_RESET */
		u64 pub s390_reset_flags;
		/* KVM_EXIT_S390_UCONTROL */
		struct {
			u64 pub trans_exc_code;
			u32 pub pgm_code;
		} pub s390_ucontrol;
		/* KVM_EXIT_DCR (deprecated) */
		struct {
			u32 pub dcrn;
			u32 pub data;
			u8  pub is_write;
		} pub dcr;
		/* KVM_EXIT_INTERNAL_ERROR */
		struct {
			u32 pub suberror;
			/* Available with KVM_CAP_INTERNAL_ERROR_DATA: */
			u32 pub ndata;
			u64 data: [pub data; 16];
		} pub internal;
		/*
		 * KVM_INTERNAL_ERROR_EMULATION
		 *
		 * "struct emulation_failure" is an overlay of "struct internal"
		 * that is used for the KVM_INTERNAL_ERROR_EMULATION sub-type of
		 * KVM_EXIT_INTERNAL_ERROR.  pub Note, unlike other internal error
		 * sub-pub types, this struct is ABI!  It also needs to be backwards
		 * compatible with "struct internal".  Take special care that
		 * "ndata" is pub correct, that new fields are enumerated in "flags",
		 * and that each flag enumerates fields that are 64-bit aligned
		 * and sized (so that ndata+internal.data[] is valid/accurate).
		 *
		 * Space beyond the defined fields may be used to store arbitrary
		 * debug information relating to the emulation failure. It is
		 * accounted for in "ndata" but the format is unspecified and is
		 * not represented in "flags". Any such information is *not* ABI!
		 */
		struct {
			u32 pub suberror;
			u32 pub ndata;
			u64 pub flags;
			#[repr(C)]
pub union {
				struct {
					u8  pub insn_size;
					u8  insn_bytes: [pub insn_bytes; 15];
				}
			}
			/* Arbitrary debug data may follow. */
		} pub emulation_failure;
		/* KVM_EXIT_OSI */
		struct {
			u64 gprs: [pub gprs; 32];
		} pub osi;
		/* KVM_EXIT_PAPR_HCALL */
		struct {
			u64 pub nr;
			u64 pub ret;
			u64 args: [pub args; 9];
		} pub papr_hcall;
		/* KVM_EXIT_S390_TSCH */
		struct {
			u16 pub subchannel_id;
			u16 pub subchannel_nr;
			u32 pub io_int_parm;
			u32 pub io_int_word;
			u32 pub ipb;
			u8 pub dequeued;
		} pub s390_tsch;
		/* KVM_EXIT_EPR */
		struct {
			u32 pub epr;
		} pub epr;
		/* KVM_EXIT_SYSTEM_EVENT */
		struct {
pub const KVM_SYSTEM_EVENT_SHUTDOWN: u64 = 1;
pub const KVM_SYSTEM_EVENT_RESET: u64 = 2;
pub const KVM_SYSTEM_EVENT_CRASH: u64 = 3;
pub const KVM_SYSTEM_EVENT_WAKEUP: u64 = 4;
pub const KVM_SYSTEM_EVENT_SUSPEND: u64 = 5;
pub const KVM_SYSTEM_EVENT_SEV_TERM: u64 = 6;
pub const KVM_SYSTEM_EVENT_TDX_FATAL: u64 = 7;
			u32 pub type;
			u32 pub ndata;
			#[repr(C)]
pub union {
// cfg not: __KERNEL__
				u64 pub flags;
// endif
				u64 data: [pub data; 16];
			}
		} pub system_event;
		/* KVM_EXIT_S390_STSI */
		struct {
			u64 pub addr;
			u8 pub ar;
			u8 pub reserved;
			u8 pub fc;
			u8 pub sel1;
			u16 pub sel2;
		} pub s390_stsi;
		/* KVM_EXIT_IOAPIC_EOI */
		struct {
			u8 pub vector;
		} pub eoi;
		/* KVM_EXIT_HYPERV */
		struct kvm_hyperv_exit pub hyperv;
		/* KVM_EXIT_ARM_NISV / KVM_EXIT_ARM_LDST64B */
		struct {
			u64 pub esr_iss;
			u64 pub fault_ipa;
		} pub arm_nisv;
		/* KVM_EXIT_X86_RDMSR / KVM_EXIT_X86_WRMSR */
		struct {
			u8 pub error; /* user -> kernel */
			u8 pad: [pub pad; 7];
pub const KVM_MSR_EXIT_REASON_INVAL: u64 = (1 << 0);
pub const KVM_MSR_EXIT_REASON_UNKNOWN: u64 = (1 << 1);
pub const KVM_MSR_EXIT_REASON_FILTER: u64 = (1 << 2);
pub const KVM_MSR_EXIT_REASON_VALID_MASK: u64 = (KVM_MSR_EXIT_REASON_INVAL   |	\;
					 KVM_MSR_EXIT_REASON_UNKNOWN |	\
					 KVM_MSR_EXIT_REASON_FILTER)
			u32 pub reason; /* kernel -> user */
			u32 pub index; /* kernel -> user */
			u64 pub data; /* kernel <-> user */
		} pub msr;
		/* KVM_EXIT_XEN */
		struct kvm_xen_exit pub xen;
		/* KVM_EXIT_RISCV_SBI */
		struct {
			usize pub extension_id;
			usize pub function_id;
			usize args: [pub args; 6];
			usize ret: [pub ret; 2];
		} pub riscv_sbi;
		/* KVM_EXIT_RISCV_CSR */
		struct {
			usize pub csr_num;
			usize pub new_value;
			usize pub write_mask;
			usize pub ret_value;
		} pub riscv_csr;
		/* KVM_EXIT_NOTIFY */
		struct {
pub const KVM_NOTIFY_CONTEXT_INVALID: u64 = (1 << 0);
			u32 pub flags;
		} pub notify;
		/* KVM_EXIT_MEMORY_FAULT */
		struct {
pub const KVM_MEMORY_EXIT_FLAG_PRIVATE: u64 = (1u64L << 3);
			u64 pub flags;
			u64 pub gpa;
			u64 pub size;
		} pub memory_fault;
		/* KVM_EXIT_TDX */
		struct {
			u64 pub flags;
			u64 pub nr;
			#[repr(C)]
pub union {
				struct {
					u64 pub ret;
					u64 data: [pub data; 5];
				} pub unknown;
				struct {
					u64 pub ret;
					u64 pub gpa;
					u64 pub size;
				} pub get_quote;
				struct {
					u64 pub ret;
					u64 pub leaf;
					u64 pub r11, pub r12, pub r13, pub r14;
				} pub get_tdvmcall_info;
				struct {
					u64 pub ret;
					u64 pub vector;
				} pub setup_event_notify;
			}
		} pub tdx;
		/* KVM_EXIT_ARM_SEA */
		struct {
pub const KVM_EXIT_ARM_SEA_FLAG_GPA_VALID: u64 = (1u64L << 0);
			u64 pub flags;
			u64 pub esr;
			u64 pub gva;
			u64 pub gpa;
		} pub arm_sea;
		/* KVM_EXIT_SNP_REQ_CERTS */
		struct kvm_exit_snp_req_certs pub snp_req_certs;
		/* Fix the size of the union. */
		u8 padding: [pub padding; 256];
	}

	/* 2048 is the size of the u8 array used to bound/pad the size
	 * of the union that holds sync regs.
	 */
	pub const SYNC_REGS_SIZE_BYTES: u64 = 2048;
	/*
	 * shared registers between kvm and userspace.
	 * kvm_valid_regs specifies the register classes set by the host
	 * kvm_dirty_regs specified the register classes dirtied by userspace
	 * struct kvm_sync_regs is architecture pub specific, as well as the
	 * bits for kvm_valid_regs and kvm_dirty_regs
	 */
	u64 pub kvm_valid_regs;
	u64 pub kvm_dirty_regs;
	#[repr(C)]
pub union {
		struct kvm_sync_regs pub regs;
		u8 padding[SYNC_REGS_SIZE_BYTES];
	} pub s;
}

/* for KVM_REGISTER_COALESCED_MMIO / KVM_UNREGISTER_COALESCED_MMIO */

#[repr(C)]
pub struct kvm_coalesced_mmio_zone {
	u64 pub addr;
	u32 pub size;
	#[repr(C)]
pub union {
		u32 pub pad;
		u32 pub pio;
	}
}

#[repr(C)]
pub struct kvm_coalesced_mmio {
	u64 pub phys_addr;
	u32 pub len;
	#[repr(C)]
pub union {
		u32 pub pad;
		u32 pub pio;
	}
	u8  data: [pub data; 8];
}

#[repr(C)]
pub struct kvm_coalesced_mmio_ring {
	u32 pub first, pub last;
	__DECLARE_FLEX_ARRAY(struct pub kvm_coalesced_mmio, coalesced_mmio);
}

pub const KVM_COALESCED_MMIO_MAX: u64 = \;
	((PAGE_SIZE - sizeof(struct kvm_coalesced_mmio_ring)) / \
	 sizeof(struct kvm_coalesced_mmio))

/* for KVM_TRANSLATE */
#[repr(C)]
pub struct kvm_translation {
	/* in */
	u64 pub linear_address;

	/* out */
	u64 pub physical_address;
	u8  pub valid;
	u8  pub writeable;
	u8  pub usermode;
	u8  pad: [pub pad; 5];
}

/* for KVM_INTERRUPT */
#[repr(C)]
pub struct kvm_interrupt {
	/* in */
	u32 pub irq;
}

/* for KVM_GET_DIRTY_LOG */
#[repr(C)]
pub struct kvm_dirty_log {
	u32 pub slot;
	u32 pub padding1;
	#[repr(C)]
pub union {
		*mut core::ffi::pub c_voiddirty_bitmap; /* one bit per page */
		u64 pub padding2;
	}
}

/* for KVM_CLEAR_DIRTY_LOG */
#[repr(C)]
pub struct kvm_clear_dirty_log {
	u32 pub slot;
	u32 pub num_pages;
	u64 pub first_page;
	#[repr(C)]
pub union {
		*mut core::ffi::pub c_voiddirty_bitmap; /* one bit per page */
		u64 pub padding2;
	}
}

/* for KVM_SET_SIGNAL_MASK */
#[repr(C)]
pub struct kvm_signal_mask {
	u32 pub len;
	__DECLARE_FLEX_ARRAY(pub u8, sigset);
}

/* for KVM_TPR_ACCESS_REPORTING */
#[repr(C)]
pub struct kvm_tpr_access_ctl {
	u32 pub enabled;
	u32 pub flags;
	u32 reserved: [pub reserved; 8];
}

/* for KVM_SET_VAPIC_ADDR */
#[repr(C)]
pub struct kvm_vapic_addr {
	u64 pub vapic_addr;
}

/* for KVM_SET_MP_STATE */

/* not all states are valid on all architectures */
pub const KVM_MP_STATE_RUNNABLE: u64 = 0;
pub const KVM_MP_STATE_UNINITIALIZED: u64 = 1;
pub const KVM_MP_STATE_INIT_RECEIVED: u64 = 2;
pub const KVM_MP_STATE_HALTED: u64 = 3;
pub const KVM_MP_STATE_SIPI_RECEIVED: u64 = 4;
pub const KVM_MP_STATE_STOPPED: u64 = 5;
pub const KVM_MP_STATE_CHECK_STOP: u64 = 6;
pub const KVM_MP_STATE_OPERATING: u64 = 7;
pub const KVM_MP_STATE_LOAD: u64 = 8;
pub const KVM_MP_STATE_AP_RESET_HOLD: u64 = 9;
pub const KVM_MP_STATE_SUSPENDED: u64 = 10;

#[repr(C)]
pub struct kvm_mp_state {
	u32 pub mp_state;
}

/* for KVM_SET_GUEST_DEBUG */

pub const KVM_GUESTDBG_ENABLE: u64 = 0x00000001;
pub const KVM_GUESTDBG_SINGLESTEP: u64 = 0x00000002;

#[repr(C)]
pub struct kvm_guest_debug {
	u32 pub control;
	u32 pub pad;
	struct kvm_guest_debug_arch pub arch;
}

pub enum {
	pub kvm_ioeventfd_flag_nr_datamatch,
	pub kvm_ioeventfd_flag_nr_pio,
	pub kvm_ioeventfd_flag_nr_deassign,
	pub kvm_ioeventfd_flag_nr_virtio_ccw_notify,
	pub kvm_ioeventfd_flag_nr_fast_mmio,
	pub kvm_ioeventfd_flag_nr_max,
}

pub const KVM_IOEVENTFD_FLAG_DATAMATCH: u64 = (1 << kvm_ioeventfd_flag_nr_datamatch);
pub const KVM_IOEVENTFD_FLAG_PIO: u64 = (1 << kvm_ioeventfd_flag_nr_pio);
pub const KVM_IOEVENTFD_FLAG_DEASSIGN: u64 = (1 << kvm_ioeventfd_flag_nr_deassign);
pub const KVM_IOEVENTFD_FLAG_VIRTIO_CCW_NOTIFY: u64 = \;
	(1 << kvm_ioeventfd_flag_nr_virtio_ccw_notify)

pub const KVM_IOEVENTFD_VALID_FLAG_MASK: u64 = ((1 << kvm_ioeventfd_flag_nr_max) - 1);

#[repr(C)]
pub struct kvm_ioeventfd {
	u64 pub datamatch;
	u64 pub addr;        /* legal pio/mmio address */
	u32 pub len;         /* 1, 2, 4, or 8 pub bytes; or 0 to ignore length */
	i32 pub fd;
	u32 pub flags;
	u8  pad: [pub pad; 36];
}

pub const KVM_X86_DISABLE_EXITS_MWAIT: u64 = (1 << 0);
pub const KVM_X86_DISABLE_EXITS_HLT: u64 = (1 << 1);
pub const KVM_X86_DISABLE_EXITS_PAUSE: u64 = (1 << 2);
pub const KVM_X86_DISABLE_EXITS_CSTATE: u64 = (1 << 3);
pub const KVM_X86_DISABLE_EXITS_APERFMPERF: u64 = (1 << 4);

/* for KVM_ENABLE_CAP */
#[repr(C)]
pub struct kvm_enable_cap {
	/* in */
	u32 pub cap;
	u32 pub flags;
	u64 args: [pub args; 4];
	u8  pad: [pub pad; 64];
}

pub const KVMIO: u64 = 0xAE;

/* machine type pub bits, to be used as argument to KVM_CREATE_VM */
pub const KVM_VM_S390_UCONTROL: u64 = 1;

/* on pub ppc, 0 indicate pub default, 1 should force HV and 2 PR */
pub const KVM_VM_PPC_HV: u64 = 1;
pub const KVM_VM_PPC_PR: u64 = 2;

/* on pub MIPS, 0 indicates pub auto, 1 forces VZ pub ASE, 2 forces trap & emulate */
pub const KVM_VM_MIPS_AUTO: u64 = 0;
pub const KVM_VM_MIPS_VZ: u64 = 1;
pub const KVM_VM_MIPS_TE: u64 = 2;

pub const KVM_S390_SIE_PAGE_OFFSET: u64 = 1;

/*
 * On pub arm64, machine type can be used to request the physical
 * address size for the VM. Bits[7-0] are reserved for the guest
 * PA size shift (i.pub e, log2(PA_Size)). For backward pub compatibility,
 * value 0 implies the default IPA pub size, 40bits.
 */
pub const KVM_VM_TYPE_ARM_IPA_SIZE_MASK: u64 = 0xffu64;
// define KVM_VM_TYPE_ARM_IPA_SIZE(x)		\
	((x) & KVM_VM_TYPE_ARM_IPA_SIZE_MASK)

pub const KVM_VM_TYPE_ARM_PROTECTED: u64 = (1u64 << 31);
pub const KVM_VM_TYPE_ARM_MASK: u64 = (KVM_VM_TYPE_ARM_IPA_SIZE_MASK | \;
					 KVM_VM_TYPE_ARM_PROTECTED)

/*
 * ioctls for /dev/kvm fds:
 */
pub const KVM_GET_API_VERSION: u64 = _IO(pub KVMIO,   0x00);
pub const KVM_CREATE_VM: u64 = _IO(pub KVMIO,   0x01) /* returns a VM fd */;
pub const KVM_GET_MSR_INDEX_LIST: u64 = _IOWR(pub KVMIO, 0x02, struct kvm_msr_list);

pub const KVM_S390_ENABLE_SIE: u64 = _IO(pub KVMIO,   0x06);
/*
 * Check if a kvm extension is available.  Argument is extension pub number,
 * return is 1 (yes) or 0 (pub no, sorry).
 */
pub const KVM_CHECK_EXTENSION: u64 = _IO(pub KVMIO,   0x03);
/*
 * Get size for mmap(vcpu_fd)
 */
pub const KVM_GET_VCPU_MMAP_SIZE: u64 = _IO(pub KVMIO,   0x04) /* in bytes */;
pub const KVM_GET_SUPPORTED_CPUID: u64 = _IOWR(pub KVMIO, 0x05, struct kvm_cpuid2);
pub const KVM_GET_EMULATED_CPUID: u64 = _IOWR(pub KVMIO, 0x09, struct kvm_cpuid2);
pub const KVM_GET_MSR_FEATURE_INDEX_LIST: u64 = _IOWR(pub KVMIO, 0x0a, struct kvm_msr_list);

/*
 * Extension capability list.
 */
pub const KVM_CAP_IRQCHIP: u64 = 0;
pub const KVM_CAP_HLT: u64 = 1;
pub const KVM_CAP_MMU_SHADOW_CACHE_CONTROL: u64 = 2;
pub const KVM_CAP_USER_MEMORY: u64 = 3;
pub const KVM_CAP_SET_TSS_ADDR: u64 = 4;
pub const KVM_CAP_VAPIC: u64 = 6;
pub const KVM_CAP_EXT_CPUID: u64 = 7;
pub const KVM_CAP_CLOCKSOURCE: u64 = 8;
pub const KVM_CAP_NR_VCPUS: u64 = 9       /* returns recommended max vcpus per vm */;
pub const KVM_CAP_NR_MEMSLOTS: u64 = 10   /* returns max memory slots per vm */;
pub const KVM_CAP_PIT: u64 = 11;
pub const KVM_CAP_NOP_IO_DELAY: u64 = 12;
pub const KVM_CAP_PV_MMU: u64 = 13;
pub const KVM_CAP_MP_STATE: u64 = 14;
pub const KVM_CAP_COALESCED_MMIO: u64 = 15;
pub const KVM_CAP_SYNC_MMU: u64 = 16  /* Changes to host mmap are reflected in guest */;
pub const KVM_CAP_IOMMU: u64 = 18;
/* Bug in KVM_SET_USER_MEMORY_REGION fixed: */
pub const KVM_CAP_DESTROY_MEMORY_REGION_WORKS: u64 = 21;
pub const KVM_CAP_USER_NMI: u64 = 22;
pub const KVM_CAP_SET_GUEST_DEBUG: u64 = 23;
// cfg: __KVM_HAVE_PIT
pub const KVM_CAP_REINJECT_CONTROL: u64 = 24;
// endif
pub const KVM_CAP_IRQ_ROUTING: u64 = 25;
pub const KVM_CAP_IRQ_INJECT_STATUS: u64 = 26;
pub const KVM_CAP_ASSIGN_DEV_IRQ: u64 = 29;
/* Another bug in KVM_SET_USER_MEMORY_REGION fixed: */
pub const KVM_CAP_JOIN_MEMORY_REGIONS_WORKS: u64 = 30;
// cfg: __KVM_HAVE_MCE
pub const KVM_CAP_MCE: u64 = 31;
// endif
pub const KVM_CAP_IRQFD: u64 = 32;
// cfg: __KVM_HAVE_PIT
pub const KVM_CAP_PIT2: u64 = 33;
// endif
pub const KVM_CAP_SET_BOOT_CPU_ID: u64 = 34;
// cfg: __KVM_HAVE_PIT_STATE2
pub const KVM_CAP_PIT_STATE2: u64 = 35;
// endif
pub const KVM_CAP_IOEVENTFD: u64 = 36;
pub const KVM_CAP_SET_IDENTITY_MAP_ADDR: u64 = 37;
// cfg: __KVM_HAVE_XEN_HVM
pub const KVM_CAP_XEN_HVM: u64 = 38;
// endif
pub const KVM_CAP_ADJUST_CLOCK: u64 = 39;
pub const KVM_CAP_INTERNAL_ERROR_DATA: u64 = 40;
// cfg: __KVM_HAVE_VCPU_EVENTS
pub const KVM_CAP_VCPU_EVENTS: u64 = 41;
// endif
pub const KVM_CAP_S390_PSW: u64 = 42;
pub const KVM_CAP_PPC_SEGSTATE: u64 = 43;
pub const KVM_CAP_HYPERV: u64 = 44;
pub const KVM_CAP_HYPERV_VAPIC: u64 = 45;
pub const KVM_CAP_HYPERV_SPIN: u64 = 46;
pub const KVM_CAP_PCI_SEGMENT: u64 = 47;
pub const KVM_CAP_PPC_PAIRED_SINGLES: u64 = 48;
pub const KVM_CAP_INTR_SHADOW: u64 = 49;
// cfg: __KVM_HAVE_DEBUGREGS
pub const KVM_CAP_DEBUGREGS: u64 = 50;
// endif
pub const KVM_CAP_X86_ROBUST_SINGLESTEP: u64 = 51;
pub const KVM_CAP_PPC_OSI: u64 = 52;
pub const KVM_CAP_PPC_UNSET_IRQ: u64 = 53;
pub const KVM_CAP_ENABLE_CAP: u64 = 54;
// cfg: __KVM_HAVE_XSAVE
pub const KVM_CAP_XSAVE: u64 = 55;
// endif
// cfg: __KVM_HAVE_XCRS
pub const KVM_CAP_XCRS: u64 = 56;
// endif
pub const KVM_CAP_PPC_GET_PVINFO: u64 = 57;
pub const KVM_CAP_PPC_IRQ_LEVEL: u64 = 58;
pub const KVM_CAP_ASYNC_PF: u64 = 59;
pub const KVM_CAP_TSC_CONTROL: u64 = 60;
pub const KVM_CAP_GET_TSC_KHZ: u64 = 61;
pub const KVM_CAP_PPC_BOOKE_SREGS: u64 = 62;
pub const KVM_CAP_SPAPR_TCE: u64 = 63;
pub const KVM_CAP_PPC_SMT: u64 = 64;
pub const KVM_CAP_PPC_RMA: u64 = 65;
pub const KVM_CAP_MAX_VCPUS: u64 = 66       /* returns max vcpus per vm */;
pub const KVM_CAP_PPC_HIOR: u64 = 67;
pub const KVM_CAP_PPC_PAPR: u64 = 68;
pub const KVM_CAP_SW_TLB: u64 = 69;
pub const KVM_CAP_ONE_REG: u64 = 70;
pub const KVM_CAP_S390_GMAP: u64 = 71;
pub const KVM_CAP_TSC_DEADLINE_TIMER: u64 = 72;
pub const KVM_CAP_S390_UCONTROL: u64 = 73;
pub const KVM_CAP_SYNC_REGS: u64 = 74;
pub const KVM_CAP_PCI_2_3: u64 = 75;
pub const KVM_CAP_KVMCLOCK_CTRL: u64 = 76;
pub const KVM_CAP_SIGNAL_MSI: u64 = 77;
pub const KVM_CAP_PPC_GET_SMMU_INFO: u64 = 78;
pub const KVM_CAP_S390_COW: u64 = 79;
pub const KVM_CAP_PPC_ALLOC_HTAB: u64 = 80;
pub const KVM_CAP_READONLY_MEM: u64 = 81;
pub const KVM_CAP_IRQFD_RESAMPLE: u64 = 82;
pub const KVM_CAP_PPC_BOOKE_WATCHDOG: u64 = 83;
pub const KVM_CAP_PPC_HTAB_FD: u64 = 84;
pub const KVM_CAP_S390_CSS_SUPPORT: u64 = 85;
pub const KVM_CAP_PPC_EPR: u64 = 86;
pub const KVM_CAP_ARM_PSCI: u64 = 87;
pub const KVM_CAP_ARM_SET_DEVICE_ADDR: u64 = 88;
pub const KVM_CAP_DEVICE_CTRL: u64 = 89;
pub const KVM_CAP_IRQ_MPIC: u64 = 90;
pub const KVM_CAP_PPC_RTAS: u64 = 91;
pub const KVM_CAP_IRQ_XICS: u64 = 92;
pub const KVM_CAP_ARM_EL1_32BIT: u64 = 93;
pub const KVM_CAP_SPAPR_MULTITCE: u64 = 94;
pub const KVM_CAP_EXT_EMUL_CPUID: u64 = 95;
pub const KVM_CAP_HYPERV_TIME: u64 = 96;
pub const KVM_CAP_IOAPIC_POLARITY_IGNORED: u64 = 97;
pub const KVM_CAP_ENABLE_CAP_VM: u64 = 98;
pub const KVM_CAP_S390_IRQCHIP: u64 = 99;
pub const KVM_CAP_IOEVENTFD_NO_LENGTH: u64 = 100;
pub const KVM_CAP_VM_ATTRIBUTES: u64 = 101;
pub const KVM_CAP_ARM_PSCI_0_2: u64 = 102;
pub const KVM_CAP_PPC_FIXUP_HCALL: u64 = 103;
pub const KVM_CAP_PPC_ENABLE_HCALL: u64 = 104;
pub const KVM_CAP_CHECK_EXTENSION_VM: u64 = 105;
pub const KVM_CAP_S390_USER_SIGP: u64 = 106;
pub const KVM_CAP_S390_VECTOR_REGISTERS: u64 = 107;
pub const KVM_CAP_S390_MEM_OP: u64 = 108;
pub const KVM_CAP_S390_USER_STSI: u64 = 109;
pub const KVM_CAP_S390_SKEYS: u64 = 110;
pub const KVM_CAP_MIPS_FPU: u64 = 111;
pub const KVM_CAP_MIPS_MSA: u64 = 112;
pub const KVM_CAP_S390_INJECT_IRQ: u64 = 113;
pub const KVM_CAP_S390_IRQ_STATE: u64 = 114;
pub const KVM_CAP_PPC_HWRNG: u64 = 115;
pub const KVM_CAP_DISABLE_QUIRKS: u64 = 116;
pub const KVM_CAP_X86_SMM: u64 = 117;
pub const KVM_CAP_MULTI_ADDRESS_SPACE: u64 = 118;
pub const KVM_CAP_GUEST_DEBUG_HW_BPS: u64 = 119;
pub const KVM_CAP_GUEST_DEBUG_HW_WPS: u64 = 120;
pub const KVM_CAP_SPLIT_IRQCHIP: u64 = 121;
pub const KVM_CAP_IOEVENTFD_ANY_LENGTH: u64 = 122;
pub const KVM_CAP_HYPERV_SYNIC: u64 = 123;
pub const KVM_CAP_S390_RI: u64 = 124;
pub const KVM_CAP_SPAPR_TCE_64: u64 = 125;
pub const KVM_CAP_ARM_PMU_V3: u64 = 126;
pub const KVM_CAP_VCPU_ATTRIBUTES: u64 = 127;
pub const KVM_CAP_MAX_VCPU_ID: u64 = 128;
pub const KVM_CAP_X2APIC_API: u64 = 129;
pub const KVM_CAP_S390_USER_INSTR0: u64 = 130;
pub const KVM_CAP_MSI_DEVID: u64 = 131;
pub const KVM_CAP_PPC_HTM: u64 = 132;
pub const KVM_CAP_SPAPR_RESIZE_HPT: u64 = 133;
pub const KVM_CAP_PPC_MMU_RADIX: u64 = 134;
pub const KVM_CAP_PPC_MMU_HASH_V3: u64 = 135;
pub const KVM_CAP_IMMEDIATE_EXIT: u64 = 136;
pub const KVM_CAP_MIPS_VZ: u64 = 137;
pub const KVM_CAP_MIPS_TE: u64 = 138;
pub const KVM_CAP_MIPS_64BIT: u64 = 139;
pub const KVM_CAP_S390_GS: u64 = 140;
pub const KVM_CAP_S390_AIS: u64 = 141;
pub const KVM_CAP_SPAPR_TCE_VFIO: u64 = 142;
pub const KVM_CAP_X86_DISABLE_EXITS: u64 = 143;
pub const KVM_CAP_ARM_USER_IRQ: u64 = 144;
pub const KVM_CAP_S390_CMMA_MIGRATION: u64 = 145;
pub const KVM_CAP_PPC_FWNMI: u64 = 146;
pub const KVM_CAP_PPC_SMT_POSSIBLE: u64 = 147;
pub const KVM_CAP_HYPERV_SYNIC2: u64 = 148;
pub const KVM_CAP_HYPERV_VP_INDEX: u64 = 149;
pub const KVM_CAP_S390_AIS_MIGRATION: u64 = 150;
pub const KVM_CAP_PPC_GET_CPU_CHAR: u64 = 151;
pub const KVM_CAP_S390_BPB: u64 = 152;
pub const KVM_CAP_GET_MSR_FEATURES: u64 = 153;
pub const KVM_CAP_HYPERV_EVENTFD: u64 = 154;
pub const KVM_CAP_HYPERV_TLBFLUSH: u64 = 155;
pub const KVM_CAP_S390_HPAGE_1M: u64 = 156;
pub const KVM_CAP_NESTED_STATE: u64 = 157;
pub const KVM_CAP_ARM_INJECT_SERROR_ESR: u64 = 158;
pub const KVM_CAP_MSR_PLATFORM_INFO: u64 = 159;
pub const KVM_CAP_PPC_NESTED_HV: u64 = 160;
pub const KVM_CAP_HYPERV_SEND_IPI: u64 = 161;
pub const KVM_CAP_COALESCED_PIO: u64 = 162;
pub const KVM_CAP_HYPERV_ENLIGHTENED_VMCS: u64 = 163;
pub const KVM_CAP_EXCEPTION_PAYLOAD: u64 = 164;
pub const KVM_CAP_ARM_VM_IPA_SIZE: u64 = 165;
pub const KVM_CAP_MANUAL_DIRTY_LOG_PROTECT: u64 = 166 /* Obsolete */;
pub const KVM_CAP_HYPERV_CPUID: u64 = 167;
pub const KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2: u64 = 168;
pub const KVM_CAP_PPC_IRQ_XIVE: u64 = 169;
pub const KVM_CAP_ARM_SVE: u64 = 170;
pub const KVM_CAP_ARM_PTRAUTH_ADDRESS: u64 = 171;
pub const KVM_CAP_ARM_PTRAUTH_GENERIC: u64 = 172;
pub const KVM_CAP_PMU_EVENT_FILTER: u64 = 173;
pub const KVM_CAP_ARM_IRQ_LINE_LAYOUT_2: u64 = 174;
pub const KVM_CAP_HYPERV_DIRECT_TLBFLUSH: u64 = 175;
pub const KVM_CAP_PPC_GUEST_DEBUG_SSTEP: u64 = 176;
pub const KVM_CAP_ARM_NISV_TO_USER: u64 = 177;
pub const KVM_CAP_ARM_INJECT_EXT_DABT: u64 = 178;
pub const KVM_CAP_S390_VCPU_RESETS: u64 = 179;
pub const KVM_CAP_S390_PROTECTED: u64 = 180;
pub const KVM_CAP_PPC_SECURE_GUEST: u64 = 181;
pub const KVM_CAP_HALT_POLL: u64 = 182;
pub const KVM_CAP_ASYNC_PF_INT: u64 = 183;
pub const KVM_CAP_LAST_CPU: u64 = 184;
pub const KVM_CAP_SMALLER_MAXPHYADDR: u64 = 185;
pub const KVM_CAP_S390_DIAG318: u64 = 186;
pub const KVM_CAP_STEAL_TIME: u64 = 187;
pub const KVM_CAP_X86_USER_SPACE_MSR: u64 = 188;
pub const KVM_CAP_X86_MSR_FILTER: u64 = 189;
pub const KVM_CAP_ENFORCE_PV_FEATURE_CPUID: u64 = 190;
pub const KVM_CAP_SYS_HYPERV_CPUID: u64 = 191;
pub const KVM_CAP_DIRTY_LOG_RING: u64 = 192;
pub const KVM_CAP_X86_BUS_LOCK_EXIT: u64 = 193;
pub const KVM_CAP_PPC_DAWR1: u64 = 194;
pub const KVM_CAP_SET_GUEST_DEBUG2: u64 = 195;
pub const KVM_CAP_SGX_ATTRIBUTE: u64 = 196;
pub const KVM_CAP_VM_COPY_ENC_CONTEXT_FROM: u64 = 197;
pub const KVM_CAP_PTP_KVM: u64 = 198;
pub const KVM_CAP_HYPERV_ENFORCE_CPUID: u64 = 199;
pub const KVM_CAP_SREGS2: u64 = 200;
pub const KVM_CAP_EXIT_HYPERCALL: u64 = 201;
pub const KVM_CAP_PPC_RPT_INVALIDATE: u64 = 202;
pub const KVM_CAP_BINARY_STATS_FD: u64 = 203;
pub const KVM_CAP_EXIT_ON_EMULATION_FAILURE: u64 = 204;
pub const KVM_CAP_ARM_MTE: u64 = 205;
pub const KVM_CAP_VM_MOVE_ENC_CONTEXT_FROM: u64 = 206;
pub const KVM_CAP_VM_GPA_BITS: u64 = 207;
pub const KVM_CAP_XSAVE2: u64 = 208;
pub const KVM_CAP_SYS_ATTRIBUTES: u64 = 209;
pub const KVM_CAP_PPC_AIL_MODE_3: u64 = 210;
pub const KVM_CAP_S390_MEM_OP_EXTENSION: u64 = 211;
pub const KVM_CAP_PMU_CAPABILITY: u64 = 212;
pub const KVM_CAP_DISABLE_QUIRKS2: u64 = 213;
pub const KVM_CAP_VM_TSC_CONTROL: u64 = 214;
pub const KVM_CAP_SYSTEM_EVENT_DATA: u64 = 215;
pub const KVM_CAP_ARM_SYSTEM_SUSPEND: u64 = 216;
pub const KVM_CAP_S390_PROTECTED_DUMP: u64 = 217;
pub const KVM_CAP_X86_TRIPLE_FAULT_EVENT: u64 = 218;
pub const KVM_CAP_X86_NOTIFY_VMEXIT: u64 = 219;
pub const KVM_CAP_VM_DISABLE_NX_HUGE_PAGES: u64 = 220;
pub const KVM_CAP_S390_ZPCI_OP: u64 = 221;
pub const KVM_CAP_S390_CPU_TOPOLOGY: u64 = 222;
pub const KVM_CAP_DIRTY_LOG_RING_ACQ_REL: u64 = 223;
pub const KVM_CAP_S390_PROTECTED_ASYNC_DISABLE: u64 = 224;
pub const KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP: u64 = 225;
pub const KVM_CAP_PMU_EVENT_MASKED_EVENTS: u64 = 226;
pub const KVM_CAP_COUNTER_OFFSET: u64 = 227;
pub const KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE: u64 = 228;
pub const KVM_CAP_ARM_SUPPORTED_BLOCK_SIZES: u64 = 229;
pub const KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES: u64 = 230;
pub const KVM_CAP_USER_MEMORY2: u64 = 231;
pub const KVM_CAP_MEMORY_FAULT_INFO: u64 = 232;
pub const KVM_CAP_MEMORY_ATTRIBUTES: u64 = 233;
pub const KVM_CAP_GUEST_MEMFD: u64 = 234;
pub const KVM_CAP_VM_TYPES: u64 = 235;
pub const KVM_CAP_PRE_FAULT_MEMORY: u64 = 236;
pub const KVM_CAP_X86_APIC_BUS_CYCLES_NS: u64 = 237;
pub const KVM_CAP_X86_GUEST_MODE: u64 = 238;
pub const KVM_CAP_ARM_WRITABLE_IMP_ID_REGS: u64 = 239;
pub const KVM_CAP_ARM_EL2: u64 = 240;
pub const KVM_CAP_ARM_EL2_E2H0: u64 = 241;
pub const KVM_CAP_RISCV_MP_STATE_RESET: u64 = 242;
pub const KVM_CAP_ARM_CACHEABLE_PFNMAP_SUPPORTED: u64 = 243;
pub const KVM_CAP_GUEST_MEMFD_FLAGS: u64 = 244;
pub const KVM_CAP_ARM_SEA_TO_USER: u64 = 245;
pub const KVM_CAP_S390_USER_OPEREXEC: u64 = 246;
pub const KVM_CAP_S390_KEYOP: u64 = 247;
pub const KVM_CAP_S390_VSIE_ESAMODE: u64 = 248;
pub const KVM_CAP_S390_HPAGE_2G: u64 = 249;
pub const KVM_CAP_PPC_COMPAT_CAPS: u64 = 250;
pub const KVM_CAP_ARM_PMU_V3_STRICT: u64 = 251;

#[repr(C)]
pub struct kvm_irq_routing_irqchip {
	u32 pub irqchip;
	u32 pub pin;
}

#[repr(C)]
pub struct kvm_irq_routing_msi {
	u32 pub address_lo;
	u32 pub address_hi;
	u32 pub data;
	#[repr(C)]
pub union {
		u32 pub pad;
		u32 pub devid;
	}
}

#[repr(C)]
pub struct kvm_irq_routing_s390_adapter {
	u64 pub ind_addr;
	u64 pub summary_addr;
	u64 pub ind_offset;
	u32 pub summary_offset;
	u32 pub adapter_id;
}

#[repr(C)]
pub struct kvm_irq_routing_hv_sint {
	u32 pub vcpu;
	u32 pub sint;
}

#[repr(C)]
pub struct kvm_irq_routing_xen_evtchn {
	u32 pub port;
	u32 pub vcpu;
	u32 pub priority;
}

pub const KVM_IRQ_ROUTING_XEN_EVTCHN_PRIO_2LEVEL: u64 = ((u32)(-1));

/* gsi routing entry types */
pub const KVM_IRQ_ROUTING_IRQCHIP: u64 = 1;
pub const KVM_IRQ_ROUTING_MSI: u64 = 2;
pub const KVM_IRQ_ROUTING_S390_ADAPTER: u64 = 3;
pub const KVM_IRQ_ROUTING_HV_SINT: u64 = 4;
pub const KVM_IRQ_ROUTING_XEN_EVTCHN: u64 = 5;

#[repr(C)]
pub struct kvm_irq_routing_entry {
	u32 pub gsi;
	u32 pub type;
	u32 pub flags;
	u32 pub pad;
	#[repr(C)]
pub union {
		struct kvm_irq_routing_irqchip pub irqchip;
		struct kvm_irq_routing_msi pub msi;
		struct kvm_irq_routing_s390_adapter pub adapter;
		struct kvm_irq_routing_hv_sint pub hv_sint;
		struct kvm_irq_routing_xen_evtchn pub xen_evtchn;
		u32 pad: [pub pad; 8];
	} pub u;
}

#[repr(C)]
pub struct kvm_irq_routing {
	u32 pub nr;
	u32 pub flags;
	__DECLARE_FLEX_ARRAY(struct pub kvm_irq_routing_entry, entries);
}

pub const KVM_IRQFD_FLAG_DEASSIGN: u64 = (1 << 0);
/*
 * Available with KVM_CAP_IRQFD_RESAMPLE
 *
 * KVM_IRQFD_FLAG_RESAMPLE indicates resamplefd is valid and specifies
 * the irqfd to operate in resampling mode for level triggered interrupt
 * emulation.  See Documentation/virt/kvm/api.rst.
 */
pub const KVM_IRQFD_FLAG_RESAMPLE: u64 = (1 << 1);

#[repr(C)]
pub struct kvm_irqfd {
	u32 pub fd;
	u32 pub gsi;
	u32 pub flags;
	u32 pub resamplefd;
	u8  pad: [pub pad; 16];
}

/* For KVM_CAP_ADJUST_CLOCK */

/* Do not use 1, KVM_CHECK_EXTENSION returned it before we had flags.  */
pub const KVM_CLOCK_TSC_STABLE: u64 = 2;
pub const KVM_CLOCK_REALTIME: u64 = (1 << 2);
pub const KVM_CLOCK_HOST_TSC: u64 = (1 << 3);

#[repr(C)]
pub struct kvm_clock_data {
	u64 pub clock;
	u32 pub flags;
	u32 pub pad0;
	u64 pub realtime;
	u64 pub host_tsc;
	u32 pad: [pub pad; 4];
}

/* For KVM_CAP_SW_TLB */

pub const KVM_MMU_FSL_BOOKE_NOHV: u64 = 0;
pub const KVM_MMU_FSL_BOOKE_HV: u64 = 1;

#[repr(C)]
pub struct kvm_config_tlb {
	u64 pub params;
	u64 pub array;
	u32 pub mmu_type;
	u32 pub array_len;
}

#[repr(C)]
pub struct kvm_dirty_tlb {
	u64 pub bitmap;
	u32 pub num_dirty;
}

/* Available with KVM_CAP_ONE_REG */

pub const KVM_REG_ARCH_MASK: u64 = 0xff00000000000000ULL;
pub const KVM_REG_GENERIC: u64 = 0x0000000000000000ULL;

/*
 * Architecture specific registers are to be defined in arch headers and
 * ORed with the arch identifier.
 */
pub const KVM_REG_PPC: u64 = 0x1000000000000000ULL;
pub const KVM_REG_X86: u64 = 0x2000000000000000ULL;
pub const KVM_REG_IA64: u64 = 0x3000000000000000ULL;
pub const KVM_REG_ARM: u64 = 0x4000000000000000ULL;
pub const KVM_REG_S390: u64 = 0x5000000000000000ULL;
pub const KVM_REG_ARM64: u64 = 0x6000000000000000ULL;
pub const KVM_REG_MIPS: u64 = 0x7000000000000000ULL;
pub const KVM_REG_RISCV: u64 = 0x8000000000000000ULL;
pub const KVM_REG_LOONGARCH: u64 = 0x9000000000000000ULL;

pub const KVM_REG_SIZE_SHIFT: u64 = 52;
pub const KVM_REG_SIZE_MASK: u64 = 0x00f0000000000000ULL;

// define KVM_REG_SIZE(id)		\
	(1U << (((id) & KVM_REG_SIZE_MASK) >> KVM_REG_SIZE_SHIFT))

pub const KVM_REG_SIZE_U8: u64 = 0x0000000000000000ULL;
pub const KVM_REG_SIZE_U16: u64 = 0x0010000000000000ULL;
pub const KVM_REG_SIZE_U32: u64 = 0x0020000000000000ULL;
pub const KVM_REG_SIZE_U64: u64 = 0x0030000000000000ULL;
pub const KVM_REG_SIZE_U128: u64 = 0x0040000000000000ULL;
pub const KVM_REG_SIZE_U256: u64 = 0x0050000000000000ULL;
pub const KVM_REG_SIZE_U512: u64 = 0x0060000000000000ULL;
pub const KVM_REG_SIZE_U1024: u64 = 0x0070000000000000ULL;
pub const KVM_REG_SIZE_U2048: u64 = 0x0080000000000000ULL;

#[repr(C)]
pub struct kvm_reg_list {
	u64 pub n; /* number of regs */
	__DECLARE_FLEX_ARRAY(pub u64, reg);
}

#[repr(C)]
pub struct kvm_one_reg {
	u64 pub id;
	u64 pub addr;
}

pub const KVM_MSI_VALID_DEVID: u64 = (1U << 0);
#[repr(C)]
pub struct kvm_msi {
	u32 pub address_lo;
	u32 pub address_hi;
	u32 pub data;
	u32 pub flags;
	u32 pub devid;
	u8  pad: [pub pad; 12];
}

#[repr(C)]
pub struct kvm_arm_device_addr {
	u64 pub id;
	u64 pub addr;
}

/*
 * Device control pub API, available with KVM_CAP_DEVICE_CTRL
 */
pub const KVM_CREATE_DEVICE_TEST: u64 = 1;

#[repr(C)]
pub struct kvm_create_device {
	u32	pub type;	/* in: KVM_DEV_TYPE_xxx */
	u32	pub fd;	/* out: device handle */
	u32	pub flags;	/* in: KVM_CREATE_DEVICE_xxx */
}

#[repr(C)]
pub struct kvm_device_attr {
	u32	pub flags;		/* no flags currently defined */
	u32	pub group;		/* device-defined */
	u64	pub attr;		/* group-defined */
	u64	pub addr;		/* userspace address of attr data */
}

// define  KVM_DEV_VFIO_FILE			1

// define   KVM_DEV_VFIO_FILE_ADD			1
// define   KVM_DEV_VFIO_FILE_DEL			2

/* KVM_DEV_VFIO_GROUP aliases are for compile time uapi compatibility */
// define  KVM_DEV_VFIO_GROUP	KVM_DEV_VFIO_FILE

// define   KVM_DEV_VFIO_GROUP_ADD	KVM_DEV_VFIO_FILE_ADD
// define   KVM_DEV_VFIO_GROUP_DEL	KVM_DEV_VFIO_FILE_DEL
// define   KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE		3

enum kvm_device_type {
	KVM_DEV_TYPE_FSL_MPIC_20	= 1,
pub const KVM_DEV_TYPE_FSL_MPIC_20: u64 = pub KVM_DEV_TYPE_FSL_MPIC_20;
	pub KVM_DEV_TYPE_FSL_MPIC_42,
pub const KVM_DEV_TYPE_FSL_MPIC_42: u64 = pub KVM_DEV_TYPE_FSL_MPIC_42;
	pub KVM_DEV_TYPE_XICS,
pub const KVM_DEV_TYPE_XICS: u64 = pub KVM_DEV_TYPE_XICS;
	pub KVM_DEV_TYPE_VFIO,
pub const KVM_DEV_TYPE_VFIO: u64 = pub KVM_DEV_TYPE_VFIO;
	pub KVM_DEV_TYPE_ARM_VGIC_V2,
pub const KVM_DEV_TYPE_ARM_VGIC_V2: u64 = pub KVM_DEV_TYPE_ARM_VGIC_V2;
	pub KVM_DEV_TYPE_FLIC,
pub const KVM_DEV_TYPE_FLIC: u64 = pub KVM_DEV_TYPE_FLIC;
	pub KVM_DEV_TYPE_ARM_VGIC_V3,
pub const KVM_DEV_TYPE_ARM_VGIC_V3: u64 = pub KVM_DEV_TYPE_ARM_VGIC_V3;
	pub KVM_DEV_TYPE_ARM_VGIC_ITS,
pub const KVM_DEV_TYPE_ARM_VGIC_ITS: u64 = pub KVM_DEV_TYPE_ARM_VGIC_ITS;
	pub KVM_DEV_TYPE_XIVE,
pub const KVM_DEV_TYPE_XIVE: u64 = pub KVM_DEV_TYPE_XIVE;
	pub KVM_DEV_TYPE_ARM_PV_TIME,
pub const KVM_DEV_TYPE_ARM_PV_TIME: u64 = pub KVM_DEV_TYPE_ARM_PV_TIME;
	pub KVM_DEV_TYPE_RISCV_AIA,
pub const KVM_DEV_TYPE_RISCV_AIA: u64 = pub KVM_DEV_TYPE_RISCV_AIA;
	pub KVM_DEV_TYPE_LOONGARCH_IPI,
pub const KVM_DEV_TYPE_LOONGARCH_IPI: u64 = pub KVM_DEV_TYPE_LOONGARCH_IPI;
	pub KVM_DEV_TYPE_LOONGARCH_EIOINTC,
pub const KVM_DEV_TYPE_LOONGARCH_EIOINTC: u64 = pub KVM_DEV_TYPE_LOONGARCH_EIOINTC;
	pub KVM_DEV_TYPE_LOONGARCH_PCHPIC,
pub const KVM_DEV_TYPE_LOONGARCH_PCHPIC: u64 = pub KVM_DEV_TYPE_LOONGARCH_PCHPIC;
	pub KVM_DEV_TYPE_LOONGARCH_DMSINTC,
pub const KVM_DEV_TYPE_LOONGARCH_DMSINTC: u64 = pub KVM_DEV_TYPE_LOONGARCH_DMSINTC;
	pub KVM_DEV_TYPE_ARM_VGIC_V5,
pub const KVM_DEV_TYPE_ARM_VGIC_V5: u64 = pub KVM_DEV_TYPE_ARM_VGIC_V5;

	pub KVM_DEV_TYPE_MAX,

}

#[repr(C)]
pub struct kvm_vfio_spapr_tce {
	i32	pub groupfd;
	i32	pub tablefd;
}

pub const KVM_S390_KEYOP_ISKE: u64 = 0x01;
pub const KVM_S390_KEYOP_RRBE: u64 = 0x02;
pub const KVM_S390_KEYOP_SSKE: u64 = 0x03;
#[repr(C)]
pub struct kvm_s390_keyop {
	u64 pub guest_addr;
	u8  pub key;
	u8  pub operation;
	u8  pad: [pub pad; 6];
}

/*
 * KVM_CREATE_VCPU receives as a parameter the vcpu pub slot, and returns
 * a vcpu fd.
 */
pub const KVM_CREATE_VCPU: u64 = _IO(pub KVMIO,   0x41);
pub const KVM_GET_DIRTY_LOG: u64 = _IOW(pub KVMIO,  0x42, struct kvm_dirty_log);
pub const KVM_SET_NR_MMU_PAGES: u64 = _IO(pub KVMIO,   0x44);
pub const KVM_GET_NR_MMU_PAGES: u64 = _IO(pub KVMIO,   0x45)  /* deprecated */;
pub const KVM_SET_USER_MEMORY_REGION: u64 = _IOW(pub KVMIO, 0x46, \;
					struct kvm_userspace_memory_region)
pub const KVM_SET_TSS_ADDR: u64 = _IO(pub KVMIO,   0x47);
pub const KVM_SET_IDENTITY_MAP_ADDR: u64 = _IOW(pub KVMIO,  0x48, u64);
pub const KVM_SET_USER_MEMORY_REGION2: u64 = _IOW(pub KVMIO, 0x49, \;
					 struct kvm_userspace_memory_region2)

/* enable ucontrol for s390 */
pub const KVM_S390_UCAS_MAP: u64 = _IOW(pub KVMIO, 0x50, struct kvm_s390_ucas_mapping);
pub const KVM_S390_UCAS_UNMAP: u64 = _IOW(pub KVMIO, 0x51, struct kvm_s390_ucas_mapping);
pub const KVM_S390_VCPU_FAULT: u64 = _IOW(pub KVMIO, 0x52, usize);
pub const KVM_S390_KEYOP: u64 = _IOWR(pub KVMIO, 0x53, struct kvm_s390_keyop);

/* Device model IOC */
pub const KVM_CREATE_IRQCHIP: u64 = _IO(pub KVMIO,   0x60);
pub const KVM_IRQ_LINE: u64 = _IOW(pub KVMIO,  0x61, struct kvm_irq_level);
pub const KVM_GET_IRQCHIP: u64 = _IOWR(pub KVMIO, 0x62, struct kvm_irqchip);
pub const KVM_SET_IRQCHIP: u64 = _IOR(pub KVMIO,  0x63, struct kvm_irqchip);
pub const KVM_CREATE_PIT: u64 = _IO(pub KVMIO,   0x64);
pub const KVM_GET_PIT: u64 = _IOWR(pub KVMIO, 0x65, struct kvm_pit_state);
pub const KVM_SET_PIT: u64 = _IOR(pub KVMIO,  0x66, struct kvm_pit_state);
pub const KVM_IRQ_LINE_STATUS: u64 = _IOWR(pub KVMIO, 0x67, struct kvm_irq_level);
pub const KVM_REGISTER_COALESCED_MMIO: u64 = \;
			_IOW(pub KVMIO,  0x67, struct kvm_coalesced_mmio_zone)
pub const KVM_UNREGISTER_COALESCED_MMIO: u64 = \;
			_IOW(pub KVMIO,  0x68, struct kvm_coalesced_mmio_zone)
pub const KVM_SET_GSI_ROUTING: u64 = _IOW(pub KVMIO,  0x6a, struct kvm_irq_routing);
pub const KVM_REINJECT_CONTROL: u64 = _IO(pub KVMIO,   0x71);
pub const KVM_IRQFD: u64 = _IOW(pub KVMIO,  0x76, struct kvm_irqfd);
pub const KVM_CREATE_PIT2: u64 = _IOW(pub KVMIO,  0x77, struct kvm_pit_config);
pub const KVM_SET_BOOT_CPU_ID: u64 = _IO(pub KVMIO,   0x78);
pub const KVM_IOEVENTFD: u64 = _IOW(pub KVMIO,  0x79, struct kvm_ioeventfd);
pub const KVM_XEN_HVM_CONFIG: u64 = _IOW(pub KVMIO,  0x7a, struct kvm_xen_hvm_config);
pub const KVM_SET_CLOCK: u64 = _IOW(pub KVMIO,  0x7b, struct kvm_clock_data);
pub const KVM_GET_CLOCK: u64 = _IOR(pub KVMIO,  0x7c, struct kvm_clock_data);
/* Available with KVM_CAP_PIT_STATE2 */
pub const KVM_GET_PIT2: u64 = _IOR(pub KVMIO,  0x9f, struct kvm_pit_state2);
pub const KVM_SET_PIT2: u64 = _IOW(pub KVMIO,  0xa0, struct kvm_pit_state2);
/* Available with KVM_CAP_PPC_GET_PVINFO */
pub const KVM_PPC_GET_PVINFO: u64 = _IOW(pub KVMIO,  0xa1, struct kvm_ppc_pvinfo);
/* Available with KVM_CAP_TSC_CONTROL for a pub vCPU, or with
*  KVM_CAP_VM_TSC_CONTROL to set defaults for a VM */
pub const KVM_SET_TSC_KHZ: u64 = _IO(pub KVMIO,  0xa2);
pub const KVM_GET_TSC_KHZ: u64 = _IO(pub KVMIO,  0xa3);
/* Available with KVM_CAP_SIGNAL_MSI */
pub const KVM_SIGNAL_MSI: u64 = _IOW(pub KVMIO,  0xa5, struct kvm_msi);
/* Available with KVM_CAP_PPC_GET_SMMU_INFO */
pub const KVM_PPC_GET_SMMU_INFO: u64 = _IOR(pub KVMIO,  0xa6, struct kvm_ppc_smmu_info);
/* Available with KVM_CAP_PPC_ALLOC_HTAB */
pub const KVM_PPC_ALLOCATE_HTAB: u64 = _IOWR(pub KVMIO, 0xa7, u32);
pub const KVM_CREATE_SPAPR_TCE: u64 = _IOW(pub KVMIO,  0xa8, struct kvm_create_spapr_tce);
pub const KVM_CREATE_SPAPR_TCE_64: u64 = _IOW(pub KVMIO,  0xa8, \;
				       struct kvm_create_spapr_tce_64)
/* Available with KVM_CAP_RMA */
pub const KVM_ALLOCATE_RMA: u64 = _IOR(pub KVMIO,  0xa9, struct kvm_allocate_rma);
/* Available with KVM_CAP_PPC_HTAB_FD */
pub const KVM_PPC_GET_HTAB_FD: u64 = _IOW(pub KVMIO,  0xaa, struct kvm_get_htab_fd);
/* Available with KVM_CAP_ARM_SET_DEVICE_ADDR */
pub const KVM_ARM_SET_DEVICE_ADDR: u64 = _IOW(pub KVMIO,  0xab, struct kvm_arm_device_addr);
/* Available with KVM_CAP_PPC_RTAS */
pub const KVM_PPC_RTAS_DEFINE_TOKEN: u64 = _IOW(pub KVMIO,  0xac, struct kvm_rtas_token_args);
/* Available with KVM_CAP_SPAPR_RESIZE_HPT */
pub const KVM_PPC_RESIZE_HPT_PREPARE: u64 = _IOR(pub KVMIO, 0xad, struct kvm_ppc_resize_hpt);
pub const KVM_PPC_RESIZE_HPT_COMMIT: u64 = _IOR(pub KVMIO, 0xae, struct kvm_ppc_resize_hpt);
/* Available with KVM_CAP_PPC_MMU_RADIX or KVM_CAP_PPC_MMU_HASH_V3 */
pub const KVM_PPC_CONFIGURE_V3_MMU: u64 = _IOW(pub KVMIO,  0xaf, struct kvm_ppc_mmuv3_cfg);
/* Available with KVM_CAP_PPC_MMU_RADIX */
pub const KVM_PPC_GET_RMMU_INFO: u64 = _IOW(pub KVMIO,  0xb0, struct kvm_ppc_rmmu_info);
/* Available with KVM_CAP_PPC_GET_CPU_CHAR */
pub const KVM_PPC_GET_CPU_CHAR: u64 = _IOR(pub KVMIO,  0xb1, struct kvm_ppc_cpu_u8);
/* Available with KVM_CAP_PMU_EVENT_FILTER */
pub const KVM_SET_PMU_EVENT_FILTER: u64 = _IOW(pub KVMIO,  0xb2, struct kvm_pmu_event_filter);
pub const KVM_PPC_SVM_OFF: u64 = _IO(pub KVMIO,  0xb3);
pub const KVM_ARM_MTE_COPY_TAGS: u64 = _IOR(pub KVMIO,  0xb4, struct kvm_arm_copy_mte_tags);
/* Available with KVM_CAP_COUNTER_OFFSET */
pub const KVM_ARM_SET_COUNTER_OFFSET: u64 = _IOW(pub KVMIO,  0xb5, struct kvm_arm_counter_offset);
pub const KVM_ARM_GET_REG_WRITABLE_MASKS: u64 = _IOR(pub KVMIO,  0xb6, struct reg_mask_range);
/* Available with KVM_CAP_PPC_COMPAT_CAPS */
pub const KVM_PPC_GET_COMPAT_CAPS: u64 = _IO(pub KVMIO,  0xb8);

/* ioctl for vm fd */
pub const KVM_CREATE_DEVICE: u64 = _IOWR(pub KVMIO,  0xe0, struct kvm_create_device);

/* ioctls for fds returned by KVM_CREATE_DEVICE */
pub const KVM_SET_DEVICE_ATTR: u64 = _IOW(pub KVMIO,  0xe1, struct kvm_device_attr);
pub const KVM_GET_DEVICE_ATTR: u64 = _IOW(pub KVMIO,  0xe2, struct kvm_device_attr);
pub const KVM_HAS_DEVICE_ATTR: u64 = _IOW(pub KVMIO,  0xe3, struct kvm_device_attr);

/*
 * ioctls for vcpu fds
 */
pub const KVM_RUN: u64 = _IO(pub KVMIO,   0x80);
pub const KVM_GET_REGS: u64 = _IOR(pub KVMIO,  0x81, struct kvm_regs);
pub const KVM_SET_REGS: u64 = _IOW(pub KVMIO,  0x82, struct kvm_regs);
pub const KVM_GET_SREGS: u64 = _IOR(pub KVMIO,  0x83, struct kvm_sregs);
pub const KVM_SET_SREGS: u64 = _IOW(pub KVMIO,  0x84, struct kvm_sregs);
pub const KVM_TRANSLATE: u64 = _IOWR(pub KVMIO, 0x85, struct kvm_translation);
pub const KVM_INTERRUPT: u64 = _IOW(pub KVMIO,  0x86, struct kvm_interrupt);
pub const KVM_GET_MSRS: u64 = _IOWR(pub KVMIO, 0x88, struct kvm_msrs);
pub const KVM_SET_MSRS: u64 = _IOW(pub KVMIO,  0x89, struct kvm_msrs);
pub const KVM_SET_CPUID: u64 = _IOW(pub KVMIO,  0x8a, struct kvm_cpuid);
pub const KVM_SET_SIGNAL_MASK: u64 = _IOW(pub KVMIO,  0x8b, struct kvm_signal_mask);
pub const KVM_GET_FPU: u64 = _IOR(pub KVMIO,  0x8c, struct kvm_fpu);
pub const KVM_SET_FPU: u64 = _IOW(pub KVMIO,  0x8d, struct kvm_fpu);
pub const KVM_GET_LAPIC: u64 = _IOR(pub KVMIO,  0x8e, struct kvm_lapic_state);
pub const KVM_SET_LAPIC: u64 = _IOW(pub KVMIO,  0x8f, struct kvm_lapic_state);
pub const KVM_SET_CPUID2: u64 = _IOW(pub KVMIO,  0x90, struct kvm_cpuid2);
pub const KVM_GET_CPUID2: u64 = _IOWR(pub KVMIO, 0x91, struct kvm_cpuid2);
/* Available with KVM_CAP_VAPIC */
pub const KVM_TPR_ACCESS_REPORTING: u64 = _IOWR(pub KVMIO, 0x92, struct kvm_tpr_access_ctl);
/* Available with KVM_CAP_VAPIC */
pub const KVM_SET_VAPIC_ADDR: u64 = _IOW(pub KVMIO,  0x93, struct kvm_vapic_addr);
/* valid for virtual machine (for floating interrupt)_and_ vcpu */
pub const KVM_S390_INTERRUPT: u64 = _IOW(pub KVMIO,  0x94, struct kvm_s390_interrupt);
/* store status for s390 */
pub const KVM_S390_STORE_STATUS_NOADDR: u64 = (-1ul);
pub const KVM_S390_STORE_STATUS_PREFIXED: u64 = (-2ul);
pub const KVM_S390_STORE_STATUS: u64 = _IOW(pub KVMIO,  0x95, usize);
/* initial ipl psw for s390 */
pub const KVM_S390_SET_INITIAL_PSW: u64 = _IOW(pub KVMIO,  0x96, struct kvm_s390_psw);
/* initial reset for s390 */
pub const KVM_S390_INITIAL_RESET: u64 = _IO(pub KVMIO,   0x97);
pub const KVM_GET_MP_STATE: u64 = _IOR(pub KVMIO,  0x98, struct kvm_mp_state);
pub const KVM_SET_MP_STATE: u64 = _IOW(pub KVMIO,  0x99, struct kvm_mp_state);
/* Available with KVM_CAP_USER_NMI */
pub const KVM_NMI: u64 = _IO(pub KVMIO,   0x9a);
/* Available with KVM_CAP_SET_GUEST_DEBUG */
pub const KVM_SET_GUEST_DEBUG: u64 = _IOW(pub KVMIO,  0x9b, struct kvm_guest_debug);
/* MCE for x86 */
pub const KVM_X86_SETUP_MCE: u64 = _IOW(pub KVMIO,  0x9c, u64);
pub const KVM_X86_GET_MCE_CAP_SUPPORTED: u64 = _IOR(pub KVMIO,  0x9d, u64);
pub const KVM_X86_SET_MCE: u64 = _IOW(pub KVMIO,  0x9e, struct kvm_x86_mce);
/* Available with KVM_CAP_VCPU_EVENTS */
pub const KVM_GET_VCPU_EVENTS: u64 = _IOR(pub KVMIO,  0x9f, struct kvm_vcpu_events);
pub const KVM_SET_VCPU_EVENTS: u64 = _IOW(pub KVMIO,  0xa0, struct kvm_vcpu_events);
/* Available with KVM_CAP_DEBUGREGS */
pub const KVM_GET_DEBUGREGS: u64 = _IOR(pub KVMIO,  0xa1, struct kvm_debugregs);
pub const KVM_SET_DEBUGREGS: u64 = _IOW(pub KVMIO,  0xa2, struct kvm_debugregs);
/*
 * vcpu version available with KVM_CAP_ENABLE_CAP
 * vm version available with KVM_CAP_ENABLE_CAP_VM
 */
pub const KVM_ENABLE_CAP: u64 = _IOW(pub KVMIO,  0xa3, struct kvm_enable_cap);
/* Available with KVM_CAP_XSAVE */
pub const KVM_GET_XSAVE: u64 = _IOR(pub KVMIO,  0xa4, struct kvm_xsave);
pub const KVM_SET_XSAVE: u64 = _IOW(pub KVMIO,  0xa5, struct kvm_xsave);
/* Available with KVM_CAP_XCRS */
pub const KVM_GET_XCRS: u64 = _IOR(pub KVMIO,  0xa6, struct kvm_xcrs);
pub const KVM_SET_XCRS: u64 = _IOW(pub KVMIO,  0xa7, struct kvm_xcrs);
/* Available with KVM_CAP_SW_TLB */
pub const KVM_DIRTY_TLB: u64 = _IOW(pub KVMIO,  0xaa, struct kvm_dirty_tlb);
/* Available with KVM_CAP_ONE_REG */
pub const KVM_GET_ONE_REG: u64 = _IOW(pub KVMIO,  0xab, struct kvm_one_reg);
pub const KVM_SET_ONE_REG: u64 = _IOW(pub KVMIO,  0xac, struct kvm_one_reg);
/* VM is being stopped by host */
pub const KVM_KVMCLOCK_CTRL: u64 = _IO(pub KVMIO,   0xad);
pub const KVM_ARM_VCPU_INIT: u64 = _IOW(pub KVMIO,  0xae, struct kvm_vcpu_init);
pub const KVM_ARM_PREFERRED_TARGET: u64 = _IOR(pub KVMIO,  0xaf, struct kvm_vcpu_init);
pub const KVM_GET_REG_LIST: u64 = _IOWR(pub KVMIO, 0xb0, struct kvm_reg_list);
/* Available with KVM_CAP_S390_MEM_OP */
pub const KVM_S390_MEM_OP: u64 = _IOW(pub KVMIO,  0xb1, struct kvm_s390_mem_op);
/* Available with KVM_CAP_S390_SKEYS */
pub const KVM_S390_GET_SKEYS: u64 = _IOW(pub KVMIO, 0xb2, struct kvm_s390_skeys);
pub const KVM_S390_SET_SKEYS: u64 = _IOW(pub KVMIO, 0xb3, struct kvm_s390_skeys);
/* Available with KVM_CAP_S390_INJECT_IRQ */
pub const KVM_S390_IRQ: u64 = _IOW(pub KVMIO,  0xb4, struct kvm_s390_irq);
/* Available with KVM_CAP_S390_IRQ_STATE */
pub const KVM_S390_SET_IRQ_STATE: u64 = _IOW(pub KVMIO, 0xb5, struct kvm_s390_irq_state);
pub const KVM_S390_GET_IRQ_STATE: u64 = _IOW(pub KVMIO, 0xb6, struct kvm_s390_irq_state);
/* Available with KVM_CAP_X86_SMM */
pub const KVM_SMI: u64 = _IO(pub KVMIO,   0xb7);
/* Available with KVM_CAP_S390_CMMA_MIGRATION */
pub const KVM_S390_GET_CMMA_BITS: u64 = _IOWR(pub KVMIO, 0xb8, struct kvm_s390_cmma_log);
pub const KVM_S390_SET_CMMA_BITS: u64 = _IOW(pub KVMIO, 0xb9, struct kvm_s390_cmma_log);
/* Memory Encryption Commands */
pub const KVM_MEMORY_ENCRYPT_OP: u64 = _IOWR(pub KVMIO, 0xba, usize);

#[repr(C)]
pub struct kvm_enc_region {
	u64 pub addr;
	u64 pub size;
}

pub const KVM_MEMORY_ENCRYPT_REG_REGION: u64 = _IOR(pub KVMIO, 0xbb, struct kvm_enc_region);
pub const KVM_MEMORY_ENCRYPT_UNREG_REGION: u64 = _IOR(pub KVMIO, 0xbc, struct kvm_enc_region);

/* Available with KVM_CAP_HYPERV_EVENTFD */
pub const KVM_HYPERV_EVENTFD: u64 = _IOW(pub KVMIO,  0xbd, struct kvm_hyperv_eventfd);

/* Available with KVM_CAP_NESTED_STATE */
pub const KVM_GET_NESTED_STATE: u64 = _IOWR(pub KVMIO, 0xbe, struct kvm_nested_state);
pub const KVM_SET_NESTED_STATE: u64 = _IOW(pub KVMIO,  0xbf, struct kvm_nested_state);

/* Available with KVM_CAP_MANUAL_DIRTY_LOG_PROTECT_2 */
pub const KVM_CLEAR_DIRTY_LOG: u64 = _IOWR(pub KVMIO, 0xc0, struct kvm_clear_dirty_log);

/* Available with KVM_CAP_HYPERV_CPUID (vcpu) / KVM_CAP_SYS_HYPERV_CPUID (system) */
pub const KVM_GET_SUPPORTED_HV_CPUID: u64 = _IOWR(pub KVMIO, 0xc1, struct kvm_cpuid2);

/* Available with KVM_CAP_ARM_SVE */
pub const KVM_ARM_VCPU_FINALIZE: u64 = _IOW(pub KVMIO,  0xc2, int);

/* Available with  KVM_CAP_S390_VCPU_RESETS */
pub const KVM_S390_NORMAL_RESET: u64 = _IO(pub KVMIO,   0xc3);
pub const KVM_S390_CLEAR_RESET: u64 = _IO(pub KVMIO,   0xc4);

/* Available with KVM_CAP_S390_PROTECTED */
pub const KVM_S390_PV_COMMAND: u64 = _IOWR(pub KVMIO, 0xc5, struct kvm_pv_cmd);

/* Available with KVM_CAP_X86_MSR_FILTER */
pub const KVM_X86_SET_MSR_FILTER: u64 = _IOW(pub KVMIO,  0xc6, struct kvm_msr_filter);

/* Available with KVM_CAP_DIRTY_LOG_RING */
pub const KVM_RESET_DIRTY_RINGS: u64 = _IO(pub KVMIO, 0xc7);

/* Per-VM Xen attributes */
pub const KVM_XEN_HVM_GET_ATTR: u64 = _IOWR(pub KVMIO, 0xc8, struct kvm_xen_hvm_attr);
pub const KVM_XEN_HVM_SET_ATTR: u64 = _IOW(pub KVMIO,  0xc9, struct kvm_xen_hvm_attr);

/* Per-vCPU Xen attributes */
pub const KVM_XEN_VCPU_GET_ATTR: u64 = _IOWR(pub KVMIO, 0xca, struct kvm_xen_vcpu_attr);
pub const KVM_XEN_VCPU_SET_ATTR: u64 = _IOW(pub KVMIO,  0xcb, struct kvm_xen_vcpu_attr);

/* Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND */
pub const KVM_XEN_HVM_EVTCHN_SEND: u64 = _IOW(pub KVMIO,  0xd0, struct kvm_irq_routing_xen_evtchn);

pub const KVM_GET_SREGS2: u64 = _IOR(pub KVMIO,  0xcc, struct kvm_sregs2);
pub const KVM_SET_SREGS2: u64 = _IOW(pub KVMIO,  0xcd, struct kvm_sregs2);

pub const KVM_DIRTY_LOG_MANUAL_PROTECT_ENABLE: u64 = (1 << 0);
pub const KVM_DIRTY_LOG_INITIALLY_SET: u64 = (1 << 1);

/*
 * Arch needs to define the macro after implementing the dirty ring
 * feature.  KVM_DIRTY_LOG_PAGE_OFFSET should be defined as the
 * starting page offset of the dirty ring structures.
 */
// cfg not: KVM_DIRTY_LOG_PAGE_OFFSET
pub const KVM_DIRTY_LOG_PAGE_OFFSET: u64 = 0;
// endif

/*
 * KVM dirty GFN pub flags, defined as:
 *
 * |---------------+---------------+--------------|
 * | bit 1 (reset) | bit 0 (dirty) | Status       |
 * |---------------+---------------+--------------|
 * |             0 |             0 | Invalid GFN  |
 * |             0 |             1 | Dirty GFN    |
 * |             1 |             X | GFN to reset |
 * |---------------+---------------+--------------|
 *
 * Lifecycle of a dirty GFN goes like:
 *
 *      dirtied         harvested        reset
 * 00 -----------> 01 -------------> 1X -------+
 *  ^                                          |
 *  |                                          |
 *  +------------------------------------------+
 *
 * The userspace program is only responsible for the 01->1X state
 * conversion after harvesting an entry.  pub Also, it must not skip any
 * dirty pub bits, so that dirty bits are always harvested in sequence.
 */
pub const KVM_DIRTY_GFN_F_DIRTY: u64 = _BITUL(0);
pub const KVM_DIRTY_GFN_F_RESET: u64 = _BITUL(1);
pub const KVM_DIRTY_GFN_F_MASK: u64 = 0x3;

/*
 * KVM dirty rings should be mapped at KVM_DIRTY_LOG_PAGE_OFFSET of
 * per-vcpu mmaped regions as an array of struct kvm_dirty_gfn.  The
 * size of the gfn buffer is decided by the first argument when
 * enabling KVM_CAP_DIRTY_LOG_RING.
 */
#[repr(C)]
pub struct kvm_dirty_gfn {
	u32 pub flags;
	u32 pub slot;
	u64 pub offset;
}

pub const KVM_BUS_LOCK_DETECTION_OFF: u64 = (1 << 0);
pub const KVM_BUS_LOCK_DETECTION_EXIT: u64 = (1 << 1);

pub const KVM_PMU_CAP_DISABLE: u64 = (1 << 0);

/**
 * struct kvm_stats_header - Header of per vm/vcpu binary statistics data.
 * @flags: Some extra information for pub header, always 0 for now.
 * @name_size: The size in bytes of the memory which contains statistics
 *             name string including trailing '\0'. The memory is allocated
 *             at the send of statistics descriptor.
 * @num_desc: The number of statistics the vm or vcpu has.
 * @id_offset: The offset of the vm/vcpu stats' id string in the file pointed
 *             by vm/vcpu stats fd.
 * @desc_offset: The offset of the vm/vcpu stats' descriptor block in the file
 *               pointd by vm/vcpu stats fd.
 * @data_offset: The offset of the vm/vcpu stats' data block in the file
 *               pointed by vm/vcpu stats fd.
 *
 * This is the header userspace needs to read from stats fd before any other
 * readings. It is used by userspace to discover all the information about the
 * vm/vcpu's binary statistics.
 * Userspace reads this header from the start of the vm/vcpu's stats fd.
 */
#[repr(C)]
pub struct kvm_stats_header {
	u32 pub flags;
	u32 pub name_size;
	u32 pub num_desc;
	u32 pub id_offset;
	u32 pub desc_offset;
	u32 pub data_offset;
}

pub const KVM_STATS_TYPE_SHIFT: u64 = 0;
pub const KVM_STATS_TYPE_MASK: u64 = (0xF << KVM_STATS_TYPE_SHIFT);
pub const KVM_STATS_TYPE_CUMULATIVE: u64 = (0x0 << KVM_STATS_TYPE_SHIFT);
pub const KVM_STATS_TYPE_INSTANT: u64 = (0x1 << KVM_STATS_TYPE_SHIFT);
pub const KVM_STATS_TYPE_PEAK: u64 = (0x2 << KVM_STATS_TYPE_SHIFT);
pub const KVM_STATS_TYPE_LINEAR_HIST: u64 = (0x3 << KVM_STATS_TYPE_SHIFT);
pub const KVM_STATS_TYPE_LOG_HIST: u64 = (0x4 << KVM_STATS_TYPE_SHIFT);
pub const KVM_STATS_TYPE_MAX: u64 = pub KVM_STATS_TYPE_LOG_HIST;

pub const KVM_STATS_UNIT_SHIFT: u64 = 4;
pub const KVM_STATS_UNIT_MASK: u64 = (0xF << KVM_STATS_UNIT_SHIFT);
pub const KVM_STATS_UNIT_NONE: u64 = (0x0 << KVM_STATS_UNIT_SHIFT);
pub const KVM_STATS_UNIT_BYTES: u64 = (0x1 << KVM_STATS_UNIT_SHIFT);
pub const KVM_STATS_UNIT_SECONDS: u64 = (0x2 << KVM_STATS_UNIT_SHIFT);
pub const KVM_STATS_UNIT_CYCLES: u64 = (0x3 << KVM_STATS_UNIT_SHIFT);
pub const KVM_STATS_UNIT_BOOLEAN: u64 = (0x4 << KVM_STATS_UNIT_SHIFT);
pub const KVM_STATS_UNIT_MAX: u64 = pub KVM_STATS_UNIT_BOOLEAN;

pub const KVM_STATS_BASE_SHIFT: u64 = 8;
pub const KVM_STATS_BASE_MASK: u64 = (0xF << KVM_STATS_BASE_SHIFT);
pub const KVM_STATS_BASE_POW10: u64 = (0x0 << KVM_STATS_BASE_SHIFT);
pub const KVM_STATS_BASE_POW2: u64 = (0x1 << KVM_STATS_BASE_SHIFT);
pub const KVM_STATS_BASE_MAX: u64 = pub KVM_STATS_BASE_POW2;

/**
 * struct kvm_stats_desc - Descriptor of a KVM statistics.
 * @flags: Annotations of the pub stats, like pub type, pub unit, etc.
 * @exponent: Used together with @flags to determine the unit.
 * @size: The number of data items for this stats.
 *        Every data item is of type u64.
 * @offset: The offset of the stats to the start of stat structure in
 *          structure kvm or kvm_vcpu.
 * @bucket_size: A parameter value used for histogram stats. It is only used
 *		for linear histogram pub stats, specifying the size of the pub bucket;
 * @name: The name string for the stats. Its size is indicated by the
 *        &kvm_stats_header->name_size.
 */
#[repr(C)]
pub struct kvm_stats_desc {
	u32 pub flags;
	__s16 pub exponent;
	u16 pub size;
	u32 pub offset;
	u32 pub bucket_size;
// cfg: __KERNEL__
	u8 name[KVM_STATS_NAME_SIZE];
// else
	__DECLARE_FLEX_ARRAY(pub u8, name);
// endif
}

pub const KVM_GET_STATS_FD: u64 = _IO(pub KVMIO,  0xce);

/* Available with KVM_CAP_XSAVE2 */
pub const KVM_GET_XSAVE2: u64 = _IOR(pub KVMIO,  0xcf, struct kvm_xsave);

/* Available with KVM_CAP_S390_PROTECTED_DUMP */
pub const KVM_S390_PV_CPU_COMMAND: u64 = _IOWR(pub KVMIO, 0xd0, struct kvm_pv_cmd);

/* Available with KVM_CAP_X86_NOTIFY_VMEXIT */
pub const KVM_X86_NOTIFY_VMEXIT_ENABLED: u64 = (1u64L << 0);
pub const KVM_X86_NOTIFY_VMEXIT_USER: u64 = (1u64L << 1);

/* Available with KVM_CAP_S390_ZPCI_OP */
pub const KVM_S390_ZPCI_OP: u64 = _IOW(pub KVMIO,  0xd1, struct kvm_s390_zpci_op);

/* Available with KVM_CAP_MEMORY_ATTRIBUTES */
pub const KVM_SET_MEMORY_ATTRIBUTES: u64 = _IOW(pub KVMIO,  0xd2, struct kvm_memory_attributes);

#[repr(C)]
pub struct kvm_memory_attributes {
	u64 pub address;
	u64 pub size;
	u64 pub attributes;
	u64 pub flags;
}

pub const KVM_MEMORY_ATTRIBUTE_PRIVATE: u64 = (1u64L << 3);

pub const KVM_CREATE_GUEST_MEMFD: u64 = _IOWR(pub KVMIO,  0xd4, struct kvm_create_guest_memfd);
pub const GUEST_MEMFD_FLAG_MMAP: u64 = (1u64L << 0);
pub const GUEST_MEMFD_FLAG_INIT_SHARED: u64 = (1u64L << 1);

#[repr(C)]
pub struct kvm_create_guest_memfd {
	u64 pub size;
	u64 pub flags;
	u64 reserved: [pub reserved; 6];
}

pub const KVM_PRE_FAULT_MEMORY: u64 = _IOWR(pub KVMIO, 0xd5, struct kvm_pre_fault_memory);

#[repr(C)]
pub struct kvm_pre_fault_memory {
	u64 pub gpa;
	u64 pub size;
	u64 pub flags;
	u64 padding: [pub padding; 5];
}

// endif /* __LINUX_KVM_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
