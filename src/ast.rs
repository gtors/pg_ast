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

pub enum Stmt {
    AlterCollation(AlterCollationStmt),
    AlterObjectDepends(AlterObjectDependsStmt),
    AlterObjectSchema(AlterObjectSchemaStmt),
}

macro_rules! stmt_impl_from {
    ($kind:ident, $struct_:ident) => {
        impl_from!(Stmt, $kind, $struct_);
    };
}
stmt_impl_from!(AlterCollation, AlterCollationStmt);
stmt_impl_from!(AlterObjectDepends, AlterObjectDependsStmt);
stmt_impl_from!(AlterObjectSchema, AlterObjectSchemaStmt);


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


pub struct AlterObjectSchemaStmt {
    /// OBJECT_FUNCTION, OBJECT_TRIGGER, etc
    pub object_kind: ObjectKind,
    /// name of the object
    pub object: String,
    /// the new schema
    pub new_schema: String,
    /// IF EXISTS present in statement
    pub if_exists: bool
}
