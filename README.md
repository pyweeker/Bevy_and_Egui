# Bevy_and_Egui

trying to work with Bevy 0.6 + Egui thanks to bevy_egui

DONE
+ rotation
+ translation : introducing queryset due to 2 query on &mut Transform

+ scale, dirty code in stroke paint
+ change color to GOLD when touching strocke widget because problem conversion <== expected enum `bevy::prelude::Color`, found struct `Color32`
+ press right arrow increment index indicator for later select entities for inspection and modify his attributes. 


//mut queryRot: Query<(&mut Transform, &RotRight)>,
//mut queryVertimov: Query<(&mut Transform, &Vertimov)>,

==>

    mut q: QuerySet<(
        QueryState<&mut Transform, With<RotRight>>,  
        QueryState<&mut Transform, With<Vertimov>>
    )>,

=============

TODO
- colors
- display info / hovering
- single select / click
- multiselect / click and drag
