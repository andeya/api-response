mod codes;
mod helper;
mod http_status;
pub use codes::*;
pub use helper::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiError;
    #[test]
    fn new_api_error() {
        assert_eq!(
            r##"ApiError { code: 12, message: "message", details: None }"##,
            format!("{:?}", ApiError::new(INVALID_ARGUMENT, "message"))
        );
    }
    #[test]
    fn code_segment() {
        assert_eq!(12010203, INVALID_ARGUMENT | S01 | S02 | S03)
    }
    #[test]
    #[should_panic]
    fn code_segment_overflow() {
        let _ = INVALID_ARGUMENT | S01 | S02 | S03 | S04 | S05;
    }
    #[test]
    fn api_err() {
        const BLOCK_ERROR0: ApiErr = ApiErr::new0();
        assert_eq!(
            r##"ApiError { code: 12, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR0.invalid_argument("message"))
        );
        const BLOCK_ERROR1: ApiErr = ApiErr::new1(S99);
        assert_eq!(
            r##"ApiError { code: 1299, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR1.invalid_argument("message"))
        );
        const BLOCK_ERROR2: ApiErr = ApiErr::new2(S99, S99);
        assert_eq!(
            r##"ApiError { code: 129999, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR2.invalid_argument("message"))
        );
        const BLOCK_ERROR3: ApiErr = ApiErr::new3(S99, S99, S99);
        assert_eq!(
            r##"ApiError { code: 12999999, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR3.invalid_argument("message"))
        );
        const BLOCK_ERROR4: ApiErr = ApiErr::new0().intro("this is intro");
        assert_eq!(r##"[********]: this is intro"##, BLOCK_ERROR4.to_string());
        const BLOCK_ERROR5: ApiErr = ApiErr::new1(S01).intro("this is intro");
        assert_eq!(r##"[**01****]: this is intro"##, BLOCK_ERROR5.to_string());
        const BLOCK_ERROR6: ApiErr = ApiErr::new2(S01, S02).intro("this is intro");
        assert_eq!(r##"[**0102**]: this is intro"##, BLOCK_ERROR6.to_string());
        const BLOCK_ERROR7: ApiErr = ApiErr::new3(S01, S02, S33).intro("this is intro");
        assert_eq!(r##"[**010233]: this is intro"##, BLOCK_ERROR7.to_string());
    }
    #[test]
    fn api_err_x() {
        const BLOCK_ERROR1: ApiErrX = ApiErrX::new1();
        assert_eq!(
            r##"ApiError { code: 1299, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR1.invalid_argument(S99, "message"))
        );
        const BLOCK_ERROR2: ApiErrX = ApiErrX::new2(S99);
        assert_eq!(
            r##"ApiError { code: 129999, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR2.invalid_argument(S99, "message"))
        );
        const BLOCK_ERROR3: ApiErrX = ApiErrX::new3(S99, S99);
        assert_eq!(
            r##"ApiError { code: 12999999, message: "message", details: None }"##,
            format!("{:?}", BLOCK_ERROR3.invalid_argument(S99, "message"))
        );
        const BLOCK_ERROR4: ApiErrX = ApiErrX::new1().intro("this is intro");
        assert_eq!(r##"[********]: this is intro"##, BLOCK_ERROR4.to_string());
        const BLOCK_ERROR5: ApiErrX = ApiErrX::new2(S11).intro("this is intro");
        assert_eq!(r##"[**11****]: this is intro"##, BLOCK_ERROR5.to_string());
        const BLOCK_ERROR6: ApiErrX = ApiErrX::new3(S11, S02).intro("this is intro");
        assert_eq!(r##"[**1102**]: this is intro"##, BLOCK_ERROR6.to_string());
    }
}
