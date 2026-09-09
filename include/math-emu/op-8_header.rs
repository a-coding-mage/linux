/* Software floating-point emulation.
   Basic eight-word fraction declaration and manipulation.
   Copyright (C) 1997,1998,1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Richard Henderson (rth@cygnus.com),
                  Jakub Jelinek (jj@ultra.linux.cz) and
                  Peter Maydell (pmaydell@chiark.greenend.org.uk).

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License
   as published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.
*/

/* We need just a few things from here for op-4, if we ever need some
   other macros, they can be added. */
macro_rules! _FP_FRAC_DECL_8 {
    ($x:ident) => { let mut $x: [_FP_W_TYPE; 8]; };
}
macro_rules! _FP_FRAC_HIGH_8 { ($x:expr) => { ($x.f[7]) }; }
macro_rules! _FP_FRAC_LOW_8 { ($x:expr) => { ($x.f[0]) }; }
macro_rules! _FP_FRAC_WORD_8 { ($x:expr, $w:expr) => { ($x.f[$w]) }; }

macro_rules! _FP_FRAC_SLL_8 {
    ($x:expr, $n:expr) => {{
        let mut _up: _FP_I_TYPE = ($n) / _FP_W_TYPE_SIZE;
        let mut _down: _FP_I_TYPE = _FP_W_TYPE_SIZE - _up;
        let _skip: _FP_I_TYPE = ($n) / _FP_W_TYPE_SIZE;
        let mut _i: _FP_I_TYPE;
        _up = ($n) % _FP_W_TYPE_SIZE;
        _down = _FP_W_TYPE_SIZE - _up;
        if _up == 0 {
            _i = 7;
            while _i >= _skip { $x.f[_i as usize] = $x.f[(_i - _skip) as usize]; _i -= 1; }
        } else {
            _i = 7;
            while _i > _skip { $x.f[_i as usize] = ($x.f[(_i-_skip) as usize] << _up) | ($x.f[(_i-_skip-1) as usize] >> _down); _i -= 1; }
            $x.f[_i as usize] = $x.f[0] << _up; _i -= 1;
        }
        while _i >= 0 { $x.f[_i as usize] = 0; _i -= 1; }
    }};
}

macro_rules! _FP_FRAC_SRL_8 {
    ($x:expr, $n:expr) => {{
        let _skip: _FP_I_TYPE = ($n) / _FP_W_TYPE_SIZE;
        let _down: _FP_I_TYPE = ($n) % _FP_W_TYPE_SIZE;
        let _up: _FP_I_TYPE = _FP_W_TYPE_SIZE - _down;
        let mut _i: _FP_I_TYPE;
        if _down == 0 {
            _i = 0; while _i <= 7-_skip { $x.f[_i as usize] = $x.f[(_i+_skip) as usize]; _i += 1; }
        } else {
            _i = 0; while _i < 7-_skip { $x.f[_i as usize] = ($x.f[(_i+_skip) as usize] >> _down) | ($x.f[(_i+_skip+1) as usize] << _up); _i += 1; }
            $x.f[_i as usize] = $x.f[7] >> _down; _i += 1;
        }
        while _i < 8 { $x.f[_i as usize] = 0; _i += 1; }
    }};
}

/* Right shift with sticky-lsb. */
macro_rules! _FP_FRAC_SRS_8 {
    ($x:expr, $n:expr, $size:expr) => {{
        let _skip: _FP_I_TYPE = ($n) / _FP_W_TYPE_SIZE;
        let _down: _FP_I_TYPE = ($n) % _FP_W_TYPE_SIZE;
        let _up: _FP_I_TYPE = _FP_W_TYPE_SIZE - _down;
        let mut _i: _FP_I_TYPE = 0;
        let mut _s: _FP_W_TYPE = 0;
        while _i < _skip { _s |= $x.f[_i as usize]; _i += 1; }
        _s |= $x.f[_i as usize] << _up;
        if _down == 0 {
            _i = 0; while _i <= 7-_skip { $x.f[_i as usize] = $x.f[(_i+_skip) as usize]; _i += 1; }
        } else {
            _i = 0; while _i < 7-_skip { $x.f[_i as usize] = ($x.f[(_i+_skip) as usize] >> _down) | ($x.f[(_i+_skip+1) as usize] << _up); _i += 1; }
            $x.f[_i as usize] = $x.f[7] >> _down; _i += 1;
        }
        while _i < 8 { $x.f[_i as usize] = 0; _i += 1; }
        $x.f[0] |= (_s != 0) as _FP_W_TYPE;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
