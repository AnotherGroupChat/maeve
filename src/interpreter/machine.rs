use error::MaeveError;
use load::save;
use protos::master::Change;
use protos::master::Conditional;
use protos::master::Context;
use protos::master::Game;
use protos::master::State;
use protos::master::branch::Branch;
use protos::master::context::Content;
use protos::master::context::Scope;
use protos::master::game;
use screen::Interfaceable;
use std::collections::HashMap;
use std::rc::Rc;

// Struct to bind. A pretty large but trivial architecture change could be to
// add all functions that consume machine to a machine implimentation. This
// would mean that machine wouldn't have to be passed around as much.
// FILE(machine.rs)
#[derive(Debug)]
pub struct Machine<'m, I: 'm + Interfaceable> {
    pub src: &'m mut I,
    pub game: Rc<&'m mut Game>,
    pub level: Rc<&'m game::Level>,
    pub items: HashMap<String, Rc<&'m game::Item>>,
}

pub enum Action {
    Act(Rc<game::Action>),
    NoOp,
    Undefined,
    Save,
}

impl<'m, I: Interfaceable> Machine<'m, I> {

    pub fn new(
        src: &'m mut I,
        game: &'m mut Game,
    ) -> Self {
        let mut items: HashMap<String, Rc<&game::Item>> = HashMap::new();
        let person : Option<Box<&game::Character>> = None;
        // let game : &'m Rc<&'m mut Game> = &Rc::new(game);

        if let &Some(ref person) = &game.person {
            let person = Box::new(&person);
            for (ref name, ref item) in person.inventory.iter() {
                items.insert(name.to_string(), Rc::new(&item));
            }
        }

        // items.extend(person.inventory.clone());
        if let Some(person) = person {
            if let &Some(level) = &game.levels.get(&person.level) {
                // items.extend(level.items.clone());
                return Machine {
                    src: src,
                    game: Rc::new(game),
                    level: Rc::new(level),
                    items: items,
                    // tokens: ,
                };
            }
            panic!(MaeveError::from("Level for character not found..."));
        }
        panic!(MaeveError::from(
            "A Character was not specifying in the game...",
        ));
    }

    fn extract_state(
        &mut self,
        context: &Context,
    ) -> Result<State, MaeveError> {
        let reference = self.game.clone().person?.level;

        // TODO Got more clones than a bobafet. Clean up.
        let state = match Scope::from_i32(context.scope)? {
            Scope::Character => {
                self.game.clone().person?.stats.get(&context.id)?.clone()
            }
            Scope::Level => {
                self.game.levels.get(&reference)?.clone().state?.clone()
            }
            Scope::Item => {
                self.game
                    .levels
                    .get(&reference)?
                    .items
                    .get(&context.id)?
                    .clone()
                    .state?
            }
            Scope::Global => self.game.globals.get(&context.id)?.clone(),
        };
        return Ok(state);
    }

    fn change_state(&mut self, change: &Change) -> Result<(), MaeveError> {
        let mut state = self.extract_state(&change.clone().context?)?;

        // TODO(madisetti): implement this for values.
        // I'm just feeling pretty lazy at the moment.
        // Technically only replace.

        match change.clone().context?.content?.clone() {
            Content::Tags(tags) => state.tags[0] = tags,
            Content::Value(value) => state.value = value,
        }

        // Update the scope.
        let change = change.clone();
        let reference = self.game.clone();
        match Scope::from_i32(change.clone().context?.scope)? {
            //Should be something like
            // *m..person?.stats.get_mut(&change.context?.id)? = state,
            Scope::Character => println!("TODO"),
            Scope::Level => {
                // Should be: self.level.state = Some(state)
                self.game.levels.get_mut(&reference.person?.level)?.state =
                    Some(state)
            }
            // Only works for character levels for now. Fixxxx.
            // Should be *self.items.get_mut(&change.context?.id) = Some(state)
            Scope::Item => {
                self.game
                    .levels
                    .get_mut(&reference.person?.level)?
                    .items
                    .get_mut(&change.context?.id)?
                    .state = Some(state)
            }
            Scope::Global => {
                *self.game.globals.get_mut(&change.context?.id)? = state
            }
        }
        return Ok(());
    }

    fn check_context(&mut self, clause: &Context) -> Result<bool, MaeveError> {
        let state = self.extract_state(clause)?;
        return match clause.clone().content? {
            Content::Tags(tags) => Ok(state.tags.contains(&tags)),
            Content::Value(value) => Ok(state.value == value),
        };
    }

    fn evaluate_conditional(
        &mut self,
        conditional: Conditional,
        description: &mut String,
    ) -> Result<(), MaeveError> {
        let branch = if self.check_context(&conditional.clause?)? {
            conditional.left
        } else {
            conditional.right
        };

        if let Some(branch) = branch {
            match branch.branch? {
                Branch::Fork(fork) => {
                    return self.evaluate_conditional(*fork, description)
                }
                Branch::Change(change) => {
                    description.push_str(&change.comment);
                    return self.change_state(&change);
                }
                Branch::Leaf(leaf) => description.push_str(&leaf),
            }
        }
        return Ok(());
    }

    pub fn process_action(
        &mut self,
        game_action: Action,
    ) -> Result<(), MaeveError> {
        match game_action {
            Action::Act(action) => {
                let mut description: String = action.description.clone();
                for conditional in action.conditionals.clone() {
                    self.evaluate_conditional(conditional, &mut description)?;
                }
                self.src.print(&description);
            },
            Action::Save => {
                save(self.src, &mut self.game)?
            },
            _ => self.src.print("Didn't do anything..."),
        }
        return Ok(());
    }
}
