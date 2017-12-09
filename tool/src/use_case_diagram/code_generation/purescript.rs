use std::io;

use use_case_diagram::UseCaseDiagram;

/// Generate a module header.
pub fn generate_module_header<W>(w: &mut W, name: &str) -> io::Result<()>
    where W: io::Write {
    write!(w, "module {} where\n", name)?;
    Ok(())
}

/// Generate the imports necessary for the other generated code.
pub fn generate_imports<W>(w: &mut W) -> io::Result<()>
    where W: io::Write {
    write!(w, "import Prelude\n")?;
    write!(w, "import Data.List as List\n")?;
    write!(w, "import Data.Set as Set\n")?;
    write!(w, "import Butterfly.Actor (Actor (..))\n")?;
    write!(w, "import Butterfly.Portal (Button (..), Portal (..))\n")?;
    Ok(())
}

/// Generate a PureScript definition for a portal.
pub fn generate_portal_definition<W>(w: &mut W, diagram: &UseCaseDiagram, name: &str)
                                     -> io::Result<()>
    where W: io::Write {
    write!(w, "{}\n", name)?;
    write!(w, "  :: ∀ f\n")?;
    write!(w, "   . {{")?;
    for (i, (_, use_case)) in diagram.use_cases().enumerate() {
        if i == 0 {
            write!(w, " ")?;
        } else {
            write!(w, "\n     , ")?;
        }
        write!(w, "{:?} :: f Unit", use_case.title)?;
    }
    write!(w, " }}\n")?;
    write!(w, "  -> Portal f\n")?;

    write!(w, "{} actions =\n", name)?;
    write!(w, "  Portal <<< List.fromFoldable $\n")?;
    write!(w, "    [")?;
    for (i, (use_case_id, use_case)) in diagram.use_cases().enumerate() {
        if i == 0 {
            write!(w, " ")?;
        } else {
            write!(w, "\n    , ")?;
        }
        write!(w, "Button {:?}\n", use_case.title)?;
        write!(w, "             (Set.fromFoldable\n")?;
        write!(w, "                [")?;
        let actors =
            diagram.associations()
            .filter(|&(_, assoc_use_case_id)| assoc_use_case_id == use_case_id)
            .map(|(actor_id, _)| diagram.actor(actor_id).unwrap());
        for (i, actor) in actors.enumerate() {
            if i == 0 {
                write!(w, " ")?;
            } else {
                write!(w, "\n                , ")?;
            }
            write!(w, "Actor {:?}", actor.name)?;
        }
        write!(w, " ])\n")?;
        write!(w, "             actions.{:?}", use_case.title)?;
    }
    write!(w, " ]\n")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use use_case_diagram::{Actor, UseCase};

    use std::fs::File;
    use std::rc::Rc;

    #[test]
    fn test_empty() {
        let diagram = UseCaseDiagram::new();
        generate_module_header(&mut io::stdout(), "ExamplePortal").unwrap();
        generate_imports(&mut io::stdout()).unwrap();
        generate_portal_definition(&mut io::stdout(), &diagram, "portal")
            .unwrap();
    }

    #[test]
    fn test_single_use_case() {
        let mut diagram = UseCaseDiagram::new();
        let _ = diagram.insert_use_case(UseCase{title: Rc::from("Ban subscriber")});
        generate_module_header(&mut io::stdout(), "ExamplePortal").unwrap();
        generate_imports(&mut io::stdout()).unwrap();
        generate_portal_definition(&mut io::stdout(), &diagram, "portal")
            .unwrap();
    }

    #[test]
    fn test_many_use_cases() {
        let mut diagram = UseCaseDiagram::new();
        let a = diagram.insert_actor(Actor{name: Rc::from("Administrator")});
        let s = diagram.insert_actor(Actor{name: Rc::from("Subscriber")});
        let bs = diagram.insert_use_case(UseCase{title: Rc::from("Ban subscriber")});
        let cs = diagram.insert_use_case(UseCase{title: Rc::from("Create subscriber")});
        let pc = diagram.insert_use_case(UseCase{title: Rc::from("Post comment")});
        diagram.insert_association(a, bs).unwrap();
        diagram.insert_association(a, cs).unwrap();
        diagram.insert_association(a, pc).unwrap();
        diagram.insert_association(s, cs).unwrap();
        diagram.insert_association(s, pc).unwrap();
        let mut file = File::create("/tmp/ExamplePortal.purs").unwrap();
        generate_module_header(&mut file, "ExamplePortal").unwrap();
        generate_imports(&mut file).unwrap();
        generate_portal_definition(&mut file, &diagram, "portal")
            .unwrap();
    }
}
