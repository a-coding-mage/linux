/*
 * Note that cond_break can only be portably used in the body of a breakable
 * construct, whereas can_loop can be used anywhere.
 */

/*
 * C source condition:
 *   #ifdef __BPF_FEATURE_MAY_GOTO
 */
#[cfg(__BPF_FEATURE_MAY_GOTO)]
macro_rules! can_loop {
    () => {{
        let mut ret: bool = true;
        unsafe {
            core::arch::asm!(
                "may_goto {l_break}",
                l_break = label {
                    ret = false;
                },
                options(nostack)
            );
        }
        ret
    }};
}

#[cfg(__BPF_FEATURE_MAY_GOTO)]
macro_rules! __cond_break {
    ($expr:expr) => {{
        unsafe {
            core::arch::asm!(
                "may_goto {l_break}",
                l_break = label {
                    $expr;
                },
                options(nostack)
            );
        }
    }};
}

/*
 * C source condition:
 *   #else
 *   #if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
 */
#[cfg(all(not(__BPF_FEATURE_MAY_GOTO), target_endian = "little"))]
macro_rules! can_loop {
    () => {{
        let mut ret: bool = true;
        unsafe {
            core::arch::asm!(
                "1:.byte 0xe5",
                ".byte 0",
                ".long (({l_break} - 1b - 8) / 8) & 0xffff",
                ".short 0",
                l_break = label {
                    ret = false;
                },
                options(nostack)
            );
        }
        ret
    }};
}

#[cfg(all(not(__BPF_FEATURE_MAY_GOTO), target_endian = "little"))]
macro_rules! __cond_break {
    ($expr:expr) => {{
        unsafe {
            core::arch::asm!(
                "1:.byte 0xe5",
                ".byte 0",
                ".long (({l_break} - 1b - 8) / 8) & 0xffff",
                ".short 0",
                l_break = label {
                    $expr;
                },
                options(nostack)
            );
        }
    }};
}

/*
 * C source condition:
 *   #else
 */
#[cfg(all(not(__BPF_FEATURE_MAY_GOTO), not(target_endian = "little")))]
macro_rules! can_loop {
    () => {{
        let mut ret: bool = true;
        unsafe {
            core::arch::asm!(
                "1:.byte 0xe5",
                ".byte 0",
                ".long ((({l_break} - 1b - 8) / 8) & 0xffff) << 16",
                ".short 0",
                l_break = label {
                    ret = false;
                },
                options(nostack)
            );
        }
        ret
    }};
}

#[cfg(all(not(__BPF_FEATURE_MAY_GOTO), not(target_endian = "little")))]
macro_rules! __cond_break {
    ($expr:expr) => {{
        unsafe {
            core::arch::asm!(
                "1:.byte 0xe5",
                ".byte 0",
                ".long ((({l_break} - 1b - 8) / 8) & 0xffff) << 16",
                ".short 0",
                l_break = label {
                    $expr;
                },
                options(nostack)
            );
        }
    }};
}

macro_rules! cond_break {
    () => {
        __cond_break!(break)
    };
}

macro_rules! cond_break_label {
    ($label:lifetime) => {
        __cond_break!(break $label)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
