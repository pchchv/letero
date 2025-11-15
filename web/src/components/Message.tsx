import { createEffect, createSignal } from "solid-js";
import { useUsers } from "../contexts/UserContext";
import { Message } from "../models/chats";
import { User } from "../models/users";
import { Placeholder } from "solid-bootstrap";

export default function ChatMessage(msg: Message) {
	const { users, setUser } = useUsers();
	const [username, setUsername] = createSignal("");

	createEffect(async () => {
		if (msg.sender_id === null) {
			setUsername("deleted");
			return;
		}

		const sender =
			msg.sender_id === users.currentUser?.id
				? users.currentUser
				: users.users[msg.sender_id];
		if (sender) {
			setUsername(sender.username);
			return;
		}

		const res = await fetch(`/users/${msg.sender_id}`);

		if (!res.ok) {
			console.error(res.status);
			console.error(await res.json());
			return;
		}

		const user: User = await res.json();
		setUser("users", msg.sender_id, user);
		setUsername(user.username);
	});

	return (
		<div class="m-3 p-3 border d-flex flex-column">
			<div class="d-flex flex-row justify-content-between">
				{username() ? (
					<span class="me-5 first-color">{username()}</span>
				) : (
					<Placeholder />
				)}
				<span>{new Date(msg.created_at).toLocaleString()}</span>
			</div>
			<span>{msg.content}</span>
		</div>
	);
}
