/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <asm/io.h>.
extern "C" {
    fn mb();
}

macro_rules! map_page_into_agp {
    ($page:expr) => {{
        // The C macro is an empty statement and does not evaluate `page`.
    }};
}

macro_rules! unmap_page_from_agp {
    ($page:expr) => {{
        // The C macro is an empty statement and does not evaluate `page`.
    }};
}

macro_rules! flush_agp_cache {
    () => {{
        unsafe { mb() };
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
