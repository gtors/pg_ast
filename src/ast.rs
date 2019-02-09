pub enum AlterNode {
    AlterCollation(AlterCollationStmt),
}


#[derive(Debug)]
pub struct QualifiedName {
    pub schema: Option<String>,
    pub name: String,
}


pub struct AlterCollationStmt {
    pub collation_name: QualifiedName,
}
