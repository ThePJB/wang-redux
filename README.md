Need scene for pick colour, set w,h, set powerup n
how would scene return a value, could just be an enum that gets passed to the thing below it. That sounds useful.

The ideal interface would be update(all inputs) -> Option<SceneOutcome>
curried its like raw_inputs -> commands -> Option<SceneOutcome>
how to reconcile that with egui??

or is that too fucked, should it just be a widget, but it needs to eat inputs as well

custom cursor demonstrating selected editor function

-------------
fn signal -> Command
fn event -> Command
fn apply_Command(&mut self, Command)

and are commands pushes or pops

fn handle_event(&mut self, event E) -> SceneOutcome
fn handle_signal(&mut self, signal S) -> SceneOutcome

-----

ok i want to drop into an editor, press space to go in game, press escape to go back to editor