#[test]
fn gen_code_enum() {
    api_response_macros::enum_digits!(CodeSegment, u8, 1, 4, S, 2);
    assert_eq!(Ok(CodeSegment::S01), CodeSegment::try_from(1));
    assert_eq!(Ok(CodeSegment::S02), CodeSegment::try_from(2));
    assert_eq!(Ok(CodeSegment::S03), CodeSegment::try_from(3));
    assert_eq!(Ok(CodeSegment::S04), CodeSegment::try_from(4));
    assert_eq!(Err(num_enum::TryFromPrimitiveError::new(5)), CodeSegment::try_from(5));
}

#[test]
fn err_path_types() {
    api_response_macros::err_path_types!();

    const ERR_PATH_ROOT: ErrPathRoot = X99("name1");
    const ERR_PATH_PARENT: ErrPathParent = ERR_PATH_ROOT.Y01("name2");
    const ERR_PATH: ErrPath = ERR_PATH_PARENT.Z20("name3");

    assert_eq!("X99(name1)", ERR_PATH_ROOT.path());
    assert_eq!("X99(name1)/Y01(name2)", ERR_PATH_PARENT.path());
    assert_eq!("X99(name1)/Y01(name2)/Z20(name3)", ERR_PATH.path());
    assert_eq!(99, ERR_PATH_ROOT.path_flag());
    assert_eq!(9901, ERR_PATH_PARENT.path_flag());
    assert_eq!(990120, ERR_PATH.path_flag());
}
