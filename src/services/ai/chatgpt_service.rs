use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;

use crate::conversation::Question;

pub struct ChatGPTService{}

impl ChatGPTService
{
    pub async fn answer(question: &Question) -> Result<String>{
        let client = ChatGPT::new("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").unwrap();
    
        // Sending a message and getting the completion
        let response: CompletionResponse = client
            .send_message(&question.question)
            .await?;
    
        Ok(response.message().content.clone())
    }

}