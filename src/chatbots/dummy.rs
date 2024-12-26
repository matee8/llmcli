use async_trait::async_trait;
use futures::{stream, StreamExt as _};

use crate::{
    Chatbot, ChatbotCreationError, ChatbotError, ResponseStream, Role,
};

#[non_exhaustive]
#[derive(Default)]
pub struct DummyChatbot;

impl DummyChatbot {}

#[async_trait]
impl Chatbot for DummyChatbot {
    #[inline]
    fn create(
        _model: String,
        _api_key: Option<String>,
    ) -> Result<Box<dyn Chatbot>, ChatbotCreationError> {
        Ok(Box::new(Self))
    }

    #[inline]
    fn name(&self) -> &'static str {
        "Dummy"
    }

    #[inline]
    fn model(&self) -> &'static str {
        "1"
    }

    #[inline]
    fn change_model(
        &mut self,
        _new_model: String,
    ) -> Result<(), crate::InvalidModelError> {
        Ok(())
    }

    #[inline]
    async fn send_message(
        &self,
        messages: &[crate::Message],
    ) -> Result<ResponseStream, ChatbotError> {
        let msg = messages.last().map_or_else(
            || "Dummy response to empty conversation.".to_owned(),
            |last_msg| {
                if last_msg.role == Role::User {
                    format!("Dummy response to: \"{}\".", last_msg.content)
                } else {
                    "Dummy response.".to_owned()
                }
            },
        );

        let stream = stream::iter(vec![Ok(msg)]).boxed();

        Ok(stream)
    }
}
