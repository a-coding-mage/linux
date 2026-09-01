/* SPDX-License-Identifier: GPL-2.0 */

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! barrier {
    () => {
        unsafe {
            core::arch::asm!("", options(nostack, preserves_flags));
        }
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! virt_mb {
    () => {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst)
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! virt_rmb {
    () => {
        barrier!()
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! virt_wmb {
    () => {
        barrier!()
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! virt_store_mb {
    ($var:expr, $value:expr) => {{
        /*
         * Atomic store should be enough, but gcc generates worse code in that
         * case.
         */
        let virt_store_mb_value = $value;
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        $var = virt_store_mb_value;
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        barrier!();
    }};
}

/* Weak barriers should be used. If not - it's a bug */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! mb {
    () => {
        std::process::abort()
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! dma_rmb {
    () => {
        std::process::abort()
    };
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! dma_wmb {
    () => {
        std::process::abort()
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! dmb {
    ($opt:ident) => {
        unsafe {
            core::arch::asm!(concat!("dmb ", stringify!($opt)), options(nostack, preserves_flags));
        }
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! virt_mb {
    () => {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst)
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! virt_rmb {
    () => {
        dmb!(ishld)
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! virt_wmb {
    () => {
        dmb!(ishst)
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! virt_store_mb {
    ($var:expr, $value:expr) => {{
        core::ptr::write_volatile(core::ptr::addr_of_mut!($var), $value);
        dmb!(ish);
    }};
}

/* Weak barriers should be used. If not - it's a bug */
#[cfg(target_arch = "aarch64")]
macro_rules! mb {
    () => {
        std::process::abort()
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! dma_rmb {
    () => {
        std::process::abort()
    };
}

#[cfg(target_arch = "aarch64")]
macro_rules! dma_wmb {
    () => {
        std::process::abort()
    };
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("Please fill in barrier macros");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
