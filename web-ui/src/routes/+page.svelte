<script lang="ts">
    import Button from '$lib/components/ui/button/button.svelte';

    import { io } from "socket.io-client";

    const socket = io("ws://localhost:8000");

    let phoneAlarmStateValue = false;

    async function phoneAlarmOff() {
        const res = await fetch("/alarmstate/false", {
            method: 'PUT',
        })
        const json = await res.json()
        lastResponse = JSON.stringify(json);
 
	}
    function phoneAlarmOn() {
        // phoneAlarmState.set(true);
	}

    let lastResponse = '';
    async function testAlarmTrigger() {
        const payload = {
            title: "test-title",
            message: "test-message",
        };
        const res = await fetch("/alarmtrigger", {
            method: 'POST',
            body: JSON.stringify(payload)
        })
        const json = await res.json()
        lastResponse = JSON.stringify(json);
    }    
</script>


{#if phoneAlarmStateValue}
    <h1>Alarm State: <span style="color: red">ON</span></h1>
{:else}
    <h1>Alarm State: <span style="color: green">OFF</span></h1>
{/if}

<Button on:click={phoneAlarmOn}>Turn ON</Button>
<Button on:click={phoneAlarmOff}>Turn OFF</Button>
<br>
<br>

<Button on:click={testAlarmTrigger}>Test Alarm Trigger</Button>
<p>{lastResponse}</p>