/* SPDX-License-Identifier: GPL-2.0 */

pub type sigchain_fun = Option<unsafe extern "C" fn(c_int)>;

unsafe extern "C" {
    pub fn sigchain_pop(sig: c_int) -> c_int;

    pub fn sigchain_push_common(f: sigchain_fun);
}
