<script>
	export let bindings;
</script>

<main>
	<h1>Welcome to Shaun's Fantasy projections and predictor!</h1>
	<h3>It is still a work in progress and rough.</h3>
	<h2>Test: {bindings.add(5,7)}
	<div class="selections">
		<button>Pre-draft Player Stats</button>
		<button>Weekly Predictions</button>
	</div>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
