import { invoke, listen } from '$lib/backend/ipc';
import { Lane } from '$lib/lane/types';
import { VirtualBranches, type VirtualBranch } from '$lib/vbranches/types';
import { plainToInstance } from 'class-transformer';
import { writable } from 'svelte/store';
import type { VirtualBranchService } from '$lib/vbranches/virtualBranch';

export class LaneController {
	private loading = writable(false);
	readonly error = writable();

	readonly lanes = writable<Lane[] | undefined>(undefined, () => {
		this.refresh();
		const unsubscribe = this.subscribe((lane) => lane);
		return () => {
			unsubscribe();
		};
	});

	constructor(
		private projectId: string,
		private virtualBranchService: VirtualBranchService
	) {}

	async refresh() {
		this.loading.set(true);
		try {
			await this.listVirtualBranches();
		} catch (err: any) {
			console.error(err);
			this.error.set(err);
		} finally {
			this.loading.set(false);
		}
	}

	private async listVirtualBranches(): Promise<VirtualBranch[]> {
		return plainToInstance(
			VirtualBranches,
			await invoke<any>('list_virtual_branches', { projectId: this.projectId })
			// TODO: Get new field name from Kiril
		).branches;
	}

	// NOTE: For backward compat, lane info is coming down this listen endpoint for now,
	// in the future we'll probably want to create a separate listen endpoint and separate invoke
	// cmd for lanes
	private subscribe(callback: (branches: VirtualBranch[]) => void) {
		return listen<any>(`project://${this.projectId}/virtual-branches`, (event) =>
			callback(plainToInstance(VirtualBranches, event.payload).branches)
		);
	}
}
