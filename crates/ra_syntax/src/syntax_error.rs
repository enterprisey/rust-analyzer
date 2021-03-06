//! FIXME: write short doc here

use std::fmt;

use ra_parser::ParseError;

use crate::{validation::EscapeError, TextRange, TextUnit, TokenizeError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError {
    kind: SyntaxErrorKind,
    location: Location,
}

// FIXME: Location should be just `Location(TextRange)`
// TextUnit enum member just unnecessarily compicates things,
// we should'n treat it specially, it just as a `TextRange { start: x, end: x + 1 }`
// see `location_to_range()` in ra_ide/src/diagnostics
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Location {
    Offset(TextUnit),
    Range(TextRange),
}

impl From<TextUnit> for Location {
    fn from(offset: TextUnit) -> Location {
        Location::Offset(offset)
    }
}

impl From<TextRange> for Location {
    fn from(range: TextRange) -> Location {
        Location::Range(range)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Offset(it) => fmt::Debug::fmt(it, f),
            Location::Range(it) => fmt::Debug::fmt(it, f),
        }
    }
}

impl SyntaxError {
    pub fn new<L: Into<Location>>(kind: SyntaxErrorKind, loc: L) -> SyntaxError {
        SyntaxError { kind, location: loc.into() }
    }

    pub fn kind(&self) -> SyntaxErrorKind {
        self.kind.clone()
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn offset(&self) -> TextUnit {
        match self.location {
            Location::Offset(offset) => offset,
            Location::Range(range) => range.start(),
        }
    }

    pub fn add_offset(mut self, plus_offset: TextUnit, minus_offset: TextUnit) -> SyntaxError {
        self.location = match self.location {
            Location::Range(range) => Location::Range(range + plus_offset - minus_offset),
            Location::Offset(offset) => Location::Offset(offset + plus_offset - minus_offset),
        };

        self
    }

    pub fn debug_dump(&self, acc: &mut impl fmt::Write) {
        writeln!(acc, "error {:?}: {}", self.location(), self.kind()).unwrap();
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxErrorKind {
    ParseError(ParseError),
    EscapeError(EscapeError),
    TokenizeError(TokenizeError),
    // FIXME: the obvious pattern of this enum dictates that the following enum variants
    // should be wrapped into something like `SemmanticError(SemmanticError)`
    // or `ValidateError(ValidateError)` or `SemmanticValidateError(...)`
    InvalidBlockAttr,
    InvalidMatchInnerAttr,
    InvalidTupleIndexFormat,
    VisibilityNotAllowed,
    InclusiveRangeMissingEnd,
}

impl fmt::Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SyntaxErrorKind::*;
        match self {
            InvalidBlockAttr => {
                write!(f, "A block in this position cannot accept inner attributes")
            }
            InvalidMatchInnerAttr => {
                write!(f, "Inner attributes are only allowed directly after the opening brace of the match expression")
            }
            InvalidTupleIndexFormat => {
                write!(f, "Tuple (struct) field access is only allowed through decimal integers with no underscores or suffix")
            }
            ParseError(msg) => write!(f, "{}", msg.0),
            EscapeError(err) => write!(f, "{}", err),
            TokenizeError(err) => write!(f, "{}", err),
            VisibilityNotAllowed => {
                write!(f, "unnecessary visibility qualifier")
            }
            InclusiveRangeMissingEnd => {
                write!(f, "An inclusive range must have an end expression")
            }
        }
    }
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[rustfmt::skip]
        let msg = match self {
            TokenizeError::EmptyInt => {
                "Missing digits after the integer base prefix"
            }
            TokenizeError::EmptyExponent => {
                "Missing digits after the exponent symbol"
            }
            TokenizeError::UnterminatedBlockComment => {
                "Missing trailing `*/` symbols to terminate the block comment"
            }
            TokenizeError::UnterminatedChar => {
                "Missing trailing `'` symbol to terminate the character literal"
            }
            TokenizeError::UnterminatedByte => {
                "Missing trailing `'` symbol to terminate the byte literal"
            }
            TokenizeError::UnterminatedString => {
                "Missing trailing `\"` symbol to terminate the string literal"
            }
            TokenizeError::UnterminatedByteString => {
                "Missing trailing `\"` symbol to terminate the byte string literal"
            }
            TokenizeError::UnterminatedRawString => {
                "Missing trailing `\"` with `#` symbols to terminate the raw string literal"
            }
            TokenizeError::UnterminatedRawByteString => {
                "Missing trailing `\"` with `#` symbols to terminate the raw byte string literal"
            }
            TokenizeError::UnstartedRawString => {
                "Missing `\"` symbol after `#` symbols to begin the raw string literal"
            }
            TokenizeError::UnstartedRawByteString => {
                "Missing `\"` symbol after `#` symbols to begin the raw byte string literal"
            }
            TokenizeError::LifetimeStartsWithNumber => {
                "Lifetime name cannot start with a number"
            }
        };
        write!(f, "{}", msg)
    }
}

impl fmt::Display for EscapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            EscapeError::ZeroChars => "Empty literal",
            EscapeError::MoreThanOneChar => "Literal should be one character long",
            EscapeError::LoneSlash => "Character must be escaped: '\\'",
            EscapeError::InvalidEscape => "Invalid escape sequence",
            EscapeError::BareCarriageReturn => "Character must be escaped: '\r'",
            EscapeError::EscapeOnlyChar => "Character must be escaped",
            EscapeError::TooShortHexEscape => "Escape sequence should have two digits",
            EscapeError::InvalidCharInHexEscape => "Escape sequence should be a hexadecimal number",
            EscapeError::OutOfRangeHexEscape => "Escape sequence should be ASCII",
            EscapeError::NoBraceInUnicodeEscape => "Invalid escape sequence",
            EscapeError::InvalidCharInUnicodeEscape => "Invalid escape sequence",
            EscapeError::EmptyUnicodeEscape => "Invalid escape sequence",
            EscapeError::UnclosedUnicodeEscape => "Missing '}'",
            EscapeError::LeadingUnderscoreUnicodeEscape => "Invalid escape sequence",
            EscapeError::OverlongUnicodeEscape => {
                "Unicode escape sequence should have at most 6 digits"
            }
            EscapeError::LoneSurrogateUnicodeEscape => {
                "Unicode escape code should not be a surrogate"
            }
            EscapeError::OutOfRangeUnicodeEscape => {
                "Unicode escape code should be at most 0x10FFFF"
            }
            EscapeError::UnicodeEscapeInByte => "Unicode escapes are not allowed in bytes",
            EscapeError::NonAsciiCharInByte => "Non ASCII characters are not allowed in bytes",
        };
        write!(f, "{}", msg)
    }
}

impl From<EscapeError> for SyntaxErrorKind {
    fn from(err: EscapeError) -> Self {
        SyntaxErrorKind::EscapeError(err)
    }
}
