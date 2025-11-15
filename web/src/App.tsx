import type { Component } from "solid-js";
import { Route, Router } from "@solidjs/router";
import MainPageRouter from "./routes/MainPageRouter";
import LoginPage from "./pages/LoginPage";
import RegisterPage from "./pages/RegisterPage";
import { UserProvider } from "./contexts/UserContext";
import ChatPage from "./pages/ChatPage";

const App: Component = () => {
	return (
		<UserProvider>
			<Router>
				<Route path="/" component={MainPageRouter}>
					<Route path="/" component={ChatPage} />
				</Route>
				<Route path="/register" component={RegisterPage} />
				<Route path="/login" component={LoginPage} />
			</Router>
		</UserProvider>
	);
};

export default App;
