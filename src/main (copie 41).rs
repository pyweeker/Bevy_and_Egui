use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
//use rand::Rng;
use rand::prelude::*;

//++++++++
//use bevy_egui::{egui::Widget};


//use bevy_prototype_lyon::prelude::*;

const BEVY_TEXTURE_ID: u64 = 0;
const BEVY_TEXTURE_ID_ONE: u64 = 1;

const BEVY_TEXTURE_ID_SECOND: u64 = 2;

//const BRICK_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
//const BRICK_COLOR: Color = Color::rgba(0.7, 0.7, 0.7, 0.5);


//const SATURATION_DESELECTED: f32 = 0.3;
//const LIGHTNESS_DESELECTED: f32 = 0.2;

const SATURATION_DESELECTED: f32 = 0.5;
const LIGHTNESS_DESELECTED: f32 = 0.4;

const SATURATION_SELECTED: f32 = 0.9;
const LIGHTNESS_SELECTED: f32 = 0.7;
const ALPHA: f32 = 0.92;

const TEXTURE_FILENAME0 : &str = "crab0.png";
const TEXTURE_FILENAME1 : &str = "crab1.png";
const TEXTURE_FILENAME2 : &str = "crab2.png";
const TEXTURE_FILENAME3 : &str = "crab3.png";

const TEXTURE_FILENAME4 : &str = "crab4.png";
const TEXTURE_FILENAME5 : &str = "crab5.png";
const TEXTURE_FILENAME6 : &str = "crab6.png";
const TEXTURE_FILENAME7 : &str = "crab7.png";
const TEXTURE_FILENAME8 : &str = "crab8.png";

const ARRAY_TEXTURE_FILENAME : [&str; 9] = [TEXTURE_FILENAME0, TEXTURE_FILENAME1, TEXTURE_FILENAME2, TEXTURE_FILENAME3, TEXTURE_FILENAME4, TEXTURE_FILENAME5, TEXTURE_FILENAME6, TEXTURE_FILENAME7, TEXTURE_FILENAME8];

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


fn rnd_pick_filename<'a>(array_names: &'a[&str; 9]) -> &'a str {
    let pickedName = array_names.choose(&mut rand::thread_rng());
    println!("__Call fn rnd_pick_filename  => give std::option::Option<&&str>    unwraping it = {}", pickedName.unwrap());
    return pickedName.unwrap();    
}


#[derive(Component)]
struct RotRight; 

#[derive(Component)]
struct Vertimov; 

#[derive(Component)]
struct ScalMorph;

#[derive(Component)]
struct KeyboardTargetable;

#[derive(Component)]
struct TextureNameCompo<'a> {
    texture_filename: &'a str,
}

#[derive(Component)]
struct Idx_of_entity {
    idx: usize,
}

impl Idx_of_entity {
fn show_details(self: &Self) {
    println!("The employee ID is {}", self.idx);
    }  
}


//...............................................
#[derive(Component)]
struct Color_Tracker {
    color_tracked: bevy::prelude::Color,
}
//...............................................

#[derive(Component)]
struct Fluo;

#[derive(Component)]
struct Cleaner_job {
    index_of_owner: usize,

}


//#[derive(Component, Widget)]       //   `Widget` is imported here, but it is only a trait, without a derive macro
#[derive(Component)]
struct Compo_egui_widget;


fn main() {
    App::new()
        
        .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        
        .insert_resource(indexboard { idx: None })

        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_startup_system(setup.system())        

        .add_system(ui_example)

        .add_system(keyboard_system)

        .add_system(indexboard_system) // besoin de le mettre dans le system ? (cycles de refresh ?)

        .add_system(fluo_system)
        .add_system(clean_colored_rect_system)
        .add_system(button_system)

        .run();
}

struct indexboard {
    //idx: usize,
    idx: Option<usize>,
}

#[derive(Component)]
struct Contributor {
    hue: f32,
}

struct UiState {
    //label: String,
    scalvalue: f32,
    rotval: f32,
    vertimov: f32,
    painting: Painting,
    inverted: bool,

    name: String,
    age: u32 ,

    current_target_idx: Option<usize>,
    current_png_String: String,

    vec_entities: Vec<Entity>,
    //vec_texturenames: Vec<&str>,
    vec_texturenames: Vec<&'static str>,

    vec_entities_colors: Vec<bevy::render::color::Color>,
}

impl Default for UiState {
    fn default() -> Self {        
        UiState { scalvalue: 1.2, rotval: 0.1, vertimov: -50.0, painting: Painting::default(), inverted: true, name: "the_name".to_string(), age: 18, current_target_idx: None, current_png_String: "".to_string(), vec_entities: Vec::<Entity>::new(), vec_texturenames: Vec::<&str>::new(), vec_entities_colors: Vec::<bevy::render::color::Color>::new()}
    }
}


fn make_colored_rect(mut commands: Commands,) {

    let mut rnd = rand::thread_rng();

    let hue = rnd.gen_range(0.0..=360.0);
    let brick_position = Vec3::new(300.0, 300.0, 0.0);


    let mut colored_rect = commands.spawn();
        colored_rect.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                //color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                //color: the_color,
                color: Color::hsla(hue, 0.5, 0.5, 0.7),
                ..Default::default()
                },
            //texture: asset_server.load(rnd_pick_filename(&ARRAY_TEXTURE_FILENAME)),
            //texture: asset_server.load(picked_name),
            transform: Transform::from_translation(brick_position),
            ..Default::default()
        });
}

//+++++

fn make_colored_rect_with_transform(mut commands: Commands, tf: Transform) {

    //let mut rnd = rand::thread_rng();

    //let hue = rnd.gen_range(0.0..=360.0);
    let hue = 180.0;

    //let brick_position = Vec3::new(300.0, 300.0, 0.0);
    let the_tf_translation = tf.translation;

    


    let mut colored_rect = commands.spawn();
        colored_rect.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(128.0, 128.0)),
                //color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                //color: the_color,
                color: Color::hsla(hue, 0.5, 0.5, 0.3),
                ..Default::default()
                },
            //texture: asset_server.load(rnd_pick_filename(&ARRAY_TEXTURE_FILENAME)),
            //texture: asset_server.load(picked_name),
            transform: Transform::from_translation(the_tf_translation),
            ..Default::default()
        });
}

///_____

fn fluo_system(mut commands: Commands, queryFluo: Query<(&Fluo, &Transform)>,) {

    //let mut rnd = rand::thread_rng();

    //let hue = rnd.gen_range(0.0..=360.0);
    let hue180 = 180.0;
    //let brick_position = Vec3::new(300.0, 300.0, 0.0);


    for (_, target_tf) in queryFluo.iter() {
        //let mut target_fluo_tf = Transform::from_translation(target_tf);  //  expected struct `bevy::prelude::Vec3`, found `&bevy::prelude::Transform`
        //let mut target_fluo_tf = target_tf; // trait copy ?
        let mut target_fluo_tf = target_tf.clone(); // trait copy ?
     



     let mut fluo_rect = commands.spawn();
        fluo_rect.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(135.0, 135.0)),
                color: Color::hsla(hue180, 0.5, 0.5, 0.2),
                ..Default::default()
                },
            transform: target_fluo_tf,
            ..Default::default()
        });



    }


    
}








fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    //mut ui_state: ResMut<UiState>,
    mut ui_state: ResMut<UiState>,
    assets: Res<AssetServer>,


    //mut queryRot: Query<(&mut Transform, &RotRight)>,
    //mut queryVertimov: Query<(&mut Transform, &Vertimov)>,


    mut q: QuerySet<(
        QueryState<&mut Transform, With<RotRight>>,  
        QueryState<&mut Transform, With<Vertimov>>,
        QueryState<&mut Sprite>,
        QueryState<&mut Transform, With<ScalMorph>>,
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

            ui.add(egui::Slider::new(&mut ui_state.scalvalue, 0.5..=2.0).text("scalvalue"));
            if ui.button("ScaleBtn").clicked() {
                //ui_state.scalvalue += 1.0;
                for mut tf in q.q3().iter_mut() {tf.scale *= ui_state.scalvalue;}
            }


            ui.add(egui::Slider::new(&mut ui_state.rotval, 0.0..= 1.0).text("rotval!"));
            if ui.button("RotaBtn").clicked() {
                
                //for (mut tf, rot) in queryRot.iter_mut() { tf.rotate(Quat::from_rotation_z( ui_state.rotval )); }

                for mut tf in q.q0().iter_mut() { tf.rotate(Quat::from_rotation_z( ui_state.rotval )); }
            }


            ui.add(egui::Slider::new(&mut ui_state.vertimov, -80.0..= 10.0).text("vertimov!"));
            if ui.button("VertimovBtn").clicked() {
                
                //for (mut tf, vertimov) in queryVertimov.iter_mut() { tf.translation.y += ui_state.vertimov ; }

                for mut tf in q.q1().iter_mut() { tf.translation.y += ui_state.vertimov ; }


            }





            //ui_state.painting.ui_control(ui);
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

            ui.horizontal(|ui| {
                ui.add(egui::widgets::Image::new(
                    egui::TextureId::User(BEVY_TEXTURE_ID_ONE),
                    [128.0, 128.0],
                ));
                ui.separator();
                
                let machin = ui.add(egui::widgets::Image::new(
                    egui::TextureId::User(BEVY_TEXTURE_ID_SECOND),
                    [128.0, 128.0],
                ));

                //machin.insert(Color_Tracker);

            
            
            });
            //.response

            //ui.add(egui::widgets::Image::new(
            //    egui::TextureId::User(BEVY_TEXTURE_ID_ONE),
            //    [128.0, 128.0],
            //));

            ui.separator();




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




    match ui_state.current_png_String.as_str() {
        "" => {let texture_handle_ONE = assets.load("python_icon.png"); egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, texture_handle_ONE); let texture_handle_SECOND = assets.load("white_square_128.png") ;egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_SECOND, texture_handle_SECOND);},
        //"b" => println!("1"),
        //"c" => println!("2"),
        _ => {let texture_handle_ONE = assets.load(ui_state.current_png_String.as_str()); egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, texture_handle_ONE);},

    
}

    //if  == 

    //let texture_handle_ONE = assets.load("python_icon.png");
    //egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, texture_handle_ONE);

    //-----------
    
}


//fn interview_crab(fresh_texture_name: &str) {
//    let fresh_texture_handle = assets.load(fresh_texture_name);
//    egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, fresh_texture_handle);
//}




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
            if ui.button("GOLD !").clicked() {
                //self.lines.clear();
                println!("+a+  {:#?}", &self);
                println!("+b+ {:#?}", &self.stroke.color);

                //for mut tf in query.iter_mut() {tf.scale += 0.10}
                //for mut sprt in query.iter_mut() {sprt.color = &self.stroke.color.clone()}     //  expected enum `bevy::prelude::Color`, found `&Color32`
                //for mut sprt in query.iter_mut() {sprt.color = &self.stroke.color.as_rgba().clone()}

                //for mut sprt in query.iter_mut() {sprt.color = bevy::prelude::Color::as_rgba(&self.stroke.color);}

                for mut sprt in query.iter_mut() {sprt.color = bevy::prelude::Color::GOLD }


                //bevy::prelude::Color::as_rgba(&self.stroke.color)
            }


            if ui.button("PALETTE !").clicked() {
                //self.lines.clear();
                //println!("+a+  {:#?}", &self);
                println!("+b+ {:#?}", &self.stroke.color);

                print_type_of(&self.stroke.color);

                
                let col0 = *&self.stroke.color[0] as f32;
                let col1 = *&self.stroke.color[1].clone() as f32;
                let col2 = *&self.stroke.color[2].clone() as f32;
                let col3 = *&self.stroke.color[3].clone() as f32;

                for mut sprt in query.iter_mut() {sprt.color = bevy::prelude::Color::rgba(col0, col1, col2, col3) }




            }
        })
        .response
    }

// https://bevy-cheatbook.github.io/pitfalls/ui-y-up.html      pitfalls


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
 
    let brick_rows = 3;
    let brick_columns = 5;

    let brick_spacing = 50.0;


    let brick_size = Vec2::new(128.0, 128.0);

    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    //let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);

    let bricks_offset = Vec3::new(-100.0,-150.0, 0.0);
    



    let mut rnd = rand::thread_rng();

    





    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            // brick


            let hue = rnd.gen_range(0.0..=360.0);

            let the_color = Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA);

            let picked_name = rnd_pick_filename(&ARRAY_TEXTURE_FILENAME);

            println!(" picked_name =>  {:?} ", picked_name);

            print_type_of(&picked_name);


            //let new_entity = commands.spawn().id();
            let mut new_entity = commands.spawn();
            new_entity.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    //color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                    color: the_color,
                    ..Default::default()
                    },
                //texture: asset_server.load(rnd_pick_filename(&ARRAY_TEXTURE_FILENAME)),
                texture: asset_server.load(picked_name),
                transform: Transform::from_translation(brick_position),
                ..Default::default()
            })


            .insert(RotRight)
            .insert(Vertimov)
            .insert(ScalMorph)
            .insert(KeyboardTargetable)

            .insert(Contributor { hue },)
            .insert(TextureNameCompo { texture_filename: picked_name },)


            .insert(Idx_of_entity { idx: ui_state.vec_entities.len() },) //;

            //.insert(Interaction::None);
            //.insert(Interaction::Hovered);

            .insert(Compo_egui_widget);

                

            ui_state.vec_entities.push(new_entity.id());
            ui_state.vec_texturenames.push(picked_name);
            ui_state.vec_entities_colors.push(the_color);
        }
    }


    // indexboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "PRESS RIGHT ARROW   ->   idx: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
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
                top: Val::Px(650.0),
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

    mut queryIDX: Query<(Entity, &Idx_of_entity)>,

    //queryIdxNametextur: Query<(&Idx_of_entity, &TextureNameCompo)>,  // lifetime `'static` required
    queryIdx_Nametextur_and_Transform: Query<(&'static Idx_of_entity, &'static TextureNameCompo, &Transform)>,

    mut egui_ctx: ResMut<EguiContext>,
    assets: Res<AssetServer>,

    mut commands: Commands,

) {
    
    //  https://bevy-cheatbook.github.io/features/input-handling.html

    if keyboard_input.just_pressed(KeyCode::Right) {

        println!(" keyboard_input.just_pressed(KeyCode::Right) ");

        if ui_state.current_target_idx.is_none() {
            println!(" ui_state.current_target_idx.is_none() ");
            ui_state.current_target_idx = Some(0);
            println!(" ... NOW ui_state.current_target_idx = Some(0)");

            let current_usize: usize = ui_state.current_target_idx.unwrap();
            println!(" ui_state.vec_texturenames[current_usize] = {} ", ui_state.vec_texturenames[current_usize]);

            let fresh_selected_texture_handle_ONE = assets.load(ui_state.vec_texturenames[current_usize]);
            egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, fresh_selected_texture_handle_ONE);
            ui_state.current_png_String = ui_state.vec_texturenames[current_usize].to_string();

            for (idx_of_e, texturename, tf) in queryIdx_Nametextur_and_Transform.iter() {                  

                if idx_of_e.idx == current_usize {
                    let hue = 180.0;
                    let the_tf_translation = tf.translation;

                    let mut colored_rect = commands.spawn();
                        colored_rect.insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(128.0, 128.0)),
                                //color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                                color: Color::hsla(hue, 0.5, 0.5, 0.3),
                                ..Default::default()
                                },
                            //texture: asset_server.load(picked_name),
                            transform: Transform::from_translation(the_tf_translation),
                            ..Default::default()
                        });
                    colored_rect.insert(Cleaner_job {index_of_owner: idx_of_e.idx});
                }
            }            

        } else {

            let previous = ui_state.current_target_idx.unwrap();
            let nextone = previous + 1;
            ui_state.current_target_idx = Some(nextone);  

            print_type_of_mut_version(&mut ui_state.current_target_idx);
            println!(" ui_state.current_target_idx = {:?} ", ui_state.current_target_idx);
            let current_usize: usize = ui_state.current_target_idx.unwrap();
            println!(" ui_state.vec_texturenames[current_usize] = {} ", ui_state.vec_texturenames[current_usize]);

            let fresh_selected_texture_handle_ONE = assets.load(ui_state.vec_texturenames[current_usize]);
            egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, fresh_selected_texture_handle_ONE);

            ui_state.current_png_String = ui_state.vec_texturenames[current_usize].to_string();

            for (idx_of_e, texturename, tf) in queryIdx_Nametextur_and_Transform.iter() {                  

                if idx_of_e.idx == current_usize {
                    let hue = 180.0;
                    let the_tf_translation = tf.translation;

                    let mut colored_rect = commands.spawn();
                        colored_rect.insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(128.0, 128.0)),
                                //color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                                color: Color::hsla(hue, 0.5, 0.5, 0.3),
                                ..Default::default()
                                },
                            //texture: asset_server.load(picked_name),
                            transform: Transform::from_translation(the_tf_translation),
                            ..Default::default()
                        });

                    colored_rect.insert(Cleaner_job {index_of_owner: idx_of_e.idx});
                }
            }            
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) {

    for (idx_of_e, texturename, tf) in queryIdx_Nametextur_and_Transform.iter() {  

        println!(
                
                "   idx_of_e.idx  =  {}    ;    texturename.texture_filename = {}     tf.translation = {} ",                
                idx_of_e.idx, texturename.texture_filename, tf.translation.y
            );
    }
    }
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

    if let Some(s) = ui_state.current_target_idx {        
        text.sections[1].value = format!("{}", ui_state.current_target_idx.unwrap());        
    } 
}

fn clean_colored_rect_system(
    mut ui_state: ResMut<UiState>,
    mut commands: Commands, 
    mut query_cleanable: Query<(Entity, &Cleaner_job)>,
    
    ) {
        let current_usize : Option<usize>;
        current_usize = ui_state.current_target_idx;
        match current_usize {            
            None => {},
            _ => for (e, cj) in query_cleanable.iter() {if cj.index_of_owner != current_usize.unwrap() {commands.entity(e).despawn(); }},
    }
}


////

/*
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
*/




fn button_system(
    mut interaction_query: Query<
        //(&Interaction),
        &Interaction,
        (Changed<Interaction>, With<Idx_of_entity>),
        >,
    
) {
    
    for interaction in interaction_query.iter_mut() {
        
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Press".to_string();
                //*color = PRESSED_BUTTON.into();
                
                //println!("Clicked   {}", Idx_of_entity::idx);
                println!("Clicked   ");
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                //*color = HOVERED_BUTTON.into();
                
                //println!("Hovered   {}", Idx_of_entity.idx);
                println!("Hovered   ");
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                //*color = NORMAL_BUTTON.into();
                //println!("None {:?}", );
            }
        }
    }
}

/*

fn tag_entites_to_be_widget(
    mut entities_to_tag_query: Query<(Entity, With<Idx_of_entity>),>,) {

    //let tag_entities_widget = |e| impl Widget for &mut e { };        expected expression
    //let tag_entities_widget = |e| impl Widget for &mut e { }      expected expression
    //let tag_entities_widget = |e| {impl Widget for &mut e { }}

    //let tag_entities_widget = |e| {impl Widget for &mut e { }};

    for e in entities_to_tag_query.iter_mut() {

        //impl Widget for &mut e { }      // can't capture dynamic environment in a fn item       help: use the `|| { ... }` closure form instead

        //let tag_entities_widget = |e| {impl Widget for &mut e { }};
        let tag_entities_widget = move |e| {impl Widget for &mut e { }};

        tag_entities_widget(e);
    }

}

*/