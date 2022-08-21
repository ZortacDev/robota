use crate::context::Context;
use crate::runner::Runner;
use crate::Result;
use rayon::prelude::*;

struct State {
    name: String,
    runners: Vec<Box<dyn Runner + Send + Sync>>,
    out_transitions: Vec<Transition>
}

struct Transition {
    condition: fn(&Context) -> TransitionCheck,
    before_actions: Vec<fn(&Context)>,
    after_actions: Vec<fn(&Context)>,
    next_state: String
}

enum TransitionCheck {
    Take,
    Ignore
}

impl State {
    fn spin(&mut self, context: &Context) -> Result<()> {
        let errors: Vec<_> = (&mut self.runners)
            .into_par_iter()
            .map(|r| r.spin(context))
            .filter(|e| e.is_err())
            .collect();

        if !errors.is_empty() {
            unimplemented!(); // FIXME: Some of the runners returned errors, aggregate them and pass them onwards
        }

        Ok(())
    }
}