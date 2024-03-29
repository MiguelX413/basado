use std::fmt::{Display, Formatter};

use crate::types::defs::{Delimiter, Keyword, Punct};

impl Display for Punct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign => write!(f, ":="),
            Self::PlusPlus => write!(f, "++"),
            Self::MinusMinus => write!(f, "--"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::StarStar => write!(f, "**"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Caret => write!(f, "^"),
            Self::Not => write!(f, "!"),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Shl => write!(f, "<<"),
            Self::Shr => write!(f, ">>"),
            Self::Eq => write!(f, "="),
            Self::EqEq => write!(f, "=="),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Ge => write!(f, ">="),
            Self::Le => write!(f, "<="),
            Self::At => write!(f, "@"),
            Self::Underscore => write!(f, "_"),
            Self::Dot => write!(f, "."),
            Self::Comma => write!(f, ","),
            Self::Semi => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::ColonColon => write!(f, "::"),
            Self::RArrow => write!(f, "->"),
            Self::FatArrow => write!(f, "=>"),
            Self::Tilde => write!(f, "~"),
            Self::ForAll => write!(f, "∀"),
            Self::Exists => write!(f, "∃"),
            Self::PlusPercent => write!(f, "+%"),
            Self::PlusPipe => write!(f, "+|"),
            Self::MinusPercent => write!(f, "-%"),
            Self::MinusPipe => write!(f, "-|"),
            Self::StarPercent => write!(f, "*%"),
            Self::StarPipe => write!(f, "*|"),
            Self::ShlPipe => write!(f, "<<|"),
        }
    }
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurlyLeft => write!(f, "{{"),
            Self::CurlyRight => write!(f, "}}"),
            Self::SquareLeft => write!(f, "["),
            Self::SquareRight => write!(f, "]"),
            Self::ParLeft => write!(f, "("),
            Self::ParRight => write!(f, ")"),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Match => write!(f, "match"),
            Self::Loop => write!(f, "loop"),
            Self::Let => write!(f, "let"),
            Self::Type => write!(f, "type"),
            Self::Class => write!(f, "class"),
            Self::Ret => write!(f, "ret"),
            Self::Where => write!(f, "where"),
            Self::Miguel => write!(f, "miguel"),
            Self::Kyasig => write!(f, "kyasig"),
            Self::Claim => write!(f, "claim"),
            Self::Cardinality => write!(f, "cardinality"),
            Self::Bytes => write!(f, "bytes"),
            Self::Bits => write!(f, "bits"),
            Self::Fn => write!(f, "fn"),
        }
    }
}
