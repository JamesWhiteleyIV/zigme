/** @type {import('@sveltejs/kit').ParamMatcher} */
export function match(param) {
	return /^(true|false)$/.test(param);
}