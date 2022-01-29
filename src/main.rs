use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

const BEVY_TEXTURE_ID: u64 = 0;
const BEVY_TEXTURE_ID_ONE: u64 = 1;

const BRICK_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);


#[derive(Component)]
struct RotRight; 


#[derive(Component)]
struct Vertimov; 


#[derive(Component)]
struct ScalMorph;


#[derive(Component)]
struct KeyboardTargetable;


//#[derive(Default)]
//struct SnakeSegments(Vec<Entity>);



fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    let texture_handle = assets.load("icon.png");
    egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);

    let texture_handle_ONE = assets.load("python_icon.png");
    egui_context.set_egui_texture(BEVY_TEXTURE_ID_ONE, texture_handle_ONE);
}



fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.9, 0.0)))
        .insert_resource(Msaa { samples: 4 })

        //.insert_resource(indexboard { idx: 0 })
        .insert_resource(indexboard { idx: None })

        //.insert_resource(SnakeSegments::default())


        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_startup_system(setup.system())

        .add_system(ui_example)

        .add_system(keyboard_system)

        .add_system(indexboard_system)


        

        .run();
}



struct indexboard {
    //idx: usize,
    idx: Option<usize>,
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

    current_target_idx: Option<usize>,
    current_target_idx_indicator: String,

    souls: Vec<Entity>,

}

impl Default for UiState {
    fn default() -> Self {
        
        //UiState { painting: Painting::default() }
        //UiState { label: "the_label".to_string(), value: 3.5, inverted: true, name: "the_name".to_string(), age: 18, }
        //UiState { value: 3.5, rotval: 0.0, vertimov: 0.0, inverted: true, name: "the_name".to_string(), age: 18, }
        //UiState { value: 3.5, rotval: 0.0, vertimov: 0.0, painting: Painting::default(), inverted: true, name: "the_name".to_string(), age: 18, current_target_idx: None}
        UiState { value: 3.5, rotval: 0.0, vertimov: 0.0, painting: Painting::default(), inverted: true, name: "the_name".to_string(), age: 18, current_target_idx: None, current_target_idx_indicator: "No Target Yet".to_string(), souls: Vec::<Entity>::new()}
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

            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID),
                [256.0, 256.0],
            ));

            ui.separator();

            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID_ONE),
                [128.0, 128.0],
            ));




     });
    //-----------

    if invert {
        ui_state.inverted = !ui_state.inverted;
    }
    if load || invert {
        let texture_handle = if ui_state.inverted {
            assets.load("icon_inverted.png")
        } else {
            assets.load("icon.png")
        };
        egui_ctx.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
    }
    if remove {
        egui_ctx.remove_egui_texture(BEVY_TEXTURE_ID);
    }

    let texture_handle_ONE = assets.load("python_icon.png");
    egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, texture_handle_ONE);

    //-----------
    
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

    //mut segments_res: ResMut<SnakeSegments>,
    
    //segments: Vec<Entity>,
    mut ui_state: ResMut<UiState>,

) {

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    

    // Add bricks
    //let brick_rows = 4;
    let brick_rows = 5;
    //let brick_columns = 5;
    let brick_columns = 7;

    //let brick_spacing = 20.0;
    let brick_spacing = 5.0;

    //let brick_size = Vec2::new(150.0, 30.0);
    let brick_size = Vec2::new(32.0, 32.0);

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


            //segments.push(commands            
            commands
                .spawn_bundle(SpriteBundle {
                    //material: brick_material.clone(),
                    //texture: brick_material.clone(),
                    //color: brick_material.clone(),
                    //sprite: Sprite::new(brick_size),
                    sprite: Sprite {
                        //custom_size: Some(Vec2::new(brick_size.0, brick_size.1)),
                        //custom_size: Some(Vec2::new(50.0, 50.0)),
                        custom_size: Some(Vec2::new(32.0, 32.0)),


                        color: BRICK_COLOR,
                        ..Default::default()
                    },
                    
                    texture: asset_server.load("crab_32.png"),
                    transform: Transform::from_translation(brick_position),
                    ..Default::default()
                //})
                })
                //.insert(Collider::Scorable);
                .insert(RotRight)
                .insert(Vertimov)
                .insert(ScalMorph)
                .insert(KeyboardTargetable);

            //segments.push(new_entity);
        }
    }


    // indexboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "idx: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(350.0),
                left: Val::Px(355.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

//------------

fn keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    //mut query: Query<(&KeyboardTargetable, &mut Transform)>,

    mut ui_state: ResMut<UiState>,

    mut query: Query<(&KeyboardTargetable, &Transform)>, // LOOK only
) {
    //let (paddle, mut transform) = query.single_mut();
    //let mut direction = 0.0;
    

    //if keyboard_input.pressed(KeyCode::Left) {
    //    direction -= 1.0;
    //}

    //if keyboard_input.pressed(KeyCode::Right) {
    //    direction += 1.0;
    //}

    //  https://bevy-cheatbook.github.io/features/input-handling.html

    //if keys.just_pressed(KeyCode::Tab) {
    if keyboard_input.just_pressed(KeyCode::Right) {

        println!(" keyboard_input.just_pressed(KeyCode::Right) ");



        if ui_state.current_target_idx.is_none() {
            println!(" ui_state.current_target_idx.is_none() ");
            ui_state.current_target_idx = Some(0);
            println!(" ... NOW ui_state.current_target_idx = Some(0)");       
        } else {

            let previous = ui_state.current_target_idx.unwrap();
            let nextone = previous + 1;
            ui_state.current_target_idx = Some(nextone);  // https://github.com/bevyengine/bevy/discussions/1205

            // https://www.reddit.com/r/bevy/comments/qeicc1/having_a_2d_array_of_entities/
            // https://en.m.wikipedia.org/wiki/Bidirectional_map
            // https://github.com/bevyengine/bevy/blob/07ed1d053e7946a116ce3eef273fc93dd246f49d/crates/bevy_core/src/label.rs#L68


            print_type_of_mut_version(&mut ui_state.current_target_idx);

            println!(" ui_state.current_target_idx = {:?} ", ui_state.current_target_idx);

            
        }




    }

    //let translation = &mut transform.translation;
    // move the paddle horizontally
    //translation.x += //direction * paddle.speed * TIME_STEP;
    // bound the paddle within the walls
    //translation.x = translation.x.min(380.0).max(-380.0);
}

//https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust?rq=1

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print_type_of_mut_version<T>(_: &mut T) {
    println!("{}", std::any::type_name::<T>())
}



fn indexboard_system(indexboard: Res<indexboard>, ui_state: ResMut<UiState>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    //text.sections[1].value = format!("{}", indexboard.idx);
    //text.sections[1].value = format!("{}", indexboard.idx.unwrap());

    //if let Some(s) = indexboard.idx.unwrap() {
    //if let Some(s) = indexboard.idx {
    if let Some(s) = ui_state.current_target_idx {
        println!("{}", &s);
        //text.sections[1].value = format!("{}", indexboard.idx.unwrap());
        text.sections[1].value = format!("{}", ui_state.current_target_idx.unwrap());


        
    } //else { text.sections[1].value = format!(" NONE "); }

    //else { print_type_of(&indexboard.idx); }
}