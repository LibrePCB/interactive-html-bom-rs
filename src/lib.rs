//! Interactive HTML BOM Generator

// Fail on warnings if feature "fail-on-warnings" is enabled.
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![warn(missing_docs)]

use jzon::{array, object, JsonValue};

trait ToJson {
  fn to_json(&self) -> JsonValue;
}

impl ToJson for usize {
  fn to_json(&self) -> JsonValue {
    (*self).into()
  }
}

impl ToJson for String {
  fn to_json(&self) -> JsonValue {
    self.clone().into()
  }
}

impl ToJson for (f32, f32) {
  fn to_json(&self) -> JsonValue {
    array![self.0, self.1]
  }
}

impl<T: ToJson> ToJson for Vec<T> {
  fn to_json(&self) -> JsonValue {
    let mut arr = array![];
    for item in self {
      arr.push(item.to_json()).unwrap();
    }
    arr
  }
}

/// Layer enum
#[derive(Clone, PartialEq)]
pub enum Layer {
  /// Front layer
  Front,
  /// Back layer
  Back,
}

impl ToJson for Layer {
  fn to_json(&self) -> JsonValue {
    match self {
      Layer::Front => "F".into(),
      Layer::Back => "B".into(),
    }
  }
}

/// Drawing kind
#[derive(PartialEq)]
pub enum DrawingKind {
  /// Polygon
  Polygon,
  /// Component reference designator text
  ReferenceText,
  /// Component value text
  ValueText,
}

/// Drawing layer
#[derive(PartialEq)]
pub enum DrawingLayer {
  /// PCB edge
  Edge,
  /// Silkscreen front
  SilkscreenFront,
  /// Silkscreen back
  SilkscreenBack,
  /// Fabrication front
  FabricationFront,
  /// Fabrication back
  FabricationBack,
}

/// Drawing structure (SVG polygon)
#[non_exhaustive]
pub struct Drawing {
  kind: DrawingKind,
  layer: DrawingLayer,
  svgpath: String,
  width: f32,
  filled: bool,
}

impl Drawing {
  /// Construct drawing
  ///
  /// # Arguments
  ///
  /// * `kind` - Drawing kind.
  /// * `layer` - Drawing layer.
  /// * `svgpath` - Outline as an SVG path \[mm\].
  /// * `width` - Line width \[mm\].
  /// * `filled` - Whether to fill the shape or not.
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(
    kind: DrawingKind,
    layer: DrawingLayer,
    svgpath: &str,
    width: f32,
    filled: bool,
  ) -> Drawing {
    Drawing {
      kind,
      layer,
      svgpath: svgpath.to_owned(),
      width,
      filled,
    }
  }
}

impl ToJson for Drawing {
  fn to_json(&self) -> JsonValue {
    let mut obj = object! {
      svgpath: self.svgpath.clone(),
      filled: self.filled,
    };
    match self.kind {
      DrawingKind::Polygon => {
        obj["type"] = "polygon".into();
        obj["width"] = self.width.into();
      }
      DrawingKind::ReferenceText => {
        obj["thickness"] = self.width.into();
        obj["ref"] = 1.into();
      }
      DrawingKind::ValueText => {
        obj["thickness"] = self.width.into();
        obj["val"] = 1.into();
      }
    }
    obj
  }
}

/// Track structure
#[non_exhaustive]
pub struct Track {
  layer: Layer,
  start: (f32, f32),
  end: (f32, f32),
  width: f32,
  net: Option<String>,
}

impl Track {
  /// Construct track
  ///
  /// # Arguments
  ///
  /// * `layer` - Layer.
  /// * `start` - Start position (x, y) \[mm\].
  /// * `end` - End position (x, y) \[mm\].
  /// * `width` - Track width \[mm\].
  /// * `net` - Net name (optional).
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(
    layer: Layer,
    start: (f32, f32),
    end: (f32, f32),
    width: f32,
    net: Option<&str>,
  ) -> Track {
    Track {
      layer,
      start,
      end,
      width,
      net: net.map(|s| s.to_owned()),
    }
  }
}

impl ToJson for Track {
  fn to_json(&self) -> JsonValue {
    let mut obj = object! {
      start: self.start.to_json(),
      end: self.end.to_json(),
      width: self.width,
    };
    if let Some(net) = &self.net {
      obj["net"] = net.clone().into();
    }
    obj
  }
}

/// Via structure
#[non_exhaustive]
pub struct Via {
  layers: Vec<Layer>,
  pos: (f32, f32),
  diameter: f32,
  drill_diameter: f32,
  net: Option<String>,
}

impl Via {
  /// Construct via
  ///
  /// # Arguments
  ///
  /// * `layers` - Layers.
  /// * `pos` - Position (x, y) \[mm\].
  /// * `diameter` - Outer diameter \[mm\].
  /// * `drill_diameter` - Drill diameter \[mm\].
  /// * `net` - Net name (optional).
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(
    layers: &[Layer],
    pos: (f32, f32),
    diameter: f32,
    drill_diameter: f32,
    net: Option<&str>,
  ) -> Via {
    Via {
      layers: layers.to_vec(),
      pos,
      diameter,
      drill_diameter,
      net: net.map(|s| s.to_owned()),
    }
  }
}

impl ToJson for Via {
  fn to_json(&self) -> JsonValue {
    let mut obj = object! {
      start: self.pos.to_json(),
      end: self.pos.to_json(),
      width: self.diameter,
      drillsize: self.drill_diameter,
    };
    if let Some(net) = &self.net {
      obj["net"] = net.clone().into();
    }
    obj
  }
}

/// Zone structure
#[non_exhaustive]
pub struct Zone {
  layer: Layer,
  svgpath: String,
  net: Option<String>,
}

impl Zone {
  /// Construct object
  ///
  /// # Arguments
  ///
  /// * `layer` - Layer.
  /// * `svgpath` - Zone outline as SVG path \[mm\].
  /// * `net` - Net name (optional).
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(layer: Layer, svgpath: &str, net: Option<&str>) -> Zone {
    Zone {
      layer,
      svgpath: svgpath.to_owned(),
      net: net.map(|s| s.to_owned()),
    }
  }
}

impl ToJson for Zone {
  fn to_json(&self) -> JsonValue {
    let mut obj = object! {
      svgpath: self.svgpath.clone(),
    };
    if let Some(net) = &self.net {
      obj["net"] = net.clone().into();
    }
    obj
  }
}

/// Footprint pad structure
#[derive(Clone)]
#[non_exhaustive]
pub struct Pad {
  layers: Vec<Layer>,
  pos: (f32, f32),
  angle: f32,
  svgpath: String,
  drill_size: Option<(f32, f32)>,
  net: Option<String>,
  pin1: bool,
}

impl Pad {
  /// Construct object
  ///
  /// # Arguments
  ///
  /// * `layers` - Layers on which the pad exists.
  /// * `pos` - Position (x, y) \[mm\].
  /// * `angle` - Rotation angle [°].
  /// * `svgpath` - Pad shape as SVG path \[mm\].
  /// * `drill_size` - Drill size (w, h) \[mm\] (only for THT pads).
  /// * `net` - Net name (optional).
  /// * `pin1` - Whether this is considered as the pin-1 or not.
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(
    layers: &[Layer],
    pos: (f32, f32),
    angle: f32,
    svgpath: &str,
    drill_size: Option<(f32, f32)>,
    net: Option<&str>,
    pin1: bool,
  ) -> Pad {
    Pad {
      layers: layers.into(),
      pos,
      angle,
      svgpath: svgpath.to_owned(),
      drill_size,
      net: net.map(|s| s.to_owned()),
      pin1,
    }
  }
}

impl ToJson for Pad {
  fn to_json(&self) -> JsonValue {
    let mut obj = object! {
      layers: self.layers.to_json(),
      pos: self.pos.to_json(),
      angle: self.angle,
      shape: "custom",
      svgpath: self.svgpath.clone(),
    };
    if let Some(drill) = &self.drill_size {
      obj["type"] = "th".into();
      obj["drillsize"] = array![drill.0, drill.1];
      obj["drillshape"] = if drill.0 != drill.1 {
        "oblong".into()
      } else {
        "circle".into()
      };
    } else {
      obj["type"] = "smd".into();
    }
    if let Some(net) = &self.net {
      obj["net"] = net.clone().into();
    }
    if self.pin1 {
      obj["pin1"] = 1.into();
    }
    obj
  }
}

/// Footprint structure
#[non_exhaustive]
pub struct Footprint {
  layer: Layer,
  pos: (f32, f32),
  angle: f32,
  bottom_left: (f32, f32),
  top_right: (f32, f32),
  fields: Vec<String>,
  pads: Vec<Pad>,
  mount: bool,
}

impl Footprint {
  /// Construct object
  ///
  /// # Arguments
  ///
  /// * `layer` - Placement layer.
  /// * `pos` - Position (x, y) \[mm\].
  /// * `angle` - Rotation angle [°].
  /// * `bottom_left` - Bottom left corner of bounding box (x, y) \[mm\].
  /// * `top_right` - Top right corner of bounding box (x, y) \[mm\].
  /// * `fields` - Custom fields, corresponding to [InteractiveHtmlBom::fields].
  /// * `pads` - Footprint pads.
  /// * `mount` - Whether the footprint is mounted or not.
  ///
  /// # Returns
  ///
  /// Returns the new object.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    layer: Layer,
    pos: (f32, f32),
    angle: f32,
    bottom_left: (f32, f32),
    top_right: (f32, f32),
    fields: &[String],
    pads: &[Pad],
    mount: bool,
  ) -> Footprint {
    Footprint {
      layer,
      pos,
      angle,
      bottom_left,
      top_right,
      fields: fields.to_vec(),
      pads: pads.to_vec(),
      mount,
    }
  }
}

impl ToJson for Footprint {
  fn to_json(&self) -> JsonValue {
    object! {
      bbox: object!{
        pos: self.pos.to_json(),
        angle: self.angle,
        relpos: self.bottom_left.to_json(),
        size: array![
          self.top_right.0 - self.bottom_left.0,
          self.top_right.1 - self.bottom_left.1],
      },
      drawings: array![],  // Not supported yet.
      layer: self.layer.to_json(),
      pads: self.pads.to_json(),
    }
  }
}

/// Reference-FootprintID map
#[derive(Clone)]
#[non_exhaustive]
pub struct RefMap {
  reference: String,
  footprint_id: usize,
}

impl RefMap {
  /// Construct object
  ///
  /// # Arguments
  ///
  /// * `reference` - Component reference (e.g. "R1").
  /// * `footprint_id` - ID of footprint as returned by
  ///                    [InteractiveHtmlBom::add_footprint].
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(reference: &str, footprint_id: usize) -> RefMap {
    RefMap {
      reference: reference.to_owned(),
      footprint_id,
    }
  }
}

impl ToJson for RefMap {
  fn to_json(&self) -> JsonValue {
    array! {
      self.reference.clone(),
      self.footprint_id,
    }
  }
}

/// Interactive HTML BOM structure
///
/// The top-level structure to build & generate a HTML BOM.
///
/// <div class="warning">
/// Please note that this struct is not completely fool-proof as it does not
/// validate lots of the added data. So make sure you add only valid BOM data.
/// Only the most important things are validated to avoid generating completely
/// broken HTML pages: Footprint IDs in BOM rows, and number of fields in
/// footprints.
/// </div>
///
/// # Examples
///
/// ```
/// use interactive_html_bom::*;
///
/// let mut ibom = InteractiveHtmlBom::new(
///   "My Project",   // Title
///   "My Company",   // Company
///   "Rev. 1",       // Revision
///   "1970-01-01",   // Date
///   (0.0, 0.0),     // Bottom left
///   (100.0, 80.0),  // Top right
/// );
///
/// // Set configuration.
/// ibom.fields = vec!["Value".into(), "Footprint".into()];
///
/// // Draw PCB.
/// ibom.drawings.push(Drawing::new(
///   DrawingKind::Polygon,             // Kind of drawing
///   DrawingLayer::Edge,               // Layer
///   "M 0 0 H 100 V 80 H -100 V -80",  // SVG path
///   0.1,                              // Line width
///   false,                            // Filled
/// ));
/// ibom.drawings.push(Drawing::new(
///   DrawingKind::ReferenceText,
///   DrawingLayer::SilkscreenFront,
///   "M 10 10 H 80 V 60 H -80 V -60",
///   0.1,
///   false,
/// ));
///
/// // Add footprints.
/// let id = ibom.add_footprint(
///   Footprint::new(
///     Layer::Front,                       // Layer
///     (50.0, 40.0),                       // Position
///     45.0,                               // Rotation
///     (-2.0, -1.0),                       // Bottom left
///     (2.0, 1.0),                         // Top right
///     &["100R".into(), "0603".into()],    // Fields
///     &[Pad::new(
///         &[Layer::Front],                // Pad layers
///         (-2.0, 0.0),                    // Pad position
///         0.0,                            // Pad rotation
///         "M -1 -1 H 2 V 2 H -2 V -2",    // Pad shape (SVG)
///         None,                           // Pad drill
///         None,                           // Pad net
///         true,                           // Pin 1
///       ),
///       // [...]
///     ],
///     true,                               // Mount or not
///   ),
/// );
///
/// // Add BOM rows (designators and their footprint IDs).
/// ibom.bom_front.push(vec![RefMap::new("R1", id)]);
/// ```
#[non_exhaustive]
pub struct InteractiveHtmlBom {
  // Metadata
  title: String,
  company: String,
  revision: String,
  date: String,
  bottom_left: (f32, f32),
  top_right: (f32, f32),

  /// Dark mode on/off
  pub dark_mode: bool,

  /// Silkscreen visibility
  pub show_silkscreen: bool,

  /// Fabrication layer visibility
  pub show_fabrication: bool,

  /// Pads visibility
  pub show_pads: bool,

  /// Checkbox column names
  pub checkboxes: Vec<String>,

  /// Custom field names, listed as columns
  pub fields: Vec<String>,

  /// User-defined HTML header
  ///
  /// <div class="warning">
  /// This should be used carefully as we neither guarantee forward- nor
  /// backward-compatibility.
  /// </div>
  pub user_header: String,

  /// User-defined HTML footer
  ///
  /// <div class="warning">
  /// This should be used carefully as we neither guarantee forward- nor
  /// backward-compatibility.
  /// </div>
  pub user_footer: String,

  /// User-defined JavaScript
  ///
  /// <div class="warning">
  /// This should be used carefully as we neither guarantee forward- nor
  /// backward-compatibility.
  /// </div>
  pub user_js: String,

  /// Drawings (PCB edges, silkscreen, fabrication)
  pub drawings: Vec<Drawing>,

  /// PCB tracks
  pub tracks: Vec<Track>,

  /// PCB vias
  pub vias: Vec<Via>,

  /// PCB zones
  pub zones: Vec<Zone>,

  /// Footprints
  pub footprints: Vec<Footprint>,

  /// BOM rows front side
  pub bom_front: Vec<Vec<RefMap>>,

  /// BOM rows back side
  pub bom_back: Vec<Vec<RefMap>>,

  /// BOM rows front+back
  pub bom_both: Vec<Vec<RefMap>>,
}

impl InteractiveHtmlBom {
  /// Construct object
  ///
  /// # Arguments
  ///
  /// * `title` - Project title.
  /// * `company` - Company/author name.
  /// * `revision` - Project revision.
  /// * `date` - Date/time as desired.
  /// * `bottom_left` - Bottom left corner of bounding box (x, y) \[mm\].
  /// * `top_right` - Top right corner of bounding box (x, y) \[mm\].
  ///
  /// # Returns
  ///
  /// Returns the new object.
  pub fn new(
    title: &str,
    company: &str,
    revision: &str,
    date: &str,
    bottom_left: (f32, f32),
    top_right: (f32, f32),
  ) -> InteractiveHtmlBom {
    InteractiveHtmlBom {
      title: title.to_owned(),
      revision: revision.to_owned(),
      company: company.to_owned(),
      date: date.to_owned(),
      bottom_left,
      top_right,
      dark_mode: false,
      show_silkscreen: true,
      show_fabrication: true,
      show_pads: true,
      checkboxes: vec!["Sourced".into(), "Placed".into()],
      fields: Vec::new(),
      user_js: String::new(),
      user_header: String::new(),
      user_footer: String::new(),
      drawings: Vec::new(),
      tracks: Vec::new(),
      vias: Vec::new(),
      zones: Vec::new(),
      footprints: Vec::new(),
      bom_front: Vec::new(),
      bom_back: Vec::new(),
      bom_both: Vec::new(),
    }
  }

  /// Add footprint
  ///
  /// # Arguments
  ///
  /// * `fpt` - The footprint to add.
  ///
  /// # Returns
  ///
  /// Returns the ID of the added footprint, to be used for referencing it
  /// in BOM rows.
  pub fn add_footprint(&mut self, fpt: Footprint) -> usize {
    self.footprints.push(fpt);
    self.footprints.len() - 1
  }

  /// Generate HTML
  pub fn generate_html(&self) -> Result<String, String> {
    // Validate footprint IDs.
    for bom in [&self.bom_back, &self.bom_front, &self.bom_both] {
      for row in bom {
        for map in row {
          if map.footprint_id >= self.footprints.len() {
            return Err("Invalid footprint ID.".into());
          }
        }
      }
    }

    // Calculate some additional data.
    let mut nets = Vec::new();
    let mut dnp_footprint_ids: Vec<usize> = Vec::new();
    for (index, footprint) in self.footprints.iter().enumerate() {
      if !footprint.mount {
        dnp_footprint_ids.push(index);
      }
      for pad in &footprint.pads {
        if let Some(net) = &pad.net {
          if !nets.contains(net) {
            nets.push(net.clone());
          }
        }
      }
    }

    // Auto-detect visibility of front/back sides depending on BOM.
    let layer_view = if !self.bom_front.is_empty() && self.bom_back.is_empty() {
      "F"
    } else if self.bom_front.is_empty() && !self.bom_back.is_empty() {
      "B"
    } else {
      "FB"
    };

    let config = object! {
        board_rotation: 0.0,
        bom_view: "left-right",
        checkboxes: self.checkboxes.join(","),
        dark_mode: self.dark_mode,
        fields: self.fields.to_json(),
        highlight_pin1: "none",
        kicad_text_formatting: false,
        layer_view: layer_view,
        offset_back_rotation: false,
        redraw_on_drag: true,
        show_fabrication: self.show_fabrication,
        show_pads: self.show_pads,
        show_silkscreen: self.show_silkscreen,
    };

    let mut data = object! {
      ibom_version: String::from_utf8_lossy(include_bytes!("web/version.txt")).to_string(),
      metadata: object!{
        title: self.title.clone(),
        company: self.company.clone(),
        revision: self.revision.clone(),
        date: self.date.clone(),
      },
      edges_bbox: object!{
        minx: self.bottom_left.0,
        maxx: self.top_right.0,
        miny: self.bottom_left.1,
        maxy: self.top_right.1,
      },
      edges: self.drawings.iter()
        .filter(|x| x.layer == DrawingLayer::Edge)
        .map(ToJson::to_json).collect::<Vec<_>>(),
      drawings: object!{
        silkscreen: object!{
          F: self.drawings.iter()
              .filter(|x| x.layer == DrawingLayer::SilkscreenFront)
              .map(ToJson::to_json).collect::<Vec<_>>(),
          B: self.drawings.iter()
              .filter(|x| x.layer == DrawingLayer::SilkscreenBack)
              .map(ToJson::to_json).collect::<Vec<_>>(),
        },
        fabrication: object!{
          F: self.drawings.iter()
              .filter(|x| x.layer == DrawingLayer::FabricationFront)
              .map(ToJson::to_json).collect::<Vec<_>>(),
          B: self.drawings.iter()
              .filter(|x| x.layer == DrawingLayer::FabricationBack)
              .map(ToJson::to_json).collect::<Vec<_>>(),
        },
      },
      tracks: object!{
        F: self.tracks.iter()
            .filter(|x| x.layer == Layer::Front)
            .map(ToJson::to_json)
            .chain(self.vias.iter()
              .filter(|x| x.layers.contains(&Layer::Front))
              .map(ToJson::to_json))
            .collect::<Vec<_>>(),
        B: self.tracks.iter()
            .filter(|x| x.layer == Layer::Back)
            .map(ToJson::to_json)
            .chain(self.vias.iter()
              .filter(|x| x.layers.contains(&Layer::Back))
              .map(ToJson::to_json))
            .collect::<Vec<_>>(),
      },
      zones: object!{
        F: self.zones.iter()
            .filter(|x| x.layer == Layer::Front)
            .map(ToJson::to_json).collect::<Vec<_>>(),
        B: self.zones.iter()
            .filter(|x| x.layer == Layer::Back)
            .map(ToJson::to_json).collect::<Vec<_>>(),
      },
      nets: nets.to_json(),
      footprints: self.footprints.to_json(),
      bom: object!{
        F: self.bom_front.to_json(),
        B: self.bom_back.to_json(),
        both: self.bom_both.to_json(),
        skipped: dnp_footprint_ids.to_json(),
        fields: object!{},  // Filled below.
      },
    };

    // Fill in footprint fields and check their length.
    for (id, fpt) in self.footprints.iter().enumerate() {
      if fpt.fields.len() != self.fields.len() {
        return Err("Inconsistent number of fields.".into());
      }
      data["bom"]["fields"][id.to_string()] = fpt.fields.to_json();
    }

    // Build JS variables.
    let config_str = "var config = ".to_owned() + &config.dump();
    let pcbdata_str =
      "var pcbdata = JSON.parse(LZString.decompressFromBase64(\"".to_owned()
        + &lz_str::compress_to_base64(&data.dump())
        + "\"))";

    // Load HTML.
    let mut html =
      String::from_utf8_lossy(include_bytes!("web/ibom.html")).to_string();

    // Replace placeholders.
    let replacements = [
      (
        "///CSS///",
        String::from_utf8_lossy(include_bytes!("web/ibom.css")),
      ),
      (
        "///SPLITJS///",
        String::from_utf8_lossy(include_bytes!("web/split.js")),
      ),
      (
        "///LZ-STRING///",
        String::from_utf8_lossy(include_bytes!("web/lz-string.js")),
      ),
      (
        "///POINTER_EVENTS_POLYFILL///",
        String::from_utf8_lossy(include_bytes!("web/pep.js")),
      ),
      (
        "///UTILJS///",
        String::from_utf8_lossy(include_bytes!("web/util.js")),
      ),
      (
        "///RENDERJS///",
        String::from_utf8_lossy(include_bytes!("web/render.js")),
      ),
      (
        "///TABLEUTILJS///",
        String::from_utf8_lossy(include_bytes!("web/table-util.js")),
      ),
      (
        "///IBOMJS///",
        String::from_utf8_lossy(include_bytes!("web/ibom.js")),
      ),
      ("///CONFIG///", config_str.as_str().into()),
      ("///PCBDATA///", pcbdata_str.as_str().into()),
      ("///USERJS///", self.user_js.as_str().into()),
      ("///USERHEADER///", self.user_header.as_str().into()),
      ("///USERFOOTER///", self.user_footer.as_str().into()),
    ];
    for replacement in &replacements {
      html = html.replace(replacement.0, &replacement.1);
    }
    Ok(html)
  }
}
