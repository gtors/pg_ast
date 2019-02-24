/// When a command can act on several kinds of objects with only one
/// parse structure required, use these constants to designate the
/// object type.  Note that commands typically don't support all the types.
pub enum ObjectKind {
    AccessMethod,
    Aggregate,
    Amop,
    Amproc,
    // type's attribute, when distinct from column
    Attribute,
    Cast,
    Column,
    Collation,
    Conversion,
    Database,
    Default,
    Defacl,
    Domain,
    Domconstraint,
    EventTrigger,
    Extension,
    Fdw,
    ForeignServer,
    ForeignTable,
    Function,
    Index,
    Language,
    Largeobject,
    MatView,
    Opclass,
    Operator,
    Opfamily,
    Policy,
    Procedure,
    Publication,
    PublicationRel,
    Role,
    Routine,
    Rule,
    Schema,
    Sequence,
    Subscription,
    StatisticExt,
    Tabconstraint,
    Table,
    Tablespace,
    Transform,
    Trigger,
    Tsconfiguration,
    Tsdictionary,
    Tsparser,
    Tstemplate,
    Type,
    UserMapping,
    View,
}


pub enum ParamMode {
    /// Input only
    In,
    /// Output only
    Out,
    /// Both
    InOut
    /// Variadic (always input)
    Variadic,
    /// Table function output column
    Table
}
