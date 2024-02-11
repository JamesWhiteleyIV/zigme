import { json } from '@sveltejs/kit';

let state = false;

// send request to pushover to trigger alarm
async function setState(state: boolean) {
    state = state;
}

/** @type {import('./$types').RequestHandler} */
export async function PUT({ params }) {
    return json({});
}