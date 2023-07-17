pub mod types;
pub use types::User;

impl User {
    pub fn is_bot(&self) -> bool {
        if let Some(bot) = self.bot {
            return bot;
        }

        false
    }

    pub fn is_webhook(&self) -> bool {
        if let Some(_webhook_id) = &self.webhook_id {
            return true;
        }

        false
    }
}