use error::MaeveError;
use interpreter::fuzz::Fuzz;
use interpreter::machine::Action;
use protos::master::game;
use interpreter::machine::Machine;
use screen::Interfaceable;

pub fn builtin<I: Interfaceable>(
    m: &Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    for fuzzed in Fuzz::new(&m.tokens) {
        if fuzzed.remainder.len() > 1 {
            return Ok(None);
        }
        match fuzzed.token.as_str() {
            "EXIT" | "QUIT" => return Err(MaeveError::Exit),
            "SAVE" => return Ok(Some(Action::Save)),
            _ => return Ok(None),
        }
    }
    return Ok(None);
}

pub fn item<I: Interfaceable>(
    m: &Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    // Attempt item search
    for fuzzed_item in Fuzz::new(&m.tokens) {
        if let Some(item) = m.items.get(&fuzzed_item.token) {
            for fuzzed_action in Fuzz::new(&fuzzed_item.remainder) {
                if let Some(action) = item.actions.get(&fuzzed_action.token) {
                    return Ok(Some(Action::Act(action)));
                }
            }
        }
    }
    return Ok(None);
}

pub fn level<I: Interfaceable>(
    m: &Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    // Attempt item search
    for fuzzed_item in Fuzz::new(&m.tokens){
        if let Some(action) = m.level.actions.get(&fuzzed_item.token) {
            return Ok(Some(Action::Act(action)));
        }
    }
    return Ok(None);
}

// Possibly move undefined definitions into the proto. That way it's a little
// more robust and game specific. Posibly add some randomness to the pick as
// well?
pub fn undefined<I: Interfaceable>(
    _m: &Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    return Ok(Some(Action::Undefined));
}

pub type Parser<M> = Fn(M) -> Result<Option<Action>, MaeveError>;
