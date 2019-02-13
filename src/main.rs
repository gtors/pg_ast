use lalrpop_util::lalrpop_mod;

mod ast;
mod enums;

use ast::AlterNode;

lalrpop_mod!(pub pg);

fn main() {
    let sql = r#"ALTER COLLATION "public".any_name REFRESH VERSION"#;
    let alter_node = pg::AlterParser::new().parse(sql).unwrap();

    match alter_node {
        AlterNode::AlterCollation(x) => println!("{:?}", x.collation_name),
        _ => println!("Not ok"),
    }
}
