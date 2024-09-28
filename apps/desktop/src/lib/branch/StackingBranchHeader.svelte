<script lang="ts">
	import StackingStatusIcon from './StackingStatusIcon.svelte';
	import { getColorFromBranchType, type BranchColor } from './stackingUtils';
	import PullRequestButton from '$lib/pr/PullRequestButton.svelte';
	import StackingPullRequestCard from '$lib/pr/StackingPullRequestCard.svelte';
	import { getContextStore } from '$lib/utils/context';
	import { VirtualBranch } from '$lib/vbranches/types';
	import Button from '@gitbutler/ui/Button.svelte';

	interface Props {
		upstreamName: string | undefined;
	}

	const { upstreamName }: Props = $props();

	let loading = $state(false);

	const branchStore = getContextStore(VirtualBranch);
	const branch = $derived($branchStore);

	$inspect('stackingBranchHeader.branch', branch);

	// TODO: Get Branch Status
	const branchType = $state<BranchColor>('integrated');
	const lineColor = $derived(getColorFromBranchType(branchType));
</script>

<div class="branch-header">
	<div class="branch-info">
		<StackingStatusIcon icon="tick-small" color={branchType} gap={false} lineTop />
		<div class="text-14 text-bold branch-info__name">
			<span class="remote-name">origin/</span>{upstreamName}
		</div>
		<div class="branch-info__btns">
			<Button icon="description" outline type="ghost" color="neutral" />
			<Button icon="edit-text" outline type="ghost" color="neutral" />
		</div>
	</div>
	<div class="branch-action">
		<div class="branch-action__line" style:--bg-color={lineColor}></div>
		<div class="branch-action__body">
			{#if !branch.upstream?.givenName}
				<PullRequestButton {loading} click={() => {}} />
			{:else}
				<StackingPullRequestCard upstreamName={branch.upstream.givenName} />
			{/if}
		</div>
	</div>
</div>

<style lang="postcss">
	.branch-header {
		display: flex;
		border-bottom: 1px solid var(--clr-border-2);
		display: flex;
		flex-direction: column;
	}

	.branch-info {
		padding: 0 13px;
		display: flex;
		justify-content: flex-start;
		align-items: center;

		& .branch-info__name {
			padding: 8px 16px;
			flex-grow: 1;
		}
		& .branch-info__btns {
			display: flex;
			gap: 0.25rem;
		}

		.remote-name {
			color: var(--clr-scale-ntrl-60);
			padding-right: 2px;
		}
	}

	.branch-action {
		width: 100%;
		display: flex;
		justify-content: flex-start;
		align-items: stretch;

		.branch-action__line {
			margin: 0 22px 0 22px;
			border: 1px solid var(--bg-color, var(--clr-border-3));
		}
		.branch-action__body {
			width: 100%;
			padding: 4px 12px 12px 0px;
		}
	}
</style>
