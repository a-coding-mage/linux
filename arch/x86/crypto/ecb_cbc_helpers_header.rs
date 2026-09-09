/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <crypto/internal/skcipher.h> and <asm/fpu/api.h>.

/*
 * Mode helpers to instantiate parameterized skcipher ECB/CBC modes without
 * having to rely on indirect calls and retpolines.
 */

macro_rules! ECB_WALK_START {
    ($req:expr, $bsize:expr, $fpu_blocks:expr) => {{
        let ctx = unsafe { crypto_skcipher_ctx(crypto_skcipher_reqtfm($req)) };
        let __fpu_blocks = $fpu_blocks;
        let __bsize = $bsize;
        let mut walk = skcipher_walk { };
        let mut err = unsafe { skcipher_walk_virt(&mut walk, $req, false) };
        while walk.nbytes > 0 {
            let mut nbytes = walk.nbytes;
            let mut do_fpu = __fpu_blocks != -1 && nbytes >= __fpu_blocks * __bsize;
            let mut src = walk.src.virt.addr;
            let mut dst = walk.dst.virt.addr;
            let mut buf = [0u8; $bsize];
            if do_fpu { unsafe { kernel_fpu_begin(); } }
            {


                $crate::ECB_WALK_START!(@body ctx, walk, __fpu_blocks, __bsize, err, nbytes, do_fpu, src, dst, buf);
            }
        }
        err
    }};
    (@body $($args:tt)*) => {};
}

macro_rules! CBC_WALK_START {
    ($req:expr, $bsize:expr, $fpu_blocks:expr) => {
        ECB_WALK_START!($req, $bsize, $fpu_blocks)
    };
}

macro_rules! ECB_WALK_ADVANCE {
    ($blocks:expr) => {{
        dst = unsafe { dst.add(($blocks) * __bsize) };
        src = unsafe { src.add(($blocks) * __bsize) };
        nbytes -= ($blocks) * __bsize;
    }};
}

macro_rules! ECB_BLOCK {
    ($blocks:expr, $func:expr) => {{
        let __blocks = $blocks;
        if do_fpu && __blocks < __fpu_blocks {
            unsafe { kernel_fpu_end(); }
            do_fpu = false;
        }
        while nbytes >= __blocks * __bsize {
            unsafe { $func(ctx, dst, src); }
            ECB_WALK_ADVANCE!($blocks);
        }
    }};
}

macro_rules! CBC_ENC_BLOCK {
    ($func:expr) => {{
        let mut __iv = walk.iv;
        while nbytes >= __bsize {
            unsafe { crypto_xor_cpy(dst, src, __iv, __bsize); }
            unsafe { $func(ctx, dst, dst); }
            __iv = dst;
            ECB_WALK_ADVANCE!(1);
        }
        unsafe { memcpy(walk.iv, __iv, __bsize); }
    }};
}

macro_rules! CBC_DEC_BLOCK {
    ($blocks:expr, $func:expr) => {{
        let __blocks = $blocks;
        if do_fpu && __blocks < __fpu_blocks {
            unsafe { kernel_fpu_end(); }
            do_fpu = false;
        }
        while nbytes >= __blocks * __bsize {
            let mut __iv = unsafe { src.add(($blocks - 1) * __bsize) };
            if dst == src {
                __iv = unsafe { memcpy(buf.as_mut_ptr(), __iv, __bsize) };
            }
            unsafe { $func(ctx, dst, src); }
            unsafe { crypto_xor(dst, walk.iv, __bsize); }
            unsafe { memcpy(walk.iv, __iv, __bsize); }
            ECB_WALK_ADVANCE!($blocks);
        }
    }};
}

macro_rules! ECB_WALK_END {
    () => {{
        if do_fpu { unsafe { kernel_fpu_end(); } }
        err = unsafe { skcipher_walk_done(&mut walk, nbytes) };
        }
        err
    }};
}

macro_rules! CBC_WALK_END {
    () => { ECB_WALK_END!() };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
