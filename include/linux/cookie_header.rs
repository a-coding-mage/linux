/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux atomic, percpu, and local
// implementations are intentionally left external to this translation.

#[repr(C, align(16))]
pub struct pcpu_gen_cookie {
    pub nesting: local_t,
    pub last: u64,
}

#[repr(C)]
pub struct gen_cookie {
    pub local: *mut pcpu_gen_cookie,
    pub forward_last: atomic64_t,
    pub reverse_last: atomic64_t,
}

pub const COOKIE_LOCAL_BATCH: u64 = 4096;

// C macro equivalent.  DEFINE_PER_CPU, ATOMIC64_INIT, and the percpu
// representation are provided by the surrounding kernel translation.
// DEFINE_COOKIE(name) declares a static per-CPU `pcpu_gen_cookie` named
// `__name` and a static `gen_cookie` named `name`, initialized with that
// per-CPU object and zero atomic counters.  Rust macro identifiers cannot
// concatenate in the same way as the C preprocessor; callers may provide the
// two identifiers explicitly when expressing this declaration.

#[inline(always)]
pub unsafe fn gen_cookie_next(gc: *mut gen_cookie) -> u64 {
    let local: *mut pcpu_gen_cookie = this_cpu_ptr((*gc).local);
    let mut val: u64;

    if likely(local_inc_return(&mut (*local).nesting) == 1) {
        val = (*local).last;
        if __is_defined(CONFIG_SMP)
            && unlikely((val & (COOKIE_LOCAL_BATCH - 1)) == 0)
        {
            let next: i64 = atomic64_add_return(
                COOKIE_LOCAL_BATCH as i64,
                &mut (*gc).forward_last,
            );
            val = (next as u64).wrapping_sub(COOKIE_LOCAL_BATCH);
        }
        val = val.wrapping_add(1);
        (*local).last = val;
    } else {
        val = atomic64_dec_return(&mut (*gc).reverse_last) as u64;
    }
    local_dec(&mut (*local).nesting);
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
