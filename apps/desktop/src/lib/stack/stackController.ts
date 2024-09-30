import { invoke, listen } from '$lib/backend/ipc';
import { Series } from '$lib/stack/types';
import { VirtualBranches, type VirtualBranch } from '$lib/vbranches/types';
import { plainToInstance } from 'class-transformer';
import { writable } from 'svelte/store';
import type { VirtualBranchService } from '$lib/vbranches/virtualBranch';

// Stack is the top-most organisational level.
// It used to be called 'Lane', and contains multiple Series (branches).
export class StackController {
	private loading = writable(false);
	readonly error = writable();

	// Used to be 'branches'
	readonly series = writable<Series[] | undefined>(undefined, () => {
		this.refresh();
		const unsubscribe = this.subscribe(async (vbranch) => await this.handleVBranchPayload(vbranch));
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
			await this.handleVBranchPayload(await this.listVirtualBranches());
		} catch (err: any) {
			console.error(err);
			this.error.set(err);
		} finally {
			this.loading.set(false);
		}
	}

	async handleVBranchPayload(branches: VirtualBranch[]) {
		console.log(branches);
		// Handle incoming data from `list_virtual_branches` and return data structure something at least like:
		// Stack {
		//  id: string
		//  series: Series[] // used to be '(Virtual)Branches[]'
		// }[]
		//

		this.series.set(branches);
		this.error.set(undefined);
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

	async createStack() {
		// Create is happening on the BE, needs new API (`create_stack`?)
		// 1. pass projectId to `create_stack` invoke fn
	}

	async pushStack() {
		// Loop over all series (branches) in the stack (lane) and:
		// 1. If only 1 branch, don't worry about updating comments, etc.
		// 2. If more than 1 branch:
		// 3. Push commits to remote
		// 4. Update comments
	}
}
