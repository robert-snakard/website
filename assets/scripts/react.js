'use strict;'

const e = React.createElement;

class Hero extends React.Component {
	constructor(props) {
		super(props)
		this.state = { toggle: false };
	}

	render() {
		if (this.state.toggle) {
			return (
				<section class="hero">
				<h1 class="button negative" onClick={() => this.setState({toggle: false})}>Clicked!</h1>
				</section>
			);
		}

		return (
			<section class="hero">
			<h1 class="button" onClick={() => this.setState({toggle: true})}>Going Up</h1>
			</section>
		);
	}
}

// link to the html
const domContainer = document.querySelector('#react');
ReactDOM.render(e(Hero), domContainer);
