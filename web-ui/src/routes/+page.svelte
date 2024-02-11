<script lang="ts">
    import { phoneAlarmState } from '$lib/store';
    import Button from '$lib/components/ui/button/button.svelte';

    let phoneAlarmStateValue = false;
    phoneAlarmState.subscribe(value => {
        value ? phoneAlarmStateValue = true : phoneAlarmStateValue = false;
    });

    function phoneAlarmOff() {
        phoneAlarmState.set(false);
	}
    function phoneAlarmOn() {
        phoneAlarmState.set(true);
	}
</script>


{#if phoneAlarmStateValue}
    <h1>Alarm State: <span style="color: red">ON</span></h1>
{:else}
    <h1>Alarm State: <span style="color: green">OFF</span></h1>
{/if}

<Button on:click={phoneAlarmOn}>Turn ON</Button>
<Button on:click={phoneAlarmOff}>Turn OFF</Button>