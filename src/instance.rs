use crate::*;
use alloc::string::String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance {
    pub position: Position,
    pub id: String, // item / ending id
}

/// a Room can have many Exits in different positions,
/// optionally with a transition and dialogue
/// todo make a from_str() function for this
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExitInstance {
    pub position: Position,
    pub exit: Exit,
    pub transition: Option<Transition>,
    pub dialogue_id: Option<String>,
}
