# SPDX-License-Identifier: GPL-2.0-only
ifeq ($(CONFIG_RUST_DIV64),y)
obj-y += div64_rust.o
# Only this native owner needs weak linkage for the original __div64_32 ABI.
# PowerPC's export-record assembly is still experimental in Rust 1.85.
RUST_ALLOWED_FEATURES_div64_rust.o := linkage$(if $(CONFIG_PPC),$(comma)asm_experimental_arch)
else
obj-y += div64.o
endif
ifeq ($(CONFIG_RUST_GCD_LCM),y)
obj-y += gcd_lcm_rust.o
else
obj-y += gcd.o lcm.o
endif
ifeq ($(CONFIG_RUST_INT_LOG),y)
obj-y += int_log_rust.o
else
obj-y += int_log.o
endif
ifeq ($(CONFIG_RUST_INT_MATH),y)
obj-y += int_math_rust.o
else
obj-y += int_pow.o int_sqrt.o
endif
ifeq ($(CONFIG_RUST_RECIPROCAL_DIV),y)
obj-y += reciprocal_div_rust.o
else
obj-y += reciprocal_div.o
endif

obj-$(CONFIG_CORDIC)		+= cordic.o
ifeq ($(CONFIG_RUST_CORDIC),y)
# Keep the original tristate selection and cordic.ko module identity.
cordic-y := cordic_rust.o
RUST_ALLOWED_FEATURES_cordic_rust.o := $(if $(CONFIG_PPC),asm_experimental_arch)
endif
obj-$(CONFIG_POLYNOMIAL)	+= polynomial.o
ifeq ($(CONFIG_RUST_POLYNOMIAL),y)
# Preserve the original tristate selection and polynomial.ko module identity.
polynomial-y := polynomial_rust.o
RUST_ALLOWED_FEATURES_polynomial_rust.o := $(if $(CONFIG_PPC),asm_experimental_arch)
endif
obj-$(CONFIG_PRIME_NUMBERS)	+= prime_numbers.o
obj-$(CONFIG_RATIONAL)		+= rational.o
ifeq ($(CONFIG_RUST_RATIONAL),y)
# Preserve the original built-in/module selection and rational.ko name.
rational-y := rational_rust.o
endif

obj-$(CONFIG_TEST_DIV64)	+= test_div64.o
obj-$(CONFIG_TEST_MULDIV64)	+= test_mul_u64_u64_div_u64.o

# Explicit rules select the existing translations despite their C counterparts
# having the same basename, including assembly/LLVM inspection targets.
# Preserve the original module names and C choice.
ifeq ($(CONFIG_RUST_DIV64_TESTS),y)
$(obj)/test_div64.o: $(src)/test_div64.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/test_div64.s: $(src)/test_div64.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/test_div64.ll: $(src)/test_div64.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)

$(obj)/test_mul_u64_u64_div_u64.o: $(src)/test_mul_u64_u64_div_u64.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/test_mul_u64_u64_div_u64.s: $(src)/test_mul_u64_u64_div_u64.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/test_mul_u64_u64_div_u64.ll: $(src)/test_mul_u64_u64_div_u64.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

obj-y += tests/
