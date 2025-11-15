import "./ChatList.tsx.css";
import { createSignal, For, onMount } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { Chat, ChatCreationResponse, Member } from "../models/chats";
import { Button, Form, ListGroup, Modal } from "solid-bootstrap";
import MemberInput from "./MemberInput";
import { useLocation } from "@solidjs/router";

export interface ChatsListProps {
	chats: Chat[];
	setChats: SetStoreFunction<Chat[]>;
}

export default function ChatsList({ chats, setChats }: ChatsListProps) {
	const params = useLocation();

	const [members, setMembers] = createStore<Member[]>([]);
	const [title, setTitle] = createSignal("");
	const [show, setShow] = createSignal(false);

	const handleClose = () => {
		setShow(false);
		setTitle("");
		setMembers([]);
	};

	const handleCreate = async (e: SubmitEvent) => {
		e.preventDefault();

		// remove repetitions or unspecified users without id
		const usersMap = new Map(
			members.filter((m) => m.id !== undefined).map((m) => [m.value, m.id]),
		);

		const users_ids: number[] = Array.from(
			usersMap
				.keys()
				.map((u) => usersMap.get(u))
				.filter((u) => u !== undefined),
		);

		const data = {
			title: title(),
			users_ids,
		};

		const res = await fetch("/chats", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		});

		if (!res.ok) {
			console.error(await res.json());
			return;
		}

		const chat_id: ChatCreationResponse = await res.json();

		setChats(chats.length, { id: chat_id.chat_id, title: title() });
		handleClose();
	};

	onMount(async () => {
		const res = await fetch("/chats");
		if (res.ok) {
			const chats: Chat[] = await res.json();
			setChats(chats);
		} else {
			console.error(res.status);
			console.error(await res.json());
		}
	});

	return (
		<div class="border-end chats-list-min-width h-100">
			<div class="border-bottom p-3 cursor" onClick={() => setShow(true)}>
				<span>New chat</span>
			</div>

			<ListGroup defaultActiveKey={params.hash}>
				<For each={chats.reverse()}>
					{(chat) => (
						<ListGroup.Item action href={"#" + chat.id}>
							{chat.title}
						</ListGroup.Item>
					)}
				</For>
			</ListGroup>

			<Modal
				show={show()}
				onHide={handleClose}
				aria-labelledby="contained-modal-title-vcenter"
				centered
			>
				<Modal.Header closeButton>
					<Modal.Title id="contained-modal-title-vcenter">New chat</Modal.Title>
				</Modal.Header>
				<Form onSubmit={handleCreate}>
					<Modal.Body>
						<Form.Control
							type="text"
							onInput={(e) => setTitle(e.currentTarget.value)}
						/>
						<For each={members}>
							{(m, i) => (
								<MemberInput
									index={i()}
									value={m.value}
									setId={(idx, id) => setMembers(idx, "id", id)}
									removeMember={(idx) =>
										setMembers([...members.filter((_, index) => index !== idx)])
									}
								/>
							)}
						</For>
						<Button
							variant="link"
							onClick={() => setMembers(members.length, { value: "" })}
						>
							Add member
						</Button>
					</Modal.Body>
					<Modal.Footer>
						<Button onClick={handleClose} type="reset">
							Close
						</Button>
						<Button type="submit">Create</Button>
					</Modal.Footer>
				</Form>
			</Modal>
		</div>
	);
}
