/** @type {import('./$types').RequestHandler} */
export async function GET() {
	const data = {"health": "OK"} 
	return new Response(JSON.stringify(data));
}

