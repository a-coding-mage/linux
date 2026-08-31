/*
 * Use the __kuser_memory_barrier helper in the CPU helper page. See
 * arch/arm/kernel/entry-armv.S in the kernel source for details.
 */

const __KUSER_MEMORY_BARRIER: usize = 0xffff0fa0;

pub unsafe fn mb() {
    let barrier: unsafe extern "C" fn() =
        core::mem::transmute::<usize, unsafe extern "C" fn()>(__KUSER_MEMORY_BARRIER);
    barrier();
}

pub unsafe fn wmb() {
    let barrier: unsafe extern "C" fn() =
        core::mem::transmute::<usize, unsafe extern "C" fn()>(__KUSER_MEMORY_BARRIER);
    barrier();
}

pub unsafe fn rmb() {
    let barrier: unsafe extern "C" fn() =
        core::mem::transmute::<usize, unsafe extern "C" fn()>(__KUSER_MEMORY_BARRIER);
    barrier();
}
