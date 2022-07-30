pub struct SpanIdentifier {
    pub filename: String,
    pub line: usize,
    pub span: (usize, usize),
    pub value: String,
}
