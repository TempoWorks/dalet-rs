use num_enum::TryFromPrimitive;

use crate::daletl::{self, t_new, Tid, ToDaletl, ToDaletlArgument, ToDaletlBody, ToDaletlTag};

const NB: daletl::Body = daletl::Body::Null;
const NA: daletl::Argument = daletl::Argument::Null;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    El(NotNullBody),
    H(String, HeadingLevel),
    P(NotNullBody),
    Br,
    Ul(Vec<Tag>),
    Ol(Vec<Tag>),
    Row(Vec<Tag>, AlignArgument),
    Link(Body, String),
    Navlink(Body, String),
    Btn(Body, String),
    Navbtn(Body, String),
    Img(String),
    Table(Vec<Tag>),
    Tcol(Vec<Tag>),
    Tpcol(Vec<Tag>),
    Hr,
    B(String),
    I(String),
    Bq(NotNullBody),
    Footlnk(TextOrNumberArgument),
    Footn(String, TextOrNumberArgument),
    A(TextOrNumberArgument),
    S(String),
    Sup(String),
    Sub(String),
    Disc(NotNullBody),
    Bl(NotNullBody, AlignArgument),
    Carousel(Vec<Tag>),
    Code(String, TextOrNullArgument),
    Pre(String),
}

impl ToDaletlTag for Tag {
    fn to_daletl_tag(self) -> daletl::Tag {
        match self {
            Tag::El(b) => t_new(Tid::El, b.to_daletl_body(), NA),
            Tag::H(b, a) => t_new(Tid::H, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::P(b) => t_new(Tid::P, b.to_daletl_body(), NA),
            Tag::Br => t_new(Tid::Br, NB, NA),
            Tag::Ul(b) => t_new(Tid::Ul, b.to_daletl_body(), NA),
            Tag::Ol(b) => t_new(Tid::Ol, b.to_daletl_body(), NA),
            Tag::Row(b, a) => t_new(Tid::Row, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::Link(b, a) => t_new(Tid::Link, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::Navlink(b, a) => t_new(Tid::Navlink, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::Btn(b, a) => t_new(Tid::Btn, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::Navbtn(b, a) => t_new(Tid::Navbtn, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::Img(a) => t_new(Tid::Img, NB, a.to_daletl_argument()),
            Tag::Table(b) => t_new(Tid::Table, b.to_daletl_body(), NA),
            Tag::Tcol(b) => t_new(Tid::Tcol, b.to_daletl_body(), NA),
            Tag::Tpcol(b) => t_new(Tid::Tpcol, b.to_daletl_body(), NA),
            Tag::Hr => t_new(Tid::Hr, NB, NA),
            Tag::B(b) => t_new(Tid::B, b.to_daletl_body(), NA),
            Tag::I(b) => t_new(Tid::I, b.to_daletl_body(), NA),
            Tag::Bq(b) => t_new(Tid::Bq, b.to_daletl_body(), NA),
            Tag::Footlnk(a) => t_new(Tid::Footlnk, NB, a.to_daletl_argument()),
            Tag::Footn(b, a) => t_new(Tid::Footn, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::A(a) => t_new(Tid::A, NB, a.to_daletl_argument()),
            Tag::S(b) => t_new(Tid::S, b.to_daletl_body(), NA),
            Tag::Sup(b) => t_new(Tid::Sup, b.to_daletl_body(), NA),
            Tag::Sub(b) => t_new(Tid::Sub, b.to_daletl_body(), NA),
            Tag::Disc(b) => t_new(Tid::Disc, b.to_daletl_body(), NA),
            Tag::Bl(b, a) => t_new(Tid::Bl, b.to_daletl_body(), a.to_daletl_argument()),
            Tag::Carousel(b) => t_new(Tid::Carousel, b.to_daletl_body(), NA),
            Tag::Code(s, a) => t_new(Tid::Code, s.to_daletl_body(), a.to_daletl_argument()),
            Tag::Pre(s) => t_new(Tid::Pre, s.to_daletl_body(), NA),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum HeadingLevel {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl ToDaletlArgument for HeadingLevel {
    fn to_daletl_argument(self) -> daletl::Argument {
        match self {
            HeadingLevel::One => NA,
            HeadingLevel::Two => 2u8.to_daletl_argument(),
            HeadingLevel::Three => 3u8.to_daletl_argument(),
            HeadingLevel::Four => 4u8.to_daletl_argument(),
            HeadingLevel::Five => 5u8.to_daletl_argument(),
            HeadingLevel::Six => 6u8.to_daletl_argument(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum AlignArgument {
    Start,
    Center,
    End,
}

impl ToDaletlArgument for AlignArgument {
    fn to_daletl_argument(self) -> daletl::Argument {
        match self {
            Self::Start => NA,
            Self::Center => 1u8.to_daletl_argument(),
            Self::End => 2u8.to_daletl_argument(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextOrNumberArgument {
    Text(String),
    Number(u8),
}

impl ToDaletlArgument for TextOrNumberArgument {
    fn to_daletl_argument(self) -> daletl::Argument {
        match self {
            Self::Number(n) => n.to_daletl_argument(),
            Self::Text(s) => s.to_daletl_argument(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextOrNullArgument {
    Text(String),
    Null,
}

impl ToDaletlArgument for TextOrNullArgument {
    fn to_daletl_argument(self) -> daletl::Argument {
        match self {
            Self::Text(s) => s.to_daletl_argument(),
            Self::Null => NA,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

impl ToDaletlBody for Body {
    fn to_daletl_body(self) -> daletl::Body {
        match self {
            Body::Null => NB,
            Body::Tags(v) => v.to_daletl_body(),
            Body::Text(v) => v.to_daletl_body(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Argument {
    Text(String),
    Number(u8),
    Null,
}

impl ToDaletlArgument for Argument {
    fn to_daletl_argument(self) -> daletl::Argument {
        match self {
            Argument::Null => NA,
            Argument::Number(v) => v.to_daletl_argument(),
            Argument::Text(v) => v.to_daletl_argument(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotNullArgument {
    Text(String),
    Number(u8),
}

impl ToDaletlArgument for NotNullArgument {
    fn to_daletl_argument(self) -> daletl::Argument {
        match self {
            NotNullArgument::Number(v) => v.to_daletl_argument(),
            NotNullArgument::Text(v) => v.to_daletl_argument(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotNullBody {
    Text(String),
    Tags(Vec<Tag>),
}

impl ToDaletlBody for NotNullBody {
    fn to_daletl_body(self) -> daletl::Body {
        match self {
            NotNullBody::Text(v) => v.to_daletl_body(),
            NotNullBody::Tags(v) => v.to_daletl_body(),
        }
    }
}

impl ToDaletlBody for Vec<Tag> {
    fn to_daletl_body(self) -> daletl::Body {
        daletl::Body::Tags(self.to_daletl())
    }
}

impl ToDaletl for Vec<Tag> {
    fn to_daletl(self) -> Vec<daletl::Tag> {
        self.into_iter().map(|tag| tag.to_daletl_tag()).collect()
    }
}

impl ToDaletlBody for String {
    fn to_daletl_body(self) -> daletl::Body {
        daletl::Body::Text(self)
    }
}

impl ToDaletlArgument for String {
    fn to_daletl_argument(self) -> daletl::Argument {
        daletl::Argument::Text(self)
    }
}

impl ToDaletlArgument for u8 {
    fn to_daletl_argument(self) -> daletl::Argument {
        daletl::Argument::Number(self)
    }
}
