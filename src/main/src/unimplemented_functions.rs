fn unimplemented() {
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {unimplemented()}
#[lang = "eh_personality"] extern fn eh_personality() {unimplemented()}
#[lang = "panic_fmt"] fn panic_fmt() -> ! {unimplemented(); loop{}}


#[no_mangle] pub extern fn __ctype_b_loc() {unimplemented()}
#[no_mangle] pub extern fn __errno_location() {unimplemented()}
#[no_mangle] pub extern fn __rawmemchr() {unimplemented()}
#[no_mangle] pub extern fn __xpg_strerror_r() {unimplemented()}
#[no_mangle] pub extern fn _exit() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_Backtrace() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_DeleteException() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_FindEnclosingFunction() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_GetIP() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_GetIPInfo() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_GetLanguageSpecificData() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_GetRegionStart() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_RaiseException() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_Resume() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_SetGR() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn _Unwind_SetIP() {unimplemented()}
#[allow(non_snake_case)] 
#[no_mangle] pub extern fn abort() {unimplemented()}
#[no_mangle] pub extern fn accept() {unimplemented()}
#[no_mangle] pub extern fn acos() {unimplemented()}
#[no_mangle] pub extern fn acosf() {unimplemented()}
#[no_mangle] pub extern fn asin() {unimplemented()}
#[no_mangle] pub extern fn asinf() {unimplemented()}
#[no_mangle] pub extern fn atan() {unimplemented()}
#[no_mangle] pub extern fn atan2() {unimplemented()}
#[no_mangle] pub extern fn atan2f() {unimplemented()}
#[no_mangle] pub extern fn atanf() {unimplemented()}
#[no_mangle] pub extern fn atexit() {unimplemented()}
#[no_mangle] pub extern fn bind() {unimplemented()}
#[no_mangle] pub extern fn bsearch() {unimplemented()}
#[no_mangle] pub extern fn cbrt() {unimplemented()}
#[no_mangle] pub extern fn cbrtf() {unimplemented()}
#[no_mangle] pub extern fn ceil() {unimplemented()}
#[no_mangle] pub extern fn ceilf() {unimplemented()}
#[no_mangle] pub extern fn chdir() {unimplemented()}
#[no_mangle] pub extern fn chmod() {unimplemented()}
#[no_mangle] pub extern fn clock_gettime() {unimplemented()}
#[no_mangle] pub extern fn close() {unimplemented()}
#[no_mangle] pub extern fn closedir() {unimplemented()}
#[no_mangle] pub extern fn connect() {unimplemented()}
#[no_mangle] pub extern fn cos() {unimplemented()}
#[no_mangle] pub extern fn cosf() {unimplemented()}
#[no_mangle] pub extern fn cosh() {unimplemented()}
#[no_mangle] pub extern fn coshf() {unimplemented()}
#[no_mangle] pub extern fn creat() {unimplemented()}
#[no_mangle] pub extern fn dl_iterate_phdr() {unimplemented()}
#[no_mangle] pub extern fn dlclose() {unimplemented()}
#[no_mangle] pub extern fn dlerror() {unimplemented()}
#[no_mangle] pub extern fn dlopen() {unimplemented()}
#[no_mangle] pub extern fn dlsym() {unimplemented()}
#[no_mangle] pub extern fn dup() {unimplemented()}
#[no_mangle] pub extern fn dup2() {unimplemented()}
#[no_mangle] pub extern fn environ() {unimplemented()}
#[no_mangle] pub extern fn execvp() {unimplemented()}
#[no_mangle] pub extern fn exit() {unimplemented()}
#[no_mangle] pub extern fn exp() {unimplemented()}
#[no_mangle] pub extern fn exp2() {unimplemented()}
#[no_mangle] pub extern fn exp2f() {unimplemented()}
#[no_mangle] pub extern fn expf() {unimplemented()}
#[no_mangle] pub extern fn expm1() {unimplemented()}
#[no_mangle] pub extern fn expm1f() {unimplemented()}
#[no_mangle] pub extern fn fcntl() {unimplemented()}
#[no_mangle] pub extern fn fdatasync() {unimplemented()}
#[no_mangle] pub extern fn fdim() {unimplemented()}
#[no_mangle] pub extern fn fdimf() {unimplemented()}
#[no_mangle] pub extern fn floor() {unimplemented()}
#[no_mangle] pub extern fn floorf() {unimplemented()}
#[no_mangle] pub extern fn fma() {unimplemented()}
#[no_mangle] pub extern fn fmaf() {unimplemented()}
#[no_mangle] pub extern fn fmax() {unimplemented()}
#[no_mangle] pub extern fn fmaxf() {unimplemented()}
#[no_mangle] pub extern fn fmin() {unimplemented()}
#[no_mangle] pub extern fn fminf() {unimplemented()}
#[no_mangle] pub extern fn fmod() {unimplemented()}
#[no_mangle] pub extern fn fmodf() {unimplemented()}
#[no_mangle] pub extern fn fork() {unimplemented()}
#[no_mangle] pub extern fn freeaddrinfo() {unimplemented()}
#[no_mangle] pub extern fn frexp() {unimplemented()}
#[no_mangle] pub extern fn frexpf() {unimplemented()}
#[no_mangle] pub extern fn fstat() {unimplemented()}
#[no_mangle] pub extern fn fsync() {unimplemented()}
#[no_mangle] pub extern fn ftruncate() {unimplemented()}
#[no_mangle] pub extern fn gai_strerror() {unimplemented()}
#[no_mangle] pub extern fn getaddrinfo() {unimplemented()}
#[no_mangle] pub extern fn getcwd() {unimplemented()}
#[no_mangle] pub extern fn getenv() {unimplemented()}
#[no_mangle] pub extern fn getnameinfo() {unimplemented()}
#[no_mangle] pub extern fn getpagesize() {unimplemented()}
#[no_mangle] pub extern fn getpeername() {unimplemented()}
#[no_mangle] pub extern fn getpid() {unimplemented()}
#[no_mangle] pub extern fn getpwuid_r() {unimplemented()}
#[no_mangle] pub extern fn getsockname() {unimplemented()}
#[no_mangle] pub extern fn gettimeofday() {unimplemented()}
#[no_mangle] pub extern fn getuid() {unimplemented()}
#[no_mangle] pub extern fn hypot() {unimplemented()}
#[no_mangle] pub extern fn hypotf() {unimplemented()}
#[no_mangle] pub extern fn ioctl() {unimplemented()}
#[no_mangle] pub extern fn kill() {unimplemented()}
#[no_mangle] pub extern fn ldexp() {unimplemented()}
#[no_mangle] pub extern fn ldexpf() {unimplemented()}
#[no_mangle] pub extern fn link() {unimplemented()}
#[no_mangle] pub extern fn listen() {unimplemented()}
#[no_mangle] pub extern fn log() {unimplemented()}
#[no_mangle] pub extern fn log1p() {unimplemented()}
#[no_mangle] pub extern fn log1pf() {unimplemented()}
#[no_mangle] pub extern fn log2() {unimplemented()}
#[no_mangle] pub extern fn log2f() {unimplemented()}
#[no_mangle] pub extern fn log10() {unimplemented()}
#[no_mangle] pub extern fn log10f() {unimplemented()}
#[no_mangle] pub extern fn logf() {unimplemented()}
#[no_mangle] pub extern fn lseek() {unimplemented()}
#[no_mangle] pub extern fn lstat() {unimplemented()}
#[no_mangle] pub extern fn madvise() {unimplemented()}
#[no_mangle] pub extern fn mkdir() {unimplemented()}
#[no_mangle] pub extern fn mmap() {unimplemented()}
#[no_mangle] pub extern fn munmap() {unimplemented()}
#[no_mangle] pub extern fn nanosleep() {unimplemented()}
#[no_mangle] pub extern fn nextafter() {unimplemented()}
#[no_mangle] pub extern fn nextafterf() {unimplemented()}
#[no_mangle] pub extern fn open() {unimplemented()}
#[no_mangle] pub extern fn opendir() {unimplemented()}
#[no_mangle] pub extern fn pathconf() {unimplemented()}
#[no_mangle] pub extern fn pipe() {unimplemented()}
#[no_mangle] pub extern fn pow() {unimplemented()}
#[no_mangle] pub extern fn powf() {unimplemented()}
#[no_mangle] pub extern fn prctl() {unimplemented()}
#[no_mangle] pub extern fn pthread_atfork() {unimplemented()}
#[no_mangle] pub extern fn pthread_attr_destroy() {unimplemented()}
#[no_mangle] pub extern fn pthread_attr_getguardsize() {unimplemented()}
#[no_mangle] pub extern fn pthread_attr_getstack() {unimplemented()}
#[no_mangle] pub extern fn pthread_attr_init() {unimplemented()}
#[no_mangle] pub extern fn pthread_attr_setstacksize() {unimplemented()}
#[no_mangle] pub extern fn pthread_cond_broadcast() {unimplemented()}
#[no_mangle] pub extern fn pthread_cond_destroy() {unimplemented()}
#[no_mangle] pub extern fn pthread_cond_signal() {unimplemented()}
#[no_mangle] pub extern fn pthread_cond_timedwait() {unimplemented()}
#[no_mangle] pub extern fn pthread_cond_wait() {unimplemented()}
#[no_mangle] pub extern fn pthread_create() {unimplemented()}
#[no_mangle] pub extern fn pthread_detach() {unimplemented()}
#[no_mangle] pub extern fn pthread_getattr_np() {unimplemented()}
#[no_mangle] pub extern fn pthread_getspecific() {unimplemented()}
#[no_mangle] pub extern fn pthread_join() {unimplemented()}
#[no_mangle] pub extern fn pthread_key_create() {unimplemented()}
#[no_mangle] pub extern fn pthread_key_delete() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutex_destroy() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutex_init() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutex_lock() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutex_trylock() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutex_unlock() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutexattr_destroy() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutexattr_init() {unimplemented()}
#[no_mangle] pub extern fn pthread_mutexattr_settype() {unimplemented()}
#[no_mangle] pub extern fn pthread_rwlock_destroy() {unimplemented()}
#[no_mangle] pub extern fn pthread_rwlock_rdlock() {unimplemented()}
#[no_mangle] pub extern fn pthread_rwlock_tryrdlock() {unimplemented()}
#[no_mangle] pub extern fn pthread_rwlock_trywrlock() {unimplemented()}
#[no_mangle] pub extern fn pthread_rwlock_unlock() {unimplemented()}
#[no_mangle] pub extern fn pthread_rwlock_wrlock() {unimplemented()}
#[no_mangle] pub extern fn pthread_self() {unimplemented()}
#[no_mangle] pub extern fn pthread_setspecific() {unimplemented()}
#[no_mangle] pub extern fn raise() {unimplemented()}
#[no_mangle] pub extern fn read() {unimplemented()}
#[no_mangle] pub extern fn readdir_r() {unimplemented()}
#[no_mangle] pub extern fn readlink() {unimplemented()}
#[no_mangle] pub extern fn recvfrom() {unimplemented()}
#[no_mangle] pub extern fn rename() {unimplemented()}
#[no_mangle] pub extern fn rmdir() {unimplemented()}
#[no_mangle] pub extern fn round() {unimplemented()}
#[no_mangle] pub extern fn roundf() {unimplemented()}
#[no_mangle] pub extern fn sbrk() {unimplemented()}
#[no_mangle] pub extern fn sched_yield() {unimplemented()}
#[no_mangle] pub extern fn send() {unimplemented()}
#[no_mangle] pub extern fn sendto() {unimplemented()}
#[no_mangle] pub extern fn setenv() {unimplemented()}
#[no_mangle] pub extern fn setgid() {unimplemented()}
#[no_mangle] pub extern fn setgroups() {unimplemented()}
#[no_mangle] pub extern fn setsid() {unimplemented()}
#[no_mangle] pub extern fn setsockopt() {unimplemented()}
#[no_mangle] pub extern fn setuid() {unimplemented()}
#[no_mangle] pub extern fn shutdown() {unimplemented()}
#[no_mangle] pub extern fn sigaction() {unimplemented()}
#[no_mangle] pub extern fn sigaltstack() {unimplemented()}
#[no_mangle] pub extern fn sigemptyset() {unimplemented()}
#[no_mangle] pub extern fn signal() {unimplemented()}
#[no_mangle] pub extern fn sigprocmask() {unimplemented()}
#[no_mangle] pub extern fn sin() {unimplemented()}
#[no_mangle] pub extern fn sinf() {unimplemented()}
#[no_mangle] pub extern fn sinh() {unimplemented()}
#[no_mangle] pub extern fn sinhf() {unimplemented()}
#[no_mangle] pub extern fn snprintf() {unimplemented()}
#[no_mangle] pub extern fn socket() {unimplemented()}
#[no_mangle] pub extern fn stat() {unimplemented()}
#[no_mangle] pub extern fn strchr() {unimplemented()}
#[no_mangle] pub extern fn strcmp() {unimplemented()}
#[no_mangle] pub extern fn strerror_r() {unimplemented()}
#[no_mangle] pub extern fn strlen() {unimplemented()}
#[no_mangle] pub extern fn strncmp() {unimplemented()}
#[no_mangle] pub extern fn strncpy() {unimplemented()}
#[no_mangle] pub extern fn strnlen() {unimplemented()}
#[no_mangle] pub extern fn strtol() {unimplemented()}
#[no_mangle] pub extern fn symlink() {unimplemented()}
#[no_mangle] pub extern fn syscall() {unimplemented()}
#[no_mangle] pub extern fn sysconf() {unimplemented()}
#[no_mangle] pub extern fn tan() {unimplemented()}
#[no_mangle] pub extern fn tanf() {unimplemented()}
#[no_mangle] pub extern fn tanh() {unimplemented()}
#[no_mangle] pub extern fn tanhf() {unimplemented()}
#[no_mangle] pub extern fn trunc() {unimplemented()}
#[no_mangle] pub extern fn truncf() {unimplemented()}
#[no_mangle] pub extern fn unlink() {unimplemented()}
#[no_mangle] pub extern fn unsetenv() {unimplemented()}
#[no_mangle] pub extern fn utimes() {unimplemented()}
#[no_mangle] pub extern fn waitpid() {unimplemented()}
#[no_mangle] pub extern fn write() {unimplemented()}