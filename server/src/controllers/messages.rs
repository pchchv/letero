use crate::{
    repositories::chats::ChatsRepository,
    models::{chats::ChatId, users::UserId},
};

#[tracing::instrument(skip(chats), ret)]
async fn check_chat_access(chats: &dyn ChatsRepository, user_id: UserId, chat_id: ChatId) -> bool {
    let Ok(chats) = chats.get_user_chats_ids(user_id).await else {
        return false;
    };

    chats.contains(&chat_id)
}