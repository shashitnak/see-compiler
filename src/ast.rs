
enum StaticValue {
    Str(String),
    Char(char),
    Int(i32),
    Long(i64),
    UInt(u32),
    ULong(u64),
    Float(f32),
    Double(f64),
}

enum Type {
    StrT,
    CharT,
    IntT,
    LongT,
    UIntT,
    ULongT,
    FloatT,
    DoubleT,
    CustomT(String)
}

enum Exp {
    Constant(StaticValue),
    Variable(Var, Type),
    InfixOperation(InfixOp, Box<Exp>, Box<Exp>),
    FuncCall(String, Vec<Exp>),
    Paren(Box<Exp>)
}

struct Var(String, StaticValue);

enum Stmt {
    Id(Exp),
    Assign(Var, Exp),
}

enum InfixOp {
    Plus,
    Minus,
    Star,
    Slash,
    BackSlash,
    Mod,
    Pow,
    And,
    Or,
    OrOr,
    Gt,
    St,
    GE,
    SE,
    EE
}

struct CurlyBraceGlobal(Vec<Stmt>);

struct CurlyBraceLocal(Vec<Exp>);

enum Program {
    StaticAssign(),
    Struct(Name, Vec<(String, Type)>),
    Function(CurlyBraceGlobal),
}
