// SPDX-License-Identifier: GPL-2.0
/*
 * Generic interfaces for unwinding user space
 */
// Linux kernel headers supplying the declarations used below are external
// dependencies of this translation.

#[inline]
unsafe fn get_user_word(word: *mut ::core::ffi::c_ulong,
                        base: ::core::ffi::c_ulong,
                        off: ::core::ffi::c_int,
                        ws: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let addr = (base as *mut ::core::ffi::c_ulong).offset(off as isize);

    // CONFIG_COMPAT conditional retained from the C implementation.
    #[cfg(CONFIG_COMPAT)]
    if ws == core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint {
        let mut data: ::core::ffi::c_uint = 0;
        let ret = get_user(data, addr as *mut ::core::ffi::c_uint);
        *word = data as ::core::ffi::c_ulong;
        return ret;
    }

    get_user(*word, addr)
}

unsafe fn unwind_user_next_common(
    state: *mut unwind_user_state,
    frame: *const unwind_user_frame,
) -> ::core::ffi::c_int {
    let (mut cfa, mut fp, mut ra): (::core::ffi::c_ulong,
                                    ::core::ffi::c_ulong,
                                    ::core::ffi::c_ulong);

    // Get the Canonical Frame Address (CFA)
    if (*frame).use_fp {
        if (*state).fp < (*state).sp {
            return -EINVAL;
        }
        cfa = (*state).fp;
    } else {
        cfa = (*state).sp;
    }
    cfa = cfa.wrapping_add((*frame).cfa_off);

    // Make sure that stack is not going in wrong direction
    if cfa <= (*state).sp {
        return -EINVAL;
    }

    // Make sure that the address is word aligned
    if cfa & ((*state).ws - 1) != 0 {
        return -EINVAL;
    }

    // Get the Return Address (RA)
    if get_user_word(&mut ra, cfa, (*frame).ra_off, (*state).ws) != 0 {
        return -EINVAL;
    }

    // Get the Frame Pointer (FP)
    if (*frame).fp_off != 0
        && get_user_word(&mut fp, cfa, (*frame).fp_off, (*state).ws) != 0
    {
        return -EINVAL;
    }

    (*state).ip = ra;
    (*state).sp = cfa;
    if (*frame).fp_off != 0 {
        (*state).fp = fp;
    }
    (*state).topmost = false;
    0
}

unsafe fn unwind_user_next_fp(state: *mut unwind_user_state) -> ::core::ffi::c_int {
    let regs = task_pt_regs(current);

    if (*state).topmost && unwind_user_at_function_start(regs) {
        let fp_entry_frame = unwind_user_frame {
            ARCH_INIT_USER_FP_ENTRY_FRAME!( (*state).ws )
        };
        return unwind_user_next_common(state, &fp_entry_frame);
    }

    let fp_frame = unwind_user_frame {
        ARCH_INIT_USER_FP_FRAME!( (*state).ws )
    };
    unwind_user_next_common(state, &fp_frame)
}

unsafe fn unwind_user_next(state: *mut unwind_user_state) -> ::core::ffi::c_int {
    let mut iter_mask = (*state).available_types;
    let mut bit: ::core::ffi::c_uint = 0;

    if (*state).done {
        return -EINVAL;
    }

    while bit < NR_UNWIND_USER_TYPE_BITS {
        if iter_mask & (1 << bit) != 0 {
            let ty = 1 << bit;
            (*state).current_type = ty;
            match ty {
                UNWIND_USER_TYPE_FP => {
                    if unwind_user_next_fp(state) == 0 {
                        return 0;
                    }
                }
                _ => {
                    WARN_ONCE!(true, "Undefined unwind bit %d", bit);
                }
            }
        }
        bit += 1;
    }

    // No successful unwind method.
    (*state).current_type = UNWIND_USER_TYPE_NONE;
    (*state).done = true;
    -EINVAL
}

unsafe fn unwind_user_start(state: *mut unwind_user_state) -> ::core::ffi::c_int {
    let regs = task_pt_regs(current);

    memset(state as *mut _, 0, core::mem::size_of::<unwind_user_state>());

    if ((*current).flags & PF_KTHREAD) != 0 || !user_mode(regs) {
        (*state).done = true;
        return -EINVAL;
    }

    // IS_ENABLED(CONFIG_HAVE_UNWIND_USER_FP)
    if cfg!(CONFIG_HAVE_UNWIND_USER_FP) {
        (*state).available_types |= UNWIND_USER_TYPE_FP;
    }

    (*state).ip = instruction_pointer(regs);
    (*state).sp = user_stack_pointer(regs);
    (*state).fp = frame_pointer(regs);
    (*state).ws = unwind_user_word_size(regs);
    if (*state).ws == 0 {
        (*state).done = true;
        return -EINVAL;
    }
    (*state).topmost = true;
    0
}

pub unsafe fn unwind_user(
    trace: *mut unwind_stacktrace,
    max_entries: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: unwind_user_state = core::mem::zeroed();

    (*trace).nr = 0;

    if max_entries == 0 {
        return -EINVAL;
    }

    if ((*current).flags & PF_KTHREAD) != 0 {
        return 0;
    }

    unwind_user_start(&mut state);
    while !state.done {
        (*trace).entries[(*trace).nr as usize] = state.ip;
        (*trace).nr += 1;
        if (*trace).nr >= max_entries {
            break;
        }
        unwind_user_next(&mut state);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
