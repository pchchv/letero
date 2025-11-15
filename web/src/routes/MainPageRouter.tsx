import { RouteSectionProps } from "@solidjs/router";
import { CookieJar } from "../utils/cookie";
import MainPage from "../pages/MainPage";
import { Component } from "solid-js";

const MainPageRouter: Component<RouteSectionProps> = (
	props: RouteSectionProps,
) => {
	const cookies = new CookieJar(document.cookie);
	const session = cookies.getCookie("session");

	if (session === undefined) {
		return <MainPage />;
	} else {
		return props.children;
	}
};

export default MainPageRouter;
