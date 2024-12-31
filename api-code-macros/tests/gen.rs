#[test]
fn gen_code_enum() {
    api_code_macros::enum_digits!(CodeSegment, u8, 1, 4, S, 2);
    assert_eq!(Ok(CodeSegment::S01), CodeSegment::try_from(1));
    assert_eq!(Ok(CodeSegment::S02), CodeSegment::try_from(2));
    assert_eq!(Ok(CodeSegment::S03), CodeSegment::try_from(3));
    assert_eq!(Ok(CodeSegment::S04), CodeSegment::try_from(4));
    assert_eq!(Err(num_enum::TryFromPrimitiveError::new(5)), CodeSegment::try_from(5));
}

#[test]
fn err_path_types() {
    api_code_macros::err_path_types!();

    const ERR_ROOT_PATH: ErrRootPath = X99("name1");
    const ERR_PARENT_PATH: ErrParentPath = ERR_ROOT_PATH.Y01("name2");
    const ERR_PATH: ErrPath = ERR_PARENT_PATH.Z20("name3");

    assert_eq!("X99(name1)", ERR_ROOT_PATH.path());
    assert_eq!("X99(name1)/Y01(name2)", ERR_PARENT_PATH.path());
    assert_eq!("X99(name1)/Y01(name2)/Z20(name3)", ERR_PATH.path());
    assert_eq!(99, ERR_ROOT_PATH.path_flag());
    assert_eq!(9901, ERR_PARENT_PATH.path_flag());
    assert_eq!(990120, ERR_PATH.path_flag());
}
