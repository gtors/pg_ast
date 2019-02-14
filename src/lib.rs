mod ast;
mod enums;

#[cfg(test)]
mod test;

use lalrpop_util::lalrpop_mod;


lalrpop_mod!(pg);


pub fn parse_sql(input: &str) -> Result<ast::Stmt, String> {
    let res = pg::StmtParser::new().parse(input);
    match res {
        Ok(stmt) => Ok(stmt),
        Err(err) => Err(format!("{:?}", err).to_string())
    }
}
