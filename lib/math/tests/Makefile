# SPDX-License-Identifier: GPL-2.0-only

obj-$(CONFIG_GCD_KUNIT_TEST)		+= gcd_kunit.o
obj-$(CONFIG_INT_LOG_KUNIT_TEST)	+= int_log_kunit.o
obj-$(CONFIG_INT_POW_KUNIT_TEST)	+= int_pow_kunit.o
obj-$(CONFIG_INT_SQRT_KUNIT_TEST)	+= int_sqrt_kunit.o
obj-$(CONFIG_POLYNOMIAL_KUNIT_TEST)	+= polynomial_kunit.o
obj-$(CONFIG_PRIME_NUMBERS_KUNIT_TEST)	+= prime_numbers_kunit.o
obj-$(CONFIG_RATIONAL_KUNIT_TEST)	+= rational_kunit.o

ifeq ($(CONFIG_RUST_GCD_KUNIT_TEST),y)
# Select the translated suite without changing the provider or its tristate.
$(obj)/gcd_kunit.o: $(src)/gcd_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/gcd_kunit.s: $(src)/gcd_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/gcd_kunit.ll: $(src)/gcd_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

ifeq ($(CONFIG_RUST_INT_LOG_KUNIT_TEST),y)
# Preserve the original tristate, module identity and native zero-input tests.
$(obj)/int_log_kunit.o: $(src)/int_log_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/int_log_kunit.s: $(src)/int_log_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/int_log_kunit.ll: $(src)/int_log_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

ifeq ($(CONFIG_RUST_INT_MATH_KUNIT_TESTS),y)
# Preserve each original tristate and module identity independently.
$(obj)/int_pow_kunit.o: $(src)/int_pow_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/int_pow_kunit.s: $(src)/int_pow_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/int_pow_kunit.ll: $(src)/int_pow_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)

$(obj)/int_sqrt_kunit.o: $(src)/int_sqrt_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/int_sqrt_kunit.s: $(src)/int_sqrt_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/int_sqrt_kunit.ll: $(src)/int_sqrt_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

ifeq ($(CONFIG_RUST_POLYNOMIAL_KUNIT_TEST),y)
# Explicit same-basename rules retain the original C fallback/module identity.
$(obj)/polynomial_kunit.o: $(src)/polynomial_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/polynomial_kunit.s: $(src)/polynomial_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/polynomial_kunit.ll: $(src)/polynomial_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

ifeq ($(CONFIG_RUST_PRIME_NUMBERS_KUNIT_TEST),y)
# Preserve the original suite, callback ABI, tristate and module identity.
$(obj)/prime_numbers_kunit.o: $(src)/prime_numbers_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/prime_numbers_kunit.s: $(src)/prime_numbers_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/prime_numbers_kunit.ll: $(src)/prime_numbers_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

ifeq ($(CONFIG_RUST_RATIONAL_KUNIT_TEST),y)
# Retain the original provider independence and rational_kunit module name.
$(obj)/rational_kunit.o: $(src)/rational_kunit.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/rational_kunit.s: $(src)/rational_kunit.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/rational_kunit.ll: $(src)/rational_kunit.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif
