// Initial main page fetch for alarm state
/** @type {import('./$types').PageServerLoad} */

export async function load({ fetch }) {
    const res = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/alarm_state`)
    const data: AlarmState = await res.json();
    return {data};
}