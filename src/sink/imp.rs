use std::{str, sync::Mutex};

use gstreamer::{
  glib::{self, ParamSpec, Value},
  subclass::{
    prelude::{ElementImpl, GstObjectImpl, ObjectImpl, ObjectSubclass},
    ElementMetadata,
  },
  Buffer, Caps, DebugCategory, ErrorMessage, FlowError, FlowSuccess,
  PadTemplate,
};
use gstreamer_base::{
  BaseSink,
  subclass::{
    prelude::BaseSinkImpl,
  },
};
use once_cell::sync::Lazy;
use tts::Tts;

static CAT: Lazy<DebugCategory> = Lazy::new(|| {
  DebugCategory::new(
    "ttssink",
    gstreamer::DebugColorFlags::empty(),
    Some("Text to speech sink using platform APIs"),
  )
});

static CAPS: Lazy<Caps> = Lazy::new(|| Caps::builder("text/x-raw").field("format", "utf8").build());

#[derive(Debug, Clone)]
struct Settings {}

struct State {
  tts: Tts,
}

pub struct TtsSink {
  #[allow(dead_code)]
  settings: Mutex<Settings>,
  state: Mutex<Option<State>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TtsSink {
  type ParentType = BaseSink;
  type Type = super::TtsSink;

  const NAME: &'static str = "GstTtsSink";

  fn new() -> Self {
    Self {
      settings: Mutex::new(Settings {}),
      state: Mutex::new(None),
    }
  }
}

impl ObjectImpl for TtsSink {
  fn properties() -> &'static [ParamSpec] {
    static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
      vec![]
    });
    PROPERTIES.as_ref()
  }

  fn set_property(&self, _id: usize, _value: &Value, pspec: &ParamSpec) {
    match pspec.name() {
      _ => unimplemented!(),
    }
  }

  fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
    match pspec.name() {
      name => panic!("No getter for {name}"),
    }
  }
}

impl GstObjectImpl for TtsSink {}

impl ElementImpl for TtsSink {
  fn metadata() -> Option<&'static ElementMetadata> {
    static ELEMENT_METADATA: Lazy<ElementMetadata> = Lazy::new(|| {
      ElementMetadata::new(
        "TTS element",
        "Converter/Text/Audio",
        "Sink a text buffer, synthesise speech using platform APIs, and source audio buffers",
        "Jasper Hugo <jasper@avstack.io>",
      )
    });

    Some(&*ELEMENT_METADATA)
  }

  fn pad_templates() -> &'static [PadTemplate] {
    static PAD_TEMPLATES: Lazy<Vec<PadTemplate>> = Lazy::new(|| {
      let sink_pad_template = gstreamer::PadTemplate::new(
        "sink",
        gstreamer::PadDirection::Sink,
        gstreamer::PadPresence::Always,
        &CAPS,
      )
      .unwrap();

      vec![sink_pad_template]
    });

    PAD_TEMPLATES.as_ref()
  }
}

impl BaseSinkImpl for TtsSink {
  fn start(&self) -> Result<(), ErrorMessage> {
    gstreamer::debug!(CAT, "start()");
    *self.state.lock().unwrap() = Some(State {
      tts: Tts::default().unwrap(),
    });
    Ok(())
  }

  fn stop(&self) -> Result<(), ErrorMessage> {
    gstreamer::debug!(CAT, "stop()");
    Ok(())
  }

  fn render(&self, buffer: &Buffer) -> Result<FlowSuccess, FlowError> {
    let buffer_reader = buffer.as_ref().map_readable().map_err(|_| FlowError::Error)?;
    let text = str::from_utf8(buffer_reader.as_slice()).map_err(|_| FlowError::Error)?;
    self.state.lock().unwrap().as_mut().unwrap().tts.speak(text, false).map_err(|_| FlowError::Error)?;
    Ok(FlowSuccess::Ok)
  }
}
