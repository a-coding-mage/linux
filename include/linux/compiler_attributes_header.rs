/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/compiler_attributes.h.
// C compiler attributes which have no direct stable Rust equivalent are kept
// as declarative marker macros so their names and conditional intent remain
// available to dependent translations.

macro_rules! __alias { ($($symbol:tt)*) => {}; }
macro_rules! __aligned { ($($x:tt)*) => {}; }
macro_rules! __aligned_largest { () => {}; }
macro_rules! __alloc_size__ { ($($x:tt)*) => {}; }
macro_rules! __always_inline { () => { #[inline(always)] }; }
macro_rules! __assume_aligned { ($($a:tt)*) => {}; }
macro_rules! __cleanup { ($($func:tt)*) => {}; }
macro_rules! __attribute_const__ { () => {}; }

// Optional compiler attributes: preserved as no-op Rust markers when the
// target compiler does not provide an equivalent.
macro_rules! __copy { ($($symbol:tt)*) => {}; }
macro_rules! __diagnose_as { ($($builtin:tt)*) => {}; }
macro_rules! __deprecated { () => {}; }
macro_rules! __designated_init { () => {}; }
macro_rules! __compiletime_error { ($($msg:tt)*) => {}; }
macro_rules! __visible { () => {}; }

macro_rules! __printf { ($($args:tt)*) => {}; }
macro_rules! __scanf { ($($args:tt)*) => {}; }
macro_rules! __gnu_inline { () => {}; }
macro_rules! __malloc { () => {}; }
macro_rules! __mode { ($($x:tt)*) => {}; }
macro_rules! __no_caller_saved_registers { () => {}; }
macro_rules! __noclone { () => {}; }

// C's fallthrough pseudo-keyword; Rust match arms do not fall through.
macro_rules! fallthrough { () => {}; }
macro_rules! __flatten { () => {}; }
macro_rules! noinline { () => {}; }
macro_rules! __nonnull_args { ($($x:tt)*) => {}; }
macro_rules! __nonstring { () => {}; }
macro_rules! __no_profile { () => {}; }
macro_rules! __noreturn { () => { ! }; }
macro_rules! __no_stack_protector { () => {}; }
macro_rules! __overloadable { () => {}; }
macro_rules! __packed { () => {}; }
macro_rules! __pass_dynamic_object_size { ($($ty:tt)*) => {}; }
macro_rules! __pass_object_size { ($($ty:tt)*) => {}; }
macro_rules! __pure { () => {}; }
macro_rules! __section { ($($section:tt)*) => {}; }
macro_rules! __uninitialized { () => {}; }
macro_rules! __always_unused { () => {}; }
macro_rules! __maybe_unused { () => {}; }
macro_rules! __used { () => {}; }
macro_rules! __always_used { () => {}; }
macro_rules! __must_check { () => {}; }
macro_rules! __compiletime_warning { ($($msg:tt)*) => {}; }
macro_rules! __disable_sanitizer_instrumentation { () => {}; }
macro_rules! __noipa { () => {}; }
macro_rules! __weak { () => {}; }

// Used by functions that use __builtin_return_address.  The C definition is
// `noinline __noclone`; both components are represented by this marker.
macro_rules! __fix_address { () => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
