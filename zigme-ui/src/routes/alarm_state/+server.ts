/** @type {import('./$types').RequestHandler} */
export async function GET() {
	const res = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/alarm_state`);
	const data = await res.json();
	return new Response(JSON.stringify(data));
}

/** @type {import('./$types').RequestHandler} */
export async function PUT({ request }) {
	let payload: AlarmState = await request.json();
	const res = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/alarm_state`, {
		method: 'PUT',
		body: JSON.stringify(payload),
		headers: {
			'content-type': 'application/json'
		}
	});
	const data = await res.json();
	return new Response(JSON.stringify(data));
}
