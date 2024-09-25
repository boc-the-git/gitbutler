<script lang="ts">
	import StackingBranchHeader from './StackingBranchHeader.svelte';
	import StackingNewHeader from './StackingNewHeader.svelte';
	import PullRequestCard from '../pr/PullRequestCard.svelte';
	import StackingCommitList from '$lib/commit/StackingCommitList.svelte';
	import { groupCommitsByRef } from '$lib/vbranches/commitGroups';
	import { getLocalAndRemoteCommits, getLocalCommits } from '$lib/vbranches/contexts';
	import { VirtualBranch } from '$lib/vbranches/types';

	interface Props {
		branches: VirtualBranch[];
	}

	const { branches }: Props = $props();

	const localCommits = getLocalCommits();
	const localAndRemoteCommits = getLocalAndRemoteCommits();

	const localCommitsConflicted = $derived($localCommits.some((commit) => commit.conflicted));
	const localAndRemoteCommitsConflicted = $derived(
		$localAndRemoteCommits.some((commit) => commit.conflicted)
	);
</script>

<StackingNewHeader />
{#each branches as branch}
	{#each groupCommitsByRef(branch.commits) as group (group.ref)}
		<div class="commit-group">
			{#if group.branchName}
				<StackingBranchHeader upstreamName={group.branchName} />
				<PullRequestCard upstreamName={group.branchName} />
			{/if}
			<StackingCommitList
				localCommits={group.localCommits}
				localAndRemoteCommits={group.remoteCommits}
				integratedCommits={group.integratedCommits}
				remoteCommits={[]}
				isUnapplied={false}
				{localCommitsConflicted}
				{localAndRemoteCommitsConflicted}
			/>
		</div>
	{/each}
{/each}

<style>
	.commit-group {
		margin-bottom: 10px;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		background: var(--clr-bg-1);
		overflow: hidden;
	}
</style>
