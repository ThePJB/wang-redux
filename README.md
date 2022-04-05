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

-----

ok add colour picker

get mouse clicks
then sort out the egui-glow

-------

I would really like to just return a buffer from draw

slotting in egui is hectic, if i have it in draw draw needs to be &mut self
and also needs to return SceneOutcome

its not parsimonious with my current interface

I suppose event handling will need to know window dimensions for mouse stuff

im kind of against encapsulation. CoI hey.

maybe something retained mode would fit better

I could do without egui

draw(rect) -> Vec<float>, now thats a nice interface
editor interface can have easily a bunch of buttons. Define them once for both drawing and click detection.
Vec<button>: Button {rect, editorCommand, rollover, selected, texture/colour, hotkey, ..}

fn gui -> Vec<button>
fn clickButtons?(Vec<button>) -> command

------
todo rip out egui
todo simplify drawing
todo put in buttons and stuff

OR move drawing into handle event, and supply relevant context to handle event.


encapsulation: for shit thats fucked where you can lose flexibility or afford to keep punching holes
the price of encapsulation: it doesnt actually do anything
for dealing with other people's code

programming: ultimate mindset test
-----------------

Done: egui gone, drawing simplified
Doing: UI buttons

function that generates Vec<button> from screen rect

fuckin maybe a scene graph is good. Because what about clicking in my level.


------------
Lesson learned: 
2d indexing: x*h or y*w
dont get frustrated its an unknown unknown



functionality: 
    clicking on level
    button textures / letters
    snowkoban text.png
