#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn lglbnr(
    mut name: *const libc::c_char,
    mut prefix: *const libc::c_char,
    mut file: *mut FILE,
) {
    let mut p: *const libc::c_char = b"-W -Wall -O3 -DNLGLOG -DNDEBUG -DNCHKSOL -DNLGLDRUPLIG -DNLGLYALSAT -DNLGLFILES -DNLGLDEMA\0"
        as *const u8 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = (78 as libc::c_int as libc::c_ulong)
        .wrapping_sub(strlen(prefix)) as libc::c_int;
    fprintf(file, b"%s%s\n\0" as *const u8 as *const libc::c_char, prefix, name);
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    fprintf(
        file,
        b"%sVersion %s %s\n\0" as *const u8 as *const libc::c_char,
        prefix,
        b"1.0.0\0" as *const u8 as *const libc::c_char,
        b"89a167d0d2efe98d983c87b5b84175b40ea55842\0" as *const u8 as *const libc::c_char,
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    fprintf(
        file,
        b"%sCopyright (C) 2010-2016 Armin Biere JKU Linz Austria.\n\0" as *const u8
            as *const libc::c_char,
        prefix,
    );
    fprintf(
        file,
        b"%sAll rights reserved.\n\0" as *const u8 as *const libc::c_char,
        prefix,
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    fprintf(
        file,
        b"%sreleased %s\n\0" as *const u8 as *const libc::c_char,
        prefix,
        b"Mon Aug 19 16:52:16 PDT 2024\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        file,
        b"%scompiled %s\n\0" as *const u8 as *const libc::c_char,
        prefix,
        b"Mon Aug 19 16:52:16 PDT 2024\0" as *const u8 as *const libc::c_char,
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    fprintf(
        file,
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        prefix,
        b"gcc (Ubuntu 13.2.0-23ubuntu4) 13.2.0\0" as *const u8 as *const libc::c_char,
    );
    loop {
        fputs(prefix, file);
        q = p;
        while *q as libc::c_int != 0 && *q as libc::c_int != ' ' as i32 {
            q = q.offset(1);
            q;
        }
        if *q as libc::c_int != 0
            && (q.offset_from(p) as libc::c_long) < len as libc::c_long
        {
            loop {
                n = q.offset(1 as libc::c_int as isize);
                while *n as libc::c_int != 0 && *n as libc::c_int != ' ' as i32 {
                    n = n.offset(1);
                    n;
                }
                if n.offset_from(p) as libc::c_long >= len as libc::c_long {
                    break;
                }
                q = n;
                if *n == 0 {
                    break;
                }
            }
        }
        while p < q {
            let fresh0 = p;
            p = p.offset(1);
            fputc(*fresh0 as libc::c_int, file);
        }
        fputc('\n' as i32, file);
        if *p == 0 {
            break;
        }
        p = p.offset(1);
        p;
    }
    fprintf(
        file,
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        prefix,
        b"Linux LAPTOP-SOU2J6CG 6.1.21.2-microsoft-standard-WSL2+ x86_64\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    fflush(file);
}
#[no_mangle]
pub unsafe extern "C" fn lglversion() -> *const libc::c_char {
    return b"1.0.0 89a167d0d2efe98d983c87b5b84175b40ea55842\0" as *const u8
        as *const libc::c_char;
}
