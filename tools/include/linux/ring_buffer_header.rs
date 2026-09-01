// Original dependencies:
// #include <asm/barrier.h>
// #include <linux/perf_event.h>

/*
 * Contract with kernel for walking the perf ring buffer from
 * user space requires the following barrier pairing (quote
 * from kernel/events/ring_buffer.c):
 *
 *   Since the mmap() consumer (userspace) can run on a
 *   different CPU:
 *
 *   kernel                             user
 *
 *   if (LOAD ->data_tail) {            LOAD ->data_head
 *                      (A)             smp_rmb()       (C)
 *      STORE $data                     LOAD $data
 *      smp_wmb()       (B)             smp_mb()        (D)
 *      STORE ->data_head               STORE ->data_tail
 *   }
 *
 *   Where A pairs with D, and B pairs with C.
 *
 *   In our case A is a control dependency that separates the
 *   load of the ->data_tail and the stores of $data. In case
 *   ->data_tail indicates there is no room in the buffer to
 *   store $data we do not.
 *
 *   D needs to be a full barrier since it separates the data
 *   READ from the tail WRITE.
 *
 *   For B a WMB is sufficient since it separates two WRITEs,
 *   and for C an RMB is sufficient since it separates two READs.
 *
 * Note, instead of B, C, D we could also use smp_store_release()
 * in B and D as well as smp_load_acquire() in C.
 *
 * However, this optimization does not make sense for all kernel
 * supported architectures since for a fair number it would
 * resolve into READ_ONCE() + smp_mb() pair for smp_load_acquire(),
 * and smp_mb() + WRITE_ONCE() pair for smp_store_release().
 *
 * Thus for those smp_wmb() in B and smp_rmb() in C would still
 * be less expensive. For the case of D this has either the same
 * cost or is less expensive, for example, due to TSO x86 can
 * avoid the CPU barrier entirely.
 */

pub unsafe fn ring_buffer_read_head(base: *mut perf_event_mmap_page) -> u64 {
    /*
     * Architectures where smp_load_acquire() does not fallback to
     * READ_ONCE() + smp_mb() pair.
     */
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "ia64",
        all(target_arch = "sparc64", target_pointer_width = "64"),
        target_arch = "riscv32",
        target_arch = "riscv64"
    ))]
    {
        return unsafe { smp_load_acquire(core::ptr::addr_of!((*base).data_head)) };
    }

    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "ia64",
        all(target_arch = "sparc64", target_pointer_width = "64"),
        target_arch = "riscv32",
        target_arch = "riscv64"
    )))]
    {
        let head: u64 = unsafe { READ_ONCE(core::ptr::addr_of!((*base).data_head)) };

        unsafe { smp_rmb() };
        return head;
    }
}

pub unsafe fn ring_buffer_write_tail(base: *mut perf_event_mmap_page, tail: u64) {
    unsafe {
        smp_store_release(core::ptr::addr_of_mut!((*base).data_tail), tail);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
