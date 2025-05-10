use enum_procs::AutoFrom;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Page {
    pub title: Option<Text>,
    pub description: Option<Text>,
    pub body: Vec<Tag>,
    pub variables: Option<Vec<Text>>,
}

impl Page {
    pub fn new(
        title: Option<Text>,
        description: Option<Text>,
        body: Vec<Tag>,
        variables: Option<Vec<Text>>,
    ) -> Self {
        Self {
            title,
            description,
            body,
            variables,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TableRows {
    Default(Vec<Tag>),
    Primary(Vec<Tag>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Tag {
    /// Default element without styles.
    Element { body: Body },
    /// Heading. Bold, big, with vertical margin.
    Heading { body: Text, heading: HeadingLevel },
    /// Paragraph with vertical margin.
    Paragraph { body: Body },

    /// Link to another resource. Open in new tab.
    Link { body: Option<Body>, dref: Text },
    /// Link to another resource. Open in current tab.
    NavLink { body: Option<Body>, dref: Text },
    /// Link to another resource with button style. Open in new tab.
    Button { body: Option<Body>, dref: Text },
    /// Link to another resource with button style. Open in current tab.
    NavButton { body: Option<Body>, dref: Text },

    /// Image with optional alt.
    Image { src: Text, alt: Option<String> },
    /// Table
    Table { body: Vec<TableRows> },
    /// List with custom marker style.
    List { body: Vec<Tag>, style: ListStyle },

    /// Bold text
    Bold { body: Text },
    /// Italic text
    Italic { body: Text },
    /// Strikethrough text
    Strikethrough { body: Text },
    /// Superscript text
    Superscript { body: Text },
    /// Subscript text
    Subscript { body: Text },

    /// Link to FootNote
    FootLink { footnote: u64 },
    /// FootNote
    FootNote { body: Text, footnote: u64 },
    /// Anchor for using in links (Link { dref: "#id" })
    Anchor { id: Text },

    /// Preformatted block of text.
    Preformatted { body: Text },
    /// BlockQuote
    BlockQuote { body: Body },
    /// Block of code. With highlighting.
    Code { body: Text, language: Option<Text> },

    /// Block with a lighter background and padding.
    Block { body: Vec<Tag> },
    /// Flex block.
    Flex {
        body: Vec<Tag>,
        wrap: bool,
        align_x: Option<Align>,
        align_y: Option<Align>,
    },
    /// Grid block.
    Grid {
        body: Vec<Tag>,
        align_x: Option<Align>,
        align_y: Option<Align>,
    },
    /// Block that can be opened. With optional title.
    Disclosure { body: Body, title: Option<Text> },
    /// Carousel of blocks with buttons for switching between them.
    Carousel { body: Vec<Tag> },

    /// Display variable from variables in page by index.
    Variable { idx: u64 },

    /// LineBreak.
    LineBreak,
    /// Horizontal line
    HorizontalBreak,
}

pub type Text = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ListStyle {
    Disc,
    Decimal,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(AutoFrom, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Body {
    Text(String),
    Tags(Vec<Tag>),
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum HeadingLevel {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}
