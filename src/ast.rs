use crate::enums::ObjectKind;
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

pub enum AlterNode {
    AlterCollation(AlterCollationStmt),
    AlterObjectDepends(AlterObjectDependsStmt),
}
macro_rules! an_impl_from {
    ($kind:ident, $struct_:ident) => {
        impl_from!(AlterNode, $kind, $struct_);
    };
}
an_impl_from!(AlterCollation, AlterCollationStmt);
an_impl_from!(AlterObjectDepends, AlterObjectDependsStmt);

pub enum AlterObject {
    QualifiedName(QualifiedName),
}
macro_rules! ao_impl_from {
    ($kind:ident, $struct_:ident) => {
        impl_from!(AlterObject, $kind, $struct_);
    };
}
ao_impl_from!(QualifiedName, QualifiedName);

#[derive(Debug)]
pub struct QualifiedName {
    pub schema: Option<String>,
    pub name: String,
}

pub struct AlterCollationStmt {
    pub collation_name: QualifiedName,
}

pub struct AlterObjectDependsStmt {
    /// OBJECT_FUNCTION, OBJECT_TRIGGER, etc
    pub object_kind: ObjectKind,
    /// in case a table is involved
    pub relation: Option<QualifiedName>,
    /// name of the object
    pub object: Option<AlterObject>,
    /// extension name
    pub extension_name: String,
}
