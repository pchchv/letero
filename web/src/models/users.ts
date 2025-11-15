export interface User {
	id: number;
	username: string;
	created_at: Date;
}

export interface UsersContextModel {
	currentUser: User | null;
	users: Record<number, User>;
}
