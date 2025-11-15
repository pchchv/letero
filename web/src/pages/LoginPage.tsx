import { A, useNavigate } from "@solidjs/router";
import { Button, Form, Stack } from "solid-bootstrap";
import { createSignal, Show } from "solid-js";
import { useUsers } from "../contexts/UserContext";
import { ILoginResponse } from "../models/auth";

const LoginPage = () => {
	const navigate = useNavigate();
	const { users, setUser } = useUsers();

	const [showError, setShowError] = createSignal(false);
	const [username, setUsername] = createSignal("");
	const [password, setPassword] = createSignal("");

	const onSubmit = async (e: SubmitEvent) => {
		e.preventDefault();
		setShowError(false);

		const data = {
			username: username(),
			password: password(),
		};

		const res = await fetch("/login", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		});

		if (res.ok) {
			const body: ILoginResponse = await res.json();
			setUser("currentUser", {
				id: body.user_id,
				username: username(),
			});
			navigate("/", { replace: true });
		} else if (res.status === 404) {
			setShowError(true);
		} else {
			console.error(res.status);
			console.error(await res.json());
		}
	};

	return (
		<div class="d-flex w-100 vh-100 justify-content-center align-items-center">
			<Stack class="border rounded p-3">
				<h1>Login</h1>
				<Form class="min-width mt-3" onSubmit={onSubmit}>
					<Form.Group class="mb-1">
						<Form.Control
							type="text"
							placeholder="Username"
							onInput={(e) => setUsername(e.currentTarget.value)}
						/>
						<Show when={showError()}>
							<Form.Text class="text-danger">
								Incorrect username or password
							</Form.Text>
						</Show>
					</Form.Group>
					<Form.Control
						type="password"
						placeholder="Password"
						onInput={(e) => setPassword(e.currentTarget.value)}
					/>

					<div class="mt-3 d-flex w-100">
						<div class="w-100">
							<A href="/">
								<Button variant="outline-secondary">back</Button>
							</A>
							<A href="/register" class="me-1">
								<Button variant="link">sign up</Button>
							</A>
						</div>

						<Button type="submit">go</Button>
					</div>
				</Form>
			</Stack>
		</div>
	);
};

export default LoginPage;
