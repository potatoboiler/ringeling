#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglclone(_: *mut LGL) -> *mut LGL;
    fn lglunclone(dst: *mut LGL, src: *mut LGL) -> libc::c_int;
    fn lglversion() -> *const libc::c_char;
    fn lglbnr(name: *const libc::c_char, prefix: *const libc::c_char, file: *mut FILE);
    fn lglsetout(_: *mut LGL, _: *mut FILE);
    fn lglsetprefix(_: *mut LGL, _: *const libc::c_char);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglsetid(_: *mut LGL, tid: libc::c_int, tids: libc::c_int);
    fn lgladd(_: *mut LGL, lit: libc::c_int);
    fn lglassume(_: *mut LGL, lit: libc::c_int);
    fn lglfixate(_: *mut LGL);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglflushcache(_: *mut LGL);
    fn lglfreeze(_: *mut LGL, lit: libc::c_int);
    fn lglmelt(_: *mut LGL, lit: libc::c_int);
    fn lglmeltall(_: *mut LGL);
    fn lglflushtimers(lgl: *mut LGL);
    fn lglstats(_: *mut LGL);
    fn lglgetconfs(_: *mut LGL) -> int64_t;
    fn lglgetdecs(_: *mut LGL) -> int64_t;
    fn lglgetprops(_: *mut LGL) -> int64_t;
    fn lglfailed(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglmb(_: *mut LGL) -> libc::c_double;
    fn lglprocesstime() -> libc::c_double;
    fn lglseterm(
        _: *mut LGL,
        term_0: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglsetmsglock(
        _: *mut LGL,
        lock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        unlock: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetime(_: *mut LGL, time: Option::<unsafe extern "C" fn() -> libc::c_double>);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
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
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Worker {
    pub lgl: *mut LGL,
    pub cloned: C2RustUnnamed_1,
    pub last: libc::c_int,
    pub res: libc::c_int,
    pub thread: pthread_t,
    pub proof: *mut FILE,
    pub post: *mut FILE,
    pub failed: *mut libc::c_int,
    pub nfailed: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub lgl: *mut LGL,
    pub count: libc::c_int,
    pub bcount: libc::c_int,
    pub decs: int64_t,
    pub confs: int64_t,
    pub props: int64_t,
    pub lock: pthread_mutex_t,
}
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
static mut startime: libc::c_double = 0.;
static mut allocated: size_t = 0;
static mut maxallocated: size_t = 0;
static mut statsfile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut histfile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut workers: *mut Worker = 0 as *const Worker as *mut Worker;
static mut nworkers: libc::c_int = 0;
static mut nassumptions: libc::c_int = 0;
static mut queue: libc::c_int = 0;
static mut szassumptions: libc::c_int = 0;
static mut maxassumptionsize: libc::c_int = 0;
static mut sumassumptions: libc::c_int = 0;
static mut redassumptions: libc::c_int = 0;
static mut times: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
static mut sumtimes: libc::c_double = 0.;
static mut assumptions: *mut *mut libc::c_int = 0 as *const *mut libc::c_int
    as *mut *mut libc::c_int;
static mut nvars: libc::c_int = 0;
static mut szvars: libc::c_int = 0;
static mut nclauses: libc::c_int = 0;
static mut nused: libc::c_int = 0;
static mut used: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut vals: *mut libc::c_schar = 0 as *const libc::c_schar as *mut libc::c_schar;
static mut nlits: libc::c_int = 0;
static mut szlits: libc::c_int = 0;
static mut lits: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut verbose: libc::c_int = 0;
static mut bar: libc::c_int = 0;
static mut nowitness: libc::c_int = 0;
static mut plain: libc::c_int = 0;
static mut doclone: libc::c_int = 0;
static mut deterministic: libc::c_int = 0;
static mut noreverse: libc::c_int = 0;
static mut addassumptions: libc::c_int = 1 as libc::c_int;
static mut noflush: libc::c_int = 0;
static mut reduce: libc::c_int = 0;
static mut nomelt: libc::c_int = 0;
static mut druptraceprefix: *const libc::c_char = 0 as *const libc::c_char;
static mut lineno: libc::c_int = 1 as libc::c_int;
static mut inputname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut inputfile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut done: libc::c_int = 0;
static mut msgmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut donemutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut queuemutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut finishedmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut finished: libc::c_int = 0;
unsafe extern "C" fn msglock(mut voidptr: *mut libc::c_void) {
    pthread_mutex_lock(&mut msgmutex);
}
unsafe extern "C" fn msgunlock(mut voidptr: *mut libc::c_void) {
    pthread_mutex_unlock(&mut msgmutex);
}
unsafe extern "C" fn msg(
    mut w: *mut Worker,
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose < level {
        return;
    }
    msglock(0 as *mut libc::c_void);
    if !w.is_null() {
        printf(
            b"c %d \0" as *const u8 as *const libc::c_char,
            w.offset_from(workers) as libc::c_long as libc::c_int,
        );
    } else {
        printf(b"c - \0" as *const u8 as *const libc::c_char);
    }
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    msgunlock(0 as *mut libc::c_void);
}
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fputs(b"*** [ilingeling] \0" as *const u8 as *const libc::c_char, stderr);
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn warn(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fputs(b"*** [ilingeling] warning: \0" as *const u8 as *const libc::c_char, stderr);
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
}
unsafe extern "C" fn currentime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe extern "C" fn getime() -> libc::c_double {
    return currentime() - startime;
}
unsafe extern "C" fn isnum(mut str: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = str;
    let fresh0 = p;
    p = p.offset(1);
    if *(*__ctype_b_loc()).offset(*fresh0 as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
        p;
    }
    return (*p == 0) as libc::c_int;
}
unsafe extern "C" fn term(mut voidptr: *mut libc::c_void) -> libc::c_int {
    let mut w: *mut Worker = voidptr as *mut Worker;
    let mut res: libc::c_int = 0;
    msg(
        w,
        3 as libc::c_int,
        b"checking early termination\0" as *const u8 as *const libc::c_char,
    );
    if pthread_mutex_lock(&mut donemutex) != 0 {
        warn(
            b"failed to lock 'done' mutex in termination check\0" as *const u8
                as *const libc::c_char,
        );
    }
    res = done;
    if pthread_mutex_unlock(&mut donemutex) != 0 {
        warn(
            b"failed to unlock 'done' mutex in termination check\0" as *const u8
                as *const libc::c_char,
        );
    }
    msg(
        w,
        3 as libc::c_int,
        b"early termination check %s\0" as *const u8 as *const libc::c_char,
        if res != 0 {
            b"succeeded\0" as *const u8 as *const libc::c_char
        } else {
            b"failed\0" as *const u8 as *const libc::c_char
        },
    );
    return res;
}
unsafe extern "C" fn progress(
    mut pmille: libc::c_int,
    mut total: libc::c_int,
    mut max: libc::c_int,
    mut avg: libc::c_double,
    mut nl: libc::c_int,
) {
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut eta: libc::c_int = 0;
    let mut fmt: [libc::c_char; 16] = [0; 16];
    let mut rem: libc::c_double = 0.;
    msglock(0 as *mut libc::c_void);
    if isatty(1 as libc::c_int) != 0 {
        fputc('\r' as i32, stdout);
    }
    lim = 10 as libc::c_int;
    i = 1 as libc::c_int;
    while lim < max && i < 11 as libc::c_int {
        lim *= 10 as libc::c_int;
        i += 1;
        i;
    }
    sprintf(fmt.as_mut_ptr(), b"c %%0%dd\0" as *const u8 as *const libc::c_char, i);
    printf(fmt.as_mut_ptr(), total);
    printf(b" / %d |\0" as *const u8 as *const libc::c_char, max);
    i = 0 as libc::c_int;
    while i < pmille / 50 as libc::c_int {
        fputc('=' as i32, stdout);
        i += 1;
        i;
    }
    if total == max {
        ch = '=' as i32;
    } else {
        match pmille % 4 as libc::c_int {
            1 => {
                ch = '\\' as i32;
            }
            2 => {
                ch = '|' as i32;
            }
            3 => {
                ch = '/' as i32;
            }
            _ => {
                ch = '-' as i32;
            }
        }
    }
    let fresh1 = i;
    i = i + 1;
    if fresh1 < 20 as libc::c_int {
        fputc(ch, stdout);
    }
    loop {
        let fresh2 = i;
        i = i + 1;
        if !(fresh2 < 20 as libc::c_int) {
            break;
        }
        fputc('-' as i32, stdout);
    }
    printf(b"| %3d%%\0" as *const u8 as *const libc::c_char, pmille / 10 as libc::c_int);
    printf(b" %.4f sec/cube\0" as *const u8 as *const libc::c_char, avg);
    if max <= total {
        eta = 0 as libc::c_int;
    } else {
        rem = (max - total) as libc::c_double * avg;
        if rem >= (100 as libc::c_int * 3600 as libc::c_int) as libc::c_double {
            eta = 2147483647 as libc::c_int;
        } else {
            eta = rem as libc::c_int;
        }
    }
    if eta < 2147483647 as libc::c_int {
        if eta > 3600 as libc::c_int {
            printf(
                b" %02d:\0" as *const u8 as *const libc::c_char,
                eta / 3600 as libc::c_int,
            );
            eta %= 3600 as libc::c_int;
        } else {
            printf(b"    \0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"%02d:%02d ETS\0" as *const u8 as *const libc::c_char,
            eta / 60 as libc::c_int,
            eta % 60 as libc::c_int,
        );
    } else {
        printf(b"   --:-- ETS\0" as *const u8 as *const libc::c_char);
    }
    if nl != 0 || isatty(1 as libc::c_int) == 0 {
        fputc('\n' as i32, stdout);
    }
    fflush(stdout);
    msgunlock(0 as *mut libc::c_void);
}
unsafe extern "C" fn initlgl(
    mut lgl: *mut LGL,
    mut w: *mut Worker,
    mut opts: libc::c_int,
) {
    lglsetid(lgl, w.offset_from(workers) as libc::c_long as libc::c_int, nworkers);
    lglsetime(
        lgl,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_double>,
            Option::<unsafe extern "C" fn() -> libc::c_double>,
        >(
            Some(
                ::core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_double,
                    unsafe extern "C" fn() -> libc::c_double,
                >(getime),
            ),
        ),
    );
    lglseterm(
        lgl,
        Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        w as *mut libc::c_void,
    );
    lglsetmsglock(
        lgl,
        Some(msglock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(msgunlock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        w as *mut libc::c_void,
    );
    if opts == 0 {
        return;
    }
    if verbose != 0 {
        lglsetopt(
            lgl,
            b"verbose\0" as *const u8 as *const libc::c_char,
            verbose - 1 as libc::c_int,
        );
    }
    if plain != 0 {
        lglsetopt(lgl, b"plain\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    }
    lglsetopt(
        lgl,
        b"reduceinc\0" as *const u8 as *const libc::c_char,
        1000 as libc::c_int,
    );
    lglsetopt(
        lgl,
        b"reduceinit\0" as *const u8 as *const libc::c_char,
        1000 as libc::c_int,
    );
    lglsetopt(
        lgl,
        b"reusetrail\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    lglsetopt(lgl, b"gluekeep\0" as *const u8 as *const libc::c_char, 3 as libc::c_int);
    lglsetopt(lgl, b"scincinc\0" as *const u8 as *const libc::c_char, 50 as libc::c_int);
    lglsetopt(
        lgl,
        b"scincincmode\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if !druptraceprefix.is_null() {
        let mut name: *mut libc::c_char = malloc(
            (strlen(druptraceprefix)).wrapping_add(30 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            name,
            b"%s%d.proof\0" as *const u8 as *const libc::c_char,
            druptraceprefix,
            w.offset_from(workers) as libc::c_long as libc::c_int,
        );
        (*w).proof = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        if ((*w).proof).is_null() {
            die(
                b"worker %d can not write DRUP proof to '%s'\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
                name,
            );
        }
        lglsetout(lgl, (*w).proof);
        sprintf(
            name,
            b"%s%d.cnf\0" as *const u8 as *const libc::c_char,
            druptraceprefix,
            w.offset_from(workers) as libc::c_long as libc::c_int,
        );
        (*w).post = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        if ((*w).post).is_null() {
            die(
                b"worker %d can not write post CNF cubes to '%s'\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
                name,
            );
        }
        free(name as *mut libc::c_void);
        lglsetopt(
            lgl,
            b"druplig\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        lglsetopt(
            lgl,
            b"drupligtrace\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    } else {
        (*w).proof = 0 as *mut FILE;
        (*w).post = (*w).proof;
    };
}
unsafe extern "C" fn justreturn(mut w: *mut Worker) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if pthread_mutex_lock(&mut donemutex) != 0 {
        warn(
            b"worker %d failed to lock 'done' mutex\0" as *const u8
                as *const libc::c_char,
            w.offset_from(workers) as libc::c_long as libc::c_int,
        );
    }
    res = done;
    if pthread_mutex_unlock(&mut donemutex) != 0 {
        warn(
            b"worker %d failed to unlock 'done' mutex\0" as *const u8
                as *const libc::c_char,
            w.offset_from(workers) as libc::c_long as libc::c_int,
        );
    }
    return res;
}
unsafe extern "C" fn sat(mut w: *mut Worker) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut name: [libc::c_char; 100] = [0; 100];
    let mut cloned: *mut LGL = 0 as *mut LGL;
    if druptraceprefix.is_null() && doclone != 0 {
        lglsetopt(
            (*w).lgl,
            b"clim\0" as *const u8 as *const libc::c_char,
            20000 as libc::c_int,
        );
    } else {
        lglsetopt(
            (*w).lgl,
            b"clim\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        lglsetopt(
            (*w).lgl,
            b"plim\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        lglsetopt(
            (*w).lgl,
            b"dlim\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    if noflush == 0 {
        lglflushcache((*w).lgl);
    }
    res = lglsat((*w).lgl);
    if res == 0 && justreturn(w) == 0 {
        msg(
            w,
            1 as libc::c_int,
            b"cloning after %d conflicts\0" as *const u8 as *const libc::c_char,
            20000 as libc::c_int,
        );
        cloned = lglclone((*w).lgl);
        lglfixate(cloned);
        lglmeltall(cloned);
        initlgl(cloned, w, 0 as libc::c_int);
        lglsetopt(
            cloned,
            b"clim\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        let fresh3 = (*w).cloned.count;
        (*w).cloned.count = (*w).cloned.count + 1;
        sprintf(
            name.as_mut_ptr(),
            b"c F%d \0" as *const u8 as *const libc::c_char,
            fresh3,
        );
        lglsetprefix(cloned, name.as_mut_ptr());
        if pthread_mutex_lock(&mut (*w).cloned.lock) != 0 {
            warn(
                b"worker %d failed to lock 'cloned' mutex\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        (*w).cloned.lgl = cloned;
        if pthread_mutex_unlock(&mut (*w).cloned.lock) != 0 {
            warn(
                b"worker %d failed to unlock 'cloned' mutex\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        res = lglsat(cloned);
        if pthread_mutex_lock(&mut (*w).cloned.lock) != 0 {
            warn(
                b"worker %d failed to lock 'cloned' mutex\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        (*w).cloned.lgl = 0 as *mut LGL;
        (*w).cloned.decs += lglgetdecs(cloned);
        (*w).cloned.confs += lglgetconfs(cloned);
        (*w).cloned.props += lglgetprops(cloned);
        (*w).cloned.decs -= lglgetdecs((*w).lgl);
        (*w).cloned.confs -= lglgetconfs((*w).lgl);
        (*w).cloned.props -= lglgetprops((*w).lgl);
        if pthread_mutex_unlock(&mut (*w).cloned.lock) != 0 {
            warn(
                b"worker %d failed to unlock 'cloned' mutex\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        if verbose >= 2 as libc::c_int && !statsfile.is_null() {
            lglsetout(cloned, statsfile);
            lglstats(cloned);
            lglsetout(cloned, stdout);
        }
        msg(
            w,
            1 as libc::c_int,
            b"joining cloned solver\0" as *const u8 as *const libc::c_char,
        );
        lglunclone((*w).lgl, cloned);
    }
    return res;
}
unsafe extern "C" fn work(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut pm: libc::c_int = 0;
    let mut lm: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut size: libc::c_int = 0;
    let mut red: libc::c_int = 0;
    let mut fin: libc::c_int = 0;
    let mut start_0: libc::c_double = 0.;
    let mut end: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut avg: libc::c_double = 0.;
    let mut w: *mut Worker = voidptr as *mut Worker;
    msg(w, 1 as libc::c_int, b"running\0" as *const u8 as *const libc::c_char);
    loop {
        if pthread_mutex_lock(&mut queuemutex) != 0 {
            die(
                b"worker %d failed to lock 'queue' mutex\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        if deterministic != 0
            && queue % nworkers != w.offset_from(workers) as libc::c_long as libc::c_int
        {
            if pthread_mutex_unlock(&mut queuemutex) != 0 {
                die(
                    b"worker %d failed to unlock 'queue' mutex\0" as *const u8
                        as *const libc::c_char,
                    w.offset_from(workers) as libc::c_long as libc::c_int,
                );
            }
            usleep(1000 as libc::c_int as __useconds_t);
        } else {
            last = queue;
            if last < nassumptions {
                queue += 1;
                queue;
            }
            if pthread_mutex_unlock(&mut queuemutex) != 0 {
                die(
                    b"worker %d failed to unlock 'queue' mutex\0" as *const u8
                        as *const libc::c_char,
                    w.offset_from(workers) as libc::c_long as libc::c_int,
                );
            }
            if !(last == nassumptions) {
                msg(
                    w,
                    2 as libc::c_int,
                    b"got job %d\0" as *const u8 as *const libc::c_char,
                    last,
                );
                count = 0 as libc::c_int;
                i = (*w).last + 1 as libc::c_int;
                while i <= last {
                    a = *assumptions.offset(i as isize);
                    if addassumptions > 1 as libc::c_int && i < last {
                        p = a;
                        loop {
                            lit = *p;
                            if !(lit != 0) {
                                break;
                            }
                            lgladd((*w).lgl, -lit);
                            p = p.offset(1);
                            p;
                        }
                        lgladd((*w).lgl, 0 as libc::c_int);
                    }
                    if nomelt == 0 {
                        p = a;
                        loop {
                            lit = *p;
                            if !(lit != 0) {
                                break;
                            }
                            idx = abs(lit);
                            if *used.offset(idx as isize) == i {
                                lglmelt((*w).lgl, idx);
                                count += 1;
                                count;
                            }
                            p = p.offset(1);
                            p;
                        }
                    }
                    i += 1;
                    i;
                }
                msg(
                    w,
                    2 as libc::c_int,
                    b"melted %d variables\0" as *const u8 as *const libc::c_char,
                    count,
                );
                (*w).last = last;
                a = *assumptions.offset((*w).last as isize);
                if noreverse != 0 {
                    p = a;
                    loop {
                        lit = *p;
                        if !(lit != 0) {
                            break;
                        }
                        lglassume((*w).lgl, lit);
                        p = p.offset(1);
                        p;
                    }
                } else {
                    i = 0 as libc::c_int;
                    while *a.offset(i as isize) != 0 {
                        i += 1;
                        i;
                    }
                    p = a.offset(i as isize).offset(-(1 as libc::c_int as isize));
                    while p >= a {
                        lglassume((*w).lgl, *p);
                        p = p.offset(-1);
                        p;
                    }
                }
                start_0 = getime();
                (*w).res = sat(w);
                end = getime();
                delta = end - start_0;
                delta = if delta <= 0 as libc::c_int as libc::c_double {
                    0 as libc::c_int as libc::c_double
                } else {
                    delta
                };
                *times.offset(last as isize) = delta;
                if bar != 0 {
                    pthread_mutex_lock(&mut finishedmutex);
                    finished += 1;
                    fin = finished;
                    sumtimes += delta;
                    avg = sumtimes / fin as libc::c_double;
                    pthread_mutex_unlock(&mut finishedmutex);
                    pm = 1000 as libc::c_int * (fin - 1 as libc::c_int) / nassumptions;
                    lm = 1000 as libc::c_int * fin / nassumptions;
                    if pm < lm {
                        progress(lm, fin, nassumptions, avg, 0 as libc::c_int);
                    }
                }
                if (*w).res == 10 as libc::c_int {
                    if bar == 0 {
                        msg(
                            w,
                            1 as libc::c_int,
                            b"job %d SATISFIABLE\0" as *const u8 as *const libc::c_char,
                            last,
                        );
                    }
                    if pthread_mutex_lock(&mut donemutex) != 0 {
                        warn(
                            b"worker %d failed to lock 'done' mutex\0" as *const u8
                                as *const libc::c_char,
                            w.offset_from(workers) as libc::c_long as libc::c_int,
                        );
                    }
                    done = 1 as libc::c_int;
                    if pthread_mutex_unlock(&mut donemutex) != 0 {
                        warn(
                            b"worker %d failed to unlock 'done' mutex\0" as *const u8
                                as *const libc::c_char,
                            w.offset_from(workers) as libc::c_long as libc::c_int,
                        );
                    }
                } else if (*w).res == 20 as libc::c_int {
                    (*w).nfailed = 0 as libc::c_int;
                    if ((*w).failed).is_null() {
                        let mut BYTES: size_t = (maxassumptionsize as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                        (*w).failed = malloc(BYTES) as *mut libc::c_int;
                        if ((*w).failed).is_null() {
                            die(b"out of memory\0" as *const u8 as *const libc::c_char);
                            exit(1 as libc::c_int);
                        }
                        memset(
                            (*w).failed as *mut libc::c_void,
                            0 as libc::c_int,
                            BYTES,
                        );
                        allocated = allocated.wrapping_add(BYTES);
                        if allocated > maxallocated {
                            maxallocated = allocated;
                        }
                    }
                    p = a;
                    loop {
                        lit = *p;
                        if !(lit != 0) {
                            break;
                        }
                        if lglfailed((*w).lgl, lit) != 0 {
                            let fresh4 = (*w).nfailed;
                            (*w).nfailed = (*w).nfailed + 1;
                            *((*w).failed).offset(fresh4 as isize) = lit;
                        }
                        p = p.offset(1);
                        p;
                    }
                    if !druptraceprefix.is_null() {
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            fprintf(
                                (*w).proof,
                                b"%d \0" as *const u8 as *const libc::c_char,
                                -*((*w).failed).offset(i as isize),
                            );
                            i += 1;
                            i;
                        }
                        fputs(b"0\n\0" as *const u8 as *const libc::c_char, (*w).proof);
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            fprintf(
                                (*w).proof,
                                b"%d \0" as *const u8 as *const libc::c_char,
                                -*((*w).failed).offset(i as isize),
                            );
                            i += 1;
                            i;
                        }
                        fputs(b"0\n\0" as *const u8 as *const libc::c_char, (*w).proof);
                    }
                    if addassumptions != 0 {
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            lgladd((*w).lgl, -*((*w).failed).offset(i as isize));
                            i += 1;
                            i;
                        }
                        lgladd((*w).lgl, 0 as libc::c_int);
                    }
                    if !druptraceprefix.is_null() {
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            fprintf(
                                (*w).post,
                                b"%d \0" as *const u8 as *const libc::c_char,
                                -*((*w).failed).offset(i as isize),
                            );
                            i += 1;
                            i;
                        }
                        fputs(b"0\n\0" as *const u8 as *const libc::c_char, (*w).post);
                    }
                    red = (*w).nfailed;
                    size = p.offset_from(a) as libc::c_long as libc::c_int;
                    sumassumptions += size;
                    redassumptions += red;
                    if bar == 0 {
                        msg(
                            w,
                            1 as libc::c_int,
                            b"job %d UNSATISFIABLE (%d failed / %d) in %.3f seconds\0"
                                as *const u8 as *const libc::c_char,
                            last,
                            red,
                            size,
                            delta,
                        );
                    }
                    if red == 0 && deterministic == 0 {
                        if bar == 0 {
                            msg(
                                w,
                                1 as libc::c_int,
                                b"job %d ACTUALLY FOUND EMPTY CLAUSE\0" as *const u8
                                    as *const libc::c_char,
                                last,
                            );
                        }
                        if pthread_mutex_lock(&mut donemutex) != 0 {
                            warn(
                                b"worker %d failed to lock 'done' mutex\0" as *const u8
                                    as *const libc::c_char,
                                w.offset_from(workers) as libc::c_long as libc::c_int,
                            );
                        }
                        done = 1 as libc::c_int;
                        if pthread_mutex_unlock(&mut donemutex) != 0 {
                            warn(
                                b"worker %d failed to unlock 'done' mutex\0" as *const u8
                                    as *const libc::c_char,
                                w.offset_from(workers) as libc::c_long as libc::c_int,
                            );
                        }
                    } else {
                        if reduce != 0 {
                            lglreducecache((*w).lgl);
                        }
                        continue;
                    }
                } else if bar == 0 {
                    msg(
                        w,
                        1 as libc::c_int,
                        b"job %d UNKNOWN\0" as *const u8 as *const libc::c_char,
                        last,
                    );
                }
            }
            if bar == 0 {
                msg(w, 1 as libc::c_int, b"done\0" as *const u8 as *const libc::c_char);
            }
            return 0 as *mut libc::c_void;
        }
    };
}
unsafe extern "C" fn init() {
    let mut w: *mut Worker = 0 as *mut Worker;
    let mut BYTES: size_t = (nworkers as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
    workers = malloc(BYTES) as *mut Worker;
    if workers.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(workers as *mut libc::c_void, 0 as libc::c_int, BYTES);
    allocated = allocated.wrapping_add(BYTES);
    if allocated > maxallocated {
        maxallocated = allocated;
    }
    w = workers;
    while w < workers.offset(nworkers as isize) {
        (*w).last = -(1 as libc::c_int);
        (*w).lgl = lglinit();
        pthread_mutex_init(&mut (*w).cloned.lock, 0 as *const pthread_mutexattr_t);
        initlgl((*w).lgl, w, 1 as libc::c_int);
        w = w.offset(1);
        w;
    }
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"allocated %d workers\0" as *const u8 as *const libc::c_char,
        nworkers,
    );
}
unsafe extern "C" fn reset() {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    if !vals.is_null() {
        let mut BYTES: size_t = (nvars as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong);
        allocated = allocated.wrapping_sub(BYTES);
        free(vals as *mut libc::c_void);
        vals = 0 as *mut libc::c_schar;
    }
    i = 0 as libc::c_int;
    while i < nworkers {
        let mut w: *mut Worker = workers.offset(i as isize);
        lglrelease((*w).lgl);
        if !((*w).proof).is_null() {
            (*w).proof = 0 as *mut FILE;
            fclose((*w).post);
            (*w).post = 0 as *mut FILE;
        }
        if !((*w).failed).is_null() {
            let mut BYTES_0: size_t = (maxassumptionsize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
            allocated = allocated.wrapping_sub(BYTES_0);
            free((*w).failed as *mut libc::c_void);
            (*w).failed = 0 as *mut libc::c_int;
        }
        i += 1;
        i;
    }
    let mut BYTES_1: size_t = (nworkers as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_1);
    free(workers as *mut libc::c_void);
    workers = 0 as *mut Worker;
    i = 0 as libc::c_int;
    while i < nassumptions {
        a = *assumptions.offset(i as isize);
        p = a;
        while *p != 0 {
            p = p.offset(1);
            p;
        }
        let mut BYTES_2: size_t = ((p.offset_from(a) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        allocated = allocated.wrapping_sub(BYTES_2);
        free(a as *mut libc::c_void);
        a = 0 as *mut libc::c_int;
        i += 1;
        i;
    }
    let mut BYTES_3: size_t = (szassumptions as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_3);
    free(assumptions as *mut libc::c_void);
    assumptions = 0 as *mut *mut libc::c_int;
    let mut BYTES_4: size_t = (nassumptions as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_4);
    free(times as *mut libc::c_void);
    times = 0 as *mut libc::c_double;
    let mut BYTES_5: size_t = (szlits as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_5);
    free(lits as *mut libc::c_void);
    lits = 0 as *mut libc::c_int;
    let mut BYTES_6: size_t = (szvars as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_6);
    free(used as *mut libc::c_void);
    used = 0 as *mut libc::c_int;
    if allocated != 0 {
        warn(
            b"internal memory leak of %lld bytes\0" as *const u8 as *const libc::c_char,
            allocated as libc::c_longlong,
        );
    }
}
unsafe extern "C" fn perr(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(stderr, b"%s:%d: \0" as *const u8 as *const libc::c_char, inputname, lineno);
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn next() -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = getc(inputfile);
    if res == '\n' as i32 {
        lineno += 1;
        lineno;
    }
    return res;
}
unsafe extern "C" fn add(mut lit: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nworkers {
        lgladd((*workers.offset(i as isize)).lgl, lit);
        i += 1;
        i;
    }
}
unsafe extern "C" fn parse() {
    let mut ch: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut assumption: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    loop {
        ch = next();
        if ch == -(1 as libc::c_int) {
            perr(
                b"unexpected end-of-file in header\0" as *const u8 as *const libc::c_char,
            );
        }
        if !(ch == 'c' as i32) {
            break;
        }
        loop {
            ch = next();
            if !(ch != '\n' as i32) {
                break;
            }
            if ch == -(1 as libc::c_int) {
                perr(
                    b"unexpected end-of-file in header comment\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if ch != 'p' as i32 || next() != ' ' as i32 || next() != 'i' as i32
        || next() != 'n' as i32 || next() != 'c' as i32 || next() != 'c' as i32
        || next() != 'n' as i32 || next() != 'f' as i32
    {
        perr(
            b"invalid header (expected 'p inccnf')\0" as *const u8 as *const libc::c_char,
        );
    }
    ch = next();
    '_CLAUSES: loop {
        if ch == ' ' as i32 || ch == '\t' as i32 || ch == '\r' as i32
            || ch == '\n' as i32
        {
            ch = next();
        } else if ch == 'c' as i32 {
            loop {
                ch = next();
                if !(ch != '\n' as i32) {
                    break;
                }
                if ch == -(1 as libc::c_int) {
                    perr(
                        b"unexpected end-of-file in body comment\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        } else {
            if ch == -(1 as libc::c_int) && nlits != 0 {
                perr(
                    b"unexpected end-of-file in clause\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if ch == 'a' as i32 && nlits != 0 {
                perr(b"unexpected 'a' in clause\0" as *const u8 as *const libc::c_char);
            }
            if !(ch == 'a' as i32) {
                if ch == -(1 as libc::c_int) {
                    break;
                }
                if !(ch == 'a' as i32) {
                    if ch == '-' as i32 {
                        sign = -(1 as libc::c_int);
                        ch = next();
                        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            perr(
                                b"expected digit after '-'\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else {
                        sign = 1 as libc::c_int;
                    }
                    if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        perr(b"expected literal\0" as *const u8 as *const libc::c_char);
                    }
                    lit = ch - '0' as i32;
                    loop {
                        ch = next();
                        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                        lit = 10 as libc::c_int * lit + ch - '0' as i32;
                    }
                    if lit > nvars {
                        if lit >= szvars {
                            let mut oldszvars: libc::c_int = szvars;
                            szvars = if szvars != 0 {
                                2 as libc::c_int * szvars
                            } else {
                                1 as libc::c_int
                            };
                            while szvars <= lit {
                                szvars *= 2 as libc::c_int;
                            }
                            let mut OBYTES: size_t = (oldszvars as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                );
                            let mut NBYTES: size_t = (szvars as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                );
                            allocated = allocated.wrapping_sub(OBYTES);
                            used = realloc(used as *mut libc::c_void, NBYTES)
                                as *mut libc::c_int;
                            if used.is_null() {
                                die(b"out of memory\0" as *const u8 as *const libc::c_char);
                            }
                            allocated = allocated.wrapping_add(NBYTES);
                            if allocated > maxallocated {
                                maxallocated = allocated;
                            }
                            i = oldszvars;
                            while i < szvars {
                                *used.offset(i as isize) = -(1 as libc::c_int);
                                i += 1;
                                i;
                            }
                        }
                        nvars = lit;
                    }
                    lit *= sign;
                    if lit != 0 {
                        nlits += 1;
                        nlits;
                    } else {
                        nlits = 0 as libc::c_int;
                        nclauses += 1;
                        nclauses;
                    }
                    add(lit);
                    continue;
                }
            }
            loop {
                ch = next();
                if ch != ' ' as i32 {
                    perr(
                        b"expected space after 'a'\0" as *const u8 as *const libc::c_char,
                    );
                }
                loop {
                    ch = next();
                    if ch == ' ' as i32 || ch == '\t' as i32 || ch == '\r' as i32
                        || ch == '\n' as i32
                    {
                        continue;
                    }
                    if ch == -(1 as libc::c_int) && nlits != 0 {
                        perr(
                            b"unexpected end-of-file in assumptions\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if ch == '-' as i32 {
                        sign = -(1 as libc::c_int);
                        ch = next();
                        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            perr(
                                b"expected digit after '-'\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else {
                        sign = 1 as libc::c_int;
                    }
                    if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        perr(b"expected literal\0" as *const u8 as *const libc::c_char);
                    }
                    lit = ch - '0' as i32;
                    loop {
                        ch = next();
                        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                        lit = 10 as libc::c_int * lit + ch - '0' as i32;
                    }
                    if lit > nvars {
                        perr(
                            b"assumption %d exceeds maximum variables %d\0" as *const u8
                                as *const libc::c_char,
                            lit,
                            nvars,
                        );
                    }
                    if *used.offset(lit as isize) < 0 as libc::c_int {
                        nused += 1;
                        nused;
                    }
                    *used.offset(lit as isize) = nassumptions;
                    lit *= sign;
                    if ch != ' ' as i32 && ch != '\t' as i32 && ch != '\r' as i32
                        && ch != '\n' as i32
                    {
                        perr(
                            b"expected white space after '%l'\0" as *const u8
                                as *const libc::c_char,
                            lit,
                        );
                    }
                    if !(lit != 0) {
                        break;
                    }
                    if nlits >= szlits {
                        let mut oldbytes: size_t = 0;
                        let mut newbytes: size_t = 0;
                        oldbytes = (szlits as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                        allocated = allocated.wrapping_sub(oldbytes);
                        szlits = if szlits != 0 {
                            2 as libc::c_int * szlits
                        } else {
                            1 as libc::c_int
                        };
                        newbytes = (szlits as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                        lits = realloc(lits as *mut libc::c_void, newbytes)
                            as *mut libc::c_int;
                        if lits.is_null() {
                            die(b"out of memory\0" as *const u8 as *const libc::c_char);
                        }
                        allocated = allocated.wrapping_add(newbytes);
                        if allocated > maxallocated {
                            maxallocated = allocated;
                        }
                    }
                    let fresh5 = nlits;
                    nlits = nlits + 1;
                    *lits.offset(fresh5 as isize) = lit;
                }
                let mut BYTES: size_t = ((nlits + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                assumption = malloc(BYTES) as *mut libc::c_int;
                if assumption.is_null() {
                    die(b"out of memory\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                memset(assumption as *mut libc::c_void, 0 as libc::c_int, BYTES);
                allocated = allocated.wrapping_add(BYTES);
                if allocated > maxallocated {
                    maxallocated = allocated;
                }
                i = 0 as libc::c_int;
                while i < nlits {
                    *assumption.offset(i as isize) = *lits.offset(i as isize);
                    i += 1;
                    i;
                }
                if nassumptions >= szassumptions {
                    let mut oldbytes_0: size_t = 0;
                    let mut newbytes_0: size_t = 0;
                    oldbytes_0 = (szassumptions as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
                        );
                    allocated = allocated.wrapping_sub(oldbytes_0);
                    szassumptions = if szassumptions != 0 {
                        2 as libc::c_int * szassumptions
                    } else {
                        1 as libc::c_int
                    };
                    newbytes_0 = (szassumptions as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
                        );
                    assumptions = realloc(assumptions as *mut libc::c_void, newbytes_0)
                        as *mut *mut libc::c_int;
                    if assumptions.is_null() {
                        die(b"out of memory\0" as *const u8 as *const libc::c_char);
                    }
                    allocated = allocated.wrapping_add(newbytes_0);
                    if allocated > maxallocated {
                        maxallocated = allocated;
                    }
                }
                let fresh6 = nassumptions;
                nassumptions = nassumptions + 1;
                let ref mut fresh7 = *assumptions.offset(fresh6 as isize);
                *fresh7 = assumption;
                if nlits > maxassumptionsize {
                    maxassumptionsize = nlits;
                }
                nlits = 0 as libc::c_int;
                loop {
                    ch = next();
                    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\r' as i32
                        || ch == '\n' as i32)
                    {
                        break;
                    }
                }
                if ch == -(1 as libc::c_int) {
                    break '_CLAUSES;
                }
                if ch == '-' as i32
                    || *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    break;
                }
                if ch != 'a' as i32 {
                    perr(
                        b"expected literal, 'a' or end-of-file\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
    }
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"maximum variable %d in %d clauses\0" as *const u8 as *const libc::c_char,
        nvars,
        nclauses,
    );
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"parsed %d assumptions\0" as *const u8 as *const libc::c_char,
        nassumptions,
    );
    nvars += 1;
    nvars;
    let mut BYTES_0: size_t = (nassumptions as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong);
    times = malloc(BYTES_0) as *mut libc::c_double;
    if times.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(times as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
    allocated = allocated.wrapping_add(BYTES_0);
    if allocated > maxallocated {
        maxallocated = allocated;
    }
    i = 0 as libc::c_int;
    while i < nassumptions {
        *times.offset(i as isize) = -(1 as libc::c_int) as libc::c_double;
        i += 1;
        i;
    }
}
unsafe extern "C" fn freeze() {
    let mut idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    idx = 1 as libc::c_int;
    while idx < nvars {
        if *used.offset(idx as isize) >= 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < nworkers {
                lglfreeze((*workers.offset(i as isize)).lgl, idx);
                i += 1;
                i;
            }
        }
        idx += 1;
        idx;
    }
}
unsafe extern "C" fn start() {
    let mut w: *mut Worker = 0 as *mut Worker;
    w = workers;
    while w < workers.offset(nworkers as isize) {
        if pthread_create(
            &mut (*w).thread,
            0 as *const pthread_attr_t,
            Some(work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            w as *mut libc::c_void,
        ) != 0
        {
            die(
                b"failed to create worker thread %d\0" as *const u8
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        w = w.offset(1);
        w;
    }
}
unsafe extern "C" fn stop() {
    let mut w: *mut Worker = 0 as *mut Worker;
    let mut avg: libc::c_double = 0.;
    w = workers;
    while w < workers.offset(nworkers as isize) {
        if pthread_join((*w).thread, 0 as *mut *mut libc::c_void) != 0 {
            die(
                b"failed to join worker %d\0" as *const u8 as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
            );
        }
        w = w.offset(1);
        w;
    }
    if bar != 0 {
        avg = if finished != 0 { sumtimes / finished as libc::c_double } else { 0.0f64 };
        progress(
            1000 as libc::c_int * finished / nassumptions,
            finished,
            nassumptions,
            avg,
            1 as libc::c_int,
        );
    }
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"joined all %d workers\0" as *const u8 as *const libc::c_char,
        nworkers,
    );
}
unsafe extern "C" fn statsps(
    mut file: *mut FILE,
    mut name: *const libc::c_char,
    mut stats_0: libc::c_longlong,
    mut time: libc::c_double,
) {
    let mut scale: *const libc::c_char = 0 as *const libc::c_char;
    if stats_0 > 10000000 as libc::c_int as libc::c_longlong {
        scale = b" million\0" as *const u8 as *const libc::c_char;
        stats_0 /= 1000000 as libc::c_int as libc::c_longlong;
    } else if stats_0 > 10000 as libc::c_int as libc::c_longlong {
        scale = b" thousand\0" as *const u8 as *const libc::c_char;
        stats_0 /= 1000 as libc::c_int as libc::c_longlong;
    } else {
        scale = b"\0" as *const u8 as *const libc::c_char;
    }
    fprintf(
        file,
        b"c %lld%s %s, %.1f%s per second\n\0" as *const u8 as *const libc::c_char,
        stats_0,
        scale,
        name,
        if time > 0 as libc::c_int as libc::c_double {
            stats_0 as libc::c_double / time
        } else {
            0.0f64
        },
        scale,
    );
}
unsafe extern "C" fn cmpdblptr(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut a: libc::c_double = *(p as *mut libc::c_double);
    let mut b: libc::c_double = *(q as *mut libc::c_double);
    if a < b {
        return -(1 as libc::c_int);
    }
    if a > b {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats() {
    let mut decs: int64_t = 0 as libc::c_int as int64_t;
    let mut confs: int64_t = 0 as libc::c_int as int64_t;
    let mut props: int64_t = 0 as libc::c_int as int64_t;
    let mut mb: libc::c_double = maxallocated as libc::c_double
        / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double;
    let mut wct: libc::c_double = getime();
    let mut prt: libc::c_double = lglprocesstime();
    let mut file: *mut FILE = if !statsfile.is_null() { statsfile } else { stdout };
    let mut sum: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut avg: libc::c_double = 0.;
    let mut std: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut cloned: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nworkers {
        lglflushtimers((*workers.offset(i as isize)).lgl);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nworkers {
        lglflushtimers((*workers.offset(i as isize)).lgl);
        fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            file,
            b"c ---------[worker %d stats]------------------\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
        fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
        lglsetout((*workers.offset(i as isize)).lgl, file);
        lglstats((*workers.offset(i as isize)).lgl);
        lglsetout((*workers.offset(i as isize)).lgl, stdout);
        mb += lglmb((*workers.offset(i as isize)).lgl);
        decs += lglgetdecs((*workers.offset(i as isize)).lgl);
        confs += lglgetconfs((*workers.offset(i as isize)).lgl);
        props += lglgetprops((*workers.offset(i as isize)).lgl);
        if pthread_mutex_lock(&mut (*workers.offset(i as isize)).cloned.lock) != 0 {
            warn(
                b"worker failed to lock 'cloned' mutex\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !((*workers.offset(i as isize)).cloned.lgl).is_null() {
            fprintf(
                file,
                b"c ---------[cloned worker %d dstats]----------\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
            lglsetout((*workers.offset(i as isize)).cloned.lgl, file);
            lglstats((*workers.offset(i as isize)).cloned.lgl);
            lglsetout((*workers.offset(i as isize)).cloned.lgl, stdout);
            mb += lglmb((*workers.offset(i as isize)).cloned.lgl);
            decs += lglgetdecs((*workers.offset(i as isize)).cloned.lgl);
            confs += lglgetconfs((*workers.offset(i as isize)).cloned.lgl);
            props += lglgetprops((*workers.offset(i as isize)).cloned.lgl);
            mb -= lglmb((*workers.offset(i as isize)).lgl);
            decs -= lglgetdecs((*workers.offset(i as isize)).lgl);
            confs -= lglgetconfs((*workers.offset(i as isize)).lgl);
            props -= lglgetprops((*workers.offset(i as isize)).lgl);
        }
        decs += (*workers.offset(i as isize)).cloned.decs;
        confs += (*workers.offset(i as isize)).cloned.confs;
        props += (*workers.offset(i as isize)).cloned.props;
        cloned += (*workers.offset(i as isize)).cloned.count;
        if pthread_mutex_unlock(&mut (*workers.offset(i as isize)).cloned.lock) != 0 {
            warn(
                b"worker failed to lock 'cloned' mutex\0" as *const u8
                    as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        file,
        b"c ---------[global-stats]-------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    statsps(
        file,
        b"scheduled jobs\0" as *const u8 as *const libc::c_char,
        queue as libc::c_longlong,
        wct,
    );
    fprintf(
        file,
        b"c %d failed assumptions %.0f%% out of %d\n\0" as *const u8
            as *const libc::c_char,
        redassumptions,
        if sumassumptions != 0 {
            100.0f64 * redassumptions as libc::c_double
                / sumassumptions as libc::c_double
        } else {
            0 as libc::c_int as libc::c_double
        },
        sumassumptions,
    );
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    fprintf(file, b"c %d cloned\n\0" as *const u8 as *const libc::c_char, cloned);
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    statsps(
        file,
        b"conflicts\0" as *const u8 as *const libc::c_char,
        confs as libc::c_longlong,
        wct,
    );
    statsps(
        file,
        b"decisions\0" as *const u8 as *const libc::c_char,
        decs as libc::c_longlong,
        wct,
    );
    statsps(
        file,
        b"propagations\0" as *const u8 as *const libc::c_char,
        props as libc::c_longlong,
        wct,
    );
    fprintf(
        file,
        b"c wall clock time %.1f seconds\n\0" as *const u8 as *const libc::c_char,
        wct,
    );
    fprintf(
        file,
        b"c process time %.1f seconds\n\0" as *const u8 as *const libc::c_char,
        prt,
    );
    fprintf(
        file,
        b"c utilization %.0f%%\n\0" as *const u8 as *const libc::c_char,
        if wct > 0 as libc::c_int as libc::c_double {
            100.0f64 * (prt / wct / nworkers as libc::c_double)
        } else {
            0.0f64
        },
    );
    fflush(file);
    n = 0 as libc::c_int;
    sum = 0 as libc::c_int as libc::c_double;
    min = -(1 as libc::c_int) as libc::c_double;
    max = min;
    i = 0 as libc::c_int;
    while i < nassumptions {
        t = *times.offset(i as isize);
        if !(t < 0 as libc::c_int as libc::c_double) {
            sum += t;
            let fresh8 = n;
            n = n + 1;
            *times.offset(fresh8 as isize) = t;
            if min < 0 as libc::c_int as libc::c_double || t < min {
                min = t;
            }
            if max < 0 as libc::c_int as libc::c_double || t > max {
                max = t;
            }
        }
        i += 1;
        i;
    }
    if n != 0 {
        fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
        avg = sum / n as libc::c_double;
        fprintf(
            file,
            b"c %d finished jobs in average time %.3f\n\0" as *const u8
                as *const libc::c_char,
            n,
            avg,
        );
        fprintf(
            file,
            b"c time: sum %.3f, min %.3f, max %.3f\n\0" as *const u8
                as *const libc::c_char,
            sum,
            min,
            max,
        );
        std = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < n {
            t = *times.offset(i as isize) - avg;
            std += t * t;
            i += 1;
            i;
        }
        std = sqrt(std);
        qsort(
            times as *mut libc::c_void,
            n as size_t,
            ::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
            Some(
                cmpdblptr
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        fprintf(
            file,
            b"c time: median %.3f, std dve %.3f\n\0" as *const u8 as *const libc::c_char,
            *times.offset((n / 2 as libc::c_int) as isize),
            std,
        );
    }
    fflush(file);
}
unsafe extern "C" fn hist() {
    let mut file: *mut FILE = if !histfile.is_null() { histfile } else { stdout };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nassumptions {
        fprintf(
            file,
            b"%.3f\n\0" as *const u8 as *const libc::c_char,
            *times.offset(i as isize),
        );
        i += 1;
        i;
    }
    fflush(file);
}
static mut catchedsig: libc::c_int = 0;
static mut sig_int_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_segv_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_abrt_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_term_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn resetsighandlers() {
    signal(2 as libc::c_int, sig_int_handler);
    signal(11 as libc::c_int, sig_segv_handler);
    signal(6 as libc::c_int, sig_abrt_handler);
    signal(15 as libc::c_int, sig_term_handler);
}
unsafe extern "C" fn caughtsigmsg(mut sig: libc::c_int) {
    if verbose == 0 {
        return;
    }
    printf(b"c\nc CAUGHT SIGNAL %d\nc\n\0" as *const u8 as *const libc::c_char, sig);
    fflush(stdout);
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if catchedsig == 0 {
        fputs(b"s UNKNOWN\n\0" as *const u8 as *const libc::c_char, stdout);
        fflush(stdout);
        catchedsig = 1 as libc::c_int;
        caughtsigmsg(sig);
        if !statsfile.is_null() {
            stats();
        }
        if !histfile.is_null() {
            hist();
        }
        if !statsfile.is_null() || !histfile.is_null() {
            caughtsigmsg(sig);
        }
    }
    resetsighandlers();
    if (getenv(b"LGLNABORT\0" as *const u8 as *const libc::c_char)).is_null() {
        raise(sig);
    } else {
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn setsighandlers() {
    sig_int_handler = signal(
        2 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_segv_handler = signal(
        11 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_abrt_handler = signal(
        6 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_term_handler = signal(
        15 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut statsfilename: *const libc::c_char = 0 as *const libc::c_char;
    let mut histfilename: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut closeinputfile: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut winner: *mut Worker = 0 as *mut Worker;
    let mut w: *mut Worker = 0 as *mut Worker;
    startime = currentime();
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize), b"-h\0" as *const u8 as *const libc::c_char)
            == 0
        {
            printf(
                b"usage: ilingeling [<option> ...][<inccnf>][<nworkers>]\n\nwhere <option> is one of the following:\n\n  -h  print this command line option summary\n\n  -v  increase verbose level\n  -q  do not print 'c job ...' lines (requires verbosity < 2)\n  -b  progress bar (implies '-q')\n\n  --version\n\n  -s  <stats> output statistics to separate file\n  -t  <hist> output job run time histogram to separate file\n\n  --clone       use cloning for hard cubes\n  --reduce      reduce learned clause cache after each job\n  --no-melt     do not melt unused assumption variables\n  --no-flush    do not flush learned clause cache before every job\n  --no-reverse  do not reverse assumptions\n\n  --deterministic | --det\n\n            jobs are mapped deterministically to workers\n\n  --no-add    do not add failed assumptions as don't care\n  -A  add all assumptions as don't care\n\n  <inccnf>    'p inccnf' + '<lit*> 0' clauses + 'a <lit>* 0' assumptions\n  <nworkers>  number of workers defaults to 1\n\n  -d|--drup   <path-prefix-for-traces>\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
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
            b"-b\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            bar = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-n\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nowitness = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-reverse\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            noreverse = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-add\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            addassumptions = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-melt\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nomelt = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--reduce\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            reduce = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--deterministic\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--det\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            deterministic = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-A\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            addassumptions = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-p\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            plain = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-s\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !statsfilename.is_null() {
                die(b"two '-s' options\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                die(b"argument to '-s' missing\0" as *const u8 as *const libc::c_char);
            }
            statsfilename = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-t\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !histfilename.is_null() {
                die(b"two '-t' options\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                die(b"argument to '-t' missing\0" as *const u8 as *const libc::c_char);
            }
            histfilename = *argv.offset(i as isize);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--clone\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            doclone = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-flush\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            noflush = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--drup\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            if i == argc {
                die(
                    b"argument to '%s' missing\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            if !druptraceprefix.is_null() {
                die(b"DRUP path prefix set twice\0" as *const u8 as *const libc::c_char);
            }
            druptraceprefix = *argv.offset(i as isize);
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
        {
            die(
                b"invalid option '%s'\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else if isnum(*argv.offset(i as isize)) != 0 {
            if nworkers != 0 {
                die(
                    b"number of workers specified twice: '%d' and '%s'\0" as *const u8
                        as *const libc::c_char,
                    nworkers,
                    *argv.offset(i as isize),
                );
            }
            nworkers = atoi(*argv.offset(i as isize));
            if nworkers <= 0 as libc::c_int {
                die(
                    b"invalid number of workers argument: '%s'\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if !inputname.is_null() {
            die(
                b"two files given: '%s' and '%s'\0" as *const u8 as *const libc::c_char,
                inputname,
                *argv.offset(i as isize),
            );
        } else {
            inputname = *argv.offset(i as isize);
        }
        i += 1;
        i;
    }
    if bar != 0 && isatty(1 as libc::c_int) == 0 {
        die(
            b"progress bar requested but <stdout> not connected to terminal\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if verbose >= 2 as libc::c_int && bar != 0 {
        die(
            b"verbosity %d > 1 with '-b'\0" as *const u8 as *const libc::c_char,
            verbose,
        );
    }
    if !statsfilename.is_null()
        && {
            statsfile = fopen(statsfilename, b"w\0" as *const u8 as *const libc::c_char);
            statsfile.is_null()
        }
    {
        die(
            b"can not write to stats file '%s'\0" as *const u8 as *const libc::c_char,
            statsfilename,
        );
    }
    if !histfilename.is_null()
        && {
            histfile = fopen(histfilename, b"w\0" as *const u8 as *const libc::c_char);
            histfile.is_null()
        }
    {
        die(
            b"can not write to job run time histogram file '%s'\0" as *const u8
                as *const libc::c_char,
            histfilename,
        );
    }
    if verbose != 0 && statsfile.is_null() {
        statsfile = stdout;
    }
    if verbose != 0 {
        lglbnr(
            b"iLingeling Incremental Parallel Lingeling\0" as *const u8
                as *const libc::c_char,
            b"c \0" as *const u8 as *const libc::c_char,
            stdout,
        );
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
    if nworkers == 0 {
        nworkers = 1 as libc::c_int;
    }
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"using %d workers\0" as *const u8 as *const libc::c_char,
        nworkers,
    );
    if !inputname.is_null() {
        inputfile = fopen(inputname, b"r\0" as *const u8 as *const libc::c_char);
        if inputfile.is_null() {
            die(b"can not read '%s'\0" as *const u8 as *const libc::c_char, inputname);
        }
        closeinputfile = 1 as libc::c_int;
    } else {
        inputname = b"<stdin>\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        inputfile = stdin;
        closeinputfile = 0 as libc::c_int;
    }
    init();
    setsighandlers();
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"parsing %s\0" as *const u8 as *const libc::c_char,
        inputname,
    );
    parse();
    if closeinputfile != 0 {
        fclose(inputfile);
    }
    msg(
        0 as *mut Worker,
        1 as libc::c_int,
        b"%d variables out of %d used in assumptions which is %.0f%%\0" as *const u8
            as *const libc::c_char,
        nused,
        nvars,
        if nvars != 0 {
            100.0f64 * (nused as libc::c_double / nvars as libc::c_double)
        } else {
            0.0f64
        },
    );
    freeze();
    start();
    stop();
    winner = 0 as *mut Worker;
    w = workers;
    while w < workers.offset(nworkers as isize) {
        if (*w).res != 0 {
            winner = w;
            if (*w).res == 10 as libc::c_int {
                break;
            }
        }
        w = w.offset(1);
        w;
    }
    if !winner.is_null()
        && {
            res = (*winner).res;
            res == 10 as libc::c_int
        } && nowitness == 0
    {
        let mut BYTES: size_t = (nvars as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong);
        vals = malloc(BYTES) as *mut libc::c_schar;
        if vals.is_null() {
            die(b"out of memory\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        memset(vals as *mut libc::c_void, 0 as libc::c_int, BYTES);
        allocated = allocated.wrapping_add(BYTES);
        if allocated > maxallocated {
            maxallocated = allocated;
        }
        i = 1 as libc::c_int;
        while i < nvars {
            *vals.offset(i as isize) = lglderef((*winner).lgl, i) as libc::c_schar;
            i += 1;
            i;
        }
    }
    resetsighandlers();
    if !statsfile.is_null() {
        stats();
    }
    if !statsfilename.is_null() {
        fclose(statsfile);
    }
    if !histfile.is_null() {
        hist();
    }
    if !histfile.is_null() {
        fclose(histfile);
    }
    if res == 10 as libc::c_int {
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
    } else if res == 20 as libc::c_int {
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"s UNKNOWN\n\0" as *const u8 as *const libc::c_char);
    }
    fflush(stdout);
    if !vals.is_null() {
        i = 1 as libc::c_int;
        while i < nvars {
            fputs(b"v \0" as *const u8 as *const libc::c_char, stdout);
            if (*vals.offset(i as isize) as libc::c_int) < 0 as libc::c_int {
                fputc('-' as i32, stdout);
            }
            printf(b"%d\n\0" as *const u8 as *const libc::c_char, i);
            i += 1;
            i;
        }
        fputs(b"v 0\n\0" as *const u8 as *const libc::c_char, stdout);
        fflush(stdout);
    }
    reset();
    return res;
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
