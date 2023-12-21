enum NodeKind{
    Program,
    CallExpr,
    BinaryExpr,
    UnaryExpr,
    NumericLiteral,
    Indentifier,
}

pub enum Stmt {
    Program{
        kind: NodeKind,
        body: Box<Stmt>
    },
    Expr(Expr),
}

enum Expr {
    BinaryExpr{
        left: Box<Expr>,
        right: Box<Expr>,
        operator: String
    },
    CallExpr,
    UnaryExpr,
    NumericLiteral{
        value: usize,
    },
    Identifier{
        symbol: String
    },
}

struct BinaryExpr{
}

//TODO: Make everything return a value, if there is none than return nil
