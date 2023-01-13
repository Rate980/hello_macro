extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, QSelf, Type,
    TypeArray, TypeGroup, TypePath, TypePtr, TypeSlice, TypeTuple,
};

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 操作可能な構文木としてのRustコードの表現を構築する
    let ast = syn::parse(input).unwrap();

    // トレイトの実装内容を構築
    impl_hello_macro(&ast)
}

#[inline]
fn type_to_string(t: Type) -> String {
    match t {
        Type::Array(t) => array_to_string(t),
        Type::BareFn(_) => "".into(),
        Type::Group(t) => group_to_string(t),
        Type::ImplTrait(_) => "".into(),
        Type::Infer(_) => "_".into(),
        Type::Macro(_) => "".into(),
        Type::Never(_) => "!".into(),
        Type::Paren(t) => paren_to_string(t),
        Type::Path(t) => type_path_to_string(t),
        Type::Ptr(t) => ptr_to_string(t),
        Type::Reference(t) => reference_to_string(t),
        Type::Slice(t) => slice_to_string(t),
        Type::TraitObject(_) => "".into(),
        Type::Tuple(t) => tuple_to_string(t),
        Type::Verbatim(_) => "".into(),
        _ => todo!(),
    }
}

#[inline]
fn reference_to_string(t: syn::TypeReference) -> String {
    type_to_string(*t.elem)
}

#[inline]
fn paren_to_string(t: syn::TypeParen) -> String {
    type_to_string(*t.elem)
}

#[inline]
fn array_to_string(t: TypeArray) -> String {
    format!("Array<{}>", type_to_string(*t.elem))
}
#[inline]
fn tuple_to_string(t: TypeTuple) -> String {
    let mut ret = "[".to_string();
    for x in t.elems {
        ret += &format!("{},", type_to_string(x))
    }
    ret[..ret.len() - 1].to_string() + "]"
}

fn path_arguments_to_string(t: AngleBracketedGenericArguments) -> String {
    let mut ret = "".into();
    for x in t.args {
        if let GenericArgument::Type(x) = x {
            ret += &*format!("{},", type_to_string(x));
        }
    }
    ret
}

#[inline]
fn path_to_string(t: Path) -> String {
    let path_segment = t.segments[t.segments.len() - 1].clone();
    println!("{}", (path_segment.ident));
    let ident = match &*path_segment.ident.to_string() {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "isize" | "f32" | "f64" => "number",
        "Vec" | "HashSet" | "BTreeSet" => "Array",
        "String" | "str" => "string",
        "HashMap" | "BTreeMap" => "Record",
        x => x,
    }
    .to_string();
    if let PathArguments::AngleBracketed(t) = path_segment.arguments {
        let arg = path_arguments_to_string(t);
        if arg.len() != 0 {
            return format!("{}<{}>", ident, arg[..arg.len() - 1].to_string());
        }
    }
    ident
}

#[inline]
fn qself_to_string(t: QSelf) -> String {
    type_to_string(*t.ty)
}

#[inline]
fn type_path_to_string(t: TypePath) -> String {
    let t = dbg!(t);
    format!("{}", path_to_string(t.path))
}

#[inline]
fn slice_to_string(t: TypeSlice) -> String {
    format!("{}[]", type_to_string(*t.elem))
}
#[inline]
fn ptr_to_string(t: TypePtr) -> String {
    type_to_string(*t.elem)
}
#[inline]
fn group_to_string(t: TypeGroup) -> String {
    type_to_string(*t.elem)
}

fn types_fn(ast: syn::ItemFn) {
    // println!("{}", ast.sig.ident.to_string());
    println!(
        "{}",
        if let syn::ReturnType::Type(_, t) = ast.sig.output {
            type_to_string(*t)
        } else {
            "".into()
        }
    );
}
fn types_struct(_ast: syn::ItemStruct) {
    // dbg!(ast);
}

#[proc_macro_attribute]
pub fn types(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ret = item.clone();
    if let Ok(ast) = syn::parse(item.clone()) {
        types_fn(ast);
        return ret;
    }
    types_struct(syn::parse(item).unwrap());
    ret
}
