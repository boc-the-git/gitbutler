<script lang="ts">
	import StackingStatusIcon from './StackingStatusIcon.svelte';
	import { getColorFromBranchType, type BranchColor } from './stackingUtils';
	import { getGitHostListingService } from '$lib/gitHost/interface/gitHostListingService';
	import { getGitHostPrService } from '$lib/gitHost/interface/gitHostPrService';
	import PullRequestButton from '$lib/pr/PullRequestButton.svelte';
	import Button from '@gitbutler/ui/Button.svelte';

	interface Props {
		upstreamName: string | undefined;
	}

	const { upstreamName }: Props = $props();

	const hostedListingServiceStore = getGitHostListingService();
	const prStore = $derived($hostedListingServiceStore?.prs);
	const prs = $derived(prStore ? $prStore : undefined);

	const listedPr = $derived(prs?.find((pr) => pr.sourceBranch === upstreamName));
	const prNumber = $derived(listedPr?.number);

	const prService = getGitHostPrService();
	const prMonitor = $derived(prNumber ? $prService?.prMonitor(prNumber) : undefined);

	const pr = $derived(prMonitor?.pr);

	let loading = $state(false);

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
			<!-- TODO: Replace 'list-view' icon -->
			<Button icon="description" outline type="ghost" color="neutral" />
			<Button icon="edit-text" outline type="ghost" color="neutral" />
		</div>
	</div>
	{#if !pr}
		<div class="branch-action">
			<div class="branch-action__line" style:--bg-color={lineColor}></div>
			<div class="branch-action__btn">
				<PullRequestButton {loading} click={() => {}} />
			</div>
		</div>
	{/if}
</div>

<style lang="postcss">
	.branch-header {
		display: flex;
		border-bottom: 1px solid var(--clr-border-2);
		user-select: text;
		cursor: text;
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
		.branch-action__btn {
			width: 100%;
			padding: 4px 16px 12px 0px;
		}
	}
</style>
