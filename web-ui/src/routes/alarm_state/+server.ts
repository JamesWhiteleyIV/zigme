import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function GET() {
    const res = await fetch("http://localhost:3000/alarm_state")
    const data = await res.json();
    return new Response(JSON.stringify(data));
}

/** @type {import('./$types').RequestHandler} */
export async function PUT({request}) {
    let payload: AlarmState = await request.json();
    const res = await fetch("http://localhost:3000/alarm_state", {
        method: 'PUT',
        body: JSON.stringify(payload),
        headers: {
            "content-type": "application/json"
        }
    })
    const data = await res.json();
    return new Response(JSON.stringify(data));
}  