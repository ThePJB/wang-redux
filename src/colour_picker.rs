use crate::level::*;
use crate::application::*;
use crate::renderer::*;
use crate::rect::*;
use crate::kmath::*;


pub struct ColourPicker {
}

impl Scene for ColourPicker {
    fn handle_event(&mut self, event: &glutin::event::Event<()>) -> SceneOutcome {
        match event {
            glutin::event::Event::WindowEvent {event: glutin::event::WindowEvent::KeyboardInput {
                input: glutin::event::KeyboardInput { virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape), ..}, ..}, ..}
            => {
                return SceneOutcome::Pop(SceneSignal::JustPop);
            },
            glutin::event::Event::WindowEvent {event: glutin::event::WindowEvent::KeyboardInput {
                input: glutin::event::KeyboardInput { virtual_keycode: Some(glutin::event::VirtualKeyCode::Space), ..}, ..}, ..}
            => {
                return SceneOutcome::Pop(SceneSignal::Colour(Vec3::new(1.0, 0.0, 0.0)));
            },
            _ => {},
        }

        SceneOutcome::None
    }

    fn handle_signal(&mut self, signal: SceneSignal) -> SceneOutcome {
        SceneOutcome::None
    }

    fn draw(&self, gl: &glow::Context, r: &mut Renderer, egui: &mut egui_glow::EguiGlow, window: &winit::window::Window) {
        let (needs_repaint, shapes) = egui.run(window, |egui_ctx| {
            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.heading(" Colour");
                // ui.color_edit_button_rgb(&mut self.colour);
                ui.end_row();
                if ui.button("Quit").clicked() {
                    println!("spaget");
                }
            });
        });


        r.draw_rect(Rect::new(0.25, 0.25, 0.5, 0.5), Vec3::new(1.0, 1.0, 0.0), 1.0);
    }
}
