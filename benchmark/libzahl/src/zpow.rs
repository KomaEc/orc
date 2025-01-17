use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsetu(_: *mut C2RustUnnamed, _: libc::c_ulonglong);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    fn zbits(_: *mut C2RustUnnamed) -> size_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zpow(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0;
    let mut bits: size_t = 0;
    let mut x: zahl_char_t = 0;
    if zsignum(c) <= 0 as libc::c_int {
        if zzero(c) != 0 {
            if zzero(b) != 0 {
                libzahl_error = 33 as libc::c_int;
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            }
            zsetu(a, 1 as libc::c_int as libc::c_ulonglong);
        } else if zzero(b) != 0 {
            libzahl_error = 33 as libc::c_int;
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else {
            (*a).sign = 0 as libc::c_int;
        }
        return;
    } else {
        if zzero(b) != 0 {
            (*a).sign = 0 as libc::c_int;
            return;
        }
    }
    bits = zbits(c);
    n = bits >> 5 as libc::c_int;
    zset(libzahl_tmp_pow_b.as_mut_ptr(), b);
    zset(libzahl_tmp_pow_c.as_mut_ptr(), c);
    zsetu(a, 1 as libc::c_int as libc::c_ulonglong);
    i = 0 as libc::c_int as size_t;
    while i < n {
        x = *((*libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
        j = 32 as libc::c_int as size_t;
        loop {
            let fresh0 = j;
            j = j.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            if x & 1 as libc::c_int as libc::c_uint != 0 {
                zmul(a, a, libzahl_tmp_pow_b.as_mut_ptr());
            }
            zsqr(libzahl_tmp_pow_b.as_mut_ptr(), libzahl_tmp_pow_b.as_mut_ptr());
            x >>= 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    x = *((*libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
    while x != 0 {
        if x & 1 as libc::c_int as libc::c_uint != 0 {
            zmul(a, a, libzahl_tmp_pow_b.as_mut_ptr());
        }
        zsqr(libzahl_tmp_pow_b.as_mut_ptr(), libzahl_tmp_pow_b.as_mut_ptr());
        x >>= 1 as libc::c_int;
    }
}
