import { DetailedCommit } from '$lib/vbranches/types';
import { Type, Transform } from 'class-transformer';

// Return type of `list_virtual_branches().series`
export class Series {
	id!: string;
	@Type(() => DetailedCommit)
	remoteCommits!: DetailedCommit[];
	@Type(() => DetailedCommit)
	localCommits!: DetailedCommit[];

	@Transform((obj) => new Date(obj.value))
	modifiedAt!: Date;
	@Transform((obj) => new Date(obj.value))
	createdAt!: Date;
}
