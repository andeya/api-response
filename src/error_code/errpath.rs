api_response_macros::err_path_types!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn useage() {
        const ERR_PATH_ROOT: ErrPathRoot = X99("name1");
        const ERR_PATH_PARENT: ErrPathParent = ERR_PATH_ROOT.Y01("name2");
        const ERR_PATH: ErrPath = ERR_PATH_PARENT.Z20("name3");

        assert_eq!("X99(name1)", ERR_PATH_ROOT.path());
        assert_eq!("X99(name1)/Y01(name2)", ERR_PATH_PARENT.path());
        assert_eq!("X99(name1)/Y01(name2)/Z20(name3)", ERR_PATH.path());
    }
}
