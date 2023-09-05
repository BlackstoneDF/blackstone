use chumsky::span::SimpleSpan;

pub enum ParsedStringType {
    Normal,
    Template(TemplateString)
}

pub struct TemplateString {
    elements: Vec<Element>,

}

/// "Hello $player, you have ${coins + 1} coins"
///  |    ||     ||         ||          ||    |
/// Literal|     |Literal   ||          |Literal
///        Single            Multi      |

pub enum Element {
    Literal(ElementLiteral),
    Single(),
    Multi()
}

pub struct ElementLiteral {
    inner: String,
    span: SimpleSpan
}

pub struct SingleElement {
    selector: String,
    span: SimpleSpan
}

pub struct MultiElement {

}
