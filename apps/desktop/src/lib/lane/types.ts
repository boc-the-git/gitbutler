import { DetailedCommit } from '$lib/vbranches/types';
import { Type, Transform } from 'class-transformer';

export class Lane {
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
