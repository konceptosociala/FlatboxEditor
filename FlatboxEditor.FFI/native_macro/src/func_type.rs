use quote::ToTokens;
use syn::{Type, TypePath};

#[derive(Debug)]
pub(crate) enum FuncType {
    Unit,
    Primitive,
    StringRef,
    Object,
    ObjectRef,
    ObjectRefMut,
    FnRef,
}

impl TryFrom<&Type> for FuncType {
    type Error = String;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        use FuncType::*;

        match value {
            Type::Path(type_path) => {
                if is_primitive(type_path) {
                    return Ok(Primitive);
                }

                Ok(Object)
            },
            Type::Reference(type_ref) => {
                if let Type::Path(ref ref_elem_type) = *type_ref.elem {
                    if ref_elem_type.path.is_ident("str") {
                        return Ok(StringRef)
                    }
                }

                if type_ref.mutability.is_some() {
                    Ok(ObjectRefMut)
                } else {
                    Ok(ObjectRef)
                }
            },
            Type::BareFn(_) => {
                Ok(FnRef)
            },
            Type::Tuple(tuple) if tuple.elems.is_empty() => {
                Ok(Unit)
            },
            _ => Err(format!("Wrong type `{}`: Non-path and non-reference return types are not supported", value.to_token_stream())),
        }
    }
}

pub(crate) fn is_primitive(type_path: &TypePath) -> bool {
    type_path.path.is_ident("i8")
        || type_path.path.is_ident("i16")
        || type_path.path.is_ident("i32")
        || type_path.path.is_ident("i64")
        || type_path.path.is_ident("i128")
        || type_path.path.is_ident("u8")
        || type_path.path.is_ident("u16")
        || type_path.path.is_ident("u32")
        || type_path.path.is_ident("u64")
        || type_path.path.is_ident("u128")
        || type_path.path.is_ident("f32")
        || type_path.path.is_ident("f64")
        || type_path.path.is_ident("char")
        || type_path.path.is_ident("bool")
}