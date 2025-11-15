import { Message } from "./chats";

export interface NewMessageEvent {
	chat_id: number;
	message: Message;
	user_id: number;
}

export interface NewChatEvent {
	title: string;
	users_ids: number[];
	chat_id: number;
}
