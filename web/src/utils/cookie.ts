export class Cookie {
	name: string;
	value: string;

	constructor(name: string, value: string) {
		this.name = name;
		this.value = value;
	}
}

export class CookieJar {
	private cookies;

	constructor(cookie: string) {
		this.cookies = cookie
			.split("; ")
			.map((c) => c.split("="))
			.map((c) => new Cookie(c[0], c[1]));
	}

	getCookie(name: string): Cookie | undefined {
		return this.cookies.find((v) => v.name === name);
	}
}
