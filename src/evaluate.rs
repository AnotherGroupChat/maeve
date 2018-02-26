//! A file that holds logical operations in order to run the game.

use error::MaeveError;
use interpreter::token::tokenize;
use load::save;
use protos::master::Game;
use protos::master::game;
use screen::Interfaceable;
use std::collections::HashMap;
use regex::Regex;
use protos::master::Conditional;
use protos::master::State;
use protos::master::Change;
use protos::master::Context;
use protos::master::context::Scope;
use protos::master::context::Content;
use protos::master::branch::Branch;

// Struct to bind. A pretty large but trivial architecture change could be to
// add all functions that consume machine to a machine implimentation. This
// would mean that machine wouldn't have to be passed around as much.
// FILE(machine.rs)
#[derive(Debug)]
struct Machine<'m, I: 'm + Interfaceable> {
    src: &'m mut I,
    game: &'m mut Game,
    tokens: &'m Vec<String>,
    level: &'m game::Level,
    items: &'m HashMap<String, game::Item>,
}

enum Action {
    Act(game::Action),
    NoOp,
}

// Clones content on load. Should by rights be actual references (Will take a
// look at shared pointers, and the RC struct). Currenty this implementation
// causes the ugly hacks for getting and settings states cause can' refer to
// these. Ideally refactor and make 101% better.
fn extract_information<'g>(
    game: &'g Game,
    items: &mut HashMap<String, game::Item>,
) -> Result<&'g game::Level, MaeveError> {
    if let Some(ref person) = game.person {
        items.extend(person.inventory.clone());
        if let Some(level) = game.levels.get(&person.level) {
            items.extend(level.items.clone());
            return Ok(level);
        }
        return Err(MaeveError::from("Level for character not found..."));
    }
    return Err(MaeveError::from(
        "A Character was not specifying in the game...",
    ));
}

// FILE(interpret/fuzz.rs)
struct Replacement {
    re: Regex,
    rep: &'static str,
}

impl Replacement {
    fn new<'r>(re: &'r str, rep: &'static str) -> Self {
        Replacement {
            re: Regex::new(re).unwrap(),
            rep: rep,
        }
    }
}

struct Fuzz {
    index: usize,
    permutes: usize,
    token: String,
    remainder: Vec<String>,
}
#[derive(Debug)]
struct FuzzItem {
    token: String,
    remainder: Vec<String>,
}

// Regex for for find and replace, mispellings etc..
// Ideally regex should be a const and not a function. But const allocation
// is still experimental in Rust. Looks like it might be rolled out at some
// point in the future, so keep posted.
lazy_static! {
    static ref REGEXES: [Replacement; 1] = [Replacement::new("ei", "ie")];
}

impl Fuzz {
    fn new(mut baseline: Vec<String>) -> Self {
        let token = baseline.pop().unwrap_or(String::from(""));
        Fuzz {
            index: 0,
            permutes: 0,
            token: token,
            remainder: baseline,
        }
    }
}

// Would be better with a generator, However generators are currently only
// experimental in Rust. Currently does a permutation with all possible words.
// I should mention this is a massive hack. A better implementation of this
// would be creating something like a Chomsky Hierarchy and intelligently check
// tokens against the game definition.
impl Iterator for Fuzz {
    type Item = FuzzItem;
    fn next(&mut self) -> Option<Self::Item> {
        let refresh = self.permutes >= REGEXES.len();
        if self.index > self.remainder.len()
            || (self.index == self.remainder.len() && refresh)
        {
            return None;
        }

        if refresh {
            let (a, b) =
                (self.token.clone(), self.remainder[self.index].clone());
            self.remainder[self.index] = a;
            self.token = b;
            self.permutes = 0;
            self.index += 1;
        }

        while self.permutes < REGEXES.len()
            && !REGEXES[self.permutes].re.is_match(&self.token)
        {
            self.permutes += 1;
        }

        let mut token = self.token.clone();
        if self.permutes < REGEXES.len() {
            token = String::from(
                REGEXES[self.permutes]
                    .re
                    .replace(&token, REGEXES[self.permutes].rep),
            );
        }
        return Some(FuzzItem {
            token: token,
            remainder: self.remainder.clone(),
        });
    }
}

// FILE(intepret/parsers.rs)

fn builtin<I: Interfaceable>(
    m: &mut Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    if m.tokens.len() == 1 {
        match &m.tokens.first().unwrap_or(&String::from(""))[..] {
            "exit" | "quit" => return Err(MaeveError::Exit),
            "save" => save(m.src, &mut m.game)?,
            _ => return Ok(None),
        };
        return Ok(Some(Action::NoOp));
    }
    return Ok(None);
}

fn item<I: Interfaceable>(
    m: &mut Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    // Attempt item search
    for fuzzed_item in Fuzz::new(m.tokens.clone()) {
        if let Some(item) = m.items.get(&fuzzed_item.token) {
            for fuzzed_action in Fuzz::new(fuzzed_item.remainder.clone()) {
                if let Some(action) = item.actions.get(&fuzzed_action.token) {
                    return Ok(Some(Action::Act(action.clone())));
                }
            }
        }
    }
    return Ok(None);
}

fn level<I: Interfaceable>(
    m: &mut Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    // Attempt item search
    for fuzzed_item in Fuzz::new(m.tokens.clone()) {
        if let Some(action) = m.level.actions.get(&fuzzed_item.token) {
            return Ok(Some(Action::Act(action.clone())));
        }
    }
    return Ok(None);
}

// Possibly move undefined definitions into the proto. That way it's a little
// more robust and game specific. Posibly add some randomness to the pick as
// well?
fn undefined<I: Interfaceable>(
    _m: &mut Machine<I>,
) -> Result<Option<Action>, MaeveError> {
    let mut act = game::Action::default();
    act.description = String::from("Yeah.. I didn't get that.");
    return Ok(Some(Action::Act(act)));
}

// FILE(machine.rs)
fn extract_state<'m, I: Interfaceable>(
    m: &mut Machine<I>,
    context: &Context,
) -> Result<State, MaeveError> {
    let reference = m.game.clone().person?.level;

    // TODO Got more clones than a bobafet. Clean up.
    let state = match Scope::from_i32(context.scope)? {
        Scope::Character => {
            m.game.clone().person?.stats.get(&context.id)?.clone()
        }
        Scope::Level => m.game.levels.get(&reference)?.clone().state?.clone(),
        Scope::Item => {
            m.game
                .levels
                .get(&reference)?
                .items
                .get(&context.id)?
                .clone()
                .state?
        }
        Scope::Global => m.game.globals.get(&context.id)?.clone(),
    };
    return Ok(state);
}

fn change_state<I: Interfaceable>(
    m: &mut Machine<I>,
    change: &Change,
) -> Result<(), MaeveError> {
    let mut state = extract_state(m, &change.clone().context?)?;

    // TODO(madisetti): implement this for values.
    // I'm just feeling pretty lazy at the moment.
    // Technically only replace.

    match change.clone().context?.content?.clone() {
        Content::Tags(tags) => state.tags[0] = tags,
        Content::Value(value) => state.value = value,
    }

    // Update the scope.
    let change = change.clone();
    let reference = m.game.clone();
    match Scope::from_i32(change.clone().context?.scope)? {
        //Should be something like
        // *m.game.person?.stats.get_mut(&change.context?.id)? = state,
        Scope::Character => println!("TODO"),
        Scope::Level => {
            m.game.levels.get_mut(&reference.person?.level)?.state = Some(state)
        }
        // Only works for character levels for now. Fixxxx.
        Scope::Item => {
            m.game
                .levels
                .get_mut(&reference.person?.level)?
                .items
                .get_mut(&change.context?.id)?
                .state = Some(state)
        }
        Scope::Global => *m.game.globals.get_mut(&change.context?.id)? = state,
    }
    return Ok(());
}

fn check_context<I: Interfaceable>(
    m: &mut Machine<I>,
    clause: &Context,
) -> Result<bool, MaeveError> {
    let state = extract_state(m, clause)?;
    return match clause.clone().content? {
        Content::Tags(tags) => Ok(state.tags.contains(&tags)),
        Content::Value(value) => Ok(state.value == value),
    };
}

fn evaluate_conditional<I: Interfaceable>(
    m: &mut Machine<I>,
    conditional: Conditional,
    description: &mut String,
) -> Result<(), MaeveError> {
    let branch = if check_context(m, &conditional.clause?)? {
        conditional.left
    } else {
        conditional.right
    };

    if let Some(branch) = branch {
        match branch.branch? {
            Branch::Fork(fork) => {
                return evaluate_conditional(m, *fork, description)
            }
            Branch::Change(change) => {
                description.push_str(&change.comment);
                return change_state(m, &change);
            }
            Branch::Leaf(leaf) => description.push_str(&leaf),
        }
    }
    return Ok(());
}

fn process_action<I: Interfaceable>(
    m: &mut Machine<I>,
    game_action: Action,
) -> Result<(), MaeveError> {
    if let Action::Act(action) = game_action {
        let mut description: String = action.description;
        for conditional in action.conditionals {
            evaluate_conditional(m, conditional, &mut description)?;
        }
        m.src.print(&description);
    }
    return Ok(());
}
pub fn evaluate<I: Interfaceable>(
    src: &mut I,
    game: &mut Game,
) -> Result<(), MaeveError> {
    let parsers: [&Fn(&mut Machine<I>) -> Result<Option<Action>, MaeveError>;
                     4] = [&builtin, &item, &level, &undefined];

    let mut items: HashMap<String, game::Item> = HashMap::new();
    let level = extract_information(&game, &mut items)?;
    let mut game = &mut game.clone();
    loop {
        let token_string = src.prompt()?;

        // TODO(45): Put a mutex on threads and pull the newest game from a
        // channel. For example: mut game = src.sync();
        let mut machine = Machine {
            src: src,
            game: &mut game,
            level: level,
            items: &items,
            tokens: &tokenize(&token_string),
        };

        for parser in parsers.iter() {
            if let Some(action) = parser(&mut machine)? {
                process_action(&mut machine, action)?;
                break;
            };
        }

        // Poor man's debug. It hurts me more than you know.
        // println!("{:?}", machine.game);
        // TODO(45): Broadcast game to the rest of the threads. For example:
        // src.broadcast(game)
    }
}
