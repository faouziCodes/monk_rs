pub enum Term {
    Var(String),
    Abstr(String, Box<Term>),
    Apply(Box<Term>, Box<Term>),
}
