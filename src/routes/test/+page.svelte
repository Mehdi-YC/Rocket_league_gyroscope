<script>
    import { onMount } from 'svelte';
  
    let gyroscopeSupported = false;
  
    onMount(() => {
      gyroscopeSupported = 'DeviceMotionEvent' in window;
  
      if (gyroscopeSupported) {
        window.addEventListener('devicemotion', handleMotionEvent);
      }
    });
  
    function handleMotionEvent(event) {
      let acceleration = event.rotationRate;
      if (acceleration) {
        let alpha = acceleration.alpha; // X-axis rotation in degrees per second
        let beta = acceleration.beta;   // Y-axis rotation in degrees per second
        let gamma = acceleration.gamma; // Z-axis rotation in degrees per second
        
        // Do something with the gyroscope data
        console.log('Gyroscope acceleration:', { alpha, beta, gamma });
      }
    }
  </script>
  
  {#if gyroscopeSupported}
    <p>Gyroscope acceleration is supported.</p>
  {:else}
    <p>Gyroscope acceleration is not supported on this device.</p>
  {/if}
  