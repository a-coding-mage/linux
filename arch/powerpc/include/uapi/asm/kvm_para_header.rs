/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright IBM Corp. 2008
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

/* Translated from the PowerPC UAPI KVM parameter header. */

/*
 * Additions to this struct must only occur at the end, and should be
 * accompanied by a KVM_MAGIC_FEAT flag to advertise that they are present
 * (albeit not necessarily relevant to the current target hardware platform).
 *
 * Struct fields are always 32 or 64 bit aligned, depending on them being 32
 * or 64 bit wide respectively.
 *
 * See Documentation/virt/kvm/ppc-pv.rst
 */
#[repr(C)]
pub struct kvm_vcpu_arch_shared {
    pub scratch1: u64,
    pub scratch2: u64,
    pub scratch3: u64,
    pub critical: u64, // Guest may not get interrupts if == r1
    pub sprg0: u64,
    pub sprg1: u64,
    pub sprg2: u64,
    pub sprg3: u64,
    pub srr0: u64,
    pub srr1: u64,
    pub dar: u64, // dear on BookE
    pub msr: u64,
    pub dsisr: u32,
    pub int_pending: u32, // Tells the guest if we have an interrupt
    pub sr: [u32; 16],
    pub mas0: u32,
    pub mas1: u32,
    pub mas7_3: u64,
    pub mas2: u64,
    pub mas4: u32,
    pub mas6: u32,
    pub esr: u32,
    pub pir: u32,

    /*
     * SPRG4-7 are user-readable, so we can only keep these consistent
     * between the shared area and the real registers when there's an
     * intervening exit to KVM.  This also applies to SPRG3 on some
     * chips.
     *
     * This suffices for access by guest userspace, since in PR-mode
     * KVM, an exit must occur when changing the guest's MSR[PR].
     * If the guest kernel writes to SPRG3-7 via the shared area, it
     * must also use the shared area for reading while in kernel space.
     */
    pub sprg4: u64,
    pub sprg5: u64,
    pub sprg6: u64,
    pub sprg7: u64,
}

pub const KVM_SC_MAGIC_R0: u32 = 0x4b564d21; // "KVM!"

/* Requires the externally supplied _EV_HCALL_TOKEN and EV_KVM_VENDOR_ID. */
#[macro_export]
macro_rules! KVM_HCALL_TOKEN {
    ($num:expr) => {
        _EV_HCALL_TOKEN!(EV_KVM_VENDOR_ID, $num)
    };
}

pub const KVM_FEATURE_MAGIC_PAGE: u32 = 1;

/* Magic page flags from host to guest */

pub const KVM_MAGIC_FEAT_SR: u32 = 1 << 0;

/* MASn, ESR, PIR, and high SPRGs */
pub const KVM_MAGIC_FEAT_MAS0_TO_SPRG7: u32 = 1 << 1;

/* Magic page flags from guest to host */

pub const MAGIC_PAGE_FLAG_NOT_MAPPED_NX: u32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
