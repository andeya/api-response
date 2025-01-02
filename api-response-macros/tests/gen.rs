#[test]
fn gen_code_enum() {
    api_response_macros::enum_digits!(CodeSegment, u8, 1, 4, S, 2);
    assert_eq!(Ok(CodeSegment::S01), CodeSegment::try_from(1));
    assert_eq!(Ok(CodeSegment::S02), CodeSegment::try_from(2));
    assert_eq!(Ok(CodeSegment::S03), CodeSegment::try_from(3));
    assert_eq!(Ok(CodeSegment::S04), CodeSegment::try_from(4));
    assert_eq!(Err(num_enum::TryFromPrimitiveError::new(5)), CodeSegment::try_from(5));
}
