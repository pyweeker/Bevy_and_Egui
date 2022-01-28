# Bevy_and_Egui

trying to work with Bevy 0.6 + Egui

DONE
+ rotation
+ translation : introducing queryset due to 2 query on &mut Transform


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
- size
- display info / hovering
- single select / click
- multiselect / click and drag
