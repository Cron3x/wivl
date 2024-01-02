pub enum Stmt{
    Program {
        body: Vec<Stmt>
    },
    BinaryExpr{
        left: Box<Stmt>,
        right: Box<Stmt>,
        operator: String, //Todo: Make to enume
    },
    NumericLiteral{
        value: String,
    },
    Identifier{
        symbol: String,
    },
}
impl std::fmt::Display for Stmt{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self {
            Self::Program{..} => write!(f, "Program"),
            Self::BinaryExpr{..} => write!(f, "BinaryExpr"),
            Self::NumericLiteral{..} => write!(f, "NumericLiteral"),
            Self::Identifier{..} => write!(f, "Identifier"),
        }
    }
}
