import IServerError from "./error";

export interface ILoginErrorResponse extends IServerError {
	readonly fields: Record<string, string[]>;
}

export interface ILoginResponse {
	readonly user_id: number;
}
