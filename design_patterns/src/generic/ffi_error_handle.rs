/*
简单枚举转换为整数
*/
enum DatabaseError {
    IsReadOnly = 1, // user attempted a write operation
    IOError = 2, // user should read the C errno() for what it was
    FileCorrupted = 3, // user should run a repair tool to recover it
}
impl From<DatabaseError> for libc::c_int {
    fn from(e: DatabaseError) -> libc::c_int {
        (e as i8).into()
    }
}

/*
    结构化的枚举应该被转换为整数代码，并有一个字符串错误消息作为细节。
*/
pub mod errors {
    enum DatabaseError {
        IsReadOnly,
        IOError(std::io::Error),
        FileCorrupted(String), // message describing the issue
    }
    impl From<DatabaseError> for libc::c_int {
        fn from(e: DatabaseError) -> libc::c_int {
            match e {
                DatabaseError::IsReadOnly => 1,
                DatabaseError::IOError(_) => 2,
                DatabaseError::FileCorrupted(_) => 3,
            }
        }
    }
}
pub mod c_api {
    use super::errors::DatabaseError;
    #[no_mangle]
    pub extern "C" fn db_error_description(e: *const DatabaseError) -> *mut libc::c_char {
        let error: &DatabaseError = unsafe {
        // SAFETY: pointer lifetime is greater than the current stack frame
            &*e
        };
        let error_str: String = match error {
            DatabaseError::IsReadOnly => {
                format!("cannot write to read-only database");
            }
            DatabaseError::IOError(e) => {
                format!("I/O Error: {}", e);
            }
            DatabaseError::FileCorrupted(s) => {
                format!("File corrupted, run repair: {}", &s);
            }
        };
        let c_error = unsafe {
            // SAFETY: copying error_str to an allocated buffer with a NUL
            // character at the end
            let mut malloc: *mut u8 = libc::malloc(error_str.len() + 1) as *mut _;
            if malloc.is_null() {
                return std::ptr::null_mut();
            }
            let src = error_str.as_bytes().as_ptr();
            std::ptr::copy_nonoverlapping(src, malloc, error_str.len());
            std::ptr::write(malloc.add(error_str.len()), 0);
            malloc as *mut libc::c_char
        };
        c_error
    }
}

/*
自定义错误类型应该变得”透明“，用C表示。
*/
// 将自定义的struct ParserError转换为C结构的parser_error，在需要parse_error的地方直接使用（传入）ParserError即可
struct ParseError {
    expected: char,
    line: u32,
    ch: u16
}
impl ParseError { /* ... */ }
/* Create a second version which is exposed as a C structure */
#[repr(C)]
pub struct parse_error {
    pub expected: libc::c_char,
    pub line: u32,
    pub ch: u16
}
impl From<ParseError> for parse_error {
    fn from(e: ParseError) -> parse_error {
        let ParseError { expected, line, ch } = e;
        parse_error { expected, line, ch }
    }
}