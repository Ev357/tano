pub fn suspend() {
    unsafe {
        libc::kill(0, libc::SIGSTOP);
    }
}
