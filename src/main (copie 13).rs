use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

const BEVY_TEXTURE_ID: u64 = 0;

const BRICK_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);


#[derive(Component)]
struct RotRight; 


#[derive(Component)]
struct Vertimov; 


#[derive(Component)]
struct ScalMorph;



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_startup_system(setup.system())

        .add_system(ui_example)

        .run();
}

//#[derive(Default)]
struct UiState {
    //label: String,
    value: f32,
    rotval: f32,
    vertimov: f32,
    painting: Painting,
    inverted: bool,

    name: String,
    age: u32 ,
}

impl Default for UiState {
    fn default() -> Self {
        
        //UiState { painting: Painting::default() }
        //UiState { label: "the_label".to_string(), value: 3.5, inverted: true, name: "the_name".to_string(), age: 18, }
        //UiState { value: 3.5, rotval: 0.0, vertimov: 0.0, inverted: true, name: "the_name".to_string(), age: 18, }
        UiState { value: 3.5, rotval: 0.0, vertimov: 0.0, painting: Painting::default(), inverted: true, name: "the_name".to_string(), age: 18, }
    }
}





fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    assets: Res<AssetServer>,


    //mut queryRot: Query<(&mut Transform, &RotRight)>,
    //mut queryVertimov: Query<(&mut Transform, &Vertimov)>,


    mut q: QuerySet<(
        QueryState<&mut Transform, With<RotRight>>,  
        QueryState<&mut Transform, With<Vertimov>>,
        QueryState<&mut Sprite>
    )>,


) {
    


    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    egui::SidePanel::left("side_panel")
        .default_width(100.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                //ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }


            ui.add(egui::Slider::new(&mut ui_state.rotval, 0.0..= 1.0).text("rotval!"));
            if ui.button("RotaBtn").clicked() {
                
                //for (mut tf, rot) in queryRot.iter_mut() { tf.rotate(Quat::from_rotation_z( ui_state.rotval )); }

                for mut tf in q.q0().iter_mut() { tf.rotate(Quat::from_rotation_z( ui_state.rotval )); }
            }


            ui.add(egui::Slider::new(&mut ui_state.vertimov, -20.0..= 20.0).text("vertimov!"));
            if ui.button("VertimovBtn").clicked() {
                
                //for (mut tf, vertimov) in queryVertimov.iter_mut() { tf.translation.y += ui_state.vertimov ; }

                for mut tf in q.q1().iter_mut() { tf.translation.y += ui_state.vertimov ; }


            }





            //ui_state.painting.ui_control(ui);
            //ui_state.painting.ui_control(ui, q.q2());
            //ui_state.painting.ui_control(ui, q.q2().iter_mut());  // expected struct `bevy::prelude::Query`, found struct `QueryIter
            ui_state.painting.ui_control(ui, q.q2());







            ui.allocate_space(egui::Vec2::new(1.0, 50.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
            });

            //ui.add(egui::widgets::Image::new(
            //    egui::TextureId::User(BEVY_TEXTURE_ID),
            //    [256.0, 256.0],
            //));
        });
    
}


#[derive(Debug)]
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


    //pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
    //pub fn ui_control(&mut self, ui: &mut egui::Ui, mut query: Query<(&mut Transform, &ScalMorph)>) -> egui::Response {
    pub fn ui_control(&mut self, ui: &mut egui::Ui, mut query: Query<(&mut Sprite)>) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("inpec Painting").clicked() {
                //self.lines.clear();
                println!("{:#?}", &self);
                println!("{:#?}", &self.stroke.color);

                //for mut tf in query.iter_mut() {tf.scale += 0.10}
                //for mut sprt in query.iter_mut() {sprt.color = &self.stroke.color.clone()}     //  expected enum `bevy::prelude::Color`, found `&Color32`
                //for mut sprt in query.iter_mut() {sprt.color = &self.stroke.color.as_rgba().clone()}

                //for mut sprt in query.iter_mut() {sprt.color = bevy::prelude::Color::as_rgba(&self.stroke.color);}

                for mut sprt in query.iter_mut() {sprt.color = bevy::prelude::Color::GOLD }


                //bevy::prelude::Color::as_rgba(&self.stroke.color)
            }
        })
        .response
    }




    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        //let (response, painter) =
            //ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());
        //let rect = response.rect;

        
    }
}

//+++++++++++++++++++++

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    

    // Add bricks
    //let brick_rows = 4;
    let brick_rows = 7;
    //let brick_columns = 5;
    let brick_columns = 10;

    //let brick_spacing = 20.0;
    let brick_spacing = 5.0;

    //let brick_size = Vec2::new(150.0, 30.0);
    let brick_size = Vec2::new(100.0, 100.0);

    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
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
                    //material: brick_material.clone(),
                    //texture: brick_material.clone(),
                    //color: brick_material.clone(),
                    //sprite: Sprite::new(brick_size),
                    sprite: Sprite {
                        //custom_size: Some(Vec2::new(brick_size.0, brick_size.1)),
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        color: BRICK_COLOR,
                        ..Default::default()
                    },
                    //texture: asset_server.load("branding/icon.png"),
                    transform: Transform::from_translation(brick_position),
                    ..Default::default()
                //})
                })
                //.insert(Collider::Scorable);
                .insert(RotRight)
                .insert(Vertimov)
                .insert(ScalMorph);
        }
    }
}




