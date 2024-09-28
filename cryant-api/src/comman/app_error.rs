use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub struct CyrantError(anyhow::Error);

// Tell axum how to convert `CyrantError` into a response.
impl IntoResponse for CyrantError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, CyrantError>`. That way you don't need to do that manually.
impl<E> From<E> for CyrantError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/*
sample usage

async fn handler_error () -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

 */