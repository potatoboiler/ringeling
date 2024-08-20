#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglfirstopt(_: *mut LGL) -> *mut libc::c_void;
    fn lglnextopt(
        _: *mut LGL,
        iterator: *mut libc::c_void,
        nameptr: *mut *const libc::c_char,
        valptr: *mut libc::c_int,
        minptr: *mut libc::c_int,
        maxptr: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn lglsetphase(_: *mut LGL, lit_0: libc::c_int);
    fn lglresetphase(_: *mut LGL, lit_0: libc::c_int);
    fn lglsetimportant(_: *mut LGL, lit_0: libc::c_int);
    fn lglsetphases(_: *mut LGL);
    fn lglonabort(
        _: *mut LGL,
        state: *mut libc::c_void,
        callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn lglincvar(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit_0: libc::c_int);
    fn lglassume(_: *mut LGL, lit_0: libc::c_int);
    fn lglfixate(_: *mut LGL);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfixed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfailed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglchanged(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglflushcache(_: *mut LGL);
    fn lglrepr(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfreeze(_: *mut LGL, lit_0: libc::c_int);
    fn lglfrozen(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglmelt(_: *mut LGL, lit_0: libc::c_int);
    fn lglusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglreusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglreuse(_: *mut LGL, lit_0: libc::c_int);
    fn lglookahead(_: *mut LGL) -> libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = libc::c_int;
pub type Type = libc::c_uint;
pub const LKHD: Type = 31;
pub const INCONSISTENT: Type = 30;
pub const CHANGED: Type = 29;
pub const CHKCLONE: Type = 28;
pub const FIXATE: Type = 27;
pub const FIXED: Type = 26;
pub const INCVAR: Type = 25;
pub const MAXVAR: Type = 24;
pub const REUSABLE: Type = 23;
pub const USABLE: Type = 22;
pub const FROZEN: Type = 21;
pub const REDUCE: Type = 20;
pub const FLUSH: Type = 19;
pub const SETPHASES: Type = 18;
pub const RESETPHASE: Type = 17;
pub const SETPHASE: Type = 16;
pub const SETIMPORTANT: Type = 15;
pub const REPR: Type = 14;
pub const SIMP: Type = 13;
pub const SAT: Type = 12;
pub const RETURN: Type = 11;
pub const RELEASE: Type = 10;
pub const PHASE: Type = 9;
pub const OPTION: Type = 8;
pub const REUSE: Type = 7;
pub const MELT: Type = 6;
pub const INIT: Type = 5;
pub const FREEZE: Type = 4;
pub const FAILED: Type = 3;
pub const DEREF: Type = 2;
pub const ASSUME: Type = 1;
pub const ADD: Type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub type_0: Type,
    pub removed: libc::c_int,
    pub arg: libc::c_int,
    pub opt: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Opt {
    pub name: *mut libc::c_char,
    pub val: libc::c_int,
    pub min: libc::c_int,
    pub max: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub from: libc::c_int,
    pub to: libc::c_int,
    pub removed: libc::c_int,
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut runs: libc::c_int = 0;
static mut golden: libc::c_int = 0;
static mut timelimit: libc::c_int = 0;
static mut lineno: libc::c_int = 0;
static mut verbose: libc::c_int = 0;
static mut checkreturn: libc::c_int = 0;
static mut ddopts: libc::c_int = 0;
static mut nmap: libc::c_int = -(1 as libc::c_int);
static mut szmap: libc::c_int = 0;
static mut map: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut prevents: libc::c_int = 0;
static mut iname: *const libc::c_char = 0 as *const libc::c_char;
static mut oname: *const libc::c_char = 0 as *const libc::c_char;
static mut events: *mut Event = 0 as *const Event as *mut Event;
static mut nevents: libc::c_int = 0;
static mut szevents: libc::c_int = 0;
static mut opts: *mut Opt = 0 as *const Opt as *mut Opt;
static mut nopts: libc::c_int = 0;
static mut szopts: libc::c_int = 0;
static mut sumoptvals: libc::c_longlong = 0;
unsafe extern "C" fn event(
    mut type_0: Type,
    mut arg: libc::c_int,
    mut opt: *const libc::c_char,
) {
    let mut e: *mut Event = 0 as *mut Event;
    let mut idx: libc::c_int = 0;
    if nevents == szevents {
        szevents = if szevents != 0 {
            2 as libc::c_int * szevents
        } else {
            1 as libc::c_int
        };
        events = realloc(
            events as *mut libc::c_void,
            (szevents as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Event>() as libc::c_ulong),
        ) as *mut Event;
    }
    let fresh0 = nevents;
    nevents = nevents + 1;
    e = events.offset(fresh0 as isize);
    (*e).type_0 = type_0;
    (*e).arg = arg;
    (*e).opt = if !opt.is_null() { strdup(opt) } else { 0 as *mut libc::c_char };
    (*e).removed = 2147483647 as libc::c_int;
    match type_0 as libc::c_uint {
        0 | 1 | 2 | 3 | 4 | 15 | 16 | 17 | 6 | 7 | 9 | 26 | 21 | 23 | 22 | 14 => {
            idx = abs((*e).arg);
            if idx > nmap {
                if idx >= szmap {
                    loop {
                        szmap = if szmap != 0 {
                            2 as libc::c_int * szmap
                        } else {
                            2 as libc::c_int
                        };
                        if !(szmap <= idx) {
                            break;
                        }
                    }
                    map = realloc(
                        map as *mut libc::c_void,
                        (szmap as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_int;
                }
                nmap = idx;
            }
            *map.offset(idx as isize) = idx;
        }
        _ => {}
    };
}
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fputs(b"*** lglddtrace: \0" as *const u8 as *const libc::c_char, stderr);
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
        b"*** lglddtrace: parse error in '%s' line %d: \0" as *const u8
            as *const libc::c_char,
        iname,
        lineno,
    );
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn rep(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
        return;
    }
    if isatty(1 as libc::c_int) == 0 {
        return;
    }
    fputs(b"c [lglddtrace] \0" as *const u8 as *const libc::c_char, stdout);
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputs(b"       \r\0" as *const u8 as *const libc::c_char, stdout);
    fflush(stdout);
}
unsafe extern "C" fn msg(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
        return;
    }
    fputs(b"c [lglddtrace] \0" as *const u8 as *const libc::c_char, stdout);
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
    let fresh1 = p;
    p = p.offset(1);
    if *(*__ctype_b_loc()).offset(*fresh1 as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        ch = *p as libc::c_int;
        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
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
    tok = strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char);
    if tok.is_null() || isnumstr(tok) == 0
        || !(strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char))
            .is_null()
    {
        perr(
            b"expected integer argument for '%s'\0" as *const u8 as *const libc::c_char,
            op,
        );
    }
    if !tok.is_null() {} else {
        __assert_fail(
            b"tok\0" as *const u8 as *const libc::c_char,
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"int intarg(char *)\0"))
                .as_ptr(),
        );
    }
    'c_5090: {
        if !tok.is_null() {} else {
            __assert_fail(
                b"tok\0" as *const u8 as *const libc::c_char,
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"int intarg(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return atoi(tok);
}
unsafe extern "C" fn remr(mut r: *mut Range) -> libc::c_int {
    return ((*r).removed <= runs) as libc::c_int;
}
unsafe extern "C" fn reme(mut e: *mut Event) -> libc::c_int {
    return ((*e).removed <= runs) as libc::c_int;
}
unsafe extern "C" fn onabort(mut d: *mut libc::c_void) {
    exit(0 as libc::c_int);
}
unsafe extern "C" fn process() {
    let mut saved1: libc::c_int = 0;
    let mut saved2: libc::c_int = 0;
    let mut null: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut rlim: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    let mut lgl: *mut LGL = 0 as *mut LGL;
    let mut e: *mut Event = 0 as *mut Event;
    let mut o: *mut Opt = 0 as *mut Opt;
    rlim.rlim_cur = timelimit as rlim_t;
    rlim.rlim_max = (rlim.rlim_cur).wrapping_add(10 as libc::c_int as rlim_t);
    setrlimit(RLIMIT_CPU as libc::c_int, &mut rlim);
    saved1 = dup(1 as libc::c_int);
    saved2 = dup(2 as libc::c_int);
    null = open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0o1 as libc::c_int);
    close(1 as libc::c_int);
    close(2 as libc::c_int);
    tmp = dup(null);
    if tmp == 1 as libc::c_int {} else {
        __assert_fail(
            b"tmp == 1\0" as *const u8 as *const libc::c_char,
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"void process(void)\0"))
                .as_ptr(),
        );
    }
    'c_6042: {
        if tmp == 1 as libc::c_int {} else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"void process(void)\0"))
                    .as_ptr(),
            );
        }
    };
    tmp = dup(null);
    if tmp == 2 as libc::c_int {} else {
        __assert_fail(
            b"tmp == 2\0" as *const u8 as *const libc::c_char,
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"void process(void)\0"))
                .as_ptr(),
        );
    }
    'c_5999: {
        if tmp == 2 as libc::c_int {} else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"void process(void)\0"))
                    .as_ptr(),
            );
        }
    };
    lgl = 0 as *mut LGL;
    res = 0 as libc::c_int;
    e = events;
    while e < events.offset(nevents as isize) {
        if !(reme(e) != 0) {
            match (*e).type_0 as libc::c_uint {
                0 => {
                    lgladd(lgl, (*e).arg);
                }
                1 => {
                    lglassume(lgl, (*e).arg);
                }
                28 => {
                    lglchkclone(lgl);
                }
                2 => {
                    res = lglderef(lgl, (*e).arg);
                }
                26 => {
                    res = lglfixed(lgl, (*e).arg);
                }
                21 => {
                    res = lglfrozen(lgl, (*e).arg);
                }
                23 => {
                    res = lglreusable(lgl, (*e).arg);
                }
                22 => {
                    res = lglusable(lgl, (*e).arg);
                }
                14 => {
                    res = lglrepr(lgl, (*e).arg);
                }
                3 => {
                    res = lglfailed(lgl, (*e).arg);
                }
                27 => {
                    lglfixate(lgl);
                }
                20 => {
                    lglreducecache(lgl);
                }
                19 => {
                    lglflushcache(lgl);
                }
                15 => {
                    lglsetimportant(lgl, (*e).arg);
                }
                18 => {
                    lglsetphases(lgl);
                }
                16 => {
                    lglsetphase(lgl, (*e).arg);
                }
                17 => {
                    lglresetphase(lgl, (*e).arg);
                }
                4 => {
                    lglfreeze(lgl, (*e).arg);
                }
                30 => {
                    res = lglinconsistent(lgl);
                }
                31 => {
                    res = lglookahead(lgl);
                }
                25 => {
                    res = lglincvar(lgl);
                }
                24 => {
                    res = lglincvar(lgl);
                }
                29 => {
                    res = lglchanged(lgl);
                }
                5 => {
                    lgl = lglinit();
                    lglonabort(
                        lgl,
                        0 as *mut libc::c_void,
                        Some(onabort as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                    );
                    if !opts.is_null() {
                        if ddopts != 0 {} else {
                            __assert_fail(
                                b"ddopts\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                224 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 19],
                                    &[libc::c_char; 19],
                                >(b"void process(void)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_5631: {
                            if ddopts != 0 {} else {
                                __assert_fail(
                                    b"ddopts\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    224 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 19],
                                        &[libc::c_char; 19],
                                    >(b"void process(void)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        o = opts;
                        while o < opts.offset(nopts as isize) {
                            lglsetopt(lgl, (*o).name, (*o).val);
                            o = o.offset(1);
                            o;
                        }
                    }
                }
                6 => {
                    lglmelt(lgl, (*e).arg);
                }
                7 => {
                    lglreuse(lgl, (*e).arg);
                }
                8 => {
                    if opts.is_null() {
                        lglsetopt(lgl, (*e).opt, (*e).arg);
                    }
                }
                13 => {
                    res = lglsimp(lgl, (*e).arg);
                }
                10 => {
                    lglrelease(lgl);
                }
                11 => {
                    if checkreturn != 0 {
                        if (*e).arg == res {} else {
                            __assert_fail(
                                b"e->arg == res\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                235 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 19],
                                    &[libc::c_char; 19],
                                >(b"void process(void)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_5474: {
                            if (*e).arg == res {} else {
                                __assert_fail(
                                    b"e->arg == res\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    235 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 19],
                                        &[libc::c_char; 19],
                                    >(b"void process(void)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                    }
                }
                12 | _ => {
                    if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            238 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 19],
                                &[libc::c_char; 19],
                            >(b"void process(void)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_5422: {
                        if (*e).type_0 as libc::c_uint
                            == SAT as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                238 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 19],
                                    &[libc::c_char; 19],
                                >(b"void process(void)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    res = lglsat(lgl);
                }
            }
        }
        e = e.offset(1);
        e;
    }
    close(null);
    close(2 as libc::c_int);
    close(1 as libc::c_int);
    tmp = dup(saved1);
    if tmp == 1 as libc::c_int {} else {
        __assert_fail(
            b"tmp == 1\0" as *const u8 as *const libc::c_char,
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"void process(void)\0"))
                .as_ptr(),
        );
    }
    'c_5278: {
        if tmp == 1 as libc::c_int {} else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                247 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"void process(void)\0"))
                    .as_ptr(),
            );
        }
    };
    tmp = dup(saved2);
    if tmp == 2 as libc::c_int {} else {
        __assert_fail(
            b"tmp == 2\0" as *const u8 as *const libc::c_char,
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"void process(void)\0"))
                .as_ptr(),
        );
    }
    'c_5233: {
        if tmp == 2 as libc::c_int {} else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                249 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 19],
                    &[libc::c_char; 19],
                >(b"void process(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn run() -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut id: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    id = fork();
    if id != 0 {
        if id < 0 as libc::c_int {
            die(b"can not generate child process\0" as *const u8 as *const libc::c_char);
        }
        tmp = wait(&mut status);
        if tmp != id {
            die(
                b"'wait' did not return child process\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        process();
        exit(0 as libc::c_int);
    }
    runs += 1;
    runs;
    return status;
}
unsafe extern "C" fn lit(mut lit_0: libc::c_int) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    if lit_0 == 0 {
        return 0 as libc::c_int;
    }
    idx = abs(lit_0);
    if (0 as libc::c_int) < idx && idx <= nmap {} else {
        __assert_fail(
            b"0 < idx && idx <= nmap\0" as *const u8 as *const libc::c_char,
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"int lit(int)\0"))
                .as_ptr(),
        );
    }
    'c_6238: {
        if (0 as libc::c_int) < idx && idx <= nmap {} else {
            __assert_fail(
                b"0 < idx && idx <= nmap\0" as *const u8 as *const libc::c_char,
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                268 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"int lit(int)\0"))
                    .as_ptr(),
            );
        }
    };
    res = *map.offset(idx as isize);
    if lit_0 < 0 as libc::c_int {
        res = -res;
    }
    return res;
}
unsafe extern "C" fn print(mut e: *mut Event, mut file: *mut FILE) {
    let mut o: *mut Opt = 0 as *mut Opt;
    match (*e).type_0 as libc::c_uint {
        0 => {
            fprintf(
                file,
                b"add %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        1 => {
            fprintf(
                file,
                b"assume %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        29 => {
            fprintf(file, b"changed\n\0" as *const u8 as *const libc::c_char);
        }
        28 => {
            fprintf(file, b"chkclone\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(
                file,
                b"deref %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        3 => {
            fprintf(
                file,
                b"failed %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        26 => {
            fprintf(
                file,
                b"fixed %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        21 => {
            fprintf(
                file,
                b"frozen %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        23 => {
            fprintf(
                file,
                b"reusable %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        22 => {
            fprintf(
                file,
                b"usable %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        14 => {
            fprintf(
                file,
                b"repr %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        27 => {
            fprintf(file, b"fixate\n\0" as *const u8 as *const libc::c_char);
        }
        20 => {
            fprintf(file, b"reduce\n\0" as *const u8 as *const libc::c_char);
        }
        19 => {
            fprintf(file, b"flush\n\0" as *const u8 as *const libc::c_char);
        }
        15 => {
            fprintf(
                file,
                b"setimportant %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        18 => {
            fprintf(file, b"setphases\n\0" as *const u8 as *const libc::c_char);
        }
        16 => {
            fprintf(
                file,
                b"setphase %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        17 => {
            fprintf(
                file,
                b"resetphase %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        4 => {
            fprintf(
                file,
                b"freeze %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        30 => {
            fprintf(file, b"inconsistent\n\0" as *const u8 as *const libc::c_char);
        }
        31 => {
            fprintf(file, b"lkhd\n\0" as *const u8 as *const libc::c_char);
        }
        25 => {
            fprintf(file, b"incvar\n\0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf(file, b"init\n\0" as *const u8 as *const libc::c_char);
            if !opts.is_null() {
                if ddopts != 0 {} else {
                    __assert_fail(
                        b"ddopts\0" as *const u8 as *const libc::c_char,
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        302 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 28],
                            &[libc::c_char; 28],
                        >(b"void print(Event *, FILE *)\0"))
                            .as_ptr(),
                    );
                }
                'c_6595: {
                    if ddopts != 0 {} else {
                        __assert_fail(
                            b"ddopts\0" as *const u8 as *const libc::c_char,
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            302 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 28],
                                &[libc::c_char; 28],
                            >(b"void print(Event *, FILE *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                o = opts;
                while o < opts.offset(nopts as isize) {
                    fprintf(
                        file,
                        b"option %s %d\n\0" as *const u8 as *const libc::c_char,
                        (*o).name,
                        (*o).val,
                    );
                    o = o.offset(1);
                    o;
                }
            }
        }
        24 => {
            fprintf(file, b"maxvar\n\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            fprintf(
                file,
                b"melt %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        7 => {
            fprintf(
                file,
                b"reuse %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        8 => {
            fprintf(
                file,
                b"option %s %d\n\0" as *const u8 as *const libc::c_char,
                (*e).opt,
                (*e).arg,
            );
        }
        9 => {
            fprintf(
                file,
                b"phase %d\n\0" as *const u8 as *const libc::c_char,
                lit((*e).arg),
            );
        }
        10 => {
            fprintf(file, b"release\n\0" as *const u8 as *const libc::c_char);
        }
        11 => {
            fprintf(
                file,
                b"return %d\n\0" as *const u8 as *const libc::c_char,
                (*e).arg,
            );
        }
        13 => {
            fprintf(file, b"simp %d\n\0" as *const u8 as *const libc::c_char, (*e).arg);
        }
        12 | _ => {
            if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                    316 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 28],
                        &[libc::c_char; 28],
                    >(b"void print(Event *, FILE *)\0"))
                        .as_ptr(),
                );
            }
            'c_6385: {
                if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        316 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 28],
                            &[libc::c_char; 28],
                        >(b"void print(Event *, FILE *)\0"))
                            .as_ptr(),
                    );
                }
            };
            fprintf(file, b"sat\n\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn type2str(mut type_0: Type) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        0 => return b"add\0" as *const u8 as *const libc::c_char,
        1 => return b"assume\0" as *const u8 as *const libc::c_char,
        29 => return b"changed\0" as *const u8 as *const libc::c_char,
        28 => return b"chkclone\0" as *const u8 as *const libc::c_char,
        2 => return b"deref\0" as *const u8 as *const libc::c_char,
        3 => return b"failed\0" as *const u8 as *const libc::c_char,
        26 => return b"fixed\0" as *const u8 as *const libc::c_char,
        21 => return b"frozen\0" as *const u8 as *const libc::c_char,
        23 => return b"reusable\0" as *const u8 as *const libc::c_char,
        22 => return b"usable\0" as *const u8 as *const libc::c_char,
        14 => return b"repr\0" as *const u8 as *const libc::c_char,
        27 => return b"fixate\0" as *const u8 as *const libc::c_char,
        20 => return b"reduce\0" as *const u8 as *const libc::c_char,
        19 => return b"flush\0" as *const u8 as *const libc::c_char,
        4 => return b"freeze\0" as *const u8 as *const libc::c_char,
        15 => return b"setimportant\0" as *const u8 as *const libc::c_char,
        18 => return b"setphases\0" as *const u8 as *const libc::c_char,
        16 => return b"setphase\0" as *const u8 as *const libc::c_char,
        17 => return b"resetphase\0" as *const u8 as *const libc::c_char,
        25 => return b"incvar\0" as *const u8 as *const libc::c_char,
        30 => return b"inconsistent\0" as *const u8 as *const libc::c_char,
        31 => return b"lkhd\0" as *const u8 as *const libc::c_char,
        5 => return b"init\0" as *const u8 as *const libc::c_char,
        24 => return b"maxvar\0" as *const u8 as *const libc::c_char,
        6 => return b"melt\0" as *const u8 as *const libc::c_char,
        7 => return b"REUSE\0" as *const u8 as *const libc::c_char,
        8 => return b"option\0" as *const u8 as *const libc::c_char,
        9 => return b"phase\0" as *const u8 as *const libc::c_char,
        10 => return b"release\0" as *const u8 as *const libc::c_char,
        11 => return b"return\0" as *const u8 as *const libc::c_char,
        13 => return b"simp\0" as *const u8 as *const libc::c_char,
        12 | _ => {
            if type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"type == SAT\0" as *const u8 as *const libc::c_char,
                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                    356 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 27],
                        &[libc::c_char; 27],
                    >(b"const char *type2str(Type)\0"))
                        .as_ptr(),
                );
            }
            'c_7023: {
                if type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"type == SAT\0" as *const u8 as *const libc::c_char,
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        356 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 27],
                            &[libc::c_char; 27],
                        >(b"const char *type2str(Type)\0"))
                            .as_ptr(),
                    );
                }
            };
            return b"sat\0" as *const u8 as *const libc::c_char;
        }
    };
}
unsafe extern "C" fn noarg(mut op: Type) {
    if !(strtok(0 as *mut libc::c_char, b" \0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        perr(b"argument after '%s'\0" as *const u8 as *const libc::c_char, type2str(op));
    }
    event(op, 0 as libc::c_int, 0 as *const libc::c_char);
}
unsafe extern "C" fn newline() {
    let mut i: libc::c_int = 0;
    if verbose == 0 {
        return;
    }
    if isatty(1 as libc::c_int) == 0 {
        return;
    }
    printf(b"\r\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 78 as libc::c_int {
        fputc(' ' as i32, stdout);
        i += 1;
        i;
    }
    printf(b"\r\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
unsafe extern "C" fn prt(mut final_0: libc::c_int) {
    let mut close_0: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    unlink(oname);
    len = strlen(oname) as libc::c_int;
    if len >= 3 as libc::c_int
        && strcmp(
            oname.offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b".gz\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        cmd = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        sprintf(cmd, b"gzip -c > %s\0" as *const u8 as *const libc::c_char, oname);
        file = popen(cmd, b"w\0" as *const u8 as *const libc::c_char);
        if !file.is_null() {
            close_0 = 2 as libc::c_int;
        }
        free(cmd as *mut libc::c_void);
    } else {
        file = fopen(oname, b"w\0" as *const u8 as *const libc::c_char);
        if !file.is_null() {
            close_0 = 1 as libc::c_int;
        }
    }
    if file.is_null() {
        die(b"can not write to '%s'\0" as *const u8 as *const libc::c_char, oname);
    }
    prevents = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nevents {
        if !(reme(events.offset(i as isize)) != 0) {
            print(events.offset(i as isize), file);
            prevents += 1;
            prevents;
        }
        i += 1;
        i;
    }
    if close_0 == 2 as libc::c_int {
        pclose(file);
    }
    if close_0 == 1 as libc::c_int {
        fclose(file);
    }
    if verbose != 0 && isatty(1 as libc::c_int) != 0 {
        fputc('\r' as i32, stdout);
        i = 0 as libc::c_int;
        while i < 78 as libc::c_int {
            fputc(' ' as i32, stdout);
            i += 1;
            i;
        }
        fputc('\r' as i32, stdout);
    }
    msg(
        b"written %s with %d events\0" as *const u8 as *const libc::c_char,
        oname,
        prevents,
    );
}
unsafe extern "C" fn dd() {
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut rgran: libc::c_int = 0;
    let mut nranges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut ranges: *mut Range = calloc(
        nevents as libc::c_ulong,
        ::core::mem::size_of::<Range>() as libc::c_ulong,
    ) as *mut Range;
    let mut r: *mut Range = 0 as *mut Range;
    let mut smap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut moved: libc::c_int = 0;
    let mut mapto: libc::c_int = 0;
    let mut nused: libc::c_int = 0;
    let mut cluster: Type = ADD;
    let mut used: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut Event = 0 as *mut Event;
    loop {
        prt(0 as libc::c_int);
        changed = 0 as libc::c_int;
        nused = 0 as libc::c_int;
        used = malloc((nmap + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        memset(
            used as *mut libc::c_void,
            0 as libc::c_int,
            (nmap + 1 as libc::c_int) as libc::c_ulong,
        );
        e = events;
        while e < events.offset(nevents as isize) {
            if !(reme(e) != 0) {
                idx = 0 as libc::c_int;
                match (*e).type_0 as libc::c_uint {
                    0 | 1 | 2 | 3 | 26 | 21 | 23 | 22 | 14 | 15 | 16 | 17 | 4 | 6 | 7
                    | 9 => {
                        idx = (*e).arg;
                    }
                    5 | 8 | 10 | 11 | 12 | 13 | 29 | 24 | 25 | 30 | 31 | 27 | 19 | 20
                    | 28 | 18 | _ => {}
                }
                if !(idx == 0) {
                    idx = abs(idx);
                    if !(*used.offset(idx as isize) != 0) {
                        *used.offset(idx as isize) = 1 as libc::c_int as libc::c_char;
                        nused += 1;
                        nused;
                    }
                }
            }
            e = e.offset(1);
            e;
        }
        if nused < nmap {
            let mut current_block_37: u64;
            smap = map;
            moved = 0 as libc::c_int;
            mapto = 1 as libc::c_int;
            map = malloc(
                (szmap as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            idx = 1 as libc::c_int;
            while idx <= nmap {
                if *used.offset(idx as isize) != 0 {
                    if *smap.offset(idx as isize) != mapto {
                        if mapto < *smap.offset(idx as isize) {} else {
                            __assert_fail(
                                b"mapto < smap[idx]\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                479 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 14],
                                    &[libc::c_char; 14],
                                >(b"void dd(void)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_9636: {
                            if mapto < *smap.offset(idx as isize) {} else {
                                __assert_fail(
                                    b"mapto < smap[idx]\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    479 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 14],
                                        &[libc::c_char; 14],
                                    >(b"void dd(void)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        moved += 1;
                        moved;
                    }
                    let fresh2 = mapto;
                    mapto = mapto + 1;
                    *map.offset(idx as isize) = fresh2;
                } else {
                    *map.offset(idx as isize) = 0 as libc::c_int;
                }
                idx += 1;
                idx;
            }
            if moved == 0 {
                current_block_37 = 3333766589073626915;
            } else {
                res = run();
                if res == golden {
                    if verbose > 1 as libc::c_int {
                        newline();
                        msg(
                            b"moved %d variables\0" as *const u8 as *const libc::c_char,
                            moved,
                        );
                    }
                    free(smap as *mut libc::c_void);
                    changed = 1 as libc::c_int;
                    current_block_37 = 11763295167351361500;
                } else {
                    current_block_37 = 3333766589073626915;
                }
            }
            match current_block_37 {
                3333766589073626915 => {
                    free(map as *mut libc::c_void);
                    map = smap;
                }
                _ => {}
            }
        }
        free(used as *mut libc::c_void);
        rgran = 2 as libc::c_int;
        while rgran >= 0 as libc::c_int {
            to = 0 as libc::c_int;
            from = to;
            r = ranges;
            loop {
                while from < nevents && reme(events.offset(from as isize)) != 0 {
                    from += 1;
                    from;
                }
                if from == nevents {
                    break;
                }
                to = from;
                if rgran == 2 as libc::c_int {
                    cluster = (*events.offset(from as isize)).type_0;
                    while (to + 1 as libc::c_int) < nevents {
                        e = events.offset(to as isize).offset(1 as libc::c_int as isize);
                        if (*e).type_0 as libc::c_uint != cluster as libc::c_uint {
                            if !(cluster as libc::c_uint
                                == DEREF as libc::c_int as libc::c_uint
                                && (*e).type_0 as libc::c_uint
                                    == RETURN as libc::c_int as libc::c_uint)
                            {
                                if !(cluster as libc::c_uint
                                    == SAT as libc::c_int as libc::c_uint
                                    && (*e).type_0 as libc::c_uint
                                        == RETURN as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                                to += 1;
                                to;
                                break;
                            }
                        } else if cluster as libc::c_uint
                            == INIT as libc::c_int as libc::c_uint
                            || cluster as libc::c_uint
                                == SAT as libc::c_int as libc::c_uint
                            || cluster as libc::c_uint
                                == RELEASE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                        to += 1;
                        to;
                    }
                } else if rgran == 1 as libc::c_int {
                    if (to + 1 as libc::c_int) < nevents
                        && (*events.offset(to as isize)).type_0 as libc::c_uint
                            == ADD as libc::c_int as libc::c_uint
                    {
                        while (to + 1 as libc::c_int) < nevents
                            && {
                                e = events.offset(to as isize);
                                reme(e) != 0
                                    || (*e).type_0 as libc::c_uint
                                        == ADD as libc::c_int as libc::c_uint && (*e).arg != 0
                            }
                        {
                            to += 1;
                            to;
                        }
                    }
                } else if (to + 1 as libc::c_int) < nevents {
                    while (to + 1 as libc::c_int) < nevents
                        && reme(events.offset(to as isize)) != 0
                    {
                        to += 1;
                        to;
                    }
                }
                if r < ranges.offset(nevents as isize) {} else {
                    __assert_fail(
                        b"r < ranges + nevents\0" as *const u8 as *const libc::c_char,
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        534 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 14],
                            &[libc::c_char; 14],
                        >(b"void dd(void)\0"))
                            .as_ptr(),
                    );
                }
                'c_9234: {
                    if r < ranges.offset(nevents as isize) {} else {
                        __assert_fail(
                            b"r < ranges + nevents\0" as *const u8
                                as *const libc::c_char,
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            534 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 14],
                                &[libc::c_char; 14],
                            >(b"void dd(void)\0"))
                                .as_ptr(),
                        );
                    }
                };
                (*r).from = from;
                (*r).to = to;
                (*r).removed = 2147483647 as libc::c_int;
                if verbose > 1 as libc::c_int {
                    msg(
                        b"range %d [%d,%d]\0" as *const u8 as *const libc::c_char,
                        r.offset_from(ranges) as libc::c_long,
                        from,
                        to,
                    );
                }
                if verbose > 2 as libc::c_int {
                    i = from;
                    while i <= to {
                        e = events.offset(i as isize);
                        if !(reme(e) != 0 && verbose < 4 as libc::c_int) {
                            if (*e).type_0 as libc::c_uint
                                == OPTION as libc::c_int as libc::c_uint
                            {
                                msg(
                                    b"range %d [%d] %s %s %d%s\0" as *const u8
                                        as *const libc::c_char,
                                    r.offset_from(ranges) as libc::c_long,
                                    i,
                                    type2str((*e).type_0),
                                    (*e).opt,
                                    (*e).arg,
                                    if reme(e) != 0 {
                                        b" (removed)\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            } else {
                                msg(
                                    b"range %d [%d] %s %d%s\0" as *const u8
                                        as *const libc::c_char,
                                    r.offset_from(ranges) as libc::c_long,
                                    i,
                                    type2str((*e).type_0),
                                    (*e).arg,
                                    if reme(e) != 0 {
                                        b" (removed)\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                        }
                        i += 1;
                        i;
                    }
                }
                r = r.offset(1);
                r;
                from = to + 1 as libc::c_int;
                if from == nevents {
                    break;
                }
            }
            nranges = r.offset_from(ranges) as libc::c_long as libc::c_int;
            if verbose > 1 as libc::c_int {
                msg(
                    b"found %d ranges of range granularity %d\0" as *const u8
                        as *const libc::c_char,
                    nranges,
                    rgran,
                );
            }
            width = nranges / 2 as libc::c_int;
            while width > 0 as libc::c_int {
                pos = 0 as libc::c_int;
                loop {
                    rep(
                        b"g%d w%d : %6d .. %-6d / %d %lld\0" as *const u8
                            as *const libc::c_char,
                        rgran,
                        width,
                        pos,
                        if pos + width <= nranges {
                            pos + width - 1 as libc::c_int
                        } else {
                            nranges - 1 as libc::c_int
                        },
                        nranges,
                        sumoptvals,
                    );
                    found = 0 as libc::c_int;
                    i = pos;
                    while i < nranges && i < pos + width {
                        r = ranges.offset(i as isize);
                        if !(remr(r) != 0) {
                            (*r).removed = runs;
                            found = 0 as libc::c_int;
                            j = (*r).from;
                            while j <= (*r).to {
                                e = events.offset(j as isize);
                                if !(reme(e) != 0) {
                                    found += 1;
                                    found;
                                    (*e).removed = runs;
                                }
                                j += 1;
                                j;
                            }
                            if found != 0 {} else {
                                __assert_fail(
                                    b"found\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    581 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 14],
                                        &[libc::c_char; 14],
                                    >(b"void dd(void)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_8856: {
                                if found != 0 {} else {
                                    __assert_fail(
                                        b"found\0" as *const u8 as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        581 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 14],
                                            &[libc::c_char; 14],
                                        >(b"void dd(void)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                        }
                        i += 1;
                        i;
                    }
                    res = run();
                    if res == golden {
                        if verbose > 1 as libc::c_int {
                            newline();
                            msg(
                                b"removed %d events\0" as *const u8 as *const libc::c_char,
                                found,
                            );
                        }
                        changed = 1 as libc::c_int;
                    } else {
                        i = pos;
                        while i < nranges && i < pos + width {
                            r = ranges.offset(i as isize);
                            if !((*r).removed < runs - 1 as libc::c_int) {
                                if (*r).removed == runs - 1 as libc::c_int {} else {
                                    __assert_fail(
                                        b"r->removed == runs - 1\0" as *const u8
                                            as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        594 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 14],
                                            &[libc::c_char; 14],
                                        >(b"void dd(void)\0"))
                                            .as_ptr(),
                                    );
                                }
                                'c_8732: {
                                    if (*r).removed == runs - 1 as libc::c_int {} else {
                                        __assert_fail(
                                            b"r->removed == runs - 1\0" as *const u8
                                                as *const libc::c_char,
                                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                            594 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 14],
                                                &[libc::c_char; 14],
                                            >(b"void dd(void)\0"))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                (*r).removed = 2147483647 as libc::c_int;
                                j = (*r).from;
                                while j <= (*r).to {
                                    e = events.offset(j as isize);
                                    if (*e).removed < runs {} else {
                                        __assert_fail(
                                            b"e->removed < runs\0" as *const u8 as *const libc::c_char,
                                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                            598 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 14],
                                                &[libc::c_char; 14],
                                            >(b"void dd(void)\0"))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_8662: {
                                        if (*e).removed < runs {} else {
                                            __assert_fail(
                                                b"e->removed < runs\0" as *const u8 as *const libc::c_char,
                                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                                598 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 14],
                                                    &[libc::c_char; 14],
                                                >(b"void dd(void)\0"))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    if (*e).removed == runs - 1 as libc::c_int {
                                        (*e).removed = 2147483647 as libc::c_int;
                                    }
                                    j += 1;
                                    j;
                                }
                            }
                            i += 1;
                            i;
                        }
                    }
                    pos += width;
                    if !(pos < nranges) {
                        break;
                    }
                }
                width = if width > 4 as libc::c_int {
                    width / 2 as libc::c_int
                } else {
                    width - 1 as libc::c_int
                };
            }
            if verbose > 1 as libc::c_int {
                newline();
            }
            rgran -= 1;
            rgran;
        }
        if ddopts != 0 {
            let mut reported: libc::c_int = 0 as libc::c_int;
            let mut o: *mut Opt = 0 as *mut Opt;
            if opts.is_null() {
                let mut it: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut name: *const libc::c_char = 0 as *const libc::c_char;
                let mut opt: Opt = Opt {
                    name: 0 as *mut libc::c_char,
                    val: 0,
                    min: 0,
                    max: 0,
                };
                let mut lgl: *mut LGL = lglinit();
                it = lglfirstopt(lgl);
                loop {
                    it = lglnextopt(
                        lgl,
                        it,
                        &mut name,
                        &mut opt.val,
                        &mut opt.min,
                        &mut opt.max,
                    );
                    if it.is_null() {
                        break;
                    }
                    if strcmp(name, b"log\0" as *const u8 as *const libc::c_char) == 0 {
                        continue;
                    }
                    if strcmp(name, b"check\0" as *const u8 as *const libc::c_char) == 0
                    {
                        continue;
                    }
                    if strcmp(name, b"verbose\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        continue;
                    }
                    if strcmp(name, b"witness\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        continue;
                    }
                    if strcmp(name, b"exitonabort\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        continue;
                    }
                    if strcmp(
                        name,
                        b"sleeponabort\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        continue;
                    }
                    if nopts == szopts {
                        szopts = if szopts != 0 {
                            2 as libc::c_int * szopts
                        } else {
                            1 as libc::c_int
                        };
                        opts = realloc(
                            opts as *mut libc::c_void,
                            (szopts as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<Opt>() as libc::c_ulong,
                                ),
                        ) as *mut Opt;
                    }
                    opt.name = strdup(name);
                    let fresh3 = nopts;
                    nopts = nopts + 1;
                    *opts.offset(fresh3 as isize) = opt;
                }
                e = events;
                while e < events.offset(nevents as isize) {
                    if !((*e).type_0 as libc::c_uint
                        != OPTION as libc::c_int as libc::c_uint)
                    {
                        o = opts;
                        while o < opts.offset(nopts as isize) {
                            if strcmp((*e).opt, (*o).name) == 0 {
                                (*o).val = (*e).arg;
                                sumoptvals
                                    += (*o).val as libc::c_longlong
                                        - (*o).min as libc::c_longlong;
                            }
                            o = o.offset(1);
                            o;
                        }
                    }
                    e = e.offset(1);
                    e;
                }
                lglrelease(lgl);
            }
            o = opts;
            while o < opts.offset(nopts as isize) {
                let mut delta: libc::c_longlong = (*o).val as libc::c_longlong
                    - (*o).min as libc::c_longlong;
                rep(
                    b"o %d / %d %lld                            \0" as *const u8
                        as *const libc::c_char,
                    o.offset_from(opts) as libc::c_long,
                    nopts,
                    sumoptvals,
                );
                reported += 1;
                reported;
                if (*o).val > (*o).min {
                    let mut oldval: libc::c_int = (*o).val;
                    (*o).val = (*o).min;
                    res = run();
                    if res == golden {
                        if verbose > 1 as libc::c_int {
                            if reported != 0 {
                                newline();
                            }
                            msg(
                                b"reduced option %s from %d to %d by one\0" as *const u8
                                    as *const libc::c_char,
                                (*o).name,
                                oldval,
                                (*o).val,
                            );
                            reported = 0 as libc::c_int;
                        }
                        changed = 1 as libc::c_int;
                        sumoptvals -= 1;
                        sumoptvals;
                    } else {
                        (*o).val = oldval;
                    }
                }
                if delta < 10 as libc::c_int as libc::c_longlong {
                    while (*o).val > (*o).min {
                        let mut oldval_0: libc::c_int = (*o).val;
                        let mut newval: libc::c_int = oldval_0 - 1 as libc::c_int;
                        if newval >= (*o).min {} else {
                            __assert_fail(
                                b"newval >= o->min\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                666 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 14],
                                    &[libc::c_char; 14],
                                >(b"void dd(void)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_8135: {
                            if newval >= (*o).min {} else {
                                __assert_fail(
                                    b"newval >= o->min\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    666 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 14],
                                        &[libc::c_char; 14],
                                    >(b"void dd(void)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        (*o).val = newval;
                        res = run();
                        if res != golden {
                            (*o).val = oldval_0;
                            break;
                        } else {
                            if verbose > 1 as libc::c_int {
                                if reported != 0 {
                                    newline();
                                }
                                msg(
                                    b"reduced option %s from %d to %d by one\0" as *const u8
                                        as *const libc::c_char,
                                    (*o).name,
                                    oldval_0,
                                    newval,
                                );
                                reported = 0 as libc::c_int;
                            }
                            changed = 1 as libc::c_int;
                            if oldval_0 - newval == 1 as libc::c_int {} else {
                                __assert_fail(
                                    b"oldval - newval == 1\0" as *const u8
                                        as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    677 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 14],
                                        &[libc::c_char; 14],
                                    >(b"void dd(void)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_8023: {
                                if oldval_0 - newval == 1 as libc::c_int {} else {
                                    __assert_fail(
                                        b"oldval - newval == 1\0" as *const u8
                                            as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        677 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 14],
                                            &[libc::c_char; 14],
                                        >(b"void dd(void)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            sumoptvals -= 1;
                            sumoptvals;
                        }
                    }
                } else {
                    let mut upper: libc::c_int = (*o).val;
                    let mut lower: libc::c_int = (*o).min;
                    if lower <= upper {} else {
                        __assert_fail(
                            b"lower <= upper\0" as *const u8 as *const libc::c_char,
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            684 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 14],
                                &[libc::c_char; 14],
                            >(b"void dd(void)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_7971: {
                        if lower <= upper {} else {
                            __assert_fail(
                                b"lower <= upper\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                684 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 14],
                                    &[libc::c_char; 14],
                                >(b"void dd(void)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    while upper > lower {
                        let mut oldval_1: libc::c_int = (*o).val;
                        let mut longnewval: libc::c_long = lower as libc::c_long
                            + (upper as libc::c_long - lower as libc::c_long)
                                / 2 as libc::c_int as libc::c_long;
                        let mut newval_0: libc::c_int = longnewval as libc::c_int;
                        (*o).val = newval_0;
                        res = run();
                        if res == golden {
                            if verbose > 1 as libc::c_int {
                                if reported != 0 {
                                    newline();
                                }
                                msg(
                                    b"reduced %s from %d to %d\0" as *const u8
                                        as *const libc::c_char,
                                    (*o).name,
                                    oldval_1,
                                    newval_0,
                                );
                                reported = 0 as libc::c_int;
                            }
                            changed = 1 as libc::c_int;
                            if newval_0 < upper {} else {
                                __assert_fail(
                                    b"newval < upper\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    698 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 14],
                                        &[libc::c_char; 14],
                                    >(b"void dd(void)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_7878: {
                                if newval_0 < upper {} else {
                                    __assert_fail(
                                        b"newval < upper\0" as *const u8 as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        698 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 14],
                                            &[libc::c_char; 14],
                                        >(b"void dd(void)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            upper = newval_0;
                            sumoptvals -= (oldval_1 - newval_0) as libc::c_longlong;
                        } else {
                            (*o).val = oldval_1;
                            if lower == newval_0 {
                                break;
                            }
                            if lower < newval_0 {} else {
                                __assert_fail(
                                    b"lower < newval\0" as *const u8 as *const libc::c_char,
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    705 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 14],
                                        &[libc::c_char; 14],
                                    >(b"void dd(void)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_7806: {
                                if lower < newval_0 {} else {
                                    __assert_fail(
                                        b"lower < newval\0" as *const u8 as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        705 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 14],
                                            &[libc::c_char; 14],
                                        >(b"void dd(void)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                            lower = newval_0;
                        }
                    }
                    if lower <= upper {} else {
                        __assert_fail(
                            b"lower <= upper\0" as *const u8 as *const libc::c_char,
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            709 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 14],
                                &[libc::c_char; 14],
                            >(b"void dd(void)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_7719: {
                        if lower <= upper {} else {
                            __assert_fail(
                                b"lower <= upper\0" as *const u8 as *const libc::c_char,
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                709 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 14],
                                    &[libc::c_char; 14],
                                >(b"void dd(void)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                if verbose > 1 as libc::c_int {
                    if reported != 0 {
                        newline();
                    }
                    msg(
                        b"final option %s set to %d\0" as *const u8
                            as *const libc::c_char,
                        (*o).name,
                        (*o).val,
                    );
                    reported = 0 as libc::c_int;
                }
                o = o.offset(1);
                o;
            }
            if reported != 0 && verbose > 1 as libc::c_int {
                newline();
            }
        }
        if !(changed != 0) {
            break;
        }
    }
    if verbose != 0 {
        newline();
    }
    free(ranges as *mut libc::c_void);
}
unsafe extern "C" fn getime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut close_0: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut buffer: [libc::c_char; 80] = [0; 80];
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut file: *mut FILE = 0 as *mut FILE;
    start = getime();
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize), b"-h\0" as *const u8 as *const libc::c_char)
            == 0
        {
            printf(
                b"usage: lglddtrace [-h][-v][-s][-d][-f][-O] <in>[.gz] <out>[.gz]\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            verbose += 1;
            verbose;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-c\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            checkreturn += 1;
            checkreturn;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-O\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ddopts += 1;
            ddopts;
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        {
            die(
                b"invalid command line option '%s' (try '-h')\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else if !oname.is_null() {
            die(
                b"two many options pecified (try '-h')\0" as *const u8
                    as *const libc::c_char,
            );
        } else if !iname.is_null() {
            oname = *argv.offset(i as isize);
        } else {
            iname = *argv.offset(i as isize);
        }
        i += 1;
        i;
    }
    if iname.is_null() {
        die(b"input file missing\0" as *const u8 as *const libc::c_char);
    }
    if oname.is_null() {
        die(b"output file missing\0" as *const u8 as *const libc::c_char);
    }
    len = strlen(iname) as libc::c_int;
    if len >= 3 as libc::c_int
        && strcmp(
            iname.offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b".gz\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        cmd = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        sprintf(cmd, b"gunzip -c %s\0" as *const u8 as *const libc::c_char, iname);
        file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
        free(cmd as *mut libc::c_void);
        if !file.is_null() {
            close_0 = 2 as libc::c_int;
        }
    } else {
        file = fopen(iname, b"r\0" as *const u8 as *const libc::c_char);
        if !file.is_null() {
            close_0 = 1 as libc::c_int;
        }
    }
    if file.is_null() {
        die(b"can not read '%s'\0" as *const u8 as *const libc::c_char, iname);
    }
    msg(b"reading %s\0" as *const u8 as *const libc::c_char, iname);
    len = 0 as libc::c_int;
    buffer[len as usize] = 0 as libc::c_int as libc::c_char;
    lineno = 1 as libc::c_int;
    count = 0 as libc::c_int;
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
                >= ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                    as libc::c_int
            {
                perr(b"line buffer exceeded\0" as *const u8 as *const libc::c_char);
            }
            let fresh4 = len;
            len = len + 1;
            buffer[fresh4 as usize] = ch as libc::c_char;
            buffer[len as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            if verbose > 2 as libc::c_int {
                msg(
                    b"line %d : %s\0" as *const u8 as *const libc::c_char,
                    lineno,
                    buffer.as_mut_ptr(),
                );
            }
            tok = strtok(
                buffer.as_mut_ptr(),
                b" \0" as *const u8 as *const libc::c_char,
            );
            if tok.is_null() {
                perr(b"empty line\0" as *const u8 as *const libc::c_char);
            } else if strcmp(tok, b"add\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    ADD,
                    intarg(
                        b"add\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"return\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    RETURN,
                    intarg(
                        b"return\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"deref\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    DEREF,
                    intarg(
                        b"deref\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"fixed\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    FIXED,
                    intarg(
                        b"fixed\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"frozen\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    FROZEN,
                    intarg(
                        b"frozen\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"reusable\0" as *const u8 as *const libc::c_char) == 0
            {
                event(
                    REUSABLE,
                    intarg(
                        b"reusable\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"usable\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    USABLE,
                    intarg(
                        b"usable\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"repr\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    REPR,
                    intarg(
                        b"repr\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"failed\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    FAILED,
                    intarg(
                        b"failed\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"assume\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    ASSUME,
                    intarg(
                        b"assume\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"phase\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    PHASE,
                    intarg(
                        b"phase\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"init\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(INIT);
            } else if strcmp(tok, b"sat\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(SAT);
            } else if strcmp(tok, b"simp\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    SIMP,
                    intarg(
                        b"simp\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"setphases\0" as *const u8 as *const libc::c_char)
                == 0
            {
                noarg(SETPHASES);
            } else if strcmp(tok, b"freeze\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    FREEZE,
                    intarg(
                        b"freeze\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"setimportant\0" as *const u8 as *const libc::c_char)
                == 0
            {
                event(
                    SETIMPORTANT,
                    intarg(
                        b"setimportant\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"setphase\0" as *const u8 as *const libc::c_char) == 0
            {
                event(
                    SETPHASE,
                    intarg(
                        b"setphase\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"resetphase\0" as *const u8 as *const libc::c_char)
                == 0
            {
                event(
                    SETPHASE,
                    intarg(
                        b"resetphase\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"melt\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    MELT,
                    intarg(
                        b"melt\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"reuse\0" as *const u8 as *const libc::c_char) == 0 {
                event(
                    REUSE,
                    intarg(
                        b"reuse\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    0 as *const libc::c_char,
                );
            } else if strcmp(tok, b"option\0" as *const u8 as *const libc::c_char) == 0 {
                opt = strtok(
                    0 as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                if opt.is_null() {
                    perr(b"option iname missing\0" as *const u8 as *const libc::c_char);
                }
                event(
                    OPTION,
                    intarg(
                        b"option\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    opt,
                );
            } else if strcmp(tok, b"release\0" as *const u8 as *const libc::c_char) == 0
            {
                noarg(RELEASE);
            } else if strcmp(tok, b"incvar\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(INCVAR);
            } else if strcmp(tok, b"maxvar\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(MAXVAR);
            } else if strcmp(tok, b"inconsistent\0" as *const u8 as *const libc::c_char)
                == 0
            {
                noarg(INCONSISTENT);
            } else if strcmp(tok, b"lkhd\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(LKHD);
            } else if strcmp(tok, b"fixate\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(FIXATE);
            } else if strcmp(tok, b"reduce\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(REDUCE);
            } else if strcmp(tok, b"flush\0" as *const u8 as *const libc::c_char) == 0 {
                noarg(FLUSH);
            } else if strcmp(tok, b"chkclone\0" as *const u8 as *const libc::c_char) == 0
            {
                noarg(CHKCLONE);
            } else if strcmp(tok, b"changed\0" as *const u8 as *const libc::c_char) == 0
            {
                noarg(CHANGED);
            } else {
                perr(b"invalid command '%s'\0" as *const u8 as *const libc::c_char, tok);
            }
            lineno += 1;
            lineno;
            count += 1;
            count;
            len = 0 as libc::c_int;
        }
    }
    if close_0 == 1 as libc::c_int {
        fclose(file);
    }
    if close_0 == 2 as libc::c_int {
        pclose(file);
    }
    msg(b"parsed %d events in %s\0" as *const u8 as *const libc::c_char, count, iname);
    golden = run();
    delta = getime() - start;
    timelimit = delta as libc::c_int;
    if delta < 0 as libc::c_int as libc::c_double {
        delta = 0 as libc::c_int as libc::c_double;
    }
    msg(
        b"golden exit code %d in %.3f seconds\0" as *const u8 as *const libc::c_char,
        golden,
        delta,
    );
    timelimit = delta as libc::c_int;
    if timelimit <= 0 as libc::c_int {
        timelimit = 1 as libc::c_int;
    }
    timelimit *= 100 as libc::c_int;
    msg(b"time limit %d seconds\0" as *const u8 as *const libc::c_char, timelimit);
    dd();
    prt(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < nevents {
        free((*events.offset(i as isize)).opt as *mut libc::c_void);
        i += 1;
        i;
    }
    free(events as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < nopts {
        free((*opts.offset(i as isize)).name as *mut libc::c_void);
        i += 1;
        i;
    }
    free(opts as *mut libc::c_void);
    msg(
        b"executed %d runs in %.3f seconds\0" as *const u8 as *const libc::c_char,
        runs,
        getime() - start,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
