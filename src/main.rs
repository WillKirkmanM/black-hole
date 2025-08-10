use eframe::egui;
use glam::{vec2, vec3, Vec3};

/// Stores the state of the application.
struct BlackHoleApp {
    /// The image buffer we will draw our visualization to.
    image: egui::ColorImage,
    /// The texture handle for our image, used by egui to render it.
    texture: Option<egui::TextureHandle>,
    /// Camera's distance from the center (0,0,0).
    radius: f32,
    /// Camera's orbital angle.
    azimuth: f32,
}

impl Default for BlackHoleApp {
    fn default() -> Self {
        Self {
            image: egui::ColorImage::new([300, 200], vec![egui::Color32::BLACK; 300 * 200]),
            texture: None,
            radius: 15.0,
            azimuth: 0.0,
        }
    }
}

impl eframe::App for BlackHoleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Controls").show(ctx, |ui| {
            ui.heading("Camera Controls");
            ui.add(egui::Slider::new(&mut self.radius, 2.0..=50.0).text("Distance"));
            ui.label("Drag the image to orbit the camera.");
            ui.separator();
            ui.heading("About");
            ui.label("A CPU-based black hole ray tracer using egui.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Black Hole Visualiser");
            ui.label("The gravitational lensing is simulated on the CPU.");
            
            // Allow dragging the image to orbit the camera
            let response = ui.image(egui::ImageSource::Texture(egui::load::SizedTexture::new(
                self.texture.get_or_insert_with(|| {
                    ui.ctx().load_texture("black_hole_image", self.image.clone(), Default::default())
                }).id(),
                ui.available_size(),
            )));

            if response.is_pointer_button_down_on() {
                let drag_delta = response.drag_delta();
                self.azimuth -= drag_delta.x * 0.01;
            }

            self.ray_trace_scene();

            if let Some(texture) = self.texture.as_mut() {
                texture.set(self.image.clone(), Default::default());
            }
        });

        ctx.request_repaint();
    }
}

impl BlackHoleApp {
    fn ray_trace_scene(&mut self) {
        let width = self.image.width();
        let height = self.image.height();
        let aspect_ratio = width as f32 / height as f32;

        let cam_pos = vec3(self.radius * self.azimuth.cos(), 3.0, self.radius * self.azimuth.sin());
        let look_at = Vec3::ZERO;
        let forward = (look_at - cam_pos).normalize();
        let right = forward.cross(Vec3::Y).normalize() * aspect_ratio;
        let up = right.cross(forward);

        let schwarzschild_radius: f32 = 1.0;
        let sr_squared = schwarzschild_radius * schwarzschild_radius;
        
        for y in 0..height {
            for x in 0..width {
                let u = (x as f32 / width as f32) * 2.0 - 1.0;
                let v = (y as f32 / height as f32) * 2.0 - 1.0;

                let mut ray_dir = (forward + right * u - up * v).normalize();
                let mut ray_pos = cam_pos;

                let mut final_color = egui::Color32::BLACK;

                for _ in 0..64 {
                    let dist_sq = ray_pos.length_squared();
                    let gravity = -ray_pos.normalize() * (1.0 / dist_sq) * 2.5;
                    ray_dir = (ray_dir + gravity).normalize();

                    ray_pos += ray_dir * 0.5;

                    if ray_pos.length_squared() < sr_squared {
                        final_color = egui::Color32::BLACK;
                        break;
                    }

                    if ray_pos.y.abs() < 0.1 {
                        let dist_from_center = vec2(ray_pos.x, ray_pos.z).length();
                        if dist_from_center > schwarzschild_radius * 1.5 && dist_from_center < schwarzschild_radius * 4.0 {
                            let pattern = ((dist_from_center * 5.0).sin() + 1.0) * 0.5;
                            final_color = egui::Color32::from_rgb((255.0 * pattern) as u8, (120.0 * pattern) as u8, 0);
                            break;
                        }
                    }

                    if ray_pos.length() > 60.0 {
                        let star_val = (ray_dir.x.sin() * ray_dir.z.sin()).abs().powf(10.0);
                        if star_val > 0.5 {
                             final_color = egui::Color32::WHITE;
                        }
                        break;
                    }
                }
                self.image[(x, y)] = final_color;
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Black Hole Visualiser",
        options,
        Box::new(|_cc| Ok(Box::<BlackHoleApp>::default())),
    )
}