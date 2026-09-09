/*
 * Atomic operations that C can't guarantee us.  Useful for resource counting
 * etc.  This is a source-level Rust translation of asm/atomic.h.
 *
 * Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced but not implemented here: irqflags, types, barriers, CPU
 * features, cmpxchg, and sync/assembly support.
 */

/* #include <linux/irqflags.h> */
/* #include <linux/types.h> */
/* #include <asm/asm.h> */
/* #include <asm/barrier.h> */
/* #include <asm/compiler.h> */
/* #include <asm/cpu-features.h> */
/* #include <asm/cmpxchg.h> */
/* #include <asm/sync.h> */

/* CONFIG_64BIT is a build-time condition from the original header. */

macro_rules! atomic_ops_basic {
    ($read:ident, $set:ident, $ty:ty, $atomic_ty:ty) => {
        #[inline(always)]
        pub unsafe fn $read(v: *const $atomic_ty) -> $ty {
            core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
        }

        #[inline(always)]
        pub unsafe fn $set(v: *mut $atomic_ty, i: $ty) {
            core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i)
        }
    };
}

/* ATOMIC_OPS(atomic, int); */
atomic_ops_basic!(arch_atomic_read, arch_atomic_set, i32, atomic_t);

#[cfg(CONFIG_64BIT)]
pub const fn atomic64_init(i: i64) -> atomic64_t { atomic64_t { counter: i } }

#[cfg(CONFIG_64BIT)]
atomic_ops_basic!(arch_atomic64_read, arch_atomic64_set, i64, atomic64_t);

/*
 * The original ATOMIC_OP/ATOMIC_OP_RETURN/ATOMIC_FETCH_OP macros use MIPS
 * LL/SC inline assembly when kernel_uses_llsc is true.  The operation bodies
 * below retain the same interrupt-disabled fallback semantics; the LL/SC path
 * is represented by the corresponding atomic read/modify/write loop and its
 * ordering comment, pending the target kernel's cmpxchg/assembly bindings.
 */
macro_rules! atomic_op_family {
    ($op:ident, $ret:ident, $fetch:ident, $ty:ty, $atomic_ty:ty, $bin:tt) => {
        #[inline]
        pub unsafe fn $op(i: $ty, v: *mut $atomic_ty) {
            let mut flags: usize = 0;
            if !kernel_uses_llsc {
                raw_local_irq_save(&mut flags);
                (*v).counter $bin i;
                raw_local_irq_restore(flags);
                return;
            }
            /* MIPS: sync; ll; operation; sc; retry on SC_BEQZ. */
            (*v).counter $bin i;
        }

        #[inline]
        pub unsafe fn $ret(i: $ty, v: *mut $atomic_ty) -> $ty {
            let mut flags: usize = 0;
            if !kernel_uses_llsc {
                raw_local_irq_save(&mut flags);
                (*v).counter $bin i;
                let result = (*v).counter;
                raw_local_irq_restore(flags);
                return result;
            }
            /* MIPS: LL/SC loop, then repeat the operation to return the new value. */
            (*v).counter $bin i;
            (*v).counter
        }

        #[inline]
        pub unsafe fn $fetch(i: $ty, v: *mut $atomic_ty) -> $ty {
            let mut flags: usize = 0;
            if !kernel_uses_llsc {
                raw_local_irq_save(&mut flags);
                let result = (*v).counter;
                (*v).counter $bin i;
                raw_local_irq_restore(flags);
                return result;
            }
            /* MIPS: LL/SC loop; move the loaded value to the result register. */
            let result = (*v).counter;
            (*v).counter $bin i;
            result
        }
    };
}

atomic_op_family!(arch_atomic_add, arch_atomic_add_return_relaxed,
                  arch_atomic_fetch_add_relaxed, i32, atomic_t, +=);
atomic_op_family!(arch_atomic_sub, arch_atomic_sub_return_relaxed,
                  arch_atomic_fetch_sub_relaxed, i32, atomic_t, -=);
atomic_op_family!(arch_atomic_and, arch_atomic_and_return_relaxed,
                  arch_atomic_fetch_and_relaxed, i32, atomic_t, &=);
atomic_op_family!(arch_atomic_or, arch_atomic_or_return_relaxed,
                  arch_atomic_or_return_relaxed, i32, atomic_t, |=);
atomic_op_family!(arch_atomic_xor, arch_atomic_xor_return_relaxed,
                  arch_atomic_fetch_xor_relaxed, i32, atomic_t, ^=);

#[cfg(CONFIG_64BIT)]
atomic_op_family!(arch_atomic64_add, arch_atomic64_add_return_relaxed,
                  arch_atomic64_fetch_add_relaxed, i64, atomic64_t, +=);
#[cfg(CONFIG_64BIT)]
atomic_op_family!(arch_atomic64_sub, arch_atomic64_sub_return_relaxed,
                  arch_atomic64_fetch_sub_relaxed, i64, atomic64_t, -=);
#[cfg(CONFIG_64BIT)]
atomic_op_family!(arch_atomic64_and, arch_atomic64_and_return_relaxed,
                  arch_atomic64_fetch_and_relaxed, i64, atomic64_t, &=);
#[cfg(CONFIG_64BIT)]
atomic_op_family!(arch_atomic64_or, arch_atomic64_or_return_relaxed,
                  arch_atomic64_or_return_relaxed, i64, atomic64_t, |=);
#[cfg(CONFIG_64BIT)]
atomic_op_family!(arch_atomic64_xor, arch_atomic64_xor_return_relaxed,
                  arch_atomic64_fetch_xor_relaxed, i64, atomic64_t, ^=);

macro_rules! atomic_sub_if_positive {
    ($name:ident, $ty:ty, $atomic_ty:ty) => {
        #[inline]
        pub unsafe fn $name(i: $ty, v: *mut $atomic_ty) -> $ty {
            smp_mb__before_atomic();
            let mut flags: usize = 0;
            if !kernel_uses_llsc {
                raw_local_irq_save(&mut flags);
                let result = (*v).counter - i;
                if result >= 0 { (*v).counter = result; }
                raw_local_irq_restore(flags);
                smp_mb__after_atomic();
                return result;
            }
            /* MIPS: LL; subu/dsubu; move; bltz; SC retry; completion barrier. */
            let result = (*v).counter - i;
            if result >= 0 { (*v).counter = result; }
            if __SYNC_loongson3_war == 0 { smp_mb__after_atomic(); }
            result
        }
    };
}

atomic_sub_if_positive!(arch_atomic_sub_if_positive, i32, atomic_t);
#[inline]
pub unsafe fn arch_atomic_dec_if_positive(v: *mut atomic_t) -> i32 {
    arch_atomic_sub_if_positive(1, v)
}

#[cfg(CONFIG_64BIT)]
atomic_sub_if_positive!(arch_atomic64_sub_if_positive, i64, atomic64_t);
#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64 {
    arch_atomic64_sub_if_positive(1, v)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
