# SPDX-License-Identifier: GPL-2.0-only

obj-$(CONFIG_GCD_KUNIT_TEST)		+= gcd_kunit.o
obj-$(CONFIG_INT_LOG_KUNIT_TEST)	+= int_log_kunit.o
obj-$(CONFIG_INT_POW_KUNIT_TEST)	+= int_pow_kunit.o
obj-$(CONFIG_INT_SQRT_KUNIT_TEST)	+= int_sqrt_kunit.o
obj-$(CONFIG_POLYNOMIAL_KUNIT_TEST)	+= polynomial_kunit.o
obj-$(CONFIG_PRIME_NUMBERS_KUNIT_TEST)	+= prime_numbers_kunit.o
obj-$(CONFIG_RATIONAL_KUNIT_TEST)	+= rational_kunit.o

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
