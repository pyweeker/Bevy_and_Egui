# Bevy_and_Egui

trying to work with Bevy 0.6 + Egui thanks to bevy_egui

DONE
+ rotation
+ translation : introducing queryset due to 2 query on &mut Transform

+ scale, dirty code in stroke paint
+ change color to GOLD when touching strocke widget because problem conversion <== expected enum `bevy::prelude::Color`, found struct `Color32`
+ press right arrow increment index indicator for later select entities for inspection and modify his attributes. 
+ dirty random colors
+ transparent texture which in reality seems opaque or mask effect when applying new color
+ right arrow increment index of current selected sprite, indicate it in Left Panel (logo only, his color not yet) , draws a square on selected sprite

//mut queryRot: Query<(&mut Transform, &RotRight)>,
//mut queryVertimov: Query<(&mut Transform, &Vertimov)>,

==>

    mut q: QuerySet<(
        QueryState<&mut Transform, With<RotRight>>,  
        QueryState<&mut Transform, With<Vertimov>>
    )>,

=============

TODO
-clone color of current selected sprite inside left panel ; unselect when pressing again and again right arrow.

-change Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA) with buttons

-Adapter for Color type coming from paint brush widget instead of gold hack.
-find interesting widgets

-==>>> use /examples/2d/contrib.rs to select one entity only instead of mass effect

- display info / hovering
- single select / click
- multiselect / click and drag
