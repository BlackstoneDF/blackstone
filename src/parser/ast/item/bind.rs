use crate::parser::ast::{expr::Identifier, Spanned};

use super::Block;

/// ```antlr
/// bind_action:
/// 	'bind' 'action' Iden ('<' Iden '>')? '(' param_def ')' (':' type)? '[' bind_tags ']' 'to'
/// 		bind_block_def;
///
/// bind_block_def:
/// 	Iden String
/// 	// Selector
/// 	('<' 'selection' | String '>')?
/// 	// Params
/// 	('(' Iden | 'return' (',' Iden | 'return') ')')?
/// 	// Tags
/// 	('[' String ':' String ']')? ';';
/// ```
pub struct BindAction {
    pub bind_tok: Spanned,
    pub action_tok: Spanned,
    pub bind_name: Spanned<String>,
    pub sel_open: Option<Spanned>,
    pub sel_name: Spanned<String>,
    pub sel_close: Option<Spanned>,

}

/// ```bls
/// event Iden(event) {}
/// ```
#[derive(Debug)]
pub struct EventDeceleration {
    pub event_token: Spanned,
    pub iden: Identifier,
    pub paren_token: Spanned,
    pub event_iden: Identifier,
    pub block: Block,
}

// TODO: Add more visibilities
pub enum Visibility {
    Pub(PublicVis),
}

pub struct PublicVis {
    pub span: Spanned,
}
