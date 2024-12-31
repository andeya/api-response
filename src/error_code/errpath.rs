api_response_macros::err_path_types!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn useage() {
        const ERR_ROOT_PATH: ErrRootPath = X99("name1");
        const ERR_PARENT_PATH: ErrParentPath = ERR_ROOT_PATH.Y01("name2");
        const ERR_PATH: ErrPath = ERR_PARENT_PATH.Z20("name3");

        assert_eq!("X99(name1)", ERR_ROOT_PATH.path());
        assert_eq!("X99(name1)/Y01(name2)", ERR_PARENT_PATH.path());
        assert_eq!("X99(name1)/Y01(name2)/Z20(name3)", ERR_PATH.path());
    }
}
