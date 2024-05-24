// 1. Bring the macros `error, warn, info, debug, trace`
//    into scope from the log package with a
// `use` statement.
//
// You should be able to run `cargo build --lib`
// successfully after this step (and each step in this file)
//
// Hint: You need to update Cargo.toml to add the `log`
// dependency, first.
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        // 2. Use debug!() to log "A new Frog has been
        //    created"
        let frog = Default::default();
        debug!(
            target: "new",
            "New frog {:?} created.",
            frog
        );
        frog
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        // 3. Use info!() to log that a Frog hopped, and how
        //    much energy is left
        if self.energy == 0 {
            // 4. Use warn!() to warn that the frog will go
            //    to sleep since he ran out of energy
            warn!(target:"hop",  "The frog will now go to sleep, because he ran out of energy after this final hop.");
            self.sleep();
        }
        info!(
            target: "hop",
            "A frog hopped! He/she has {} energy left.",
            self.energy
        );
    }
    pub fn sleep(&mut self) {
        if self.sleeping {
            // 5. Use error!() to log a (non-fatal) error
            //    stating that the Frog is already asleep
            error!(
            target: "sleep",
            "The frog is already asleep, so can't go to sleep any harder"
            );
        } else {
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        // 6. Use trace!() to log that a default value was
        //    generated, with the debug representation
        let frog = Frog {
            energy: 5,
            sleeping: false,
        };
        trace!(
            target: "default",
            "Created default {:#?}",
            frog
        );
        frog
    }
}
