// vim ts=4 sw=4 et
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

#[test]
fn test_alter_schema() {
    parse_sql(r#"ALTER COLLATION "test" SET SCHEMA test_123"#).unwrap();
    //parse_sql(r#"ALTER TABLE IF EXISTS tt8 SET SCHEMA alter2"#).unwrap();
    //parse_sql(r#"ALTER AGGREGATE alt_agg2(int) SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER COLLATION test11 SET SCHEMA test_schema"#).unwrap();
    parse_sql(r#"ALTER CONVERSION alt_conv3 SET SCHEMA alt_nsp2"#).unwrap();
    //parse_sql(r#"ALTER FOREIGN TABLE IF EXISTS doesnt_exist_ft1 SET SCHEMA foreign_schema"#).unwrap();
    //parse_sql(r#"ALTER FOREIGN TABLE ft1 SET SCHEMA foreign_schema"#).unwrap();
    //parse_sql(r#"ALTER FUNCTION alt_func3(int) SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER MATERIALIZED VIEW mvtest_tvm SET SCHEMA mvtest_mvschema"#).unwrap();
    //parse_sql(r#"ALTER OPERATOR @+@(int4, int4) SET SCHEMA alt_nsp2"#).unwrap();
    //parse_sql(r#"ALTER OPERATOR @-@(int4, int4) SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER OPERATOR CLASS alt_opc2 USING hash SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER OPERATOR FAMILY alt_opf2 USING hash SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER STATISTICS alt_stat3 SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER TABLE new_system_table SET SCHEMA pg_catalog"#).unwrap();
    //parse_sql(r#"ALTER TABLE temp_view_test.tmp1 SET SCHEMA testviewschm2"#).unwrap();
    parse_sql(r#"ALTER TABLE tx1 SET SCHEMA temp_view_test"#).unwrap();
    parse_sql(r#"ALTER TEXT SEARCH CONFIGURATION alt_ts_conf3 SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER TEXT SEARCH DICTIONARY alt_ts_dict3 SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER TEXT SEARCH PARSER alt_ts_prs2 SET SCHEMA alt_nsp2"#).unwrap();
    parse_sql(r#"ALTER TEXT SEARCH TEMPLATE alt_ts_temp2 SET SCHEMA alt_nsp2"#).unwrap();
}
