<script lang="ts">
    import Button from '$lib/components/ui/button/button.svelte';

	/** @type {import('./$types').PageData} */
	export let data: AlarmState;
    let alarmState = data;
    
    async function phoneAlarmOn() {
        const payload = {
            phone_alarms: true,
            phone_notifications: false,
        };
        const res = await fetch("/alarm_state", {
            method: 'PUT',
            body: JSON.stringify(payload)
        });
        alarmState = await res.json();
        console.log(alarmState);
	}

    async function phoneAlarmOff() {
        const payload = {
            phone_alarms: false,
            phone_notifications: true,
        };
        const res = await fetch("/alarm_state", {
            method: 'PUT',
            body: JSON.stringify(payload)
        });
        alarmState = await res.json();
        console.log(alarmState);
	}

    async function testAlarmTrigger() {
        const res = await fetch("/alarm_trigger", {
            method: 'POST',
        })
        const data = await res.json()
        console.log(data);
    }    
</script>


<span>
    <Button on:click={phoneAlarmOn}>Turn ON</Button>
    <Button on:click={phoneAlarmOff}>Turn OFF</Button>
    {#if alarmState.phone_alarms}
        <span style="color: red">ALARM IS ON</span>
    {:else}
        <span style="color: green">ALARM IS OFF</span>
    {/if}
</span>

<br>
<br>
<Button on:click={testAlarmTrigger}>Test Alarm Trigger</Button>