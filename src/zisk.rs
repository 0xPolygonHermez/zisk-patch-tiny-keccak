extern "C" {
    fn syscall_keccak_f(w: *mut u64);
}

/// This function is a wrapper around the syscall to call the keccakf function
#[inline]
pub fn keccakf(state: &mut [u64; 25]) {
    unsafe {
        syscall_keccak_f(state.as_mut_ptr());
    }
}
