import { Button, Form } from "solid-bootstrap";
import { User } from "../models/users";
import { createSignal, For, onCleanup, Show } from "solid-js";

interface MemberInputProps {
	index: number;
	value: string;
	removeMember: (index: number) => void;
	setId: (index: number, id: number) => void;
}

export default function MemberInput(props: MemberInputProps) {
	const [users, setUsers] = createSignal<User[]>([]);
	const [success, setSuccess] = createSignal(false);

	let timer: number;

	const onInputMember = async (e: InputEvent) => {
		const input = e.currentTarget as HTMLInputElement;
		setSuccess(false);

		clearTimeout(timer);
		timer = window.setTimeout(() => runQuery(input.value), 500);
	};

	const runQuery = async (value: string) => {
		if (value.length < 3 || value.length > 30) return;

		const res = await fetch(
			"/search/users?" + new URLSearchParams({ username: value }),
		);
		if (!res.ok) return;

		const users: User[] = await res.json();
		setUsers(users);

		const u = users.find((u) => u.username === value);
		if (u !== undefined) {
			props.setId(props.index, u.id);
			setSuccess(true);
		}
	};

	onCleanup(() => clearTimeout(timer));

	return (
		<Form.Group class="d-flex flex-row mb-1 mt-1">
			<Form.Control
				type="text"
				value={props.value}
				onInput={onInputMember}
				list={"user_search_" + props.index}
				isValid={success()}
			/>
			<Show when={users()}>
				<datalist id={"user_search_" + props.index}>
					<For each={users()}>{(user) => <option value={user.username} />}</For>
				</datalist>
			</Show>
			<Button
				variant="danger"
				class="ms-1"
				onClick={() => props.removeMember(props.index)}
			>
				X
			</Button>
		</Form.Group>
	);
}
