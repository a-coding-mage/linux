// SPDX-License-Identifier: GPL-2.0+

const EINVAL: i32 = 22;

pub fn test_fpu() -> i32 {
    /*
     * This sequence of operations tests that rounding mode is
     * to nearest and that denormal numbers are supported.
     * Volatile variables are used to avoid compiler optimizing
     * the calculations away.
     */
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;
    let mut d: f64 = 0.0;
    let mut e: f64 = 0.0;
    let mut f: f64 = 0.0;
    let mut g: f64 = 0.0;

    unsafe {
        std::ptr::write_volatile(&mut a, 4.0);
        std::ptr::write_volatile(&mut b, 1e-15);
        std::ptr::write_volatile(&mut c, 1e-310);

        /* Sets precision flag */
        std::ptr::write_volatile(
            &mut d,
            std::ptr::read_volatile(&a) + std::ptr::read_volatile(&b),
        );

        /* Result depends on rounding mode */
        std::ptr::write_volatile(
            &mut e,
            std::ptr::read_volatile(&a) + std::ptr::read_volatile(&b) / 2.0,
        );

        /* Denormal and very large values */
        std::ptr::write_volatile(
            &mut f,
            std::ptr::read_volatile(&b) / std::ptr::read_volatile(&c),
        );

        /* Depends on denormal support */
        std::ptr::write_volatile(
            &mut g,
            std::ptr::read_volatile(&a)
                + std::ptr::read_volatile(&c) * std::ptr::read_volatile(&f),
        );

        if std::ptr::read_volatile(&d) > std::ptr::read_volatile(&a)
            && std::ptr::read_volatile(&e) > std::ptr::read_volatile(&a)
            && std::ptr::read_volatile(&g) > std::ptr::read_volatile(&a)
        {
            0
        } else {
            -EINVAL
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
