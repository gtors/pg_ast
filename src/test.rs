use super::parse_sql;

#[test]
fn test_collation() {
    parse_sql(r#"ALTER COLLATION "en_US" REFRESH VERSION"#).unwrap();
}

#[test]
fn test_alter_dependences() {
    parse_sql(r#"ALTER INDEX "e" DEPENDS ON EXTENSION _test_"#).unwrap();
    parse_sql(r#"ALTER MATERIALIZED VIEW d DEPENDS ON EXTENSION test_ext5"#).unwrap();
    parse_sql(r#"ALTER TRIGGER c ON a DEPENDS ON EXTENSION test_ext5"#).unwrap();
}

