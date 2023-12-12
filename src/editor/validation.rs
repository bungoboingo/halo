use crate::editor::{icon, Message};
use crate::widget::Element;
use crate::{theme, FragmentShader};
use iced::widget::tooltip;
use naga::valid::Capabilities;
use std::fmt::Formatter;
use std::ops::Range;
use std::sync::Arc;

#[derive(Default, Debug)]
pub enum Status {
    #[default]
    Validated,
    Validating,
    Invalid(Error),
    NeedsValidation,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Validated => "Shader is valid",
            Status::Validating => "Shader is being validated",
            Status::Invalid(_) => "Shader is invalid!",
            Status::NeedsValidation => "Shader needs validation!",
        };

        write!(f, "{str}")
    }
}

impl Status {
    pub fn icon(&self) -> Element<Message> {
        //TODO colors
        let icon = match self {
            Status::Validated => icon('\u{e801}'),
            Status::Invalid(_) => icon('\u{e802}'),
            Status::Validating => icon('\u{e803}'),
            Status::NeedsValidation => icon('\u{e803}'),
        };

        tooltip(icon, self.to_string(), tooltip::Position::Bottom)
            .padding(10)
            .style(theme::Container::Tooltip)
            .into()
    }
}

//assumes shader is wgsl
pub async fn validate(shader: Arc<FragmentShader>) -> Result<Arc<FragmentShader>, Error> {
    //parse separately so we can show errors instead of panicking on pipeline creation
    let concat_shader = format!(
        "{}\n{}",
        include_str!("../viewer/shaders/uniforms.wgsl"),
        shader
    );

    let parsed =
        naga::front::wgsl::parse_str(&concat_shader).map_err(|parse_error| Error::Parse {
            message: parse_error.message().to_string(),
            errors: parse_error
                .labels()
                .filter_map(|(span, err)| span.to_range().map(|r| (r, err.to_string())))
                .collect::<Vec<_>>(),
        })?;

    naga::valid::Validator::new(
        naga::valid::ValidationFlags::default(),
        Capabilities::all(), //TODO get from device capabilities
    )
    .validate(&parsed)
    .map_err(|err| Error::Validation(err.to_string()))?;

    Ok(shader)
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Shader parsing error")]
    Parse {
        message: String,
        errors: Vec<(Range<usize>, String)>,
    },
    #[error("Validation error: {0}")]
    Validation(String),
}
