use std::{
    collections::{BTreeMap, HashSet},
    thread::LocalKey,
};

// re-export
pub use inventory;
use serde::{Deserialize, Serialize};

use super::{ErrDecl, ErrPath, ErrPathParent, ErrPathRoot, ErrType};

/// Quickly create an `ApiError` builder `ApiErr` and collect error code mode
/// information.
#[macro_export]
macro_rules! api_err {
    ($err_decl:expr) => {{
        $crate::error_code::tally::inventory::submit! {
            $err_decl
        }
        $err_decl.api_error()
    }};
    ($err_type:expr, & $local_key_err_path:expr) => {{
        $crate::error_code::tally::inventory::submit! {
            $crate::error_code::tally::LocalKeyErrDecl::new($err_type, &$local_key_err_path)
        }
        $err_type | &$local_key_err_path
    }};
    ($err_type:expr, $new_text:expr, & $local_key_err_path:expr) => {{
        $crate::error_code::tally::inventory::submit! {
            $crate::error_code::tally::LocalKeyErrDecl::new($err_type.with_text($new_text), &$local_key_err_path)
        }
        ($err_type | $new_text) | &$local_key_err_path
    }};

    ($err_type:expr, $err_path:expr) => {{
        $crate::error_code::tally::inventory::submit! {
            $err_type.declare($err_path)
        }
        $err_type | &$err_path
    }};
    ($err_type:expr, $new_text:expr, $err_path:expr) => {{
        $crate::error_code::tally::inventory::submit! {
            $err_type.with_text($new_text).declare($err_path)
        }
        ($err_type | $new_text) | &$err_path
    }};
}

#[non_exhaustive]
pub struct LocalKeyErrDecl {
    err_type: ErrType,
    err_path: &'static LocalKey<ErrPath>,
}
impl LocalKeyErrDecl {
    pub const fn new(err_type: ErrType, err_path: &'static LocalKey<ErrPath>) -> Self {
        Self { err_type, err_path }
    }
}

inventory::collect!(ErrDecl);
inventory::collect!(LocalKeyErrDecl);

/// Obtain the list of error code declaration.
pub fn tally_err_decl() -> ErrDeclTally {
    let total = inventory::iter::<ErrDecl>
        .into_iter()
        .map(ToOwned::to_owned)
        .chain(
            inventory::iter::<LocalKeyErrDecl>
                .into_iter()
                .map(|v| v.err_type + v.err_path),
        )
        .collect();
    ErrDeclTally { total }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ErrDeclTally {
    total: Vec<ErrDecl>,
}

pub type ErrDeclTree =
    BTreeMap<ErrPathRoot, BTreeMap<ErrPathParent, BTreeMap<ErrPath, BTreeMap<ErrType, HashSet<ErrDecl>>>>>;

pub type ErrDeclTreeText = BTreeMap<String, BTreeMap<String, BTreeMap<String, BTreeMap<String, HashSet<String>>>>>;

#[derive(Serialize, Deserialize)]
pub struct KV<T> {
    pub key: String,
    pub value: T,
}

pub type ErrDeclVecText = Vec<KV<Vec<KV<Vec<KV<Vec<KV<HashSet<String>>>>>>>>>;

impl ErrDeclTally {
    pub const fn total(&self) -> &Vec<ErrDecl> {
        &self.total
    }
    pub fn unique(&self) -> Vec<ErrDecl> {
        let mut seen = HashSet::new();
        let mut unique = self.total.clone();
        unique.retain(|v| seen.insert(v.to_string()));
        unique
    }
    pub fn tree(&self) -> ErrDeclTree {
        let mut b_tree_map: ErrDeclTree = BTreeMap::new();
        let unique = self.unique();
        for ele in unique {
            let curr = ele.err_path();
            let parent = curr.parent();
            let root = parent.root();
            let typ = ele.err_type();
            b_tree_map
                .entry(root)
                .or_default()
                .entry(parent)
                .or_default()
                .entry(*curr)
                .or_default()
                .entry(*typ)
                .or_default()
                .insert(ele);
        }
        b_tree_map
    }
    pub fn text_tree(&self) -> ErrDeclTreeText {
        let mut b_tree_map: ErrDeclTreeText = BTreeMap::new();
        let unique = self.unique();
        for ele in unique {
            let curr = ele.err_path();
            let parent = curr.parent();
            let root = parent.root();
            let brief = ele.extract();
            b_tree_map
                .entry(format!("X{:02}({})", root.flag(), root.name()))
                .or_default()
                .entry(format!("Y{:02}({})", parent.flag(), parent.name()))
                .or_default()
                .entry(format!("Z{:02}({})", curr.flag(), curr.name()))
                .or_default()
                .entry(format!("ErrCode({})", brief.code()))
                .or_default()
                .insert(brief.message().to_owned());
        }
        b_tree_map
    }
    pub fn json(&self) -> String {
        unsafe { serde_json::to_string_pretty(&self.text_tree()).unwrap_unchecked() }
    }
    pub fn text_vec(&self) -> ErrDeclVecText {
        self.text_tree()
            .iter()
            .map(|(k, v)| KV {
                key: k.clone(),
                value: v
                    .iter()
                    .map(|(k, v)| KV {
                        key: k.clone(),
                        value: v
                            .iter()
                            .map(|(k, v)| KV {
                                key: k.clone(),
                                value: v
                                    .iter()
                                    .map(|(k, v)| KV {
                                        key: k.clone(),
                                        value: v.clone(),
                                    })
                                    .collect(),
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect()
    }
    pub fn xml(&self) -> String {
        let mut writer = String::new();
        let mut ser = quick_xml::se::Serializer::with_root(&mut writer, Some("ErrorDeclarations")).unwrap();
        ser.indent(' ', 2);
        self.text_vec().serialize(ser).unwrap();
        writer
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ApiError,
        error_code::{
            ErrDecl, ErrFlag, ErrPath, ErrPathParent, ErrPathRoot, ErrType, X00, X01,
            tally::{ErrDeclTally, tally_err_decl},
        },
    };

    #[test]
    fn macro_api_err() {
        const ET: ErrType = ErrType::new(ErrFlag::E100, "The operation was cancelled.");
        const EP_LV1: ErrPathRoot = X00("product");
        const EP_LV2: ErrPathParent = EP_LV1.Y01("system");
        const EP_LV3: ErrPath = EP_LV2.Z20("module");
        const EC: ErrDecl = ErrDecl::new(ET, EP_LV3);

        let ae0: ApiError = api_err!(EC);
        assert_eq!("The operation was cancelled. Code(100000120)", ae0.to_string());
        let ae1: ApiError = api_err!(ET, EP_LV3);
        let _ = api_err!(ET, EP_LV3);
        assert_eq!("The operation was cancelled. Code(100000120)", ae1.to_string());
        let ae2: ApiError = api_err!(ET, "This is new message.", EP_LV3);
        assert_eq!("This is new message. Code(100000120)", ae2.to_string());

        thread_local! {static EP_LV3_1:ErrPath=X01("product-2").Y01("system-2").Z02("module-2")}
        let ae3: ApiError = api_err!(ET, &EP_LV3_1);
        assert_eq!("The operation was cancelled. Code(100010102)", ae3.to_string());
        let ae4: ApiError = api_err!(ET, "This is new message-2.", &EP_LV3_1);
        assert_eq!("This is new message-2. Code(100010102)", ae4.to_string());

        let s = format!("{:?}", tally_err_decl());
        println!("{s}");

        let tally: ErrDeclTally = tally_err_decl();
        for err_decl in tally.unique() {
            println!("{err_decl}");
        }
        assert_eq!(tally.unique().len(), 4);
        assert_eq!(tally.total().len(), 6);
        println!("{}", tally.json());
        println!("{}", tally.xml());
    }
}
