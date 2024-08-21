#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDR {
    pub mem: C2RustUnnamed_2,
    pub opt: C2RustUnnamed_1,
    pub header: C2RustUnnamed_0,
    pub add: C2RustUnnamed,
    pub path: *mut libc::c_char,
    pub errmsg: *mut libc::c_char,
    pub closefile: libc::c_int,
    pub file: *mut FILE,
    pub lineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub state: *mut libc::c_void,
    pub fun: ldradd,
}
pub type ldradd = Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub state: *mut libc::c_void,
    pub fun: ldrheader,
}
pub type ldrheader =
    Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub state: *mut libc::c_void,
    pub fun: ldropt,
}
pub type ldropt =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub state: *mut libc::c_void,
    pub alloc: ldralloc,
    pub dealloc: ldrdealloc,
}
pub type ldrdealloc =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> ()>;
pub type ldralloc = Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>;
pub type ldrealloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub parsed: libc::c_int,
    pub specified: libc::c_int,
}
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
unsafe extern "C" fn ldrstdalloc(
    mut mem: *mut libc::c_void,
    mut bytes: size_t,
) -> *mut libc::c_void {
    malloc(bytes)
}
unsafe extern "C" fn ldrstdealloc(
    mut mem: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut bytes: size_t,
) {
    free(ptr);
}
unsafe extern "C" fn ldrstdrealloc(
    mut mem: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut ob: size_t,
    mut nb: size_t,
) -> *mut libc::c_void {
    realloc(ptr, nb)
}
#[no_mangle]
pub unsafe extern "C" fn ldrinit() -> *mut LDR {
    ldrminit(
        std::ptr::null_mut::<libc::c_void>(),
        Some(ldrstdalloc as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void),
        Some(
            ldrstdrealloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        ),
        Some(
            ldrstdealloc
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> (),
        ),
    )
}
#[no_mangle]
pub unsafe extern "C" fn ldrminit(
    mut state: *mut libc::c_void,
    mut alloc: ldralloc,
    mut realloc_0: ldrealloc,
    mut dealloc: ldrdealloc,
) -> *mut LDR {
    let mut res: *mut LDR = alloc.expect("non-null function pointer")(
        state,
        ::core::mem::size_of::<LDR>() as libc::c_ulong,
    ) as *mut LDR;
    if res.is_null() {
        return res;
    }
    memset(
        res as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<LDR>() as libc::c_ulong,
    );
    (*res).mem.state = state;
    (*res).mem.alloc = alloc;
    (*res).mem.dealloc = dealloc;
    res
}
unsafe extern "C" fn ldrdelstr(mut ldr: *mut LDR, mut str: *mut libc::c_char) {
    if !str.is_null() {
        ((*ldr).mem.dealloc).expect("non-null function pointer")(
            (*ldr).mem.state,
            str as *mut libc::c_void,
            (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn ldrstrdup(
    mut ldr: *mut LDR,
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    let mut bytes: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut res: *mut libc::c_char =
        ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, bytes)
            as *mut libc::c_char;
    strcpy(res, str)
}
#[no_mangle]
pub unsafe extern "C" fn ldrelease(mut ldr: *mut LDR) {
    if !((*ldr).file).is_null() {
        if (*ldr).closefile == 1 as libc::c_int {
            fclose((*ldr).file);
        }
        if (*ldr).closefile == 2 as libc::c_int {
            pclose((*ldr).file);
        }
    }
    ldrdelstr(ldr, (*ldr).errmsg);
    ldrdelstr(ldr, (*ldr).path);
    ((*ldr).mem.dealloc).expect("non-null function pointer")(
        (*ldr).mem.state,
        ldr as *mut libc::c_void,
        ::core::mem::size_of::<LDR>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetopt(
    mut ldr: *mut LDR,
    mut state: *mut libc::c_void,
    mut fun: ldropt,
) {
    (*ldr).opt.fun = fun;
    (*ldr).opt.state = state;
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetheader(
    mut ldr: *mut LDR,
    mut state: *mut libc::c_void,
    mut fun: ldrheader,
) {
    (*ldr).header.fun = fun;
    (*ldr).header.state = state;
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetadd(
    mut ldr: *mut LDR,
    mut state: *mut libc::c_void,
    mut fun: ldradd,
) {
    (*ldr).add.fun = fun;
    (*ldr).add.state = state;
}
unsafe extern "C" fn ldrfilexists(mut path: *const libc::c_char) -> libc::c_int {
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    (stat(path, &mut buf) == 0) as libc::c_int
}
unsafe extern "C" fn ldrperr(mut ldr: *mut LDR, mut msg: *const libc::c_char) -> libc::c_int {
    let mut bytes: size_t = 0;
    let mut len: size_t = 0;
    let mut str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    bytes = (strlen(msg))
        .wrapping_add(strlen((*ldr).path))
        .wrapping_add(20 as libc::c_int as libc::c_ulong);
    str = ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, bytes)
        as *mut libc::c_char;
    sprintf(
        str,
        b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
        (*ldr).path,
        (*ldr).lineno,
        msg,
    );
    len = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*ldr).errmsg = strcpy(
        ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, len)
            as *mut libc::c_char,
        str,
    );
    ((*ldr).mem.dealloc).expect("non-null function pointer")(
        (*ldr).mem.state,
        str as *mut libc::c_void,
        bytes,
    );
    0 as libc::c_int
}
unsafe extern "C" fn ldrhas(
    mut str: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = strlen(str) as libc::c_int;
    let mut k: libc::c_int = strlen(suffix) as libc::c_int;
    if l < k {
        return 0 as libc::c_int;
    }
    (strcmp(str.offset(l as isize).offset(-(k as isize)), suffix) == 0) as libc::c_int
}
unsafe extern "C" fn ldrcmd(
    mut ldr: *mut LDR,
    mut fmt: *const libc::c_char,
    mut name: *const libc::c_char,
) -> *mut FILE {
    let mut res: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut len: libc::c_int = (strlen(fmt))
        .wrapping_add(strlen(name))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut s: *mut libc::c_char =
        ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, len as size_t)
            as *mut libc::c_char;
    sprintf(s, fmt, name);
    res = popen(s, b"r\0" as *const u8 as *const libc::c_char);
    ((*ldr).mem.dealloc).expect("non-null function pointer")(
        (*ldr).mem.state,
        s as *mut libc::c_void,
        len as size_t,
    );
    res
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetpath(mut ldr: *mut LDR, mut path: *const libc::c_char) -> i32 {
    (*ldr).path = ldrstrdup(ldr, path);
    if ldrfilexists(path) == 0 {
        return ldrperr(
            ldr,
            b"file does not exist\0" as *const u8 as *const libc::c_char,
        );
    }
    (*ldr).closefile = 2 as libc::c_int;
    if ldrhas(path, b".gz\0" as *const u8 as *const libc::c_char) != 0 {
        (*ldr).file = ldrcmd(
            ldr,
            b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
            path,
        );
    } else if ldrhas(path, b".bz2\0" as *const u8 as *const libc::c_char) != 0 {
        (*ldr).file = ldrcmd(ldr, b"bzcat %s\0" as *const u8 as *const libc::c_char, path);
    } else if ldrhas(path, b".7z\0" as *const u8 as *const libc::c_char) != 0 {
        (*ldr).file = ldrcmd(
            ldr,
            b"7z x -so %s 2>/dev/null\0" as *const u8 as *const libc::c_char,
            path,
        );
    } else if ldrhas(path, b".lzma\0" as *const u8 as *const libc::c_char) != 0 {
        (*ldr).file = ldrcmd(ldr, b"lzcat %s\0" as *const u8 as *const libc::c_char, path);
    } else {
        (*ldr).file = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
        (*ldr).closefile = 1 as libc::c_int;
    }
    if ((*ldr).file).is_null() {
        ldrperr(
            ldr,
            b"can not open file\0" as *const u8 as *const libc::c_char,
        )
    } else {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetfile(mut ldr: *mut LDR, mut file: *mut FILE) {
    (*ldr).file = file;
    (*ldr).path = ldrstrdup(
        ldr,
        b"<unspecified-path>\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetnamedfile(
    mut ldr: *mut LDR,
    mut file: *mut FILE,
    mut path: *const libc::c_char,
) {
    (*ldr).file = file;
    (*ldr).path = ldrstrdup(ldr, path);
}
#[no_mangle]
pub unsafe extern "C" fn ldrerr(mut ldr: *mut LDR) -> *const libc::c_char {
    (*ldr).errmsg
}
unsafe extern "C" fn ldrnext(mut ldr: *mut LDR) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    ch = getc((*ldr).file);
    if ch == '\n' as i32 {
        (*ldr).lineno += 1;
        (*ldr).lineno;
    }
    ch
}
#[no_mangle]
pub unsafe extern "C" fn ldrparse(mut ldr: *mut LDR) -> libc::c_int {
    let mut current_block: u64;
    let mut vars: C2RustUnnamed_3 = C2RustUnnamed_3 {
        parsed: 0,
        specified: 0,
    };
    let mut clauses: C2RustUnnamed_3 = C2RustUnnamed_3 {
        parsed: 0,
        specified: 0,
    };
    let mut ch: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut digit: libc::c_int = 0;
    if !((*ldr).errmsg).is_null() {
        return 0 as libc::c_int;
    }
    loop {
        ch = ldrnext(ldr);
        if ch != 'c' as i32 {
            break;
        }
        loop {
            ch = ldrnext(ldr);
            if ch == '\n' as i32 {
                break;
            }
            if ch == -(1 as libc::c_int) {
                return ldrperr(
                    ldr,
                    b"end-of-file in comment before header\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if ch != 'p' as i32 {
        return ldrperr(
            ldr,
            b"expected 'p' or 'c'\0" as *const u8 as *const libc::c_char,
        );
    }
    if ldrnext(ldr) != ' ' as i32 {
        return ldrperr(
            ldr,
            b"expected space after 'p'\0" as *const u8 as *const libc::c_char,
        );
    }
    if ldrnext(ldr) != 'c' as i32 {
        return ldrperr(
            ldr,
            b"expected 'c' after 'p '\0" as *const u8 as *const libc::c_char,
        );
    }
    if ldrnext(ldr) != 'n' as i32 {
        return ldrperr(
            ldr,
            b"expected 'n' after 'p c'\0" as *const u8 as *const libc::c_char,
        );
    }
    if ldrnext(ldr) != 'f' as i32 {
        return ldrperr(
            ldr,
            b"expected 'f' after 'p cn'\0" as *const u8 as *const libc::c_char,
        );
    }
    if ldrnext(ldr) != ' ' as i32 {
        return ldrperr(
            ldr,
            b"expected space after 'p cnf'\0" as *const u8 as *const libc::c_char,
        );
    }
    ch = ldrnext(ldr);
    if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return ldrperr(
            ldr,
            b"expected digit after 'p cnf '\0" as *const u8 as *const libc::c_char,
        );
    }
    vars.specified = ch - '0' as i32;
    loop {
        ch = ldrnext(ldr);
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            if (2147483647 as libc::c_int / 10 as libc::c_int) < vars.specified {
                current_block = 14777158775944759451;
                break;
            }
            vars.specified *= 10 as libc::c_int;
            digit = ch - '0' as i32;
            if 2147483647 as libc::c_int - digit < vars.specified {
                current_block = 14777158775944759451;
                break;
            }
            vars.specified += digit;
        } else {
            if ch != ' ' as i32 {
                return ldrperr(
                    ldr,
                    b"expected space after maximum variable index\0" as *const u8
                        as *const libc::c_char,
                );
            }
            ch = ldrnext(ldr);
            if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                return ldrperr(
                    ldr,
                    b"expected digit after space after variable index\0" as *const u8
                        as *const libc::c_char,
                );
            }
            clauses.specified = ch - '0' as i32;
            current_block = 15125582407903384992;
            break;
        }
    }
    '_NUMBER_TOO_LARGE: loop {
        match current_block {
            14777158775944759451 => {
                return ldrperr(
                    ldr,
                    b"number too large\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                ch = ldrnext(ldr);
                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    if (2147483647 as libc::c_int / 10 as libc::c_int) < clauses.specified {
                        current_block = 14777158775944759451;
                        continue;
                    }
                    clauses.specified *= 10 as libc::c_int;
                    digit = ch - '0' as i32;
                    if 2147483647 as libc::c_int - digit < clauses.specified {
                        current_block = 14777158775944759451;
                        continue;
                    }
                    clauses.specified += digit;
                    current_block = 15125582407903384992;
                } else {
                    while ch == ' ' as i32 || ch == '\t' as i32 || ch == '\r' as i32 {
                        ch = ldrnext(ldr);
                    }
                    if ch != '\n' as i32 {
                        return ldrperr(
                            ldr,
                            b"expected new line after header\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if ((*ldr).header.fun).is_some() {
                        ((*ldr).header.fun).expect("non-null function pointer")(
                            (*ldr).header.state,
                            vars.specified,
                            clauses.specified,
                        );
                    }
                    clauses.parsed = 0 as libc::c_int;
                    vars.parsed = clauses.parsed;
                    lit = 0 as libc::c_int;
                    's_198: loop {
                        ch = ldrnext(ldr);
                        if ch == ' ' as i32
                            || ch == '\t' as i32
                            || ch == '\r' as i32
                            || ch == '\n' as i32
                        {
                            continue;
                        }
                        if ch == 'c' as i32 {
                            loop {
                                ch = ldrnext(ldr);
                                if ch == '\n' as i32 {
                                    break;
                                }
                                if ch == -(1 as libc::c_int) {
                                    return ldrperr(
                                        ldr,
                                        b"end-of-file in comment after header\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                            }
                        } else {
                            if ch == -(1 as libc::c_int) {
                                break '_NUMBER_TOO_LARGE;
                            }
                            if ch == '-' as i32 {
                                ch = ldrnext(ldr);
                                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                {
                                    return ldrperr(
                                        ldr,
                                        b"expected digit after '-'\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                sign = -(1 as libc::c_int);
                            } else if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                            {
                                return ldrperr(
                                    ldr,
                                    b"expected digit or '-'\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                sign = 1 as libc::c_int;
                            }
                            if clauses.specified == clauses.parsed {
                                return ldrperr(
                                    ldr,
                                    b"too many clauses\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            lit = ch - '0' as i32;
                            loop {
                                ch = ldrnext(ldr);
                                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                                {
                                    break;
                                }
                                if (2147483647 as libc::c_int / 10 as libc::c_int) < lit {
                                    current_block = 14777158775944759451;
                                    break 's_198;
                                }
                                lit *= 10 as libc::c_int;
                                digit = ch - '0' as i32;
                                if 2147483647 as libc::c_int - digit < lit {
                                    current_block = 14777158775944759451;
                                    break 's_198;
                                }
                                lit += digit;
                            }
                            if lit > vars.specified {
                                return ldrperr(
                                    ldr,
                                    b"maximum variable index exceeded\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            lit *= sign;
                            if ((*ldr).add.fun).is_some() {
                                ((*ldr).add.fun).expect("non-null function pointer")(
                                    (*ldr).add.state,
                                    lit,
                                );
                            }
                            if lit != 0 {
                                continue;
                            }
                            clauses.parsed += 1;
                            clauses.parsed;
                        }
                    }
                }
            }
        }
    }
    if lit != 0 {
        return ldrperr(
            ldr,
            b"zero sentinel missing at end-of-file\0" as *const u8 as *const libc::c_char,
        );
    }
    if clauses.parsed + 1 as libc::c_int == clauses.specified {
        return ldrperr(
            ldr,
            b"one clause is missing\0" as *const u8 as *const libc::c_char,
        );
    }
    if clauses.parsed < clauses.specified {
        return ldrperr(
            ldr,
            b"clauses are missing\0" as *const u8 as *const libc::c_char,
        );
    }
    1 as libc::c_int
}
