mod imp;

use gstreamer::{glib, prelude::StaticType, Rank};

glib::wrapper! {
  pub struct TtsSink(ObjectSubclass<imp::TtsSink>) @extends gstreamer_base::BaseTransform, gstreamer::Element, gstreamer::Object;
}

pub fn register(plugin: &gstreamer::Plugin) -> Result<(), glib::BoolError> {
  gstreamer::Element::register(
    Some(plugin),
    "ttssink",
    Rank::None,
    TtsSink::static_type(),
  )
}
