import { json } from '@sveltejs/kit';
import { get } from 'svelte/store';
import fs from "fs";

const STATE_FILENAME = "state.txt";

import { page } from '$app/stores';

let state = '?';

/** @type {import('./$types').RequestHandler} */
export async function PUT({ params }) {
    fs.writeFile(STATE_FILENAME, params.state, (err) => {
        if (err) throw err;
    })
    state = params.state;
    return json({state: state});
}

/** @type {import('./$types').RequestHandler} */
export async function GET() {
    fs.readFile(STATE_FILENAME, (err, inputD) => {
        if (err) throw err;
           console.log(inputD.toString());
           state = inputD.toString();
     })
    return json({state: state});
}

