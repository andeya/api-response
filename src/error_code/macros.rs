use std::fmt::Display;

// re-export
pub use inventory;

use super::CodeSegment;
use crate::error_code::ApiErr;

/// Quickly create an `ApiError` builder `ApiErr` and collect error code mode
/// information.
#[macro_export]
macro_rules! api_err {
    ($intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::None, ::core::option::Option::None, ::core::option::Option::None)
        }
        $crate::error_code::ApiErr::new0().intro($intro)
    }};
    ($s1:expr, $intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::Some($s1), ::core::option::Option::None, ::core::option::Option::None)
        }
        $crate::error_code::ApiErr::new1($s1).intro($intro)
    }};
    ($s1:expr, $s2:expr, $intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::Some($s1), ::core::option::Option::Some($s2), ::core::option::Option::None)
        }
        $crate::error_code::ApiErr::new2($s1,$s2).intro($intro)
    }};
    ($s1:expr, $s2:expr, $s3:expr, $intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::Some($s1), ::core::option::Option::Some($s2), ::core::option::Option::Some($s3))
        }
        $crate::error_code::ApiErr::new3($s1,$s2,$s3).intro($intro)
    }};
}

/// Quickly create an `ApiError` builder `ApiErrX` and collect error code mode
/// information.
#[macro_export]
macro_rules! api_err_x {
    ($intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::None, ::core::option::Option::None, ::core::option::Option::None)
        }
        $crate::error_code::ApiErrX::new1().intro($intro)
    }};
    ($s1:expr, $intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::Some($s1), ::core::option::Option::None, ::core::option::Option::None)
        }
        $crate::error_code::ApiErrX::new2($s1).intro($intro)
    }};
    ($s1:expr, $s2:expr, $intro:expr) => {{
        $crate::error_code::inventory::submit! {
            $crate::error_code::ErrorCodeMode::new($intro, ::core::option::Option::Some($s1), ::core::option::Option::Some($s2), ::core::option::Option::None)
        }
        $crate::error_code::ApiErrX::new3($s1,$s2).intro($intro)
    }};
}

/// Obtain the list of error code modes.
pub fn error_code_modes() -> Vec<&'static ErrorCodeMode> {
    error_code_mode_iter().collect()
}

/// Obtain the iterator of error code modes.
pub fn error_code_mode_iter() -> impl Iterator<Item = &'static ErrorCodeMode> {
    inventory::iter::<ErrorCodeMode>.into_iter()
}

inventory::collect!(ErrorCodeMode);

#[derive(Debug)]
#[non_exhaustive]
pub struct ErrorCodeMode {
    pub intro: &'static str,
    pub segment1: Option<CodeSegment>,
    pub segment2: Option<CodeSegment>,
    pub segment3: Option<CodeSegment>,
}

impl ErrorCodeMode {
    pub const fn new(
        intro: &'static str,
        segment1: Option<CodeSegment>,
        segment2: Option<CodeSegment>,
        segment3: Option<CodeSegment>,
    ) -> Self {
        Self {
            intro,
            segment1,
            segment2,
            segment3,
        }
    }
}

impl Display for ErrorCodeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(
            &ApiErr {
                intro: self.intro,
                s1: self.segment1,
                s2: self.segment2,
                s3: self.segment3,
            },
            f,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::error_code::*;

    #[test]
    fn macro_api_err() {
        const C1: ApiErr = api_err!("macro_api_err");
        assert_eq!("[??******]: macro_api_err", C1.to_string());

        const C2: ApiErr = api_err!(S01, "macro_api_err");
        assert_eq!("[??01****]: macro_api_err", C2.to_string());

        const C3: ApiErr = api_err!(S01, S22, "macro_api_err");
        assert_eq!("[??0122**]: macro_api_err", C3.to_string());

        const C4: ApiErr = api_err!(S01, S22, S33, "macro_api_err");
        assert_eq!("[??012233]: macro_api_err", C4.to_string());

        const X1: ApiErrX = api_err_x!("macro_api_err");
        assert_eq!("[??******]: macro_api_err", X1.to_string());

        const X2: ApiErrX = api_err_x!(S01, "macro_api_err");
        assert_eq!("[??01****]: macro_api_err", X2.to_string());

        const X3: ApiErrX = api_err_x!(S01, S22, "macro_api_err");
        assert_eq!("[??0122**]: macro_api_err", X3.to_string());

        let s = format!("{:?}", error_code_modes());
        println!("{s}");
        assert_eq!(
            r##"[ErrorCodeMode { intro: "macro_api_err", segment1: Some(S01), segment2: Some(S22), segment3: None }, ErrorCodeMode { intro: "macro_api_err", segment1: Some(S01), segment2: None, segment3: None }, ErrorCodeMode { intro: "macro_api_err", segment1: None, segment2: None, segment3: None }, ErrorCodeMode { intro: "macro_api_err", segment1: Some(S01), segment2: Some(S22), segment3: Some(S33) }, ErrorCodeMode { intro: "macro_api_err", segment1: Some(S01), segment2: Some(S22), segment3: None }, ErrorCodeMode { intro: "macro_api_err", segment1: Some(S01), segment2: None, segment3: None }, ErrorCodeMode { intro: "macro_api_err", segment1: None, segment2: None, segment3: None }]"##,
            s
        );

        for error_code_mode in error_code_mode_iter() {
            println!("{error_code_mode}");
        }
    }
}
