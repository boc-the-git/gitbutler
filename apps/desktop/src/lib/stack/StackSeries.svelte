<script lang="ts">
	import { BaseBranch } from '$lib/baseBranch/baseBranch';
	import StackingBranchHeader from '$lib/branch/StackingBranchHeader.svelte';
	import StackingNewStackCard from '$lib/branch/StackingNewStackCard.svelte';
	import StackingCommitList from '$lib/commit/StackingCommitList.svelte';
	import { getContextStore } from '$lib/utils/context';
	import { groupCommitsByRef } from '$lib/vbranches/commitGroups';
	import { getLocalAndRemoteCommits, getLocalCommits } from '$lib/vbranches/contexts';
	import { Commit, DetailedCommit, VirtualBranch } from '$lib/vbranches/types';
	import Modal from '@gitbutler/ui/Modal.svelte';

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

	const _baseBranch = getContextStore(BaseBranch);
	let createRefModal: Modal;
	// console.log('BASE.BRANCH', $baseBranch);

	function openCreateRefModal(e: Event, commit: DetailedCommit | Commit) {
		e.stopPropagation();
		createRefModal.show(commit);
	}
	console.log('groupCommitsByRef', groupCommitsByRef(branches[0]?.commits ?? []));
</script>

<StackingNewStackCard addBranch={openCreateRefModal} />
<!-- TODO: Add connecting line on background between NewStackCard above and branches below -->
{#each branches as branch}
	{#each groupCommitsByRef(branch.commits) as group (group.ref)}
		<div class="branch-group">
			<StackingBranchHeader />
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
	.branch-group {
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-m);
		background: var(--clr-bg-1);
		overflow: hidden;
	}
</style>
