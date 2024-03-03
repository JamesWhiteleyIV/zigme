/** @type {import('./$types').RequestHandler} */
export async function POST() {
	const payload = {
		title: 'test-title',
		message: 'test-message',
		timestamp: '0000-00-00 00:00:00 XXX'
	};
    //TODO dont hardcode server url
	const res = await fetch("http://zigme-api:3020/alarm_trigger", {
		method: 'POST',
		body: JSON.stringify(payload),
		headers: {
			'content-type': 'application/json'
		}
	});
	const data = await res.json();
	return new Response(JSON.stringify(data));
}
