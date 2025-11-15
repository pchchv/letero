import { A, useLocation } from "@solidjs/router";
import { Button } from "solid-bootstrap";
import ChatsList from "../components/ChatsList";
import { createMemo, createSignal, onCleanup, onMount } from "solid-js";
import { Chat, Message } from "../models/chats";
import { NewChatEvent, NewMessageEvent } from "../models/events";
import ChatView from "../components/Chat";
import { createStore } from "solid-js/store";

export default function ChatPage() {
	const params = useLocation();
	const chatId = createMemo(() => params.hash.slice(1));
	const [hasMore, setHasMore] = createSignal(false);
	const [messages, setMessages] = createSignal<Message[]>([]);
	const [chats, setChats] = createStore<Chat[]>([]);
	let chatContainer!: HTMLDivElement;

	onMount(() => {
		const events = new EventSource("/events");

		events.addEventListener("Message", (event) => {
			const eventData: NewMessageEvent = JSON.parse(event.data);
			if (chatId() === eventData.chat_id.toString()) {
				setMessages([...messages(), eventData.message]);
				requestAnimationFrame(() => {
					if (chatContainer) chatContainer.scrollTop = chatContainer.scrollHeight;
				});
			}
		});

		events.addEventListener("Chat", (event) => {
			const eventData: NewChatEvent = JSON.parse(event.data);
			const chat: Chat = {
				id: eventData.chat_id,
				title: eventData.title,
			};
			setChats(chats.length, chat);
		});

		onCleanup(() => {
			events.close();
		});
	});

	return (
		<div class="w-100 h-100 d-flex flex-row">
			<aside class="d-flex h-100 align-items-end fit p-3 border-end">
				<A href="/logout" target="_self">
					<Button variant="danger">Exit</Button>
				</A>
			</aside>
			<main class="d-flex flex-row w-100">
				<ChatsList {...{ chats, setChats }} />
				<ChatView {...{ hasMore, setHasMore, messages, setMessages, chatId }} refCallback={el => chatContainer = el} />
			</main>
		</div>
	);
}
