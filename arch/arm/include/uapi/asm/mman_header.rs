// Dependency supplied by the generic memory-management header:
// `asm-generic/mman.h`.

// The symbols `MAP_FIXED`, `FIRST_USER_ADDRESS`, and `EINVAL` are supplied by
// the surrounding headers/build configuration.
macro_rules! arch_mmap_check {
	($addr:expr, $len:expr, $flags:expr) => {
		if (($flags) & MAP_FIXED != 0) && (($addr) < FIRST_USER_ADDRESS) {
			-EINVAL
		} else {
			0
		}
	};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
