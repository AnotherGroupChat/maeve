//! Holds definition to wrap and alter game.

use error::MaeveError;
use load::save;
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

// TODO: Remove pub once constructor is implemented. In addition add mut to
// lookup values so that states can be directly set in change state.
#[derive(Debug)]
pub struct Machine<'m, I: 'm + Interfaceable> {
    pub src: &'m mut I,
    pub game: &'m mut Game,
    pub level: &'m game::Level,
    pub items: HashMap<String, game::Item>,
    pub person: &'m game::Character,
}

pub enum Action {
    Act(Rc<game::Action>),
    NoOp,
    Undefined,
    Save,
}

pub fn extract_information<'g>(
    game: &'g Game,
    items: &mut HashMap<String, game::Item>,
) -> Result<(&'g game::Level, &'g game::Character), MaeveError> {
    // TODO: Replace with constructor.
    if let Some(ref person) = game.person {
        items.extend(person.inventory.clone());
        if let Some(level) = game.levels.get(&person.level) {
            items.extend(level.items.clone());
            return Ok((level, person));
        }
        return Err(MaeveError::from("Level for character not found..."));
    }
    return Err(MaeveError::from(
        "A Character was not specifying in the game...",
    ));
}

impl<'m, I: Interfaceable> Machine<'m, I> {
    //TODO: Ideally encorporate extract info into this such that Machine can
    // be created within the impl and then pass an instance back. On trying to
    // implement this I ended up in borrowing hell as the lifetimes should
    // extend the whole object and somehow magically exceed scope. Currently
    // it's easier just to procss this out of scope. pub fn new(src: &'m
    // mut I, game: &'m mut Game) -> Self

    /* TODO: Create a mutable version of this such that state can be
     * directly set. This would also mean not copying game in extract phase
     * such that the reference can be directly set. */
    fn extract_state(
        &'m self,
        context: &Context,
    ) -> Result<&'m State, MaeveError> {
        let state = match Scope::from_i32(context.scope)? {
            Scope::Character => self.person.stats.get(&context.id)?,
            Scope::Level => self.level.state.as_ref().unwrap(),
            Scope::Item => self.items.get(&context.id)?.state.as_ref().unwrap(),
            Scope::Global => self.game.globals.get(&context.id)?,
        };
        return Ok(state);
    }

    fn change_state(&'m mut self, context: &Context) -> Result<(), MaeveError> {
        let mut state = self.extract_state(&context)?.clone();

        // TODO(madisetti): implement this for values.
        // I'm just feeling pretty lazy at the moment.
        // Technically only replace.
        match context.content.as_ref().unwrap() {
            &Content::Tags(ref tags) => {
                state.tags.as_mut_slice()[0] = tags.to_string()
            }
            &Content::Value(value) => state.value = value,
        }

        // TODO: If the reference version of this is created. Use the
        // references to directly set the values on the game object. As per
        // explained earlier, borrow hell, amkes this difficult to achieve
        // (maybe once I learn a bit more rust foo.
        match Scope::from_i32(context.scope)? {
            Scope::Character => {
                *self.game
                    .person
                    .as_mut()
                    .unwrap()
                    .stats
                    .get_mut(&context.id)? = state
            }
            Scope::Level => {
                self.game.levels.get_mut(&self.person.level)?.state =
                    Some(state)
            }
            // TODO: Only works for character levels for now. Fixxxx.
            Scope::Item => {
                self.game
                    .levels
                    .get_mut(&self.person.level)?
                    .items
                    .get_mut(&context.id)?
                    .state = Some(state)
            }
            Scope::Global => *self.game.globals.get_mut(&context.id)? = state,
        }

        return Ok(());
    }

    fn check_context(&'m self, clause: &Context) -> Result<bool, MaeveError> {
        let state = self.extract_state(clause)?;
        return match clause.content.as_ref().unwrap() {
            &Content::Tags(ref tags) => Ok(state.tags.contains(&tags)),
            &Content::Value(value) => Ok(state.value == value),
        };
    }

    fn evaluate_conditional(
        &'m mut self,
        conditional: &Conditional,
        description: &mut String,
    ) -> Result<(), MaeveError> {
        let branch =
            if self.check_context(&conditional.clause.as_ref().unwrap())? {
                &conditional.left
            } else {
                &conditional.right
            };

        if let &Some(ref branch) = branch {
            match branch.branch.as_ref().unwrap() {
                &Branch::Fork(ref fork) => {
                    return self.evaluate_conditional(&*fork, description)
                }
                &Branch::Change(ref change) => {
                    description.push_str(&change.comment);
                    return self.change_state(change.context.as_ref().unwrap());
                }
                &Branch::Leaf(ref leaf) => description.push_str(&leaf),
            }
        }
        return Ok(());
    }

    pub fn process_action(
        &'m mut self,
        game_action: Action,
    ) -> Result<String, MaeveError> {
        let mut description = String::from("");
        match game_action {
            Action::Act(action) => {
                description.push_str(action.description.as_ref());
                self.evaluate_conditional(
                    &action.conditional.as_ref().unwrap(),
                    &mut description,
                )?;
            }
            Action::Save => save(self.src, &mut self.game)?,
            _ => description.push_str("Didn't do anything..."),
        }
        return Ok(description);
    }
}
