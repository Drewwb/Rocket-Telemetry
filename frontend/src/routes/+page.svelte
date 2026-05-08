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

<style>
  main {
    background: #0a0a0a;
    color: #00ff88;
    font-family: monospace;
    min-height: 100vh;
    padding: 2rem;
  }

  .cards {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    max-width: 600px;
  }

  .card {
    background: #111;
    border: 1px solid #00ff88;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
  }

  .value {
    font-size: 2rem;
    font-weight: bold;
  }

  .label {
    font-size: 0.8rem;
    opacity: 0.7;
  }

.card.warning {
  border-color: #ffaa00;
  color: #ffaa00;
}

.card.critical {
  border-color: #ff3333;
  color: #ff3333;
}
</style>

<main>
    <h1>🚀 Rocket Telemetry</h1>
    {#if reading}
        <p>Stage: {reading.stage}</p>

        <div class="cards">
            <div id="chamber_pressure" class="card" class:warning={reading.chamber_pressure > 109 && reading.chamber_pressure <= 125} class:critical={reading.chamber_pressure > 125}>
                <p class="label">Chamber Pressure</p>
                <p class="value">{reading.chamber_pressure.toFixed(2)} PSI</p>
            </div>
            <div id="thrust" class="card" class:warning={reading.thrust > 559 && reading.thrust <= 620} class:critical={reading.thrust > 620}>
                <p class="label">Thrust</p>
                <p class="value">{reading.thrust.toFixed(2)} kN</p>
            </div>
            <div id="flow_rate" class="card" class:warning={reading.flow_rate > 23 && reading.flow_rate <= 27} class:critical={reading.flow_rate > 27}>
                <p class="label">Flow Rate</p>
                <p class="value">{reading.flow_rate.toFixed(2)} kg/s</p>
            </div>
            <div id="temperature" class="card" class:warning={reading.temperature > 330 && reading.temperature <= 360} class:critical={reading.temperature > 360}>
                <p class="label">Temperature</p>
                <p class="value">{reading.temperature.toFixed(2)} °C</p>
            </div>
            <div id="vibration" class="card" class:warning={reading.vibration > 2.8 && reading.vibration <= 3.2} class:critical={reading.vibration > 3.2}>
                <p class="label">vibration</p>
                <p class="value">{reading.vibration.toFixed(2)} g</p>
            </div>
        </div>
    {:else}
        <p>Waiting for telemetry...</p>
    {/if}
</main>