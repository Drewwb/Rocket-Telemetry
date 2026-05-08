<script>
    import { onMount } from 'svelte';

    let message = "Waiting for telemetry..";

    onMount(() => {
        const ws = new WebSocket('ws://localhost:3000/ws');
        
        ws.onmessage = (event) => {
            message = event.data;
        };
    });
</script>

<main>
  <h1>🚀 Rocket Telemetry</h1>
  <p>{message}</p>
</main>