import { Button, Form } from "solid-bootstrap";
import {
	GetChatMessagesResponse,
	Message,
	SendMessageResponse,
} from "../models/chats";
import {
	Accessor,
	createEffect,
	createSignal,
	For,
	onCleanup,
	Setter,
	Show,
} from "solid-js";
import { useUsers } from "../contexts/UserContext";
import ChatMessage from "./Message";

export interface ChatProps {
	messages: Accessor<Message[]>;
	setMessages: Setter<Message[]>;
	hasMore: Accessor<boolean>;
	setHasMore: Setter<boolean>;
	chatId: Accessor<string>;
	refCallback: (el: HTMLDivElement) => void;
}

export default function ChatView({
	messages,
	setMessages,
	hasMore,
	setHasMore,
	chatId,
	refCallback,
}: ChatProps) {
	let container!: HTMLDivElement;
	let sentinel!: HTMLDivElement;
	let observer: IntersectionObserver;

	createEffect(async () => {
		refCallback(container);
		if (!chatId()) return;

		const res = await fetch(
			`/chats/${chatId()}?${new URLSearchParams({ limit: "10" })}`,
		);

		if (!res.ok) {
			console.error(await res.json());
			return { has_more: false, messages: [] };
		}

		const body: GetChatMessagesResponse = await res.json();
		setMessages(body.messages.reverse());
		setHasMore(body.has_more);

		requestAnimationFrame(() => {
			if (container) container.scrollTop = container.scrollHeight;
		});

		if (body.has_more) {
			observer = new IntersectionObserver(async (entries) => {
				if (entries[0].isIntersecting && hasMore()) {
					const last_message = messages().at(0);
					if (!last_message) return;

					const urlParams = {
						limit: "5",
						last_message_id: last_message.id.toString(),
					};
					const res = await fetch(
						`/chats/${chatId()}?${new URLSearchParams(urlParams)}`,
					);
					if (!res.ok) {
						console.error(await res.json());
						return;
					}

					const body: GetChatMessagesResponse = await res.json();
					setMessages([...body.messages.reverse(), ...messages()]);
					setHasMore(body.has_more);
				}
			});
			observer.observe(sentinel);
		}
	});

	onCleanup(() => observer.disconnect());

	const { users } = useUsers();
	const [content, setContent] = createSignal("");

	const sendMessage = async () => {
		const trim_content = content().trim();

		if (!trim_content || !users.currentUser) {
			return;
		}

		const data = {
			content: trim_content,
		};

		const res = await fetch(`/chats/${chatId()}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		});

		if (!res.ok) {
			console.error(res.status);
			console.error(await res.json());
			return;
		}

		const body: SendMessageResponse = await res.json();
		const message: Message = {
			content: content(),
			id: body.message_id,
			created_at: new Date().toISOString(),
			sender_id: users.currentUser.id,
			chat_id: Number.parseInt(chatId()),
		};

		setMessages([...messages(), message]);
		setContent("");

		requestAnimationFrame(() => {
			if (container) container.scrollTop = container.scrollHeight;
		});
	};

	const onSendMessage = async (e: SubmitEvent) => {
		e.preventDefault();
		sendMessage();
	};

	const onKeyDown = (e: KeyboardEvent) => {
		if (e.ctrlKey && e.key === "Enter") {
			e.preventDefault();
			sendMessage();
		}
	};

	return (
		<div class="w-100 d-flex flex-column">
			<div
				class="overflow-auto flex-grow-1"
				ref={container}
			>
				<Show when={hasMore()}>
					<div ref={sentinel} />
				</Show>

				<For each={messages()}>{(msg) => <ChatMessage {...msg} />}</For>
			</div>
			<Show when={chatId()}>
				<Form class="d-flex flex-row" onSubmit={onSendMessage}>
					<Form.Control
						as="textarea"
						class="flex-grow-1"
						style={{ height: "100px" }}
						value={content()}
						onInput={(e) => setContent(e.currentTarget.value)}
						onKeyDown={onKeyDown}
					/>
					<Button type="submit">Send</Button>
				</Form>
			</Show>
		</div>
	);
}
