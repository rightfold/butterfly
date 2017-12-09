use std::collections::{HashMap, HashSet};
use std::collections::{hash_map, hash_set};
use std::error::Error;
use std::fmt;
use std::iter;
use std::rc::Rc;

/// An actor identifier is unique per use case diagram.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ActorId(pub usize);

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A use case identifier is unique per use case diagram.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UseCaseId(pub usize);

impl fmt::Display for UseCaseId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An actor of zero or more use cases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    pub name: Rc<str>,
}

/// A use case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UseCase {
    pub title: Rc<str>,
}

/// An error that describes an invalid association.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssociationError {
    /// The association refers to a nonexistent actor.
    NonexistentActor(ActorId),

    /// The association refers to a nonexistent use case.
    NonexistentUseCase(UseCaseId),
}

impl fmt::Display for AssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AssociationError::NonexistentActor(actor_id) =>
                write!(f, "invalid association: nonexistent actor {}", actor_id),
            &AssociationError::NonexistentUseCase(use_case_id) =>
                write!(f, "invalid association: nonexistent use case {}", use_case_id),
        }
    }
}

impl Error for AssociationError {
    fn description(&self) -> &str {
        match self {
            &AssociationError::NonexistentActor(_) =>
                "invalid association: nonexistent actor",
            &AssociationError::NonexistentUseCase(_) =>
                "invalid association: nonexistent use case",
        }
    }
}

/// A use case diagram is a graph containing actors, use cases, and
/// associations.
#[derive(Clone, Debug)]
pub struct UseCaseDiagram {
    next_actor_id: usize,
    next_use_case_id: usize,

    actors: HashMap<ActorId, Actor>,
    use_cases: HashMap<UseCaseId, UseCase>,
    associations: HashSet<(ActorId, UseCaseId)>,
}

impl UseCaseDiagram {
    /// A new use case diagram with no actors and no use cases.
    pub fn new() -> Self {
        let diagram = UseCaseDiagram{
            next_actor_id: 0,
            next_use_case_id: 0,

            actors: HashMap::new(),
            use_cases: HashMap::new(),
            associations: HashSet::new(),
        };
        diagram.assert_invariants();
        diagram
    }

    fn next_actor_id(&mut self) -> ActorId {
        let actor_id = ActorId(self.next_actor_id);
        self.next_actor_id += 1;
        self.assert_invariants();
        actor_id
    }

    fn next_use_case_id(&mut self) -> UseCaseId {
        let use_case_id = UseCaseId(self.next_use_case_id);
        self.next_use_case_id += 1;
        self.assert_invariants();
        use_case_id
    }

    /// Get the actor with the given identifier.
    pub fn actor(&self, actor_id: ActorId) -> Option<&Actor> {
        self.actors.get(&actor_id)
    }

    /// Get the use case with the given identifier.
    pub fn use_case(&self, use_case_id: UseCaseId) -> Option<&UseCase> {
        self.use_cases.get(&use_case_id)
    }

    /// All actors in this use case diagram.
    pub fn actors(&self) -> Actors {
        self.actors.iter().map(|(&k, v)| (k, v))
    }

    /// All use cases in this use case diagram.
    pub fn use_cases(&self) -> UseCases {
        self.use_cases.iter().map(|(&k, v)| (k, v))
    }

    /// All associations in this use case diagram.
    pub fn associations(&self) -> Associations {
        self.associations.iter().cloned()
    }

    /// Insert a new actor, returning its identifier.
    pub fn insert_actor(&mut self, actor: Actor) -> ActorId {
        let actor_id = self.next_actor_id();
        self.actors.insert(actor_id, actor);
        self.assert_invariants();
        actor_id
    }

    /// Insert a new use case, returning its identifier.
    pub fn insert_use_case(&mut self, use_case: UseCase) -> UseCaseId {
        let use_case_id = self.next_use_case_id();
        self.use_cases.insert(use_case_id, use_case);
        self.assert_invariants();
        use_case_id
    }

    /// Insert a new association. Return an error if either the actor or the
    /// use case does not exist.
    pub fn insert_association(&mut self, actor_id: ActorId, use_case_id: UseCaseId)
                              -> Result<(), AssociationError> {
        if !self.actors.contains_key(&actor_id) {
            return Err(AssociationError::NonexistentActor(actor_id));
        }
        if !self.use_cases.contains_key(&use_case_id) {
            return Err(AssociationError::NonexistentUseCase(use_case_id));
        }
        self.associations.insert((actor_id, use_case_id));
        self.assert_invariants();
        Ok(())
    }

    fn assert_invariants(&self) {
        for &(actor_id, use_case_id) in &self.associations {
            assert!(self.actors.contains_key(&actor_id),
                    concat!("UseCaseDiagram invariant violation: association ",
                            "refers to nonexistent actor."));
            assert!(self.use_cases.contains_key(&use_case_id),
                    concat!("UseCaseDiagram invariant violation: association ",
                            "refers to nonexistent use case."));
        }
    }
}

/// Iterator of actors.
pub type Actors<'a> =
    iter::Map<hash_map::Iter<'a, ActorId, Actor>,
              fn((&'a ActorId, &'a Actor)) -> (ActorId, &'a Actor)>;

/// Iterator of use cases.
pub type UseCases<'a> =
    iter::Map<hash_map::Iter<'a, UseCaseId, UseCase>,
              fn((&'a UseCaseId, &'a UseCase)) -> (UseCaseId, &'a UseCase)>;

/// Iterator of associations.
pub type Associations<'a> =
    iter::Cloned<hash_set::Iter<'a, (ActorId, UseCaseId)>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let diagram = UseCaseDiagram::new();

        assert_eq!(diagram.actors().len(), 0);
        assert_eq!(diagram.use_cases().len(), 0);
        assert_eq!(diagram.associations().len(), 0);
    }

    #[test]
    fn test_insert_actor() {
        let mut diagram = UseCaseDiagram::new();
        let actor_1 = Actor{name: Rc::from("Actor 1")};
        let actor_2 = Actor{name: Rc::from("Actor 2")};

        let actor_id_1 = diagram.insert_actor(actor_1.clone());
        let actor_id_2 = diagram.insert_actor(actor_2.clone());

        assert_eq!(diagram.actors().collect::<HashMap<_, _>>(),
                   [(actor_id_1, &actor_1),
                    (actor_id_2, &actor_2)].iter().cloned().collect());
        assert_eq!(diagram.use_cases().len(), 0);
        assert_eq!(diagram.associations().len(), 0);
    }

    #[test]
    fn test_insert_use_case() {
        let mut diagram = UseCaseDiagram::new();
        let use_case_1 = UseCase{title: Rc::from("Use case 1")};
        let use_case_2 = UseCase{title: Rc::from("Use case 2")};

        let use_case_id_1 = diagram.insert_use_case(use_case_1.clone());
        let use_case_id_2 = diagram.insert_use_case(use_case_2.clone());

        assert_eq!(diagram.actors().len(), 0);
        assert_eq!(diagram.use_cases().collect::<HashMap<_, _>>(),
                   [(use_case_id_1, &use_case_1),
                    (use_case_id_2, &use_case_2)].iter().cloned().collect());
        assert_eq!(diagram.associations().len(), 0);
    }

    #[test]
    fn test_insert_association() {
        let mut diagram = UseCaseDiagram::new();
        let actor = Actor{name: Rc::from("Actor 1")};
        let use_case = UseCase{title: Rc::from("Use case 1")};

        let err = diagram.insert_association(ActorId(0), UseCaseId(0));
        assert!(err.is_err());

        let actor_id = diagram.insert_actor(actor.clone());
        let use_case_id = diagram.insert_use_case(use_case.clone());

        let ok = diagram.insert_association(actor_id, use_case_id);
        assert!(ok.is_ok());

        assert_eq!(diagram.actors().len(), 1);
        assert_eq!(diagram.use_cases().len(), 1);
        assert_eq!(diagram.associations().collect::<Vec<_>>(),
                   vec![(actor_id, use_case_id)]);
    }
}

pub mod code_generation;
