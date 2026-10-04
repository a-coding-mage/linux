// SPDX-License-Identifier: GPL-2.0-only
// VM diagnostic arguments are unevaluated when DEBUG_VM is disabled, as in C.
macro_rules! pa_vm_bug_page {
    ($c:expr,$p:expr$(,)?) => {
        #[cfg(CONFIG_DEBUG_VM)]
        {
            rust_pa_bug_page($c, $p);
        }
    };
}
macro_rules! pa_vm_bug {
    ($c:expr) => {
        #[cfg(CONFIG_DEBUG_VM)]
        {
            rust_pa_b_vm_bug($c);
        }
    };
}
macro_rules! pa_vm_warn {
    ($c:expr) => {
        #[cfg(CONFIG_DEBUG_VM)]
        {
            rust_pa_warn($c);
        }
    };
}
macro_rules! pa_vm_warn_page {
    ($c:expr,$p:expr) => {
        #[cfg(CONFIG_DEBUG_VM)]
        {
            rust_pa_warn_page($c, $p);
        }
    };
}
