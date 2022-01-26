//use bevy::prelude::*;


use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
//use wasm_bindgen::prelude::*;

const BEVY_TEXTURE_ID: u64 = 0;

const TIME_STEP: f32 = 1.0 / 60.0;
//----------------------------

#[derive(Component)]
enum Collider {
    Solid,
    Scorable,
    Paddle,
}


#[derive(Component)]
struct Ball {
    velocity: Vec3,
}
//----------------------------
/// This example demonstrates the following functionality and use-cases of bevy_egui:
/// - rendering loaded assets;
/// - toggling hidpi scaling (by pressing '/' button);
/// - configuring egui contexts during the startup.
//#[wasm_bindgen(start)]
pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .insert_resource(ClearColor(Color::rgb(0.9, 0.2, 0.2)))

        .add_startup_system(setup)

        //.add_startup_system(load_assets)
        //.add_startup_system(configure_visuals)
        
        .add_system(ui_example)

        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                //.with_system(paddle_movement_system)
                //.with_system(ball_collision_system)
                .with_system(ball_movement_system),
        )

        .run();
}

#[derive(Default)]
struct UiState {
    label: String,
    value: f32,
    //painting: Painting,
    inverted: bool,

    name: String,
    age: u32 ,
}

//-------------------------------

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec3::new(150.0, 30.0, 1.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_color = Color::rgb(0.5, 0.5, 1.0);
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            // brick
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: brick_color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: brick_position,
                        scale: brick_size,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Collider::Scorable);
        }
    }

    //------------------------------ Ball
    // ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -50.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}
//-------------------------------


fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    assets: Res<AssetServer>,
) {
    
    //.............>

    //egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
        
        //ui_state.painting.ui_control(ui);                         // bar pour epaisseur et couleur et reset

        // -------- --------   --------  ----------------> canvas pour peindre
        //egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
        //    ui_state.painting.ui_content(ui);
        //});
        //   canvas pour peindre   <---------------- -------- -------- -------- 
    //});

    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    egui::SidePanel::left("side_panel")
        .default_width(100.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 50.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
            });

            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID),
                [256.0, 256.0],
            ));

            

            ui.allocate_space(egui::Vec2::new(1.0, 289.0));

            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                //ui.text_edit_singleline(&mut name);            name    not found in this scope
                ui.text_edit_singleline(&mut ui_state.name);
            });
            //ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            ui.add(egui::Slider::new(&mut ui_state.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                //age += 1;
                ui_state.age += 1;
            }
            //ui.label(format!("Hello '{}', age {}", name, age));
            ui.label(format!("Hello '{}', age {}", ui_state.name, ui_state.age));
            });

    

    
}

struct Painting {
    lines: Vec<Vec<egui::Vec2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
        }
    }
}

impl Painting {
    

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());
        let rect = response.rect;

        //if self.lines.is_empty() {
        //    self.lines.push(vec![]);
        //}

        //let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = pointer_pos - rect.min;
            
            //if current_line.last() != Some(&canvas_pos) {
            //    current_line.push(canvas_pos);
            //}
        } //else if !current_line.is_empty() {
        //    self.lines.push(vec![]);
        //}

        //for line in &self.lines {
        //    if line.len() >= 2 {
        //       let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
        //        painter.add(egui::Shape::line(points, self.stroke));
        //    }
        //}
    }
}


fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>) {
    let (ball, mut transform) = ball_query.single_mut();
    transform.translation += ball.velocity * TIME_STEP;
}