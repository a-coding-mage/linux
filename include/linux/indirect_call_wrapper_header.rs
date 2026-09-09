/* SPDX-License-Identifier: GPL-2.0 */

// C header guard omitted; this file is intended to be included once.
// CONFIG_MITIGATION_RETPOLINE is represented by the Rust feature of the same name.
#[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
macro_rules! INDIRECT_CALL_1 {
    ($f:expr, $f1:expr, $($args:expr),* $(,)?) => {{
        let __f1 = $f;
        if likely!(__f1 == $f1) { $f1($($args),*) } else { __f1($($args),*) }
    }};
}

#[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
macro_rules! INDIRECT_CALL_2 {
    ($f:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => {{
        let __f2 = $f;
        if likely!(__f2 == $f2) { $f2($($args),*) } else {
            INDIRECT_CALL_1!(__f2, $f1, $($args),*)
        }
    }};
}

#[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
macro_rules! INDIRECT_CALL_3 {
    ($f:expr, $f3:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => {{
        let __f3 = $f;
        if likely!(__f3 == $f3) { $f3($($args),*) } else {
            INDIRECT_CALL_2!(__f3, $f2, $f1, $($args),*)
        }
    }};
}

#[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
macro_rules! INDIRECT_CALL_4 {
    ($f:expr, $f4:expr, $f3:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => {{
        let __f4 = $f;
        if likely!(__f4 == $f4) { $f4($($args),*) } else {
            INDIRECT_CALL_3!(__f4, $f3, $f2, $f1, $($args),*)
        }
    }};
}

#[cfg(not(feature = "CONFIG_MITIGATION_RETPOLINE"))]
macro_rules! INDIRECT_CALL_1 { ($f:expr, $f1:expr, $($args:expr),* $(,)?) => { $f($($args),*) }; }
#[cfg(not(feature = "CONFIG_MITIGATION_RETPOLINE"))]
macro_rules! INDIRECT_CALL_2 { ($f:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => { $f($($args),*) }; }
#[cfg(not(feature = "CONFIG_MITIGATION_RETPOLINE"))]
macro_rules! INDIRECT_CALL_3 { ($f:expr, $f3:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => { $f($($args),*) }; }
#[cfg(not(feature = "CONFIG_MITIGATION_RETPOLINE"))]
macro_rules! INDIRECT_CALL_4 { ($f:expr, $f4:expr, $f3:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => { $f($($args),*) }; }

macro_rules! INDIRECT_CALLABLE_DECLARE { ($f:ident) => { $f }; }
#[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
macro_rules! INDIRECT_CALLABLE_SCOPE { () => {}; }
#[cfg(not(feature = "CONFIG_MITIGATION_RETPOLINE"))]
macro_rules! INDIRECT_CALLABLE_SCOPE { () => { static }; }
#[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
macro_rules! EXPORT_INDIRECT_CALLABLE { ($f:ident) => { EXPORT_SYMBOL!($f) }; }
#[cfg(not(feature = "CONFIG_MITIGATION_RETPOLINE"))]
macro_rules! EXPORT_INDIRECT_CALLABLE { ($f:ident) => {}; }

// IS_ENABLED(CONFIG_IPV6/CONFIG_INET) is preserved through the corresponding
// Rust feature flags; these build-time conditions cannot otherwise be resolved here.
#[cfg(feature = "CONFIG_IPV6")]
macro_rules! INDIRECT_CALL_INET { ($f:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => { INDIRECT_CALL_2!($f, $f2, $f1, $($args),*) }; }
#[cfg(all(not(feature = "CONFIG_IPV6"), feature = "CONFIG_INET"))]
macro_rules! INDIRECT_CALL_INET { ($f:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => { INDIRECT_CALL_1!($f, $f1, $($args),*) }; }
#[cfg(not(any(feature = "CONFIG_IPV6", feature = "CONFIG_INET")))]
macro_rules! INDIRECT_CALL_INET { ($f:expr, $f2:expr, $f1:expr, $($args:expr),* $(,)?) => { $f($($args),*) }; }

#[cfg(feature = "CONFIG_INET")]
macro_rules! INDIRECT_CALL_INET_1 { ($f:expr, $f1:expr, $($args:expr),* $(,)?) => { INDIRECT_CALL_1!($f, $f1, $($args),*) }; }
#[cfg(not(feature = "CONFIG_INET"))]
macro_rules! INDIRECT_CALL_INET_1 { ($f:expr, $f1:expr, $($args:expr),* $(,)?) => { $f($($args),*) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
