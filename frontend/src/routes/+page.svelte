<script lang="ts">
  import { onMount } from 'svelte';

    interface TelemetryReading {
        chamber_pressure: number;
        thrust: number;
        flow_rate: number;
        temperature: number;
        vibration: number;
        stage: string;
    }

  let reading: TelemetryReading | null = $state(null);

  onMount(() => {
    const ws = new WebSocket('ws://localhost:3000/ws');

    ws.onmessage = (event) => {
      reading = JSON.parse(event.data);
    };
  });
</script>

<main>
  <h1>🚀 Rocket Telemetry</h1>

  {#if reading}
    <p>Stage: {reading.stage}</p>
    <p>Chamber Pressure: {reading.chamber_pressure.toFixed(2)} PSI</p>
    <p>Thrust: {reading.thrust.toFixed(2)} kN</p>
    <p>Flow Rate: {reading.flow_rate.toFixed(2)} kg/s</p>
    <p>Temperature: {reading.temperature.toFixed(2)} °C</p>
    <p>Vibration: {reading.vibration.toFixed(2)} g</p>
  {:else}
    <p>Waiting for telemetry...</p>
  {/if}
</main>