use crate::enums;
use std::convert::From;

macro_rules! impl_from {
    ($enum_:ident, $kind:ident, $struct_:ident) => {
        impl From<$struct_> for $enum_ {
            fn from(s: $struct_) -> Self {
                $enum_::$kind(s)
            }
        }
    };
}

pub enum Stmt {
    AlterCollation(AlterCollationStmt),
    AlterObjDepends(AlterObjDependsStmt),
}

macro_rules! stmt_impl_from {
    ($kind:ident, $struct_:ident) => {
        impl_from!(Stmt, $kind, $struct_);
    };
}
stmt_impl_from!(AlterCollation, AlterCollationStmt);
stmt_impl_from!(AlterObjDepends, AlterObjDependsStmt);


pub enum Indirection {
    Str(String),
    Star,
    Slice(Option<i64>,Option<i64>),
    Index(i64)
}


pub enum AlterObj {
    QualName(QualName),
}
macro_rules! ao_impl_from {
    ($kind:ident, $struct_:ident) => {
        impl_from!(AlterObj, $kind, $struct_);
    };
}
ao_impl_from!(QualName, QualName);

/// Qualified name
#[derive(Debug, Default)]
pub struct QualName {
    pub database Option<String>,
    pub schema: Option<String>,
    pub name: String,
}

impl QualName {
    fn from_str1(s: &str) -> Self {
        QualName { database: None, schema: None, name: s.to_string() }
    }

    fn from_str2(s1: &str, s2: &str) -> Self {
        QualName { database: None, schema: s1.to_string(), name: s2.to_string() }
    }

    fn from_str3(s1: &str, s2: &str, s3: &str) -> Self {
        QualName { database: s1.to_string(), schema: s2.to_string(), name: s3.to_string() }
    }
}

pub struct AlterCollationStmt {
    pub collation_name: QualName,
}

pub struct AlterObjDependsStmt {
    /// OBJECT_FUNCTION, OBJECT_TRIGGER, etc
    pub object_kind: enums::ObjKind,
    /// in case a table is involved
    pub relation: Option<QualName>,
    /// name of the object
    pub object: Option<AlterObj>,
    /// extension name
    pub extension_name: String,
}

pub struct ObjWithArgs {
    /// Name of the function/procedure/operator
    pub name: QualName,
    /// None means zero args
    pub args: Option<Vec<>>
}

pub struct FuncParam {
    pub name: String,
    pub type_kind: TypeKind,
    pub mode: enums::ParamMode,
    //pub default: ???
}

pub enum ArrayBounds {
    Unbounded
    Bounded(i64)
}

/// TypeCast - a CAST expression
pub struct TypeCast {
    /// the expression being casted
	pub expr: Expr,			
    /// the target type
	pub type_name: TypeName		
}

/// TypeName - specifies a type in definitions
///
/// If is_pct_type is true, then names is actually a field name and we look up
/// the type of that field.  Otherwise (the normal case), names is a type
/// name possibly qualified with schema and database name.
#[derive(Default, Debug)]
pub struct TypeName {
    pub name: QualName,
    /// unique collection?
    pub is_set: bool,
    /// %TYPE specified?
    pub is_pct_type: bool,
    /// Type modifier expression(s)
    pub typmods: Option<Vec<?>>
    /// Prespecified type modifier
    pub typemod: Option<i32>,
    pub array_bounds: Option<Vec<ArrayBounds>>, 
}

impl From<QualName> for TypeName {
    fn from(name: QualName) -> Self {
        TypeName {
            name: name,
            is_set: false,
            is_pct_type: false,
            typmods: None,
            typemod: None,
        }
    }
}
