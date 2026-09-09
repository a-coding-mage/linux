/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Today's hack: quantum tunneling in structs
 *
 * 'entries' and 'term' are never anywhere referenced by word in code. In fact,
 * they serve as the hanging-off data accessed through repl.data[].
 */

/* tbl has the following structure equivalent, but is C99 compliant:
 * struct {
 *  struct type##_replace repl;
 *  struct type##_standard entries[nhooks];
 *  struct type##_error term;
 * } *tbl;
 */

/* The original macro uses token-pasted type names (typ2##_ERROR_INIT and
 * typ2##_STANDARD_INIT). Rust callers provide the corresponding initializer
 * expressions explicitly.
 */
macro_rules! xt_alloc_initial_table {
    ($type:ty, $error_init:expr, $standard_init:expr, $info:expr) => {{
        let mut hook_mask: u32 = $info.valid_hooks;
        let nhooks: usize = hweight32(hook_mask) as usize;
        let mut bytes: usize = 0;
        let mut hooknum: usize = 0;
        let mut i: usize = 0;

        #[repr(C)]
        struct XtInitialTable<T, E> {
            repl: T,
            entries: [T; 0],
            _marker: core::marker::PhantomData<E>,
        }

        let term_offset = (core::mem::size_of::<XtInitialTable<$type, _>>()
            + core::mem::align_of::<$type>() - 1)
            & !(core::mem::align_of::<$type>() - 1);
        let tbl = kzalloc(term_offset + core::mem::size_of_val(&$error_init), GFP_KERNEL);
        if tbl.is_null() {
            core::ptr::null_mut()
        } else {
            let term = unsafe { (tbl as *mut u8).add(term_offset) as *mut _ };
            unsafe {
                strscpy((*tbl).repl.name.as_mut_ptr(), $info.name.as_ptr());
                *term = $error_init;
                (*tbl).repl.valid_hooks = hook_mask;
                (*tbl).repl.num_entries = nhooks + 1;
                (*tbl).repl.size = nhooks * core::mem::size_of::<$type>()
                    + core::mem::size_of_val(&$error_init);
                while hook_mask != 0 {
                    if (hook_mask & 1) != 0 {
                        (*tbl).repl.hook_entry[hooknum] = bytes;
                        (*tbl).repl.underflow[hooknum] = bytes;
                        /* The flexible-array member is the hanging-off data
                         * immediately following the replacement structure. */
                        let entries = (tbl as *mut u8)
                            .add(core::mem::size_of::<XtInitialTable<$type, _>>())
                            as *mut $type;
                        entries.add(i).write($standard_init);
                        i += 1;
                        bytes += core::mem::size_of::<$type>();
                    }
                    hook_mask >>= 1;
                    hooknum += 1;
                }
            }
            tbl
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
