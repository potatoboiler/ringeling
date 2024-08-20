#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglsetphase(_: *mut LGL, lit: libc::c_int);
    fn lglresetphase(_: *mut LGL, lit: libc::c_int);
    fn lglsetimportant(_: *mut LGL, lit: libc::c_int);
    fn lglsetphases(_: *mut LGL);
    fn lglmaxvar(_: *mut LGL) -> libc::c_int;
    fn lglincvar(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit: libc::c_int);
    fn lglassume(_: *mut LGL, lit: libc::c_int);
    fn lglfixate(_: *mut LGL);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglfixed(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglfailed(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglchanged(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglflushcache(_: *mut LGL);
    fn lglrepr(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglfreeze(_: *mut LGL, lit: libc::c_int);
    fn lglfrozen(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglmelt(_: *mut LGL, lit: libc::c_int);
    fn lglusable(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglreusable(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglreuse(_: *mut LGL, lit: libc::c_int);
    fn lglookahead(_: *mut LGL) -> libc::c_int;
    fn lglstats(_: *mut LGL);
    fn lglchkclone(_: *mut LGL);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
static mut verbose: libc::c_int = 0;
static mut exitonabort: libc::c_int = 0;
static mut lineno: libc::c_int = 0;
static mut name: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** lgluntrace: \0" as *const u8 as *const libc::c_char,
        stderr,
    );
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn perr(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(
        stderr,
        b"*** lgluntrace: parse error in '%s' line %d: \0" as *const u8 as *const libc::c_char,
        name,
        lineno,
    );
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn msg(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
        return;
    }
    fputs(
        b"c [lgluntrace] \0" as *const u8 as *const libc::c_char,
        stdout,
    );
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
}
unsafe extern "C" fn isnumstr(mut str: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    p = str;
    if *p as libc::c_int == '-' as i32 {
        p = p.offset(1);
        p;
    }
    let fresh0 = p;
    p = p.offset(1);
    if *(*__ctype_b_loc()).offset(*fresh0 as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        ch = *p as libc::c_int;
        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
        p = p.offset(1);
        p;
    }
    return (ch == 0) as libc::c_int;
}
unsafe extern "C" fn intarg(mut op: *mut libc::c_char) -> libc::c_int {
    let mut tok: *const libc::c_char = 0 as *const libc::c_char;
    tok = strtok(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    if tok.is_null()
        || isnumstr(tok) == 0
        || !(strtok(
            0 as *mut libc::c_char,
            b" \0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
    {
        perr(
            b"expected integer argument for '%s'\0" as *const u8 as *const libc::c_char,
            op,
        );
        exit(1 as libc::c_int);
    }
    return atoi(tok);
}
unsafe extern "C" fn noarg(mut str: *const libc::c_char, mut op: *mut libc::c_char) -> libc::c_int {
    if strcmp(str, op) != 0 {
        return 0 as libc::c_int;
    }
    if !(strtok(
        0 as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    ))
    .is_null()
    {
        perr(
            b"argument after '%s'\0" as *const u8 as *const libc::c_char,
            op,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn exitonsig(mut sig: libc::c_int) {
    msg(
        b"exit(%d) on signal %d\0" as *const u8 as *const libc::c_char,
        sig,
        sig,
    );
    exit(sig);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut close: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    let mut buffer: [libc::c_char; 80] = [0; 80];
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lgl: *mut LGL = 0 as *mut LGL;
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            printf(
                b"usage: lgluntrace [-h][-v][-e][<trace>[.gz]]\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            verbose = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-e\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            exitonabort = 1 as libc::c_int;
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            die(
                b"invalid command line option '%s' (try '-h')\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else if !name.is_null() {
            die(
                b"two traces '%s' and '%s' specified (try '-h')\0" as *const u8
                    as *const libc::c_char,
                name,
                *argv.offset(i as isize),
            );
        } else {
            name = *argv.offset(i as isize);
        }
        i += 1;
        i;
    }
    if !name.is_null() {
        len = strlen(name) as libc::c_int;
        if len >= 3 as libc::c_int
            && strcmp(
                name.offset(len as isize)
                    .offset(-(3 as libc::c_int as isize)),
                b".gz\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            cmd = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
            sprintf(
                cmd,
                b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
                name,
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            free(cmd as *mut libc::c_void);
            if !file.is_null() {
                close = 2 as libc::c_int;
            }
        } else {
            file = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
            if !file.is_null() {
                close = 1 as libc::c_int;
            }
        }
        if file.is_null() {
            die(
                b"can not read '%s'\0" as *const u8 as *const libc::c_char,
                name,
            );
        }
    } else {
        name = b"<stdin>\0" as *const u8 as *const libc::c_char;
        file = stdin;
    }
    if exitonabort != 0 {
        msg(b"setting signal handlers since '-e' specified\0" as *const u8 as *const libc::c_char);
        signal(
            2 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            11 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            6 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            15 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    msg(b"reading %s\0" as *const u8 as *const libc::c_char, name);
    len = 0 as libc::c_int;
    buffer[len as usize] = 0 as libc::c_int as libc::c_char;
    lineno = 1 as libc::c_int;
    res = 0 as libc::c_int;
    lgl = 0 as *mut LGL;
    loop {
        ch = getc(file);
        if ch == -(1 as libc::c_int) {
            break;
        }
        if ch == '\r' as i32 {
            continue;
        }
        if ch != '\n' as i32 {
            if len + 1 as libc::c_int
                >= ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int
            {
                perr(b"line buffer exceeded\0" as *const u8 as *const libc::c_char);
            }
            let fresh1 = len;
            len = len + 1;
            buffer[fresh1 as usize] = ch as libc::c_char;
            buffer[len as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            msg(
                b"line %d : %s\0" as *const u8 as *const libc::c_char,
                lineno,
                buffer.as_mut_ptr(),
            );
            tok = strtok(
                buffer.as_mut_ptr(),
                b" \0" as *const u8 as *const libc::c_char,
            );
            if tok.is_null() {
                perr(b"empty line\0" as *const u8 as *const libc::c_char);
            } else if strcmp(tok, b"add\0" as *const u8 as *const libc::c_char) == 0 {
                lgladd(
                    lgl,
                    intarg(b"add\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"return\0" as *const u8 as *const libc::c_char) == 0 {
                arg = intarg(b"return\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                if arg != res {
                    die(
                        b"expected return value %d but got %d\0" as *const u8
                            as *const libc::c_char,
                        arg,
                        res,
                    );
                }
            } else if strcmp(tok, b"deref\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglderef(
                    lgl,
                    intarg(b"deref\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"failed\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglfailed(
                    lgl,
                    intarg(b"failed\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"fixed\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglfixed(
                    lgl,
                    intarg(b"fixed\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"repr\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglrepr(
                    lgl,
                    intarg(b"repr\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if noarg(
                tok,
                b"incvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                res = lglincvar(lgl);
            } else if noarg(
                tok,
                b"maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                res = lglmaxvar(lgl);
            } else if noarg(
                tok,
                b"changed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                res = lglchanged(lgl);
            } else if noarg(
                tok,
                b"inconsistent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                res = lglinconsistent(lgl);
            } else if noarg(
                tok,
                b"lkhd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                res = lglookahead(lgl);
            } else if noarg(
                tok,
                b"fixate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglfixate(lgl);
            } else if noarg(
                tok,
                b"reduce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglreducecache(lgl);
            } else if noarg(
                tok,
                b"flush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglflushcache(lgl);
            } else if noarg(
                tok,
                b"chkclone\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglchkclone(lgl);
            } else if strcmp(tok, b"assume\0" as *const u8 as *const libc::c_char) == 0 {
                lglassume(
                    lgl,
                    intarg(b"assume\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if noarg(
                tok,
                b"init\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lgl = lglinit();
            } else if noarg(
                tok,
                b"sat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                res = lglsat(lgl);
            } else if strcmp(tok, b"simp\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglsimp(
                    lgl,
                    intarg(b"simp\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if noarg(
                tok,
                b"stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglstats(lgl);
            } else if strcmp(tok, b"freeze\0" as *const u8 as *const libc::c_char) == 0 {
                lglfreeze(
                    lgl,
                    intarg(b"freeze\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"melt\0" as *const u8 as *const libc::c_char) == 0 {
                lglmelt(
                    lgl,
                    intarg(b"melt\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"reuse\0" as *const u8 as *const libc::c_char) == 0 {
                lglreuse(
                    lgl,
                    intarg(b"reuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"frozen\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglfrozen(
                    lgl,
                    intarg(b"frozen\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"usable\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglusable(
                    lgl,
                    intarg(b"usable\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"reusable\0" as *const u8 as *const libc::c_char) == 0 {
                res = lglreusable(
                    lgl,
                    intarg(b"reusable\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"setimportant\0" as *const u8 as *const libc::c_char) == 0 {
                lglsetimportant(
                    lgl,
                    intarg(
                        b"setimportant\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ),
                );
            } else if noarg(
                tok,
                b"setphases\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglsetphases(lgl);
            } else if strcmp(tok, b"setphase\0" as *const u8 as *const libc::c_char) == 0 {
                lglsetphase(
                    lgl,
                    intarg(b"setphase\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if strcmp(tok, b"resetphase\0" as *const u8 as *const libc::c_char) == 0 {
                lglresetphase(
                    lgl,
                    intarg(
                        b"resetphase\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ),
                );
            } else if strcmp(tok, b"option\0" as *const u8 as *const libc::c_char) == 0 {
                opt = strtok(
                    0 as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                if opt.is_null() {
                    perr(b"option name missing\0" as *const u8 as *const libc::c_char);
                }
                lglsetopt(
                    lgl,
                    opt,
                    intarg(b"option\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                );
            } else if noarg(
                tok,
                b"release\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                lglrelease(lgl);
            } else {
                perr(
                    b"invalid command '%s'\0" as *const u8 as *const libc::c_char,
                    tok,
                );
            }
            lineno += 1;
            lineno;
            len = 0 as libc::c_int;
        }
    }
    if close == 1 as libc::c_int {
        fclose(file);
    }
    if close == 2 as libc::c_int {
        pclose(file);
    }
    msg(b"done %s\0" as *const u8 as *const libc::c_char, name);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
