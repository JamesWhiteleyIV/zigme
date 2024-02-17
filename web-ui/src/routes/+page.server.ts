/** @type {import('./$types').PageServerLoad} */

export async function load({ params, fetch }) {
    const res = await fetch("http://localhost:3000/alarm_state")
    const data: AlarmState = await res.json();
    return {data};
}