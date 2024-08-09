use chumsky::prelude::*;

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    // Symbols
    /// [
    LSquare,
    /// ]
    RSquare,
    /// [[
    ElOpen,
    /// ]]
    ElClose,

    // Arguments
    NumberArgument(u8),
    TextArgument(&'src str),

    // Body
    TextBody(&'src str),
    /// Multi Line text
    MLText(&'src str),
    /// Multi Line with min spaces text
    MLMSText(usize, &'src str),
    /// Raw Multi line text
    RMLText(&'src str),

    /// Special
    TextTag(&'src str),
    Paragraph(&'src str),

    /// Special removed before parse
    Comment(&'src str),
    EmptyLine,

    // Tags
    El,
    H,
    P,
    Br,
    Ul,
    Ol,
    Row,
    Link,
    Navlink,
    Btn,
    Navbtn,
    Img,
    Table,
    Tcol,
    Tpcol,
    Hr,
    B,
    I,
    Bq,
    Footlnk,
    Footn,
    A,
    S,
    Sup,
    Sub,
    Disc,
    Block,
    Carousel,
    Code,
    Pre,
    Meta,
}

impl<'src> From<Spanned<Token<'src>>> for Token<'src> {
    fn from(value: Spanned<Token<'src>>) -> Self {
        value.0
    }
}
