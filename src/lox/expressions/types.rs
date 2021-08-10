pub enum Literal {
    True, False, Nil, Number(f64), String(String)
}

pub enum Expr {
    Binary  {
         left: Expr,
         operator: Token,
         right: Expr,
    },
    Grouping  {
         expression: Expr,
    },
    Literal  {
         value: Literal,
    },
    Unary  {
         operator: Token,
         right: Expr,
    },
}
