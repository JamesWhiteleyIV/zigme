import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function POST() {
    const payload = {
        title: "test-title",
        message: "test-message",
    };
    const res = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/alarm_trigger`, {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
            "content-type": "application/json"
        }
    })
    const data = await res.json();
    return new Response(JSON.stringify(data));
}  