//! SW64-specific definitions for 64-bit linux-like values

use pthread_mutex_t;

pub type blksize_t = u32;
pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type nlink_t = u32;
pub type suseconds_t = i64;
pub type wchar_t = i32;
pub type __u64 = ::c_ulong;
pub type __s64 = ::c_long;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino64_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        __pad1: ::c_int,

        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_blksize: ::blksize_t,
        pub st_nlink: ::nlink_t,
        __pad2: ::c_int,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __glibc_reserved: [::c_long; 3],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino64_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off64_t,
        pub st_blocks: ::blkcnt64_t,

        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_blksize: ::blksize_t,
        pub st_nlink: ::nlink_t,
        __pad0: ::c_int,

        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 3],
    }

    pub struct statfs64 {
        pub f_type: ::c_long,
        pub f_bsize: ::c_long,
        pub f_blocks: ::c_long,
        pub f_bfree: ::c_long,
        pub f_bavail: ::c_long,
        pub f_files: ::c_ulong,
        pub f_ffree: ::c_ulong,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_long,
        pub f_frsize: ::c_long,
        pub f_flags: ::c_long,
        pub f_spare: [::c_long; 4],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct statvfs64 {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::c_long,
        pub f_bfree: ::c_long,
        pub f_bavail: ::c_long,
        pub f_files: ::c_ulong,
        pub f_ffree: ::c_ulong,
        pub f_favail: ::c_ulong,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct pthread_attr_t {
        __size: [::c_ulong; 7]
    }

    // GUYC20201103 add sigaction stack_t siginfo_t

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        pub sa_flags: ::c_int,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
        _align: [usize; 0],
    }

    // GUYC20201103 add flock
    
    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_uint,
        pub __seq: ::c_ushort,
        __pad1: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong
    }

    // GUYC20210303 copy from 1.34.2 libc/src/unix/notbsd/linux/other/mod.rs
    pub struct statfs {
        pub f_type: ::__fsword_t,
        pub f_bsize: ::__fsword_t,
        pub f_blocks: ::c_ulong,
        pub f_bfree: ::c_ulong,
        pub f_bavail: ::c_ulong,

        pub f_files: ::c_ulong,
        pub f_ffree: ::c_ulong,
        pub f_fsid: ::fsid_t,

        pub f_namelen: ::__fsword_t,
        pub f_frsize: ::c_ulong,
        pub f_flags: ::c_ulong,
        f_spare: [::c_ulong; 4],
    }
}

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;

align_const! {
    pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
        pthread_mutex_t {
            size: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
    pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
        pthread_mutex_t {
            size: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
    pub const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
        pthread_mutex_t {
            size: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        };
}

pub const TIOCGSOFTCAR: ::c_ulong = 0x5419;
pub const TIOCSSOFTCAR: ::c_ulong = 0x541A;

pub const RLIMIT_NOFILE: ::__rlimit_resource_t = 6;
pub const RLIMIT_NPROC: ::__rlimit_resource_t = 8;
pub const RLIMIT_CPU: ::__rlimit_resource_t = 0;

pub const O_APPEND: ::c_int = 0o00010;
pub const O_CREAT: ::c_int = 0o01000;
pub const O_EXCL: ::c_int = 0o04000;
pub const O_NOCTTY: ::c_int = 0o010000;
pub const O_DIRECT: ::c_int = 0o02000000;
pub const O_DIRECTORY: ::c_int = 0o0100000;
pub const O_NOFOLLOW: ::c_int = 0o0200000;
pub const O_NONBLOCK: ::c_int = 0o00004;
pub const O_CLOEXEC: ::c_int = 0o010000000;


pub const O_DSYNC: ::c_int = 0o040000;
pub const O_SYNC: ::c_int = 0o020000000 |O_DSYNC;
pub const O_RSYNC: ::c_int = O_SYNC;    // TODO
pub const O_FSYNC: ::c_int = O_SYNC;    // TODO

pub const O_NOATIME: ::c_int = 0o04000000;
pub const O_PATH: ::c_int = 0o040000000;
pub const O_TMPFILE: ::c_int = 0o0100100000;

pub const MAP_GROWSDOWN: ::c_int = 0x01000;

pub const EDEADLK: ::c_int = 11;
pub const ENAMETOOLONG: ::c_int = 63;
pub const ENOLCK: ::c_int = 77;
pub const ENOSYS: ::c_int = 78;
pub const ENOTEMPTY: ::c_int = 66;
pub const ELOOP: ::c_int = 62;

pub const ENOMSG: ::c_int = 80;
pub const EIDRM: ::c_int = 81;
pub const ECHRNG: ::c_int = 88;
pub const EL2NSYNC: ::c_int = 89;
pub const EL3HLT: ::c_int = 90;
pub const EL3RST: ::c_int = 91;
pub const ELNRNG: ::c_int = 93;
pub const EUNATCH: ::c_int = 94;
pub const ENOCSI: ::c_int = 95;
pub const EL2HLT: ::c_int = 96;
pub const EBADE: ::c_int = 97;
pub const EBADR: ::c_int = 98;
pub const EXFULL: ::c_int = 99;
pub const ENOANO: ::c_int = 100;
pub const EBADRQC: ::c_int = 101;
pub const EBADSLT: ::c_int = 102;
pub const EMULTIHOP: ::c_int = 110;
pub const EOVERFLOW: ::c_int = 112;
pub const ENOTUNIQ: ::c_int = 113;
pub const EBADFD: ::c_int = 114;
pub const EBADMSG: ::c_int = 84;


pub const EREMCHG: ::c_int = 115;
pub const ELIBACC: ::c_int = 122;
pub const ELIBBAD: ::c_int = 123;
pub const ELIBSCN: ::c_int = 124;
pub const ELIBMAX: ::c_int = 125;
pub const ELIBEXEC: ::c_int = 126;
pub const EILSEQ: ::c_int = 116;

pub const ERESTART: ::c_int = 127;
pub const ESTRPIPE: ::c_int = 128;
pub const EUSERS: ::c_int = 68;
pub const ENOTSOCK: ::c_int = 38;
pub const EDESTADDRREQ: ::c_int = 39;
pub const EMSGSIZE: ::c_int = 40;
pub const EPROTOTYPE: ::c_int = 41;
pub const ENOPROTOOPT: ::c_int = 42;
pub const EPROTONOSUPPORT: ::c_int = 43;
pub const ESOCKTNOSUPPORT: ::c_int = 44;
pub const EOPNOTSUPP: ::c_int = 45;
pub const EPFNOSUPPORT: ::c_int = 46;
pub const EAFNOSUPPORT: ::c_int = 47;
pub const EADDRINUSE: ::c_int = 48;
pub const EADDRNOTAVAIL: ::c_int = 49;
pub const ENETDOWN: ::c_int = 50;
pub const ENETUNREACH: ::c_int = 51;
pub const ENETRESET: ::c_int = 52;
pub const ECONNABORTED: ::c_int = 53;
pub const ECONNRESET: ::c_int = 54;
pub const ENOBUFS: ::c_int = 55;
pub const EISCONN: ::c_int = 56;
pub const ENOTCONN: ::c_int = 57;
pub const ESHUTDOWN: ::c_int = 58;
pub const ETOOMANYREFS: ::c_int = 59;
pub const ETIMEDOUT: ::c_int = 60;
pub const ECONNREFUSED: ::c_int = 61;
pub const EHOSTDOWN: ::c_int = 64;
pub const EHOSTUNREACH: ::c_int = 65;
pub const EALREADY: ::c_int = 37;
pub const EINPROGRESS: ::c_int = 36;
pub const ESTALE: ::c_int = 70;
pub const EDQUOT: ::c_int = 69;

pub const ENOMEDIUM: ::c_int = 129;
pub const EMEDIUMTYPE: ::c_int = 130;
pub const ECANCELED: ::c_int = 131;
pub const ENOKEY: ::c_int = 132;
pub const EKEYEXPIRED: ::c_int = 133;
pub const EKEYREVOKED: ::c_int = 134;
pub const EKEYREJECTED: ::c_int = 135;
pub const EOWNERDEAD: ::c_int = 136;
pub const ENOTRECOVERABLE: ::c_int = 137;
pub const EHWPOISON: ::c_int = 139;
pub const ERFKILL: ::c_int = 138;

pub const SOL_SOCKET: ::c_int = 0xffff;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_TYPE: ::c_int = 0x1008;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDBUFFORCE: ::c_int = 0x100a;
pub const SO_RCVBUFFORCE: ::c_int = 0x100b;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PRIORITY: ::c_int = 12;

pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 0x0200;
pub const SO_PASSCRED: ::c_int = 17;
pub const SO_PEERCRED: ::c_int = 18;
pub const SO_RCVLOWAT: ::c_int = 0x1010;
pub const SO_SNDLOWAT: ::c_int = 0x1011;
pub const SO_RCVTIMEO: ::c_int = 0x1012;
pub const SO_SNDTIMEO: ::c_int = 0x1013;

pub const SO_SECURITY_AUTHENTICATION: ::c_int = 19;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: ::c_int = 20;
pub const SO_SECURITY_ENCRYPTION_NETWORK: ::c_int = 21;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_ATTACH_FILTER: ::c_int = 26;
pub const SO_DETACH_FILTER: ::c_int = 27;
pub const SO_GET_FILTER: ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME: ::c_int = 28;
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_ACCEPTCONN: ::c_int = 0x1014;

pub const SO_PEERSEC: ::c_int = 30;
pub const SO_PASSSEC: ::c_int = 34;
pub const SO_TIMESTAMPNS: ::c_int = 35;
pub const SCM_TIMESTAMPNS: ::c_int = SO_TIMESTAMPNS;
pub const SO_MARK: ::c_int = 36;
pub const SO_PROTOCOL: ::c_int = 0x1028;
pub const SO_DOMAIN: ::c_int = 0x1029;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_WIFI_STATUS: ::c_int = 41;
pub const SCM_WIFI_STATUS: ::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_NOFCS: ::c_int = 43;
pub const SO_LOCK_FILTER: ::c_int = 44;
pub const SO_SELECT_ERR_QUEUE: ::c_int = 45;
pub const SO_BUSY_POLL: ::c_int = 46;
pub const SO_MAX_PACING_RATE: ::c_int = 47;
pub const SO_BPF_EXTENSIONS: ::c_int = 48;
pub const SO_INCOMING_CPU: ::c_int = 49;
pub const SO_ATTACH_BPF: ::c_int = 50;
pub const SO_DETACH_BPF: ::c_int = SO_DETACH_FILTER;
pub const SA_ONSTACK: ::c_int = 0x00000001;
pub const SA_SIGINFO: ::c_int = 0x00000040;
pub const SA_NOCLDWAIT: ::c_int = 0x00000020;

pub const SIGCHLD: ::c_int = 20;
pub const SIGBUS: ::c_int = 10;
pub const SIGUSR1: ::c_int = 30;
pub const SIGUSR2: ::c_int = 31;
pub const SIGCONT: ::c_int = 19;
pub const SIGSTOP: ::c_int = 17;
pub const SIGTSTP: ::c_int = 18;
pub const SIGURG: ::c_int = 16;
pub const SIGIO: ::c_int = 23;
pub const SIGSYS: ::c_int = 12;
pub const SIGINFO: ::c_int = 29;
pub const SIGSTKFLT: ::c_int = 32;
pub const SIGUNUSED: ::c_int = 31;
pub const SIGPOLL: ::c_int = SIGIO;
pub const SIGPWR: ::c_int = SIGINFO;
pub const SIG_SETMASK: ::c_int = 3;
pub const SIG_BLOCK: ::c_int = 1;
pub const SIG_UNBLOCK: ::c_int = 2;

pub const POLLWRNORM: ::c_short = 0x0100; // TODO
pub const POLLWRBAND: ::c_short = 0x0200;
pub const O_ASYNC: ::c_int = 0o020000;
pub const O_NDELAY: ::c_int = 0o00004;
pub const PTRACE_DETACH: ::c_uint = 17;
pub const EFD_NONBLOCK: ::c_int = 0o000000004;

pub const F_GETLK: ::c_int = 7;
pub const F_GETOWN: ::c_int = 6;
pub const F_SETOWN: ::c_int = 5;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_RDLCK: ::c_int = 1;
pub const F_WRLCK: ::c_int = 2;
pub const F_UNLCK: ::c_int = 8;

pub const SFD_NONBLOCK: ::c_int = 0o000000004;
pub const TIOCEXCL: ::c_ulong = 0x540C; 
pub const TIOCNXCL: ::c_ulong = 0x540D;
pub const TIOCSCTTY: ::c_ulong = 0x540E;
pub const TIOCSTI: ::c_ulong = 0x5412;
pub const TIOCMGET: ::c_ulong = 0x5415;
pub const TIOCMBIS: ::c_ulong = 0x5416;
pub const TIOCMBIC: ::c_ulong = 0x5417;
pub const TIOCMSET: ::c_ulong = 0x5418;
pub const TIOCCONS: ::c_ulong = 0x541D;

pub const SFD_CLOEXEC: ::c_int = 0o010000000;

pub const NCCS: usize = 32;
pub const O_TRUNC: ::c_int = 0o02000;
pub const EBFONT: ::c_int = 104;
pub const ENOSTR: ::c_int = 87;
pub const ENODATA: ::c_int = 86;
pub const ETIME: ::c_int = 83;
pub const ENOSR: ::c_int = 82;
pub const ENONET: ::c_int = 105;
pub const ENOPKG: ::c_int = 92;
pub const EREMOTE: ::c_int = 71;
pub const EREMOTEIO: ::c_int = 121;
pub const ENOLINK: ::c_int = 106;
pub const EADV: ::c_int = 107;
pub const ESRMNT: ::c_int = 108;
pub const ECOMM: ::c_int = 109;
pub const EPROTO: ::c_int = 85;
pub const EDOTDOT: ::c_int = 111;
pub const SA_NODEFER: ::c_int = 0x00000008;
pub const SA_NOMASK: ::c_int = SA_NODEFER;
pub const SA_RESETHAND: ::c_int = 0x00000010;
pub const SA_ONESHOT: ::c_int = SA_RESETHAND;
pub const SA_RESTART: ::c_int = 0x00000002;
pub const SA_NOCLDSTOP: ::c_int = 0x00000004;

pub const EPOLL_CLOEXEC: ::c_int = 0o010000000; // TODO
pub const EFD_CLOEXEC: ::c_int = 0o010000000;   // TODO

pub const MAP_LOCKED: ::c_int = 0x08000;
pub const MAP_NORESERVE: ::c_int = 0x10000;

pub const EDEADLOCK: ::c_int = EDEADLK;

pub const FIOCLEX: ::c_ulong = 0x20006601;
pub const FIONBIO: ::c_ulong = 0x8004667e;

pub const MCL_CURRENT: ::c_int = 8192;
pub const MCL_FUTURE: ::c_int = 16384;

pub const SIGSTKSZ: ::size_t = 16384;
pub const MINSIGSTKSZ: ::size_t = 4096;
pub const CBAUD: ::tcflag_t = 0o0000037;
pub const CBAUDEX: ::tcflag_t = 0o0000000;
pub const TAB1: ::c_int = 0o00002000;
pub const TAB2: ::c_int = 0o00004000;
pub const TAB3: ::c_int = 0o00006000;
pub const CR1: ::c_int  = 0o00010000;
pub const CR2: ::c_int  = 0o00020000;
pub const CR3: ::c_int  = 0o00030000;
pub const FF1: ::c_int  = 0o00040000;
pub const BS1: ::c_int  = 0o00100000;
pub const VT1: ::c_int  = 0o00200000;

pub const VWERASE: usize = 4;
pub const VREPRINT: usize = 6;
pub const VSUSP: usize = 10;
pub const VSTART: usize = 12;
pub const VSTOP: usize = 13;
pub const VDISCARD: usize = 1001;
pub const VTIME: usize = 17;
pub const IXON: ::tcflag_t = 0o0001000;
pub const IXOFF: ::tcflag_t = 0o0002000;
pub const ONLCR: ::tcflag_t = 0o0000002;
pub const CSIZE: ::tcflag_t = 0o00001400;
pub const CS6: ::tcflag_t = 0o00000400;
pub const CS7: ::tcflag_t = 0o00001000;
pub const CS8: ::tcflag_t = 0o00001400;
pub const CSTOPB: ::tcflag_t = 0o00002000;
pub const CREAD: ::tcflag_t = 0o00004000;
pub const PARENB: ::tcflag_t = 0o00010000;
pub const PARODD: ::tcflag_t = 0o00020000;

pub const HUPCL: ::tcflag_t = 0o00040000;
pub const CLOCAL: ::tcflag_t = 0o00100000;
pub const ECHOKE: ::tcflag_t = 0x00000001;
pub const ECHOE: ::tcflag_t = 0x00000002;
pub const ECHOK: ::tcflag_t = 0x00000004;
pub const ECHONL: ::tcflag_t = 0x00000010;
pub const ECHOPRT: ::tcflag_t = 0x00000020;
pub const ECHOCTL: ::tcflag_t = 0x00000040;
pub const ISIG: ::tcflag_t = 0x00000080;
pub const ICANON: ::tcflag_t = 0x00000100;
pub const PENDIN: ::tcflag_t = 0x20000000;
pub const NOFLSH: ::tcflag_t = 0x80000000;
pub const CIBAUD: ::tcflag_t = 0o07600000;
pub const VSWTC: usize = 700;
pub const OLCUC:  ::tcflag_t = 0o0000004;
pub const NLDLY:  ::tcflag_t = 0o00001400;
pub const CRDLY:  ::tcflag_t = 0o00030000;
pub const TABDLY: ::tcflag_t = 0o00006000;
pub const BSDLY:  ::tcflag_t = 0o00100000;
pub const FFDLY:  ::tcflag_t = 0o00040000;
pub const VTDLY:  ::tcflag_t = 0o00200000;
pub const XTABS:  ::tcflag_t = 0o00006000;

pub const B0: ::speed_t = 0o0000000;
pub const B50: ::speed_t = 0o0000001;
pub const B75: ::speed_t = 0o0000002;
pub const B110: ::speed_t = 0o0000003;
pub const B134: ::speed_t = 0o0000004;
pub const B150: ::speed_t = 0o0000005;
pub const B200: ::speed_t = 0o0000006;
pub const B300: ::speed_t = 0o0000007;
pub const B600: ::speed_t = 0o0000010;
pub const B1200: ::speed_t = 0o0000011;
pub const B1800: ::speed_t = 0o0000012;
pub const B2400: ::speed_t = 0o0000013;
pub const B4800: ::speed_t = 0o0000014;
pub const B9600: ::speed_t = 0o0000015;
pub const B19200: ::speed_t = 0o0000016;
pub const B38400: ::speed_t = 0o0000017;
pub const EXTA: ::speed_t = B19200;
pub const EXTB: ::speed_t = B38400;
//pub const BOTHER: ::speed_t = 0o00037;
pub const B57600: ::speed_t = 0o00020;
pub const B115200: ::speed_t = 0o00021;
pub const B230400: ::speed_t = 0o00022;
pub const B460800: ::speed_t = 0o00023;
pub const B500000: ::speed_t = 0o00024;
pub const B576000: ::speed_t = 0o00025;
pub const B921600: ::speed_t = 0o00026;
pub const B1000000: ::speed_t = 0o00027;
pub const B1152000: ::speed_t = 0o00030;
pub const B1500000: ::speed_t = 0o00031;
pub const B2000000: ::speed_t = 0o00032;
pub const B2500000: ::speed_t = 0o00033;
pub const B3000000: ::speed_t = 0o00034;
pub const B3500000: ::speed_t = 0o00035;
pub const B4000000: ::speed_t = 0o00036;

//pub const VEOL: usize = 1;
//pub const VEOL2: usize = 2;
pub const VEOL: usize = 5;
pub const VEOL2: usize = 7;
pub const VMIN: usize = 16;
//pub const _VMIN: usize = 4;
pub const IEXTEN: ::tcflag_t = 0x00000400;
pub const TOSTOP: ::tcflag_t = 0x00400000;
pub const FLUSHO: ::tcflag_t = 0x00800000;
pub const EXTPROC: ::tcflag_t = 0x10000000;
pub const TCGETS: ::c_ulong = 0x402c7413;
//pub const TCGETS2: ::c_ulong = 0x401a542a;
pub const TCSETS: ::c_ulong = 0x802c7414;
pub const TCSETSW: ::c_ulong = 0x802c7415;
//pub const TCSETSW2: ::c_ulong = 0x801a542c;
pub const TCSETSF: ::c_ulong = 0x802c7416;
//pub const TCSETSF2: ::c_ulong = 0x801a542d;
pub const TCGETA: ::c_ulong = 0x40127417;
pub const TCSETA: ::c_ulong = 0x80127418;
pub const TCSETAW: ::c_ulong = 0x80127419;
pub const TCSETAF: ::c_ulong = 0x8012741c;
pub const TCSBRK: ::c_ulong = 0x2000741d;
pub const TCSBRKP: ::c_ulong = 0x5425;
pub const FIONREAD: ::c_ulong = 0x4004667f;
pub const TCXONC: ::c_ulong = 0x2000741e;
pub const TCFLSH: ::c_ulong = 0x2000741f;
pub const TIOCINQ: ::c_ulong = FIONREAD;

pub const TIOCGPGRP: ::c_ulong = 0x40047477;
pub const TIOCSPGRP: ::c_ulong = 0x80047476;
pub const TIOCOUTQ: ::c_ulong = 0x40047473;
pub const TIOCGWINSZ: ::c_ulong = 0x40087468;
pub const TIOCSWINSZ: ::c_ulong = 0x80087467;

// GUYC20201112 add RLIMIT_RSS ~ TIOCM_DSR
pub const RLIMIT_RSS: ::__rlimit_resource_t = 5;
pub const RLIMIT_AS: ::__rlimit_resource_t = 7;
pub const RLIMIT_MEMLOCK: ::__rlimit_resource_t = 9;
pub const MAP_ANON: ::c_int = 0x0010;
pub const MAP_ANONYMOUS: ::c_int = 0x0010;
pub const MAP_DENYWRITE: ::c_int = 0x2000;
pub const MAP_EXECUTABLE: ::c_int = 0x04000;
pub const MAP_POPULATE: ::c_int = 0x20000;
pub const MAP_NONBLOCK: ::c_int = 0x40000;
pub const MAP_STACK: ::c_int = 0x80000;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;
pub const VEOF: usize = 100;
pub const MAP_HUGETLB: ::c_int = 0x100000;
pub const TCSANOW: ::c_int = 0;
pub const TCSADRAIN: ::c_int = 1;
pub const TCSAFLUSH: ::c_int = 2;
pub const TIOCLINUX: ::c_ulong = 0x541C;
pub const TIOCGSERIAL: ::c_ulong = 0x541E;
pub const RTLD_DEEPBIND: ::c_int = 0x8;
pub const RTLD_GLOBAL: ::c_int = 0x100;
pub const RTLD_NOLOAD: ::c_int = 0x4;
pub const TIOCM_ST: ::c_int = 0x008;
pub const TIOCM_SR: ::c_int = 0x010;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_DSR: ::c_int = 0x100;

pub const SYS_osf_syscall: ::c_long = 0;
pub const SYS_exit: ::c_long = 1;
pub const SYS_fork: ::c_long = 2;
pub const SYS_read: ::c_long = 3;
pub const SYS_write:    ::c_long = 4;
pub const SYS_osf_old_open: ::c_long = 5;
pub const SYS_close:    ::c_long = 6;
pub const SYS_osf_wait4:    ::c_long = 7;
pub const SYS_osf_old_creat:    ::c_long = 8;
pub const SYS_link: ::c_long = 9;
pub const SYS_unlink:   ::c_long = 10;
pub const SYS_osf_execve:   ::c_long = 11;
pub const SYS_chdir: ::c_long = 12;
pub const SYS_fchdir: ::c_long = 13;
pub const SYS_mknod: ::c_long = 14;
pub const SYS_chmod: ::c_long = 15;
pub const SYS_chown: ::c_long = 16;
pub const SYS_brk: ::c_long = 17;
pub const SYS_osf_getfsstat: ::c_long = 18;
pub const SYS_lseek: ::c_long = 19;
pub const SYS_getxpid: ::c_long = 20;
pub const SYS_osf_mount: ::c_long = 21;
pub const SYS_umount: ::c_long = 22;
pub const SYS_setuid: ::c_long = 23;
pub const SYS_getxuid: ::c_long = 24;
pub const SYS_exec_with_loader: ::c_long = 25;
pub const SYS_ptrace: ::c_long = 26;
pub const SYS_osf_nrecvmsg: ::c_long = 27;
pub const SYS_osf_nsendmsg: ::c_long = 28;
pub const SYS_osf_nrecvfrom: ::c_long = 29;
pub const SYS_osf_naccept: ::c_long = 30;
pub const SYS_osf_ngetpeername: ::c_long = 31;
pub const SYS_osf_ngetsockname: ::c_long = 32;
pub const SYS_access: ::c_long = 33;
pub const SYS_osf_chflags: ::c_long = 34;
pub const SYS_osf_fchflags: ::c_long = 35;
pub const SYS_sync: ::c_long = 36;
pub const SYS_kill: ::c_long = 37;
pub const SYS_osf_old_stat: ::c_long = 38;
pub const SYS_setpgid: ::c_long = 39;
pub const SYS_osf_old_lstat: ::c_long = 40;
pub const SYS_dup: ::c_long = 41;
pub const SYS_pipe: ::c_long = 42;
pub const SYS_osf_set_program_attributes: ::c_long = 43;
pub const SYS_osf_profil: ::c_long = 44;
pub const SYS_open: ::c_long = 45;
pub const SYS_osf_old_sigaction: ::c_long = 46;
pub const SYS_getxgid: ::c_long = 47;
pub const SYS_osf_sigprocmask: ::c_long = 48;
pub const SYS_osf_getlogin: ::c_long = 49;
pub const SYS_osf_setlogin: ::c_long = 50;
pub const SYS_acct: ::c_long =  51;
pub const SYS_sigpending: ::c_long = 52;
pub const SYS_ioctl: ::c_long = 54;
pub const SYS_osf_reboot: ::c_long = 55;
pub const SYS_osf_revoke: ::c_long = 56;
pub const SYS_symlink: ::c_long = 57;
pub const SYS_readlink: ::c_long = 58;
pub const SYS_execve: ::c_long = 59;
pub const SYS_umask: ::c_long = 60;
pub const SYS_chroot: ::c_long = 61;
pub const SYS_osf_old_fstat: ::c_long = 62;
pub const SYS_getpgrp: ::c_long = 63;
pub const SYS_getpagesize: ::c_long = 64;
pub const SYS_osf_mremap: ::c_long = 65;
pub const SYS_vfork: ::c_long = 66;
pub const SYS_stat: ::c_long = 67;
pub const SYS_lstat: ::c_long = 68;
pub const SYS_osf_sbrk: ::c_long = 69;
pub const SYS_osf_sstk: ::c_long = 70;
pub const SYS_mmap: ::c_long = 71;
pub const SYS_osf_old_vadvise: ::c_long = 72;
pub const SYS_munmap: ::c_long = 73;
pub const SYS_mprotect: ::c_long = 74;
pub const SYS_madvise: ::c_long = 75;
pub const SYS_vhangup: ::c_long = 76;
pub const SYS_osf_kmodcall: ::c_long = 77;
pub const SYS_osf_mincore: ::c_long = 78;
pub const SYS_getgroups: ::c_long = 79;
pub const SYS_setgroups: ::c_long = 80;
pub const SYS_osf_old_getpgrp: ::c_long = 81;
pub const SYS_setpgrp:  ::c_long = 82;
pub const SYS_osf_setitimer: ::c_long = 83;
pub const SYS_osf_old_wait: ::c_long = 84;
pub const SYS_osf_table: ::c_long = 85;
pub const SYS_osf_getitimer: ::c_long = 86;
pub const SYS_gethostname: ::c_long = 87;
pub const SYS_sethostname: ::c_long = 88;
pub const SYS_getdtablesize: ::c_long = 89;
pub const SYS_dup2: ::c_long = 90;
pub const SYS_fstat: ::c_long = 91;
pub const SYS_fcntl: ::c_long = 92;
pub const SYS_osf_select: ::c_long = 93;
pub const SYS_poll: ::c_long = 94;
pub const SYS_fsync: ::c_long = 95;
pub const SYS_setpriority: ::c_long = 96;
pub const SYS_socket: ::c_long = 97;
pub const SYS_connect: ::c_long = 98;
pub const SYS_accept: ::c_long = 99;
pub const SYS_getpriority: ::c_long = 100;
pub const SYS_send: ::c_long = 101;
pub const SYS_recv: ::c_long = 102;
pub const SYS_sigreturn: ::c_long = 103;
pub const SYS_bind: ::c_long = 104;
pub const SYS_setsockopt: ::c_long = 105;
pub const SYS_listen: ::c_long = 106;
pub const SYS_osf_plock: ::c_long = 107;
pub const SYS_osf_old_sigvec: ::c_long = 108;
pub const SYS_osf_old_sigblock: ::c_long = 109;
pub const SYS_osf_old_sigsetmask: ::c_long = 110;
pub const SYS_sigsuspend: ::c_long = 111;
pub const SYS_osf_sigstack: ::c_long = 112;
pub const SYS_recvmsg: ::c_long = 113;
pub const SYS_sendmsg: ::c_long = 114;
pub const SYS_osf_old_vtrace: ::c_long = 115;
pub const SYS_osf_gettimeofday: ::c_long = 116;
pub const SYS_osf_getrusage: ::c_long = 117;
pub const SYS_getsockopt: ::c_long = 118;
pub const SYS_readv: ::c_long = 120;
pub const SYS_writev: ::c_long = 121;
pub const SYS_osf_settimeofday: ::c_long = 122;
pub const SYS_fchown: ::c_long = 123;
pub const SYS_fchmod: ::c_long = 124;
pub const SYS_recvfrom: ::c_long = 125;
pub const SYS_setreuid: ::c_long = 126;
pub const SYS_setregid: ::c_long = 127;
pub const SYS_rename: ::c_long = 128;
pub const SYS_truncate: ::c_long = 129;
pub const SYS_ftruncate: ::c_long = 130;
pub const SYS_flock: ::c_long = 131;
pub const SYS_setgid: ::c_long = 132;
pub const SYS_sendto: ::c_long = 133;
pub const SYS_shutdown: ::c_long = 134;
pub const SYS_socketpair: ::c_long = 135;
pub const SYS_mkdir: ::c_long = 136;
pub const SYS_rmdir: ::c_long = 137;
pub const SYS_osf_utimes:   ::c_long =  138;
pub const SYS_osf_old_sigreturn:    ::c_long =  139;
pub const SYS_osf_adjtime:  ::c_long =  140;
pub const SYS_getpeername:  ::c_long =  141;
pub const SYS_osf_gethostid:    ::c_long =  142;
pub const SYS_osf_sethostid:    ::c_long =  143;
pub const SYS_getrlimit:    ::c_long =  144;
pub const SYS_setrlimit:    ::c_long =  145;
pub const SYS_osf_old_killpg:   ::c_long =  146;
pub const SYS_setsid:   ::c_long =  147;
pub const SYS_quotactl: ::c_long =  148;
pub const SYS_osf_oldquota: ::c_long =  149;
pub const SYS_getsockname:  ::c_long =  150;
pub const SYS_osf_pid_block:    ::c_long =  153;
pub const SYS_osf_pid_unblock:  ::c_long =  154;
pub const SYS_sigaction:    ::c_long =  156;
pub const SYS_osf_sigwaitprim:  ::c_long =  157;
pub const SYS_osf_nfssvc:   ::c_long =  158;
pub const SYS_osf_getdirentries:    ::c_long =  159;
pub const SYS_osf_statfs:   ::c_long =  160;
pub const SYS_osf_fstatfs:  ::c_long =  161;
pub const SYS_osf_asynch_daemon:    ::c_long =  163;
pub const SYS_osf_getfh:    ::c_long =  164;
pub const SYS_osf_getdomainname:    ::c_long =  165;
pub const SYS_setdomainname:    ::c_long =  166;
pub const SYS_osf_exportfs: ::c_long =  169;
pub const SYS_membarrier: ::c_long =  172;
pub const SYS_osf_alt_plock:    ::c_long =  181;
pub const SYS_osf_getmnt:   ::c_long =  184;
pub const SYS_osf_alt_sigpending:   ::c_long =  187;
pub const SYS_osf_alt_setsid:   ::c_long =  188;
pub const SYS_osf_swapon:   ::c_long =  199;
pub const SYS_msgctl: ::c_long =  200;
pub const SYS_msgget: ::c_long =  201;
pub const SYS_msgrcv: ::c_long =  202;
pub const SYS_msgsnd: ::c_long =  203;
pub const SYS_semctl: ::c_long =  204;
pub const SYS_semget: ::c_long =  205;
pub const SYS_semop: ::c_long =  206;
pub const SYS_osf_utsname: ::c_long =  207;
pub const SYS_lchown: ::c_long =  208;
pub const SYS_osf_shmat: ::c_long =  209;
pub const SYS_shmctl: ::c_long =  210;
pub const SYS_shmdt: ::c_long =  211;
pub const SYS_shmget: ::c_long =  212;
pub const SYS_osf_mvalid: ::c_long =  213;
pub const SYS_osf_getaddressconf: ::c_long =  214;
pub const SYS_osf_msleep: ::c_long =  215;
pub const SYS_osf_mwakeup: ::c_long =  216;
pub const SYS_msync: ::c_long =  217;
pub const SYS_osf_signal: ::c_long =  218;
pub const SYS_osf_utc_gettime: ::c_long =  219;
pub const SYS_osf_utc_adjtime: ::c_long =  220;
pub const SYS_osf_security: ::c_long =  222;
pub const SYS_osf_kloadcall: ::c_long =  223;
pub const SYS_osf_stat: ::c_long =  224;
pub const SYS_osf_lstat: ::c_long =  225;
pub const SYS_osf_fstat: ::c_long =  226;
pub const SYS_osf_statfs64: ::c_long =  227;
pub const SYS_osf_fstatfs64: ::c_long =  228;
pub const SYS_getpgid: ::c_long =  233;
pub const SYS_getsid: ::c_long =  234;
pub const SYS_sigaltstack: ::c_long =  235;
pub const SYS_osf_waitid: ::c_long =  236;
pub const SYS_osf_priocntlset: ::c_long =  237;
pub const SYS_osf_sigsendset: ::c_long =  238;
pub const SYS_osf_set_speculative: ::c_long =  239;
pub const SYS_osf_msfs_syscall: ::c_long =  240;
pub const SYS_osf_sysinfo: ::c_long =  241;
pub const SYS_osf_uadmin: ::c_long =  242;
pub const SYS_osf_fuser: ::c_long =  243;
pub const SYS_osf_proplist_syscall: ::c_long =  244;
pub const SYS_osf_ntp_adjtime:  ::c_long =  245;
pub const SYS_osf_ntp_gettime:  ::c_long =  246;
pub const SYS_osf_pathconf: ::c_long =  247;
pub const SYS_osf_fpathconf: ::c_long =  248;
pub const SYS_osf_uswitch:  ::c_long =  250;
pub const SYS_osf_usleep_thread:    ::c_long =  251;
pub const SYS_osf_audcntl: ::c_long =  252;
pub const SYS_osf_audgen: ::c_long =  253;
pub const SYS_sysfs:  ::c_long =  254;
pub const SYS_osf_subsys_info: ::c_long =  255;
pub const SYS_osf_getsysinfo: ::c_long =  256;
pub const SYS_osf_setsysinfo: ::c_long =  257;
pub const SYS_osf_afs_syscall: ::c_long =  258;
pub const SYS_osf_swapctl: ::c_long =  259;
pub const SYS_osf_memcntl: ::c_long =  260;
pub const SYS_osf_fdatasync: ::c_long =  261;
pub const SYS_bdflush: ::c_long =  300;
pub const SYS_sethae: ::c_long =  301;
pub const SYS_mount:  ::c_long =  302;
pub const SYS_old_adjtimex: ::c_long =  303;
pub const SYS_swapoff: ::c_long = 304;
pub const SYS_getdents: ::c_long = 305;
pub const SYS_create_module: ::c_long = 306;
pub const SYS_init_module: ::c_long = 307;
pub const SYS_delete_module: ::c_long = 308;
pub const SYS_get_kernel_syms: ::c_long = 309;
pub const SYS_syslog: ::c_long = 310;
pub const SYS_reboot: ::c_long = 311;
pub const SYS_clone: ::c_long = 312;
pub const SYS_uselib: ::c_long = 313;
pub const SYS_mlock: ::c_long = 314;
pub const SYS_munlock: ::c_long = 315;
pub const SYS_mlockall: ::c_long = 316;
pub const SYS_munlockall: ::c_long = 317;
pub const SYS_sysinfo: ::c_long = 318;
pub const SYS__sysctl: ::c_long = 319;
pub const SYS_oldumount: ::c_long = 321;
pub const SYS_swapon: ::c_long = 322;
pub const SYS_times: ::c_long = 323;
pub const SYS_personality:  ::c_long =  324;
pub const SYS_setfsuid: ::c_long =  325;
pub const SYS_setfsgid: ::c_long =  326;
pub const SYS_ustat:    ::c_long =  327;
pub const SYS_statfs:   ::c_long =  328;
pub const SYS_fstatfs:  ::c_long =  329;
pub const SYS_sched_setparam:   ::c_long =  330;
pub const SYS_sched_getparam:   ::c_long =  331;
pub const SYS_sched_setscheduler:   ::c_long =  332;
pub const SYS_sched_getscheduler:   ::c_long =  333;
pub const SYS_sched_yield:  ::c_long =  334;
pub const SYS_sched_get_priority_max:   ::c_long =  335;
pub const SYS_sched_get_priority_min:   ::c_long =  336;
pub const SYS_sched_rr_get_interval:    ::c_long =  337;
pub const SYS_afs_syscall:  ::c_long =  338;
pub const SYS_uname:    ::c_long =  339;
pub const SYS_nanosleep:    ::c_long =  340;
pub const SYS_mremap:   ::c_long =  341;
pub const SYS_nfsservctl:   ::c_long =  342;
pub const SYS_setresuid:    ::c_long =  343;
pub const SYS_getresuid:    ::c_long =  344;
pub const SYS_pciconfig_read:   ::c_long =  345;
pub const SYS_pciconfig_write:  ::c_long =  346;
pub const SYS_query_module: ::c_long =  347;
pub const SYS_prctl:    ::c_long =  348;
pub const SYS_pread64:  ::c_long =  349;
pub const SYS_pwrite64: ::c_long =  350;
pub const SYS_rt_sigreturn: ::c_long =  351;
pub const SYS_rt_sigaction: ::c_long =  352;
pub const SYS_rt_sigprocmask:   ::c_long =  353;
pub const SYS_rt_sigpending:    ::c_long =  354;
pub const SYS_rt_sigtimedwait:  ::c_long =  355;
pub const SYS_rt_sigqueueinfo:  ::c_long =  356;
pub const SYS_rt_sigsuspend:    ::c_long =  357;
pub const SYS_select:   ::c_long =  358;
pub const SYS_gettimeofday: ::c_long =  359;
pub const SYS_settimeofday: ::c_long =  360;
pub const SYS_getitimer:    ::c_long =  361;
pub const SYS_setitimer:    ::c_long =  362;
pub const SYS_utimes:   ::c_long =  363;
pub const SYS_getrusage:    ::c_long =  364;
pub const SYS_wait4:    ::c_long =  365;
pub const SYS_adjtimex: ::c_long =  366;
pub const SYS_getcwd:   ::c_long =  367;
pub const SYS_capget:   ::c_long =  368;
pub const SYS_capset:   ::c_long =  369;
pub const SYS_sendfile: ::c_long =  370;
pub const SYS_setresgid:    ::c_long =  371;
pub const SYS_getresgid:    ::c_long =  372;
pub const SYS_dipc: ::c_long =  373;
pub const SYS_pivot_root:   ::c_long =  374;
pub const SYS_mincore:  ::c_long =  375;
pub const SYS_pciconfig_iobase: ::c_long =  376;
pub const SYS_getdents64:   ::c_long =  377;
pub const SYS_gettid:   ::c_long =  378;
pub const SYS_readahead:    ::c_long =  379;
pub const SYS_tkill:    ::c_long =  381;
pub const SYS_setxattr: ::c_long =  382;
pub const SYS_lsetxattr:    ::c_long =  383;
pub const SYS_fsetxattr:    ::c_long =  384;
pub const SYS_getxattr: ::c_long =  385;
pub const SYS_lgetxattr:    ::c_long =  386;
pub const SYS_fgetxattr:    ::c_long =  387;
pub const SYS_listxattr:    ::c_long =  388;
pub const SYS_llistxattr:   ::c_long =  389;
pub const SYS_flistxattr:   ::c_long =  390;
pub const SYS_removexattr:  ::c_long =  391;
pub const SYS_lremovexattr: ::c_long =  392;
pub const SYS_fremovexattr: ::c_long =  393;
pub const SYS_futex:    ::c_long =  394;
pub const SYS_sched_setaffinity:    ::c_long =  395;
pub const SYS_sched_getaffinity:    ::c_long =  396;
pub const SYS_tuxcall: ::c_long = 397;
pub const SYS_io_setup: ::c_long = 398;
pub const SYS_io_destroy: ::c_long = 399;
pub const SYS_io_getevents: ::c_long = 400;
pub const SYS_io_submit: ::c_long = 401;
pub const SYS_io_cancel: ::c_long = 402;
pub const SYS_exit_group: ::c_long = 405;
pub const SYS_lookup_dcookie: ::c_long = 406;
pub const SYS_epoll_create: ::c_long = 407;
pub const SYS_epoll_ctl: ::c_long = 408;
pub const SYS_epoll_wait: ::c_long = 409;
pub const SYS_sys_epoll_create: ::c_long = 407;
pub const SYS_sys_epoll_ctl: ::c_long = 408;
pub const SYS_sys_epoll_wait: ::c_long = 409;
pub const SYS_remap_file_pages: ::c_long =  410;
pub const SYS_set_tid_address:  ::c_long =  411;
pub const SYS_restart_syscall:  ::c_long =  412;
pub const SYS_fadvise64:    ::c_long =  413;
pub const SYS_timer_create: ::c_long =  414;
pub const SYS_timer_settime:    ::c_long =  415;
pub const SYS_timer_gettime:    ::c_long =  416;
pub const SYS_timer_getoverrun: ::c_long =  417;
pub const SYS_timer_delete: ::c_long =  418;
pub const SYS_clock_settime:    ::c_long =  419;
pub const SYS_clock_gettime:    ::c_long =  420;
pub const SYS_clock_getres: ::c_long =  421;
pub const SYS_clock_nanosleep:  ::c_long =  422;
pub const SYS_semtimedop:   ::c_long =  423;
pub const SYS_tgkill:   ::c_long =  424;
pub const SYS_stat64:   ::c_long =  425;
pub const SYS_lstat64:  ::c_long =  426;
pub const SYS_fstat64:  ::c_long =  427;
pub const SYS_vserver:  ::c_long =  428;
pub const SYS_mbind:    ::c_long =  429;
pub const SYS_get_mempolicy:    ::c_long =  430;
pub const SYS_set_mempolicy:    ::c_long =  431;
pub const SYS_mq_open:  ::c_long =  432;
pub const SYS_mq_unlink:    ::c_long =  433;
pub const SYS_mq_timedsend: ::c_long =  434;
pub const SYS_mq_timedreceive:  ::c_long =  435;
pub const SYS_mq_notify:    ::c_long =  436;
pub const SYS_mq_getsetattr:    ::c_long =  437;
pub const SYS_waitid:   ::c_long =  438;
pub const SYS_add_key:  ::c_long =  439;
pub const SYS_request_key:  ::c_long =  440;
pub const SYS_keyctl:   ::c_long =  441;
pub const SYS_ioprio_set: ::c_long = 442;
pub const SYS_ioprio_get: ::c_long = 443;
pub const SYS_inotify_init: ::c_long = 444;
pub const SYS_inotify_add_watch: ::c_long = 445;
pub const SYS_inotify_rm_watch: ::c_long = 446;
pub const SYS_fdatasync: ::c_long = 447;
pub const SYS_kexec_load: ::c_long = 448;
pub const SYS_migrate_pages: ::c_long = 449;
pub const SYS_openat: ::c_long = 450;
pub const SYS_mkdirat: ::c_long = 451;
pub const SYS_mknodat: ::c_long = 452;
pub const SYS_fchownat: ::c_long = 453;
pub const SYS_futimesat: ::c_long = 454;
pub const SYS_fstatat64: ::c_long = 455;
pub const SYS_unlinkat: ::c_long = 456;
pub const SYS_renameat: ::c_long = 457;
pub const SYS_linkat: ::c_long = 458;
pub const SYS_symlinkat: ::c_long = 459;
pub const SYS_readlinkat: ::c_long = 460;
pub const SYS_fchmodat: ::c_long = 461;
pub const SYS_faccessat: ::c_long = 462;
pub const SYS_pselect6: ::c_long = 463;
pub const SYS_ppoll: ::c_long = 464;
pub const SYS_unshare: ::c_long = 465;
pub const SYS_set_robust_list: ::c_long = 466;
pub const SYS_get_robust_list: ::c_long = 467;
pub const SYS_splice: ::c_long = 468;
pub const SYS_sync_file_range: ::c_long = 469;
pub const SYS_tee: ::c_long = 470;
pub const SYS_vmsplice: ::c_long = 471;
pub const SYS_move_pages: ::c_long = 472;
pub const SYS_getcpu: ::c_long = 473;
pub const SYS_epoll_pwait: ::c_long = 474;
pub const SYS_utimensat: ::c_long = 475;
pub const SYS_signalfd: ::c_long = 476;
pub const SYS_timerfd: ::c_long = 477;
pub const SYS_eventfd: ::c_long = 478;
pub const SYS_recvmmsg: ::c_long = 479;
pub const SYS_fallocate: ::c_long = 480;
pub const SYS_timerfd_create: ::c_long = 481;
pub const SYS_timerfd_settime: ::c_long = 482;
pub const SYS_timerfd_gettime: ::c_long = 483;
pub const SYS_signalfd4: ::c_long = 484;
pub const SYS_eventfd2: ::c_long = 485;
pub const SYS_epoll_create1: ::c_long = 486;
pub const SYS_dup3: ::c_long = 487;
pub const SYS_pipe2: ::c_long = 488;
pub const SYS_inotify_init1: ::c_long = 489;
pub const SYS_preadv: ::c_long = 490;
pub const SYS_pwritev: ::c_long = 491;
pub const SYS_rt_tgsigqueueinfo: ::c_long = 492;
pub const SYS_perf_event_open: ::c_long = 493;
pub const SYS_fanotify_init: ::c_long = 494;
pub const SYS_fanotify_mark: ::c_long = 495;
pub const SYS_prlimit64: ::c_long = 496;
pub const SYS_name_to_handle_at: ::c_long = 497;
pub const SYS_open_by_handle_at: ::c_long = 498;
pub const SYS_clock_adjtime: ::c_long = 499;
pub const SYS_syncfs: ::c_long = 500;
pub const SYS_setns: ::c_long = 501;
pub const SYS_accept4: ::c_long = 502;
pub const SYS_sendmmsg: ::c_long = 503;
pub const SYS_process_vm_readv: ::c_long = 504;
pub const SYS_process_vm_writev: ::c_long = 505;
pub const SYS_kcmp: ::c_long = 506;
pub const SYS_finit_module: ::c_long = 507;
pub const SYS_sched_setattr: ::c_long = 508;
pub const SYS_sched_getattr: ::c_long = 509;
pub const SYS_renameat2: ::c_long = 510;
pub const SYS_getrandom: ::c_long = 511;
pub const SYS_memfd_create: ::c_long = 512;
pub const SYS_execveat: ::c_long = 513;
pub const SYS_seccomp: ::c_long = 514;
pub const SYS_copy_file_range: ::c_long = 515;
pub const SYS_preadv2: ::c_long = 516;
pub const SYS_pwritev2: ::c_long = 517;
pub const SYS_statx: ::c_long = 518;
pub const SYS_clone3: ::c_long = 519; // CHENHB20220214

pub const SYS_pidfd_open: ::c_long = 50000;

pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 32;

#[link(name = "util")]
extern "C" {
    pub fn sysctl(
        name: *mut ::c_int,
        namelen: ::c_int,
        oldp: *mut ::c_void,
        oldlenp: *mut ::size_t,
        newp: *mut ::c_void,
        newlen: ::size_t,
    ) -> ::c_int;
}
