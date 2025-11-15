export interface Chat {
	id: number;
	title: string;
}

export interface Member {
	id: number | undefined;
	value: string;
}

export interface ChatCreationResponse {
	chat_id: number;
}

export interface Message {
	chat_id: number;
	content: string;
	created_at: string;
	id: number;
	sender_id: number | null;
}

export interface GetChatMessagesResponse {
	has_more: boolean;
	messages: Message[];
}

export interface SendMessageResponse {
	message_id: number;
}
