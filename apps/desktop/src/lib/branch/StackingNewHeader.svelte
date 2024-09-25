<script lang="ts">
	import Link from '$lib/shared/Link.svelte';
	import Spacer from '$lib/shared/Spacer.svelte';
	import Button from '@gitbutler/ui/Button.svelte';
	import Icon from '@gitbutler/ui/Icon.svelte';
	import type { Commit, DetailedCommit } from '$lib/vbranches/types';

	interface Props {
		addBranch: (e: Event, commit: DetailedCommit | Commit) => void;
	}

	const { addBranch }: Props = $props();

	function closeStackingCard() {
		// 1. Hide card
		// 2. Set cookie to avoid showing details again
	}

	const showDetails = $state(true);
</script>

<section class="card">
	<button tabindex="0" class="card__close" onclick={closeStackingCard}>
		<Icon name="cross-small" />
	</button>
	{#if showDetails}
		<div class="card__body">
			<h2 class="text-16 text-bold">New branch stacking</h2>
			<p class="text-12 card__description">
				Allows you to add a branch that depends on previous branches. This helps you create smaller
				PRs that are reviewed and merged in order.
				<Link href="https://docs.gitbutler.com/stacking" target="_blank">Read more</Link>
			</p>
		</div>
	{/if}
	<Spacer />
	<section class="card__action">
		<div class="card__action--left">
			<div class="card__action--icon">
				<Icon name="plus-small" />
			</div>
			<div class="card__action--bar"></div>
		</div>
		<Button grow style="neutral" onclick={addBranch}>Add a branch to the stack</Button>
	</section>
</section>

<style>
	.card {
		position: relative;
		display: flex;
		flex-direction: column;
	}

	.card__body {
		padding: 16px 16px 0 16px;
	}

	.card__close {
		position: absolute;
		top: 4px;
		right: 4px;

		color: var(--clr-scale-ntrl-60);
	}

	.card__description {
		color: var(--clr-scale-ntrl-50);
		line-height: 18px;
	}

	.card__action {
		width: 100%;
		display: flex;
		justify-content: around;
		align-items: flex-start;
		padding: 0 13px;
		gap: 1rem;
	}

	.card__action--left {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 0.25rem;

		& .card__action--icon {
			display: flex;
			align-items: center;
			justify-content: center;
			width: 20px;
			height: 28px;
			border-radius: 30%;
			background-color: var(--clr-scale-ntrl-80);
		}
		& .card__action--bar {
			height: 10px;
			border: 1px solid var(--clr-border-3);
		}
	}
</style>
