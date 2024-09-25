export type BranchColor = 'neutral' | 'integrated';

export function getColorFromBranchType(type: BranchColor) {
	const colorMap = {
		neutral: 'var(--clr-scale-ntrl-80)',
		integrated: 'var(--clr-commit-integrated)'
	};

	return colorMap[type];
}
