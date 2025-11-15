import {
	createContext,
	createEffect,
	createSignal,
	ParentComponent,
	ParentProps,
	useContext,
} from "solid-js";
import { User, UsersContextModel } from "../models/users";
import { createStore, SetStoreFunction } from "solid-js/store";

interface UsersContextValue {
	users: UsersContextModel;
	setUser: SetStoreFunction<UsersContextModel>;
}

const UsersContext = createContext<UsersContextValue>();

export const UserProvider: ParentComponent = (props: ParentProps) => {
	const [users, setUser] = createStore<UsersContextModel>({
		currentUser: null,
		users: {},
	});

	createEffect(async () => {
		if (users.currentUser) {
			return;
		}

		const res = await fetch("/users/0");

		if (!res.ok) {
			console.error(res.status);
			console.error(await res.json());
			return;
		}

		const body: User = await res.json();
		setUser("currentUser", body);
	});

	return (
		<UsersContext.Provider value={{ users, setUser }}>
			{props.children}
		</UsersContext.Provider>
	);
};

export function useUsers() {
	const ctx = useContext(UsersContext);
	if (!ctx) throw new Error("useUser must be used within <UserProvider>");
	return ctx;
}
``;
