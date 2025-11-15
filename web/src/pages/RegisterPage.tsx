import "./auth.css";
import { A, useNavigate } from "@solidjs/router";
import { Button, Form, Stack } from "solid-bootstrap";
import { createSignal, Show } from "solid-js";
import { ILoginErrorResponse, ILoginResponse } from "../models/auth";
import { useUsers } from "../contexts/UserContext";

const RegisterPage = () => {
	const navigate = useNavigate();
	const { setUser } = useUsers();

	let loginInput!: HTMLInputElement;
	const [usernameErrors, setUsernameErrors] = createSignal<string[]>([]);
	const [passwordErrors, setPasswordErrors] = createSignal<string[]>([]);
	const [password, setPassword] = createSignal("");
	const [confirmPassword, setConfirmPassword] = createSignal("");
	const [passwordsEquals, setPasswordAreEquals] = createSignal(false);

	const onSubmit = async (e: SubmitEvent) => {
		e.preventDefault();

		if (password() !== confirmPassword()) {
			setPasswordAreEquals(true);
			return;
		}

		setUsernameErrors([]);
		setPasswordErrors([]);
		setPasswordAreEquals(false);

		const data = {
			username: loginInput.value,
			password: password(),
		};

		const res = await fetch("/users", {
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
				username: loginInput.value,
			});
			navigate("/", { replace: true });
		} else if (res.status === 400) {
			const err: ILoginErrorResponse = await res.json();

			const passError = err.fields["password"];
			if (passError) {
				setPasswordErrors(passError);
			}

			const nameError = err.fields["username"];
			if (nameError) {
				setUsernameErrors(nameError);
			}
		} else if (res.status === 409) {
			setUsernameErrors(["There is already a user with that name"]);
		} else {
			console.error(res.status);
			console.error(await res.json());
		}
	};

	return (
		<div class="d-flex w-100 vh-100 justify-content-center align-items-center">
			<Stack class="border rounded p-3">
				<h1>Register</h1>
				<Form class="min-width mt-3" onSubmit={onSubmit}>
					<Form.Group class="mb-1">
						<Form.Control
							type="text"
							placeholder="Username"
							class="mb-1"
							ref={loginInput}
						/>
						<Show when={usernameErrors()}>
							<Form.Text class="text-danger">
								{usernameErrors().join(", ")}
							</Form.Text>
						</Show>
					</Form.Group>
					<Form.Group class="mb-1">
						<Form.Control
							type="password"
							placeholder="Password"
							class="mb-1"
							onInput={(e) => setPassword(e.currentTarget.value)}
						/>
						<Show when={passwordErrors()}>
							<Form.Text class="text-danger">
								{passwordErrors().join(", ")}
							</Form.Text>
						</Show>
					</Form.Group>
					<Form.Group>
						<Form.Control
							type="password"
							placeholder="Confirm password"
							class="mb-1"
							onInput={(e) => setConfirmPassword(e.currentTarget.value)}
						/>
						<Show when={passwordsEquals()}>
							<Form.Text class="text-danger">passwords don't match</Form.Text>
						</Show>
					</Form.Group>

					<div class="mt-3 d-flex w-100">
						<div class="w-100">
							<A href="/">
								<Button variant="outline-secondary">back</Button>
							</A>
							<A href="/login" class="me-1">
								<Button variant="link">login</Button>
							</A>
						</div>

						<Button type="submit">go</Button>
					</div>
				</Form>
			</Stack>
		</div>
	);
};

export default RegisterPage;
