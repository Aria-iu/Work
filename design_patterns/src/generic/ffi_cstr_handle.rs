/*
    C语言中使用的字符串与Rust语言中使用的字符串有不同的行为：
        C语言的字符串是无终止的，而Rust语言的字符串会存储其长度。
        C语言的字符串可以包含任何任意的非零字节，而Rust的字符串必须是UTF-8。
        C语言的字符串使用 unsafe 的指针操作来访问和操作，而与Rust字符串的交互是通过安全方法进行的。

    Rust标准库提供了与Rust的 String 和 &str 相对应的C语言等价表示，称为 CString 和 &CStr ，
    这使得我们可以避免在C语言字符串和Rust字符串之间转换的复杂性和 unsafe 代码。
    &CStr 类型还允许我们使用借用数据，这意味着在Rust和C之间传递字符串是一个零成本的操作。
*/

pub mod unsafe_module {
    // other module content
    /// Log a message at the specified level.
    ///
    /// # Safety
    ///
    /// It is the caller's guarantee to ensure `msg`:
    ///
    /// - is not a null pointer
    /// - points to valid, initialized data
    /// - points to memory ending in a null byte
    /// - won't be mutated for the duration of this function call
    #[no_mangle]
    pub unsafe extern "C" fn mylib_log(
        msg: *const libc::c_char,
        level: libc::c_int
    ) {
        let level: crate::LogLevel = match level { /* ... */ };
        // SAFETY: The caller has already guaranteed this is okay (see the
        // `# Safety` section of the doc-comment).
        let msg_str: &str = match std::ffi::CStr::from_ptr(msg).to_str() {
            Ok(s) => s,
            Err(e) => {
                crate::log_error("FFI string conversion failed");
                return;
            }
        };
        crate::log(msg_str, level);
    }
}

/*
当向FFI函数传递字符串时，应该遵循四个原则：
    1. 使拥有的字符串的生命周期尽可能长。
    2. 在转换过程中尽量减少 unsafe 代码。
    3. 如果C代码可以修改字符串数据，使用 Vec 而不是 CString 。
    4. 除非外部函数API要求，否则字符串的所有权不应该转移给被调用者。

*/

pub mod unsafe_module {
    // other module content
    extern "C" {
        fn seterr(message: *const libc::c_char);
        fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    }
    fn report_error_to_ffi<S: Into<String>>(
        err: S
    ) -> Result<(), std::ffi::NulError>{
            let c_err = std::ffi::CString::new(err.into())?;
            unsafe {
                // SAFETY: calling an FFI whose documentation says the pointer is
                // const, so no modification should occur
                seterr(c_err.as_ptr());
            }
            Ok(())
            // The lifetime of c_err continues until here
    }
    fn get_error_from_ffi() -> Result<String, std::ffi::IntoStringError> {
        let mut buffer = vec![0u8; 1024];
        unsafe {
            // SAFETY: calling an FFI whose documentation implies
            // that the input need only live as long as the call
            let written: usize = geterr(buffer.as_mut_ptr(), 1023).into();
            buffer.truncate(written + 1);
        }
        std::ffi::CString::new(buffer).unwrap().into_string()
    }
}