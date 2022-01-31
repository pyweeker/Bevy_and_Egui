use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
//use rand::Rng;
use rand::prelude::*;

const BEVY_TEXTURE_ID: u64 = 0;
const BEVY_TEXTURE_ID_ONE: u64 = 1;

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


//fn rnd_pick_filename(array_names: &[&str; 9]) {
fn rnd_pick_filename<'a>(array_names: &'a[&str; 9]) -> &'a str {

    let pickedName = array_names.choose(&mut rand::thread_rng());

    return pickedName.unwrap();
    
}





//const LOADEDTEXTUR0

//texture: asset_server.load(TEXTURE_FILENAME0)


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
    //texture_filename: &str,
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

        //.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        //.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        
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





//#[derive(Default)]
//struct UiState {
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

    vec_entities: Vec<Entity>,
    //vec_texturenames: Vec<&str>,
    vec_texturenames: Vec<&'static str>,


}

impl Default for UiState {
    fn default() -> Self {
        
        UiState { value: 3.5, rotval: 0.0, vertimov: 0.0, painting: Painting::default(), inverted: true, name: "the_name".to_string(), age: 18, current_target_idx: None, current_target_idx_indicator: "No Target Yet".to_string(), vec_entities: Vec::<Entity>::new(), vec_texturenames: Vec::<&str>::new()}
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
            if ui.button("inspec Painting").clicked() {
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
 
    let brick_rows = 3;
    let brick_columns = 5;

    let brick_spacing = 5.0;


    let brick_size = Vec2::new(128.0, 128.0);

    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    

    //let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());


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

            let picked_name = rnd_pick_filename(&ARRAY_TEXTURE_FILENAME);

            println!(" picked_name =>  {:?} ", picked_name);

            print_type_of(&picked_name);


            //let new_entity = commands.spawn().id();
            let mut new_entity = commands.spawn();
            new_entity.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128.0, 128.0)),
                    color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                    ..Default::default()
                    },
                texture: asset_server.load(rnd_pick_filename(&ARRAY_TEXTURE_FILENAME)),
                transform: Transform::from_translation(brick_position),
                ..Default::default()
            })


            .insert(RotRight)
            .insert(Vertimov)
            .insert(ScalMorph)
            .insert(KeyboardTargetable)

            .insert(Contributor { hue },)
            .insert(TextureNameCompo { texture_filename: picked_name },)


            .insert(Idx_of_entity { idx: ui_state.vec_entities.len() },);




            

                

            ui_state.vec_entities.push(new_entity.id());
            ui_state.vec_texturenames.push(picked_name);
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

    mut queryIDX: Query<(Entity, &Idx_of_entity)>,

    //queryIdxNametextur: Query<(&Idx_of_entity, &TextureNameCompo)>,  // lifetime `'static` required
    queryIdxNametextur: Query<(&'static Idx_of_entity, &'static TextureNameCompo)>,


    mut egui_ctx: ResMut<EguiContext>,
    assets: Res<AssetServer>,


    


) {
    //let (paddle, mut transform) = query.single_mut();
    //let mut direction = 0.0;
    

    //if keyboard_input.pressed(KeyCode::Left) {
    //    direction -= 1.0;
    //}



    //  https://bevy-cheatbook.github.io/features/input-handling.html

    //if keys.just_pressed(KeyCode::Tab) {
    if keyboard_input.just_pressed(KeyCode::Right) {

        println!(" keyboard_input.just_pressed(KeyCode::Right) ");



        if ui_state.current_target_idx.is_none() {
            println!(" ui_state.current_target_idx.is_none() ");
            ui_state.current_target_idx = Some(0);
            println!(" ... NOW ui_state.current_target_idx = Some(0)");

            //interview_crab();

            let current_usize: usize = ui_state.current_target_idx.unwrap();
            println!(" ui_state.vec_texturenames[current_usize] = {} ", ui_state.vec_texturenames[current_usize]);

        } else {

            let previous = ui_state.current_target_idx.unwrap();
            let nextone = previous + 1;
            ui_state.current_target_idx = Some(nextone);  // https://github.com/bevyengine/bevy/discussions/1205

            // https://www.reddit.com/r/bevy/comments/qeicc1/having_a_2d_array_of_entities/
            // https://en.m.wikipedia.org/wiki/Bidirectional_map
            // https://github.com/bevyengine/bevy/blob/07ed1d053e7946a116ce3eef273fc93dd246f49d/crates/bevy_core/src/label.rs#L68


            print_type_of_mut_version(&mut ui_state.current_target_idx);

            println!(" ui_state.current_target_idx = {:?} ", ui_state.current_target_idx);


            let current_usize: usize = ui_state.current_target_idx.unwrap();

            println!(" ui_state.vec_texturenames[current_usize] = {} ", ui_state.vec_texturenames[current_usize]);

            //$$$$$$$$$$$$$$$$$$$$$

            let fresh_selected_texture_handle_ONE = assets.load(ui_state.vec_texturenames[current_usize]);
            egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, fresh_selected_texture_handle_ONE);

            //$$$$$$$$$$$$$$$$$$$$$$

            
        }




    }

    if keyboard_input.just_pressed(KeyCode::Space) {

     //for foobar in ui_state.vec_entities.iter() {println!("{:?}", foobar); print_type_of(foobar);}

     

     //for (e, idx_of_e) in queryIDX .iter() {println!("{}", idx_of_e); } // `Idx_of_entity` cannot be formatted with the default formatter
     //for (e, idx_of_e) in queryIDX .iter() { print_type_of(idx_of_e); } // george::Idx_of_entity



     //********************************************************************************************************************************************
     //for (e, idx_of_e) in queryIDX .iter() {println!("for (e, idx_of_e) in queryIDX .iter() ..>  println    idx_of_e.idx = {}", idx_of_e.idx); }
     //everybody_tell_texturenames();

     for (idx_of_e, texturename) in queryIdxNametextur.iter() {  // explicit lifetime required in the type of `queryIdxNametextur`

        println!(
                "   idx_of_e.idx  =  {}    ;    texturename.texture_filename = {}",
                idx_of_e.idx, texturename.texture_filename
            );

    }
     //********************************************************************************************************************************************






     //for foobar in ui_state.vec_entities.iter() {println!("{:#?}", foobar); print_type_of(foobar); foobar.show_details();}

     //for foobar in ui_state.vec_entities.iter() {for bidul in foobar.iter() {println!("{:?}", bidul);} }



     //for foobar in ui_state.vec_entities.iter() {println!("{:?}", foobar.idx);}
     //for foobar in ui_state.vec_entities.iter() {println!("{:?}", foobar.Idx_of_entity);}

     //for foobar in ui_state.vec_entities.iter() {println!("{:?}", foobar.Idx_of_entity.idx);}

     

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
        //println!("indexboard_system  says   {}", &s); // SPAM la console !!!

        //text.sections[1].value = format!("{}", indexboard.idx.unwrap());
        text.sections[1].value = format!("{}", ui_state.current_target_idx.unwrap());


        
    } 

}