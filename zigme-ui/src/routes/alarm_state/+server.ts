/** @type {import('./$types').RequestHandler} */
export async function GET() {
    // TODO: do not hard code
	const res = await fetch("http://zigme-api:3020/alarm_state", {
        headers: {
			'content-type': 'application/json'
		}
    });
	const data = await res.json();
	return new Response(JSON.stringify(data));
}

/** @type {import('./$types').RequestHandler} */
export async function PUT({ request }) {
	let payload: AlarmState = await request.json();
    // TODO: do not hard code
	const res = await fetch("http://zigme-api:3020/alarm_state", {
		method: 'PUT',
		body: JSON.stringify(payload),
		headers: {
			'content-type': 'application/json'
		}
	});
	const data = await res.json();
	return new Response(JSON.stringify(data));
}
