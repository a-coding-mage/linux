/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture and kernel layers.

pub const POSTED_INTR_ON: usize = 0;
pub const POSTED_INTR_SN: usize = 1;

pub const PID_TABLE_ENTRY_VALID: usize = 1;

pub const NR_PIR_VECTORS: usize = 256;
pub const NR_PIR_WORDS: usize = NR_PIR_VECTORS / BITS_PER_LONG;

/* Posted-Interrupt Descriptor */
#[repr(C, align(64))]
pub struct pi_desc {
    pub pir: [usize; NR_PIR_WORDS], /* Posted interrupt requested */
    pub control: pi_desc_control,
    pub rsvd: [u32; 6],
}

#[repr(C)]
pub struct pi_desc_control_fields {
    pub notifications: u16, /* Suppress and outstanding bits */
    pub nv: u8,
    pub rsvd_2: u8,
    pub ndst: u32,
}

#[repr(C)]
pub union pi_desc_control {
    pub fields: pi_desc_control_fields,
    pub control: u64,
}

/*
 * De-multiplexing posted interrupts is on the performance path, the code
 * below is written to optimize the cache performance based on the following
 * considerations:
 * 1.Posted interrupt descriptor (PID) fits in a cache line that is frequently
 *   accessed by both CPU and IOMMU.
 * 2.During software processing of posted interrupts, the CPU needs to do
 *   natural width read and xchg for checking and clearing posted interrupt
 *   request (PIR), a 256 bit field within the PID.
 * 3.On the other side, the IOMMU does atomic swaps of the entire PID cache
 *   line when posting interrupts and setting control bits.
 * 4.The CPU can access the cache line a magnitude faster than the IOMMU.
 * 5.Each time the IOMMU does interrupt posting to the PIR will evict the PID
 *   cache line. The cache line states after each operation are as follows,
 *   assuming a 64-bit kernel:
 *   CPU\tIOMMU\t\t\tPID Cache line state
 *   ---------------------------------------------------------------
 *...read64\t\t\t\t\texclusive
 *...lock xchg64\t\t\t\tmodified
 *...\t\t\tpost/atomic swap\tinvalid
 *...-------------------------------------------------------------
 *
 * To reduce L1 data cache miss, it is important to avoid contention with
 * IOMMU's interrupt posting/atomic swap. Therefore, a copy of PIR is used
 * when processing posted interrupts in software, e.g. to dispatch interrupt
 * handlers for posted MSIs, or to move interrupts from the PIR to the vIRR
 * in KVM.
 *
 * In addition, the code is trying to keep the cache line state consistent
 * as much as possible. e.g. when making a copy and clearing the PIR
 * (assuming non-zero PIR bits are present in the entire PIR), it does:
 *\t read, read, read, read, xchg, xchg, xchg, xchg
 * instead of:
 *\t read, xchg, read, xchg, read, xchg, read, xchg
 */
#[inline(always)]
pub unsafe fn pi_harvest_pir(pir: *mut usize, pir_vals: *mut usize) -> bool {
    let mut pending: usize = 0;

    for i in 0..NR_PIR_WORDS {
        let value = core::ptr::read_volatile(pir.add(i));
        *pir_vals.add(i) = value;
        pending |= value;
    }

    if pending == 0 {
        return false;
    }

    for i in 0..NR_PIR_WORDS {
        if *pir_vals.add(i) == 0 {
            continue;
        }
        *pir_vals.add(i) = arch_xchg(pir.add(i), 0);
    }

    true
}

#[inline]
pub unsafe fn pi_test_and_set_on(pi_desc: *mut pi_desc) -> bool {
    test_and_set_bit(POSTED_INTR_ON, &mut (*pi_desc).control as *mut _ as *mut usize)
}

#[inline]
pub unsafe fn pi_test_and_clear_on(pi_desc: *mut pi_desc) -> bool {
    test_and_clear_bit(POSTED_INTR_ON, &mut (*pi_desc).control as *mut _ as *mut usize)
}

#[inline]
pub unsafe fn pi_test_and_clear_sn(pi_desc: *mut pi_desc) -> bool {
    test_and_clear_bit(POSTED_INTR_SN, &mut (*pi_desc).control as *mut _ as *mut usize)
}

#[inline]
pub unsafe fn pi_test_and_set_pir(vector: usize, pi_desc: *mut pi_desc) -> bool {
    test_and_set_bit(vector, (*pi_desc).pir.as_mut_ptr())
}

#[inline]
pub unsafe fn pi_is_pir_empty(pi_desc: *mut pi_desc) -> bool {
    bitmap_empty((*pi_desc).pir.as_ptr(), NR_VECTORS)
}

#[inline]
pub unsafe fn pi_set_sn(pi_desc: *mut pi_desc) {
    set_bit(POSTED_INTR_SN, &mut (*pi_desc).control as *mut _ as *mut usize);
}

#[inline]
pub unsafe fn pi_set_on(pi_desc: *mut pi_desc) {
    set_bit(POSTED_INTR_ON, &mut (*pi_desc).control as *mut _ as *mut usize);
}

#[inline]
pub unsafe fn pi_clear_on(pi_desc: *mut pi_desc) {
    clear_bit(POSTED_INTR_ON, &mut (*pi_desc).control as *mut _ as *mut usize);
}

#[inline]
pub unsafe fn pi_clear_sn(pi_desc: *mut pi_desc) {
    clear_bit(POSTED_INTR_SN, &mut (*pi_desc).control as *mut _ as *mut usize);
}

#[inline]
pub unsafe fn pi_test_on(pi_desc: *mut pi_desc) -> bool {
    test_bit(POSTED_INTR_ON, &mut (*pi_desc).control as *mut _ as *mut usize)
}

#[inline]
pub unsafe fn pi_test_sn(pi_desc: *mut pi_desc) -> bool {
    test_bit(POSTED_INTR_SN, &mut (*pi_desc).control as *mut _ as *mut usize)
}

#[inline]
pub unsafe fn pi_test_pir(vector: usize, pi_desc: *mut pi_desc) -> bool {
    test_bit(vector, (*pi_desc).pir.as_mut_ptr())
}

/* Non-atomic helpers */
#[inline]
pub unsafe fn __pi_set_sn(pi_desc: *mut pi_desc) {
    (*pi_desc).control.fields.notifications |= 1u16 << POSTED_INTR_SN;
}

#[inline]
pub unsafe fn __pi_clear_sn(pi_desc: *mut pi_desc) {
    (*pi_desc).control.fields.notifications &= !(1u16 << POSTED_INTR_SN);
}

#[cfg(CONFIG_X86_POSTED_MSI)]
/* Not all external vectors are subject to interrupt remapping. */
#[inline]
pub unsafe fn pi_pending_this_cpu(vector: u32) -> bool {
    let pid = this_cpu_ptr(&posted_msi_pi_desc);
    if warn_on_once(vector > NR_VECTORS as u32 || vector < FIRST_EXTERNAL_VECTOR) {
        return false;
    }
    test_bit(vector as usize, (*pid).pir.as_mut_ptr())
}

extern "C" {
    pub fn intel_posted_msi_init();
}

#[cfg(not(CONFIG_X86_POSTED_MSI))]
#[inline]
pub fn pi_pending_this_cpu(_vector: u32) -> bool { false }

#[cfg(not(CONFIG_X86_POSTED_MSI))]
#[inline]
pub fn intel_posted_msi_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
