#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglusage(_: *mut LGL);
    fn lglversion() -> *const libc::c_char;
    fn lglseterm(
        _: *mut LGL,
        term: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglsec(_: *mut LGL) -> libc::c_double;
    fn lglstats(_: *mut LGL);
    fn lglflushtimers(lgl: *mut LGL);
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn lglbnr(name: *const libc::c_char, prefix: *const libc::c_char, file: *mut FILE);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn lglparsefile(
        _: *mut LGL,
        _: *mut FILE,
        force_0: libc::c_int,
        lineno_ptr: *mut libc::c_int,
        max_var_ptr: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn lglparsepath(
        _: *mut LGL,
        path: *const libc::c_char,
        force_0: libc::c_int,
        lineno_ptr: *mut libc::c_int,
        max_var_ptr: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglopts(_: *mut LGL, prefix: *const libc::c_char, _: libc::c_int);
    fn lglrgopts(_: *mut LGL);
    fn lglpcs(_: *mut LGL, mixed: libc::c_int);
    fn lglsizes(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglreadopts(_: *mut LGL, _: *mut FILE) -> libc::c_int;
    fn lglgetopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglhasopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglctrav(
        _: *mut LGL,
        state: *mut libc::c_void,
        trav: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    );
    fn lglprint(_: *mut LGL, _: *mut FILE);
    fn lglassume(_: *mut LGL, lit: libc::c_int);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OBuf {
    pub line: [libc::c_char; 81],
    pub pos: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    strtol(
        __nptr,
        std::ptr::null_mut::<libc::c_void>() as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int
}
static mut lgl4sigh: *mut LGL = 0 as *const LGL as *mut LGL;
static mut catchedsig: libc::c_int = 0;
static mut verbose: libc::c_int = 0;
static mut force: libc::c_int = 0;
static mut targets: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut sztargets: libc::c_int = 0;
static mut ntargets: libc::c_int = 0;
static mut sig_int_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_segv_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_abrt_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_term_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_bus_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_alrm_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn resetsighandlers() {
    signal(2 as libc::c_int, sig_int_handler);
    signal(11 as libc::c_int, sig_segv_handler);
    signal(6 as libc::c_int, sig_abrt_handler);
    signal(15 as libc::c_int, sig_term_handler);
    signal(7 as libc::c_int, sig_bus_handler);
}
unsafe extern "C" fn caughtsigmsg(mut sig: libc::c_int) {
    if verbose < 0 as libc::c_int {
        return;
    }
    printf(
        b"c\nc CAUGHT SIGNAL %d\0" as *const u8 as *const libc::c_char,
        sig,
    );
    match sig {
        2 => {
            printf(b" SIGINT\0" as *const u8 as *const libc::c_char);
        }
        11 => {
            printf(b" SIGSEGV\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            printf(b" SIGABRT\0" as *const u8 as *const libc::c_char);
        }
        15 => {
            printf(b" SIGTERM\0" as *const u8 as *const libc::c_char);
        }
        7 => {
            printf(b" SIGBUS\0" as *const u8 as *const libc::c_char);
        }
        14 => {
            printf(b" SIGALRM\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    printf(b"\nc\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if catchedsig == 0 {
        catchedsig = 1 as libc::c_int;
        caughtsigmsg(sig);
        fputs(
            b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char,
            stdout,
        );
        fflush(stdout);
        if verbose >= 0 as libc::c_int {
            lglflushtimers(lgl4sigh);
            lglstats(lgl4sigh);
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
    sig_bus_handler = signal(
        7 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
static mut timelimit: libc::c_int = -(1 as libc::c_int);
static mut caughtalarm: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn catchalrm(mut sig: libc::c_int) {
    if caughtalarm == 0 {
        caughtalarm = 1 as libc::c_int;
        caughtsigmsg(sig);
        if timelimit >= 0 as libc::c_int {
            printf(
                b"c time limit of %d reached after %.1f seconds\nc\n\0" as *const u8
                    as *const libc::c_char,
                timelimit,
                lglsec(lgl4sigh),
            );
            fflush(stdout);
        }
    }
}
unsafe extern "C" fn checkalarm(mut ptr: *mut libc::c_void) -> libc::c_int {
    caughtalarm
}
unsafe extern "C" fn flushobuf(
    mut obuf: *mut OBuf,
    mut simponly: libc::c_int,
    mut file: *mut FILE,
) {
    let fresh0 = (*obuf).pos;
    (*obuf).pos += 1;
    (*obuf).line[fresh0 as usize] = '\n' as i32 as libc::c_char;
    let fresh1 = (*obuf).pos;
    (*obuf).pos += 1;
    (*obuf).line[fresh1 as usize] = 0 as libc::c_int as libc::c_char;
    if simponly != 0 {
        fputs(b"c \0" as *const u8 as *const libc::c_char, file);
    }
    fputc('v' as i32, file);
    fputs(((*obuf).line).as_mut_ptr(), file);
    (*obuf).pos = 0 as libc::c_int;
}
unsafe extern "C" fn print2obuf(
    mut obuf: *mut OBuf,
    mut i: libc::c_int,
    mut simponly: libc::c_int,
    mut file: *mut FILE,
) {
    let mut str: [libc::c_char; 20] = [0; 20];
    let mut len: libc::c_int = 0;
    sprintf(
        str.as_mut_ptr(),
        b" %d\0" as *const u8 as *const libc::c_char,
        i,
    );
    len = strlen(str.as_mut_ptr()) as libc::c_int;
    if (*obuf).pos + len > 79 as libc::c_int {
        flushobuf(obuf, simponly, file);
    }
    strcpy(
        ((*obuf).line).as_mut_ptr().offset((*obuf).pos as isize),
        str.as_mut_ptr(),
    );
    (*obuf).pos += len;
}
unsafe extern "C" fn writefile(
    mut name: *const libc::c_char,
    mut clptr: *mut libc::c_int,
) -> *mut FILE {
    let mut len: libc::c_int = strlen(name) as libc::c_int;
    let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut res: *mut FILE = std::ptr::null_mut::<FILE>();
    if len >= 3 as libc::c_int
        && strcmp(
            name.offset(len as isize)
                .offset(-(3 as libc::c_int as isize)),
            b".gz\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        tmp = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        unlink(name);
        sprintf(
            tmp,
            b"gzip -c > %s\0" as *const u8 as *const libc::c_char,
            name,
        );
        res = popen(tmp, b"w\0" as *const u8 as *const libc::c_char);
        if !res.is_null() {
            *clptr = 2 as libc::c_int;
        }
        free(tmp as *mut libc::c_void);
    } else {
        res = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        if !res.is_null() {
            *clptr = 1 as libc::c_int;
        }
    }
    if res.is_null() {
        fprintf(
            stderr,
            b"*** lingeling error: can not write %s\n\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    res
}
unsafe extern "C" fn closefile(mut file: *mut FILE, mut type_0: libc::c_int) {
    if type_0 == 1 as libc::c_int {
        fclose(file);
    }
    if type_0 == 2 as libc::c_int {
        pclose(file);
    }
}
unsafe extern "C" fn lgltravcounter(mut voidptr: *mut libc::c_void, mut lit: libc::c_int) {
    let mut cntptr: *mut libc::c_int = voidptr as *mut libc::c_int;
    if lit == 0 {
        *cntptr += 1 as libc::c_int;
    }
}
unsafe extern "C" fn lglpushtarget(mut target: libc::c_int) {
    if ntargets == sztargets {
        sztargets = if sztargets != 0 {
            2 as libc::c_int * sztargets
        } else {
            1 as libc::c_int
        };
        targets = realloc(
            targets as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(sztargets as libc::c_ulong),
        ) as *mut libc::c_int;
    }
    let fresh2 = ntargets;
    ntargets += 1;
    *targets.offset(fresh2 as isize) = target;
}
static mut primes: [libc::c_int; 5] = [
    200000033 as libc::c_int,
    200000039 as libc::c_int,
    200000051 as libc::c_int,
    200000069 as libc::c_int,
    200000081 as libc::c_int,
];
static mut nprimes: libc::c_uint = 0;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut clout: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lineno: libc::c_int = 0;
    let mut simponly: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut target: libc::c_int = 0;
    let mut iname: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut oname: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut pname: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut match_0: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut p: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut err: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut thanks: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut out: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut pfile: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut maxvar: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut nopts: libc::c_int = 0;
    let mut simplevel: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut lgl: *mut LGL = std::ptr::null_mut::<LGL>();
    let mut obuf: OBuf = OBuf {
        line: [0; 81],
        pos: 0,
    };
    lineno = 1 as libc::c_int;
    out = std::ptr::null_mut::<FILE>();
    simplevel = 0 as libc::c_int;
    simponly = simplevel;
    clout = simponly;
    res = clout;
    thanks = std::ptr::null::<libc::c_char>();
    pname = thanks;
    oname = pname;
    iname = oname;
    lgl = lglinit();
    lgl4sigh = lgl;
    setsighandlers();
    i = 1 as libc::c_int;
    loop {
        if i >= argc {
            current_block = 17727836384662615028;
            break;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            printf(
                b"usage: lingeling [<option> ...][<file>[.<suffix>]]\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"where <option> is one of the following:\n\0" as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"-q               be quiet (same as '--verbose=-1')\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-s               only simplify and print to output file\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-O<L>            set simplification level to <L>\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-o <output>      set output file (default 'stdout')\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-p <options>     read options from file\n\0" as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"-T <seconds>     set time limit\n\0" as *const u8 as *const libc::c_char);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"-a <assumption>  use multiple assumptions\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"-h|--help        print command line option summary\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-f|--force       force reading even without header\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-r|--ranges      print value ranges of options\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-d|--defaults    print default values of options\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-P|--pcs         print (full) PCS file\n\0" as *const u8 as *const libc::c_char,
            );
            printf(
                b"--pcs-mixed      print mixed PCS file\n\0" as *const u8 as *const libc::c_char,
            );
            printf(
                b"--pcs-reduced    print reduced PCS file\n\0" as *const u8 as *const libc::c_char,
            );
            printf(
                b"-e|--embedded    ditto but in an embedded format print\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"-n|--no-witness   do not print solution (see '--witness')\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"-c               increase checking level\n\0" as *const u8 as *const libc::c_char,
            );
            printf(
                b"-l               increase logging level\n\0" as *const u8 as *const libc::c_char,
            );
            printf(
                b"-v               increase verbose level\n\0" as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"--verify         online forward check\n\0" as *const u8 as *const libc::c_char,
            );
            printf(b"--proof          generate proof file\n\0" as *const u8 as *const libc::c_char);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"--thanks=<whom>  alternative way of specifying the seed\n\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"                 (inspired by Vampire)\n\0" as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"The following options can also be used in the form '--<name>=<int>',\njust '--<name>' for increment and '--no-<name>' for zero.  They\ncan be embedded into the CNF file, set through the API or capitalized\nwith prefix 'LGL' instead of '--' through environment variables.\nTheir default values are displayed in square brackets.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"The input <file> can be compressed.  This is detected by matching\nthe <suffix> of the filename against 'gz', 'bz2, 'xz', 'zip', '7z'.\nHowever uncompressing a file is implemented by starting an external\nprocess running corresponding helper programs, e.g., 'gzip', 'bzip2'.\nThus those have to be installed and in the current path if needed.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            lglusage(lgl);
            current_block = 14603147171032977705;
            break;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
            fflush(stdout);
            current_block = 14603147171032977705;
            break;
        } else {
            if strcmp(
                *argv.offset(i as isize),
                b"-s\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                simponly = 1 as libc::c_int;
            } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
                && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                    == 'O' as i32
            {
                if simplevel > 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"*** lingeling error: multiple '-O..' options\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    simplevel = atoi((*argv.offset(i as isize)).offset(2 as libc::c_int as isize));
                    if simplevel <= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"*** lingeling error: invalid '%s' option\n\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-q\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                lglsetopt(
                    lgl,
                    b"verbose\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-o\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        b"*** lingeling error: argument to '-o' missing\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else if !oname.is_null() {
                    fprintf(
                        stderr,
                        b"*** lingeling error: multiple output files '%s' and '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        oname,
                        *argv.offset(i as isize),
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    oname = *argv.offset(i as isize);
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-p\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        b"*** lingeling error: argument to '-p' missing\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else if !pname.is_null() {
                    fprintf(
                        stderr,
                        b"*** lingeling error: multiple option files '%s' and '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        pname,
                        *argv.offset(i as isize),
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    pname = *argv.offset(i as isize);
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-T\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        b"*** lingeling error: argument to '-T' missing\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else if timelimit >= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"*** lingeling error: timit limit set twice\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    p = *argv.offset(i as isize);
                    while *p as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        p = p.offset(1);
                        p;
                    }
                    if p == *argv.offset(i as isize) as *const libc::c_char
                        || *p as libc::c_int != 0
                        || {
                            timelimit = atoi(*argv.offset(i as isize));
                            timelimit < 0 as libc::c_int
                        }
                    {
                        fprintf(
                            stderr,
                            b"*** lingeling error: invalid time limit '-T %s'\n\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-a\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        b"*** lingeling error: argument to '-a' missing\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    target = atoi(*argv.offset(i as isize));
                    if target == 0 {
                        fprintf(
                            stderr,
                            b"*** lingeling error: invalid literal in '-a %d'\n\0" as *const u8
                                as *const libc::c_char,
                            target,
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    } else {
                        lglpushtarget(target);
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-d\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--defaults\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                lglopts(
                    lgl,
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-e\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--embedded\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                lglopts(
                    lgl,
                    b"c \0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-r\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--ranges\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                lglrgopts(lgl);
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-P\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pcs\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                printf(b"# generated by 'lingeling --pcs'\n\0" as *const u8 as *const libc::c_char);
                printf(
                    b"# version %s\n\0" as *const u8 as *const libc::c_char,
                    lglversion(),
                );
                lglpcs(lgl, 0 as libc::c_int);
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"--pcs-mixed\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                printf(
                    b"# generated by 'lingeling --pcs-mixed'\n\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"# version %s\n\0" as *const u8 as *const libc::c_char,
                    lglversion(),
                );
                lglpcs(lgl, 1 as libc::c_int);
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"--pcs-reduced\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                printf(
                    b"# generated by 'lingeling --pcs-reduced'\n\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"# version %s\n\0" as *const u8 as *const libc::c_char,
                    lglversion(),
                );
                lglpcs(lgl, -(1 as libc::c_int));
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-f\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--force\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                force = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-n\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"no-witness\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                lglsetopt(
                    lgl,
                    b"witness\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-c\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                lglsetopt(
                    lgl,
                    b"check\0" as *const u8 as *const libc::c_char,
                    lglgetopt(lgl, b"check\0" as *const u8 as *const libc::c_char)
                        + 1 as libc::c_int,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-l\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                lglsetopt(
                    lgl,
                    b"log\0" as *const u8 as *const libc::c_char,
                    lglgetopt(lgl, b"log\0" as *const u8 as *const libc::c_char) + 1 as libc::c_int,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-v\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                lglsetopt(
                    lgl,
                    b"verbose\0" as *const u8 as *const libc::c_char,
                    lglgetopt(lgl, b"verbose\0" as *const u8 as *const libc::c_char)
                        + 1 as libc::c_int,
                );
            } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
                == '-' as i32
            {
                if *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                {
                    match_0 = strchr(
                        (*argv.offset(i as isize)).offset(2 as libc::c_int as isize),
                        '=' as i32,
                    );
                    if !match_0.is_null() {
                        p = match_0.offset(1 as libc::c_int as isize);
                        if *p as libc::c_int == '-' as i32 {
                            p = p.offset(1);
                            p;
                        }
                        len =
                            p.offset_from(*argv.offset(i as isize)) as libc::c_long as libc::c_int;
                        if strncmp(
                            *argv.offset(i as isize),
                            b"--write-api-trace=\0" as *const u8 as *const libc::c_char,
                            len as libc::c_ulong,
                        ) == 0
                        {
                            current_block = 7351195479953500246;
                        } else if strncmp(
                            *argv.offset(i as isize),
                            b"--thanks=\0" as *const u8 as *const libc::c_char,
                            len as libc::c_ulong,
                        ) == 0
                        {
                            thanks = match_0.offset(1 as libc::c_int as isize);
                            current_block = 7351195479953500246;
                        } else if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                            as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            current_block = 15067877082424188916;
                        } else {
                            loop {
                                p = p.offset(1);
                                if *p == 0 {
                                    current_block = 2956972668325154207;
                                    break;
                                }
                                if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                {
                                    current_block = 15067877082424188916;
                                    break;
                                }
                            }
                            match current_block {
                                15067877082424188916 => {}
                                _ => {
                                    len = (match_0.offset_from(*argv.offset(i as isize))
                                        as libc::c_long
                                        - 2 as libc::c_int as libc::c_long)
                                        as libc::c_int;
                                    tmp = malloc((len + 1 as libc::c_int) as libc::c_ulong)
                                        as *mut libc::c_char;
                                    j = 0 as libc::c_int;
                                    p = (*argv.offset(i as isize))
                                        .offset(2 as libc::c_int as isize);
                                    while *p as libc::c_int != '=' as i32 {
                                        let fresh3 = j;
                                        j += 1;
                                        *tmp.offset(fresh3 as isize) = *p;
                                        p = p.offset(1);
                                        p;
                                    }
                                    *tmp.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                                    val = atoi(match_0.offset(1 as libc::c_int as isize));
                                    current_block = 6988365858197790817;
                                }
                            }
                        }
                    } else {
                        if strncmp(
                            *argv.offset(i as isize),
                            b"--no-\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                            tmp = strdup(
                                (*argv.offset(i as isize)).offset(5 as libc::c_int as isize),
                            );
                            val = 0 as libc::c_int;
                        } else {
                            tmp = strdup(
                                (*argv.offset(i as isize)).offset(2 as libc::c_int as isize),
                            );
                            val = lglgetopt(lgl, tmp) + 1 as libc::c_int;
                        }
                        current_block = 6988365858197790817;
                    }
                    match current_block {
                        7351195479953500246 => {}
                        15067877082424188916 => {}
                        _ => {
                            if lglhasopt(lgl, tmp) == 0 {
                                free(tmp as *mut libc::c_void);
                                current_block = 15067877082424188916;
                            } else {
                                lglsetopt(lgl, tmp, val);
                                free(tmp as *mut libc::c_void);
                                current_block = 7351195479953500246;
                            }
                        }
                    }
                } else if *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize) != 0 {
                    current_block = 15067877082424188916;
                } else if lglhasopt(
                    lgl,
                    (*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
                ) == 0
                {
                    current_block = 15067877082424188916;
                } else {
                    val = lglgetopt(
                        lgl,
                        (*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
                    ) + 1 as libc::c_int;
                    lglsetopt(
                        lgl,
                        (*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
                        val,
                    );
                    current_block = 7351195479953500246;
                }
                match current_block {
                    7351195479953500246 => {}
                    _ => {
                        fprintf(
                            stderr,
                            b"*** lingeling error: invalid command line option '%s'\n\0"
                                as *const u8 as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    }
                }
            } else if !iname.is_null() {
                fprintf(
                    stderr,
                    b"*** lingeling error: can not read '%s' and '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    iname,
                    *argv.offset(i as isize),
                );
                res = 1 as libc::c_int;
                current_block = 14603147171032977705;
                break;
            } else {
                iname = *argv.offset(i as isize);
            }
            i += 1;
            i;
        }
    }
    if current_block == 17727836384662615028 {
        verbose = lglgetopt(lgl, b"verbose\0" as *const u8 as *const libc::c_char);
        if verbose >= 0 as libc::c_int {
            lglbnr(
                b"Lingeling SAT Solver\0" as *const u8 as *const libc::c_char,
                b"c \0" as *const u8 as *const libc::c_char,
                stdout,
            );
            if simponly != 0 {
                printf(b"c simplifying only\n\0" as *const u8 as *const libc::c_char);
            }
            if !oname.is_null() {
                printf(
                    b"c output file %s\n\0" as *const u8 as *const libc::c_char,
                    oname,
                );
            }
            if simponly != 0 || !oname.is_null() {
                fflush(stdout);
            }
            lglsetopt(
                lgl,
                b"trep\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        if !thanks.is_null() {
            let mut seed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut ch: libc::c_uint = 0;
            let mut iseed: libc::c_int = 0;
            p = thanks;
            loop {
                ch = *p as libc::c_uint;
                if ch == 0 {
                    break;
                }
                let fresh4 = i_0;
                i_0 = i_0.wrapping_add(1);
                seed = seed
                    .wrapping_add((primes[fresh4 as usize] as libc::c_uint).wrapping_mul(ch));
                if i_0 == nprimes {
                    i_0 = 0 as libc::c_int as libc::c_uint;
                }
                p = p.offset(1);
                p;
            }
            if seed >= 2147483647 as libc::c_int as libc::c_uint {
                seed >>= 1 as libc::c_int;
            }
            iseed = seed as libc::c_int;
            if verbose != 0 {
                printf(
                    b"c will have to thank %s (--seed=%d)\nc\n\0" as *const u8
                        as *const libc::c_char,
                    thanks,
                    iseed,
                );
            }
            lglsetopt(lgl, b"seed\0" as *const u8 as *const libc::c_char, iseed);
        }
        if verbose >= 2 as libc::c_int {
            printf(
                b"c\nc options after command line parsing:\nc\n\0" as *const u8
                    as *const libc::c_char,
            );
            lglopts(
                lgl,
                b"c \0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            printf(b"c\n\0" as *const u8 as *const libc::c_char);
            lglsizes(lgl);
            printf(b"c\n\0" as *const u8 as *const libc::c_char);
        }
        if !pname.is_null() {
            pfile = fopen(pname, b"r\0" as *const u8 as *const libc::c_char);
            if pfile.is_null() {
                fprintf(
                    stderr,
                    b"*** lingeling error: can not read option file %s\n\0" as *const u8
                        as *const libc::c_char,
                    pname,
                );
                res = 1 as libc::c_int;
                current_block = 14603147171032977705;
            } else {
                if verbose >= 0 as libc::c_int {
                    printf(
                        b"c reading options file %s\n\0" as *const u8 as *const libc::c_char,
                        pname,
                    );
                    fflush(stdout);
                }
                nopts = lglreadopts(lgl, pfile);
                if verbose >= 0 as libc::c_int {
                    printf(
                        b"c read and set %d options\nc\n\0" as *const u8 as *const libc::c_char,
                        nopts,
                    );
                    fflush(stdout);
                }
                fclose(pfile);
                current_block = 17418136423408909163;
            }
        } else {
            current_block = 17418136423408909163;
        }
        match current_block {
            14603147171032977705 => {}
            _ => {
                if iname.is_null() {
                    iname = b"<stdin>\0" as *const u8 as *const libc::c_char;
                    err = lglparsefile(lgl, stdin, force, &mut lineno, &mut maxvar);
                } else {
                    err = lglparsepath(lgl, iname, force, &mut lineno, &mut maxvar);
                }
                if !err.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                        iname,
                        lineno,
                        err,
                    );
                    res = 1 as libc::c_int;
                } else {
                    if verbose >= 1 as libc::c_int {
                        printf(b"c\n\0" as *const u8 as *const libc::c_char);
                        if verbose >= 2 as libc::c_int {
                            printf(
                                b"c final options:\nc\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        lglopts(
                            lgl,
                            b"c \0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int,
                        );
                    }
                    if timelimit >= 0 as libc::c_int {
                        if verbose >= 0 as libc::c_int {
                            printf(
                                b"c\nc setting time limit of %d seconds\n\0" as *const u8
                                    as *const libc::c_char,
                                timelimit,
                            );
                            fflush(stdout);
                        }
                        lglseterm(
                            lgl,
                            Some(
                                checkalarm
                                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                            ),
                            &mut caughtalarm as *mut libc::c_int as *mut libc::c_void,
                        );
                        sig_alrm_handler = signal(
                            14 as libc::c_int,
                            Some(catchalrm as unsafe extern "C" fn(libc::c_int) -> ()),
                        );
                        alarm(timelimit as libc::c_uint);
                    }
                    i = 0 as libc::c_int;
                    while i < ntargets {
                        lglassume(lgl, *targets.offset(i as isize));
                        i += 1;
                        i;
                    }
                    if simplevel > 0 as libc::c_int {
                        if verbose >= 1 as libc::c_int {
                            printf(
                                b"c simplifying with simplification level %d\n\0" as *const u8
                                    as *const libc::c_char,
                                simplevel,
                            );
                            fflush(stdout);
                        }
                        res = lglsimp(lgl, simplevel);
                        if verbose >= 1 as libc::c_int {
                            printf(
                                b"c simplifying result %d after %.2f seconds\n\0" as *const u8
                                    as *const libc::c_char,
                                res,
                                lglsec(lgl),
                            );
                            fflush(stdout);
                        }
                    }
                    res = lglsat(lgl);
                    if timelimit >= 0 as libc::c_int {
                        caughtalarm = 0 as libc::c_int;
                        signal(14 as libc::c_int, sig_alrm_handler);
                    }
                    if !oname.is_null() {
                        let mut start: libc::c_double = lglsec(lgl);
                        let mut delta: libc::c_double = 0.;
                        if strcmp(oname, b"-\0" as *const u8 as *const libc::c_char) == 0 {
                            out = stdout;
                            oname = b"<stdout>\0" as *const u8 as *const libc::c_char;
                            clout = 0 as libc::c_int;
                            current_block = 10528013381497917728;
                        } else {
                            out = writefile(oname, &mut clout);
                            if out.is_null() {
                                res = 1 as libc::c_int;
                                current_block = 14603147171032977705;
                            } else {
                                current_block = 10528013381497917728;
                            }
                        }
                        match current_block {
                            14603147171032977705 => {}
                            _ => {
                                if verbose >= 0 as libc::c_int {
                                    count = 0 as libc::c_int;
                                    lglctrav(
                                        lgl,
                                        &mut count as *mut libc::c_int as *mut libc::c_void,
                                        Some(
                                            lgltravcounter
                                                as unsafe extern "C" fn(
                                                    *mut libc::c_void,
                                                    libc::c_int,
                                                )
                                                    -> (),
                                        ),
                                    );
                                    printf(
                                        b"c\nc writing 'p cnf %d %d' to '%s'\n\0" as *const u8
                                            as *const libc::c_char,
                                        maxvar,
                                        count,
                                        oname,
                                    );
                                    fflush(stdout);
                                }
                                lglprint(lgl, out);
                                closefile(out, clout);
                                if verbose >= 0 as libc::c_int {
                                    delta = lglsec(lgl) - start;
                                    if delta < 0 as libc::c_int as libc::c_double {
                                        delta = 0 as libc::c_int as libc::c_double;
                                    }
                                    printf(
                                        b"c collected garbage and wrote '%s' in %.1f seconds\n\0"
                                            as *const u8 as *const libc::c_char,
                                        oname,
                                        delta,
                                    );
                                    printf(b"c\n\0" as *const u8 as *const libc::c_char);
                                    fflush(stdout);
                                }
                                current_block = 4533671380017093834;
                            }
                        }
                    } else {
                        current_block = 4533671380017093834;
                    }
                    match current_block {
                        14603147171032977705 => {}
                        _ => {
                            if simponly == 0 || verbose >= 0 as libc::c_int {
                                if simponly != 0 {
                                    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
                                }
                                if res == 10 as libc::c_int {
                                    fputs(
                                        b"s SATISFIABLE\n\0" as *const u8
                                            as *const libc::c_char,
                                        stdout,
                                    );
                                } else if res == 20 as libc::c_int {
                                    fputs(
                                        b"s UNSATISFIABLE\n\0" as *const u8
                                            as *const libc::c_char,
                                        stdout,
                                    );
                                } else {
                                    fputs(
                                        b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char,
                                        stdout,
                                    );
                                }
                                if !thanks.is_null() {
                                    printf(
                                        b"c\nc Thanks to %s!\nc\n\0" as *const u8
                                            as *const libc::c_char,
                                        thanks,
                                    );
                                }
                                fflush(stdout);
                                if res == 10 as libc::c_int
                                    && lglgetopt(
                                        lgl,
                                        b"witness\0" as *const u8 as *const libc::c_char,
                                    ) != 0
                                {
                                    obuf.pos = 0 as libc::c_int;
                                    i = 1 as libc::c_int;
                                    while i <= maxvar {
                                        lit = if lglderef(lgl, i) > 0 as libc::c_int {
                                            i
                                        } else {
                                            -i
                                        };
                                        print2obuf(&mut obuf, lit, simponly, stdout);
                                        i += 1;
                                        i;
                                    }
                                    print2obuf(&mut obuf, 0 as libc::c_int, simponly, stdout);
                                    if obuf.pos > 0 as libc::c_int {
                                        flushobuf(&mut obuf, simponly, stdout);
                                    }
                                    fflush(stdout);
                                }
                            }
                            if verbose >= 0 as libc::c_int {
                                fputs(b"c\n\0" as *const u8 as *const libc::c_char, stdout);
                                lglstats(lgl);
                            }
                        }
                    }
                }
            }
        }
    }
    resetsighandlers();
    lgl4sigh = std::ptr::null_mut::<LGL>();
    lglrelease(lgl);
    free(targets as *mut libc::c_void);
    if verbose > 0 as libc::c_int {
        printf(b"c exit %d\n\0" as *const u8 as *const libc::c_char, res);
    }
    fflush(stdout);
    res
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
unsafe extern "C" fn run_static_initializers() {
    nprimes = (::core::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_uint;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
