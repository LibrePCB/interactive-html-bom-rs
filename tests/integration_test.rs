use interactive_html_bom::*;

#[test]
fn test_empty() {
  let bom = InteractiveHtmlBom::new(
    "Test Title",
    "Test Company",
    "Test Revision",
    "Test Date",
    (0.0, 0.0),
    (0.0, 0.0),
  );

  let html = bom.generate_html().unwrap();
  assert!(html.contains("<html"));
}

#[test]
fn test_everything() {
  let mut bom = InteractiveHtmlBom::new(
    "Test Title",
    "Test Company",
    "Test Revision",
    "Test Date",
    (0.0, 0.0),
    (100.0, 100.0),
  );

  bom.dark_mode = true;
  bom.show_silkscreen = false;
  bom.show_fabrication = false;
  bom.checkboxes = vec!["Foo".into(), "Bar".into()];
  bom.fields = vec!["Field 1".into(), "Field 2".into()];
  bom.user_header = "<!-- header -->".into();
  bom.user_footer = "<!-- footer -->".into();
  bom.user_js = "<!-- js -->".into();

  bom.drawings.push(Drawing::new(
    DrawingKind::Polygon,
    DrawingLayer::Edge,
    "",
    0.1,
    false,
  ));
  bom.drawings.push(Drawing::new(
    DrawingKind::Polygon,
    DrawingLayer::SilkscreenFront,
    "M 0 0",
    0.1,
    false,
  ));
  bom.drawings.push(Drawing::new(
    DrawingKind::ReferenceText,
    DrawingLayer::SilkscreenBack,
    "",
    0.1,
    false,
  ));
  bom.drawings.push(Drawing::new(
    DrawingKind::Polygon,
    DrawingLayer::FabricationFront,
    "M 0 0",
    0.1,
    false,
  ));
  bom.drawings.push(Drawing::new(
    DrawingKind::ValueText,
    DrawingLayer::FabricationBack,
    "M 0 0",
    0.1,
    false,
  ));

  bom.tracks.push(Track::new(
    Layer::Front,
    (0.0, 0.0),
    (100.0, 100.0),
    1.0,
    None,
  ));
  bom.tracks.push(Track::new(
    Layer::Back,
    (0.0, 0.0),
    (100.0, 100.0),
    1.0,
    Some("net 1"),
  ));

  bom
    .vias
    .push(Via::new(&[Layer::Front], (50.0, 50.0), 1.0, 0.5, None));
  bom.vias.push(Via::new(
    &[Layer::Front, Layer::Back],
    (50.0, 50.0),
    1.0,
    0.5,
    Some("net 2"),
  ));

  bom.zones.push(Zone::new(Layer::Front, "M 0 0", None));
  bom
    .zones
    .push(Zone::new(Layer::Back, "M 0 0", Some("net 3")));

  bom.footprints.push(Footprint::new(
    Layer::Front,
    (50.0, 50.0),
    45.0,
    (-5.0, -5.0),
    (5.0, 5.0),
    &["Value 1".into(), "Value 2".into()],
    &[],
    false,
  ));
  bom.footprints.push(Footprint::new(
    Layer::Front,
    (50.0, 50.0),
    45.0,
    (-5.0, -5.0),
    (5.0, 5.0),
    &["Value 1".into(), "Value 2".into()],
    &[
      Pad::new(
        &[Layer::Front],
        (0.0, -5.0),
        45.0,
        "M 0 0",
        None,
        None,
        false,
      ),
      Pad::new(
        &[Layer::Front, Layer::Back],
        (0.0, 5.0),
        45.0,
        "M 0 0",
        Some((0.5, 1.0)),
        Some("net 4"),
        true,
      ),
    ],
    true,
  ));

  bom
    .bom_front
    .push(vec![RefMap::new("R1", 0), RefMap::new("R2", 1)]);
  bom
    .bom_back
    .push(vec![RefMap::new("R1", 0), RefMap::new("R2", 1)]);
  bom.bom_both.push(vec![RefMap::new("R1", 0)]);
  bom.bom_both.push(vec![RefMap::new("R2", 1)]);

  let html = bom.generate_html().unwrap();
  assert!(html.contains("<html"));
}

#[test]
fn test_invalid_footprint_id() {
  let mut bom = InteractiveHtmlBom::new(
    "Test Title",
    "Test Company",
    "Test Revision",
    "Test Date",
    (0.0, 0.0),
    (100.0, 100.0),
  );

  bom.bom_both.push(vec![RefMap::new("R1", 0)]);

  let err = bom.generate_html().unwrap_err();
  assert_eq!(err, "Invalid footprint ID.");
}

#[test]
fn test_inconsistent_fields() {
  let mut bom = InteractiveHtmlBom::new(
    "Test Title",
    "Test Company",
    "Test Revision",
    "Test Date",
    (0.0, 0.0),
    (100.0, 100.0),
  );

  bom.fields = vec!["Field 1".into()];

  bom.footprints.push(Footprint::new(
    Layer::Front,
    (50.0, 50.0),
    45.0,
    (-5.0, -5.0),
    (5.0, 5.0),
    &["Value 1".into(), "Value 2".into()],
    &[],
    false,
  ));

  let err = bom.generate_html().unwrap_err();
  assert_eq!(err, "Inconsistent number of fields.");
}
