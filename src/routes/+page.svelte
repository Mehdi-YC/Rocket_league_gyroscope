<script>
  import { onMount, afterUpdate } from 'svelte';

  let isFullscreen = false;

  let x = 0;
  let y = 0;
  let z = 0;
  
  let alpha = 0;
  let beta = 0;
  let gamma = 0;

  let jump = 0;
  let boost = 0;
  let default_z = 0;

  let dotX = 0;
  let dotY = 0;
  let barWidth = 0;
  let API_toggle = false

  function toggle_jump(val){
    jump = val;
    if (API_toggle) fetch(`/api?v=${x},${y},${z},${alpha},${beta},${gamma},${jump},${boost},${default_z}`);
  }

  function toggle_boost(val){
    boost = val;
    if (API_toggle)  fetch(`/api?v=${x},${y},${z},${alpha},${beta},${gamma},${jump},${boost},${default_z}`);
  }
  function toggle_api(){
    API_toggle = !API_toggle;
    if (!API_toggle){
      x = -5.0
      y=0.0
      z= default_z
      boost = 0.0
      jump = 0.0
      fetch(`/api?v=${x},${y},${z},${alpha},${beta},${gamma},${jump},${boost},${default_z}`);
      console.log("toggle api")
    }
  }

  function send_values(){
    if (API_toggle) fetch(`/api?v=${x},${y},${z},${alpha},${beta},${gamma},${jump},${boost},${default_z}`);
  }




  function lockOrientation() {
    if (window.screen.orientation && window.screen.orientation.lock) {
      window.screen.orientation.lock('landscape');
    } else if (window.screen.lockOrientation) {
      window.screen.lockOrientation('landscape');
    } else if (window.screen.mozLockOrientation) {
      window.screen.mozLockOrientation('landscape');
    } else if (window.screen.msLockOrientation) {
      window.screen.msLockOrientation('landscape');
    } else if (window.screen.orientation && window.screen.orientation.type) {
      // For older browsers that don't support the lock() method
      // You may display a message or instructions to the user
      // on how to manually change the orientation.
      console.log('Please switch to landscape orientation.');
    }
  }

  function toggleFullscreen() {
    if (!isFullscreen) {
      enterFullscreen();
    } else {
      exitFullscreen();
    }
  }

  function enterFullscreen() {
    const element = document.documentElement;

    if (element.requestFullscreen) {
      element.requestFullscreen();
    } else if (element.mozRequestFullScreen) {
      element.mozRequestFullScreen();
    } else if (element.webkitRequestFullscreen) {
      element.webkitRequestFullscreen();
    } else if (element.msRequestFullscreen) {
      element.msRequestFullscreen();
    }

    isFullscreen = true;
  }

  function exitFullscreen() {
    if (document.exitFullscreen) {
      document.exitFullscreen();
    } else if (document.mozCancelFullScreen) {
      document.mozCancelFullScreen();
    } else if (document.webkitExitFullscreen) {
      document.webkitExitFullscreen();
    } else if (document.msExitFullscreen) {
      document.msExitFullscreen();
    }

    isFullscreen = false;
  }
  onMount(() => {

    if (window.DeviceOrientationEvent) {
      window.addEventListener('devicemotion', handleMotionEvent);
      window.addEventListener('deviceorientation', handleOrientation);
      const interval = setInterval(send_values, 60);
    }

    return () => {
      clearInterval(interval);
    };

  });

  afterUpdate(() => {
    updateDotPosition();
    updateBarWidth();
  });



  function handleMotionEvent(event) {
    // console.log("Motion : ",event)
      let acceleration = event.rotationRate;
      if (acceleration) {
        alpha = acceleration.alpha; // X-axis rotation in degrees per second
        beta = acceleration.beta;   // Y-axis rotation in degrees per second
        gamma = acceleration.gamma; // Z-axis rotation in degrees per second

      }

      
      const radius = 50;
      const centerX = 100;
      const centerY = 100;

      const maxMovement = 45;

      const offsetX = (x / maxMovement) * radius;
      const offsetY = (y / maxMovement) * radius;

      dotX = centerX + offsetX;
      dotY = centerY + offsetY;
      const maxRotation = 360;

      barWidth = (z / maxRotation) * 100;
    }


  function handleOrientation(event) {
    // console.log("Orientation : ",event)
    x = event.beta;
    y = event.gamma;
    z = event.alpha;
  }

  function updateDotPosition() {
    const radius = 50;
    const centerX = 100;
    const centerY = 100;

    const maxMovement = 45;

    const offsetX = (x / maxMovement) * radius;
    const offsetY = (y / maxMovement) * radius;

    dotX = centerX + offsetX;
    dotY = centerY + offsetY;
  }

  function updateBarWidth() {
    const maxRotation = 360;

    barWidth = (z / maxRotation) * 100;
  }
</script>


<div class="main">
  <div class="container">
    <div class="circle">
      <div class="axes">
        <div class="x-axis"></div>
        <div class="y-axis"></div>
      </div>
      <div class="dot" style="left: {dotX}px; top: {dotY}px;"></div>
    </div>
    
    <div class="horizontal-bar">
      <div class="bar" style="width: {barWidth}%;"></div>
      <div style="display:flex;flex-direction: row; justify-content: space-evenly;padding-top: 20px;">
        <p>alpha :{parseFloat(alpha).toFixed(2)}</p>
        <p>beta  : {parseFloat(beta).toFixed(2)}</p>
        <p>gamma : {parseFloat(gamma).toFixed(2)}</p>
        <p>x :{parseFloat(x).toFixed(2)}</p>
        <p>y :{parseFloat(y).toFixed(2)}</p>
        <p>z :{parseFloat(z).toFixed(2)}</p>
      </div>
    </div>
    <!-- <p style="z-index:10">z : {z}</p> -->

  </div>
  
  <div class="jump_boost">
    <div style='background-color: {jump === 1.0 ? "green" : "transparent"};'  class="box jump"  on:touchstart={(e)=>{toggle_jump(1.0)}}  on:touchend={(e)=>{toggle_jump(0.0)}}></div>
    <div style='background-color: {boost === 1.0 ? "green" : "transparent"};' class="box boost" on:touchstart={(e)=>{toggle_boost(1.0)}}  on:touchend={(e)=>{toggle_boost(0.0)}}></div>
  </div>
</div>

<div class="other">
  <input type="text" bind:value={default_z}>
  <br>
  <button on:click={toggleFullscreen}>
    {isFullscreen ? 'Exit Fullscreen' : 'Enter Fullscreen'}
  </button>
  <button on:click={lockOrientation}>Lock Orientation</button>
  <button on:click={toggle_api}>Toggle API</button>
  <p>send : {API_toggle}</p>

</div>
<style>
  :fullscreen {
  width: 100vw;
  height: 100vh;
}
.circle {
  width: 200px !important;
  height: 200px !important;
  border: 1px solid black;
  border-radius: 50%;
  position: relative;

}

.axes {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
}

.x-axis,
.y-axis {
  position: absolute;
  background-color: lightgray;
  width: 100%;
  height: 2px;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
}

.x-axis {
  transform-origin: center left;
  transform: rotate(-90deg) translateY(-50%);
}

.dot {
  position: absolute;
  background-color: red;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
}

.horizontal-bar {
  width: 60%;
  height: 30px;
  background-color: lightgray;
  margin-top: 20px;
  position: relative;
}

.bar {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background-color: dodgerblue;
  
  transition: width 0.2s;
}
.container{
  display: flex;
  flex-direction: row;
}
.box{
  height: 200px;
  width: 50%;
  border: 1px solid black;
}

.jump_boost{
  height: 200px;
  display: flex;
  width: 100%;

}
</style>