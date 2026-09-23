# SPDX-License-Identifier: GPL-2.0-only
obj-y += div64.o
ifeq ($(CONFIG_RUST_GCD_LCM),y)
obj-y += gcd_lcm_rust.o
else
obj-y += gcd.o lcm.o
endif
obj-y += int_log.o
ifeq ($(CONFIG_RUST_INT_MATH),y)
obj-y += int_math_rust.o
else
obj-y += int_pow.o int_sqrt.o
endif
obj-y += reciprocal_div.o

obj-$(CONFIG_CORDIC)		+= cordic.o
obj-$(CONFIG_POLYNOMIAL)	+= polynomial.o
obj-$(CONFIG_PRIME_NUMBERS)	+= prime_numbers.o
obj-$(CONFIG_RATIONAL)		+= rational.o

obj-$(CONFIG_TEST_DIV64)	+= test_div64.o
obj-$(CONFIG_TEST_MULDIV64)	+= test_mul_u64_u64_div_u64.o

obj-y += tests/
