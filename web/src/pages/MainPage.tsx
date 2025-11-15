import { Button, Stack } from "solid-bootstrap";
import "./MainPage.tsx.css";
import { A } from "@solidjs/router";

export default function MainPage() {
	return (
		<Stack>
			<div class="all-screen all-centered">
				<div class="stack horizontal-centered">
					<h1>
						Just<span class="first-color">ice</span>!
					</h1>
					<p>Just another chat</p>
					<div class="row stack gap-3">
						<A href="register" class="wfit">
							<Button variant="primary">sign up</Button>
						</A>
						<A href="login" class="wfit">
							<Button variant="secondary">login</Button>
						</A>
					</div>
				</div>
			</div>
			<div class="stack horizontal-centered">
				<span class="powered">
					powered by <span class="bootstrap-color">Bootstrap</span>
				</span>
			</div>
		</Stack>
	);
}
